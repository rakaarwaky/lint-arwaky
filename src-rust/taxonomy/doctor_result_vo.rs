use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::taxonomy::{AdapterName, ComplianceStatus, DescriptionVO, ErrorMessage, FilePathList};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DoctorResultVO {
    pub python_version: DescriptionVO,
    pub is_installed: ComplianceStatus,
    pub config_found: FilePathList,
    pub adapter_statuses: HashMap<AdapterName, String>,
    pub issues: Vec<ErrorMessage>,
    pub healthy: ComplianceStatus,
}

impl DoctorResultVO {
    pub fn new(
        python_version: DescriptionVO,
        is_installed: ComplianceStatus,
        config_found: FilePathList,
        adapter_statuses: HashMap<AdapterName, String>,
        issues: Vec<ErrorMessage>,
        healthy: ComplianceStatus,
    ) -> Self {
        Self {
            python_version,
            is_installed,
            config_found,
            adapter_statuses,
            issues,
            healthy,
        }
    }
}

impl std::fmt::Display for DoctorResultVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DoctorResult(healthy={}, python={})",
            self.healthy.value, self.python_version.value
        )
    }
}
