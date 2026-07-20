use uuid::Uuid;

use crate::domain::exception::UnitException;
use crate::domain::model::enums::ReasonCode;
use crate::domain::model::{Municipio, Unit};
use crate::domain::port::{MunicipioPort, UnitPort};

pub async fn required_municipio(
    municipio_port: &dyn MunicipioPort,
    municipio_id: Uuid,
) -> Result<Municipio, UnitException> {
    municipio_port
        .find_by_id(municipio_id)
        .await?
        .ok_or_else(|| UnitException::new(ReasonCode::MunicipioNotFound))
}

pub async fn required_unit(
    unit_port: &dyn UnitPort,
    unit_id: Uuid,
    municipio_id: Uuid,
) -> Result<Unit, UnitException> {
    unit_port
        .find_by_id_and_municipio_id(unit_id, municipio_id)
        .await?
        .ok_or_else(|| UnitException::new(ReasonCode::UnitNotFound))
}
