use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::domain::model::enums::ReasonCode;

use super::{ReasonCodeException, TechnicalError, UnitException};

#[derive(Debug)]
pub struct PersistenceException {
    exception: UnitException,
    retryable: bool,
}

impl PersistenceException {
    pub fn connection(source: impl Error + Send + Sync + 'static) -> Self {
        Self::with_context("Falha ao conectar ao PostgreSQL", true, source)
    }

    pub fn pool_acquisition(source: impl Error + Send + Sync + 'static) -> Self {
        Self::with_context("Falha ao adquirir conexão do pool", false, source)
    }

    pub fn query(source: impl Error + Send + Sync + 'static) -> Self {
        Self::with_context("Falha ao executar consulta PostgreSQL", false, source)
    }

    pub fn transient_query(source: impl Error + Send + Sync + 'static) -> Self {
        Self::with_context(
            "Falha transitória ao executar consulta PostgreSQL",
            true,
            source,
        )
    }

    pub fn transaction(source: impl Error + Send + Sync + 'static) -> Self {
        Self::with_context("Falha ao executar transação PostgreSQL", false, source)
    }

    pub fn transaction_conflict(source: impl Error + Send + Sync + 'static) -> Self {
        Self::with_context("Conflito transitório na transação PostgreSQL", true, source)
    }

    pub fn timeout(source: impl Error + Send + Sync + 'static) -> Self {
        Self::with_context(
            "Tempo limite da operação PostgreSQL excedido",
            false,
            source,
        )
    }

    pub const fn reason_code(&self) -> ReasonCode {
        self.exception.reason_code()
    }

    pub fn message(&self) -> &str {
        self.exception.message()
    }

    pub const fn is_retryable(&self) -> bool {
        self.retryable
    }

    fn with_context(
        context: impl Into<String>,
        retryable: bool,
        source: impl Error + Send + Sync + 'static,
    ) -> Self {
        let technical_error = TechnicalError::with_source(context, source);

        Self {
            exception: UnitException::with_source(ReasonCode::PersistenceFailure, technical_error),
            retryable,
        }
    }
}

impl ReasonCodeException for PersistenceException {
    fn reason_code(&self) -> ReasonCode {
        PersistenceException::reason_code(self)
    }

    fn message(&self) -> &str {
        PersistenceException::message(self)
    }
}

impl Display for PersistenceException {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.exception, formatter)
    }
}

impl Error for PersistenceException {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.exception.source()
    }
}
