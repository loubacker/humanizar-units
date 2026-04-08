package com.humanizar.units.repository;

import java.util.UUID;

import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import com.humanizar.units.model.Units;

@Repository
public interface UnitsRepository extends JpaRepository<Units, UUID> {

}