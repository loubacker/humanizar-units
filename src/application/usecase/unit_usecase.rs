use uuid::Uuid;

use crate::domain::exception::UnitException;
use crate::domain::model::Unit;
use crate::domain::model::enums::ReasonCode;
use crate::domain::port::{MunicipioPort, UnitPort};

use super::lookup::{required_municipio, required_unit};

pub async fn create(
    unit_port: &dyn UnitPort,
    municipio_port: &dyn MunicipioPort,
    unit: Unit,
) -> Result<Unit, UnitException> {
    required_municipio(municipio_port, unit.municipio_id()).await?;
    ensure_cnpj_available(unit_port, &unit, None).await?;
    unit_port.save(unit).await
}

pub async fn find_by_municipio_id(
    unit_port: &dyn UnitPort,
    municipio_id: Uuid,
) -> Result<Vec<Unit>, UnitException> {
    unit_port.find_by_municipio_id(municipio_id).await
}

pub async fn find_by_id_and_municipio_id(
    unit_port: &dyn UnitPort,
    unit_id: Uuid,
    municipio_id: Uuid,
) -> Result<Unit, UnitException> {
    required_unit(unit_port, unit_id, municipio_id).await
}

pub async fn find_by_ids(
    unit_port: &dyn UnitPort,
    unit_ids: &[Uuid],
) -> Result<Vec<Unit>, UnitException> {
    if unit_ids.is_empty() {
        return Ok(Vec::new());
    }

    unit_port.find_by_ids(unit_ids).await
}

pub async fn update(unit_port: &dyn UnitPort, unit: Unit) -> Result<Unit, UnitException> {
    ensure_cnpj_available(unit_port, &unit, unit.id()).await?;
    unit_port.save(unit).await
}

pub async fn delete(
    unit_port: &dyn UnitPort,
    unit_id: Uuid,
    municipio_id: Uuid,
) -> Result<(), UnitException> {
    required_unit(unit_port, unit_id, municipio_id).await?;

    if !unit_port.delete_by_id(unit_id).await? {
        return Err(UnitException::new(ReasonCode::UnitNotFound));
    }

    Ok(())
}

async fn ensure_cnpj_available(
    unit_port: &dyn UnitPort,
    unit: &Unit,
    current_id: Option<Uuid>,
) -> Result<(), UnitException> {
    let existing = unit_port
        .find_by_municipio_id_and_cnpj(unit.municipio_id(), unit.cnpj())
        .await?;

    if existing.is_some_and(|existing| existing.id() != current_id) {
        return Err(UnitException::new(ReasonCode::UnitDuplicated));
    }

    Ok(())
}
