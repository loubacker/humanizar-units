use uuid::Uuid;

use crate::domain::exception::UnitException;
use crate::domain::model::Municipio;
use crate::domain::model::enums::ReasonCode;
use crate::domain::port::{MunicipioPort, UnitPort};

use super::lookup::required_municipio;

pub async fn create(
    municipio_port: &dyn MunicipioPort,
    municipio: Municipio,
) -> Result<Municipio, UnitException> {
    ensure_codigo_ibge_available(municipio_port, &municipio, None).await?;
    municipio_port.save(municipio).await
}

pub async fn find_all(municipio_port: &dyn MunicipioPort) -> Result<Vec<Municipio>, UnitException> {
    municipio_port.find_all().await
}

pub async fn find_by_id(
    municipio_port: &dyn MunicipioPort,
    municipio_id: Uuid,
) -> Result<Municipio, UnitException> {
    required_municipio(municipio_port, municipio_id).await
}

pub async fn update(
    municipio_port: &dyn MunicipioPort,
    municipio: Municipio,
) -> Result<Municipio, UnitException> {
    ensure_codigo_ibge_available(municipio_port, &municipio, municipio.id()).await?;
    municipio_port.save(municipio).await
}

pub async fn delete(
    municipio_port: &dyn MunicipioPort,
    unit_port: &dyn UnitPort,
    municipio_id: Uuid,
) -> Result<(), UnitException> {
    required_municipio(municipio_port, municipio_id).await?;

    if unit_port.count_by_municipio_id(municipio_id).await? > 0 {
        return Err(UnitException::new(ReasonCode::MunicipioHasUnits));
    }

    if !municipio_port.delete_by_id(municipio_id).await? {
        return Err(UnitException::new(ReasonCode::MunicipioNotFound));
    }

    Ok(())
}

async fn ensure_codigo_ibge_available(
    municipio_port: &dyn MunicipioPort,
    municipio: &Municipio,
    current_id: Option<Uuid>,
) -> Result<(), UnitException> {
    let existing = municipio_port
        .find_by_codigo_ibge(municipio.codigo_ibge())
        .await?;

    if existing.is_some_and(|existing| existing.id() != current_id) {
        return Err(UnitException::new(ReasonCode::MunicipioDuplicated));
    }

    Ok(())
}
