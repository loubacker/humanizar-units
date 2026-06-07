package com.humanizar.units.controller.municipio;

import java.util.List;
import java.util.UUID;

import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.humanizar.units.config.ResilientMethodsConfig.Retry;
import com.humanizar.units.dto.MunicipioDTO;
import com.humanizar.units.service.municipio.MunicipioServiceRetrieve;

@RestController
@RequestMapping("/api/v1/municipio")
public class MunicipioControllerRetrieve {

    private final MunicipioServiceRetrieve municipioServiceRetrieve;

    public MunicipioControllerRetrieve(MunicipioServiceRetrieve municipioServiceRetrieve) {
        this.municipioServiceRetrieve = municipioServiceRetrieve;
    }

    @Retry
    @GetMapping
    @PreAuthorize("isAuthenticated()")
    public ResponseEntity<List<MunicipioDTO>> listarMunicipios() {
        return ResponseEntity.ok(municipioServiceRetrieve.listarMunicipios());
    }

    @Retry
    @GetMapping("/{municipioId}")
    @PreAuthorize("isAuthenticated()")
    public ResponseEntity<MunicipioDTO> buscarMunicipio(@PathVariable UUID municipioId) {
        return ResponseEntity.ok(municipioServiceRetrieve.buscarMunicipio(municipioId));
    }
}
