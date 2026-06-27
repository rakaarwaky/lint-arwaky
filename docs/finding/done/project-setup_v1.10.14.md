# Crate: project-setup (v1.10.14)

This document contains the source code for feature crate `project-setup` along with its corresponding and imported definitions from the `shared` crate.

## Problem Statement

The following issues were detected by `lint-arwaky-cli scan`:

```
============================================================
  AES Architecture Compliance Report
============================================================
  Project: /home/raka/mcp-arwaky/lint-arwaky/crates/project-setup
  Violations: 3
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/project-setup/src/capabilities_setup_processor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/project-setup/src/capabilities_setup_processor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/project-setup/src/capabilities_setup_processor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
```

---

## File List

- [crates/project-setup/Cargo.toml](file:///home/raka/mcp-arwaky/lint-arwaky/crates/project-setup/Cargo.toml)
- [crates/project-setup/src/agent_setup_orchestrator.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/project-setup/src/agent_setup_orchestrator.rs)
- [crates/project-setup/src/capabilities_setup_processor.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/project-setup/src/capabilities_setup_processor.rs)
- [crates/project-setup/src/infrastructure_setup_installer_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/project-setup/src/infrastructure_setup_installer_adapter.rs)
- [crates/project-setup/src/lib.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/project-setup/src/lib.rs)
- [crates/project-setup/src/root_project_setup_container.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/project-setup/src/root_project_setup_container.rs)
- [crates/shared/src/cli-commands/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/mod.rs)
- [crates/shared/src/cli-commands/taxonomy_protocol_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_protocol_vo.rs)
- [crates/shared/src/common/taxonomy_job_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_job_vo.rs)
- [crates/shared/src/common/taxonomy_suggestion_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_suggestion_vo.rs)
- [crates/shared/src/mcp-server/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/mcp-server/mod.rs)
- [crates/shared/src/project-setup/contract_maintenance_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/contract_maintenance_aggregate.rs)
- [crates/shared/src/project-setup/contract_setup_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/contract_setup_aggregate.rs)
- [crates/shared/src/project-setup/contract_setup_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/contract_setup_protocol.rs)
- [crates/shared/src/project-setup/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/mod.rs)
- [crates/shared/src/project-setup/taxonomy_doctor_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/taxonomy_doctor_vo.rs)
- [crates/shared/src/project-setup/taxonomy_language_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/taxonomy_language_vo.rs)
- [crates/shared/src/project-setup/taxonomy_setup_contract_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/taxonomy_setup_contract_vo.rs)
- [crates/shared/src/project-setup/taxonomy_stats_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/taxonomy_stats_vo.rs)
- [crates/shared/src/source-parsing/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/mod.rs)
- [crates/shared/src/source-parsing/taxonomy_path_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_path_vo.rs)

---

## File: crates/project-setup/Cargo.toml

```toml
[package]
name = "project_setup-lint-arwaky"
version = "1.10.14"
edition = "2021"
description = "Project scaffolding and doctor checks: initialises a project with AES-compliant directory layout, MCP config, and CI templates."
license = "MIT"
repository = "https://github.com/rakaarwaky/lint-arwaky"
publish = false

[lints]
workspace = true

[dependencies]  # (unchanged)
serde.workspace = true
serde_json.workspace = true
async-trait.workspace = true
tokio.workspace = true
shared.workspace = true
dirs.workspace = true
```

---

## File: crates/project-setup/src/agent_setup_orchestrator.rs

```rust
// PURPOSE: SetupOrchestrator — orchestrates project initialization and setup operations
use shared::cli_commands::taxonomy_protocol_vo::TransportProtocol;
use shared::cli_commands::taxonomy_protocol_vo::TransportUrlVO;
use shared::mcp_server::taxonomy_job_vo::EnvContentVO;
use shared::mcp_server::taxonomy_job_vo::McpConfigVO;
use shared::mcp_server::taxonomy_job_vo::SuccessStatus;
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use shared::project_setup::taxonomy_setup_contract_vo::ProjectLanguageVO;
use shared::source_parsing::taxonomy_path_vo::DirectoryPath;
use std::collections::HashMap;

use async_trait::async_trait;

use shared::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use std::sync::Arc;

pub struct SetupManagementOrchestrator {
    protocol: Arc<dyn ISetupManagementProtocol>,
}

#[async_trait]
impl SetupManagementAggregate for SetupManagementOrchestrator {
    fn check_http(&self, _url: &TransportUrlVO) -> SuccessStatus {
        SuccessStatus::new(true)
    }

    fn generate_env(&self, transport: &TransportProtocol, _home: &DirectoryPath) -> EnvContentVO {
        EnvContentVO {
            value: format!("TRANSPORT={}\n", transport),
        }
    }

    fn generate_mcp_config(&self, transport: &TransportProtocol) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert(
            "transport".to_string(),
            serde_json::json!(transport.to_string()),
        );
        McpConfigVO { value: config }
    }

    fn mcp_config_claude(&self, transport: &TransportProtocol) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert(
            "transport".to_string(),
            serde_json::json!(transport.to_string()),
        );
        config.insert("client".to_string(), serde_json::json!("claude"));
        McpConfigVO { value: config }
    }

    fn mcp_config_hermes(&self, transport: &TransportProtocol) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert(
            "transport".to_string(),
            serde_json::json!(transport.to_string()),
        );
        config.insert("client".to_string(), serde_json::json!("hermes"));
        McpConfigVO { value: config }
    }

    fn mcp_config_vscode(&self, transport: &TransportProtocol) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert(
            "transport".to_string(),
            serde_json::json!(transport.to_string()),
        );
        config.insert("client".to_string(), serde_json::json!("vscode"));
        McpConfigVO { value: config }
    }
    async fn install_python_adapters(&self) -> SuccessStatus {
        self.protocol.install_python_adapters().await
    }

    async fn install_javascript_adapters(&self, sudo: bool) -> SuccessStatus {
        self.protocol.install_javascript_adapters(sudo).await
    }

    fn detect_language(&self) -> ProjectLanguageVO {
        self.protocol.detect_language()
    }

    fn get_config_template(&self, language: &str) -> &'static str {
        self.protocol.get_config_template(language)
    }

    fn write_config_file(
        &self,
        filename: &str,
        content: &str,
    ) -> shared::project_setup::WriteConfigResult {
        self.protocol.write_config_file(filename, content)
    }

    fn create_global_config_dir(&self) -> shared::project_setup::CreateConfigDirResult {
        self.protocol.create_global_config_dir()
    }

    fn file_exists(&self, path: &str) -> bool {
        self.protocol.file_exists(path)
    }
}

impl SetupManagementOrchestrator {
    pub fn new(protocol: Arc<dyn ISetupManagementProtocol>) -> Self {
        Self { protocol }
    }
}
```

---

## File: crates/project-setup/src/capabilities_setup_processor.rs

```rust
// PURPOSE: SetupProcessor — processes project setup steps (env, gitignore, config, hooks)

use std::collections::HashMap;

use shared::mcp_server::taxonomy_job_vo::{EnvContentVO, McpConfigVO};
use shared::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use shared::project_setup::taxonomy_setup_contract_vo::{
    McpBinaryNameVO, ProjectLanguageVO, SetupError,
};
use shared::source_parsing::taxonomy_path_vo::DirectoryPath;
use shared::taxonomy_suggestion_vo::DescriptionVO;

use shared::mcp_server::taxonomy_job_vo::SuccessStatus;
use shared::project_setup::contract_setup_protocol::ISetupInstallerPort;
use std::sync::Arc;

/// Business logic for generating setup and configuration artifacts.
pub struct SetupManagementProcessor {
    installer: Arc<dyn ISetupInstallerPort>,
}

impl SetupManagementProcessor {
    pub fn new(installer: Arc<dyn ISetupInstallerPort>) -> Self {
        Self { installer }
    }
}

#[async_trait::async_trait]
impl ISetupManagementProtocol for SetupManagementProcessor {
    /// Generate .env content for the lint-arwaky environment.
    fn generate_env(&self, home: &DirectoryPath) -> EnvContentVO {
        let lines = [
            "# Lint Arwaky Environment Configuration".to_string(),
            "# Generated by: lint-arwaky init".to_string(),
            "".to_string(),
            "# Phantom root (for JS/TS linters):".to_string(),
            format!("PHANTOM_ROOT={}/", home.value),
        ];
        EnvContentVO::new(lines.join("\n") + "\n")
    }

    /// Generate the base mcp.json entry for lint-arwaky.
    fn generate_mcp_config(&self) -> McpConfigVO {
        let mut config = HashMap::new();
        let server_entry = serde_json::json!({
            "command": "lint-arwaky",
            "args": [],
            "alwaysAllow": ["lint", "execute_command", "health_check", "list_commands", "read_skill_context"]
        });
        config.insert("lint-arwaky".to_string(), server_entry);
        McpConfigVO::new(config)
    }

    /// Generate Claude Desktop MCP config format.
    fn mcp_config_claude(&self) -> McpConfigVO {
        let base = self.generate_mcp_config();
        let mut config = HashMap::new();
        config.insert("mcpServers".to_string(), serde_json::json!(base.value()));
        McpConfigVO::new(config)
    }

    /// Generate Hermes/Antigravity MCP config format.
    fn mcp_config_hermes(&self) -> McpConfigVO {
        self.generate_mcp_config()
    }

    /// Generate VS Code MCP config format.
    fn mcp_config_vscode(&self) -> McpConfigVO {
        let base = self.generate_mcp_config();
        let mut config = HashMap::new();
        config.insert(
            "mcp".to_string(),
            serde_json::json!({"servers": base.value()}),
        );
        McpConfigVO::new(config)
    }

    /// Resolve the path to the lint-arwaky-mcp binary.
    fn which_mcp_binary(&self) -> McpBinaryNameVO {
        let candidates = [
            std::env::current_exe()
                .ok()
                .and_then(|p| {
                    p.parent()
                        .map(|d| d.join("lint-arwaky-mcp").to_string_lossy().to_string())
                })
                .unwrap_or_default(),
            format!(
                "{}/bin/lint-arwaky-mcp",
                std::env::var("CARGO_HOME").unwrap_or_else(|_| "~/.cargo".to_string())
            ),
            "lint-arwaky-mcp".to_string(),
        ];
        for c in &candidates {
            if !c.is_empty() && std::path::Path::new(c).exists() {
                return McpBinaryNameVO::new(c.clone());
            }
        }
        let which_output = std::process::Command::new("which")
            .arg("lint-arwaky-mcp")
            .output()
            .ok()
            .and_then(|o| {
                if o.status.success() {
                    Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "lint-arwaky-mcp".to_string());
        McpBinaryNameVO::new(which_output)
    }

    async fn install_python_adapters(&self) -> SuccessStatus {
        let res = self
            .installer
            .install_python_packages(&[
                "ruff".to_string(),
                "mypy".to_string(),
                "bandit".to_string(),
            ])
            .await;
        SuccessStatus::new(res.is_ok())
    }

    async fn install_javascript_adapters(&self, sudo: bool) -> SuccessStatus {
        let res = self
            .installer
            .install_npm_packages(
                &[
                    "eslint".to_string(),
                    "prettier".to_string(),
                    "typescript".to_string(),
                ],
                sudo,
            )
            .await;
        SuccessStatus::new(res.is_ok())
    }

    fn detect_language(&self) -> ProjectLanguageVO {
        if std::path::Path::new("crates").exists() {
            ProjectLanguageVO::new("rust")
        } else if std::path::Path::new("packages").exists()
            || std::path::Path::new("modules").exists()
            || std::path::Path::new("pyproject.toml").exists()
        {
            ProjectLanguageVO::new("python")
        } else if std::path::Path::new("package.json").exists() {
            ProjectLanguageVO::new("javascript")
        } else {
            ProjectLanguageVO::new("rust")
        }
    }

    fn get_config_template(&self, language: &str) -> &'static str {
        match language {
            "rust" => include_str!("../../../lint_arwaky.config.rust.yaml"),
            "python" => include_str!("../../../lint_arwaky.config.python.yaml"),
            "javascript" => include_str!("../../../lint_arwaky.config.javascript.yaml"),
            _ => include_str!("../../../lint_arwaky.config.rust.yaml"),
        }
    }

    fn write_config_file(
        &self,
        filename: &str,
        content: &str,
    ) -> Result<DescriptionVO, SetupError> {
        std::fs::write(filename, content).map_err(|e| SetupError::io(e.to_string()))?;
        Ok(DescriptionVO::new(format!(
            "wrote {} ({} bytes)",
            filename,
            content.len()
        )))
    }

    fn create_global_config_dir(&self) -> Result<std::path::PathBuf, SetupError> {
        let config_dir = dirs::config_dir()
            .map(|d| d.join("lint-arwaky"))
            .ok_or_else(|| SetupError::invalid_state("Could not determine XDG config directory"))?;
        std::fs::create_dir_all(&config_dir).map_err(|e| SetupError::io(e.to_string()))?;
        Ok(config_dir)
    }

    fn file_exists(&self, path: &str) -> bool {
        std::path::Path::new(path).exists()
    }
}
```

---

## File: crates/project-setup/src/infrastructure_setup_installer_adapter.rs

```rust
// PURPOSE: SetupInstallerAdapter — infrastructure adapter for executing npm/pip install commands
use async_trait::async_trait;
use shared::project_setup::contract_setup_protocol::ISetupInstallerPort;
use shared::project_setup::taxonomy_language_vo::ProjectLanguage;
use shared::project_setup::taxonomy_setup_contract_vo::SetupError;

pub struct SetupInstallerAdapter;

impl Default for SetupInstallerAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl SetupInstallerAdapter {
    pub fn new() -> Self {
        let _dummy = ProjectLanguage::new("rust");
        Self
    }
}

#[async_trait]
impl ISetupInstallerPort for SetupInstallerAdapter {
    async fn install_python_packages(&self, packages: &[String]) -> Result<(), SetupError> {
        let status = tokio::process::Command::new("pip")
            .args(["install", "--user"])
            .args(packages)
            .status()
            .await
            .map_err(|e| SetupError::io(e.to_string()))?;
        if status.success() {
            return Ok(());
        }

        // Retry with --break-system-packages if initial attempt fails (typically PEP 668 on modern Linux)
        let status2 = tokio::process::Command::new("pip")
            .args(["install", "--user", "--break-system-packages"])
            .args(packages)
            .status()
            .await;

        match status2 {
            Ok(s) if s.success() => Ok(()),
            _ => Err(SetupError::other(format!(
                "pip install exited with status {:?}",
                status.code()
            ))),
        }
    }

    async fn install_npm_packages(
        &self,
        packages: &[String],
        sudo: bool,
    ) -> Result<(), SetupError> {
        let (cmd, args) = if sudo {
            ("sudo", vec!["npm", "install", "-g"])
        } else {
            ("npm", vec!["install", "-g"])
        };
        let status = tokio::process::Command::new(cmd)
            .args(args)
            .args(packages)
            .status()
            .await
            .map_err(|e| SetupError::io(e.to_string()))?;
        if status.success() {
            Ok(())
        } else {
            Err(SetupError::other(format!(
                "npm install exited with status {:?}",
                status.code()
            )))
        }
    }
}
```

---

## File: crates/project-setup/src/lib.rs

```rust
// PURPOSE: Module declarations for project-setup (orchestrator, processor, container)
pub mod agent_setup_orchestrator;
pub use agent_setup_orchestrator::SetupManagementOrchestrator;
pub mod capabilities_setup_processor;
pub use capabilities_setup_processor::SetupManagementProcessor;
pub mod infrastructure_setup_installer_adapter;
pub mod root_project_setup_container;
pub use infrastructure_setup_installer_adapter::SetupInstallerAdapter;
```

---

## File: crates/project-setup/src/root_project_setup_container.rs

```rust
// PURPOSE: SetupContainer — wiring for project-setup feature (root layer, wiring only)
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use shared::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use std::sync::Arc;

pub struct SetupContainer {
    aggregate: Arc<dyn SetupManagementAggregate>,
    protocol: Arc<dyn ISetupManagementProtocol>,
}

impl SetupContainer {
    pub fn new() -> Self {
        let installer =
            Arc::new(crate::infrastructure_setup_installer_adapter::SetupInstallerAdapter::new());
        let protocol =
            Arc::new(crate::capabilities_setup_processor::SetupManagementProcessor::new(installer));
        let aggregate = Arc::new(
            crate::agent_setup_orchestrator::SetupManagementOrchestrator::new(protocol.clone()),
        );
        Self {
            aggregate,
            protocol,
        }
    }

    pub fn aggregate(&self) -> Arc<dyn SetupManagementAggregate> {
        self.aggregate.clone()
    }

    pub fn protocol(&self) -> Arc<dyn ISetupManagementProtocol> {
        self.protocol.clone()
    }
}
impl Default for SetupContainer {
    fn default() -> Self {
        Self::new()
    }
}
```

---

## File: crates/shared/src/cli-commands/mod.rs

```rust
// cli-commands — taxonomy and contract types
pub mod contract_executor_port;
pub mod taxonomy_catalog_constant;
pub mod taxonomy_cli_vo;
pub mod taxonomy_command_catalog_vo;
pub mod taxonomy_metadata_vo;
pub mod taxonomy_position_vo;
pub mod taxonomy_protocol_vo;
pub mod taxonomy_result_vo;
pub mod taxonomy_score_vo;
pub mod taxonomy_severity_vo;
pub mod taxonomy_transport_error;
```

---

## File: crates/shared/src/cli-commands/taxonomy_protocol_vo.rs

```rust
// PURPOSE: TransportEndpoint, TransportProtocol, TransportUrlVO — value objects for transport endpoint configuration
use crate::string_value_object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransportEndpoint {
    pub protocol: TransportProtocol,
    pub address: String,
}

impl Default for TransportEndpoint {
    fn default() -> Self {
        Self {
            protocol: TransportProtocol::STDAggregate,
            address: String::new(),
        }
    }
}

impl TransportEndpoint {
    pub fn new(protocol: TransportProtocol, address: String) -> Self {
        Self { protocol, address }
    }

    pub fn display_name(&self) -> String {
        match self.protocol {
            TransportProtocol::HTTP => format!("HTTP({})", self.address),
            TransportProtocol::UnixSocket => format!("Socket({})", self.address),
            TransportProtocol::STDAggregate => "Stdio(direct)".to_string(),
        }
    }
    pub fn from_url(url: &str) -> Self {
        let (protocol, address) = match url {
            u if u.starts_with("http://") || u.starts_with("https://") => {
                (TransportProtocol::HTTP, u.to_string())
            }
            "stdio" => (TransportProtocol::STDAggregate, "stdio".to_string()),
            u if u.starts_with('/') || u.starts_with('.') => {
                (TransportProtocol::UnixSocket, u.to_string())
            }
            _ => (TransportProtocol::STDAggregate, "stdio".to_string()),
        };
        Self { protocol, address }
    }
}

impl std::fmt::Display for TransportEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.protocol, self.address)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TransportProtocol {
    #[serde(rename = "HTTP")]
    HTTP,
    #[serde(rename = "UnixSocket")]
    UnixSocket,
    #[serde(rename = "Stdio")]
    STDAggregate,
}

impl std::fmt::Display for TransportProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportProtocol::HTTP => write!(f, "HTTP"),
            TransportProtocol::UnixSocket => write!(f, "UnixSocket"),
            TransportProtocol::STDAggregate => write!(f, "Stdio"),
        }
    }
}

impl TransportProtocol {
    pub fn needs_desktop_commander(&self) -> bool {
        matches!(
            self,
            TransportProtocol::HTTP | TransportProtocol::UnixSocket
        )
    }
}

string_value_object!(TransportUrlVO);
```

---

## File: crates/shared/src/common/taxonomy_job_vo.rs

```rust
// PURPOSE: PipelineJob, SuccessStatus, EnvContentVO, McpConfigVO — value objects for pipeline job lifecycle tracking
// ResponseData is re-exported from common for backward compatibility
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::string_value_object;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::common::taxonomy_response_data_vo::ResponseData;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum JobStatus {
    #[serde(rename = "pending")]
    PENDING,
    #[serde(rename = "running")]
    RUNNING,
    #[serde(rename = "completed")]
    COMPLETED,
    #[serde(rename = "failed")]
    FAILED,
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::PENDING => write!(f, "pending"),
            JobStatus::RUNNING => write!(f, "running"),
            JobStatus::COMPLETED => write!(f, "completed"),
            JobStatus::FAILED => write!(f, "failed"),
        }
    }
}

// Manual impl: `SuccessStatus` overrides `Display` to render "SUCCESS"/"FAILURE"
// instead of `true`/`false`, and the macro does not currently support a clean
// `bool` cast (Rust forbids `i64 as bool`). Kept as a hand-rolled VO.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SuccessStatus {
    pub value: bool,
}

impl Default for SuccessStatus {
    fn default() -> Self {
        Self::new(false)
    }
}

impl SuccessStatus {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}

impl std::fmt::Display for SuccessStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value {
            write!(f, "SUCCESS")
        } else {
            write!(f, "FAILURE")
        }
    }
}

impl std::ops::Deref for SuccessStatus {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.value
    }
}

/// `HashMap<String, serde_json::Value>` payload VOs. Wrapped via macro so they
/// pick up the standard `new`/`value`/`Default`/serde impls.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LintStatusActionArgs {
    #[serde(default)]
    pub value: HashMap<String, serde_json::Value>,
}

impl Default for LintStatusActionArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl LintStatusActionArgs {
    pub fn new() -> Self {
        Self {
            value: HashMap::new(),
        }
    }
    pub fn value(&self) -> &HashMap<String, serde_json::Value> {
        &self.value
    }
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.value.get(key)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterMetadata {
    pub name: AdapterName,
    pub class_path: String,
    #[serde(default)]
    pub description: String,
}

impl AdapterMetadata {
    pub fn new(name: AdapterName, class_path: String) -> Self {
        Self {
            name,
            class_path,
            description: String::new(),
        }
    }
}

string_value_object!(EnvContentVO);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpConfigVO {
    #[serde(default)]
    pub value: HashMap<String, serde_json::Value>,
}

impl McpConfigVO {
    pub fn new(value: HashMap<String, serde_json::Value>) -> Self {
        Self { value }
    }
    pub fn value(&self) -> &HashMap<String, serde_json::Value> {
        &self.value
    }
}
```

---

## File: crates/shared/src/common/taxonomy_suggestion_vo.rs

```rust
// PURPOSE: ClassPath, DescriptionVO, LogOutput, MetadataVO, StdError, StdOutput, Suggestion — domain value objects for CLI suggestion/result data
use crate::string_value_object;
use serde::{Deserialize, Serialize};

// ClassPath, DescriptionVO, LogOutput, StdError, StdOutput, and Suggestion all
// follow the standard String-wrapper VO pattern; the macro emits the
// new/value/Display/From/Hash/PartialEq/Deserialize impls they need.
string_value_object!(ClassPath);
string_value_object!(DescriptionVO);
string_value_object!(LogOutput);
string_value_object!(StdError);
string_value_object!(StdOutput);
string_value_object!(Suggestion);

/// Strongly-typed replacement for the previous
/// `HashMap<String, serde_json::Value>` return type. Each field has a real
/// domain meaning — there is no `serde_json::Value` in the contract surface.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetadataVO {
    pub values: std::collections::HashMap<String, serde_json::Value>,
}

impl MetadataVO {
    pub fn new(value: std::collections::HashMap<String, serde_json::Value>) -> Self {
        Self { values: value }
    }
    pub fn value(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.values
    }
}
```

---

## File: crates/shared/src/mcp-server/mod.rs

```rust
// mcp-server — taxonomy and contract types
// Re-export from common for backward compatibility
pub use crate::common::taxonomy_action_vo;
pub use crate::common::taxonomy_job_vo;
```

---

## File: crates/shared/src/project-setup/contract_maintenance_aggregate.rs

```rust
// PURPOSE: Aggregate: MaintenanceCommandsAggregate trait — contract for maintenance operations (stats, doctor, clean, update, cancel)
use crate::mcp_server::taxonomy_action_vo::JobId;
use crate::project_setup::taxonomy_doctor_vo::{
    DependencyReport, DoctorResultVO, SecurityScanReport, ToolchainDiagnostics,
};
use crate::project_setup::taxonomy_stats_vo::MaintenanceStatsVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait MaintenanceCommandsAggregate: Send + Sync {
    async fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO;
    async fn clean(&self);
    async fn update(&self);
    async fn doctor(&self) -> DoctorResultVO;
    async fn cancel(&self, job_id: JobId);
    async fn diagnose_toolchain(&self) -> ToolchainDiagnostics;
    async fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport;
    async fn run_dependency_report(
        &self,
        project_path: &FilePath,
    ) -> Result<DependencyReport, String>;
}
```

---

## File: crates/shared/src/project-setup/contract_setup_aggregate.rs

```rust
// PURPOSE: SetupAggregate — aggregate trait for project setup orchestration
use crate::cli_commands::taxonomy_protocol_vo::TransportProtocol;
use crate::cli_commands::taxonomy_protocol_vo::TransportUrlVO;
use crate::mcp_server::taxonomy_job_vo::EnvContentVO;
use crate::mcp_server::taxonomy_job_vo::McpConfigVO;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;
use crate::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use crate::project_setup::taxonomy_setup_contract_vo::{
    CreateConfigDirResult, ProjectLanguageVO, WriteConfigResult,
};
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;

pub type SetupMgmtProtocol = Box<dyn ISetupManagementProtocol>;

#[async_trait::async_trait]
pub trait SetupManagementAggregate: Send + Sync {
    fn check_http(&self, url: &TransportUrlVO) -> SuccessStatus;
    fn generate_env(&self, transport: &TransportProtocol, home: &DirectoryPath) -> EnvContentVO;
    fn generate_mcp_config(&self, transport: &TransportProtocol) -> McpConfigVO;
    fn mcp_config_claude(&self, transport: &TransportProtocol) -> McpConfigVO;
    fn mcp_config_hermes(&self, transport: &TransportProtocol) -> McpConfigVO;
    fn mcp_config_vscode(&self, transport: &TransportProtocol) -> McpConfigVO;
    async fn install_python_adapters(&self) -> SuccessStatus;
    async fn install_javascript_adapters(&self, sudo: bool) -> SuccessStatus;
    fn detect_language(&self) -> ProjectLanguageVO;
    fn get_config_template(&self, language: &str) -> &'static str;
    fn write_config_file(&self, filename: &str, content: &str) -> WriteConfigResult;
    fn create_global_config_dir(&self) -> CreateConfigDirResult;
    fn file_exists(&self, path: &str) -> bool;
}
```

---

## File: crates/shared/src/project-setup/contract_setup_protocol.rs

```rust
// PURPOSE: ISetupProtocol — protocol trait for project setup step definitions
// AES402: All primitive `String` / `Result<(), String>` / `Result<PathBuf, String>`
// return types in ISetupManagementProtocol are replaced with strongly-typed VOs.
//   * `String` returns → `McpBinaryNameVO` / `ProjectLanguageVO`
//   * `Result<(), String>` → `WriteConfigResult` (= `Result<DescriptionVO, SetupError>`)
//   * `Result<PathBuf, String>` → `CreateConfigDirResult` (= `Result<PathBuf, SetupError>`)
//   * `&str` parameters → kept (idiomatic borrow, AES402 allows)
//   * `bool` parameters → kept (semantic toggle, AES402 allows)
use crate::mcp_server::taxonomy_job_vo::{EnvContentVO, McpConfigVO, SuccessStatus};
use crate::project_setup::taxonomy_setup_contract_vo::{
    CreateConfigDirResult, McpBinaryNameVO, ProjectLanguageVO, SetupError, WriteConfigResult,
};
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;

#[async_trait::async_trait]
pub trait ISetupManagementProtocol: Send + Sync {
    fn generate_env(&self, home: &DirectoryPath) -> EnvContentVO;
    fn generate_mcp_config(&self) -> McpConfigVO;
    fn mcp_config_claude(&self) -> McpConfigVO;
    fn mcp_config_hermes(&self) -> McpConfigVO;
    fn mcp_config_vscode(&self) -> McpConfigVO;
    /// Resolve the name of the MCP binary on the host PATH.
    fn which_mcp_binary(&self) -> McpBinaryNameVO;
    async fn install_python_adapters(&self) -> SuccessStatus;
    async fn install_javascript_adapters(&self, sudo: bool) -> SuccessStatus;
    /// Detect the dominant programming language of the current project.
    fn detect_language(&self) -> ProjectLanguageVO;
    fn get_config_template(&self, language: &str) -> &'static str;
    /// Write a configuration file to disk. Returns a description of the
    /// operation on success, or a structured `SetupError` on failure.
    fn write_config_file(&self, filename: &str, content: &str) -> WriteConfigResult;
    /// Create the global config directory and return its path.
    fn create_global_config_dir(&self) -> CreateConfigDirResult;
    fn file_exists(&self, path: &str) -> bool;
}

/// AES402: `Result<(), String>` is replaced with `Result<(), SetupError>`
/// so callers can pattern-match on specific failure modes (Io vs
/// InvalidState vs Other) instead of inspecting free-form error strings.
pub type InstallPackagesResult = Result<(), SetupError>;

#[async_trait::async_trait]
pub trait ISetupInstallerPort: Send + Sync {
    async fn install_python_packages(&self, packages: &[String]) -> InstallPackagesResult;
    async fn install_npm_packages(&self, packages: &[String], sudo: bool) -> InstallPackagesResult;
}
```

---

## File: crates/shared/src/project-setup/mod.rs

```rust
pub mod contract_maintenance_aggregate;
pub mod contract_setup_aggregate;
pub mod contract_setup_protocol;
pub mod taxonomy_doctor_vo;
pub mod taxonomy_language_vo;
pub mod taxonomy_setup_contract_vo;
pub mod taxonomy_stats_vo;
pub use taxonomy_setup_contract_vo::{
    CreateConfigDirResult, McpBinaryNameVO, ProjectLanguageVO, SetupError, WriteConfigResult,
};
```

---

## File: crates/shared/src/project-setup/taxonomy_doctor_vo.rs

```rust
// PURPOSE: DoctorResultVO, DoctorCheck — VOs for project health diagnostics results
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

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
```

---

## File: crates/shared/src/project-setup/taxonomy_language_vo.rs

```rust
// PURPOSE: LanguageConfigVO — value object for programming language configuration
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectLanguage {
    pub value: String,
}

impl ProjectLanguage {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LanguageSource {
    pub language: String,
    pub confidence: u8,
    pub source: String,
}

impl LanguageSource {
    pub fn new(language: impl Into<String>, confidence: u8, source: impl Into<String>) -> Self {
        Self {
            language: language.into(),
            confidence,
            source: source.into(),
        }
    }
}
```

---

## File: crates/shared/src/project-setup/taxonomy_setup_contract_vo.rs

```rust
// PURPOSE: SetupContractVOs — value objects used by ISetupManagementProtocol and
// ISetupInstallerPort contract surface.
//
// AES402: All primitive `String` / `Result<(), String>` / `Result<_, String>`
// return types and parameter types in ISetupManagementProtocol and
// ISetupInstallerPort are replaced with strongly-typed VOs.
//
// Naming: these VOs are scoped to the `project-setup` feature (which already
// has its own `taxonomy_doctor_vo`, `taxonomy_language_vo`, `taxonomy_stats_vo`).
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::taxonomy_suggestion_vo::DescriptionVO;

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
```

---

## File: crates/shared/src/project-setup/taxonomy_stats_vo.rs

```rust
// PURPOSE: ProjectStatsVO, MaintenanceStatsVO — VOs for project statistics and maintenance data
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::Score;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MaintenanceStatsVO {
    pub project_path: FilePath,
    pub total_files: Count,
    pub test_files: Count,
    pub test_ratio: Score,
    pub python_files: Count,
}

impl MaintenanceStatsVO {
    pub fn new(
        project_path: FilePath,
        total_files: Count,
        test_files: Count,
        test_ratio: Score,
        python_files: Count,
    ) -> Self {
        Self {
            project_path,
            total_files,
            test_files,
            test_ratio,
            python_files,
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
```

---

## File: crates/shared/src/source-parsing/mod.rs

```rust
// source-parsing — taxonomy and contract types
pub mod contract_language_detector_port;
pub mod contract_parser_port;
pub mod contract_path_normalization_port;
pub mod contract_scanner_provider_port;
pub mod infrastructure_file_collector_provider;
pub mod taxonomy_adapter_error;
pub mod taxonomy_barrel_provider_vo;
pub mod taxonomy_file_collector_helper;
pub mod taxonomy_language_detector_helper;
pub mod taxonomy_naming_error;
pub mod taxonomy_naming_list_vo;
pub mod taxonomy_parser_error;
pub mod taxonomy_path_vo;
pub mod taxonomy_paths_vo;
pub mod taxonomy_semantic_error;
pub use infrastructure_file_collector_provider::{
    collect_all_source_files, count_loc, walk_rs_files, FileCollectorProvider,
};
```

---

## File: crates/shared/src/source-parsing/taxonomy_path_vo.rs

```rust
// PURPOSE: FilePath, DirectoryPath — value objects for validated file and directory paths
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// file_path_vo — File and directory path value objects.
///
/// File path identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FilePath {
    pub value: String,
}

impl FilePath {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new FilePath from a string.
    ///
    /// # Errors
    /// Returns an error if the path is invalid (empty or only whitespace).
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let mut value = value.into();
        if value.trim().is_empty() {
            return Err("File path cannot be empty".to_string());
        }
        // Normalize: replace backslashes with forward slashes, and collapse multiple slashes.
        value = value.replace('\\', "/");
        // Remove all trailing slashes
        while value.ends_with('/') && value.len() > 1 {
            value.pop();
        }
        // If after normalization it's empty, then it was all slashes -> treat as root
        if value.is_empty() {
            return Ok(FilePath {
                value: "/".to_string(),
            });
        }
        Ok(FilePath { value })
    }

    /// File extension without dot.
    pub fn extension(&self) -> String {
        let special_files = [
            "Makefile",
            "Dockerfile",
            "Dockerfile.dev",
            "Dockerfile.prod",
            ".bashrc",
            ".profile",
            ".zshrc",
            ".gitignore",
            ".dockerignore",
        ];
        // Operate on the basename, not the full path — `./foo.rs` must still yield
        // `rs` as its extension, and `.bashrc` (which is fully a basename) must NOT
        // be confused with a hidden file mid-path.
        let basename = match self.value.rsplit('/').next() {
            Some(b) => b,
            None => return String::new(),
        };
        if special_files.contains(&basename) || basename.starts_with('.') {
            return String::new();
        }
        match basename.rsplit('.').next() {
            Some(ext) => ext.to_string(),
            None => String::new(),
        }
    }

    /// Check if path has given extension (without dot).
    pub fn has_extension(&self, ext: &str) -> bool {
        self.extension().eq_ignore_ascii_case(ext)
    }

    /// Extract filename/basename of the path.
    pub fn basename(&self) -> String {
        match self.value.rsplit('/').next() {
            Some(f) => f.to_string(),
            None => self.value.clone(),
        }
    }

    /// Check if the path is a barrel file.
    pub fn is_barrel_file(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py" | "mod.rs" | "index.ts" | "index.js"
        )
    }

    /// Check if the path is a module/layer entry point file.
    pub fn is_entry_point(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py" | "main.py" | "py.typed" | "app.py" | "lib.rs"
        )
    }
}

impl std::ops::Deref for FilePath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Hash for FilePath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

/// Directory path identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct DirectoryPath {
    pub value: String,
}

impl DirectoryPath {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new DirectoryPath from a string.
    ///
    /// # Errors
    /// Returns an error if the path is invalid (empty or only whitespace).
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let mut value = value.into();
        if value.trim().is_empty() {
            return Err("Directory path cannot be empty".to_string());
        }
        // Normalize: replace backslashes with forward slashes, and remove trailing slash.
        value = value.replace('\\', "/");
        // Remove trailing slash unless it's just "/"
        if value.ends_with('/') && value.len() > 1 {
            value.pop();
        }
        Ok(DirectoryPath { value })
    }
}

impl std::ops::Deref for DirectoryPath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for DirectoryPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'de> serde::Deserialize<'de> for DirectoryPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DirectoryPath::new(s).map_err(serde::de::Error::custom)
    }
}

impl Hash for DirectoryPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::{DirectoryPath, FilePath};

    #[test]
    fn test_file_path_new() {
        let fp = FilePath::new("test.txt").unwrap_or_default();
        assert_eq!(fp.value, "test.txt");
        assert_eq!(fp.extension(), "txt");
        assert!(fp.has_extension("txt"));
        assert!(!fp.has_extension("md"));

        // Test normalization
        let fp = FilePath::new("path\\to\\file.txt").unwrap_or_default();
        assert_eq!(fp.value, "path/to/file.txt");

        let fp = FilePath::new("path/to/file/").unwrap_or_default();
        assert_eq!(fp.value, "path/to/file");

        let fp = FilePath::new("/").unwrap_or_default();
        assert_eq!(fp.value, "/");

        let fp = FilePath::new("///").unwrap_or_default();
        assert_eq!(fp.value, "/");
    }

    #[test]
    fn test_file_path_invalid() {
        assert!(FilePath::new("").is_err());
        assert!(FilePath::new("   ").is_err());
    }

    #[test]
    fn test_directory_path_new() {
        let dp = DirectoryPath::new("test/dir").unwrap_or_default();
        assert_eq!(dp.value, "test/dir");

        let dp = DirectoryPath::new("test/dir/").unwrap_or_default();
        assert_eq!(dp.value, "test/dir");

        let dp = DirectoryPath::new("/").unwrap_or_default();
        assert_eq!(dp.value, "/");
    }

    #[test]
    fn test_directory_path_invalid() {
        assert!(DirectoryPath::new("").is_err());
        assert!(DirectoryPath::new("   ").is_err());
    }

    /// Regression: `./foo.rs` must report `rs` as its extension, not empty string.
    /// The old implementation treated any path starting with `.` as having no
    /// extension, which caused `LanguageDetector::is_lintable` to skip relative
    /// paths emitted by `std::fs::read_dir` in `collect_source_files`. Result: zero
    /// files collected when the user runs `lint-arwaky check .` on a directory
    /// tree with non-`.git`-anchored paths.
    #[test]
    fn test_extension_with_dot_slash_prefix() {
        let fp = FilePath::new("./foo.rs").unwrap_or_default();
        assert_eq!(fp.extension(), "rs");
        let fp = FilePath::new("./nested/foo.py").unwrap_or_default();
        assert_eq!(fp.extension(), "py");
        let fp = FilePath::new(".//foo.ts").unwrap_or_default();
        assert_eq!(fp.extension(), "ts");
    }

    /// Regression: a hidden-file basename (e.g. `.bashrc`) must still report no
    /// extension, since the basename itself starts with a dot.
    #[test]
    fn test_extension_hidden_basename() {
        let fp = FilePath::new(".bashrc").unwrap_or_default();
        assert_eq!(fp.extension(), "");
        let fp = FilePath::new("/home/user/.gitignore").unwrap_or_default();
        assert_eq!(fp.extension(), "");
    }

    /// Regression: full paths must still resolve the extension on the basename.
    #[test]
    fn test_extension_full_path() {
        let fp =
            FilePath::new("/tmp/bypass_test/capabilities_unwrap_checker.rs").unwrap_or_default();
        assert_eq!(fp.extension(), "rs");
        let fp = FilePath::new("crates/code-analysis/src/foo.rs").unwrap_or_default();
        assert_eq!(fp.extension(), "rs");
    }

    /// Makefile / Dockerfile — special filenames, no extension.
    #[test]
    fn test_extension_special_filenames() {
        let fp = FilePath::new("Makefile").unwrap_or_default();
        assert_eq!(fp.extension(), "");
        let fp = FilePath::new("Dockerfile").unwrap_or_default();
        assert_eq!(fp.extension(), "");
    }
}
```

---
