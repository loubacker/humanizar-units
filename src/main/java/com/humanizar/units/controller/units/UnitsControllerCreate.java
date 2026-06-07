package com.humanizar.units.controller.units;

import java.util.UUID;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.humanizar.units.dto.UnitDTO;
import com.humanizar.units.service.units.UnitsServiceCreate;

@RestController
@RequestMapping("/api/v1/municipio/{municipioId}/units")
public class UnitsControllerCreate {

    private static final Logger log = LoggerFactory.getLogger(UnitsControllerCreate.class);

    private final UnitsServiceCreate unitsServiceCreate;

    public UnitsControllerCreate(UnitsServiceCreate unitsServiceCreate) {
        this.unitsServiceCreate = unitsServiceCreate;
    }

    @PostMapping("/register")
    @PreAuthorize("hasRole('ADMINISTRADOR')")
    public ResponseEntity<UnitDTO> criarUnit(@PathVariable UUID municipioId, @RequestBody UnitDTO unitDTO) {
        log.info("Recebido POST /api/v1/municipio/{}/units/register. operacao=CREATE", municipioId);
        UnitDTO createdUnit = unitsServiceCreate.criarUnit(municipioId, unitDTO);
        log.info("POST /api/v1/municipio/{}/units/register concluido. unitId={}", municipioId,
                createdUnit.unitId());
        return ResponseEntity.status(HttpStatus.CREATED).body(createdUnit);
    }
}
