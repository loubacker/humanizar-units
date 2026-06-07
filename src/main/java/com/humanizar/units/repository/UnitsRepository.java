package com.humanizar.units.repository;

import java.util.Collection;
import java.util.List;
import java.util.Optional;
import java.util.UUID;

import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import com.humanizar.units.exception.UnitException;
import com.humanizar.units.model.Units;
import com.humanizar.units.model.enums.ReasonCode;

@Repository
public interface UnitsRepository extends JpaRepository<Units, UUID> {

    List<Units> findByMunicipioId(UUID municipioId);

    Optional<Units> findByIdAndMunicipioId(UUID id, UUID municipioId);

    List<Units> findByIdIn(Collection<UUID> ids);

    long countByMunicipioId(UUID municipioId);

    default Units obterObrigatorio(UUID municipioId, UUID unitId) {
        return findByIdAndMunicipioId(unitId, municipioId)
                .orElseThrow(() -> new UnitException(
                        ReasonCode.UNIT_NOT_FOUND,
                        "Unidade nao encontrada para o municipio e unitId informados."));
    }
}
