use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::domain::model::enums::ReasonCode;

use super::PersistenceException;
use super::{ReasonCodeException, TechnicalError};

#[derive(Debug)]
pub struct UnitException {
    reason_code: ReasonCode,
    details: TechnicalError,
}

impl UnitException {
    pub fn new(reason_code: ReasonCode) -> Self {
        Self {
            reason_code,
            details: TechnicalError::new(reason_code.message()),
        }
    }

    pub fn with_message(reason_code: ReasonCode, message: impl Into<String>) -> Self {
        Self {
            reason_code,
            details: TechnicalError::new(resolve_message(reason_code, message.into())),
        }
    }

    pub fn with_source<E>(reason_code: ReasonCode, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self {
            reason_code,
            details: TechnicalError::with_source(reason_code.message(), source),
        }
    }

    pub fn with_message_and_source<E>(
        reason_code: ReasonCode,
        message: impl Into<String>,
        source: E,
    ) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self {
            reason_code,
            details: TechnicalError::with_source(
                resolve_message(reason_code, message.into()),
                source,
            ),
        }
    }

    pub const fn reason_code(&self) -> ReasonCode {
        self.reason_code
    }

    pub fn message(&self) -> &str {
        self.details.message()
    }

    pub const fn is_retryable(&self) -> bool {
        self.reason_code.is_retryable()
    }
}

impl Display for UnitException {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.details, formatter)
    }
}

impl Error for UnitException {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.details.source()
    }
}

impl ReasonCodeException for UnitException {
    fn reason_code(&self) -> ReasonCode {
        UnitException::reason_code(self)
    }

    fn message(&self) -> &str {
        UnitException::message(self)
    }
}

impl From<PersistenceException> for UnitException {
    fn from(exception: PersistenceException) -> Self {
        Self::with_source(ReasonCode::PersistenceFailure, exception)
    }
}

fn resolve_message(reason_code: ReasonCode, message: String) -> String {
    if message.trim().is_empty() {
        return reason_code.message().to_owned();
    }

    message
}
