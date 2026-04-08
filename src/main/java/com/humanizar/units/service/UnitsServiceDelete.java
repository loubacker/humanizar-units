package com.humanizar.units.service;

import java.util.UUID;

import com.humanizar.units.repository.UnitsRepository;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import com.humanizar.units.exception.UnitException;
import com.humanizar.units.model.Units;
import com.humanizar.units.model.enums.ReasonCode;

@Service
@Transactional
public class UnitsServiceDelete {

    private final UnitsRepository unitsRepository;

    public UnitsServiceDelete(UnitsRepository unitsRepository) {
        this.unitsRepository = unitsRepository;
    }

    public void deletarUnit(UUID unitId) {
        Units unit = unitsRepository.findById(unitId)
                .orElseThrow(() -> new UnitException(
                        ReasonCode.UNIT_NOT_FOUND,
                        "Unidade nao encontrada para o unitId informado."));
        unitsRepository.delete(unit);
    }
}
