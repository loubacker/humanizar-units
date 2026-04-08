package com.humanizar.units.dto;

import java.util.UUID;

public record UnitDTO(
        UUID unitId,
        String unitName,
        String razaoSocial,
        String endereco,
        String numero,
        String complemento,
        String bairro,
        String cidade,
        String estado,
        String cep,
        String cnpj) {
}
