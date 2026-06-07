package com.humanizar.units.service.units;

import java.util.UUID;

import org.springframework.stereotype.Service;

import com.humanizar.units.dto.UnitDTO;
import com.humanizar.units.mapper.UnitMapper;
import com.humanizar.units.model.Units;
import com.humanizar.units.repository.UnitsRepository;

@Service
public class UnitsServiceCreate {

    private final UnitsRepository unitsRepository;
    private final UnitMapper unitMapper;

    public UnitsServiceCreate(UnitsRepository unitsRepository, UnitMapper unitMapper) {
        this.unitsRepository = unitsRepository;
        this.unitMapper = unitMapper;
    }

    public UnitDTO criarUnit(UUID municipioId, UnitDTO unitDTO) {
        Units unit = unitMapper.toEntity(municipioId, unitDTO);
        Units unitSalvo = unitsRepository.save(unit);
        return unitMapper.toDTO(unitSalvo);
    }
}
