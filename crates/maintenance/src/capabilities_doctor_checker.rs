use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_message_vo::ComplianceStatus;
use shared_common::taxonomy_paths_vo::FilePathList;
use shared_common::taxonomy_suggestion_vo::DescriptionVO;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_maintenance::contract_maintenance_protocol::IDoctorProtocol;
use shared_maintenance::taxonomy_maintenance_vo::{DoctorResultVO, ToolchainDiagnostics};
use std::sync::Arc;

use shared_maintenance::utility_maintenance_helpers;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct DoctorChecker {
    _io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Implementation ─────────────────────
impl IDoctorProtocol for DoctorChecker {
    /// Environment health summary: language runtime versions, adapter
    /// statuses, and overall install health derived from the toolchain.
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

    /// Every tool version and status the doctor report is built from, in the
    /// same traversal order the doctor contract specifies.
    fn diagnose_toolchain(&self) -> ToolchainDiagnostics {
        utility_maintenance_helpers::build_toolchain_diagnostics()
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl DoctorChecker {
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        Self { _io: io }
    }
}
