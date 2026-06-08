use crate::taxonomy::{AdapterName, BooleanVO, Timestamp};
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
