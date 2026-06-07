package com.humanizar.units.service.units;

import java.util.List;
import java.util.UUID;

import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import com.humanizar.units.dto.UnitDTO;
import com.humanizar.units.mapper.UnitMapper;
import com.humanizar.units.repository.UnitsRepository;

@Service
@Transactional(readOnly = true)
public class UnitsServiceRetrieve {

    private final UnitsRepository unitsRepository;
    private final UnitMapper unitMapper;

    public UnitsServiceRetrieve(UnitsRepository unitsRepository, UnitMapper unitMapper) {
        this.unitsRepository = unitsRepository;
        this.unitMapper = unitMapper;
    }

    public List<UnitDTO> listarUnits(UUID municipioId) {
        return unitsRepository.findByMunicipioId(municipioId)
                .stream()
                .map(unitMapper::toDTO)
                .toList();
    }

    public List<UnitDTO> obterPorIds(List<UUID> ids) {
        if (ids == null || ids.isEmpty()) {
            return List.of();
        }

        return unitsRepository.findByIdIn(ids)
                .stream()
                .map(unitMapper::toDTO)
                .toList();
    }
}
