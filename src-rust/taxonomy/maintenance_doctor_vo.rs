use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DoctorResultVO {
    pub python_version: String,
    pub is_installed: bool,
    pub config_found: Vec<String>,
    pub adapter_statuses: HashMap<String, String>,
    pub issues: Vec<String>,
    pub healthy: bool,
}

impl DoctorResultVO {
    pub fn new(
        python_version: String,
        is_installed: bool,
        config_found: Vec<String>,
        adapter_statuses: HashMap<String, String>,
        issues: Vec<String>,
        healthy: bool,
    ) -> Self {
        Self { python_version, is_installed, config_found, adapter_statuses, issues, healthy }
    }
}

impl std::fmt::Display for DoctorResultVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DoctorResult(healthy={}, python={})", self.healthy, self.python_version)
    }
}
