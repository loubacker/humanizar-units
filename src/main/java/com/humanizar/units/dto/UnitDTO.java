package com.humanizar.units.dto;

import java.util.UUID;

public record UnitDTO(
        UUID unitId,
        UUID municipioId,
        String unitName,
        String razaoSocial,
        String endereco,
        String numero,
        String complemento,
        String bairro,
        String cep,
        String cnpj) {
}
