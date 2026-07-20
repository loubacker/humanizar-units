use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    id: Option<Uuid>,
    municipio_id: Uuid,
    unit_name: String,
    razao_social: String,
    endereco: String,
    numero: String,
    complemento: Option<String>,
    bairro: String,
    cep: String,
    cnpj: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

impl Unit {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        municipio_id: Uuid,
        unit_name: impl Into<String>,
        razao_social: impl Into<String>,
        endereco: impl Into<String>,
        numero: impl Into<String>,
        complemento: Option<String>,
        bairro: impl Into<String>,
        cep: impl Into<String>,
        cnpj: impl Into<String>,
    ) -> Self {
        Self {
            id: None,
            municipio_id,
            unit_name: unit_name.into(),
            razao_social: razao_social.into(),
            endereco: endereco.into(),
            numero: numero.into(),
            complemento,
            bairro: bairro.into(),
            cep: cep.into(),
            cnpj: cnpj.into(),
            created_at: None,
            updated_at: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn restore(
        id: Option<Uuid>,
        municipio_id: Uuid,
        unit_name: impl Into<String>,
        razao_social: impl Into<String>,
        endereco: impl Into<String>,
        numero: impl Into<String>,
        complemento: Option<String>,
        bairro: impl Into<String>,
        cep: impl Into<String>,
        cnpj: impl Into<String>,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id,
            municipio_id,
            unit_name: unit_name.into(),
            razao_social: razao_social.into(),
            endereco: endereco.into(),
            numero: numero.into(),
            complemento,
            bairro: bairro.into(),
            cep: cep.into(),
            cnpj: cnpj.into(),
            created_at,
            updated_at,
        }
    }

    pub const fn id(&self) -> Option<Uuid> {
        self.id
    }

    pub const fn municipio_id(&self) -> Uuid {
        self.municipio_id
    }

    pub fn unit_name(&self) -> &str {
        &self.unit_name
    }

    pub fn razao_social(&self) -> &str {
        &self.razao_social
    }

    pub fn endereco(&self) -> &str {
        &self.endereco
    }

    pub fn numero(&self) -> &str {
        &self.numero
    }

    pub fn complemento(&self) -> Option<&str> {
        self.complemento.as_deref()
    }

    pub fn bairro(&self) -> &str {
        &self.bairro
    }

    pub fn cep(&self) -> &str {
        &self.cep
    }

    pub fn cnpj(&self) -> &str {
        &self.cnpj
    }

    pub const fn created_at(&self) -> Option<NaiveDateTime> {
        self.created_at
    }

    pub const fn updated_at(&self) -> Option<NaiveDateTime> {
        self.updated_at
    }
}
