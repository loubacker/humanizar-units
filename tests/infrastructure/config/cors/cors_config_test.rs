use axum::Router;
use axum::body::Body;
use axum::http::header::{
    ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_MAX_AGE, ACCESS_CONTROL_REQUEST_HEADERS,
    ACCESS_CONTROL_REQUEST_METHOD, ORIGIN,
};
use axum::http::{Request, StatusCode};
use axum::routing::get;
use humanizar_units::infrastructure::config::{ApplicationEnvironment, CorsConfig};
use tower::ServiceExt;

#[tokio::test]
async fn development_allows_localhost_and_loopback_with_dynamic_ports() {
    let app = test_router(&CorsConfig::development());

    for origin in ["http://localhost:3000", "http://127.0.0.1:5173"] {
        let response = app
            .clone()
            .oneshot(request("GET", origin))
            .await
            .expect("a requisição CORS deve ser processada");

        assert_eq!(StatusCode::OK, response.status());
        assert_eq!(
            origin,
            response
                .headers()
                .get(ACCESS_CONTROL_ALLOW_ORIGIN)
                .expect("a origem local deve ser permitida")
        );
        assert_eq!(
            "true",
            response
                .headers()
                .get(ACCESS_CONTROL_ALLOW_CREDENTIALS)
                .expect("credentials deve estar habilitado")
        );
    }
}

#[tokio::test]
async fn development_rejects_origins_outside_local_policy() {
    let response = test_router(&CorsConfig::development())
        .oneshot(request("GET", "https://external.example"))
        .await
        .expect("a requisição CORS deve ser processada");

    assert_eq!(StatusCode::OK, response.status());
    assert!(
        response
            .headers()
            .get(ACCESS_CONTROL_ALLOW_ORIGIN)
            .is_none()
    );
}

#[tokio::test]
async fn production_uses_only_the_explicit_allowlist() {
    let config = CorsConfig::production(["https://admin.humanizar.test"])
        .expect("a allowlist de produção deve ser válida");
    let app = test_router(&config);
    let allowed = app
        .clone()
        .oneshot(request("GET", "https://admin.humanizar.test"))
        .await
        .expect("a requisição permitida deve ser processada");
    let local = app
        .oneshot(request("GET", "http://localhost:3000"))
        .await
        .expect("a requisição local deve ser processada");

    assert_eq!(
        "https://admin.humanizar.test",
        allowed
            .headers()
            .get(ACCESS_CONTROL_ALLOW_ORIGIN)
            .expect("a origem configurada deve ser permitida")
    );
    assert!(local.headers().get(ACCESS_CONTROL_ALLOW_ORIGIN).is_none());
}

#[tokio::test]
async fn preflight_exposes_the_java_methods_headers_and_max_age() {
    let request = Request::builder()
        .method("OPTIONS")
        .uri("/resource")
        .header(ORIGIN, "http://localhost:3000")
        .header(ACCESS_CONTROL_REQUEST_METHOD, "POST")
        .header(
            ACCESS_CONTROL_REQUEST_HEADERS,
            "authorization,x-client-id,x-correlation-id,x-page-size",
        )
        .body(Body::empty())
        .expect("a requisição preflight deve ser válida");
    let response = test_router(&CorsConfig::development())
        .oneshot(request)
        .await
        .expect("o preflight deve ser processado");
    let methods = response
        .headers()
        .get(ACCESS_CONTROL_ALLOW_METHODS)
        .expect("os métodos devem ser informados")
        .to_str()
        .expect("os métodos devem ser texto");
    let headers = response
        .headers()
        .get(ACCESS_CONTROL_ALLOW_HEADERS)
        .expect("os headers devem ser informados")
        .to_str()
        .expect("os headers devem ser texto")
        .to_ascii_lowercase();

    assert_eq!(StatusCode::OK, response.status());
    for method in ["GET", "POST", "PUT", "DELETE", "OPTIONS"] {
        assert!(methods.contains(method));
    }
    for header in [
        "authorization",
        "x-client-id",
        "x-correlation-id",
        "x-page-size",
    ] {
        assert!(headers.contains(header));
    }
    assert_eq!(
        "3600",
        response
            .headers()
            .get(ACCESS_CONTROL_MAX_AGE)
            .expect("o max-age deve ser informado")
    );
}

#[test]
fn production_rejects_empty_wildcard_and_path_origins() {
    assert!(CorsConfig::production::<_, &str>([]).is_err());
    assert!(CorsConfig::production(["*"]).is_err());
    assert!(CorsConfig::production(["https://admin.humanizar.test/path"]).is_err());
    assert!(CorsConfig::new(ApplicationEnvironment::Production, ["javascript:alert(1)"]).is_err());
}

fn test_router(config: &CorsConfig) -> Router {
    config.apply(Router::new().route("/resource", get(|| async { "ok" })))
}

fn request(method: &str, origin: &str) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri("/resource")
        .header(ORIGIN, origin)
        .body(Body::empty())
        .expect("a requisição deve ser válida")
}
