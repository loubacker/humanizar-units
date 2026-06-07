package com.humanizar.units.controller.municipio;

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

import com.humanizar.units.dto.MunicipioDTO;
import com.humanizar.units.service.municipio.MunicipioServiceUpdate;

@RestController
@RequestMapping("/api/v1/municipio")
public class MunicipioControllerUpdate {

    private static final Logger log = LoggerFactory.getLogger(MunicipioControllerUpdate.class);

    private final MunicipioServiceUpdate municipioServiceUpdate;

    public MunicipioControllerUpdate(MunicipioServiceUpdate municipioServiceUpdate) {
        this.municipioServiceUpdate = municipioServiceUpdate;
    }

    @PutMapping("/update/{municipioId}")
    @PreAuthorize("hasRole('ADMINISTRADOR')")
    public ResponseEntity<MunicipioDTO> atualizarMunicipio(@PathVariable UUID municipioId,
            @RequestBody MunicipioDTO municipioDTO) {
        log.info("Recebido PUT /api/v1/municipio/update/{}. operacao=UPDATE", municipioId);
        MunicipioDTO municipioAtualizado = municipioServiceUpdate.atualizarMunicipio(municipioId, municipioDTO);
        log.info("PUT /api/v1/municipio/update/{} concluido com sucesso.", municipioId);
        return ResponseEntity.status(HttpStatus.OK).body(municipioAtualizado);
    }
}
