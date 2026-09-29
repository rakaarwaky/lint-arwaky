use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::maintenance::contract_maintenance_protocol::IToolchainDiagnosticProtocol;
use shared::maintenance::taxonomy_maintenance_vo::ToolchainDiagnostics;
use std::sync::Arc;

use shared::maintenance::utility_maintenance_helpers;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct ToolchainDiagnosticChecker {
    _io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Implementation ─────────────────────
impl IToolchainDiagnosticProtocol for ToolchainDiagnosticChecker {
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

// ─── Block 3: Constructors & Helpers ──────────────────────
impl ToolchainDiagnosticChecker {
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        Self { _io: io }
    }
}
