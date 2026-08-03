// AES205: Part of circular dependency chain
use crate::taxonomy::taxonomy_aes205_cycle_end::CycleEndEntity;

pub struct CycleStartEntity {
    pub data: String,
}

impl CycleStartEntity {
    pub fn new() -> Self {
        Self { data: "start".to_string() }
    }

    pub fn process(&self) -> String {
        let _end = CycleEndEntity::new();
        self.data.clone()
    }
}
