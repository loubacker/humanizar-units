use chrono::NaiveDateTime;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitEntity {
    pub(crate) id: Option<Uuid>,
    pub(crate) municipio_id: Uuid,
    pub(crate) unit_name: String,
    pub(crate) razao_social: String,
    pub(crate) endereco: String,
    pub(crate) numero: String,
    pub(crate) complemento: Option<String>,
    pub(crate) bairro: String,
    pub(crate) cep: String,
    pub(crate) cnpj: String,
    pub(crate) created_at: Option<NaiveDateTime>,
    pub(crate) updated_at: Option<NaiveDateTime>,
}

impl UnitEntity {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        id: Option<Uuid>,
        municipio_id: Uuid,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
        unit_name: String,
        razao_social: String,
        endereco: String,
        numero: String,
        complemento: Option<String>,
        bairro: String,
        cep: String,
        cnpj: String,
    ) -> Self {
        Self {
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
            updated_at,
        }
    }

    pub(crate) fn from_row(row: &Row) -> Result<Self, tokio_postgres::Error> {
        Ok(Self {
            id: Some(row.try_get("id")?),
            municipio_id: row.try_get("municipio_id")?,
            unit_name: row.try_get("unit_name")?,
            razao_social: row.try_get("razao_social")?,
            endereco: row.try_get("endereco")?,
            numero: row.try_get("numero")?,
            complemento: row.try_get("complemento")?,
            bairro: row.try_get("bairro")?,
            cep: row.try_get("cep")?,
            cnpj: row.try_get("cnpj")?,
            created_at: Some(row.try_get("created_at")?),
            updated_at: Some(row.try_get("updated_at")?),
        })
    }
}
