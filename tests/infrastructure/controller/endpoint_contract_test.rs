#![forbid(unsafe_code)]

use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::{Method, Request, Response, StatusCode};
use http_body_util::BodyExt;
use humanizar_units::application::service::{MunicipioService, UnitService};
use humanizar_units::domain::port::{MunicipioPort, UnitPort};
use humanizar_units::infrastructure::controller::{ApplicationState, create_router};
use serde_json::{Value, json};
use tower::ServiceExt;

#[path = "../../support/in_memory_ports.rs"]
mod in_memory_ports;
#[path = "../security/test_support.rs"]
mod security_test_support;

use in_memory_ports::InMemoryPorts;
use security_test_support::{JwksServer, TestClaims, rsa_token, security_config};

#[tokio::test]
async fn legacy_endpoints_preserve_payloads_statuses_and_permissions() {
    let jwks = JwksServer::start("active-key").await;
    let security = security_config(&jwks).await;
    let ports = Arc::new(InMemoryPorts::default());
    let app = test_router(ports, &security);
    let administrator_token = rsa_token("active-key", &TestClaims::valid());
    let prefixed_administrator_token = prefixed_administrator_token();
    let coordinator_token = coordinator_token();

    assert_status(
        &app,
        request(Method::GET, "/health", None, None),
        StatusCode::OK,
    )
    .await;
    assert_status(
        &app,
        request(Method::GET, "/api/v1/municipio", None, None),
        StatusCode::UNAUTHORIZED,
    )
    .await;
    assert_status(
        &app,
        request(
            Method::POST,
            "/api/v1/municipio/register",
            Some(&coordinator_token),
            Some(municipio_body()),
        ),
        StatusCode::FORBIDDEN,
    )
    .await;

    let municipio_response = call(
        &app,
        request(
            Method::POST,
            "/api/v1/municipio/register",
            Some(&administrator_token),
            Some(municipio_body()),
        ),
    )
    .await;
    assert_eq!(StatusCode::CREATED, municipio_response.status());
    let municipio = json_body(municipio_response).await;
    let municipio_id = municipio["municipioId"]
        .as_str()
        .expect("a resposta deve possuir municipioId");
    assert_ne!("11111111-1111-4111-8111-111111111111", municipio_id);

    let municipio_detail = call(
        &app,
        request(
            Method::GET,
            &format!("/api/v1/municipio/{municipio_id}"),
            Some(&coordinator_token),
            None,
        ),
    )
    .await;
    assert_eq!(StatusCode::OK, municipio_detail.status());
    assert_eq!(
        municipio_id,
        json_body(municipio_detail).await["municipioId"]
    );

    let municipio_update = call(
        &app,
        request(
            Method::PUT,
            &format!("/api/v1/municipio/update/{municipio_id}"),
            Some(&administrator_token),
            Some(json!({
                "municipioId": "44444444-4444-4444-8444-444444444444",
                "codigoIbge": "3550308",
                "nome": "Sao Paulo Atualizado",
                "uf": "SP"
            })),
        ),
    )
    .await;
    assert_eq!(StatusCode::OK, municipio_update.status());
    let updated_municipio = json_body(municipio_update).await;
    assert_eq!(municipio_id, updated_municipio["municipioId"]);
    assert_eq!("Sao Paulo Atualizado", updated_municipio["nome"]);

    let unit_response = call(
        &app,
        request(
            Method::POST,
            &format!("/api/v1/municipio/{municipio_id}/units/register"),
            Some(&prefixed_administrator_token),
            Some(unit_body()),
        ),
    )
    .await;
    assert_eq!(StatusCode::CREATED, unit_response.status());
    let unit = json_body(unit_response).await;
    let unit_id = unit["unitId"]
        .as_str()
        .expect("a resposta deve possuir unitId");
    assert_eq!(municipio_id, unit["municipioId"]);
    assert_ne!("22222222-2222-4222-8222-222222222222", unit_id);

    let update_unit = call(
        &app,
        request(
            Method::PUT,
            &format!("/api/v1/municipio/{municipio_id}/units/update/{unit_id}"),
            Some(&administrator_token),
            Some(json!({
                "unitId": "55555555-5555-4555-8555-555555555555",
                "municipioId": "66666666-6666-4666-8666-666666666666",
                "unitName": "Unidade Atualizada",
                "razaoSocial": "Humanizar Ltda",
                "endereco": "Rua Um",
                "numero": "10",
                "complemento": "Sala 2",
                "bairro": "Centro",
                "cep": "01001000",
                "cnpj": "12345678000190"
            })),
        ),
    )
    .await;
    assert_eq!(StatusCode::OK, update_unit.status());
    let updated_unit = json_body(update_unit).await;
    assert_eq!(unit_id, updated_unit["unitId"]);
    assert_eq!(municipio_id, updated_unit["municipioId"]);
    assert_eq!("Unidade Atualizada", updated_unit["unitName"]);

    let list = call(
        &app,
        request(
            Method::GET,
            &format!("/api/v1/municipio/{municipio_id}/units"),
            Some(&coordinator_token),
            None,
        ),
    )
    .await;
    assert_eq!(StatusCode::OK, list.status());
    assert_eq!(
        1,
        json_body(list)
            .await
            .as_array()
            .expect("lista esperada")
            .len()
    );

    let empty_batch = call(
        &app,
        request(Method::GET, "/api/v1/units", Some(&coordinator_token), None),
    )
    .await;
    assert_eq!(StatusCode::OK, empty_batch.status());
    assert!(
        json_body(empty_batch)
            .await
            .as_array()
            .expect("lista esperada")
            .is_empty()
    );

    let batch = call(
        &app,
        request(
            Method::GET,
            &format!("/api/v1/units?ids={unit_id}&ids={unit_id},{unit_id}"),
            Some(&coordinator_token),
            None,
        ),
    )
    .await;
    assert_eq!(StatusCode::OK, batch.status());
    assert_eq!(
        1,
        json_body(batch)
            .await
            .as_array()
            .expect("lista esperada")
            .len()
    );

    let blocked_delete = call(
        &app,
        request(
            Method::DELETE,
            &format!("/api/v1/municipio/delete/{municipio_id}"),
            Some(&administrator_token),
            None,
        ),
    )
    .await;
    assert_error(blocked_delete, StatusCode::CONFLICT, "MUNICIPIO_HAS_UNITS").await;

    let delete_unit = call(
        &app,
        request(
            Method::DELETE,
            &format!("/api/v1/municipio/{municipio_id}/units/delete/{unit_id}"),
            Some(&administrator_token),
            None,
        ),
    )
    .await;
    assert_eq!(StatusCode::OK, delete_unit.status());
    assert_eq!(
        "Unidade excluida com sucesso.",
        text_body(delete_unit).await
    );

    let delete_municipio = call(
        &app,
        request(
            Method::DELETE,
            &format!("/api/v1/municipio/delete/{municipio_id}"),
            Some(&administrator_token),
            None,
        ),
    )
    .await;
    assert_eq!(StatusCode::OK, delete_municipio.status());
    assert_eq!(
        "Municipio removido com sucesso.",
        text_body(delete_municipio).await
    );
}

#[tokio::test]
async fn transport_rejections_and_unknown_routes_keep_the_shared_contract() {
    let jwks = JwksServer::start("active-key").await;
    let security = security_config(&jwks).await;
    let app = test_router(Arc::new(InMemoryPorts::default()), &security);
    let token = rsa_token("active-key", &TestClaims::valid());

    let invalid_path = call(
        &app,
        request(
            Method::GET,
            "/api/v1/municipio/not-a-uuid",
            Some(&token),
            None,
        ),
    )
    .await;
    assert_error(invalid_path, StatusCode::BAD_REQUEST, "INVALID_REQUEST").await;

    let invalid_query = call(
        &app,
        request(
            Method::GET,
            "/api/v1/units?ids=not-a-uuid",
            Some(&token),
            None,
        ),
    )
    .await;
    assert_error(invalid_query, StatusCode::BAD_REQUEST, "INVALID_REQUEST").await;

    let invalid_json = call(
        &app,
        Request::builder()
            .method(Method::POST)
            .uri("/api/v1/municipio/register")
            .header(AUTHORIZATION, format!("Bearer {token}"))
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from("{"))
            .expect("a requisicao deve ser criada"),
    )
    .await;
    assert_error(invalid_json, StatusCode::BAD_REQUEST, "INVALID_REQUEST").await;

    let validation_error = call(
        &app,
        request(
            Method::POST,
            "/api/v1/municipio/register",
            Some(&token),
            Some(json!({"codigoIbge": " ", "nome": "Teste", "uf": "SP"})),
        ),
    )
    .await;
    assert_error(
        validation_error,
        StatusCode::BAD_REQUEST,
        "VALIDATION_ERROR",
    )
    .await;

    let unknown = call(
        &app,
        request(Method::GET, "/api/v1/unknown", Some(&token), None),
    )
    .await;
    assert_eq!(StatusCode::NOT_FOUND, unknown.status());
    assert!(text_body(unknown).await.is_empty());
}

fn test_router(
    ports: Arc<InMemoryPorts>,
    security: &humanizar_units::infrastructure::config::SecurityConfig,
) -> Router {
    let municipio_port: Arc<dyn MunicipioPort> = ports.clone();
    let unit_port: Arc<dyn UnitPort> = ports;
    let municipio_service = Arc::new(MunicipioService::new(
        municipio_port.clone(),
        unit_port.clone(),
    ));
    let unit_service = Arc::new(UnitService::new(unit_port, municipio_port));
    let state = ApplicationState::new(municipio_service, unit_service);

    create_router(state, security)
}

fn coordinator_token() -> String {
    let mut claims = TestClaims::valid();
    claims.realm_access = Some(json!({ "roles": ["COORDENADOR"] }));
    rsa_token("active-key", &claims)
}

fn prefixed_administrator_token() -> String {
    let mut claims = TestClaims::valid();
    claims.realm_access = Some(json!({ "roles": ["ROLE_ADMINISTRADOR"] }));
    rsa_token("active-key", &claims)
}

fn request(method: Method, uri: &str, token: Option<&str>, body: Option<Value>) -> Request<Body> {
    let mut builder = Request::builder().method(method).uri(uri);

    if let Some(token) = token {
        builder = builder.header(AUTHORIZATION, format!("Bearer {token}"));
    }

    match body {
        Some(body) => builder
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(body.to_string())),
        None => builder.body(Body::empty()),
    }
    .expect("a requisicao deve ser criada")
}

async fn call(app: &Router, request: Request<Body>) -> Response<Body> {
    app.clone()
        .oneshot(request)
        .await
        .expect("a requisicao deve ser processada")
}

async fn assert_status(app: &Router, request: Request<Body>, expected_status: StatusCode) {
    assert_eq!(expected_status, call(app, request).await.status());
}

async fn assert_error(response: Response<Body>, status: StatusCode, reason_code: &str) {
    assert_eq!(status, response.status());
    let body = json_body(response).await;
    assert_eq!(reason_code, body["reasonCode"]);
}

async fn json_body(response: Response<Body>) -> Value {
    serde_json::from_slice(
        &response
            .into_body()
            .collect()
            .await
            .expect("o body deve ser coletado")
            .to_bytes(),
    )
    .expect("o body deve conter JSON")
}

async fn text_body(response: Response<Body>) -> String {
    String::from_utf8(
        response
            .into_body()
            .collect()
            .await
            .expect("o body deve ser coletado")
            .to_bytes()
            .to_vec(),
    )
    .expect("o body deve conter UTF-8")
}

fn municipio_body() -> Value {
    json!({
        "municipioId": "11111111-1111-4111-8111-111111111111",
        "codigoIbge": "3550308",
        "nome": "Sao Paulo",
        "uf": "SP"
    })
}

fn unit_body() -> Value {
    json!({
        "unitId": "22222222-2222-4222-8222-222222222222",
        "municipioId": "33333333-3333-4333-8333-333333333333",
        "unitName": "Unidade Centro",
        "razaoSocial": "Humanizar Ltda",
        "endereco": "Rua Um",
        "numero": "10",
        "complemento": null,
        "bairro": "Centro",
        "cep": "01001000",
        "cnpj": "12345678000190"
    })
}
