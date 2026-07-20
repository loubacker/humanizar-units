use axum::Router;
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::infrastructure::config::{CorsConfig, SecurityConfig};

use super::health_controller;
use super::municipio_controller;
use super::state::ApplicationState;
use super::unit_controller;

pub fn create_router(
    state: ApplicationState,
    security: &SecurityConfig,
    cors: &CorsConfig,
) -> Router {
    let public_routes = Router::new().route("/health", get(health_controller::health));
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
    let router = public_routes
        .merge(authenticated_routes)
        .merge(administrator_routes)
        .fallback(StatusCode::NOT_FOUND)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .level(Level::INFO)
                        .include_headers(false),
                )
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );

    cors.apply(router)
}
