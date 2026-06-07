package com.humanizar.units.service.units;

import java.util.UUID;

import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import com.humanizar.units.model.Units;
import com.humanizar.units.repository.UnitsRepository;

@Service
@Transactional
public class UnitsServiceDelete {

    private final UnitsRepository unitsRepository;

    public UnitsServiceDelete(UnitsRepository unitsRepository) {
        this.unitsRepository = unitsRepository;
    }

    public void deletarUnit(UUID municipioId, UUID unitId) {
        Units unit = unitsRepository.obterObrigatorio(municipioId, unitId);
        unitsRepository.delete(unit);
    }
}
