use crate::taxonomy::{AdapterMetadata, AdapterName};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterMetadataList {
    #[serde(default)]
    pub values: Vec<AdapterMetadata>,
}

impl AdapterMetadataList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn push(&mut self, item: AdapterMetadata) {
        self.values.push(item);
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl std::ops::Deref for AdapterMetadataList {
    type Target = Vec<AdapterMetadata>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterNameList {
    #[serde(default)]
    pub values: Vec<AdapterName>,
}

impl AdapterNameList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn push(&mut self, item: AdapterName) {
        self.values.push(item);
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl std::ops::Deref for AdapterNameList {
    type Target = Vec<AdapterName>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterClassMap {
    #[serde(default)]
    pub values: std::collections::HashMap<String, String>,
}

impl AdapterClassMap {
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
        }
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}
