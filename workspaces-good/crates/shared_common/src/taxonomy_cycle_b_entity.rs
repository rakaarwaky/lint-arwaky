use crate::shared_common::taxonomy_cycle_a_entity::CycleAEntity;

pub struct CycleBEntity {
    pub a: CycleAEntity,
}

impl CycleBEntity {
    pub fn new(a: CycleAEntity) -> Self {
        Self { a }
    }
}
