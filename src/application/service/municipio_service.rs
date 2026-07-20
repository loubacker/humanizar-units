use std::sync::Arc;

use uuid::Uuid;

use crate::application::dto::MunicipioDto;
use crate::application::mapper::MunicipioMapper;
use crate::application::usecase::municipio_usecase;
use crate::domain::exception::UnitException;
use crate::domain::port::{MunicipioPort, UnitPort};

pub struct MunicipioService {
    municipio_port: Arc<dyn MunicipioPort>,
    unit_port: Arc<dyn UnitPort>,
}

impl MunicipioService {
    pub fn new(municipio_port: Arc<dyn MunicipioPort>, unit_port: Arc<dyn UnitPort>) -> Self {
        Self {
            municipio_port,
            unit_port,
        }
    }

    pub async fn create(&self, dto: MunicipioDto) -> Result<MunicipioDto, UnitException> {
        let municipio = MunicipioMapper::to_new_domain(&dto)?;
        let saved = municipio_usecase::create(self.municipio_port.as_ref(), municipio).await?;

        Ok(MunicipioMapper::to_dto(saved))
    }

    pub async fn find_all(&self) -> Result<Vec<MunicipioDto>, UnitException> {
        let municipios = municipio_usecase::find_all(self.municipio_port.as_ref()).await?;

        Ok(MunicipioMapper::to_dtos(municipios))
    }

    pub async fn find_by_id(&self, municipio_id: Uuid) -> Result<MunicipioDto, UnitException> {
        let municipio =
            municipio_usecase::find_by_id(self.municipio_port.as_ref(), municipio_id).await?;

        Ok(MunicipioMapper::to_dto(municipio))
    }

    pub async fn update(
        &self,
        municipio_id: Uuid,
        dto: MunicipioDto,
    ) -> Result<MunicipioDto, UnitException> {
        let current =
            municipio_usecase::find_by_id(self.municipio_port.as_ref(), municipio_id).await?;
        let municipio = MunicipioMapper::to_updated_domain(&current, &dto)?;
        let saved = municipio_usecase::update(self.municipio_port.as_ref(), municipio).await?;

        Ok(MunicipioMapper::to_dto(saved))
    }

    pub async fn delete(&self, municipio_id: Uuid) -> Result<(), UnitException> {
        municipio_usecase::delete(
            self.municipio_port.as_ref(),
            self.unit_port.as_ref(),
            municipio_id,
        )
        .await
    }
}
