package com.humanizar.units.service;

import java.util.List;

import com.humanizar.units.repository.UnitsRepository;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import com.humanizar.units.dto.UnitDTO;
import com.humanizar.units.exception.UnitException;
import com.humanizar.units.mapper.UnitMapper;
import com.humanizar.units.model.enums.ReasonCode;

@Service
@Transactional
public class UnitsServiceRetrieve {

    private final UnitsRepository unitsRepository;
    private final UnitMapper unitMapper;

    public UnitsServiceRetrieve(UnitsRepository unitsRepository, UnitMapper unitMapper) {
        this.unitsRepository = unitsRepository;
        this.unitMapper = unitMapper;
    }

    public List<UnitDTO> listarUnits() {
        List<UnitDTO> units = unitsRepository.findAll()
                .stream()
                .map(unitMapper::toDTO)
                .toList();

        if (units.isEmpty()) {
            throw new UnitException(ReasonCode.UNIT_NOT_FOUND, "Nenhuma unidade cadastrada.");
        }

        return units;
    }
}
