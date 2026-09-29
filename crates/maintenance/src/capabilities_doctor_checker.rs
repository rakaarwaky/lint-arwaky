use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_suggestion_vo::DescriptionVO;
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::maintenance::contract_maintenance_protocol::IDoctorProtocol;
use shared::maintenance::taxonomy_maintenance_vo::{DoctorResultVO, ToolchainDiagnostics};
use std::sync::Arc;

use shared::maintenance::utility_maintenance_helpers;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct DoctorChecker {
    _io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Implementation ─────────────────────
impl IDoctorProtocol for DoctorChecker {
    fn doctor(&self) -> DoctorResultVO {
        let tools = self.diagnose_toolchain();
        let rust_ver = tools
            .rust_tools
            .first()
            .map(|t| t.version.clone())
            .unwrap_or_default();
        let python_ver = tools
            .python_tools
            .first()
            .map(|t| t.version.clone())
            .unwrap_or_default();
        let node_ver = tools
            .js_tools
            .first()
            .map(|t| t.version.clone())
            .unwrap_or_default();
        let all_ok = tools.rust_tools.iter().all(|t| t.status == "OK");
        let mut adapter_statuses = std::collections::HashMap::new();
        for t in &tools.rust_tools {
            if let Ok(name) = AdapterName::new(t.name.clone()) {
                adapter_statuses.insert(name, t.status.clone());
            }
        }
        for t in &tools.python_tools {
            if let Ok(name) = AdapterName::new(t.name.clone()) {
                adapter_statuses.insert(name, t.status.clone());
            }
        }
        DoctorResultVO {
            python_version: DescriptionVO::new(python_ver),
            rust_version: DescriptionVO::new(rust_ver),
            node_version: DescriptionVO::new(node_ver),
            is_installed: ComplianceStatus::new(all_ok),
            config_found: FilePathList::new(Vec::new()),
            adapter_statuses,
            issues: Vec::new(),
            healthy: ComplianceStatus::new(all_ok),
        }
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl DoctorChecker {
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        Self { _io: io }
    }

    fn diagnose_toolchain(&self) -> ToolchainDiagnostics {
        let rust_tools = vec![
            utility_maintenance_helpers::check_tool("rustc", &["--version"], true),
            utility_maintenance_helpers::check_tool("cargo", &["--version"], true),
        ];
        let mut clippy_status =
            utility_maintenance_helpers::check_tool("cargo", &["clippy", "--version"], true);
        clippy_status.name = "clippy".to_string();
        let rust_tools = [rust_tools, vec![clippy_status]].concat();
        let rust_tools = [
            rust_tools.clone(),
            vec![utility_maintenance_helpers::check_tool(
                "rustfmt",
                &["--version"],
                true,
            )],
        ]
        .concat();
        let python_tools = vec![
            utility_maintenance_helpers::check_tool("python3", &["--version"], false),
            utility_maintenance_helpers::check_tool("ruff", &["--version"], false),
            utility_maintenance_helpers::check_tool("mypy", &["--version"], false),
        ];
        let mut js_tools = vec![utility_maintenance_helpers::check_tool(
            "node",
            &["--version"],
            false,
        )];
        js_tools.push(utility_maintenance_helpers::check_tool(
            "eslint",
            &["--version"],
            false,
        ));
        let vcs_tools = vec![utility_maintenance_helpers::check_tool(
            "git",
            &["--version"],
            true,
        )];
        let binary_path = std::env::current_exe()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        ToolchainDiagnostics {
            rust_tools,
            python_tools,
            js_tools,
            vcs_tools,
            binary_path,
        }
    }
}
