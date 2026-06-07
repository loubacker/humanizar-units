package com.humanizar.units.controller.units;

import java.util.List;
import java.util.UUID;

import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

import com.humanizar.units.config.ResilientMethodsConfig.Retry;
import com.humanizar.units.dto.UnitDTO;
import com.humanizar.units.service.units.UnitsServiceRetrieve;

@RestController
@RequestMapping("/api/v1/units")
public class UnitsControllerBatchRetrieve {

    private final UnitsServiceRetrieve unitsServiceRetrieve;

    public UnitsControllerBatchRetrieve(UnitsServiceRetrieve unitsServiceRetrieve) {
        this.unitsServiceRetrieve = unitsServiceRetrieve;
    }

    @Retry
    @GetMapping
    @PreAuthorize("isAuthenticated()")
    public List<UnitDTO> obterPorIds(@RequestParam(value = "ids", required = false) List<UUID> ids) {
        return unitsServiceRetrieve.obterPorIds(ids != null ? ids : List.of());
    }
}
