#![forbid(unsafe_code)]

#[path = "../../../src/infrastructure/controller/http_observability.rs"]
mod http_observability;
#[path = "../../support/tracing_capture.rs"]
mod tracing_capture;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::routing::get;
use http_observability::HttpObservability;
use tower::ServiceExt;
use tracing_capture::CapturedEvents;
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;

#[tokio::test(flavor = "current_thread")]
async fn healthcheck_success_is_silent_and_failure_remains_visible() {
    let captured = CapturedEvents::default();
    let subscriber = Registry::default().with(captured.clone());
    let subscriber_guard = tracing::subscriber::set_default(subscriber);
    let healthy_router = HttpObservability::apply_healthcheck(
        Router::new().route("/health", get(|| async { StatusCode::OK })),
    );

    let healthy_response = request(healthy_router, "/health").await;
    assert_eq!(StatusCode::OK, healthy_response);
    assert!(captured.events().is_empty());

    let unhealthy_router = HttpObservability::apply_healthcheck(
        Router::new().route("/health", get(|| async { StatusCode::SERVICE_UNAVAILABLE })),
    );
    let unhealthy_response = request(unhealthy_router, "/health").await;
    assert_eq!(StatusCode::SERVICE_UNAVAILABLE, unhealthy_response);
    drop(subscriber_guard);

    let events: Vec<_> = captured
        .events()
        .into_iter()
        .filter(|event| event.fields.contains_key("status"))
        .collect();
    assert_eq!(1, events.len());
    assert_eq!("INFO", events[0].level);
    assert_eq!(Some(&"503".to_owned()), events[0].fields.get("status"));
    assert!(events[0].fields.contains_key("latency"));
}

#[tokio::test(flavor = "current_thread")]
async fn regular_route_keeps_the_existing_access_log() {
    let captured = CapturedEvents::default();
    let subscriber = Registry::default().with(captured.clone());
    let subscriber_guard = tracing::subscriber::set_default(subscriber);
    let router = HttpObservability::apply(
        Router::new().route("/resource", get(|| async { StatusCode::NO_CONTENT })),
    );

    let response = request(router, "/resource").await;
    assert_eq!(StatusCode::NO_CONTENT, response);
    drop(subscriber_guard);

    let events: Vec<_> = captured
        .events()
        .into_iter()
        .filter(|event| event.fields.contains_key("status"))
        .collect();
    assert_eq!(1, events.len());
    assert_eq!("INFO", events[0].level);
    assert_eq!(Some(&"204".to_owned()), events[0].fields.get("status"));
    assert!(events[0].fields.contains_key("latency"));
}

async fn request(router: Router, path: &str) -> StatusCode {
    router
        .oneshot(
            Request::builder()
                .uri(path)
                .body(Body::empty())
                .expect("a requisição deve ser válida"),
        )
        .await
        .expect("o router deve responder")
        .status()
}
