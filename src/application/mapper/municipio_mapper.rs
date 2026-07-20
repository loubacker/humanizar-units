use crate::application::dto::MunicipioDto;
use crate::domain::exception::UnitException;
use crate::domain::model::Municipio;

use super::field_validator::required_field;

pub struct MunicipioMapper;

impl MunicipioMapper {
    pub fn to_new_domain(dto: &MunicipioDto) -> Result<Municipio, UnitException> {
        let fields = validated_fields(dto)?;

        Ok(Municipio::new(fields.codigo_ibge, fields.nome, fields.uf))
    }

    pub fn to_updated_domain(
        current: &Municipio,
        dto: &MunicipioDto,
    ) -> Result<Municipio, UnitException> {
        let fields = validated_fields(dto)?;

        Ok(Municipio::restore(
            current.id(),
            fields.codigo_ibge,
            fields.nome,
            fields.uf,
            current.created_at(),
            current.updated_at(),
        ))
    }

    pub fn to_dto(municipio: Municipio) -> MunicipioDto {
        MunicipioDto::new(
            municipio.id(),
            Some(municipio.codigo_ibge().to_owned()),
            Some(municipio.nome().to_owned()),
            Some(municipio.uf().to_owned()),
        )
    }

    pub fn to_dtos(municipios: Vec<Municipio>) -> Vec<MunicipioDto> {
        municipios.into_iter().map(Self::to_dto).collect()
    }
}

struct MunicipioFields {
    codigo_ibge: String,
    nome: String,
    uf: String,
}

fn validated_fields(dto: &MunicipioDto) -> Result<MunicipioFields, UnitException> {
    Ok(MunicipioFields {
        codigo_ibge: required_field("codigoIbge", dto.codigo_ibge())?,
        nome: required_field("nome", dto.nome())?,
        uf: required_field("uf", dto.uf())?,
    })
}
