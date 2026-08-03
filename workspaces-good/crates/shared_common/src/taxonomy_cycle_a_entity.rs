use crate::shared_common::taxonomy_cycle_b_entity::CycleBEntity;

pub struct CycleAEntity {
    pub b: CycleBEntity,
}

impl CycleAEntity {
    pub fn new(b: CycleBEntity) -> Self {
        Self { b }
    }
}
