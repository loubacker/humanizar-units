use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MunicipioDto {
    municipio_id: Option<Uuid>,
    codigo_ibge: Option<String>,
    nome: Option<String>,
    uf: Option<String>,
}

impl MunicipioDto {
    pub fn new(
        municipio_id: Option<Uuid>,
        codigo_ibge: Option<String>,
        nome: Option<String>,
        uf: Option<String>,
    ) -> Self {
        Self {
            municipio_id,
            codigo_ibge,
            nome,
            uf,
        }
    }

    pub const fn municipio_id(&self) -> Option<Uuid> {
        self.municipio_id
    }

    pub fn codigo_ibge(&self) -> Option<&str> {
        self.codigo_ibge.as_deref()
    }

    pub fn nome(&self) -> Option<&str> {
        self.nome.as_deref()
    }

    pub fn uf(&self) -> Option<&str> {
        self.uf.as_deref()
    }
}
