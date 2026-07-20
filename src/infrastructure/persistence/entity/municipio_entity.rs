use chrono::NaiveDateTime;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MunicipioEntity {
    pub(crate) id: Option<Uuid>,
    pub(crate) codigo_ibge: String,
    pub(crate) nome: String,
    pub(crate) uf: String,
    pub(crate) created_at: Option<NaiveDateTime>,
    pub(crate) updated_at: Option<NaiveDateTime>,
}

impl MunicipioEntity {
    pub(crate) fn new(
        id: Option<Uuid>,
        codigo_ibge: String,
        nome: String,
        uf: String,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id,
            codigo_ibge,
            nome,
            uf,
            created_at,
            updated_at,
        }
    }

    pub(crate) fn from_row(row: &Row) -> Result<Self, tokio_postgres::Error> {
        Ok(Self {
            id: Some(row.try_get("id")?),
            codigo_ibge: row.try_get("codigo_ibge")?,
            nome: row.try_get("nome")?,
            uf: row.try_get("uf")?,
            created_at: Some(row.try_get("created_at")?),
            updated_at: Some(row.try_get("updated_at")?),
        })
    }
}
