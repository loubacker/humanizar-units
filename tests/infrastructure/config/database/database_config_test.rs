#![forbid(unsafe_code)]

use std::time::Duration;

use humanizar_units::infrastructure::config::{DatabaseConfig, DatabaseSettings};

#[test]
fn settings_preserve_hikari_defaults_and_normalize_jdbc_url() {
    let settings = DatabaseSettings::new(
        "jdbc:postgresql://127.0.0.1:5432/humanizar_units",
        "humanizar",
        "secret",
    )
    .expect("as configurações devem ser válidas");

    assert_eq!(
        "postgresql://127.0.0.1:5432/humanizar_units",
        settings.database_url()
    );
    assert_eq!(3, settings.minimum_idle());
    assert_eq!(10, settings.maximum_pool_size());
    assert_eq!(Duration::from_secs(30), settings.connection_timeout());
    assert_eq!(Duration::from_secs(300), settings.idle_timeout());
    assert_eq!(Duration::from_secs(600), settings.maximum_lifetime());
}

#[test]
fn settings_accept_both_postgres_url_schemes() {
    for scheme in ["postgres", "postgresql"] {
        let settings = DatabaseSettings::new(
            format!("{scheme}://127.0.0.1:5432/humanizar_units"),
            "humanizar",
            "secret",
        );

        assert!(settings.is_ok());
    }
}

#[test]
fn settings_reject_invalid_urls_credentials_and_empty_values() {
    assert!(DatabaseSettings::new("mysql://localhost/units", "user", "secret").is_err());
    assert!(DatabaseSettings::new("postgresql://localhost", "user", "secret").is_err());
    let error = DatabaseSettings::new(
        "postgresql://user:super-secret@localhost/units",
        "user",
        "secret",
    )
    .err()
    .expect("credenciais na URL devem ser rejeitadas");
    assert!(!error.to_string().contains("super-secret"));
    assert!(DatabaseSettings::new("postgresql://localhost/units", "", "secret").is_err());
    assert!(DatabaseSettings::new("postgresql://localhost/units", "user", "").is_err());
}

#[test]
fn settings_reject_tls_until_the_database_moves_off_host() {
    assert!(
        DatabaseSettings::new(
            "postgresql://localhost/units?sslmode=require",
            "user",
            "secret",
        )
        .is_err()
    );
}

#[test]
fn pool_policy_rejects_invalid_sizes_and_timeouts() {
    let settings = || {
        DatabaseSettings::new("postgresql://localhost/units", "user", "secret")
            .expect("a configuração base deve ser válida")
    };

    assert!(
        settings()
            .with_pool_policy(
                11,
                10,
                Duration::from_secs(1),
                Duration::from_secs(1),
                Duration::from_secs(1),
            )
            .is_err()
    );
    assert!(
        settings()
            .with_pool_policy(
                1,
                1,
                Duration::ZERO,
                Duration::from_secs(1),
                Duration::from_secs(1),
            )
            .is_err()
    );
}

#[tokio::test]
async fn startup_fails_when_postgres_is_unavailable() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("uma porta deve ser reservada");
    let address = listener
        .local_addr()
        .expect("a porta deve possuir endereço");
    drop(listener);
    let settings = DatabaseSettings::new(
        format!("postgresql://{address}/humanizar_units"),
        "humanizar",
        "secret",
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

    let error = DatabaseConfig::initialize(settings)
        .await
        .err()
        .expect("o startup deve falhar sem PostgreSQL");

    assert_eq!("Falha ao inicializar o pool PostgreSQL", error.to_string());
}
