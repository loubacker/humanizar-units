use uuid::Uuid;

use crate::domain::exception::{PersistenceException, TechnicalError};
use crate::infrastructure::config::DatabaseConfig;
use crate::infrastructure::handler::PostgresErrorHandler;
use crate::infrastructure::persistence::entity::MunicipioEntity;

#[derive(Clone)]
pub struct MunicipioRepository {
    database: DatabaseConfig,
}

impl MunicipioRepository {
    pub const fn new(database: DatabaseConfig) -> Self {
        Self { database }
    }

    pub async fn save(
        &self,
        municipio: MunicipioEntity,
    ) -> Result<MunicipioEntity, PersistenceException> {
        match municipio.id {
            Some(municipio_id) => self.update(municipio_id, municipio).await,
            None => self.insert(municipio).await,
        }
    }

    pub async fn find_all(&self) -> Result<Vec<MunicipioEntity>, PersistenceException> {
        let connection = self.database.acquire().await?;
        let rows = connection
            .query(
                r#"
                    SELECT
                        id,
                        codigo_ibge,
                        nome,
                        uf,
                        created_at,
                        updated_at
                    FROM public.municipio
                    ORDER BY nome, id
                "#,
                &[],
            )
            .await
            .map_err(PostgresErrorHandler::query_exception)?;

        map_rows(&rows)
    }

    pub async fn find_by_id(
        &self,
        municipio_id: Uuid,
    ) -> Result<Option<MunicipioEntity>, PersistenceException> {
        let connection = self.database.acquire().await?;
        let row = connection
            .query_opt(
                r#"
                    SELECT
                        id,
                        codigo_ibge,
                        nome,
                        uf,
                        created_at,
                        updated_at
                    FROM public.municipio
                    WHERE id = $1
                "#,
                &[&municipio_id],
            )
            .await
            .map_err(PostgresErrorHandler::query_exception)?;

        row.as_ref()
            .map(MunicipioEntity::from_row)
            .transpose()
            .map_err(PostgresErrorHandler::query_exception)
    }

    pub async fn find_by_codigo_ibge(
        &self,
        codigo_ibge: &str,
    ) -> Result<Option<MunicipioEntity>, PersistenceException> {
        let connection = self.database.acquire().await?;
        let row = connection
            .query_opt(
                r#"
                    SELECT
                        id,
                        codigo_ibge,
                        nome,
                        uf,
                        created_at,
                        updated_at
                    FROM public.municipio
                    WHERE codigo_ibge = $1
                "#,
                &[&codigo_ibge],
            )
            .await
            .map_err(PostgresErrorHandler::query_exception)?;

        row.as_ref()
            .map(MunicipioEntity::from_row)
            .transpose()
            .map_err(PostgresErrorHandler::query_exception)
    }

    pub async fn delete_by_id(&self, municipio_id: Uuid) -> Result<bool, PersistenceException> {
        let connection = self.database.acquire().await?;
        let affected_rows = connection
            .execute(
                "DELETE FROM public.municipio WHERE id = $1",
                &[&municipio_id],
            )
            .await
            .map_err(PostgresErrorHandler::query_exception)?;

        Ok(affected_rows > 0)
    }

    async fn insert(
        &self,
        municipio: MunicipioEntity,
    ) -> Result<MunicipioEntity, PersistenceException> {
        let connection = self.database.acquire().await?;
        let municipio_id = Uuid::new_v4();
        let row = connection
            .query_one(
                r#"
                    INSERT INTO public.municipio (
                        id,
                        codigo_ibge,
                        nome,
                        uf,
                        created_at,
                        updated_at
                    )
                    VALUES (
                        $1,
                        $2,
                        $3,
                        $4,
                        CURRENT_TIMESTAMP,
                        CURRENT_TIMESTAMP
                    )
                    RETURNING
                        id,
                        codigo_ibge,
                        nome,
                        uf,
                        created_at,
                        updated_at
                "#,
                &[
                    &municipio_id,
                    &municipio.codigo_ibge,
                    &municipio.nome,
                    &municipio.uf,
                ],
            )
            .await
            .map_err(PostgresErrorHandler::query_exception)?;

        MunicipioEntity::from_row(&row).map_err(PostgresErrorHandler::query_exception)
    }

    async fn update(
        &self,
        municipio_id: Uuid,
        municipio: MunicipioEntity,
    ) -> Result<MunicipioEntity, PersistenceException> {
        let connection = self.database.acquire().await?;
        let row = connection
            .query_opt(
                r#"
                    UPDATE public.municipio
                    SET
                        codigo_ibge = $2,
                        nome = $3,
                        uf = $4,
                        updated_at = CURRENT_TIMESTAMP
                    WHERE id = $1
                    RETURNING
                        id,
                        codigo_ibge,
                        nome,
                        uf,
                        created_at,
                        updated_at
                "#,
                &[
                    &municipio_id,
                    &municipio.codigo_ibge,
                    &municipio.nome,
                    &municipio.uf,
                ],
            )
            .await
            .map_err(PostgresErrorHandler::query_exception)?
            .ok_or_else(|| {
                PersistenceException::query(TechnicalError::new(
                    "Municipio nao encontrado durante atualizacao PostgreSQL",
                ))
            })?;

        MunicipioEntity::from_row(&row).map_err(PostgresErrorHandler::query_exception)
    }
}

fn map_rows(rows: &[tokio_postgres::Row]) -> Result<Vec<MunicipioEntity>, PersistenceException> {
    rows.iter()
        .map(MunicipioEntity::from_row)
        .collect::<Result<Vec<_>, _>>()
        .map_err(PostgresErrorHandler::query_exception)
}
