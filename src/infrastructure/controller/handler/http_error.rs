use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::domain::exception::{
    PersistenceException, ReasonCodeException, SecurityException, UnitException,
};
use crate::domain::model::enums::ReasonCode;
use crate::infrastructure::controller::dto::ErrorResponseDto;

#[derive(Debug)]
pub struct HttpError<E> {
    exception: E,
    path: String,
}

impl<E> HttpError<E> {
    pub fn new(exception: E, path: impl Into<String>) -> Self {
        Self {
            exception,
            path: path.into(),
        }
    }
}

impl<E> IntoResponse for HttpError<E>
where
    E: ReasonCodeException,
{
    fn into_response(self) -> Response {
        let reason_code = self.exception.reason_code();
        let status = status_for(reason_code);

        log_exception(status, &self.path, &self.exception);

        let response = ErrorResponseDto::new(
            status.as_u16(),
            status.canonical_reason().unwrap_or("Internal Server Error"),
            reason_code.code(),
            self.exception.message(),
            self.path,
        );

        (status, Json(response)).into_response()
    }
}

pub type UnitHttpError = HttpError<UnitException>;
pub type SecurityHttpError = HttpError<SecurityException>;
pub type PersistenceHttpError = HttpError<PersistenceException>;

const fn status_for(reason_code: ReasonCode) -> StatusCode {
    match reason_code {
        ReasonCode::InvalidRequest | ReasonCode::ValidationError => StatusCode::BAD_REQUEST,
        ReasonCode::AuthenticationFailure => StatusCode::UNAUTHORIZED,
        ReasonCode::AuthorizationFailure => StatusCode::FORBIDDEN,
        ReasonCode::UnitNotFound | ReasonCode::MunicipioNotFound => StatusCode::NOT_FOUND,
        ReasonCode::UnitDuplicated
        | ReasonCode::MunicipioDuplicated
        | ReasonCode::MunicipioHasUnits => StatusCode::CONFLICT,
        ReasonCode::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        ReasonCode::PersistenceFailure => StatusCode::SERVICE_UNAVAILABLE,
    }
}

fn log_exception<E>(status: StatusCode, path: &str, exception: &E)
where
    E: ReasonCodeException,
{
    if status.is_server_error() {
        tracing::error!(
            reason_code = exception.reason_code().code(),
            status = status.as_u16(),
            path,
            message = exception.message(),
            source = ?std::error::Error::source(exception)
        );
        return;
    }

    tracing::warn!(
        reason_code = exception.reason_code().code(),
        status = status.as_u16(),
        path,
        message = exception.message(),
        source = ?std::error::Error::source(exception)
    );
}
