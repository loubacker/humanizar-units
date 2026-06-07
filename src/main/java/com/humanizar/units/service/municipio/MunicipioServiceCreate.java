package com.humanizar.units.service.municipio;

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
public class MunicipioServiceCreate {

    private final MunicipioRepository municipioRepository;
    private final MunicipioMapper municipioMapper;

    public MunicipioServiceCreate(MunicipioRepository municipioRepository, MunicipioMapper municipioMapper) {
        this.municipioRepository = municipioRepository;
        this.municipioMapper = municipioMapper;
    }

    public MunicipioDTO criarMunicipio(MunicipioDTO municipioDTO) {
        municipioMapper.validate(municipioDTO);
        garantirCodigoIbgeDisponivel(municipioDTO.codigoIbge());

        Municipio municipio = municipioMapper.toEntity(municipioDTO);
        Municipio municipioSalvo = municipioRepository.save(municipio);
        return municipioMapper.toDTO(municipioSalvo);
    }

    private void garantirCodigoIbgeDisponivel(String codigoIbge) {
        municipioRepository.findByCodigoIbge(codigoIbge)
                .ifPresent(existente -> {
                    throw new UnitException(
                            ReasonCode.MUNICIPIO_DUPLICATED,
                            "Ja existe municipio com o codigoIbge informado.");
                });
    }
}
