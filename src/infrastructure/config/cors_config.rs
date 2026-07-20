use std::collections::HashSet;
use std::env;
use std::time::Duration;

use axum::Router;
use axum::http::{HeaderName, HeaderValue, Method};
use tower_http::cors::{AllowOrigin, CorsLayer};
use url::Url;

use crate::domain::exception::TechnicalError;

use super::EnvironmentConfig;

pub type CorsConfigError = TechnicalError;

const CORS_ALLOWED_ORIGINS: &str = "CORS_ALLOWED_ORIGINS";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationEnvironment {
    Development,
    Production,
}

impl ApplicationEnvironment {
    fn from_env() -> Result<Self, CorsConfigError> {
        let value = env::var("APP_ENV").unwrap_or_else(|_| "development".to_owned());

        match value.trim().to_ascii_lowercase().as_str() {
            "development" | "dev" | "local" => Ok(Self::Development),
            "production" | "prod" => Ok(Self::Production),
            environment => Err(CorsConfigError::new(format!(
                "APP_ENV possui valor não suportado: {environment}"
            ))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CorsConfig {
    environment: ApplicationEnvironment,
    allowed_origins: HashSet<HeaderValue>,
}

impl CorsConfig {
    pub fn from_env() -> Result<Self, CorsConfigError> {
        EnvironmentConfig::load()?;
        let environment = ApplicationEnvironment::from_env()?;
        let configured_origins = env::var(CORS_ALLOWED_ORIGINS).unwrap_or_default();

        Self::new(environment, configured_origins.split(','))
    }

    pub fn development() -> Self {
        Self {
            environment: ApplicationEnvironment::Development,
            allowed_origins: HashSet::new(),
        }
    }

    pub fn production<I, S>(allowed_origins: I) -> Result<Self, CorsConfigError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Self::new(ApplicationEnvironment::Production, allowed_origins)
    }

    pub fn new<I, S>(
        environment: ApplicationEnvironment,
        allowed_origins: I,
    ) -> Result<Self, CorsConfigError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let allowed_origins = allowed_origins
            .into_iter()
            .filter_map(|origin| {
                let value = origin.as_ref().trim();
                (!value.is_empty()).then_some(value.to_owned())
            })
            .map(|origin| parse_origin(&origin))
            .collect::<Result<HashSet<_>, _>>()?;

        if environment == ApplicationEnvironment::Production && allowed_origins.is_empty() {
            return Err(CorsConfigError::new(
                "CORS_ALLOWED_ORIGINS é obrigatório em produção",
            ));
        }

        Ok(Self {
            environment,
            allowed_origins,
        })
    }

    pub fn layer(&self) -> CorsLayer {
        let environment = self.environment;
        let allowed_origins = self.allowed_origins.clone();
        let allow_origin = AllowOrigin::predicate(move |origin, _request| {
            allowed_origins.contains(origin)
                || (environment == ApplicationEnvironment::Development
                    && is_local_development_origin(origin))
        });

        CorsLayer::new()
            .allow_origin(allow_origin)
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_headers(allowed_headers())
            .allow_credentials(true)
            .max_age(Duration::from_secs(3_600))
    }

    pub fn apply(&self, router: Router) -> Router {
        router.layer(self.layer())
    }
}

fn parse_origin(origin: &str) -> Result<HeaderValue, CorsConfigError> {
    if origin == "*" {
        return Err(CorsConfigError::new(
            "CORS não aceita origem curinga quando credentials está habilitado",
        ));
    }

    let parsed = Url::parse(origin)
        .map_err(|error| CorsConfigError::new(format!("Origem CORS inválida: {error}")))?;

    if !matches!(parsed.scheme(), "http" | "https") || parsed.host_str().is_none() {
        return Err(CorsConfigError::new(
            "Origem CORS deve usar HTTP ou HTTPS e possuir host",
        ));
    }

    if !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.query().is_some()
        || parsed.fragment().is_some()
        || parsed.path() != "/"
    {
        return Err(CorsConfigError::new(
            "Origem CORS não pode conter credenciais, path, query ou fragment",
        ));
    }

    let normalized = parsed.origin().ascii_serialization();
    HeaderValue::from_str(&normalized)
        .map_err(|error| CorsConfigError::new(format!("Origem CORS inválida: {error}")))
}

fn is_local_development_origin(origin: &HeaderValue) -> bool {
    let Ok(origin) = origin.to_str() else {
        return false;
    };
    let Ok(origin) = Url::parse(origin) else {
        return false;
    };

    origin.scheme() == "http"
        && matches!(origin.host_str(), Some("localhost" | "127.0.0.1"))
        && origin.username().is_empty()
        && origin.password().is_none()
        && origin.path() == "/"
        && origin.query().is_none()
        && origin.fragment().is_none()
}

fn allowed_headers() -> [HeaderName; 9] {
    [
        HeaderName::from_static("authorization"),
        HeaderName::from_static("content-type"),
        HeaderName::from_static("x-client-id"),
        HeaderName::from_static("x-client-secret"),
        HeaderName::from_static("x-requested-with"),
        HeaderName::from_static("x-correlation-id"),
        HeaderName::from_static("x-total-count"),
        HeaderName::from_static("x-page-number"),
        HeaderName::from_static("x-page-size"),
    ]
}
