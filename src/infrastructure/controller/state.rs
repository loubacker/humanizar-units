use std::sync::Arc;

use crate::application::service::{MunicipioService, UnitService};

#[derive(Clone)]
pub struct ApplicationState {
    municipio_service: Arc<MunicipioService>,
    unit_service: Arc<UnitService>,
}

impl ApplicationState {
    pub const fn new(
        municipio_service: Arc<MunicipioService>,
        unit_service: Arc<UnitService>,
    ) -> Self {
        Self {
            municipio_service,
            unit_service,
        }
    }

    pub fn municipio_service(&self) -> &MunicipioService {
        &self.municipio_service
    }

    pub fn unit_service(&self) -> &UnitService {
        &self.unit_service
    }
}
