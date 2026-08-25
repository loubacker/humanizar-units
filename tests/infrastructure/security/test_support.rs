#![allow(dead_code)]

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::routing::get;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use humanizar_units::infrastructure::config::{SecurityConfig, SecuritySettings};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use rsa::RsaPrivateKey;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::traits::PublicKeyParts;
use serde::Serialize;
use serde_json::{Value, json};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

pub const ISSUER: &str = "https://auth.humanizar.test";
pub const AUDIENCE: &str = "humanizar-client";
pub const AUTHORIZED_PARTY: &str = "humanizar-admin-dashboard";
pub const SUBJECT: &str = "47c713bf-d4e5-45ff-bada-0a2969739667";
const JWKS_PATH: &str = "/realms/humanizar/protocol/openid-connect/certs";

pub const PRIVATE_KEY_FOR_TESTS: &[u8] = include_bytes!("fixtures/test_private_key.pem");

pub struct JwksServer {
    jwks_url: String,
    state: Arc<RwLock<Value>>,
    requests: Arc<AtomicUsize>,
    task: JoinHandle<()>,
}

impl JwksServer {
    pub async fn start(key_id: &str) -> Self {
        let state = Arc::new(RwLock::new(jwks(key_id)));
        let requests = Arc::new(AtomicUsize::new(0));
        let app_state = ServerState {
            jwks: Arc::clone(&state),
            requests: Arc::clone(&requests),
        };
        let app = Router::new()
            .route(JWKS_PATH, get(jwks_handler))
            .with_state(app_state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("o servidor JWKS de teste deve abrir uma porta local");
        let address = listener
            .local_addr()
            .expect("o servidor JWKS deve possuir endereço local");
        let task = tokio::spawn(async move {
            axum::serve(listener, app)
                .await
                .expect("o servidor JWKS de teste não deve falhar");
        });

        Self {
            jwks_url: format!("http://{address}{JWKS_PATH}"),
            state,
            requests,
            task,
        }
    }

    pub fn jwks_url(&self) -> &str {
        &self.jwks_url
    }

    pub async fn replace_key(&self, key_id: &str) {
        *self.state.write().await = jwks(key_id);
    }

    pub fn request_count(&self) -> usize {
        self.requests.load(Ordering::SeqCst)
    }
}

impl Drop for JwksServer {
    fn drop(&mut self) {
        self.task.abort();
    }
}

#[derive(Clone)]
struct ServerState {
    jwks: Arc<RwLock<Value>>,
    requests: Arc<AtomicUsize>,
}

async fn jwks_handler(State(state): State<ServerState>) -> Json<Value> {
    state.requests.fetch_add(1, Ordering::SeqCst);
    Json(state.jwks.read().await.clone())
}

pub async fn security_config(server: &JwksServer) -> SecurityConfig {
    let settings = SecuritySettings::new(server.jwks_url(), ISSUER, AUDIENCE)
        .expect("as configurações de teste devem ser válidas")
        .with_cache_policy(Duration::from_secs(300), Duration::from_millis(1));

    SecurityConfig::initialize(settings)
        .await
        .expect("o SecurityConfig deve carregar o JWKS de teste")
}

#[derive(Debug, Clone, Serialize)]
pub struct TestClaims {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,
    pub iss: String,
    pub aud: Value,
    pub exp: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<u64>,
    pub azp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm_access: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Value>,
}

impl TestClaims {
    pub fn valid() -> Self {
        Self {
            sub: Some(SUBJECT.to_owned()),
            iss: ISSUER.to_owned(),
            aud: json!([AUDIENCE, "humanizar-admin-gateway"]),
            exp: now_for_tests() + 3_600,
            nbf: Some(now_for_tests().saturating_sub(1)),
            azp: Some(AUTHORIZED_PARTY.to_owned()),
            client_id: None,
            realm_access: Some(json!({ "roles": ["ADMINISTRADOR"] })),
            role: None,
            roles: None,
        }
    }
}

pub fn rsa_token(key_id: &str, claims: &TestClaims) -> String {
    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some(key_id.to_owned());

    encode(
        &header,
        claims,
        &EncodingKey::from_rsa_pem(PRIVATE_KEY_FOR_TESTS)
            .expect("a chave RSA privada de teste deve ser válida"),
    )
    .expect("o token RSA de teste deve ser criado")
}

pub fn hmac_token(key_id: &str, claims: &TestClaims) -> String {
    let mut header = Header::new(Algorithm::HS256);
    header.kid = Some(key_id.to_owned());

    encode(&header, claims, &EncodingKey::from_secret(b"test-secret"))
        .expect("o token HMAC de teste deve ser criado")
}

fn jwks(key_id: &str) -> Value {
    let private_key = RsaPrivateKey::from_pkcs1_pem(
        std::str::from_utf8(PRIVATE_KEY_FOR_TESTS).expect("a chave RSA deve ser UTF-8"),
    )
    .expect("a chave RSA privada de teste deve ser válida");
    let public_key = private_key.to_public_key();
    let modulus = URL_SAFE_NO_PAD.encode(public_key.n().to_bytes_be());
    let exponent = URL_SAFE_NO_PAD.encode(public_key.e().to_bytes_be());

    json!({
        "keys": [{
            "kty": "RSA",
            "use": "sig",
            "alg": "RS256",
            "kid": key_id,
            "n": modulus,
            "e": exponent
        }]
    })
}

pub fn now_for_tests() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("o relógio deve estar após o Unix epoch")
        .as_secs()
}
