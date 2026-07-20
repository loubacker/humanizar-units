#![forbid(unsafe_code)]

use chrono::NaiveDate;
use humanizar_units::domain::model::{Municipio, Unit};
use uuid::Uuid;

#[test]
fn new_models_expose_business_fields_without_persisted_state() {
    let municipio = Municipio::new("3550308", "Sao Paulo", "SP");
    let municipio_id = Uuid::new_v4();
    let unit = Unit::new(
        municipio_id,
        "Unidade Centro",
        "Humanizar Centro Ltda",
        "Rua Central",
        "100",
        Some("Sala 1".to_owned()),
        "Centro",
        "01001000",
        "12345678000190",
    );

    assert_eq!(None, municipio.id());
    assert_eq!("3550308", municipio.codigo_ibge());
    assert_eq!(None, municipio.created_at());
    assert_eq!(None, unit.id());
    assert_eq!(municipio_id, unit.municipio_id());
    assert_eq!(Some("Sala 1"), unit.complemento());
    assert_eq!(None, unit.updated_at());
}

#[test]
fn restored_models_preserve_identity_and_timestamps() {
    let municipio_id = Uuid::new_v4();
    let unit_id = Uuid::new_v4();
    let created_at = NaiveDate::from_ymd_opt(2026, 7, 17)
        .expect("data valida")
        .and_hms_opt(10, 0, 0)
        .expect("hora valida");
    let updated_at = NaiveDate::from_ymd_opt(2026, 7, 17)
        .expect("data valida")
        .and_hms_opt(11, 0, 0)
        .expect("hora valida");

    let municipio = Municipio::restore(
        Some(municipio_id),
        "3550308",
        "Sao Paulo",
        "SP",
        Some(created_at),
        Some(updated_at),
    );
    let unit = Unit::restore(
        Some(unit_id),
        municipio_id,
        "Unidade Centro",
        "Humanizar Centro Ltda",
        "Rua Central",
        "100",
        None,
        "Centro",
        "01001000",
        "12345678000190",
        Some(created_at),
        Some(updated_at),
    );

    assert_eq!(Some(municipio_id), municipio.id());
    assert_eq!(Some(created_at), municipio.created_at());
    assert_eq!(Some(unit_id), unit.id());
    assert_eq!(Some(updated_at), unit.updated_at());
}
