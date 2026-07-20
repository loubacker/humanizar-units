use std::error::Error as StdError;

use tokio_postgres::Error;
use tokio_postgres::error::SqlState;

use crate::domain::exception::{PersistenceException, UnitException};
use crate::domain::model::enums::ReasonCode;

const UNIT_CNPJ_CONSTRAINT: &str = "uk_units_municipio_cnpj";
const MUNICIPIO_CODIGO_IBGE_CONSTRAINT: &str = "uk_municipio_codigo_ibge";

pub struct PostgresErrorHandler;

impl PostgresErrorHandler {
    pub fn is_transient(error: &Error) -> bool {
        error.is_closed() || error.code().is_some_and(Self::is_transient_sql_state)
    }

    pub fn is_transient_sql_state(sql_state: &SqlState) -> bool {
        sql_state.code().starts_with("08")
            || matches!(
                sql_state,
                &SqlState::T_R_SERIALIZATION_FAILURE
                    | &SqlState::T_R_DEADLOCK_DETECTED
                    | &SqlState::LOCK_NOT_AVAILABLE
                    | &SqlState::QUERY_CANCELED
                    | &SqlState::ADMIN_SHUTDOWN
                    | &SqlState::CRASH_SHUTDOWN
                    | &SqlState::CANNOT_CONNECT_NOW
                    | &SqlState::IO_ERROR
            )
    }

    pub fn query_exception(error: Error) -> PersistenceException {
        if Self::is_transient(&error) {
            return PersistenceException::transient_query(error);
        }

        PersistenceException::query(error)
    }

    pub fn transaction_exception(error: Error) -> PersistenceException {
        if Self::is_transient(&error) {
            return PersistenceException::transaction_conflict(error);
        }

        PersistenceException::transaction(error)
    }

    pub fn write_exception(error: PersistenceException) -> UnitException {
        let reason_code = postgres_error(&error)
            .and_then(Error::as_db_error)
            .and_then(|database_error| database_error.constraint())
            .and_then(Self::constraint_reason_code);

        if let Some(reason_code) = reason_code {
            return UnitException::with_source(reason_code, error);
        }

        error.into()
    }

    pub fn constraint_reason_code(constraint: &str) -> Option<ReasonCode> {
        match constraint {
            UNIT_CNPJ_CONSTRAINT => Some(ReasonCode::UnitDuplicated),
            MUNICIPIO_CODIGO_IBGE_CONSTRAINT => Some(ReasonCode::MunicipioDuplicated),
            _ => None,
        }
    }
}

fn postgres_error<'a>(error: &'a (dyn StdError + 'static)) -> Option<&'a Error> {
    let mut current = Some(error);

    while let Some(source) = current {
        if let Some(postgres_error) = source.downcast_ref::<Error>() {
            return Some(postgres_error);
        }

        current = source.source();
    }

    None
}
