package com.humanizar.units.service.units;

import java.util.UUID;

import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import com.humanizar.units.dto.UnitDTO;
import com.humanizar.units.mapper.UnitMapper;
import com.humanizar.units.model.Units;
import com.humanizar.units.repository.UnitsRepository;

@Service
@Transactional
public class UnitsServiceUpdate {

    private final UnitsRepository unitsRepository;
    private final UnitMapper unitMapper;

    public UnitsServiceUpdate(UnitsRepository unitsRepository, UnitMapper unitMapper) {
        this.unitsRepository = unitsRepository;
        this.unitMapper = unitMapper;
    }

    public UnitDTO atualizarUnit(UUID municipioId, UUID unitId, UnitDTO unitDTO) {
        Units unit = unitsRepository.obterObrigatorio(municipioId, unitId);

        unitMapper.applyUpdates(unit, unitDTO);

        Units unitAtualizado = unitsRepository.save(unit);
        return unitMapper.toDTO(unitAtualizado);
    }
}
