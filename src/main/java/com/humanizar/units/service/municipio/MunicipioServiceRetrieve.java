package com.humanizar.units.service.municipio;

import java.util.List;
import java.util.UUID;

import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import com.humanizar.units.dto.MunicipioDTO;
import com.humanizar.units.mapper.MunicipioMapper;
import com.humanizar.units.model.Municipio;
import com.humanizar.units.repository.MunicipioRepository;

@Service
@Transactional(readOnly = true)
public class MunicipioServiceRetrieve {

    private final MunicipioRepository municipioRepository;
    private final MunicipioMapper municipioMapper;

    public MunicipioServiceRetrieve(MunicipioRepository municipioRepository, MunicipioMapper municipioMapper) {
        this.municipioRepository = municipioRepository;
        this.municipioMapper = municipioMapper;
    }

    public List<MunicipioDTO> listarMunicipios() {
        return municipioRepository.findAll()
                .stream()
                .map(municipioMapper::toDTO)
                .toList();
    }

    public MunicipioDTO buscarMunicipio(UUID municipioId) {
        Municipio municipio = municipioRepository.obterObrigatorio(municipioId);
        return municipioMapper.toDTO(municipio);
    }
}
