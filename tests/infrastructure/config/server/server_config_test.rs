#![forbid(unsafe_code)]

use humanizar_units::infrastructure::config::ServerConfig;

#[test]
fn server_config_exposes_the_legacy_port_and_explicit_bind_address() {
    let config = ServerConfig::new("0.0.0.0", 9095).expect("a configuracao deve ser valida");

    assert_eq!("0.0.0.0", config.host());
    assert_eq!(9095, config.port());
    assert_eq!("0.0.0.0:9095", config.bind_address());
}

#[test]
fn server_config_rejects_empty_host_and_zero_port() {
    assert!(ServerConfig::new(" ", 9095).is_err());
    assert!(ServerConfig::new("127.0.0.1", 0).is_err());
}
