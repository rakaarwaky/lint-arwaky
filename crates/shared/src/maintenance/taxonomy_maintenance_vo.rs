// PURPOSE: Maintenance value objects — doctor diagnostics, tool status, security and
//          dependency reports, and project statistics.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::Score;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;

// ─── Doctor diagnostics ───────────────────────────────────────────────

/// Output from executing an external tool (clippy, ruff, eslint, ...).
pub struct ToolOutput {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ToolchainDiagnostics {
    pub rust_tools: Vec<ToolStatus>,
    pub python_tools: Vec<ToolStatus>,
    pub js_tools: Vec<ToolStatus>,
    pub vcs_tools: Vec<ToolStatus>,
    pub binary_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HealthCheckAdapterVO {
    pub name: String,
    pub language: String,
    pub available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct HealthCheckResult {
    pub adapters: Vec<HealthCheckAdapterVO>,
}

// ─── Security and dependency reports ──────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityFinding {
    pub severity: String,
    pub test_id: String,
    pub file: String,
    pub line: u64,
    pub issue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DependencyReport {
    pub language: String,
    pub dependencies: Vec<DependencyInfo>,
}

// ─── Project statistics ───────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MaintenanceStatsVO {
    pub project_path: FilePath,
    pub total_files: Count,
    pub test_files: Count,
    pub test_ratio: Score,
    pub python_files: Count,
    pub rust_files: Count,
    pub js_files: Count,
}

impl MaintenanceStatsVO {
    pub fn new(
        project_path: FilePath,
        total_files: Count,
        test_files: Count,
        test_ratio: Score,
        python_files: Count,
        rust_files: Count,
        js_files: Count,
    ) -> Self {
        Self {
            project_path,
            total_files,
            test_files,
            test_ratio,
            python_files,
            rust_files,
            js_files,
        }
    }
}

impl std::fmt::Display for MaintenanceStatsVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MaintenanceStats({}: {} files, {} test, {:.1}%)",
            self.project_path,
            self.total_files.value,
            self.test_files.value,
            self.test_ratio.value * 100.0
        )
    }
}

// ─── Self-update ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SelfUpdateResultVO {
    /// The version currently running (from CARGO_PKG_VERSION).
    pub current_version: String,
    /// The latest release tag found on GitHub, or empty on error.
    pub latest_version: String,
    /// True when a newer release is available and was installed.
    pub already_up_to_date: bool,
    /// True when a newer version was successfully downloaded and installed.
    pub upgraded: bool,
    /// Human-readable status message (e.g. "Already up to date (v3.7.0)").
    pub status: String,
}

impl SelfUpdateResultVO {
    pub fn success(current: &str, latest: &str, already_up_to_date: bool) -> Self {
        let upgraded = !already_up_to_date;
        let status = if already_up_to_date {
            format!("Already up to date ({})", latest)
        } else {
            format!("Upgraded from {} to {}", current, latest)
        };
        Self {
            current_version: current.to_string(),
            latest_version: latest.to_string(),
            already_up_to_date,
            upgraded,
            status,
        }
    }

    pub fn error(current: &str, message: &str) -> Self {
        Self {
            current_version: current.to_string(),
            latest_version: String::new(),
            already_up_to_date: false,
            upgraded: false,
            status: format!("Error: {}", message),
        }
    }
}

impl std::fmt::Display for SelfUpdateResultVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.status)
    }
}
