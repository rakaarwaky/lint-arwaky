// AES205: Part of circular dependency chain
use crate::taxonomy::taxonomy_aes205_cycle_start::CycleStartEntity;

pub struct CycleEndEntity {
    pub data: String,
}

impl CycleEndEntity {
    pub fn new() -> Self {
        Self { data: "end".to_string() }
    }

    pub fn process(&self) -> String {
        let _start = CycleStartEntity::new();
        self.data.clone()
    }
}
