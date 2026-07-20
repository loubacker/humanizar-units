#![forbid(unsafe_code)]

use std::time::Duration;

use humanizar_units::infrastructure::config::{RetryConfig, RetrySettings};

#[test]
fn settings_preserve_the_java_retry_contract() {
    let settings = RetrySettings::new(
        2,
        Duration::from_secs(30),
        Duration::from_millis(100),
        Duration::from_secs(1),
    )
    .expect("as configurações devem ser válidas");
    let config = RetryConfig::new(settings);

    assert_eq!(2, config.settings().max_retries());
    assert_eq!(Duration::from_secs(30), config.settings().timeout());
    assert_eq!(
        Duration::from_millis(100),
        config.settings().minimum_delay()
    );
    assert_eq!(Duration::from_secs(1), config.settings().maximum_delay());
}

#[test]
fn settings_allow_disabling_retries_without_disabling_the_initial_attempt() {
    let settings = RetrySettings::new(
        0,
        Duration::from_secs(1),
        Duration::from_millis(1),
        Duration::from_millis(1),
    );

    assert!(settings.is_ok());
}

#[test]
fn settings_reject_zero_durations_and_inverted_delays() {
    assert!(
        RetrySettings::new(
            2,
            Duration::ZERO,
            Duration::from_millis(1),
            Duration::from_millis(2),
        )
        .is_err()
    );
    assert!(
        RetrySettings::new(
            2,
            Duration::from_secs(1),
            Duration::from_millis(2),
            Duration::from_millis(1),
        )
        .is_err()
    );
}
