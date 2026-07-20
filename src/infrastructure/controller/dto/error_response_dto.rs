use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponseDto {
    timestamp: DateTime<Utc>,
    status: u16,
    error: String,
    reason_code: String,
    message: String,
    path: String,
}

impl ErrorResponseDto {
    pub fn new(
        status: u16,
        error: impl Into<String>,
        reason_code: impl Into<String>,
        message: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        Self::with_timestamp(Utc::now(), status, error, reason_code, message, path)
    }

    pub fn with_timestamp(
        timestamp: DateTime<Utc>,
        status: u16,
        error: impl Into<String>,
        reason_code: impl Into<String>,
        message: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        Self {
            timestamp,
            status,
            error: error.into(),
            reason_code: reason_code.into(),
            message: message.into(),
            path: path.into(),
        }
    }
}
