use std::sync::Arc;

use uuid::Uuid;

use crate::application::dto::{UnitDto, UnitIdsQueryDto};
use crate::application::mapper::UnitMapper;
use crate::application::usecase::unit_usecase;
use crate::domain::exception::UnitException;
use crate::domain::port::{MunicipioPort, UnitPort};

pub struct UnitService {
    unit_port: Arc<dyn UnitPort>,
    municipio_port: Arc<dyn MunicipioPort>,
}

impl UnitService {
    pub fn new(unit_port: Arc<dyn UnitPort>, municipio_port: Arc<dyn MunicipioPort>) -> Self {
        Self {
            unit_port,
            municipio_port,
        }
    }

    pub async fn create(&self, municipio_id: Uuid, dto: UnitDto) -> Result<UnitDto, UnitException> {
        let unit = UnitMapper::to_new_domain(municipio_id, &dto)?;
        let saved =
            unit_usecase::create(self.unit_port.as_ref(), self.municipio_port.as_ref(), unit)
                .await?;

        Ok(UnitMapper::to_dto(saved))
    }

    pub async fn find_by_municipio_id(
        &self,
        municipio_id: Uuid,
    ) -> Result<Vec<UnitDto>, UnitException> {
        let units =
            unit_usecase::find_by_municipio_id(self.unit_port.as_ref(), municipio_id).await?;

        Ok(UnitMapper::to_dtos(units))
    }

    pub async fn find_by_ids(&self, query: UnitIdsQueryDto) -> Result<Vec<UnitDto>, UnitException> {
        let units = unit_usecase::find_by_ids(self.unit_port.as_ref(), query.ids()).await?;

        Ok(UnitMapper::to_dtos(units))
    }

    pub async fn update(
        &self,
        municipio_id: Uuid,
        unit_id: Uuid,
        dto: UnitDto,
    ) -> Result<UnitDto, UnitException> {
        let current = unit_usecase::find_by_id_and_municipio_id(
            self.unit_port.as_ref(),
            unit_id,
            municipio_id,
        )
        .await?;
        let unit = UnitMapper::to_updated_domain(&current, &dto)?;
        let saved = unit_usecase::update(self.unit_port.as_ref(), unit).await?;

        Ok(UnitMapper::to_dto(saved))
    }

    pub async fn delete(&self, municipio_id: Uuid, unit_id: Uuid) -> Result<(), UnitException> {
        unit_usecase::delete(self.unit_port.as_ref(), unit_id, municipio_id).await
    }
}
