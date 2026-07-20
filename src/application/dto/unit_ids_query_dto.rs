use uuid::Uuid;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UnitIdsQueryDto {
    ids: Vec<Uuid>,
}

impl UnitIdsQueryDto {
    pub const fn new(ids: Vec<Uuid>) -> Self {
        Self { ids }
    }

    pub fn ids(&self) -> &[Uuid] {
        &self.ids
    }
}
