package com.humanizar.units.mapper;

import org.springframework.stereotype.Component;

import com.humanizar.units.dto.UnitDTO;
import com.humanizar.units.exception.UnitException;
import com.humanizar.units.model.Units;
import com.humanizar.units.model.enums.ReasonCode;

@Component
public class UnitMapper {

    public void validate(UnitDTO unitDTO) {
        if (unitDTO == null) {
            throw new UnitException(ReasonCode.VALIDATION_ERROR, "Payload da unidade nao pode ser nulo.");
        }

        validateRequired("unitName", unitDTO.unitName());
        validateRequired("razaoSocial", unitDTO.razaoSocial());
        validateRequired("endereco", unitDTO.endereco());
        validateRequired("numero", unitDTO.numero());
        validateRequired("bairro", unitDTO.bairro());
        validateRequired("cidade", unitDTO.cidade());
        validateRequired("estado", unitDTO.estado());
        validateRequired("cep", unitDTO.cep());
        validateRequired("cnpj", unitDTO.cnpj());
    }

    public Units toEntity(UnitDTO dto) {
        validate(dto);

        return Units.builder()
                .id(dto.unitId())
                .unitName(dto.unitName())
                .razaoSocial(dto.razaoSocial())
                .endereco(dto.endereco())
                .numero(dto.numero())
                .complemento(dto.complemento())
                .bairro(dto.bairro())
                .cidade(dto.cidade())
                .estado(dto.estado())
                .cep(dto.cep())
                .cnpj(dto.cnpj())
                .build();
    }

    public UnitDTO toDTO(Units entity) {
        if (entity == null) {
            return null;
        }

        return new UnitDTO(
                entity.getId(),
                entity.getUnitName(),
                entity.getRazaoSocial(),
                entity.getEndereco(),
                entity.getNumero(),
                entity.getComplemento(),
                entity.getBairro(),
                entity.getCidade(),
                entity.getEstado(),
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
        entity.setCidade(unitDTO.cidade());
        entity.setEstado(unitDTO.estado());
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
