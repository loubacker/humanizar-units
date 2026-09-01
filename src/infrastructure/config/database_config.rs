use std::time::Duration;

use bb8_postgres::PostgresConnectionManager;
use bb8_postgres::bb8::{Pool, PooledConnection, QueueStrategy, RunError};
use tokio_postgres::{Config as PostgresConfig, NoTls};
use url::Url;

use crate::domain::exception::{PersistenceException, TechnicalError};
use crate::infrastructure::diagnostics::SafeUrl;

use super::EnvironmentConfig;

pub type DatabaseConfigError = TechnicalError;

const DEFAULT_MIN_IDLE: u32 = 3;
const DEFAULT_MAX_SIZE: u32 = 10;
const DEFAULT_CONNECTION_TIMEOUT_SECONDS: u64 = 30;
const DEFAULT_IDLE_TIMEOUT_SECONDS: u64 = 300;
const DEFAULT_MAX_LIFETIME_SECONDS: u64 = 600;

type Manager = PostgresConnectionManager<NoTls>;

pub struct DatabaseSettings {
    database_url: Url,
    username: String,
    password: String,
    minimum_idle: u32,
    maximum_pool_size: u32,
    connection_timeout: Duration,
    idle_timeout: Duration,
    maximum_lifetime: Duration,
}

impl DatabaseSettings {
    pub fn from_env() -> Result<Self, DatabaseConfigError> {
        EnvironmentConfig::load()?;
        let database_url = EnvironmentConfig::required("DB_URL")?;
        let username = EnvironmentConfig::required("DB_USERNAME")?;
        let password = EnvironmentConfig::required("DB_PASSWORD")?;
        let minimum_idle = EnvironmentConfig::positive_u32("DB_POOL_MIN_IDLE", DEFAULT_MIN_IDLE)?;
        let maximum_pool_size =
            EnvironmentConfig::positive_u32("DB_POOL_MAX_SIZE", DEFAULT_MAX_SIZE)?;
        let connection_timeout = EnvironmentConfig::positive_duration_seconds(
            "DB_POOL_CONNECTION_TIMEOUT_SECONDS",
            DEFAULT_CONNECTION_TIMEOUT_SECONDS,
        )?;
        let idle_timeout = EnvironmentConfig::positive_duration_seconds(
            "DB_POOL_IDLE_TIMEOUT_SECONDS",
            DEFAULT_IDLE_TIMEOUT_SECONDS,
        )?;
        let maximum_lifetime = EnvironmentConfig::positive_duration_seconds(
            "DB_POOL_MAX_LIFETIME_SECONDS",
            DEFAULT_MAX_LIFETIME_SECONDS,
        )?;

        Self::new(database_url, username, password)?.with_pool_policy(
            minimum_idle,
            maximum_pool_size,
            connection_timeout,
            idle_timeout,
            maximum_lifetime,
        )
    }

    pub fn new(
        database_url: impl AsRef<str>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<Self, DatabaseConfigError> {
        let database_url = normalize_database_url(database_url.as_ref())?;
        let username = normalize_required("DB_USERNAME", username.into())?;
        let password = normalize_required("DB_PASSWORD", password.into())?;

        Ok(Self {
            database_url,
            username,
            password,
            minimum_idle: DEFAULT_MIN_IDLE,
            maximum_pool_size: DEFAULT_MAX_SIZE,
            connection_timeout: Duration::from_secs(DEFAULT_CONNECTION_TIMEOUT_SECONDS),
            idle_timeout: Duration::from_secs(DEFAULT_IDLE_TIMEOUT_SECONDS),
            maximum_lifetime: Duration::from_secs(DEFAULT_MAX_LIFETIME_SECONDS),
        })
    }

    pub fn with_pool_policy(
        mut self,
        minimum_idle: u32,
        maximum_pool_size: u32,
        connection_timeout: Duration,
        idle_timeout: Duration,
        maximum_lifetime: Duration,
    ) -> Result<Self, DatabaseConfigError> {
        validate_pool_policy(
            minimum_idle,
            maximum_pool_size,
            connection_timeout,
            idle_timeout,
            maximum_lifetime,
        )?;
        self.minimum_idle = minimum_idle;
        self.maximum_pool_size = maximum_pool_size;
        self.connection_timeout = connection_timeout;
        self.idle_timeout = idle_timeout;
        self.maximum_lifetime = maximum_lifetime;

        Ok(self)
    }

    pub fn database_url(&self) -> &str {
        self.database_url.as_str()
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn endpoint(&self) -> SafeUrl {
        SafeUrl::from_url(&self.database_url)
    }

    pub const fn minimum_idle(&self) -> u32 {
        self.minimum_idle
    }

    pub const fn maximum_pool_size(&self) -> u32 {
        self.maximum_pool_size
    }

    pub const fn connection_timeout(&self) -> Duration {
        self.connection_timeout
    }

    pub const fn idle_timeout(&self) -> Duration {
        self.idle_timeout
    }

    pub const fn maximum_lifetime(&self) -> Duration {
        self.maximum_lifetime
    }

    fn postgres_config(&self) -> Result<PostgresConfig, DatabaseConfigError> {
        let mut config = self
            .database_url
            .as_str()
            .parse::<PostgresConfig>()
            .map_err(|error| {
                DatabaseConfigError::with_source(
                    format!("DB_URL inválida para {}", self.endpoint()),
                    error,
                )
            })?;
        config.user(&self.username);
        config.password(self.password.as_bytes());

        Ok(config)
    }
}

#[derive(Clone)]
pub struct DatabaseConfig {
    pool: Pool<Manager>,
}

impl DatabaseConfig {
    pub async fn from_env() -> Result<Self, DatabaseConfigError> {
        Self::initialize(DatabaseSettings::from_env()?).await
    }

    pub async fn initialize(settings: DatabaseSettings) -> Result<Self, DatabaseConfigError> {
        let endpoint = settings.endpoint();

        tracing::info!(
            service = "humanizar-units",
            banco = %endpoint,
            usuario = settings.username(),
            conexoes_minimas = settings.minimum_idle,
            conexoes_maximas = settings.maximum_pool_size,
            timeout_conexao_segundos = settings.connection_timeout.as_secs(),
            "Inicializando o pool PostgreSQL"
        );

        let manager = PostgresConnectionManager::new(settings.postgres_config()?, NoTls);
        let pool = Pool::builder()
            .min_idle(Some(settings.minimum_idle))
            .max_size(settings.maximum_pool_size)
            .connection_timeout(settings.connection_timeout)
            .idle_timeout(Some(settings.idle_timeout))
            .max_lifetime(Some(settings.maximum_lifetime))
            .queue_strategy(QueueStrategy::Fifo)
            .test_on_check_out(true)
            .retry_connection(true)
            .build(manager)
            .await
            .map_err(|error| {
                DatabaseConfigError::with_source(
                    format!(
                        "Falha ao inicializar o pool PostgreSQL em {endpoint} com o usuário {}",
                        settings.username()
                    ),
                    error,
                )
            })?;

        tracing::info!(
            service = "humanizar-units",
            banco = %endpoint,
            "Pool PostgreSQL inicializado"
        );

        Ok(Self { pool })
    }

    pub async fn acquire(&self) -> Result<PooledConnection<'_, Manager>, PersistenceException> {
        self.pool.get().await.map_err(map_pool_error)
    }
}

fn normalize_database_url(value: &str) -> Result<Url, DatabaseConfigError> {
    let value = value.trim();

    if value.is_empty() {
        return Err(DatabaseConfigError::new("DB_URL não pode ser vazio"));
    }

    let value = value.strip_prefix("jdbc:").unwrap_or(value);
    let url = Url::parse(value)
        .map_err(|error| DatabaseConfigError::with_source("DB_URL inválida", error))?;

    if !matches!(url.scheme(), "postgres" | "postgresql") {
        return Err(DatabaseConfigError::new(
            "DB_URL deve usar postgres:// ou postgresql://",
        ));
    }

    if url.host_str().is_none() || url.path().trim_matches('/').is_empty() {
        return Err(DatabaseConfigError::new(
            "DB_URL deve possuir host e nome do banco",
        ));
    }

    if !url.username().is_empty() || url.password().is_some() {
        return Err(DatabaseConfigError::new(
            "DB_URL não deve conter usuário ou senha",
        ));
    }

    if url
        .query_pairs()
        .find(|(name, _)| name == "sslmode")
        .is_some_and(|(_, value)| value != "disable")
    {
        return Err(DatabaseConfigError::new(
            "DB_URL deve usar sslmode=disable enquanto PostgreSQL e aplicação forem locais",
        ));
    }

    Ok(url)
}

fn normalize_required(name: &str, value: String) -> Result<String, DatabaseConfigError> {
    let value = value.trim();

    if value.is_empty() {
        return Err(DatabaseConfigError::new(format!(
            "{name} não pode ser vazio"
        )));
    }

    Ok(value.to_owned())
}

fn validate_pool_policy(
    minimum_idle: u32,
    maximum_pool_size: u32,
    connection_timeout: Duration,
    idle_timeout: Duration,
    maximum_lifetime: Duration,
) -> Result<(), DatabaseConfigError> {
    if minimum_idle == 0 || maximum_pool_size == 0 {
        return Err(DatabaseConfigError::new(
            "Os tamanhos do pool devem ser maiores que zero",
        ));
    }

    if minimum_idle > maximum_pool_size {
        return Err(DatabaseConfigError::new(
            "DB_POOL_MIN_IDLE não pode ser maior que DB_POOL_MAX_SIZE",
        ));
    }

    if [connection_timeout, idle_timeout, maximum_lifetime].contains(&Duration::ZERO) {
        return Err(DatabaseConfigError::new(
            "Os timeouts do pool devem ser maiores que zero",
        ));
    }

    Ok(())
}

fn map_pool_error(error: RunError<tokio_postgres::Error>) -> PersistenceException {
    match error {
        RunError::TimedOut => PersistenceException::pool_acquisition(error),
        RunError::User(source) => PersistenceException::connection(source),
    }
}
