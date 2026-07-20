use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::exception::UnitException;
use crate::domain::model::Unit;

#[async_trait]
pub trait UnitPort: Send + Sync {
    async fn save(&self, unit: Unit) -> Result<Unit, UnitException>;

    async fn find_by_municipio_id(&self, municipio_id: Uuid) -> Result<Vec<Unit>, UnitException>;

    async fn find_by_id_and_municipio_id(
        &self,
        unit_id: Uuid,
        municipio_id: Uuid,
    ) -> Result<Option<Unit>, UnitException>;

    async fn find_by_municipio_id_and_cnpj(
        &self,
        municipio_id: Uuid,
        cnpj: &str,
    ) -> Result<Option<Unit>, UnitException>;

    async fn find_by_ids(&self, unit_ids: &[Uuid]) -> Result<Vec<Unit>, UnitException>;

    async fn count_by_municipio_id(&self, municipio_id: Uuid) -> Result<u64, UnitException>;

    async fn delete_by_id(&self, unit_id: Uuid) -> Result<bool, UnitException>;
}
