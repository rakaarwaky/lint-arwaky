use serde::{Serialize, Deserialize};
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScanStarted {
    pub path: FilePath,
    pub adapters: Vec<AdapterName>,
    #[serde(default)]
    pub timestamp: String,
}

impl ScanStarted {
    pub fn new(path: FilePath, adapters: Vec<AdapterName>) -> Self {
        Self { path, adapters, timestamp: String::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScanCompleted {
    pub path: FilePath,
    pub score: Score,
    pub worst_severity: Severity,
    pub violation_count: Count,
    pub duration_ms: Duration,
    #[serde(default)]
    pub is_passing: ComplianceStatus,
    #[serde(default)]
    pub timestamp: String,
}

impl ScanCompleted {
    pub fn new(path: FilePath, score: Score, worst_severity: Severity, violation_count: Count, duration_ms: Duration) -> Self {
        Self { path, score, worst_severity, violation_count, duration_ms, is_passing: ComplianceStatus::new(true), timestamp: String::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScanFailed {
    pub path: FilePath,
    pub adapter: AdapterName,
    pub error_message: ErrorMessage,
    #[serde(default)]
    pub error_code: Option<ErrorCode>,
    #[serde(default)]
    pub timestamp: String,
}

impl ScanFailed {
    pub fn new(path: FilePath, adapter: AdapterName, error_message: ErrorMessage) -> Self {
        Self { path, adapter, error_message, error_code: None, timestamp: String::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FixApplied {
    pub path: FilePath,
    pub adapter: AdapterName,
    pub error_code: ErrorCode,
    pub changes_count: Count,
    #[serde(default)]
    pub timestamp: String,
}

impl FixApplied {
    pub fn new(path: FilePath, adapter: AdapterName, error_code: ErrorCode, changes_count: Count) -> Self {
        Self { path, adapter, error_code, changes_count, timestamp: String::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterRegistered {
    pub adapter_name: AdapterName,
    #[serde(default)]
    pub timestamp: String,
}

impl AdapterRegistered {
    pub fn new(adapter_name: AdapterName) -> Self {
        Self { adapter_name, timestamp: String::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HookInstalled {
    pub path: FilePath,
    pub executable: FilePath,
    #[serde(default)]
    pub timestamp: String,
}

impl HookInstalled {
    pub fn new(path: FilePath, executable: FilePath) -> Self {
        Self { path, executable, timestamp: String::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HookRemoved {
    pub path: FilePath,
    #[serde(default)]
    pub timestamp: String,
}

impl HookRemoved {
    pub fn new(path: FilePath) -> Self {
        Self { path, timestamp: String::new() }
    }
}
