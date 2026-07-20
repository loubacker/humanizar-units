use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitDto {
    unit_id: Option<Uuid>,
    municipio_id: Option<Uuid>,
    unit_name: Option<String>,
    razao_social: Option<String>,
    endereco: Option<String>,
    numero: Option<String>,
    complemento: Option<String>,
    bairro: Option<String>,
    cep: Option<String>,
    cnpj: Option<String>,
}

impl UnitDto {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        unit_id: Option<Uuid>,
        municipio_id: Option<Uuid>,
        unit_name: Option<String>,
        razao_social: Option<String>,
        endereco: Option<String>,
        numero: Option<String>,
        complemento: Option<String>,
        bairro: Option<String>,
        cep: Option<String>,
        cnpj: Option<String>,
    ) -> Self {
        Self {
            unit_id,
            municipio_id,
            unit_name,
            razao_social,
            endereco,
            numero,
            complemento,
            bairro,
            cep,
            cnpj,
        }
    }

    pub const fn unit_id(&self) -> Option<Uuid> {
        self.unit_id
    }

    pub const fn municipio_id(&self) -> Option<Uuid> {
        self.municipio_id
    }

    pub fn unit_name(&self) -> Option<&str> {
        self.unit_name.as_deref()
    }

    pub fn razao_social(&self) -> Option<&str> {
        self.razao_social.as_deref()
    }

    pub fn endereco(&self) -> Option<&str> {
        self.endereco.as_deref()
    }

    pub fn numero(&self) -> Option<&str> {
        self.numero.as_deref()
    }

    pub fn complemento(&self) -> Option<&str> {
        self.complemento.as_deref()
    }

    pub fn bairro(&self) -> Option<&str> {
        self.bairro.as_deref()
    }

    pub fn cep(&self) -> Option<&str> {
        self.cep.as_deref()
    }

    pub fn cnpj(&self) -> Option<&str> {
        self.cnpj.as_deref()
    }
}
