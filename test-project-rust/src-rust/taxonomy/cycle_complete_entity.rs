// AES020: part of circular import cycle
use crate::taxonomy::cycle_starter_entity::CycleStarterEntity;

pub struct CycleCompleteEntity;

impl CycleCompleteEntity {
    pub fn get_starter(&self) -> CycleStarterEntity {
        CycleStarterEntity
    }
}
