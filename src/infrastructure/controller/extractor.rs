use axum::Json;
use axum::extract::rejection::{JsonRejection, PathRejection};
use axum::extract::{FromRequest, FromRequestParts, Path, Request};
use axum::http::request::Parts;
use serde::de::DeserializeOwned;
use url::form_urlencoded;
use uuid::Uuid;

use crate::application::dto::UnitIdsQueryDto;
use crate::domain::exception::UnitException;
use crate::domain::model::enums::ReasonCode;
use crate::infrastructure::controller::handler::UnitHttpError;

pub struct ApiJson<T>(pub T);

impl<T, S> FromRequest<S> for ApiJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = UnitHttpError;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        let path = request.uri().path().to_owned();
        Json::<T>::from_request(request, state)
            .await
            .map(|Json(value)| Self(value))
            .map_err(|error: JsonRejection| invalid_request(error, path))
    }
}

pub struct ApiPath<T>(pub T);

impl<T, S> FromRequestParts<S> for ApiPath<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = UnitHttpError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let path = parts.uri.path().to_owned();
        Path::<T>::from_request_parts(parts, state)
            .await
            .map(|Path(value)| Self(value))
            .map_err(|error: PathRejection| invalid_request(error, path))
    }
}

pub struct ApiQuery<T>(pub T);

impl<S> FromRequestParts<S> for ApiQuery<UnitIdsQueryDto>
where
    S: Send + Sync,
{
    type Rejection = UnitHttpError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parse_unit_ids(parts.uri.query())
            .map(Self)
            .map_err(|exception| UnitHttpError::new(exception, parts.uri.path()))
    }
}

fn parse_unit_ids(query: Option<&str>) -> Result<UnitIdsQueryDto, UnitException> {
    let mut unit_ids = Vec::new();

    for (name, value) in form_urlencoded::parse(query.unwrap_or_default().as_bytes()) {
        if name != "ids" {
            continue;
        }

        for unit_id in value
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            unit_ids.push(
                Uuid::parse_str(unit_id).map_err(|error| {
                    UnitException::with_source(ReasonCode::InvalidRequest, error)
                })?,
            );
        }
    }

    Ok(UnitIdsQueryDto::new(unit_ids))
}

fn invalid_request(
    source: impl std::error::Error + Send + Sync + 'static,
    path: String,
) -> UnitHttpError {
    UnitHttpError::new(
        UnitException::with_source(ReasonCode::InvalidRequest, source),
        path,
    )
}
