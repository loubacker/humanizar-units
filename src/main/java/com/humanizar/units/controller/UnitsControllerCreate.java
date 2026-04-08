package com.humanizar.units.controller;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.humanizar.units.dto.UnitDTO;
import com.humanizar.units.service.UnitsServiceCreate;

@RestController
@RequestMapping("api/v1")
public class UnitsControllerCreate {

    private static final Logger log = LoggerFactory.getLogger(UnitsControllerCreate.class);

    private final UnitsServiceCreate unitsServiceCreate;

    public UnitsControllerCreate(UnitsServiceCreate unitsServiceCreate) {
        this.unitsServiceCreate = unitsServiceCreate;
    }

    @PostMapping("/unit/register")
    @PreAuthorize("hasRole('ADMINISTRADOR')")
    public ResponseEntity<UnitDTO> criarUnit(@RequestBody UnitDTO unitDTO) {
        log.info("Recebido POST /api/v1/unit/register. operacao=CREATE, nomeUnidade={}",
                unitDTO != null ? unitDTO.unitName() : null);
        UnitDTO createdUnit = unitsServiceCreate.criarUnit(unitDTO);
        log.info("POST /api/v1/unit/register concluido com sucesso. unitId={}",
                createdUnit != null ? createdUnit.unitId() : null);
        return ResponseEntity.status(HttpStatus.CREATED).body(createdUnit);
    }
}
