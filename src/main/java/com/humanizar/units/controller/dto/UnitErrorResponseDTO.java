package com.humanizar.units.controller.dto;

import java.time.OffsetDateTime;

public record UnitErrorResponseDTO(
        int status,
        String reasonCode,
        String message,
        String path,
        OffsetDateTime timestamp) {
}
