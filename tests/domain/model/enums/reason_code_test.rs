#![forbid(unsafe_code)]

use humanizar_units::domain::model::enums::ReasonCode;

#[test]
fn should_expose_stable_codes_messages_and_retryability() {
    let cases = [
        (
            ReasonCode::InvalidRequest,
            "INVALID_REQUEST",
            "Requisição inválida.",
            false,
        ),
        (
            ReasonCode::ValidationError,
            "VALIDATION_ERROR",
            "Falha de validação da solicitação.",
            false,
        ),
        (
            ReasonCode::AuthenticationFailure,
            "AUTHENTICATION_FAILURE",
            "Falha de autenticação: credenciais ausentes ou inválidas.",
            false,
        ),
        (
            ReasonCode::AuthorizationFailure,
            "AUTHORIZATION_FAILURE",
            "Falha de autorização: acesso negado para este recurso.",
            false,
        ),
        (
            ReasonCode::UnitNotFound,
            "UNIT_NOT_FOUND",
            "Unidade não encontrada.",
            false,
        ),
        (
            ReasonCode::MunicipioNotFound,
            "MUNICIPIO_NOT_FOUND",
            "Município não encontrado.",
            false,
        ),
        (
            ReasonCode::UnitDuplicated,
            "UNIT_DUPLICATED",
            "Unidade com este CNPJ já cadastrada no município.",
            false,
        ),
        (
            ReasonCode::MunicipioDuplicated,
            "MUNICIPIO_DUPLICATED",
            "Município já cadastrado.",
            false,
        ),
        (
            ReasonCode::MunicipioHasUnits,
            "MUNICIPIO_HAS_UNITS",
            "Município possui unidades vinculadas.",
            false,
        ),
        (
            ReasonCode::UnexpectedError,
            "UNEXPECTED_ERROR",
            "Erro inesperado.",
            false,
        ),
        (
            ReasonCode::PersistenceFailure,
            "PERSISTENCE_FAILURE",
            "Falha de persistência no banco de dados.",
            true,
        ),
    ];

    for (reason_code, code, message, retryable) in cases {
        assert_eq!(reason_code.code(), code);
        assert_eq!(reason_code.message(), message);
        assert_eq!(reason_code.is_retryable(), retryable);
    }
}
