// PURPOSE: DoctorResultVO, DoctorCheck — VOs for project health diagnostics results
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DoctorResultVO {
    pub python_version: DescriptionVO,
    pub rust_version: DescriptionVO,
    pub node_version: DescriptionVO,
    pub is_installed: ComplianceStatus,
    pub config_found: FilePathList,
    pub adapter_statuses: HashMap<AdapterName, String>,
    pub issues: Vec<ErrorMessage>,
    pub healthy: ComplianceStatus,
}

// Constructor removed — struct has pub fields, direct initialization preferred.

impl std::fmt::Display for DoctorResultVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DoctorResult(healthy={}, python={}, rust={}, node={})",
            self.healthy.value,
            self.python_version.value,
            self.rust_version.value,
            self.node_version.value
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolStatus {
    pub name: String,
    pub status: String, // "OK", "WARN", "FAIL"
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolchainDiagnostics {
    pub rust_tools: Vec<ToolStatus>,
    pub python_tools: Vec<ToolStatus>,
    pub js_tools: Vec<ToolStatus>,
    pub vcs_tools: Vec<ToolStatus>,
    pub binary_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityFinding {
    pub severity: String,
    pub test_id: String,
    pub file: String,
    pub line: u64,
    pub issue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityScanReport {
    pub language: String,
    pub tool_name: String,
    pub findings: Vec<SecurityFinding>,
    pub tool_installed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DependencyInfo {
    pub name: String,
    pub version: String,
    pub dep_type: String, // "direct" or "transitive"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DependencyReport {
    pub language: String,
    pub dependencies: Vec<DependencyInfo>,
}
