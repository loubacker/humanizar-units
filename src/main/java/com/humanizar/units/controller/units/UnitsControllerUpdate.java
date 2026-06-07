package com.humanizar.units.controller.units;

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
import com.humanizar.units.service.units.UnitsServiceUpdate;

@RestController
@RequestMapping("/api/v1/municipio/{municipioId}/units")
public class UnitsControllerUpdate {

    private static final Logger log = LoggerFactory.getLogger(UnitsControllerUpdate.class);

    private final UnitsServiceUpdate unitsServiceUpdate;

    public UnitsControllerUpdate(UnitsServiceUpdate unitsServiceUpdate) {
        this.unitsServiceUpdate = unitsServiceUpdate;
    }

    @PutMapping("/update/{unitId}")
    @PreAuthorize("hasRole('ADMINISTRADOR')")
    public ResponseEntity<UnitDTO> atualizarUnit(@PathVariable UUID municipioId, @PathVariable UUID unitId,
            @RequestBody UnitDTO unitDTO) {
        log.info("Recebido PUT /api/v1/municipio/{}/units/update/{}. operacao=UPDATE", municipioId, unitId);
        UnitDTO atualizado = unitsServiceUpdate.atualizarUnit(municipioId, unitId, unitDTO);
        return ResponseEntity.status(HttpStatus.OK).body(atualizado);
    }
}
