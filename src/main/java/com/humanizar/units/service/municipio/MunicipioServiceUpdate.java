package com.humanizar.units.service.municipio;

import java.util.UUID;

import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import com.humanizar.units.dto.MunicipioDTO;
import com.humanizar.units.exception.UnitException;
import com.humanizar.units.mapper.MunicipioMapper;
import com.humanizar.units.model.Municipio;
import com.humanizar.units.model.enums.ReasonCode;
import com.humanizar.units.repository.MunicipioRepository;

@Service
@Transactional
public class MunicipioServiceUpdate {

    private final MunicipioRepository municipioRepository;
    private final MunicipioMapper municipioMapper;

    public MunicipioServiceUpdate(MunicipioRepository municipioRepository, MunicipioMapper municipioMapper) {
        this.municipioRepository = municipioRepository;
        this.municipioMapper = municipioMapper;
    }

    public MunicipioDTO atualizarMunicipio(UUID municipioId, MunicipioDTO municipioDTO) {
        Municipio municipio = municipioRepository.obterObrigatorio(municipioId);
        municipioMapper.validate(municipioDTO);
        garantirCodigoIbgeDisponivel(municipioId, municipioDTO.codigoIbge());

        municipioMapper.applyUpdates(municipio, municipioDTO);
        Municipio municipioAtualizado = municipioRepository.save(municipio);
        return municipioMapper.toDTO(municipioAtualizado);
    }

    private void garantirCodigoIbgeDisponivel(UUID municipioId, String codigoIbge) {
        municipioRepository.findByCodigoIbge(codigoIbge)
                .filter(existente -> !existente.getId().equals(municipioId))
                .ifPresent(existente -> {
                    throw new UnitException(
                            ReasonCode.MUNICIPIO_DUPLICATED,
                            "Ja existe municipio com o codigoIbge informado.");
                });
    }
}
