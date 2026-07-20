use axum::Json;

use crate::infrastructure::controller::dto::HealthResponseDto;

pub async fn health() -> Json<HealthResponseDto> {
    Json(HealthResponseDto::up())
}
