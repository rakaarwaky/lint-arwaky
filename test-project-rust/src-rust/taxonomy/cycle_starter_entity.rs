// AES020: part of circular import cycle
use crate::taxonomy::cycle_complete_entity::CycleCompleteEntity;

pub struct CycleStarterEntity;

impl CycleStarterEntity {
    pub fn get_complete(&self) -> CycleCompleteEntity {
        CycleCompleteEntity
    }
}
