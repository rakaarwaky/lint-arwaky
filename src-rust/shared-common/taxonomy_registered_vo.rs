use crate::shared_common::taxonomy_common_vo::Timestamp;
use crate::shared_common::taxonomy_name_vo::AdapterName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterRegistered {
    pub adapter_name: AdapterName,
    #[serde(default)]
    pub timestamp: Timestamp,
}

impl AdapterRegistered {
    pub fn new(adapter_name: AdapterName) -> Self {
        Self {
            adapter_name,
            timestamp: Timestamp::default(),
        }
    }
}
