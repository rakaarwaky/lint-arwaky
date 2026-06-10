// PURPOSE: VO: Adapter value object
use crate::pipeline_jobs::taxonomy_job_vo::AdapterMetadata;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterMetadataList {
    #[serde(default)]
    pub values: Vec<AdapterMetadata>,
}

impl Default for AdapterMetadataList {
    fn default() -> Self {
        Self::new()
    }
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

impl Default for AdapterNameList {
    fn default() -> Self {
        Self::new()
    }
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

impl Default for AdapterClassMap {
    fn default() -> Self {
        Self::new()
    }
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
