#![forbid(unsafe_code)]

#[path = "../../../support/tracing_capture.rs"]
mod tracing_capture;

use axum::body::to_bytes;
use axum::response::IntoResponse;
use humanizar_units::domain::exception::{PersistenceException, SecurityException, UnitException};
use humanizar_units::domain::model::enums::ReasonCode;
use humanizar_units::infrastructure::controller::handler::{
    PersistenceHttpError, SecurityHttpError, UnitHttpError,
};
use serde_json::{Value, json};
use tracing_capture::CapturedEvents;
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;

#[tokio::test]
async fn should_map_every_reason_code_to_shared_http_error_contract() {
    let cases = [
        (ReasonCode::InvalidRequest, 400, "Bad Request"),
        (ReasonCode::ValidationError, 400, "Bad Request"),
        (ReasonCode::AuthenticationFailure, 401, "Unauthorized"),
        (ReasonCode::AuthorizationFailure, 403, "Forbidden"),
        (ReasonCode::UnitNotFound, 404, "Not Found"),
        (ReasonCode::MunicipioNotFound, 404, "Not Found"),
        (ReasonCode::UnitDuplicated, 409, "Conflict"),
        (ReasonCode::MunicipioDuplicated, 409, "Conflict"),
        (ReasonCode::MunicipioHasUnits, 409, "Conflict"),
        (ReasonCode::UnexpectedError, 500, "Internal Server Error"),
        (ReasonCode::PersistenceFailure, 503, "Service Unavailable"),
    ];

    for (reason_code, expected_status, expected_error) in cases {
        let path = "/api/v1/municipio/123/units/456";
        let response = UnitHttpError::new(UnitException::new(reason_code), path).into_response();

        assert_eq!(response.status().as_u16(), expected_status);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json_body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json_body["status"], json!(expected_status));
        assert_eq!(json_body["error"], json!(expected_error));
        assert_eq!(json_body["reasonCode"], json!(reason_code.code()));
        assert_eq!(json_body["message"], json!(reason_code.message()));
        assert_eq!(json_body["path"], json!(path));
        assert!(json_body["timestamp"].as_str().is_some());
    }
}

#[tokio::test]
async fn should_not_expose_technical_source_in_error_response() {
    let exception = UnitException::with_source(
        ReasonCode::PersistenceFailure,
        std::io::Error::other("database password rejected"),
    );
    let response = UnitHttpError::new(exception, "/api/v1/units").into_response();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json_body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        json_body["message"],
        json!("Falha de persistência no banco de dados.")
    );
    assert!(
        !body
            .windows("database password rejected".len())
            .any(|window| { window == "database password rejected".as_bytes() })
    );
}

#[tokio::test]
async fn security_alias_uses_the_same_transport_contract() {
    let response =
        SecurityHttpError::new(SecurityException::authorization(), "/api/v1/units").into_response();
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("o body deve ser lido");
    let body: Value = serde_json::from_slice(&body).expect("o body deve ser JSON");

    assert_eq!(403, status.as_u16());
    assert_eq!(json!("AUTHORIZATION_FAILURE"), body["reasonCode"]);
    assert_eq!(json!("/api/v1/units"), body["path"]);
}

#[tokio::test]
async fn persistence_alias_returns_503_without_exposing_the_technical_source() {
    let response = PersistenceHttpError::new(
        PersistenceException::transient_query(std::io::Error::other("SQLSTATE 40001")),
        "/api/v1/units",
    )
    .into_response();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("o body deve ser lido");
    let body: Value = serde_json::from_slice(&bytes).expect("o body deve ser JSON");

    assert_eq!(503, status.as_u16());
    assert_eq!(json!("PERSISTENCE_FAILURE"), body["reasonCode"]);
    assert_eq!(json!("/api/v1/units"), body["path"]);
    assert!(!String::from_utf8_lossy(&bytes).contains("SQLSTATE 40001"));
}

#[test]
fn transport_logs_warn_for_4xx_and_error_for_5xx_with_structured_fields() {
    let captured_events = CapturedEvents::default();
    let subscriber = Registry::default().with(captured_events.clone());

    tracing::subscriber::with_default(subscriber, || {
        let _authentication_response = SecurityHttpError::new(
            SecurityException::authentication_with_source(std::io::Error::other("JWT inválido")),
            "/api/v1/units",
        )
        .into_response();
        let _unexpected_response = UnitHttpError::new(
            UnitException::with_source(
                ReasonCode::UnexpectedError,
                std::io::Error::other("falha interna"),
            ),
            "/api/v1/units",
        )
        .into_response();
    });

    let events = captured_events.events();
    let authentication = events
        .iter()
        .find(|event| event.fields.get("reason_code") == Some(&"AUTHENTICATION_FAILURE".to_owned()))
        .expect("o evento de autenticação deve ser registrado");
    let unexpected = events
        .iter()
        .find(|event| event.fields.get("reason_code") == Some(&"UNEXPECTED_ERROR".to_owned()))
        .expect("o evento inesperado deve ser registrado");

    assert_eq!("WARN", authentication.level);
    assert_eq!(Some(&"401".to_owned()), authentication.fields.get("status"));
    assert_eq!(
        Some(&"/api/v1/units".to_owned()),
        authentication.fields.get("path")
    );
    assert_eq!("ERROR", unexpected.level);
    assert_eq!(Some(&"500".to_owned()), unexpected.fields.get("status"));
    assert!(
        !format!("{events:?}").contains("Bearer "),
        "o log não pode conter credenciais Bearer"
    );
}
