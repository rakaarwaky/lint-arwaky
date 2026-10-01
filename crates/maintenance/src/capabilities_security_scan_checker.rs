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
        let path = Path::new(root);
        if self.io.path_exists(&path.join("Cargo.lock")) {
            return self.scan_rust(root);
        }
        if self.io.path_exists(&path.join("requirements.txt"))
            || self.io.path_exists(&path.join("poetry.lock"))
            || self.io.path_exists(&path.join("Pipfile.lock"))
            || self.io.path_exists(&path.join("uv.lock"))
        {
            return self.scan_python(root);
        }
        if self.io.path_exists(&path.join("package-lock.json"))
            || self.io.path_exists(&path.join("npm-shrinkwrap.json"))
        {
            return self.scan_javascript(root);
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

    fn scan_rust(&self, root: &str) -> SecurityScanReport {
        let (stdout, _, _) = self.io.run_external_command_in(
            &ToolName::new("cargo"),
            &["audit", "--json"],
            root,
        );
        let parsed = serde_json::from_str::<serde_json::Value>(&stdout).ok();
        let findings = parsed
            .as_ref()
            .and_then(|json| json.pointer("/vulnerabilities/list"))
            .and_then(serde_json::Value::as_array)
            .map(|advisories| {
                advisories
                    .iter()
                    .map(|advisory| SecurityFinding {
                        severity: advisory
                            .get("severity")
                            .and_then(serde_json::Value::as_str)
                            .unwrap_or("unknown")
                            .to_string(),
                        test_id: advisory
                            .pointer("/advisory/id")
                            .and_then(serde_json::Value::as_str)
                            .unwrap_or("unknown")
                            .to_string(),
                        file: advisory
                            .pointer("/package/name")
                            .and_then(serde_json::Value::as_str)
                            .unwrap_or("unknown")
                            .to_string(),
                        line: 0,
                        issue: "Advisory vulnerability".to_string(),
                    })
                    .collect()
            })
            .unwrap_or_default();
        SecurityScanReport {
            language: "Rust".to_string(),
            tool_name: "cargo-audit".to_string(),
            findings,
            tool_installed: parsed.is_some(),
        }
    }

    fn scan_python(&self, root: &str) -> SecurityScanReport {
        let requirements = Path::new(root).join("requirements.txt");
        let requirements_string = requirements.to_string_lossy().to_string();
        let args = if self.io.path_exists(&requirements) {
            vec!["--format", "json", "--requirement", &requirements_string]
        } else {
            vec!["--format", "json"]
        };
        let (stdout, _, _) =
            self.io
                .run_external_command_in(&ToolName::new("pip-audit"), &args, root);
        let parsed = serde_json::from_str::<serde_json::Value>(&stdout).ok();
        let findings = parsed
            .as_ref()
            .and_then(|json| {
                json.as_array()
                    .or_else(|| json.get("dependencies").and_then(serde_json::Value::as_array))
            })
            .map(|dependencies| {
                dependencies
                    .iter()
                    .flat_map(|dependency| {
                        let package = dependency
                            .get("name")
                            .and_then(serde_json::Value::as_str)
                            .unwrap_or("unknown")
                            .to_string();
                        dependency
                            .get("vulns")
                            .and_then(serde_json::Value::as_array)
                            .into_iter()
                            .flatten()
                            .map(move |vulnerability| SecurityFinding {
                                severity: "unknown".to_string(),
                                test_id: vulnerability
                                    .get("id")
                                    .and_then(serde_json::Value::as_str)
                                    .unwrap_or("unknown")
                                    .to_string(),
                                file: package.clone(),
                                line: 0,
                                issue: vulnerability
                                    .get("description")
                                    .and_then(serde_json::Value::as_str)
                                    .unwrap_or("Dependency vulnerability")
                                    .to_string(),
                            })
                    })
                    .collect()
            })
            .unwrap_or_default();
        SecurityScanReport {
            language: "Python".to_string(),
            tool_name: "pip-audit".to_string(),
            findings,
            tool_installed: parsed.is_some(),
        }
    }

    fn scan_javascript(&self, root: &str) -> SecurityScanReport {
        let (stdout, _, _) = self.io.run_external_command_in(
            &ToolName::new("npm"),
            &["audit", "--json"],
            root,
        );
        let parsed = serde_json::from_str::<serde_json::Value>(&stdout).ok();
        let findings = parsed
            .as_ref()
            .and_then(|json| json.get("vulnerabilities"))
            .and_then(serde_json::Value::as_object)
            .map(|vulnerabilities| {
                vulnerabilities
                    .iter()
                    .map(|(package, vulnerability)| SecurityFinding {
                        severity: vulnerability
                            .get("severity")
                            .and_then(serde_json::Value::as_str)
                            .unwrap_or("unknown")
                            .to_string(),
                        test_id: format!("npm-audit::{package}"),
                        file: package.clone(),
                        line: 0,
                        issue: vulnerability
                            .get("title")
                            .and_then(serde_json::Value::as_str)
                            .unwrap_or("Dependency vulnerability")
                            .to_string(),
                    })
                    .collect()
            })
            .unwrap_or_default();
        SecurityScanReport {
            language: "JavaScript/TypeScript".to_string(),
            tool_name: "npm audit".to_string(),
            findings,
            tool_installed: parsed.is_some(),
        }
    }
}
