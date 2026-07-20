use std::env;

use crate::domain::exception::TechnicalError;

use super::EnvironmentConfig;

pub type ServerConfigError = TechnicalError;

const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 9095;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConfig {
    host: String,
    port: u16,
}

impl ServerConfig {
    pub fn from_env() -> Result<Self, ServerConfigError> {
        EnvironmentConfig::load()?;
        let host = env::var("SERVER_HOST").unwrap_or_else(|_| DEFAULT_HOST.to_owned());
        let port = env::var("SERVER_PORT")
            .map_or_else(|_| Ok(DEFAULT_PORT), |value| parse_port(&value))?;

        Self::new(host, port)
    }

    pub fn new(host: impl Into<String>, port: u16) -> Result<Self, ServerConfigError> {
        let host = host.into();
        let host = host.trim();

        if host.is_empty() {
            return Err(ServerConfigError::new("SERVER_HOST nao pode ser vazio"));
        }

        if port == 0 {
            return Err(ServerConfigError::new(
                "SERVER_PORT deve ser maior que zero",
            ));
        }

        Ok(Self {
            host: host.to_owned(),
            port,
        })
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub const fn port(&self) -> u16 {
        self.port
    }
}

fn parse_port(value: &str) -> Result<u16, ServerConfigError> {
    value
        .trim()
        .parse::<u16>()
        .map_err(|error| ServerConfigError::with_source("SERVER_PORT invalida", error))
        .and_then(|port| ServerConfig::new(DEFAULT_HOST, port).map(|config| config.port))
}
