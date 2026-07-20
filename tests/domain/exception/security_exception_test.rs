#![forbid(unsafe_code)]

use std::error::Error;

use humanizar_units::domain::exception::SecurityException;
use humanizar_units::domain::model::enums::ReasonCode;

#[test]
fn authentication_factory_uses_the_authentication_contract() {
    let exception = SecurityException::authentication();

    assert_eq!(ReasonCode::AuthenticationFailure, exception.reason_code());
    assert_eq!(
        ReasonCode::AuthenticationFailure.message(),
        exception.message()
    );
    assert!(exception.source().is_none());
}

#[test]
fn authorization_factory_uses_the_authorization_contract() {
    let exception = SecurityException::authorization();

    assert_eq!(ReasonCode::AuthorizationFailure, exception.reason_code());
    assert_eq!(
        ReasonCode::AuthorizationFailure.message(),
        exception.message()
    );
    assert!(exception.source().is_none());
}

#[test]
fn security_factories_preserve_technical_sources() {
    let authentication = SecurityException::authentication_with_source(std::io::Error::other(
        "assinatura JWT inválida",
    ));
    let authorization = SecurityException::authorization_with_source(std::io::Error::other(
        "role obrigatória ausente",
    ));

    assert_eq!(
        "assinatura JWT inválida",
        authentication
            .source()
            .expect("a causa de autenticação deve ser preservada")
            .to_string()
    );
    assert_eq!(
        "role obrigatória ausente",
        authorization
            .source()
            .expect("a causa de autorização deve ser preservada")
            .to_string()
    );
}
