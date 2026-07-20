#![allow(dead_code)]

use std::sync::Mutex;

use async_trait::async_trait;
use chrono::Utc;
use humanizar_units::domain::exception::UnitException;
use humanizar_units::domain::model::{Municipio, Unit};
use humanizar_units::domain::port::{MunicipioPort, UnitPort};
use uuid::Uuid;

#[derive(Default)]
pub struct InMemoryPorts {
    state: Mutex<PortState>,
}

#[derive(Default)]
struct PortState {
    municipios: Vec<Municipio>,
    units: Vec<Unit>,
    find_by_ids_calls: usize,
}

impl InMemoryPorts {
    pub fn find_by_ids_calls(&self) -> usize {
        self.state
            .lock()
            .expect("o estado de teste deve estar disponivel")
            .find_by_ids_calls
    }
}

#[async_trait]
impl MunicipioPort for InMemoryPorts {
    async fn save(&self, municipio: Municipio) -> Result<Municipio, UnitException> {
        let mut state = self.state.lock().expect("o estado deve estar disponivel");
        let saved = persisted_municipio(municipio);

        replace_or_insert(&mut state.municipios, saved.clone(), Municipio::id);
        Ok(saved)
    }

    async fn find_all(&self) -> Result<Vec<Municipio>, UnitException> {
        Ok(self
            .state
            .lock()
            .expect("o estado deve estar disponivel")
            .municipios
            .clone())
    }

    async fn find_by_id(&self, municipio_id: Uuid) -> Result<Option<Municipio>, UnitException> {
        Ok(self
            .state
            .lock()
            .expect("o estado deve estar disponivel")
            .municipios
            .iter()
            .find(|municipio| municipio.id() == Some(municipio_id))
            .cloned())
    }

    async fn find_by_codigo_ibge(
        &self,
        codigo_ibge: &str,
    ) -> Result<Option<Municipio>, UnitException> {
        Ok(self
            .state
            .lock()
            .expect("o estado deve estar disponivel")
            .municipios
            .iter()
            .find(|municipio| municipio.codigo_ibge() == codigo_ibge)
            .cloned())
    }

    async fn delete_by_id(&self, municipio_id: Uuid) -> Result<bool, UnitException> {
        let mut state = self.state.lock().expect("o estado deve estar disponivel");
        Ok(remove_by_id(
            &mut state.municipios,
            municipio_id,
            Municipio::id,
        ))
    }
}

#[async_trait]
impl UnitPort for InMemoryPorts {
    async fn save(&self, unit: Unit) -> Result<Unit, UnitException> {
        let mut state = self.state.lock().expect("o estado deve estar disponivel");
        let saved = persisted_unit(unit);

        replace_or_insert(&mut state.units, saved.clone(), Unit::id);
        Ok(saved)
    }

    async fn find_by_municipio_id(&self, municipio_id: Uuid) -> Result<Vec<Unit>, UnitException> {
        Ok(self
            .state
            .lock()
            .expect("o estado deve estar disponivel")
            .units
            .iter()
            .filter(|unit| unit.municipio_id() == municipio_id)
            .cloned()
            .collect())
    }

    async fn find_by_id_and_municipio_id(
        &self,
        unit_id: Uuid,
        municipio_id: Uuid,
    ) -> Result<Option<Unit>, UnitException> {
        Ok(self
            .state
            .lock()
            .expect("o estado deve estar disponivel")
            .units
            .iter()
            .find(|unit| unit.id() == Some(unit_id) && unit.municipio_id() == municipio_id)
            .cloned())
    }

    async fn find_by_municipio_id_and_cnpj(
        &self,
        municipio_id: Uuid,
        cnpj: &str,
    ) -> Result<Option<Unit>, UnitException> {
        Ok(self
            .state
            .lock()
            .expect("o estado deve estar disponivel")
            .units
            .iter()
            .find(|unit| unit.municipio_id() == municipio_id && unit.cnpj() == cnpj)
            .cloned())
    }

    async fn find_by_ids(&self, unit_ids: &[Uuid]) -> Result<Vec<Unit>, UnitException> {
        let mut state = self.state.lock().expect("o estado deve estar disponivel");
        state.find_by_ids_calls += 1;

        Ok(state
            .units
            .iter()
            .filter(|unit| unit.id().is_some_and(|unit_id| unit_ids.contains(&unit_id)))
            .cloned()
            .collect())
    }

    async fn count_by_municipio_id(&self, municipio_id: Uuid) -> Result<u64, UnitException> {
        let total = self
            .state
            .lock()
            .expect("o estado deve estar disponivel")
            .units
            .iter()
            .filter(|unit| unit.municipio_id() == municipio_id)
            .count();

        Ok(u64::try_from(total).expect("a contagem de teste deve caber em u64"))
    }

    async fn delete_by_id(&self, unit_id: Uuid) -> Result<bool, UnitException> {
        let mut state = self.state.lock().expect("o estado deve estar disponivel");
        Ok(remove_by_id(&mut state.units, unit_id, Unit::id))
    }
}

fn persisted_municipio(municipio: Municipio) -> Municipio {
    let now = Utc::now().naive_utc();

    Municipio::restore(
        municipio.id().or_else(|| Some(Uuid::new_v4())),
        municipio.codigo_ibge(),
        municipio.nome(),
        municipio.uf(),
        municipio.created_at().or(Some(now)),
        Some(now),
    )
}

fn persisted_unit(unit: Unit) -> Unit {
    let now = Utc::now().naive_utc();

    Unit::restore(
        unit.id().or_else(|| Some(Uuid::new_v4())),
        unit.municipio_id(),
        unit.unit_name(),
        unit.razao_social(),
        unit.endereco(),
        unit.numero(),
        unit.complemento().map(str::to_owned),
        unit.bairro(),
        unit.cep(),
        unit.cnpj(),
        unit.created_at().or(Some(now)),
        Some(now),
    )
}

fn replace_or_insert<T: Clone>(items: &mut Vec<T>, item: T, id: impl Fn(&T) -> Option<Uuid>) {
    if let Some(index) = items.iter().position(|current| id(current) == id(&item)) {
        items[index] = item;
        return;
    }

    items.push(item);
}

fn remove_by_id<T>(items: &mut Vec<T>, expected_id: Uuid, id: impl Fn(&T) -> Option<Uuid>) -> bool {
    let initial_length = items.len();
    items.retain(|item| id(item) != Some(expected_id));
    items.len() != initial_length
}
