use axum::Json;
use axum::extract::{OriginalUri, State};
use axum::http::StatusCode;
use uuid::Uuid;

use crate::application::dto::MunicipioDto;
use crate::infrastructure::controller::extractor::{ApiJson, ApiPath};
use crate::infrastructure::controller::handler::UnitHttpError;
use crate::infrastructure::controller::result_ext::ApplicationResultExt;
use crate::infrastructure::controller::state::ApplicationState;

pub async fn find_all(
    State(state): State<ApplicationState>,
    OriginalUri(uri): OriginalUri,
) -> Result<Json<Vec<MunicipioDto>>, UnitHttpError> {
    state
        .municipio_service()
        .find_all()
        .await
        .for_uri(&uri)
        .map(Json)
}

pub async fn find_by_id(
    State(state): State<ApplicationState>,
    OriginalUri(uri): OriginalUri,
    ApiPath(municipio_id): ApiPath<Uuid>,
) -> Result<Json<MunicipioDto>, UnitHttpError> {
    state
        .municipio_service()
        .find_by_id(municipio_id)
        .await
        .for_uri(&uri)
        .map(Json)
}

pub async fn create(
    State(state): State<ApplicationState>,
    OriginalUri(uri): OriginalUri,
    ApiJson(dto): ApiJson<MunicipioDto>,
) -> Result<(StatusCode, Json<MunicipioDto>), UnitHttpError> {
    let dto = state.municipio_service().create(dto).await.for_uri(&uri)?;

    Ok((StatusCode::CREATED, Json(dto)))
}

pub async fn update(
    State(state): State<ApplicationState>,
    OriginalUri(uri): OriginalUri,
    ApiPath(municipio_id): ApiPath<Uuid>,
    ApiJson(dto): ApiJson<MunicipioDto>,
) -> Result<Json<MunicipioDto>, UnitHttpError> {
    state
        .municipio_service()
        .update(municipio_id, dto)
        .await
        .for_uri(&uri)
        .map(Json)
}

pub async fn delete(
    State(state): State<ApplicationState>,
    OriginalUri(uri): OriginalUri,
    ApiPath(municipio_id): ApiPath<Uuid>,
) -> Result<String, UnitHttpError> {
    state
        .municipio_service()
        .delete(municipio_id)
        .await
        .for_uri(&uri)?;

    Ok("Municipio removido com sucesso.".to_owned())
}
