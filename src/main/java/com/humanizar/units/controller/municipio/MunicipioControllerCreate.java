package com.humanizar.units.controller.municipio;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.humanizar.units.dto.MunicipioDTO;
import com.humanizar.units.service.municipio.MunicipioServiceCreate;

@RestController
@RequestMapping("/api/v1/municipio")
public class MunicipioControllerCreate {

    private static final Logger log = LoggerFactory.getLogger(MunicipioControllerCreate.class);

    private final MunicipioServiceCreate municipioServiceCreate;

    public MunicipioControllerCreate(MunicipioServiceCreate municipioServiceCreate) {
        this.municipioServiceCreate = municipioServiceCreate;
    }

    @PostMapping("/register")
    @PreAuthorize("hasRole('ADMINISTRADOR')")
    public ResponseEntity<MunicipioDTO> criarMunicipio(@RequestBody MunicipioDTO municipioDTO) {
        log.info("Recebido POST /api/v1/municipio/register. operacao=CREATE, codigoIbge={}",
                municipioDTO != null ? municipioDTO.codigoIbge() : null);
        MunicipioDTO municipioCriado = municipioServiceCreate.criarMunicipio(municipioDTO);
        log.info("POST /api/v1/municipio/register concluido com sucesso. municipioId={}",
                municipioCriado != null ? municipioCriado.municipioId() : null);
        return ResponseEntity.status(HttpStatus.CREATED).body(municipioCriado);
    }
}
