#![forbid(unsafe_code)]

use std::sync::Arc;

use humanizar_units::application::dto::{MunicipioDto, UnitDto, UnitIdsQueryDto};
use humanizar_units::application::service::{MunicipioService, UnitService};
use humanizar_units::domain::model::enums::ReasonCode;
use humanizar_units::domain::port::{MunicipioPort, UnitPort};
use uuid::Uuid;

#[path = "../../support/in_memory_ports.rs"]
mod in_memory_ports;

use in_memory_ports::InMemoryPorts;

#[tokio::test]
async fn services_execute_domain_rules_without_repeating_port_or_mapping_logic() {
    let ports = Arc::new(InMemoryPorts::default());
    let municipio_port: Arc<dyn MunicipioPort> = ports.clone();
    let unit_port: Arc<dyn UnitPort> = ports.clone();
    let municipio_service = MunicipioService::new(municipio_port.clone(), unit_port.clone());
    let unit_service = UnitService::new(unit_port, municipio_port);
    let municipio = municipio_service
        .create(municipio_dto("3550308"))
        .await
        .expect("o municipio deve ser criado");
    let municipio_id = municipio
        .municipio_id()
        .expect("o municipio criado deve possuir ID");

    let duplicate = municipio_service
        .create(municipio_dto("3550308"))
        .await
        .expect_err("codigo IBGE duplicado deve falhar");
    assert_eq!(ReasonCode::MunicipioDuplicated, duplicate.reason_code());

    let created_unit = unit_service
        .create(municipio_id, unit_dto("12345678000190"))
        .await
        .expect("a unit deve ser criada");
    assert_eq!(Some(municipio_id), created_unit.municipio_id());

    let duplicate_unit = unit_service
        .create(municipio_id, unit_dto("12345678000190"))
        .await
        .expect_err("CNPJ duplicado deve falhar");
    assert_eq!(ReasonCode::UnitDuplicated, duplicate_unit.reason_code());

    let municipio_with_units = municipio_service
        .delete(municipio_id)
        .await
        .expect_err("municipio com units nao pode ser excluido");
    assert_eq!(
        ReasonCode::MunicipioHasUnits,
        municipio_with_units.reason_code()
    );
}

#[tokio::test]
async fn empty_batch_returns_without_calling_the_port() {
    let ports = Arc::new(InMemoryPorts::default());
    let municipio_port: Arc<dyn MunicipioPort> = ports.clone();
    let unit_port: Arc<dyn UnitPort> = ports.clone();
    let service = UnitService::new(unit_port, municipio_port);

    let units = service
        .find_by_ids(UnitIdsQueryDto::default())
        .await
        .expect("a busca vazia deve funcionar");

    assert!(units.is_empty());
    assert_eq!(0, ports.find_by_ids_calls());
}

#[tokio::test]
async fn unit_create_requires_an_existing_municipio() {
    let ports = Arc::new(InMemoryPorts::default());
    let municipio_port: Arc<dyn MunicipioPort> = ports.clone();
    let unit_port: Arc<dyn UnitPort> = ports;
    let service = UnitService::new(unit_port, municipio_port);

    let error = service
        .create(Uuid::new_v4(), unit_dto("12345678000190"))
        .await
        .expect_err("municipio ausente deve falhar");

    assert_eq!(ReasonCode::MunicipioNotFound, error.reason_code());
}

fn municipio_dto(codigo_ibge: &str) -> MunicipioDto {
    MunicipioDto::new(
        None,
        Some(codigo_ibge.to_owned()),
        Some("Municipio Teste".to_owned()),
        Some("SP".to_owned()),
    )
}

fn unit_dto(cnpj: &str) -> UnitDto {
    UnitDto::new(
        None,
        None,
        Some("Unidade Centro".to_owned()),
        Some("Humanizar Ltda".to_owned()),
        Some("Rua Um".to_owned()),
        Some("10".to_owned()),
        None,
        Some("Centro".to_owned()),
        Some("01001000".to_owned()),
        Some(cnpj.to_owned()),
    )
}
