#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReasonCode {
    InvalidRequest,
    ValidationError,
    AuthenticationFailure,
    AuthorizationFailure,
    UnitNotFound,
    MunicipioNotFound,
    UnitDuplicated,
    MunicipioDuplicated,
    MunicipioHasUnits,
    UnexpectedError,
    PersistenceFailure,
}

impl ReasonCode {
    pub const fn code(self) -> &'static str {
        match self {
            Self::InvalidRequest => "INVALID_REQUEST",
            Self::ValidationError => "VALIDATION_ERROR",
            Self::AuthenticationFailure => "AUTHENTICATION_FAILURE",
            Self::AuthorizationFailure => "AUTHORIZATION_FAILURE",
            Self::UnitNotFound => "UNIT_NOT_FOUND",
            Self::MunicipioNotFound => "MUNICIPIO_NOT_FOUND",
            Self::UnitDuplicated => "UNIT_DUPLICATED",
            Self::MunicipioDuplicated => "MUNICIPIO_DUPLICATED",
            Self::MunicipioHasUnits => "MUNICIPIO_HAS_UNITS",
            Self::UnexpectedError => "UNEXPECTED_ERROR",
            Self::PersistenceFailure => "PERSISTENCE_FAILURE",
        }
    }

    pub const fn message(self) -> &'static str {
        match self {
            Self::InvalidRequest => "Requisição inválida.",
            Self::ValidationError => "Falha de validação da solicitação.",
            Self::AuthenticationFailure => {
                "Falha de autenticação: credenciais ausentes ou inválidas."
            }
            Self::AuthorizationFailure => "Falha de autorização: acesso negado para este recurso.",
            Self::UnitNotFound => "Unidade não encontrada.",
            Self::MunicipioNotFound => "Município não encontrado.",
            Self::UnitDuplicated => "Unidade com este CNPJ já cadastrada no município.",
            Self::MunicipioDuplicated => "Município já cadastrado.",
            Self::MunicipioHasUnits => "Município possui unidades vinculadas.",
            Self::UnexpectedError => "Erro inesperado.",
            Self::PersistenceFailure => "Falha de persistência no banco de dados.",
        }
    }

    pub const fn is_retryable(self) -> bool {
        matches!(self, Self::PersistenceFailure)
    }
}
