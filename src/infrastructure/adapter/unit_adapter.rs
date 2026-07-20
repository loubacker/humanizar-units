use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::exception::UnitException;
use crate::domain::model::Unit;
use crate::domain::port::UnitPort;
use crate::infrastructure::handler::PostgresErrorHandler;
use crate::infrastructure::persistence::entity::UnitEntity;
use crate::infrastructure::persistence::repository::UnitRepository;
use crate::infrastructure::resilience::RetryExecutor;

pub struct UnitAdapter {
    repository: UnitRepository,
    retry_executor: RetryExecutor,
}

impl UnitAdapter {
    pub const fn new(repository: UnitRepository, retry_executor: RetryExecutor) -> Self {
        Self {
            repository,
            retry_executor,
        }
    }
}

#[async_trait]
impl UnitPort for UnitAdapter {
    async fn save(&self, unit: Unit) -> Result<Unit, UnitException> {
        self.repository
            .save(to_entity(unit))
            .await
            .map_err(PostgresErrorHandler::write_exception)
            .map(to_domain)
    }

    async fn find_by_municipio_id(&self, municipio_id: Uuid) -> Result<Vec<Unit>, UnitException> {
        let entities = self
            .retry_executor
            .execute_read("unit.find_by_municipio_id", || {
                self.repository.find_by_municipio_id(municipio_id)
            })
            .await?;

        Ok(map_entities(entities))
    }

    async fn find_by_id_and_municipio_id(
        &self,
        unit_id: Uuid,
        municipio_id: Uuid,
    ) -> Result<Option<Unit>, UnitException> {
        let unit = self
            .retry_executor
            .execute_read("unit.find_by_id_and_municipio_id", || {
                self.repository
                    .find_by_id_and_municipio_id(unit_id, municipio_id)
            })
            .await?;

        Ok(unit.map(to_domain))
    }

    async fn find_by_municipio_id_and_cnpj(
        &self,
        municipio_id: Uuid,
        cnpj: &str,
    ) -> Result<Option<Unit>, UnitException> {
        let unit = self
            .retry_executor
            .execute_read("unit.find_by_municipio_id_and_cnpj", || {
                self.repository
                    .find_by_municipio_id_and_cnpj(municipio_id, cnpj)
            })
            .await?;

        Ok(unit.map(to_domain))
    }

    async fn find_by_ids(&self, unit_ids: &[Uuid]) -> Result<Vec<Unit>, UnitException> {
        let entities = self
            .retry_executor
            .execute_read("unit.find_by_ids", || self.repository.find_by_ids(unit_ids))
            .await?;

        Ok(map_entities(entities))
    }

    async fn count_by_municipio_id(&self, municipio_id: Uuid) -> Result<u64, UnitException> {
        Ok(self
            .retry_executor
            .execute_read("unit.count_by_municipio_id", || {
                self.repository.count_by_municipio_id(municipio_id)
            })
            .await?)
    }

    async fn delete_by_id(&self, unit_id: Uuid) -> Result<bool, UnitException> {
        Ok(self.repository.delete_by_id(unit_id).await?)
    }
}

fn to_entity(unit: Unit) -> UnitEntity {
    UnitEntity::new(
        unit.id(),
        unit.municipio_id(),
        unit.created_at(),
        unit.updated_at(),
        unit.unit_name().to_owned(),
        unit.razao_social().to_owned(),
        unit.endereco().to_owned(),
        unit.numero().to_owned(),
        unit.complemento().map(str::to_owned),
        unit.bairro().to_owned(),
        unit.cep().to_owned(),
        unit.cnpj().to_owned(),
    )
}

fn to_domain(unit: UnitEntity) -> Unit {
    Unit::restore(
        unit.id,
        unit.municipio_id,
        unit.unit_name,
        unit.razao_social,
        unit.endereco,
        unit.numero,
        unit.complemento,
        unit.bairro,
        unit.cep,
        unit.cnpj,
        unit.created_at,
        unit.updated_at,
    )
}

fn map_entities(entities: Vec<UnitEntity>) -> Vec<Unit> {
    entities.into_iter().map(to_domain).collect()
}
