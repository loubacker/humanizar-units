use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponseDto<T> {
    code: String,
    success: bool,
    correlation_id: String,
    message: String,
    data: T,
    timestamp: DateTime<Utc>,
}

impl<T> SuccessResponseDto<T> {
    pub fn new(
        code: impl Into<String>,
        correlation_id: impl Into<String>,
        message: impl Into<String>,
        data: T,
    ) -> Self {
        Self::with_timestamp(code, correlation_id, message, data, Utc::now())
    }

    pub fn with_timestamp(
        code: impl Into<String>,
        correlation_id: impl Into<String>,
        message: impl Into<String>,
        data: T,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            code: code.into(),
            success: true,
            correlation_id: correlation_id.into(),
            message: message.into(),
            data,
            timestamp,
        }
    }
}
