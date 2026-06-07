package com.humanizar.units.dto;

import java.util.UUID;

public record MunicipioDTO(
        UUID municipioId,
        String codigoIbge,
        String nome,
        String uf) {
}
