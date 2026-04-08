package com.humanizar.units.controller;

import java.util.UUID;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.humanizar.units.dto.UnitDTO;
import com.humanizar.units.service.UnitsServiceUpdate;

@RestController
@RequestMapping("/api/v1")
public class UnitsControllerUpdate {

    private static final Logger log = LoggerFactory.getLogger(UnitsControllerUpdate.class);

    private final UnitsServiceUpdate unitsServiceUpdate;

    public UnitsControllerUpdate(UnitsServiceUpdate unitsServiceUpdate) {
        this.unitsServiceUpdate = unitsServiceUpdate;
    }

    @PutMapping("/unit/update/{unitId}")
    @PreAuthorize("hasRole('ADMINISTRADOR')")
    public ResponseEntity<UnitDTO> atualizarUnit(@PathVariable UUID unitId,
            @RequestBody UnitDTO unitDTO) {
        log.info("Recebido PUT /api/v1/unit/update/{}. operacao=UPDATE", unitId);
        UnitDTO atualizado = unitsServiceUpdate.atualizarUnit(unitId, unitDTO);
        log.info("PUT /api/v1/unit/update/{} concluido com sucesso.", unitId);
        return ResponseEntity.status(HttpStatus.OK).body(atualizado);
    }
}
