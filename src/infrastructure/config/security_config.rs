use std::env;
use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::middleware;
use url::Url;

use crate::domain::exception::TechnicalError;
use crate::infrastructure::diagnostics::SafeUrl;

use crate::infrastructure::security::{
    JwksCache, JwksHttpTimeouts, JwtValidator, authenticate, require_administrator,
};

use super::EnvironmentConfig;

pub type SecurityConfigError = TechnicalError;

const DEFAULT_AUDIENCE: &str = "humanizar-client";
const DEFAULT_CACHE_TTL_SECONDS: u64 = 300;
const DEFAULT_REFRESH_INTERVAL_SECONDS: u64 = 10;
const DEFAULT_CONNECT_TIMEOUT_SECONDS: u64 = 5;
const DEFAULT_REQUEST_TIMEOUT_SECONDS: u64 = 10;

#[derive(Debug, Clone)]
pub struct SecuritySettings {
    jwks_url: Url,
    issuer: String,
    audience: String,
    cache_ttl: Duration,
    minimum_refresh_interval: Duration,
    connect_timeout: Duration,
    request_timeout: Duration,
}

impl SecuritySettings {
    pub fn from_env() -> Result<Self, SecurityConfigError> {
        EnvironmentConfig::load()?;
        let jwks_url = EnvironmentConfig::required("KEYCLOAK_ISSUER")?;
        let issuer = EnvironmentConfig::required("JWT_ISSUER")?;
        let audience = env::var("JWT_AUDIENCE").unwrap_or_else(|_| DEFAULT_AUDIENCE.to_owned());
        let cache_ttl = EnvironmentConfig::positive_duration_seconds(
            "JWKS_CACHE_TTL_SECONDS",
            DEFAULT_CACHE_TTL_SECONDS,
        )?;
        let minimum_refresh_interval = EnvironmentConfig::positive_duration_seconds(
            "JWKS_REFRESH_INTERVAL_SECONDS",
            DEFAULT_REFRESH_INTERVAL_SECONDS,
        )?;
        let connect_timeout = EnvironmentConfig::positive_duration_seconds(
            "JWKS_CONNECT_TIMEOUT_SECONDS",
            DEFAULT_CONNECT_TIMEOUT_SECONDS,
        )?;
        let request_timeout = EnvironmentConfig::positive_duration_seconds(
            "JWKS_REQUEST_TIMEOUT_SECONDS",
            DEFAULT_REQUEST_TIMEOUT_SECONDS,
        )?;

        Self::new(jwks_url, issuer, audience)?
            .with_cache_policy(cache_ttl, minimum_refresh_interval)
            .with_http_timeouts(connect_timeout, request_timeout)
    }

    pub fn new(
        jwks_url: impl AsRef<str>,
        issuer: impl Into<String>,
        audience: impl Into<String>,
    ) -> Result<Self, SecurityConfigError> {
        let jwks_url = normalize_required("KEYCLOAK_ISSUER", jwks_url.as_ref())?;
        let issuer = normalize_required("JWT_ISSUER", &issuer.into())?;
        let audience = normalize_required("JWT_AUDIENCE", &audience.into())?;
        let jwks_url = Url::parse(&jwks_url)
            .map_err(|error| SecurityConfigError::with_source("KEYCLOAK_ISSUER inválida", error))?;

        if !matches!(jwks_url.scheme(), "http" | "https") || jwks_url.host_str().is_none() {
            return Err(SecurityConfigError::new(
                "KEYCLOAK_ISSUER deve usar HTTP ou HTTPS e possuir host",
            ));
        }

        if !jwks_url.username().is_empty()
            || jwks_url.password().is_some()
            || jwks_url.query().is_some()
            || jwks_url.fragment().is_some()
        {
            return Err(SecurityConfigError::new(
                "KEYCLOAK_ISSUER não deve conter credenciais, query ou fragmento",
            ));
        }

        Ok(Self {
            jwks_url,
            issuer,
            audience,
            cache_ttl: Duration::from_secs(DEFAULT_CACHE_TTL_SECONDS),
            minimum_refresh_interval: Duration::from_secs(DEFAULT_REFRESH_INTERVAL_SECONDS),
            connect_timeout: Duration::from_secs(DEFAULT_CONNECT_TIMEOUT_SECONDS),
            request_timeout: Duration::from_secs(DEFAULT_REQUEST_TIMEOUT_SECONDS),
        })
    }

    pub fn with_cache_policy(
        mut self,
        cache_ttl: Duration,
        minimum_refresh_interval: Duration,
    ) -> Self {
        self.cache_ttl = cache_ttl;
        self.minimum_refresh_interval = minimum_refresh_interval;
        self
    }

    pub fn with_http_timeouts(
        mut self,
        connect_timeout: Duration,
        request_timeout: Duration,
    ) -> Result<Self, SecurityConfigError> {
        if connect_timeout.is_zero() || request_timeout.is_zero() {
            return Err(SecurityConfigError::new(
                "Os timeouts do JWKS devem ser maiores que zero",
            ));
        }

        if connect_timeout > request_timeout {
            return Err(SecurityConfigError::new(
                "JWKS_CONNECT_TIMEOUT_SECONDS não pode ser maior que JWKS_REQUEST_TIMEOUT_SECONDS",
            ));
        }

        self.connect_timeout = connect_timeout;
        self.request_timeout = request_timeout;

        Ok(self)
    }

    pub fn jwks_url(&self) -> &Url {
        &self.jwks_url
    }

    pub fn jwks_endpoint(&self) -> SafeUrl {
        SafeUrl::from_url(&self.jwks_url)
    }

    pub fn issuer(&self) -> &str {
        &self.issuer
    }

    pub fn audience(&self) -> &str {
        &self.audience
    }

    pub const fn connect_timeout(&self) -> Duration {
        self.connect_timeout
    }

    pub const fn request_timeout(&self) -> Duration {
        self.request_timeout
    }
}

#[derive(Clone)]
pub struct SecurityConfig {
    validator: Arc<JwtValidator>,
}

impl SecurityConfig {
    pub async fn from_env() -> Result<Self, SecurityConfigError> {
        Self::initialize(SecuritySettings::from_env()?).await
    }

    pub async fn initialize(settings: SecuritySettings) -> Result<Self, SecurityConfigError> {
        let jwks_endpoint = settings.jwks_endpoint();
        let issuer_endpoint = sanitized_url_for_log(settings.issuer());

        tracing::info!(
            service = "humanizar-units",
            jwks = %jwks_endpoint,
            emissor = %issuer_endpoint,
            audiencia = settings.audience(),
            timeout_conexao_segundos = settings.connect_timeout().as_secs(),
            timeout_resposta_segundos = settings.request_timeout().as_secs(),
            "Carregando o JWKS do Keycloak"
        );

        let SecuritySettings {
            jwks_url,
            issuer,
            audience,
            cache_ttl,
            minimum_refresh_interval,
            connect_timeout,
            request_timeout,
        } = settings;
        let jwks_cache = JwksCache::initialize(
            jwks_url,
            cache_ttl,
            minimum_refresh_interval,
            JwksHttpTimeouts::new(connect_timeout, request_timeout),
        )
        .await
        .map_err(|error| {
            SecurityConfigError::with_source(
                format!("Falha ao inicializar o cache JWKS em {jwks_endpoint}"),
                error,
            )
        })?;

        tracing::info!(
            service = "humanizar-units",
            jwks = %jwks_endpoint,
            "JWKS carregado"
        );

        let validator = JwtValidator::new(jwks_cache, &issuer, &audience);

        Ok(Self {
            validator: Arc::new(validator),
        })
    }

    pub fn protect_authenticated(&self, routes: Router) -> Router {
        routes.route_layer(middleware::from_fn_with_state(
            Arc::clone(&self.validator),
            authenticate,
        ))
    }

    pub fn protect_administrator(&self, routes: Router) -> Router {
        routes
            .route_layer(middleware::from_fn(require_administrator))
            .route_layer(middleware::from_fn_with_state(
                Arc::clone(&self.validator),
                authenticate,
            ))
    }
}

fn sanitized_url_for_log(value: &str) -> String {
    Url::parse(value)
        .ok()
        .filter(|url| matches!(url.scheme(), "http" | "https") && url.host_str().is_some())
        .map(|url| SafeUrl::from_url(&url).to_string())
        .unwrap_or_else(|| "<url-invalida>".to_owned())
}

fn normalize_required(name: &str, value: &str) -> Result<String, SecurityConfigError> {
    let value = value.trim();

    if value.is_empty() {
        return Err(SecurityConfigError::new(format!(
            "{name} não pode ser vazio"
        )));
    }

    Ok(value.to_owned())
}
