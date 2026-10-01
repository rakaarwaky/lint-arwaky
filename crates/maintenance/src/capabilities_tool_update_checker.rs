use shared_common::taxonomy_tool_name_vo::ToolName;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_maintenance::contract_maintenance_protocol::IToolUpdateProtocol;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct ToolUpdateChecker {
    io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Implementation ─────────────────────
impl IToolUpdateProtocol for ToolUpdateChecker {
    fn update(&self) {
        let _ = self.io.run_external_command_in(
            &ToolName::new("pip"),
            &["install", "--upgrade", "ruff", "mypy", "bandit"],
            ".",
        );
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl ToolUpdateChecker {
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        Self { io }
    }
}
