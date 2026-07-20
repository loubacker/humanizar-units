use uuid::Uuid;

use crate::application::dto::UnitDto;
use crate::domain::exception::UnitException;
use crate::domain::model::Unit;

use super::field_validator::required_field;

pub struct UnitMapper;

impl UnitMapper {
    pub fn to_new_domain(municipio_id: Uuid, dto: &UnitDto) -> Result<Unit, UnitException> {
        let fields = validated_fields(dto)?;

        Ok(Unit::new(
            municipio_id,
            fields.unit_name,
            fields.razao_social,
            fields.endereco,
            fields.numero,
            dto.complemento().map(str::to_owned),
            fields.bairro,
            fields.cep,
            fields.cnpj,
        ))
    }

    pub fn to_updated_domain(current: &Unit, dto: &UnitDto) -> Result<Unit, UnitException> {
        let fields = validated_fields(dto)?;

        Ok(Unit::restore(
            current.id(),
            current.municipio_id(),
            fields.unit_name,
            fields.razao_social,
            fields.endereco,
            fields.numero,
            dto.complemento().map(str::to_owned),
            fields.bairro,
            fields.cep,
            fields.cnpj,
            current.created_at(),
            current.updated_at(),
        ))
    }

    pub fn to_dto(unit: Unit) -> UnitDto {
        UnitDto::new(
            unit.id(),
            Some(unit.municipio_id()),
            Some(unit.unit_name().to_owned()),
            Some(unit.razao_social().to_owned()),
            Some(unit.endereco().to_owned()),
            Some(unit.numero().to_owned()),
            unit.complemento().map(str::to_owned),
            Some(unit.bairro().to_owned()),
            Some(unit.cep().to_owned()),
            Some(unit.cnpj().to_owned()),
        )
    }

    pub fn to_dtos(units: Vec<Unit>) -> Vec<UnitDto> {
        units.into_iter().map(Self::to_dto).collect()
    }
}

struct UnitFields {
    unit_name: String,
    razao_social: String,
    endereco: String,
    numero: String,
    bairro: String,
    cep: String,
    cnpj: String,
}

fn validated_fields(dto: &UnitDto) -> Result<UnitFields, UnitException> {
    Ok(UnitFields {
        unit_name: required_field("unitName", dto.unit_name())?,
        razao_social: required_field("razaoSocial", dto.razao_social())?,
        endereco: required_field("endereco", dto.endereco())?,
        numero: required_field("numero", dto.numero())?,
        bairro: required_field("bairro", dto.bairro())?,
        cep: required_field("cep", dto.cep())?,
        cnpj: required_field("cnpj", dto.cnpj())?,
    })
}
