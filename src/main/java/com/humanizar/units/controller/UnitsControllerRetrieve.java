package com.humanizar.units.controller;

import java.util.List;

import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.humanizar.units.config.ResilientMethodsConfig.Retry;
import com.humanizar.units.dto.UnitDTO;
import com.humanizar.units.service.UnitsServiceRetrieve;

@RestController
@RequestMapping("/api/v1")
public class UnitsControllerRetrieve {

    private final UnitsServiceRetrieve unitsServiceRetrieve;

    public UnitsControllerRetrieve(UnitsServiceRetrieve unitsServiceRetrieve) {
        this.unitsServiceRetrieve = unitsServiceRetrieve;
    }

    @Retry
    @GetMapping("/units")
    @PreAuthorize("isAuthenticated()")
    public List<UnitDTO> listarUnits() {
        return unitsServiceRetrieve.listarUnits();
    }
}
