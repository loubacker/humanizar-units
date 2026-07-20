use std::time::Duration;

use crate::domain::exception::TechnicalError;
use crate::infrastructure::resilience::RetryExecutor;

use super::EnvironmentConfig;

pub type RetryConfigError = TechnicalError;

const DEFAULT_MAX_RETRIES: usize = 2;
const DEFAULT_TIMEOUT_SECONDS: u64 = 30;
const DEFAULT_MINIMUM_DELAY_MILLIS: u64 = 100;
const DEFAULT_MAXIMUM_DELAY_MILLIS: u64 = 1_000;

#[derive(Debug, Clone)]
pub struct RetrySettings {
    max_retries: usize,
    timeout: Duration,
    minimum_delay: Duration,
    maximum_delay: Duration,
    jitter_seed: Option<u64>,
}

impl RetrySettings {
    pub fn from_env() -> Result<Self, RetryConfigError> {
        EnvironmentConfig::load()?;
        let max_retries = EnvironmentConfig::non_negative_usize(
            "DATABASE_RETRY_MAX_RETRIES",
            DEFAULT_MAX_RETRIES,
        )?;
        let timeout = EnvironmentConfig::positive_duration_seconds(
            "DATABASE_RETRY_TIMEOUT_SECONDS",
            DEFAULT_TIMEOUT_SECONDS,
        )?;
        let minimum_delay = EnvironmentConfig::positive_duration_millis(
            "DATABASE_RETRY_MIN_DELAY_MILLIS",
            DEFAULT_MINIMUM_DELAY_MILLIS,
        )?;
        let maximum_delay = EnvironmentConfig::positive_duration_millis(
            "DATABASE_RETRY_MAX_DELAY_MILLIS",
            DEFAULT_MAXIMUM_DELAY_MILLIS,
        )?;

        Self::new(max_retries, timeout, minimum_delay, maximum_delay)
    }

    pub fn new(
        max_retries: usize,
        timeout: Duration,
        minimum_delay: Duration,
        maximum_delay: Duration,
    ) -> Result<Self, RetryConfigError> {
        if timeout.is_zero() || minimum_delay.is_zero() || maximum_delay.is_zero() {
            return Err(RetryConfigError::new(
                "Timeout e atrasos do retry devem ser maiores que zero",
            ));
        }

        if minimum_delay > maximum_delay {
            return Err(RetryConfigError::new(
                "O atraso mínimo do retry não pode superar o atraso máximo",
            ));
        }

        Ok(Self {
            max_retries,
            timeout,
            minimum_delay,
            maximum_delay,
            jitter_seed: None,
        })
    }

    pub fn with_jitter_seed(mut self, jitter_seed: u64) -> Self {
        self.jitter_seed = Some(jitter_seed);
        self
    }

    pub const fn max_retries(&self) -> usize {
        self.max_retries
    }

    pub const fn timeout(&self) -> Duration {
        self.timeout
    }

    pub const fn minimum_delay(&self) -> Duration {
        self.minimum_delay
    }

    pub const fn maximum_delay(&self) -> Duration {
        self.maximum_delay
    }

    pub(crate) const fn jitter_seed(&self) -> Option<u64> {
        self.jitter_seed
    }
}

#[derive(Debug, Clone)]
pub struct RetryConfig {
    settings: RetrySettings,
}

impl RetryConfig {
    pub fn from_env() -> Result<Self, RetryConfigError> {
        RetrySettings::from_env().map(Self::new)
    }

    pub const fn new(settings: RetrySettings) -> Self {
        Self { settings }
    }

    pub fn executor(&self) -> RetryExecutor {
        RetryExecutor::new(self.settings.clone())
    }

    pub const fn settings(&self) -> &RetrySettings {
        &self.settings
    }
}
