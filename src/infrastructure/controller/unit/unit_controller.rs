use axum::Json;
use axum::extract::{OriginalUri, State};
use axum::http::StatusCode;
use uuid::Uuid;

use crate::application::dto::{UnitDto, UnitIdsQueryDto};
use crate::infrastructure::controller::extractor::{ApiJson, ApiPath, ApiQuery};
use crate::infrastructure::controller::handler::UnitHttpError;
use crate::infrastructure::controller::result_ext::ApplicationResultExt;
use crate::infrastructure::controller::state::ApplicationState;

pub async fn find_by_municipio_id(
    State(state): State<ApplicationState>,
    OriginalUri(uri): OriginalUri,
    ApiPath(municipio_id): ApiPath<Uuid>,
) -> Result<Json<Vec<UnitDto>>, UnitHttpError> {
    state
        .unit_service()
        .find_by_municipio_id(municipio_id)
        .await
        .for_uri(&uri)
        .map(Json)
}

pub async fn find_by_ids(
    State(state): State<ApplicationState>,
    OriginalUri(uri): OriginalUri,
    ApiQuery(query): ApiQuery<UnitIdsQueryDto>,
) -> Result<Json<Vec<UnitDto>>, UnitHttpError> {
    state
        .unit_service()
        .find_by_ids(query)
        .await
        .for_uri(&uri)
        .map(Json)
}

pub async fn create(
    State(state): State<ApplicationState>,
    OriginalUri(uri): OriginalUri,
    ApiPath(municipio_id): ApiPath<Uuid>,
    ApiJson(dto): ApiJson<UnitDto>,
) -> Result<(StatusCode, Json<UnitDto>), UnitHttpError> {
    let dto = state
        .unit_service()
        .create(municipio_id, dto)
        .await
        .for_uri(&uri)?;

    Ok((StatusCode::CREATED, Json(dto)))
}

pub async fn update(
    State(state): State<ApplicationState>,
    OriginalUri(uri): OriginalUri,
    ApiPath((municipio_id, unit_id)): ApiPath<(Uuid, Uuid)>,
    ApiJson(dto): ApiJson<UnitDto>,
) -> Result<Json<UnitDto>, UnitHttpError> {
    state
        .unit_service()
        .update(municipio_id, unit_id, dto)
        .await
        .for_uri(&uri)
        .map(Json)
}

pub async fn delete(
    State(state): State<ApplicationState>,
    OriginalUri(uri): OriginalUri,
    ApiPath((municipio_id, unit_id)): ApiPath<(Uuid, Uuid)>,
) -> Result<String, UnitHttpError> {
    state
        .unit_service()
        .delete(municipio_id, unit_id)
        .await
        .for_uri(&uri)?;

    Ok("Unidade excluida com sucesso.".to_owned())
}
