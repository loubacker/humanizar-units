package com.humanizar.units.service;

import java.util.UUID;

import com.humanizar.units.repository.UnitsRepository;
import org.springframework.stereotype.Service;

import com.humanizar.units.dto.UnitDTO;
import com.humanizar.units.exception.UnitException;
import com.humanizar.units.mapper.UnitMapper;
import com.humanizar.units.model.Units;
import com.humanizar.units.model.enums.ReasonCode;

@Service
public class UnitsServiceUpdate {

    private final UnitsRepository unitsRepository;
    private final UnitMapper unitMapper;

    public UnitsServiceUpdate(UnitsRepository unitsRepository, UnitMapper unitMapper) {
        this.unitsRepository = unitsRepository;
        this.unitMapper = unitMapper;
    }

    public UnitDTO atualizarUnit(UUID unitId, UnitDTO unitDTO) {
        Units unit = unitsRepository.findById(unitId)
                .orElseThrow(() -> new UnitException(
                        ReasonCode.UNIT_NOT_FOUND,
                        "Unidade nao encontrada para o unitId informado."));

        unitMapper.applyUpdates(unit, unitDTO);

        Units unitAtualizado = unitsRepository.save(unit);
        return unitMapper.toDTO(unitAtualizado);
    }
}
