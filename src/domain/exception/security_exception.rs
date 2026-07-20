use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::domain::model::enums::ReasonCode;

use super::{ReasonCodeException, UnitException};

#[derive(Debug)]
pub struct SecurityException {
    exception: UnitException,
}

impl SecurityException {
    pub fn authentication() -> Self {
        Self::from_reason_code(ReasonCode::AuthenticationFailure)
    }

    pub fn authentication_with_source(source: impl Error + Send + Sync + 'static) -> Self {
        Self::from_reason_code_and_source(ReasonCode::AuthenticationFailure, source)
    }

    pub fn authorization() -> Self {
        Self::from_reason_code(ReasonCode::AuthorizationFailure)
    }

    pub fn authorization_with_source(source: impl Error + Send + Sync + 'static) -> Self {
        Self::from_reason_code_and_source(ReasonCode::AuthorizationFailure, source)
    }

    pub const fn reason_code(&self) -> ReasonCode {
        self.exception.reason_code()
    }

    pub fn message(&self) -> &str {
        self.exception.message()
    }

    fn from_reason_code(reason_code: ReasonCode) -> Self {
        Self {
            exception: UnitException::new(reason_code),
        }
    }

    fn from_reason_code_and_source(
        reason_code: ReasonCode,
        source: impl Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            exception: UnitException::with_source(reason_code, source),
        }
    }
}

impl ReasonCodeException for SecurityException {
    fn reason_code(&self) -> ReasonCode {
        SecurityException::reason_code(self)
    }

    fn message(&self) -> &str {
        SecurityException::message(self)
    }
}

impl Display for SecurityException {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.exception, formatter)
    }
}

impl Error for SecurityException {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.exception.source()
    }
}
