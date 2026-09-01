use uuid::Uuid;

use crate::domain::exception::{PersistenceException, TechnicalError};
use crate::infrastructure::config::DatabaseConfig;
use crate::infrastructure::handler::PostgresErrorHandler;
use crate::infrastructure::persistence::entity::UnitEntity;

#[derive(Clone)]
pub struct UnitRepository {
    database: DatabaseConfig,
}

impl UnitRepository {
    pub const fn new(database: DatabaseConfig) -> Self {
        Self { database }
    }

    pub async fn save(&self, unit: UnitEntity) -> Result<UnitEntity, PersistenceException> {
        match unit.id {
            Some(unit_id) => self.update(unit_id, unit).await,
            None => self.insert(unit).await,
        }
    }

    pub async fn find_by_municipio_id(
        &self,
        municipio_id: Uuid,
    ) -> Result<Vec<UnitEntity>, PersistenceException> {
        let connection = self.database.acquire().await?;
        let rows = connection
            .query(
                r#"
                    SELECT
                        id,
                        municipio_id,
                        unit_name,
                        razao_social,
                        endereco,
                        numero,
                        complemento,
                        bairro,
                        cep,
                        cnpj,
                        created_at,
                        updated_at
                    FROM public.units
                    WHERE municipio_id = $1
                    ORDER BY unit_name, id
                "#,
                &[&municipio_id],
            )
            .await
            .map_err(PostgresErrorHandler::query_exception)?;

        map_rows(&rows)
    }

    pub async fn find_by_id_and_municipio_id(
        &self,
        unit_id: Uuid,
        municipio_id: Uuid,
    ) -> Result<Option<UnitEntity>, PersistenceException> {
        let connection = self.database.acquire().await?;
        let row = connection
            .query_opt(
                r#"
                    SELECT
                        id,
                        municipio_id,
                        unit_name,
                        razao_social,
                        endereco,
                        numero,
                        complemento,
                        bairro,
                        cep,
                        cnpj,
                        created_at,
                        updated_at
                    FROM public.units
                    WHERE id = $1
                      AND municipio_id = $2
                "#,
                &[&unit_id, &municipio_id],
            )
            .await
            .map_err(PostgresErrorHandler::query_exception)?;

        row.as_ref()
            .map(UnitEntity::from_row)
            .transpose()
            .map_err(PostgresErrorHandler::query_exception)
    }

    pub async fn find_by_municipio_id_and_cnpj(
        &self,
        municipio_id: Uuid,
        cnpj: &str,
    ) -> Result<Option<UnitEntity>, PersistenceException> {
        let connection = self.database.acquire().await?;
        let row = connection
            .query_opt(
                r#"
                    SELECT
                        id,
                        municipio_id,
                        unit_name,
                        razao_social,
                        endereco,
                        numero,
                        complemento,
                        bairro,
                        cep,
                        cnpj,
                        created_at,
                        updated_at
                    FROM public.units
                    WHERE municipio_id = $1
                      AND cnpj = $2
                "#,
                &[&municipio_id, &cnpj],
            )
            .await
            .map_err(PostgresErrorHandler::query_exception)?;

        row.as_ref()
            .map(UnitEntity::from_row)
            .transpose()
            .map_err(PostgresErrorHandler::query_exception)
    }

    pub async fn find_by_ids(
        &self,
        unit_ids: &[Uuid],
    ) -> Result<Vec<UnitEntity>, PersistenceException> {
        if unit_ids.is_empty() {
            return Ok(Vec::new());
        }

        let connection = self.database.acquire().await?;
        let unit_ids = unit_ids.to_vec();
        let rows = connection
            .query(
                r#"
                    SELECT
                        id,
                        municipio_id,
                        unit_name,
                        razao_social,
                        endereco,
                        numero,
                        complemento,
                        bairro,
                        cep,
                        cnpj,
                        created_at,
                        updated_at
                    FROM public.units
                    WHERE id = ANY($1)
                    ORDER BY unit_name, id
                "#,
                &[&unit_ids],
            )
            .await
            .map_err(PostgresErrorHandler::query_exception)?;

        map_rows(&rows)
    }

    pub async fn count_by_municipio_id(
        &self,
        municipio_id: Uuid,
    ) -> Result<u64, PersistenceException> {
        let connection = self.database.acquire().await?;
        let row = connection
            .query_one(
                "SELECT COUNT(*) AS total FROM public.units WHERE municipio_id = $1",
                &[&municipio_id],
            )
            .await
            .map_err(PostgresErrorHandler::query_exception)?;
        let total: i64 = row
            .try_get("total")
            .map_err(PostgresErrorHandler::query_exception)?;

        u64::try_from(total).map_err(|error| {
            PersistenceException::query(TechnicalError::with_source(
                "Contagem PostgreSQL de unidades retornou valor invÃ¡lido",
                error,
            ))
        })
    }

    pub async fn delete_by_id(&self, unit_id: Uuid) -> Result<bool, PersistenceException> {
        let connection = self.database.acquire().await?;
        let affected_rows = connection
            .execute("DELETE FROM public.units WHERE id = $1", &[&unit_id])
            .await
            .map_err(PostgresErrorHandler::query_exception)?;

        Ok(affected_rows > 0)
    }

    async fn insert(&self, unit: UnitEntity) -> Result<UnitEntity, PersistenceException> {
        let connection = self.database.acquire().await?;
        let unit_id = Uuid::new_v4();
        let complemento = unit.complemento.as_deref();
        let row = connection
            .query_one(
                r#"
                    INSERT INTO public.units (
                        id,
                        municipio_id,
                        unit_name,
                        razao_social,
                        endereco,
                        numero,
                        complemento,
                        bairro,
                        cep,
                        cnpj,
                        created_at,
                        updated_at
                    )
                    VALUES (
                        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
                        CURRENT_TIMESTAMP,
                        CURRENT_TIMESTAMP
                    )
                    RETURNING
                        id,
                        municipio_id,
                        unit_name,
                        razao_social,
                        endereco,
                        numero,
                        complemento,
                        bairro,
                        cep,
                        cnpj,
                        created_at,
                        updated_at
                "#,
                &[
                    &unit_id,
                    &unit.municipio_id,
                    &unit.unit_name,
                    &unit.razao_social,
                    &unit.endereco,
                    &unit.numero,
                    &complemento,
                    &unit.bairro,
                    &unit.cep,
                    &unit.cnpj,
                ],
            )
            .await
            .map_err(PostgresErrorHandler::query_exception)?;

        UnitEntity::from_row(&row).map_err(PostgresErrorHandler::query_exception)
    }

    async fn update(
        &self,
        unit_id: Uuid,
        unit: UnitEntity,
    ) -> Result<UnitEntity, PersistenceException> {
        let connection = self.database.acquire().await?;
        let complemento = unit.complemento.as_deref();
        let row = connection
            .query_opt(
                r#"
                    UPDATE public.units
                    SET
                        municipio_id = $2,
                        unit_name = $3,
                        razao_social = $4,
                        endereco = $5,
                        numero = $6,
                        complemento = $7,
                        bairro = $8,
                        cep = $9,
                        cnpj = $10,
                        updated_at = CURRENT_TIMESTAMP
                    WHERE id = $1
                    RETURNING
                        id,
                        municipio_id,
                        unit_name,
                        razao_social,
                        endereco,
                        numero,
                        complemento,
                        bairro,
                        cep,
                        cnpj,
                        created_at,
                        updated_at
                "#,
                &[
                    &unit_id,
                    &unit.municipio_id,
                    &unit.unit_name,
                    &unit.razao_social,
                    &unit.endereco,
                    &unit.numero,
                    &complemento,
                    &unit.bairro,
                    &unit.cep,
                    &unit.cnpj,
                ],
            )
            .await
            .map_err(PostgresErrorHandler::query_exception)?
            .ok_or_else(|| {
                PersistenceException::query(TechnicalError::new(
                    "Unidade nÃ£o encontrada durante atualizaÃ§Ã£o PostgreSQL",
                ))
            })?;

        UnitEntity::from_row(&row).map_err(PostgresErrorHandler::query_exception)
    }
}

fn map_rows(rows: &[tokio_postgres::Row]) -> Result<Vec<UnitEntity>, PersistenceException> {
    rows.iter()
        .map(UnitEntity::from_row)
        .collect::<Result<Vec<_>, _>>()
        .map_err(PostgresErrorHandler::query_exception)
}
