use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use backon::{ExponentialBuilder, Retryable};

use crate::domain::exception::PersistenceException;
use crate::infrastructure::config::RetrySettings;

#[derive(Debug, Clone)]
pub struct RetryExecutor {
    settings: RetrySettings,
}

impl RetryExecutor {
    pub(crate) const fn new(settings: RetrySettings) -> Self {
        Self { settings }
    }

    pub async fn execute_read<T, Operation, OperationFuture>(
        &self,
        operation_name: impl Into<String>,
        operation: Operation,
    ) -> Result<T, PersistenceException>
    where
        Operation: FnMut() -> OperationFuture,
        OperationFuture: Future<Output = Result<T, PersistenceException>>,
    {
        let operation_name = operation_name.into();
        let retry_number = Arc::new(AtomicUsize::new(0));
        let retry_number_for_log = Arc::clone(&retry_number);
        let max_retries = self.settings.max_retries();
        let mut backoff = ExponentialBuilder::default()
            .with_min_delay(self.settings.minimum_delay())
            .with_max_delay(self.settings.maximum_delay())
            .with_factor(2.0)
            .with_max_times(max_retries);
        backoff = match self.settings.jitter_seed() {
            Some(seed) => backoff.with_jitter_seed(seed),
            None => backoff.with_jitter(),
        };
        let retry = operation
            .retry(backoff)
            .sleep(tokio::time::sleep)
            .when(PersistenceException::is_retryable)
            .notify(move |error, delay| {
                let current_retry = retry_number_for_log.fetch_add(1, Ordering::Relaxed) + 1;
                tracing::warn!(
                    operation = operation_name,
                    retry = current_retry,
                    max_retries,
                    delay_ms = delay.as_millis(),
                    reason_code = error.reason_code().code(),
                    message = error.message(),
                    "Nova tentativa de leitura após falha transitória"
                );
            });

        tokio::time::timeout(self.settings.timeout(), retry)
            .await
            .map_err(PersistenceException::timeout)?
    }
}
