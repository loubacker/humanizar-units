package com.humanizar.units.controller.municipio;

import java.util.UUID;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.humanizar.units.service.municipio.MunicipioServiceDelete;

@RestController
@RequestMapping("/api/v1/municipio")
public class MunicipioControllerDelete {

    private static final Logger log = LoggerFactory.getLogger(MunicipioControllerDelete.class);

    private final MunicipioServiceDelete municipioServiceDelete;

    public MunicipioControllerDelete(MunicipioServiceDelete municipioServiceDelete) {
        this.municipioServiceDelete = municipioServiceDelete;
    }

    @DeleteMapping("/delete/{municipioId}")
    @PreAuthorize("hasRole('ADMINISTRADOR')")
    public ResponseEntity<String> deletarMunicipio(@PathVariable UUID municipioId) {
        log.info("Recebido DELETE /api/v1/municipio/delete/{}. operacao=DELETE", municipioId);
        municipioServiceDelete.deletarMunicipio(municipioId);
        log.info("DELETE /api/v1/municipio/delete/{} concluido com sucesso.", municipioId);
        return ResponseEntity.ok("Municipio removido com sucesso.");
    }
}
