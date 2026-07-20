#[path = "../../security/test_support.rs"]
mod test_support;

use std::time::Duration;

use humanizar_units::infrastructure::config::{SecurityConfig, SecuritySettings};

use test_support::{AUDIENCE, ISSUER, JwksServer};

#[test]
fn settings_derive_the_jwks_url_and_preserve_identity_contracts() {
    let settings = SecuritySettings::new("https://auth.humanizar.test/", ISSUER, AUDIENCE)
        .expect("as configurações devem ser válidas");

    assert_eq!(
        "https://auth.humanizar.test/oauth2/jwks",
        settings.jwks_url().as_str()
    );
    assert_eq!(ISSUER, settings.issuer());
    assert_eq!(AUDIENCE, settings.audience());
}

#[test]
fn settings_reject_invalid_or_empty_values() {
    assert!(SecuritySettings::new("", ISSUER, AUDIENCE).is_err());
    assert!(SecuritySettings::new("file:///tmp/auth", ISSUER, AUDIENCE).is_err());
    assert!(SecuritySettings::new("https://auth.test", "", AUDIENCE).is_err());
    assert!(SecuritySettings::new("https://auth.test", ISSUER, "").is_err());
}

#[tokio::test]
async fn startup_fails_when_jwks_is_unavailable() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("uma porta de teste deve ser reservada");
    let address = listener
        .local_addr()
        .expect("a porta deve possuir endereço");
    drop(listener);
    let settings = SecuritySettings::new(format!("http://{address}"), ISSUER, AUDIENCE)
        .expect("as configurações devem ser válidas")
        .with_cache_policy(Duration::from_secs(60), Duration::from_secs(1));

    let error = SecurityConfig::initialize(settings)
        .await
        .err()
        .expect("o startup deve falhar sem JWKS");

    assert_eq!("Falha ao inicializar o cache JWKS", error.to_string());
}

#[tokio::test]
async fn startup_loads_the_jwks_once() {
    let server = JwksServer::start("startup-key").await;
    let settings = SecuritySettings::new(server.base_url(), ISSUER, AUDIENCE)
        .expect("as configurações devem ser válidas");

    SecurityConfig::initialize(settings)
        .await
        .expect("o startup deve carregar o JWKS");

    assert_eq!(1, server.request_count());
}
