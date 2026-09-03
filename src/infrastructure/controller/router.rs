use axum::Router;
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};

use crate::infrastructure::config::SecurityConfig;

use super::health_controller;
use super::http_observability::HttpObservability;
use super::municipio::municipio_controller;
use super::state::ApplicationState;
use super::unit::unit_controller;

pub fn create_router(state: ApplicationState, security: &SecurityConfig) -> Router {
    let health_routes = HttpObservability::apply_healthcheck(
        Router::new().route("/health", get(health_controller::health)),
    );
    let authenticated_routes = security.protect_authenticated(
        Router::new()
            .route("/api/v1/municipio", get(municipio_controller::find_all))
            .route(
                "/api/v1/municipio/{municipio_id}",
                get(municipio_controller::find_by_id),
            )
            .route(
                "/api/v1/municipio/{municipio_id}/units",
                get(unit_controller::find_by_municipio_id),
            )
            .route("/api/v1/units", get(unit_controller::find_by_ids))
            .with_state(state.clone()),
    );
    let administrator_routes = security.protect_administrator(
        Router::new()
            .route(
                "/api/v1/municipio/register",
                post(municipio_controller::create),
            )
            .route(
                "/api/v1/municipio/update/{municipio_id}",
                put(municipio_controller::update),
            )
            .route(
                "/api/v1/municipio/delete/{municipio_id}",
                delete(municipio_controller::delete),
            )
            .route(
                "/api/v1/municipio/{municipio_id}/units/register",
                post(unit_controller::create),
            )
            .route(
                "/api/v1/municipio/{municipio_id}/units/update/{unit_id}",
                put(unit_controller::update),
            )
            .route(
                "/api/v1/municipio/{municipio_id}/units/delete/{unit_id}",
                delete(unit_controller::delete),
            )
            .with_state(state),
    );
    health_routes.merge(HttpObservability::apply(
        authenticated_routes
            .merge(administrator_routes)
            .fallback(StatusCode::NOT_FOUND),
    ))
}
