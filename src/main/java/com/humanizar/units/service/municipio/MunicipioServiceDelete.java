package com.humanizar.units.service.municipio;

import java.util.UUID;

import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import com.humanizar.units.exception.UnitException;
import com.humanizar.units.model.Municipio;
import com.humanizar.units.model.enums.ReasonCode;
import com.humanizar.units.repository.MunicipioRepository;
import com.humanizar.units.repository.UnitsRepository;

@Service
@Transactional
public class MunicipioServiceDelete {

    private final MunicipioRepository municipioRepository;
    private final UnitsRepository unitsRepository;

    public MunicipioServiceDelete(MunicipioRepository municipioRepository, UnitsRepository unitsRepository) {
        this.municipioRepository = municipioRepository;
        this.unitsRepository = unitsRepository;
    }

    public void deletarMunicipio(UUID municipioId) {
        Municipio municipio = municipioRepository.obterObrigatorio(municipioId);

        long unidadesVinculadas = unitsRepository.countByMunicipioId(municipioId);
        if (unidadesVinculadas > 0) {
            throw new UnitException(
                    ReasonCode.MUNICIPIO_HAS_UNITS,
                    "Municipio possui unidades vinculadas e nao pode ser removido.");
        }

        municipioRepository.delete(municipio);
    }
}
