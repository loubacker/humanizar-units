#![forbid(unsafe_code)]

use chrono::NaiveDate;
use humanizar_units::application::dto::{MunicipioDto, UnitDto};
use humanizar_units::application::mapper::{MunicipioMapper, UnitMapper};
use humanizar_units::domain::model::enums::ReasonCode;
use humanizar_units::domain::model::{Municipio, Unit};
use uuid::Uuid;

#[test]
fn municipio_mapper_validates_once_and_preserves_identity_on_update() {
    let municipio_id = Uuid::new_v4();
    let created_at = timestamp(2026, 1, 2);
    let updated_at = timestamp(2026, 2, 3);
    let current = Municipio::restore(
        Some(municipio_id),
        "3550308",
        "Sao Paulo",
        "SP",
        Some(created_at),
        Some(updated_at),
    );
    let dto = MunicipioDto::new(
        Some(Uuid::new_v4()),
        Some("3304557".to_owned()),
        Some("Rio de Janeiro".to_owned()),
        Some("RJ".to_owned()),
    );

    let updated =
        MunicipioMapper::to_updated_domain(&current, &dto).expect("o municipio deve ser mapeado");

    assert_eq!(Some(municipio_id), updated.id());
    assert_eq!(Some(created_at), updated.created_at());
    assert_eq!(Some(updated_at), updated.updated_at());
    assert_eq!("3304557", updated.codigo_ibge());
}

#[test]
fn unit_mapper_uses_path_municipio_and_rejects_blank_required_fields() {
    let path_municipio_id = Uuid::new_v4();
    let dto = unit_dto(Some(Uuid::new_v4()), " ");

    let error =
        UnitMapper::to_new_domain(path_municipio_id, &dto).expect_err("nome em branco deve falhar");

    assert_eq!(ReasonCode::ValidationError, error.reason_code());
    assert_eq!("Campo obrigatorio invalido: unitName.", error.message());

    let valid = unit_dto(Some(Uuid::new_v4()), "Unidade Centro");
    let unit = UnitMapper::to_new_domain(path_municipio_id, &valid)
        .expect("a unit valida deve ser mapeada");
    assert_eq!(path_municipio_id, unit.municipio_id());
}

#[test]
fn unit_mapper_preserves_domain_identity_and_timestamps() {
    let unit_id = Uuid::new_v4();
    let municipio_id = Uuid::new_v4();
    let created_at = timestamp(2026, 1, 2);
    let updated_at = timestamp(2026, 2, 3);
    let current = Unit::restore(
        Some(unit_id),
        municipio_id,
        "Original",
        "Razao",
        "Endereco",
        "1",
        None,
        "Centro",
        "01001000",
        "12345678000190",
        Some(created_at),
        Some(updated_at),
    );
    let dto = unit_dto(Some(Uuid::new_v4()), "Atualizada");

    let updated =
        UnitMapper::to_updated_domain(&current, &dto).expect("a atualizacao deve ser mapeada");

    assert_eq!(Some(unit_id), updated.id());
    assert_eq!(municipio_id, updated.municipio_id());
    assert_eq!(Some(created_at), updated.created_at());
    assert_eq!(Some(updated_at), updated.updated_at());
}

fn unit_dto(body_municipio_id: Option<Uuid>, unit_name: &str) -> UnitDto {
    UnitDto::new(
        Some(Uuid::new_v4()),
        body_municipio_id,
        Some(unit_name.to_owned()),
        Some("Humanizar Ltda".to_owned()),
        Some("Rua Um".to_owned()),
        Some("10".to_owned()),
        None,
        Some("Centro".to_owned()),
        Some("01001000".to_owned()),
        Some("12345678000190".to_owned()),
    )
}

fn timestamp(year: i32, month: u32, day: u32) -> chrono::NaiveDateTime {
    NaiveDate::from_ymd_opt(year, month, day)
        .expect("a data deve ser valida")
        .and_hms_opt(10, 30, 0)
        .expect("a hora deve ser valida")
}
