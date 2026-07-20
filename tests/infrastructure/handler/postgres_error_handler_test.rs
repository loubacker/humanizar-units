#![forbid(unsafe_code)]

use humanizar_units::domain::model::enums::ReasonCode;
use humanizar_units::infrastructure::handler::PostgresErrorHandler;
use tokio_postgres::error::SqlState;

#[test]
fn connection_sqlstate_class_is_transient() {
    for code in ["08000", "08001", "08003", "08006", "08007", "08P01"] {
        assert!(PostgresErrorHandler::is_transient_sql_state(
            &SqlState::from_code(code)
        ));
    }
}

#[test]
fn explicit_transient_sqlstates_are_classified_for_read_retry() {
    let retryable = [
        &SqlState::T_R_SERIALIZATION_FAILURE,
        &SqlState::T_R_DEADLOCK_DETECTED,
        &SqlState::LOCK_NOT_AVAILABLE,
        &SqlState::QUERY_CANCELED,
        &SqlState::ADMIN_SHUTDOWN,
        &SqlState::CRASH_SHUTDOWN,
        &SqlState::CANNOT_CONNECT_NOW,
        &SqlState::IO_ERROR,
    ];

    assert!(
        retryable
            .into_iter()
            .all(PostgresErrorHandler::is_transient_sql_state)
    );
}

#[test]
fn integrity_authentication_syntax_and_resource_errors_are_not_retried() {
    let terminal = [
        &SqlState::UNIQUE_VIOLATION,
        &SqlState::FOREIGN_KEY_VIOLATION,
        &SqlState::INVALID_PASSWORD,
        &SqlState::SYNTAX_ERROR,
        &SqlState::TOO_MANY_CONNECTIONS,
        &SqlState::DISK_FULL,
    ];

    assert!(
        terminal
            .into_iter()
            .all(|state| !PostgresErrorHandler::is_transient_sql_state(state))
    );
}

#[test]
fn known_unique_constraints_map_to_stable_domain_reason_codes() {
    assert_eq!(
        Some(ReasonCode::UnitDuplicated),
        PostgresErrorHandler::constraint_reason_code("uk_units_municipio_cnpj")
    );
    assert_eq!(
        Some(ReasonCode::MunicipioDuplicated),
        PostgresErrorHandler::constraint_reason_code("uk_municipio_codigo_ibge")
    );
    assert_eq!(
        None,
        PostgresErrorHandler::constraint_reason_code("unknown_constraint")
    );
}
