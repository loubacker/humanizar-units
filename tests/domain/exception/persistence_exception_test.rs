#![forbid(unsafe_code)]

use std::error::Error;

use humanizar_units::domain::exception::{PersistenceException, ReasonCodeException};
use humanizar_units::domain::model::enums::ReasonCode;

#[test]
fn persistence_exception_is_restricted_to_persistence_failure() {
    let exception = PersistenceException::query(std::io::Error::other("query failed"));

    assert_eq!(ReasonCode::PersistenceFailure, exception.reason_code());
    assert_eq!(
        ReasonCode::PersistenceFailure.message(),
        exception.message()
    );
    assert_eq!(
        ReasonCode::PersistenceFailure.message(),
        exception.to_string()
    );
}

#[test]
fn factories_define_retryability_without_changing_the_public_reason_code() {
    let transient = [
        PersistenceException::connection(std::io::Error::other("connection closed")),
        PersistenceException::transient_query(std::io::Error::other("serialization failure")),
        PersistenceException::transaction_conflict(std::io::Error::other("deadlock")),
    ];
    let terminal = [
        PersistenceException::pool_acquisition(std::io::Error::other("pool exhausted")),
        PersistenceException::query(std::io::Error::other("syntax error")),
        PersistenceException::transaction(std::io::Error::other("constraint violation")),
        PersistenceException::timeout(std::io::Error::other("deadline exceeded")),
    ];

    assert!(transient.iter().all(PersistenceException::is_retryable));
    assert!(terminal.iter().all(|exception| !exception.is_retryable()));
    assert!(
        transient
            .iter()
            .chain(&terminal)
            .all(|exception| exception.reason_code() == ReasonCode::PersistenceFailure)
    );
}

#[test]
fn technical_context_and_source_are_preserved_in_the_error_chain() {
    let exception = PersistenceException::transient_query(std::io::Error::other("SQLSTATE 40001"));
    let context = exception.source().expect("o contexto técnico deve existir");
    let source = context.source().expect("a causa original deve existir");

    assert_eq!(
        "Falha transitória ao executar consulta PostgreSQL",
        context.to_string()
    );
    assert_eq!("SQLSTATE 40001", source.to_string());
}

#[test]
fn persistence_exception_satisfies_the_shared_exception_contract() {
    fn assert_contract(exception: &impl ReasonCodeException) {
        assert_eq!(ReasonCode::PersistenceFailure, exception.reason_code());
    }

    assert_contract(&PersistenceException::query(std::io::Error::other(
        "query failed",
    )));
}
