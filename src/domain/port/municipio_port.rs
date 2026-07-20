use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::exception::UnitException;
use crate::domain::model::Municipio;

#[async_trait]
pub trait MunicipioPort: Send + Sync {
    async fn save(&self, municipio: Municipio) -> Result<Municipio, UnitException>;

    async fn find_all(&self) -> Result<Vec<Municipio>, UnitException>;

    async fn find_by_id(&self, municipio_id: Uuid) -> Result<Option<Municipio>, UnitException>;

    async fn find_by_codigo_ibge(
        &self,
        codigo_ibge: &str,
    ) -> Result<Option<Municipio>, UnitException>;

    async fn delete_by_id(&self, municipio_id: Uuid) -> Result<bool, UnitException>;
}
