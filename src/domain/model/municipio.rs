use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Municipio {
    id: Option<Uuid>,
    codigo_ibge: String,
    nome: String,
    uf: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

impl Municipio {
    pub fn new(
        codigo_ibge: impl Into<String>,
        nome: impl Into<String>,
        uf: impl Into<String>,
    ) -> Self {
        Self {
            id: None,
            codigo_ibge: codigo_ibge.into(),
            nome: nome.into(),
            uf: uf.into(),
            created_at: None,
            updated_at: None,
        }
    }

    pub fn restore(
        id: Option<Uuid>,
        codigo_ibge: impl Into<String>,
        nome: impl Into<String>,
        uf: impl Into<String>,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id,
            codigo_ibge: codigo_ibge.into(),
            nome: nome.into(),
            uf: uf.into(),
            created_at,
            updated_at,
        }
    }

    pub const fn id(&self) -> Option<Uuid> {
        self.id
    }

    pub fn codigo_ibge(&self) -> &str {
        &self.codigo_ibge
    }

    pub fn nome(&self) -> &str {
        &self.nome
    }

    pub fn uf(&self) -> &str {
        &self.uf
    }

    pub const fn created_at(&self) -> Option<NaiveDateTime> {
        self.created_at
    }

    pub const fn updated_at(&self) -> Option<NaiveDateTime> {
        self.updated_at
    }
}
