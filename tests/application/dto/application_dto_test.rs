#![forbid(unsafe_code)]

use humanizar_units::application::dto::{MunicipioDto, UnitDto, UnitIdsQueryDto};
use serde_json::json;
use uuid::Uuid;

#[test]
fn municipio_dto_preserves_the_legacy_json_contract() {
    let municipio_id = Uuid::new_v4();
    let dto = MunicipioDto::new(
        Some(municipio_id),
        Some("3550308".to_owned()),
        Some("Sao Paulo".to_owned()),
        Some("SP".to_owned()),
    );

    assert_eq!(
        json!({
            "municipioId": municipio_id,
            "codigoIbge": "3550308",
            "nome": "Sao Paulo",
            "uf": "SP"
        }),
        serde_json::to_value(dto).expect("o DTO deve ser serializado")
    );
}

#[test]
fn unit_dto_preserves_nulls_and_ignores_unknown_properties() {
    let value = json!({
        "unitId": null,
        "municipioId": null,
        "unitName": "Unidade Centro",
        "razaoSocial": "Humanizar Ltda",
        "endereco": "Rua Um",
        "numero": "10",
        "complemento": null,
        "bairro": "Centro",
        "cep": "01001000",
        "cnpj": "12345678000190",
        "unknownProperty": "ignored"
    });
    let dto: UnitDto = serde_json::from_value(value).expect("campos extras devem ser ignorados");

    assert_eq!(None, dto.unit_id());
    assert_eq!(None, dto.municipio_id());
    assert_eq!(Some("Unidade Centro"), dto.unit_name());
    assert_eq!(None, dto.complemento());
}

#[test]
fn unit_ids_query_dto_is_immutable_and_defaults_to_empty() {
    assert!(UnitIdsQueryDto::default().ids().is_empty());

    let unit_id = Uuid::new_v4();
    assert_eq!(&[unit_id], UnitIdsQueryDto::new(vec![unit_id]).ids());
}
