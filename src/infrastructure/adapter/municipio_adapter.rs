use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::exception::UnitException;
use crate::domain::model::Municipio;
use crate::domain::port::MunicipioPort;
use crate::infrastructure::handler::PostgresErrorHandler;
use crate::infrastructure::persistence::entity::MunicipioEntity;
use crate::infrastructure::persistence::repository::MunicipioRepository;
use crate::infrastructure::resilience::RetryExecutor;

pub struct MunicipioAdapter {
    repository: MunicipioRepository,
    retry_executor: RetryExecutor,
}

impl MunicipioAdapter {
    pub const fn new(repository: MunicipioRepository, retry_executor: RetryExecutor) -> Self {
        Self {
            repository,
            retry_executor,
        }
    }
}

#[async_trait]
impl MunicipioPort for MunicipioAdapter {
    async fn save(&self, municipio: Municipio) -> Result<Municipio, UnitException> {
        self.repository
            .save(to_entity(municipio))
            .await
            .map_err(PostgresErrorHandler::write_exception)
            .map(to_domain)
    }

    async fn find_all(&self) -> Result<Vec<Municipio>, UnitException> {
        let entities = self
            .retry_executor
            .execute_read("municipio.find_all", || self.repository.find_all())
            .await?;

        Ok(map_entities(entities))
    }

    async fn find_by_id(&self, municipio_id: Uuid) -> Result<Option<Municipio>, UnitException> {
        let municipio = self
            .retry_executor
            .execute_read("municipio.find_by_id", || {
                self.repository.find_by_id(municipio_id)
            })
            .await?;

        Ok(municipio.map(to_domain))
    }

    async fn find_by_codigo_ibge(
        &self,
        codigo_ibge: &str,
    ) -> Result<Option<Municipio>, UnitException> {
        let municipio = self
            .retry_executor
            .execute_read("municipio.find_by_codigo_ibge", || {
                self.repository.find_by_codigo_ibge(codigo_ibge)
            })
            .await?;

        Ok(municipio.map(to_domain))
    }

    async fn delete_by_id(&self, municipio_id: Uuid) -> Result<bool, UnitException> {
        Ok(self.repository.delete_by_id(municipio_id).await?)
    }
}

fn to_entity(municipio: Municipio) -> MunicipioEntity {
    MunicipioEntity::new(
        municipio.id(),
        municipio.codigo_ibge().to_owned(),
        municipio.nome().to_owned(),
        municipio.uf().to_owned(),
        municipio.created_at(),
        municipio.updated_at(),
    )
}

fn to_domain(municipio: MunicipioEntity) -> Municipio {
    Municipio::restore(
        municipio.id,
        municipio.codigo_ibge,
        municipio.nome,
        municipio.uf,
        municipio.created_at,
        municipio.updated_at,
    )
}

fn map_entities(entities: Vec<MunicipioEntity>) -> Vec<Municipio> {
    entities.into_iter().map(to_domain).collect()
}
