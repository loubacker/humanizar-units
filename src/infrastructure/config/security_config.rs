use std::env;
use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::middleware;
use url::Url;

use crate::domain::exception::TechnicalError;

use crate::infrastructure::security::{
    JwksCache, JwtValidator, authenticate, require_administrator,
};

use super::EnvironmentConfig;

pub type SecurityConfigError = TechnicalError;

const DEFAULT_AUDIENCE: &str = "humanizar-client";
const DEFAULT_CACHE_TTL_SECONDS: u64 = 300;
const DEFAULT_REFRESH_INTERVAL_SECONDS: u64 = 10;

#[derive(Debug, Clone)]
pub struct SecuritySettings {
    jwks_url: Url,
    issuer: String,
    audience: String,
    cache_ttl: Duration,
    minimum_refresh_interval: Duration,
}

impl SecuritySettings {
    pub fn from_env() -> Result<Self, SecurityConfigError> {
        EnvironmentConfig::load()?;
        let auth_server_url = EnvironmentConfig::required("AUTH_SERVER_URL")?;
        let issuer = env::var("JWT_ISSUER").unwrap_or_else(|_| auth_server_url.clone());
        let audience = env::var("JWT_AUDIENCE").unwrap_or_else(|_| DEFAULT_AUDIENCE.to_owned());
        let cache_ttl = EnvironmentConfig::positive_duration_seconds(
            "JWKS_CACHE_TTL_SECONDS",
            DEFAULT_CACHE_TTL_SECONDS,
        )?;
        let minimum_refresh_interval = EnvironmentConfig::positive_duration_seconds(
            "JWKS_REFRESH_INTERVAL_SECONDS",
            DEFAULT_REFRESH_INTERVAL_SECONDS,
        )?;

        Self::new(auth_server_url, issuer, audience)
            .map(|settings| settings.with_cache_policy(cache_ttl, minimum_refresh_interval))
    }

    pub fn new(
        auth_server_url: impl AsRef<str>,
        issuer: impl Into<String>,
        audience: impl Into<String>,
    ) -> Result<Self, SecurityConfigError> {
        let auth_server_url = normalize_base_url(auth_server_url.as_ref())?;
        let issuer = normalize_required("JWT_ISSUER", &issuer.into())?;
        let audience = normalize_required("JWT_AUDIENCE", &audience.into())?;
        let mut base_url = Url::parse(&auth_server_url)
            .map_err(|error| SecurityConfigError::with_source("AUTH_SERVER_URL inválida", error))?;

        if !matches!(base_url.scheme(), "http" | "https") || base_url.host_str().is_none() {
            return Err(SecurityConfigError::new(
                "AUTH_SERVER_URL deve usar HTTP ou HTTPS e possuir host",
            ));
        }

        base_url.set_path("/");
        base_url.set_query(None);
        base_url.set_fragment(None);
        let jwks_url = base_url.join("oauth2/jwks").map_err(|error| {
            SecurityConfigError::with_source("Não foi possível montar a URL JWKS", error)
        })?;

        Ok(Self {
            jwks_url,
            issuer,
            audience,
            cache_ttl: Duration::from_secs(DEFAULT_CACHE_TTL_SECONDS),
            minimum_refresh_interval: Duration::from_secs(DEFAULT_REFRESH_INTERVAL_SECONDS),
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

    pub fn jwks_url(&self) -> &Url {
        &self.jwks_url
    }

    pub fn issuer(&self) -> &str {
        &self.issuer
    }

    pub fn audience(&self) -> &str {
        &self.audience
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
        let jwks_cache = JwksCache::initialize(
            settings.jwks_url,
            settings.cache_ttl,
            settings.minimum_refresh_interval,
        )
        .await
        .map_err(|error| {
            SecurityConfigError::with_source("Falha ao inicializar o cache JWKS", error)
        })?;
        let validator = JwtValidator::new(jwks_cache, &settings.issuer, &settings.audience);

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

fn normalize_required(name: &str, value: &str) -> Result<String, SecurityConfigError> {
    let value = value.trim();

    if value.is_empty() {
        return Err(SecurityConfigError::new(format!(
            "{name} não pode ser vazio"
        )));
    }

    Ok(value.to_owned())
}

fn normalize_base_url(value: &str) -> Result<String, SecurityConfigError> {
    normalize_required("AUTH_SERVER_URL", value).map(|value| value.trim_end_matches('/').to_owned())
}
