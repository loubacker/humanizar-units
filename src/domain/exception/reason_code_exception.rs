use std::error::Error;

use crate::domain::model::enums::ReasonCode;

pub trait ReasonCodeException: Error + Send + Sync {
    fn reason_code(&self) -> ReasonCode;

    fn message(&self) -> &str;
}
