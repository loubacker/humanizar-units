#![forbid(unsafe_code)]

use std::error::Error;
use std::io;

use humanizar_units::domain::exception::UnitException;
use humanizar_units::domain::model::enums::ReasonCode;

#[test]
fn should_use_reason_code_default_message() {
    let exception = UnitException::new(ReasonCode::UnitNotFound);

    assert_eq!(exception.reason_code(), ReasonCode::UnitNotFound);
    assert_eq!(exception.message(), "Unidade não encontrada.");
    assert_eq!(exception.to_string(), "Unidade não encontrada.");
    assert!(!exception.is_retryable());
    assert!(exception.source().is_none());
}

#[test]
fn should_use_custom_message_when_present() {
    let exception = UnitException::with_message(
        ReasonCode::ValidationError,
        "CNPJ da unidade é obrigatório.",
    );

    assert_eq!(exception.message(), "CNPJ da unidade é obrigatório.");
}

#[test]
fn should_fall_back_to_default_message_when_custom_message_is_blank() {
    let exception = UnitException::with_message(ReasonCode::InvalidRequest, "   ");

    assert_eq!(exception.message(), "Requisição inválida.");
}

#[test]
fn should_preserve_technical_source_without_changing_public_message() {
    let source = io::Error::other("database connection refused");
    let exception = UnitException::with_message_and_source(
        ReasonCode::PersistenceFailure,
        "Falha ao salvar unidade.",
        source,
    );

    assert_eq!(exception.message(), "Falha ao salvar unidade.");
    assert!(exception.is_retryable());
    assert_eq!(
        exception.source().map(ToString::to_string),
        Some("database connection refused".to_owned())
    );
}
