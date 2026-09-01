use std::sync::Arc;

use tokio::net::TcpListener;

use crate::application::service::{MunicipioService, UnitService};
use crate::domain::exception::TechnicalError;
use crate::domain::port::{MunicipioPort, UnitPort};
use crate::infrastructure::adapter::{MunicipioAdapter, UnitAdapter};
use crate::infrastructure::config::{
    DatabaseConfig, DatabaseSettings, RetryConfig, SecurityConfig, SecuritySettings, ServerConfig,
};
use crate::infrastructure::controller::{ApplicationState, create_router};
use crate::infrastructure::persistence::repository::{MunicipioRepository, UnitRepository};

pub async fn run() -> Result<(), TechnicalError> {
    initialize_tracing()?;

    tracing::info!(
        service = "humanizar-units",
        "Carregando a configuração do humanizar-units"
    );

    let server = ServerConfig::from_env()?;
    let retry = RetryConfig::from_env()?;
    let database_settings = DatabaseSettings::from_env()?;
    let security_settings = SecuritySettings::from_env()?;
    let database = DatabaseConfig::initialize(database_settings).await?;
    let security = SecurityConfig::initialize(security_settings).await?;
    let state = build_state(database, &retry);
    let router = create_router(state, &security);
    let bind_address = server.bind_address();
    let listener = TcpListener::bind(&bind_address)
        .await
        .map_err(|error| TechnicalError::with_source("Falha ao iniciar listener HTTP", error))?;

    tracing::info!(
        service = "humanizar-units",
        host = server.host(),
        port = server.port(),
        mode = "FULL",
        "Servidor HTTP iniciado"
    );

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|error| TechnicalError::with_source("Falha durante execucao HTTP", error))
}

fn build_state(database: DatabaseConfig, retry: &RetryConfig) -> ApplicationState {
    let municipio_repository = MunicipioRepository::new(database.clone());
    let unit_repository = UnitRepository::new(database);
    let municipio_port: Arc<dyn MunicipioPort> = Arc::new(MunicipioAdapter::new(
        municipio_repository,
        retry.executor(),
    ));
    let unit_port: Arc<dyn UnitPort> =
        Arc::new(UnitAdapter::new(unit_repository, retry.executor()));
    let municipio_service = Arc::new(MunicipioService::new(
        Arc::clone(&municipio_port),
        Arc::clone(&unit_port),
    ));
    let unit_service = Arc::new(UnitService::new(unit_port, municipio_port));

    ApplicationState::new(municipio_service, unit_service)
}

fn initialize_tracing() -> Result<(), TechnicalError> {
    tracing_subscriber::fmt()
        .with_target(false)
        .try_init()
        .map_err(|error| TechnicalError::new(format!("Falha ao inicializar tracing: {error}")))
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};

        let mut terminate = match signal(SignalKind::terminate()) {
            Ok(terminate) => terminate,
            Err(error) => {
                tracing::error!(source = ?error, "Falha ao registrar sinal SIGTERM");
                return;
            }
        };

        tokio::select! {
            result = tokio::signal::ctrl_c() => {
                if let Err(error) = result {
                    tracing::error!(source = ?error, "Falha ao aguardar sinal de encerramento");
                }
            }
            _ = terminate.recv() => {}
        }

        return;
    }

    #[cfg(not(unix))]
    if let Err(error) = tokio::signal::ctrl_c().await {
        tracing::error!(source = ?error, "Falha ao aguardar sinal de encerramento");
    }
}
