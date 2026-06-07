package com.humanizar.units.mapper;

import java.util.UUID;

import org.springframework.stereotype.Component;

import com.humanizar.units.dto.UnitDTO;
import com.humanizar.units.exception.UnitException;
import com.humanizar.units.model.Units;
import com.humanizar.units.model.enums.ReasonCode;

@Component
public class UnitMapper {

    public void validate(UnitDTO unitDTO) {
        if (unitDTO == null) {
            throw new UnitException(ReasonCode.VALIDATION_ERROR, "Unidade nao pode ser nula.");
        }

        validateRequired("unitName", unitDTO.unitName());
        validateRequired("razaoSocial", unitDTO.razaoSocial());
        validateRequired("endereco", unitDTO.endereco());
        validateRequired("numero", unitDTO.numero());
        validateRequired("bairro", unitDTO.bairro());
        validateRequired("cep", unitDTO.cep());
        validateRequired("cnpj", unitDTO.cnpj());
    }

    public Units toEntity(UUID municipioId, UnitDTO unitDTO) {
        validate(unitDTO);

        return Units.builder()
                .id(unitDTO.unitId())
                .municipioId(municipioId)
                .unitName(unitDTO.unitName())
                .razaoSocial(unitDTO.razaoSocial())
                .endereco(unitDTO.endereco())
                .numero(unitDTO.numero())
                .complemento(unitDTO.complemento())
                .bairro(unitDTO.bairro())
                .cep(unitDTO.cep())
                .cnpj(unitDTO.cnpj())
                .build();
    }

    public UnitDTO toDTO(Units entity) {
        if (entity == null) {
            throw new UnitException(ReasonCode.PERSISTENCE_FAILURE, "Entidade de unidade nao pode ser nula.");
        }

        return new UnitDTO(
                entity.getId(),
                entity.getMunicipioId(),
                entity.getUnitName(),
                entity.getRazaoSocial(),
                entity.getEndereco(),
                entity.getNumero(),
                entity.getComplemento(),
                entity.getBairro(),
                entity.getCep(),
                entity.getCnpj());
    }

    public void applyUpdates(Units entity, UnitDTO unitDTO) {
        validate(unitDTO);

        entity.setUnitName(unitDTO.unitName());
        entity.setRazaoSocial(unitDTO.razaoSocial());
        entity.setEndereco(unitDTO.endereco());
        entity.setNumero(unitDTO.numero());
        entity.setComplemento(unitDTO.complemento());
        entity.setBairro(unitDTO.bairro());
        entity.setCep(unitDTO.cep());
        entity.setCnpj(unitDTO.cnpj());
    }

    private void validateRequired(String fieldName, String value) {
        if (value == null || value.isBlank()) {
            throw new UnitException(
                    ReasonCode.VALIDATION_ERROR,
                    "Campo obrigatorio invalido: " + fieldName + ".");
        }
    }
}
