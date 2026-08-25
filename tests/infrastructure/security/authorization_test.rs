#![forbid(unsafe_code)]

mod test_support;

use axum::Router;
use axum::body::Body;
use axum::http::header::{ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, ORIGIN};
use axum::http::{Method, Request, StatusCode};
use axum::routing::{get, post};
use http_body_util::BodyExt;
use humanizar_units::infrastructure::config::CorsConfig;
use serde_json::{Value, json};
use tower::ServiceExt;

use test_support::{JwksServer, TestClaims, rsa_token, security_config};

#[tokio::test]
async fn authenticated_reads_and_administrator_writes_are_explicitly_separated() {
    let server = JwksServer::start("active-key").await;
    let security = security_config(&server).await;
    let mut coordinator_claims = TestClaims::valid();
    coordinator_claims.realm_access = Some(json!({ "roles": ["COORDENADOR"] }));
    let coordinator_token = rsa_token("active-key", &coordinator_claims);
    let administrator_token = rsa_token("active-key", &TestClaims::valid());
    let app = application_router(&security);

    let read = app
        .clone()
        .oneshot(request(
            Method::GET,
            "/api/v1/units",
            Some(&coordinator_token),
        ))
        .await
        .expect("a leitura deve ser processada");
    let forbidden_write = app
        .clone()
        .oneshot(request(
            Method::POST,
            "/api/v1/units",
            Some(&coordinator_token),
        ))
        .await
        .expect("a escrita proibida deve ser processada");
    let allowed_write = app
        .oneshot(request(
            Method::POST,
            "/api/v1/units",
            Some(&administrator_token),
        ))
        .await
        .expect("a escrita administrativa deve ser processada");

    assert_eq!(StatusCode::OK, read.status());
    assert_eq!(StatusCode::FORBIDDEN, forbidden_write.status());
    assert_eq!(StatusCode::OK, allowed_write.status());
}

#[tokio::test]
async fn security_returns_shared_error_contract_with_path_and_reason_code() {
    let server = JwksServer::start("active-key").await;
    let security = security_config(&server).await;
    let app = CorsConfig::development().apply(application_router(&security));
    let unauthorized = app
        .clone()
        .oneshot(cors_request(Method::GET, "/api/v1/units", None))
        .await
        .expect("a resposta 401 deve ser processada");
    let mut user_claims = TestClaims::valid();
    user_claims.realm_access = Some(json!({ "roles": ["COORDENADOR"] }));
    let token = rsa_token("active-key", &user_claims);
    let forbidden = app
        .oneshot(cors_request(Method::POST, "/api/v1/units", Some(&token)))
        .await
        .expect("a resposta 403 deve ser processada");

    assert_error(
        unauthorized,
        StatusCode::UNAUTHORIZED,
        "AUTHENTICATION_FAILURE",
    )
    .await;
    assert_error(forbidden, StatusCode::FORBIDDEN, "AUTHORIZATION_FAILURE").await;
}

#[tokio::test]
async fn public_and_unknown_routes_do_not_require_authentication() {
    let server = JwksServer::start("active-key").await;
    let security = security_config(&server).await;
    let app = application_router(&security);
    let health = app
        .clone()
        .oneshot(request(Method::GET, "/health", None))
        .await
        .expect("a rota pública deve ser processada");
    let missing = app
        .oneshot(request(Method::GET, "/does-not-exist", None))
        .await
        .expect("a rota inexistente deve ser processada");

    assert_eq!(StatusCode::OK, health.status());
    assert_eq!(StatusCode::NOT_FOUND, missing.status());
}

fn application_router(
    security: &humanizar_units::infrastructure::config::SecurityConfig,
) -> Router {
    let public = Router::new().route("/health", get(|| async { "UP" }));
    let reads = security
        .protect_authenticated(Router::new().route("/api/v1/units", get(|| async { "units" })));
    let writes = security
        .protect_administrator(Router::new().route("/api/v1/units", post(|| async { "created" })));

    public.merge(reads).merge(writes)
}

fn request(method: Method, path: &str, token: Option<&str>) -> Request<Body> {
    let mut request = Request::builder().method(method).uri(path);

    if let Some(token) = token {
        request = request.header(AUTHORIZATION, format!("Bearer {token}"));
    }

    request
        .body(Body::empty())
        .expect("a requisição deve ser válida")
}

fn cors_request(method: Method, path: &str, token: Option<&str>) -> Request<Body> {
    let mut request = request(method, path, token);
    request.headers_mut().insert(
        ORIGIN,
        "http://localhost:3000".parse().expect("origem válida"),
    );
    request
}

async fn assert_error(
    response: axum::response::Response,
    expected_status: StatusCode,
    expected_reason_code: &str,
) {
    assert_eq!(expected_status, response.status());
    assert_eq!(
        "http://localhost:3000",
        response
            .headers()
            .get(ACCESS_CONTROL_ALLOW_ORIGIN)
            .expect("o erro deve preservar CORS")
    );
    let body = response
        .into_body()
        .collect()
        .await
        .expect("o body deve ser lido")
        .to_bytes();
    let body: Value = serde_json::from_slice(&body).expect("o erro deve ser JSON");

    assert_eq!(expected_reason_code, body["reasonCode"]);
    assert_eq!("/api/v1/units", body["path"]);
    assert_eq!(expected_status.as_u16(), body["status"]);
}
