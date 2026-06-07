package com.humanizar.units.repository;

import java.util.Optional;
import java.util.UUID;

import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import com.humanizar.units.exception.UnitException;
import com.humanizar.units.model.Municipio;
import com.humanizar.units.model.enums.ReasonCode;

@Repository
public interface MunicipioRepository extends JpaRepository<Municipio, UUID> {

    Optional<Municipio> findByCodigoIbge(String codigoIbge);

    default Municipio obterObrigatorio(UUID municipioId) {
        return findById(municipioId)
                .orElseThrow(() -> new UnitException(
                        ReasonCode.MUNICIPIO_NOT_FOUND,
                        "Municipio nao encontrado para o municipioId informado."));
    }
}
