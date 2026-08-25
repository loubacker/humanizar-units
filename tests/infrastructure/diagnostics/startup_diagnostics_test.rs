#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt::{Display, Formatter};

use humanizar_units::infrastructure::diagnostics::{SafeUrl, StartupReport};
use url::Url;

#[test]
fn safe_url_removes_credentials_query_and_fragment() {
    let url = Url::parse("postgresql://humanizar:super-secreta@db:5432/units?sslmode=disable#pool")
        .expect("a URL de teste deve ser válida");

    let safe_url = SafeUrl::from_url(&url);

    assert_eq!("postgresql://db:5432/units", safe_url.as_str());
    assert_eq!("postgresql://db:5432/units", safe_url.to_string());
}

#[test]
fn safe_url_preserves_implicit_ports_and_paths() {
    let url =
        Url::parse("https://auth.humanizar.test/realms/humanizar/protocol/openid-connect/certs")
            .expect("a URL de teste deve ser válida");

    assert_eq!(
        "https://auth.humanizar.test/realms/humanizar/protocol/openid-connect/certs",
        SafeUrl::from_url(&url).as_str()
    );
}

#[test]
fn failure_report_lists_every_cause_in_order() {
    let error = ChainedError::new(
        "Falha ao inicializar o pool PostgreSQL em postgresql://db:5432/units",
        Some(ChainedError::new(
            "error connecting to server",
            Some(ChainedError::new("Connection refused (os error 111)", None)),
        )),
    );

    let report = StartupReport::failure(&error);

    assert_eq!(
        "Falha ao iniciar humanizar-units: Falha ao inicializar o pool PostgreSQL em postgresql://db:5432/units\n  \
         causa 1: error connecting to server\n  \
         causa 2: Connection refused (os error 111)",
        report
    );
}

#[test]
fn failure_report_keeps_a_single_line_without_causes() {
    let report = StartupReport::failure(&ChainedError::new("DB_URL é obrigatório", None));

    assert_eq!(
        "Falha ao iniciar humanizar-units: DB_URL é obrigatório",
        report
    );
}

#[derive(Debug)]
struct ChainedError {
    message: String,
    source: Option<Box<ChainedError>>,
}

impl ChainedError {
    fn new(message: &str, source: Option<Self>) -> Self {
        Self {
            message: message.to_owned(),
            source: source.map(Box::new),
        }
    }
}

impl Display for ChainedError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for ChainedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| source.as_ref() as &(dyn Error + 'static))
    }
}
