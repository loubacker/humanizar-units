package com.humanizar.units.mapper;

import org.springframework.stereotype.Component;

import com.humanizar.units.dto.MunicipioDTO;
import com.humanizar.units.exception.UnitException;
import com.humanizar.units.model.Municipio;
import com.humanizar.units.model.enums.ReasonCode;

@Component
public class MunicipioMapper {

    public void validate(MunicipioDTO municipioDTO) {
        if (municipioDTO == null) {
            throw new UnitException(ReasonCode.VALIDATION_ERROR, "Municipio nao pode ser nulo.");
        }

        validateRequired("codigoIbge", municipioDTO.codigoIbge());
        validateRequired("nome", municipioDTO.nome());
        validateRequired("uf", municipioDTO.uf());
    }

    public Municipio toEntity(MunicipioDTO municipioDTO) {
        validate(municipioDTO);

        return Municipio.builder()
                .id(municipioDTO.municipioId())
                .codigoIbge(municipioDTO.codigoIbge())
                .nome(municipioDTO.nome())
                .uf(municipioDTO.uf())
                .build();
    }

    public MunicipioDTO toDTO(Municipio entity) {
        if (entity == null) {
            throw new UnitException(ReasonCode.PERSISTENCE_FAILURE, "Entidade de municipio nao pode ser nula.");
        }

        return new MunicipioDTO(
                entity.getId(),
                entity.getCodigoIbge(),
                entity.getNome(),
                entity.getUf());
    }

    public void applyUpdates(Municipio entity, MunicipioDTO municipioDTO) {
        validate(municipioDTO);

        entity.setCodigoIbge(municipioDTO.codigoIbge());
        entity.setNome(municipioDTO.nome());
        entity.setUf(municipioDTO.uf());
    }

    private void validateRequired(String fieldName, String value) {
        if (value == null || value.isBlank()) {
            throw new UnitException(
                    ReasonCode.VALIDATION_ERROR,
                    "Campo obrigatorio invalido: " + fieldName + ".");
        }
    }
}
