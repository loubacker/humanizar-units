use std::io::ErrorKind;
use std::sync::OnceLock;
use std::time::Duration;

use crate::domain::exception::TechnicalError;

pub type EnvironmentConfigError = TechnicalError;

static ENVIRONMENT_LOAD_RESULT: OnceLock<Result<(), String>> = OnceLock::new();

pub struct EnvironmentConfig;

impl EnvironmentConfig {
    pub fn load() -> Result<(), EnvironmentConfigError> {
        match ENVIRONMENT_LOAD_RESULT.get_or_init(load_dotenv) {
            Ok(()) => Ok(()),
            Err(message) => Err(EnvironmentConfigError::new(message.clone())),
        }
    }

    pub(crate) fn required(name: &str) -> Result<String, EnvironmentConfigError> {
        Self::load()?;
        let value = std::env::var(name)
            .map_err(|_| EnvironmentConfigError::new(format!("{name} é obrigatório")))?;
        let value = value.trim();

        if value.is_empty() {
            return Err(EnvironmentConfigError::new(format!(
                "{name} não pode ser vazio"
            )));
        }

        Ok(value.to_owned())
    }

    pub(crate) fn positive_u32(name: &str, default: u32) -> Result<u32, EnvironmentConfigError> {
        Self::load()?;
        let Ok(value) = std::env::var(name) else {
            return Ok(default);
        };
        let value = value.trim().parse::<u32>().map_err(|error| {
            EnvironmentConfigError::with_source(format!("{name} deve ser um número inteiro"), error)
        })?;

        if value == 0 {
            return Err(EnvironmentConfigError::new(format!(
                "{name} deve ser maior que zero"
            )));
        }

        Ok(value)
    }

    pub(crate) fn non_negative_usize(
        name: &str,
        default: usize,
    ) -> Result<usize, EnvironmentConfigError> {
        Self::load()?;
        let Ok(value) = std::env::var(name) else {
            return Ok(default);
        };

        value.trim().parse::<usize>().map_err(|error| {
            EnvironmentConfigError::with_source(
                format!("{name} deve ser um número inteiro não negativo"),
                error,
            )
        })
    }

    pub(crate) fn positive_duration_seconds(
        name: &str,
        default: u64,
    ) -> Result<Duration, EnvironmentConfigError> {
        Self::positive_duration(name, default, Duration::from_secs)
    }

    pub(crate) fn positive_duration_millis(
        name: &str,
        default: u64,
    ) -> Result<Duration, EnvironmentConfigError> {
        Self::positive_duration(name, default, Duration::from_millis)
    }

    fn positive_duration(
        name: &str,
        default: u64,
        to_duration: fn(u64) -> Duration,
    ) -> Result<Duration, EnvironmentConfigError> {
        Self::load()?;
        let Ok(value) = std::env::var(name) else {
            return Ok(to_duration(default));
        };
        let value = value.trim().parse::<u64>().map_err(|error| {
            EnvironmentConfigError::with_source(format!("{name} deve ser um número inteiro"), error)
        })?;

        if value == 0 {
            return Err(EnvironmentConfigError::new(format!(
                "{name} deve ser maior que zero"
            )));
        }

        Ok(to_duration(value))
    }
}

fn load_dotenv() -> Result<(), String> {
    match dotenvy::from_path(".env") {
        Ok(_) => Ok(()),
        Err(dotenvy::Error::Io(error)) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("Falha ao carregar arquivo .env: {error}")),
    }
}
