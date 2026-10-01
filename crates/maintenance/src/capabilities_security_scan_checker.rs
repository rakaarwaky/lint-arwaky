use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_tool_name_vo::ToolName;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_maintenance::contract_maintenance_protocol::ISecurityScanProtocol;
use shared_maintenance::taxonomy_maintenance_vo::{SecurityFinding, SecurityScanReport};
use std::path::Path;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct SecurityScanChecker {
    io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Implementation ─────────────────────
impl ISecurityScanProtocol for SecurityScanChecker {
    fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport {
        let root = &project_path.value;
        let cargo_lock = Path::new(root).join("Cargo.lock");
        if cargo_lock.exists() {
            let (s, _, _) = self.io.run_external_command_in(
                &ToolName::new("cargo"),
                &["audit", "--json"],
                root,
            );
            let mut findings = Vec::new();
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&s) {
                if let Some(list) = json
                    .get("vulnerabilities")
                    .and_then(|v| v.get("list"))
                    .and_then(|l| l.as_array())
                {
                    for adv in list {
                        let pkg = adv
                            .get("package")
                            .and_then(|p| p.get("name"))
                            .and_then(|n| n.as_str())
                            .unwrap_or("unknown")
                            .to_string();
                        let severity = adv
                            .get("severity")
                            .and_then(|s| s.as_str())
                            .unwrap_or("unknown")
                            .to_string();
                        let cve = adv
                            .get("advisory")
                            .and_then(|a| a.get("id"))
                            .and_then(|i| i.as_str())
                            .unwrap_or("unknown")
                            .to_string();
                        findings.push(SecurityFinding {
                            severity,
                            test_id: cve,
                            file: pkg,
                            line: 0,
                            issue: "Advisory vulnerability".to_string(),
                        });
                    }
                }
            }
            return SecurityScanReport {
                language: "Rust".to_string(),
                tool_name: "cargo-audit".to_string(),
                findings,
                tool_installed: true,
            };
        }
        SecurityScanReport {
            language: "Unknown".to_string(),
            tool_name: "none".to_string(),
            findings: Vec::new(),
            tool_installed: false,
        }
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl SecurityScanChecker {
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        Self { io }
    }
}
