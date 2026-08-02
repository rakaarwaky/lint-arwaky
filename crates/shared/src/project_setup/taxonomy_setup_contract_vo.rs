// PURPOSE: SetupContractVOs — value objects used by ISetupManagementProtocol and
// ISetupInstallerProtocol contract surface.
//
// AES402: All primitive `String` / `Result<(), String>` / `Result<_, String>`
// return types and parameter types in ISetupManagementProtocol and
// ISetupInstallerProtocol are replaced with strongly-typed VOs.
//
// Naming: these VOs are scoped to the `project-setup` feature (which already
// has its own `taxonomy_doctor_vo`, `taxonomy_language_vo`, `taxonomy_stats_vo`).
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::common::taxonomy_suggestion_vo::DescriptionVO;

/// Name of the MCP binary as resolved on the host PATH (e.g. "lint-arwaky-cli").
/// Replaces the previous `String` return type of
/// `ISetupManagementProtocol::which_mcp_binary`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct McpBinaryNameVO {
    pub value: String,
}

impl McpBinaryNameVO {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Programming language detected for a project (e.g. "rust", "python",
/// "javascript", "typescript"). Replaces the previous `String` return type
/// of `ISetupManagementProtocol::detect_language`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectLanguageVO {
    pub value: String,
}

impl ProjectLanguageVO {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// List of programming languages detected for a project. Replaces the
/// previous `Vec<String>` return type of
/// `ISetupManagementProtocol::detect_languages`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectLanguagesVO {
    pub values: Vec<ProjectLanguageVO>,
}

impl ProjectLanguagesVO {
    pub fn new(values: Vec<ProjectLanguageVO>) -> Self {
        Self { values }
    }
    pub fn iter(&self) -> impl Iterator<Item = &ProjectLanguageVO> {
        self.values.iter()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// Error type for setup operations that previously returned
/// `Result<(), String>` or `Result<PathBuf, String>`. Replaces ad-hoc
/// `String` error types with a domain error VO so callers can
/// pattern-match on specific failure modes instead of free-form strings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SetupError {
    /// Filesystem / IO error (could not write file, could not create dir,
    /// could not read configuration, etc.). The wrapped string carries the
    /// OS-level error message; treat as opaque display text only.
    Io(String),
    /// The setup step was attempted with arguments that conflict with the
    /// current project state (e.g. trying to install a dependency that the
    /// project's lockfile already pins to an incompatible version).
    InvalidState(String),
    /// Catch-all for setup errors that don't fit a specific variant.
    /// Wraps a human-readable diagnostic message.
    Other(String),
}

impl SetupError {
    pub fn io(message: impl Into<String>) -> Self {
        Self::Io(message.into())
    }
    pub fn invalid_state(message: impl Into<String>) -> Self {
        Self::InvalidState(message.into())
    }
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other(message.into())
    }
}

impl std::fmt::Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(m) | Self::InvalidState(m) | Self::Other(m) => write!(f, "{}", m),
        }
    }
}

impl std::error::Error for SetupError {}

/// Result of writing a configuration file. The previous return type was
/// `Result<(), String>` — we now return `Result<DescriptionVO, SetupError>`
/// where the description carries a success message (e.g. "wrote
/// /path/to/lint_arwaky.config.yaml (256 bytes)") and the error carries a
/// structured failure cause.
pub type WriteConfigResult = Result<DescriptionVO, SetupError>;

/// Result of creating the global config directory. The previous return
/// type was `Result<std::path::PathBuf, String>` — we now return a
/// `FilePath` on success (which wraps `PathBuf` with the rest of the
/// contract's path-handling surface) and a `SetupError` on failure.
pub type CreateConfigDirResult = Result<PathBuf, SetupError>;
