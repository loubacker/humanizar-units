package com.humanizar.units.controller.units;

import java.util.UUID;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.humanizar.units.service.units.UnitsServiceDelete;

@RestController
@RequestMapping("/api/v1/municipio/{municipioId}/units")
public class UnitsControllerDelete {

    private static final Logger log = LoggerFactory.getLogger(UnitsControllerDelete.class);

    private final UnitsServiceDelete unitsServiceDelete;

    public UnitsControllerDelete(UnitsServiceDelete unitsServiceDelete) {
        this.unitsServiceDelete = unitsServiceDelete;
    }

    @DeleteMapping("/delete/{unitId}")
    @PreAuthorize("hasRole('ADMINISTRADOR')")
    public ResponseEntity<String> deletarUnit(@PathVariable UUID municipioId, @PathVariable UUID unitId) {
        log.info("Recebido DELETE /api/v1/municipio/{}/units/delete/{}. operacao=DELETE", municipioId, unitId);
        unitsServiceDelete.deletarUnit(municipioId, unitId);
        return ResponseEntity.ok("Unidade excluida com sucesso.");
    }
}
