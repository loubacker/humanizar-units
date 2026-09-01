#![forbid(unsafe_code)]

#[path = "../../security/test_support.rs"]
mod test_support;

use std::time::{Duration, Instant};

use humanizar_units::infrastructure::config::{SecurityConfig, SecuritySettings};
use humanizar_units::infrastructure::diagnostics::StartupReport;
use tokio::net::TcpListener;

use test_support::{AUDIENCE, ISSUER, JwksServer};

#[test]
fn settings_use_the_complete_jwks_url_and_preserve_identity_contracts() {
    let jwks_url = "https://auth.humanizar.test/realms/humanizar/protocol/openid-connect/certs";
    let settings = SecuritySettings::new(jwks_url, ISSUER, AUDIENCE)
        .expect("as configurações devem ser válidas");

    assert_eq!(jwks_url, settings.jwks_url().as_str());
    assert_eq!(ISSUER, settings.issuer());
    assert_eq!(AUDIENCE, settings.audience());
}

#[test]
fn settings_reject_invalid_or_empty_values() {
    assert!(SecuritySettings::new("", ISSUER, AUDIENCE).is_err());
    assert!(SecuritySettings::new("file:///tmp/auth", ISSUER, AUDIENCE).is_err());
    assert!(SecuritySettings::new("https://auth.test/certs", "", AUDIENCE).is_err());
    assert!(SecuritySettings::new("https://auth.test/certs", ISSUER, "").is_err());
}

#[test]
fn settings_reject_credentials_query_and_fragment_in_the_jwks_url() {
    let error = SecuritySettings::new(
        "https://cliente:super-secreta@auth.humanizar.test/certs",
        ISSUER,
        AUDIENCE,
    )
    .expect_err("credenciais na URL devem ser rejeitadas");

    assert_eq!(
        "KEYCLOAK_ISSUER não deve conter credenciais, query ou fragmento",
        error.to_string()
    );
    assert!(!error.to_string().contains("super-secreta"));
    assert!(
        SecuritySettings::new(
            "https://auth.humanizar.test/certs?tenant=1",
            ISSUER,
            AUDIENCE
        )
        .is_err()
    );
    assert!(
        SecuritySettings::new("https://auth.humanizar.test/certs#keys", ISSUER, AUDIENCE).is_err()
    );
}

#[test]
fn settings_reject_invalid_http_timeouts() {
    let settings = || {
        SecuritySettings::new("https://auth.humanizar.test/certs", ISSUER, AUDIENCE)
            .expect("as configurações base devem ser válidas")
    };

    assert_eq!(
        Duration::from_secs(5),
        settings().connect_timeout(),
        "o timeout de conexão deve possuir padrão explícito"
    );
    assert_eq!(
        Duration::from_secs(10),
        settings().request_timeout(),
        "o timeout de resposta deve possuir padrão explícito"
    );
    assert!(
        settings()
            .with_http_timeouts(Duration::ZERO, Duration::from_secs(10))
            .is_err()
    );
    assert!(
        settings()
            .with_http_timeouts(Duration::from_secs(10), Duration::from_secs(5))
            .is_err()
    );
}

#[tokio::test]
async fn startup_failure_reports_the_jwks_endpoint_and_the_technical_cause() {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("uma porta de teste deve ser reservada");
    let address = listener
        .local_addr()
        .expect("a porta deve possuir endereço");
    drop(listener);
    let jwks_url = format!("http://{address}/realms/humanizar/protocol/openid-connect/certs");
    let settings = SecuritySettings::new(&jwks_url, ISSUER, AUDIENCE)
        .expect("as configurações devem ser válidas")
        .with_cache_policy(Duration::from_secs(60), Duration::from_secs(1));

    let error = SecurityConfig::initialize(settings)
        .await
        .err()
        .expect("o startup deve falhar sem JWKS");
    let report = StartupReport::failure(&error);

    assert_eq!(
        format!("Falha ao inicializar o cache JWKS em {jwks_url}"),
        error.to_string()
    );
    assert!(report.starts_with("Falha ao iniciar humanizar-units: "));
    assert!(report.contains(&format!(
        "\n  causa 1: Falha ao consultar o JWKS em {jwks_url}"
    )));
}

#[tokio::test]
async fn startup_gives_up_when_keycloak_never_answers() {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("uma porta de teste deve ser reservada");
    let address = listener
        .local_addr()
        .expect("a porta deve possuir endereço");
    let silent_server = tokio::spawn(async move {
        let mut accepted = Vec::new();

        while let Ok((connection, _)) = listener.accept().await {
            accepted.push(connection);
        }
    });
    let jwks_url = format!("http://{address}/realms/humanizar/protocol/openid-connect/certs");
    let settings = SecuritySettings::new(&jwks_url, ISSUER, AUDIENCE)
        .expect("as configurações devem ser válidas")
        .with_http_timeouts(Duration::from_millis(200), Duration::from_millis(400))
        .expect("os timeouts devem ser válidos");
    let started_at = Instant::now();

    let error = SecurityConfig::initialize(settings)
        .await
        .err()
        .expect("o startup deve falhar quando o JWKS não responde");
    let elapsed = started_at.elapsed();

    silent_server.abort();
    assert!(
        elapsed < Duration::from_secs(5),
        "o startup deve desistir pelo timeout configurado, mas levou {elapsed:?}"
    );
    assert_eq!(
        format!("Falha ao inicializar o cache JWKS em {jwks_url}"),
        error.to_string()
    );
}

#[tokio::test]
async fn startup_loads_the_jwks_once() {
    let server = JwksServer::start("startup-key").await;
    let settings = SecuritySettings::new(server.jwks_url(), ISSUER, AUDIENCE)
        .expect("as configurações devem ser válidas");

    SecurityConfig::initialize(settings)
        .await
        .expect("o startup deve carregar o JWKS");

    assert_eq!(1, server.request_count());
}
