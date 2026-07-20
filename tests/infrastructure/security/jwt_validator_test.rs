mod test_support;

use axum::extract::Extension;
use axum::http::header::AUTHORIZATION;
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use http_body_util::BodyExt;
use humanizar_units::infrastructure::security::AuthenticatedUser;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde_json::{Value, json};
use tower::ServiceExt;

use test_support::{
    AUDIENCE, JwksServer, PRIVATE_KEY_FOR_TESTS, TestClaims, hmac_token, rsa_token, security_config,
};

#[tokio::test]
async fn valid_token_creates_the_authenticated_user_and_normalizes_roles() {
    let server = JwksServer::start("active-key").await;
    let config = security_config(&server).await;
    let mut claims = TestClaims::valid();
    claims.role = Some(json!(" role_administrador, coordenador "));
    claims.roles = Some(json!(["ROLE_COORDENADOR", "RECEPCAO, administrador"]));
    let token = rsa_token("active-key", &claims);
    let response = authenticated_router(&config)
        .oneshot(authenticated_request(&token))
        .await
        .expect("a requisição autenticada deve ser processada");
    let status = response.status();
    let body = response
        .into_body()
        .collect()
        .await
        .expect("o body deve ser lido")
        .to_bytes();
    let body: Value = serde_json::from_slice(&body).expect("o body deve ser JSON");

    assert_eq!(StatusCode::OK, status);
    assert_eq!(test_support::SUBJECT, body["subject"]);
    assert_eq!(AUDIENCE, body["clientId"]);
    assert_eq!(true, body["administrator"]);
    assert_eq!(true, body["coordenador"]);
    assert_eq!(true, body["recepcao"]);
}

#[tokio::test]
async fn validator_rejects_wrong_issuer_audience_expiration_and_not_before() {
    let server = JwksServer::start("active-key").await;
    let config = security_config(&server).await;
    let now = test_support::now_for_tests();
    let mut cases = Vec::new();

    let mut wrong_issuer = TestClaims::valid();
    wrong_issuer.iss = "https://attacker.test".to_owned();
    cases.push(wrong_issuer);

    let mut wrong_audience = TestClaims::valid();
    wrong_audience.aud = "another-client".to_owned();
    cases.push(wrong_audience);

    let mut expired = TestClaims::valid();
    expired.exp = now.saturating_sub(120);
    cases.push(expired);

    let mut not_active = TestClaims::valid();
    not_active.nbf = Some(now + 3_600);
    cases.push(not_active);

    let mut missing_subject = TestClaims::valid();
    missing_subject.sub = None;
    cases.push(missing_subject);

    for claims in cases {
        let token = rsa_token("active-key", &claims);
        let response = authenticated_router(&config)
            .oneshot(authenticated_request(&token))
            .await
            .expect("a rejeição deve ser processada");

        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
    }
}

#[tokio::test]
async fn validator_rejects_wrong_algorithm_signature_and_missing_key_id() {
    let server = JwksServer::start("active-key").await;
    let config = security_config(&server).await;
    let claims = TestClaims::valid();
    let wrong_algorithm = hmac_token("active-key", &claims);
    let wrong_signature = corrupt_signature(&rsa_token("active-key", &claims));
    let header = Header::new(Algorithm::RS256);
    let missing_key_id = encode(
        &header,
        &claims,
        &EncodingKey::from_rsa_pem(PRIVATE_KEY_FOR_TESTS)
            .expect("a chave RSA de teste deve ser válida"),
    )
    .expect("o JWT sem kid deve ser criado");

    for token in [wrong_algorithm, wrong_signature, missing_key_id] {
        let response = authenticated_router(&config)
            .oneshot(authenticated_request(&token))
            .await
            .expect("a rejeição deve ser processada");

        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
    }
}

#[tokio::test]
async fn unknown_key_triggers_one_jwks_refresh_and_accepts_rotated_key() {
    let server = JwksServer::start("old-key").await;
    let config = security_config(&server).await;
    server.replace_key("new-key").await;
    let token = rsa_token("new-key", &TestClaims::valid());
    let response = authenticated_router(&config)
        .oneshot(authenticated_request(&token))
        .await
        .expect("a requisição com chave rotacionada deve ser processada");

    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(2, server.request_count());
}

#[tokio::test]
async fn malformed_or_unknown_key_tokens_are_unauthorized() {
    let server = JwksServer::start("active-key").await;
    let config = security_config(&server).await;
    let unknown_key = rsa_token("unknown-key", &TestClaims::valid());

    for token in ["not-a-jwt".to_owned(), unknown_key] {
        let response = authenticated_router(&config)
            .oneshot(authenticated_request(&token))
            .await
            .expect("a rejeição deve ser processada");

        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
    }
}

fn authenticated_router(
    config: &humanizar_units::infrastructure::config::SecurityConfig,
) -> Router {
    config.protect_authenticated(Router::new().route("/protected", get(identity)))
}

async fn identity(Extension(user): Extension<AuthenticatedUser>) -> impl IntoResponse {
    Json(json!({
        "subject": user.subject(),
        "clientId": user.client_id(),
        "administrator": user.has_role("ADMINISTRADOR"),
        "coordenador": user.has_role("COORDENADOR"),
        "recepcao": user.has_role("RECEPCAO")
    }))
}

fn authenticated_request(token: &str) -> Request<axum::body::Body> {
    Request::builder()
        .uri("/protected")
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .body(axum::body::Body::empty())
        .expect("a requisição deve ser válida")
}

fn corrupt_signature(token: &str) -> String {
    let mut bytes = token.as_bytes().to_vec();
    let signature_start = bytes
        .iter()
        .rposition(|byte| *byte == b'.')
        .expect("o JWT deve possuir assinatura")
        + 1;
    bytes[signature_start] = if bytes[signature_start] == b'A' {
        b'B'
    } else {
        b'A'
    };

    String::from_utf8(bytes).expect("o JWT alterado deve continuar UTF-8")
}
