#![forbid(unsafe_code)]

#[path = "../../support/tracing_capture.rs"]
mod tracing_capture;

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use humanizar_units::domain::exception::PersistenceException;
use humanizar_units::domain::model::enums::ReasonCode;
use humanizar_units::infrastructure::config::{RetryConfig, RetrySettings};
use tracing_capture::CapturedEvents;
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;

#[tokio::test]
async fn transient_read_succeeds_after_two_retries() {
    let attempts = Arc::new(AtomicUsize::new(0));
    let attempts_for_operation = Arc::clone(&attempts);
    let executor = executor(2, Duration::from_secs(1));

    let value = executor
        .execute_read("find_units", move || {
            let current_attempt = attempts_for_operation.fetch_add(1, Ordering::SeqCst) + 1;
            async move {
                if current_attempt < 3 {
                    return Err(PersistenceException::transient_query(
                        std::io::Error::other(format!("failure-{current_attempt}")),
                    ));
                }

                Ok("units")
            }
        })
        .await
        .expect("a terceira tentativa deve funcionar");

    assert_eq!("units", value);
    assert_eq!(3, attempts.load(Ordering::SeqCst));
}

#[tokio::test]
async fn exhausted_retry_preserves_the_last_persistence_exception() {
    let attempts = Arc::new(AtomicUsize::new(0));
    let attempts_for_operation = Arc::clone(&attempts);
    let executor = executor(2, Duration::from_secs(1));

    let error = executor
        .execute_read("find_units", move || {
            let current_attempt = attempts_for_operation.fetch_add(1, Ordering::SeqCst) + 1;
            async move {
                Err::<(), _>(PersistenceException::transient_query(
                    std::io::Error::other(format!("failure-{current_attempt}")),
                ))
            }
        })
        .await
        .expect_err("o retry deve ser esgotado");

    assert_eq!(3, attempts.load(Ordering::SeqCst));
    assert_eq!(ReasonCode::PersistenceFailure, error.reason_code());
    assert!(error.is_retryable());
    assert!(format!("{error:?}").contains("failure-3"));
}

#[tokio::test]
async fn terminal_failure_is_not_retried() {
    let attempts = Arc::new(AtomicUsize::new(0));
    let attempts_for_operation = Arc::clone(&attempts);
    let executor = executor(2, Duration::from_secs(1));

    let error = executor
        .execute_read("find_units", move || {
            attempts_for_operation.fetch_add(1, Ordering::SeqCst);
            async {
                Err::<(), _>(PersistenceException::query(std::io::Error::other(
                    "syntax error",
                )))
            }
        })
        .await
        .expect_err("a operação deve falhar");

    assert_eq!(1, attempts.load(Ordering::SeqCst));
    assert!(!error.is_retryable());
}

#[tokio::test]
async fn total_timeout_includes_the_current_operation() {
    let attempts = Arc::new(AtomicUsize::new(0));
    let attempts_for_operation = Arc::clone(&attempts);
    let executor = executor(2, Duration::from_millis(20));

    let error = executor
        .execute_read("slow_find", move || {
            attempts_for_operation.fetch_add(1, Ordering::SeqCst);
            async {
                tokio::time::sleep(Duration::from_secs(1)).await;
                Ok::<(), PersistenceException>(())
            }
        })
        .await
        .expect_err("o prazo total deve ser respeitado");

    assert_eq!(1, attempts.load(Ordering::SeqCst));
    assert!(!error.is_retryable());
    assert!(format!("{error:?}").contains("Tempo limite"));
}

#[tokio::test(flavor = "current_thread")]
async fn retry_is_logged_as_structured_warning_without_the_technical_source() {
    let captured_events = CapturedEvents::default();
    let subscriber = Registry::default().with(captured_events.clone());
    let subscriber_guard = tracing::subscriber::set_default(subscriber);
    let attempts = Arc::new(AtomicUsize::new(0));
    let attempts_for_operation = Arc::clone(&attempts);
    let executor = executor(1, Duration::from_secs(1));

    executor
        .execute_read("find_units", move || {
            let current_attempt = attempts_for_operation.fetch_add(1, Ordering::SeqCst);
            async move {
                if current_attempt == 0 {
                    return Err(PersistenceException::transient_query(
                        std::io::Error::other("database password must remain secret"),
                    ));
                }

                Ok(())
            }
        })
        .await
        .expect("a segunda tentativa deve funcionar");
    drop(subscriber_guard);

    let events = captured_events.events();
    let retry = events
        .iter()
        .find(|event| event.fields.get("retry") == Some(&"1".to_owned()))
        .expect("o retry deve ser registrado");

    assert_eq!("WARN", retry.level);
    assert_eq!(Some(&"1".to_owned()), retry.fields.get("max_retries"));
    assert_eq!(
        Some(&"PERSISTENCE_FAILURE".to_owned()),
        retry.fields.get("reason_code")
    );
    assert!(!format!("{events:?}").contains("database password"));
}

fn executor(
    max_retries: usize,
    timeout: Duration,
) -> humanizar_units::infrastructure::resilience::RetryExecutor {
    let settings = RetrySettings::new(
        max_retries,
        timeout,
        Duration::from_millis(1),
        Duration::from_millis(2),
    )
    .expect("as configurações devem ser válidas")
    .with_jitter_seed(42);

    RetryConfig::new(settings).executor()
}
