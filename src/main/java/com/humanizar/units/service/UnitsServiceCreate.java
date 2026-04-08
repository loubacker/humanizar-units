package com.humanizar.units.service;

import com.humanizar.units.repository.UnitsRepository;
import org.springframework.stereotype.Service;

import com.humanizar.units.dto.UnitDTO;
import com.humanizar.units.mapper.UnitMapper;
import com.humanizar.units.model.Units;

@Service
public class UnitsServiceCreate {

    private final UnitsRepository unitsRepository;
    private final UnitMapper unitMapper;

    public UnitsServiceCreate(UnitsRepository unitsRepository, UnitMapper unitMapper) {
        this.unitsRepository = unitsRepository;
        this.unitMapper = unitMapper;
    }

    public UnitDTO criarUnit(UnitDTO unitDTO) {
        Units unit = unitMapper.toEntity(unitDTO);
        Units unitSalvo = unitsRepository.save(unit);
        return unitMapper.toDTO(unitSalvo);
    }
}
