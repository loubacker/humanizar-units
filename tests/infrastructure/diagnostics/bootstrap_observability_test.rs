#![forbid(unsafe_code)]

#[path = "../security/test_support.rs"]
mod test_support;
#[path = "../../support/tracing_capture.rs"]
mod tracing_capture;

use std::time::Duration;

use humanizar_units::infrastructure::config::{
    DatabaseConfig, DatabaseSettings, SecurityConfig, SecuritySettings,
};
use tracing_capture::{CapturedEvent, CapturedEvents};
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;

use test_support::{AUDIENCE, ISSUER, JwksServer};

const ISSUER_WITH_PRIVATE_PARTS: &str = "https://issuer-user:issuer-secret@auth.humanizar.test/realms/humanizar?tenant=private#fragment-private";

const SERVER: &str = include_str!("../../../src/infrastructure/server.rs");
const DATABASE_CONFIG: &str = include_str!("../../../src/infrastructure/config/database_config.rs");
const SECURITY_CONFIG: &str = include_str!("../../../src/infrastructure/config/security_config.rs");

#[test]
fn bootstrap_contract_preserves_the_instrumented_sequence() {
    assert_before(
        SERVER,
        "initialize_tracing()?",
        "Carregando a configuração do humanizar-units",
    );
    assert_before(
        SERVER,
        "Carregando a configuração do humanizar-units",
        "DatabaseConfig::initialize",
    );
    assert_before(
        SERVER,
        "DatabaseConfig::initialize",
        "SecurityConfig::initialize",
    );
    assert_before(SERVER, "TcpListener::bind", "Servidor HTTP iniciado");
    assert!(SERVER.contains("mode = \"FULL\""));

    assert_before(
        DATABASE_CONFIG,
        "Inicializando o pool PostgreSQL",
        ".build(manager)",
    );
    assert_before(
        DATABASE_CONFIG,
        ".build(manager)",
        "Pool PostgreSQL inicializado",
    );
    assert_before(
        SECURITY_CONFIG,
        "Carregando o JWKS do Keycloak",
        "JwksCache::initialize",
    );
    assert_before(SECURITY_CONFIG, "JwksCache::initialize", "JWKS carregado");
}

#[tokio::test(flavor = "current_thread")]
async fn jwks_events_are_ordered_and_sanitized() {
    let server = JwksServer::start("bootstrap-key").await;
    let settings = SecuritySettings::new(server.jwks_url(), ISSUER_WITH_PRIVATE_PARTS, AUDIENCE)
        .expect("as configurações devem ser válidas");
    let captured = CapturedEvents::default();
    let subscriber = Registry::default().with(captured.clone());
    let subscriber_guard = tracing::subscriber::set_default(subscriber);

    SecurityConfig::initialize(settings)
        .await
        .expect("o bootstrap deve carregar o JWKS");
    drop(subscriber_guard);

    let events = captured.events();
    let loading = find_event(&events, "Carregando o JWKS do Keycloak");
    let loaded = find_event(&events, "JWKS carregado");

    assert_eq!("INFO", loading.level);
    assert_eq!("INFO", loaded.level);
    assert_eq!(
        Some("humanizar-units"),
        loading.fields.get("service").map(String::as_str)
    );
    assert_eq!(
        Some("humanizar-units"),
        loaded.fields.get("service").map(String::as_str)
    );
    assert_eq!(
        Some("https://auth.humanizar.test/realms/humanizar"),
        loading.fields.get("emissor").map(String::as_str)
    );
    assert!(
        event_position(&events, "Carregando o JWKS do Keycloak")
            < event_position(&events, "JWKS carregado")
    );
    assert_eq!(1, event_count(&events, "Carregando o JWKS do Keycloak"));
    assert_eq!(1, event_count(&events, "JWKS carregado"));
    assert_eq!(1, server.request_count());
    assert_events_do_not_contain(&events, "bootstrap-key");
    assert_events_do_not_contain(&events, "issuer-secret");
    assert_events_do_not_contain(&events, "tenant=private");
    assert_events_do_not_contain(&events, "fragment-private");
    assert_events_do_not_contain(&events, "bootstrap-secret");
}

#[tokio::test(flavor = "current_thread")]
async fn failed_jwks_startup_does_not_emit_success() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("uma porta deve ser reservada");
    let address = listener
        .local_addr()
        .expect("a porta deve possuir endereço");
    drop(listener);
    let jwks_url = format!("http://{address}/realms/humanizar/protocol/openid-connect/certs");
    let settings = SecuritySettings::new(jwks_url, ISSUER, AUDIENCE)
        .expect("as configurações devem ser válidas");
    let captured = CapturedEvents::default();
    let subscriber = Registry::default().with(captured.clone());
    let subscriber_guard = tracing::subscriber::set_default(subscriber);

    let result = SecurityConfig::initialize(settings).await;
    drop(subscriber_guard);

    assert!(result.is_err());
    let events = captured.events();
    assert_eq!(1, event_count(&events, "Carregando o JWKS do Keycloak"));
    assert_eq!(0, event_count(&events, "JWKS carregado"));
}

#[tokio::test(flavor = "current_thread")]
async fn failed_database_startup_does_not_emit_success_or_credentials() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("uma porta deve ser reservada");
    let address = listener
        .local_addr()
        .expect("a porta deve possuir endereço");
    drop(listener);
    let settings = DatabaseSettings::new(
        format!("postgresql://{address}/humanizar_units?sslmode=disable"),
        "bootstrap-user",
        "bootstrap-secret",
    )
    .expect("as configurações devem ser válidas")
    .with_pool_policy(
        1,
        1,
        Duration::from_millis(100),
        Duration::from_secs(1),
        Duration::from_secs(1),
    )
    .expect("a política deve ser válida");
    let captured = CapturedEvents::default();
    let subscriber = Registry::default().with(captured.clone());
    let subscriber_guard = tracing::subscriber::set_default(subscriber);

    let result = DatabaseConfig::initialize(settings).await;
    drop(subscriber_guard);

    assert!(result.is_err());
    let events = captured.events();
    assert_eq!(1, event_count(&events, "Inicializando o pool PostgreSQL"));
    assert_eq!(0, event_count(&events, "Pool PostgreSQL inicializado"));
    assert_events_do_not_contain(&events, "bootstrap-secret");
    assert_events_do_not_contain(&events, "sslmode=disable");
}

fn assert_before(source: &str, first: &str, second: &str) {
    let first_position = source
        .find(first)
        .unwrap_or_else(|| panic!("{first} deve existir"));
    let second_position = source
        .find(second)
        .unwrap_or_else(|| panic!("{second} deve existir"));

    assert!(
        first_position < second_position,
        "{first} deve aparecer antes de {second}"
    );
}

fn find_event<'a>(events: &'a [CapturedEvent], message: &str) -> &'a CapturedEvent {
    events
        .iter()
        .find(|event| {
            event
                .fields
                .get("message")
                .is_some_and(|value| value == message)
        })
        .unwrap_or_else(|| panic!("o evento {message} deve existir"))
}

fn event_position(events: &[CapturedEvent], message: &str) -> usize {
    events
        .iter()
        .position(|event| {
            event
                .fields
                .get("message")
                .is_some_and(|value| value == message)
        })
        .unwrap_or_else(|| panic!("o evento {message} deve existir"))
}

fn event_count(events: &[CapturedEvent], message: &str) -> usize {
    events
        .iter()
        .filter(|event| {
            event
                .fields
                .get("message")
                .is_some_and(|value| value == message)
        })
        .count()
}

fn assert_events_do_not_contain(events: &[CapturedEvent], sentinel: &str) {
    assert!(
        events
            .iter()
            .flat_map(|event| event.fields.values())
            .all(|value| !value.contains(sentinel)),
        "os eventos não devem expor {sentinel}"
    );
}
