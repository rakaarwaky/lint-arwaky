use shared::common::{
    AdapterName, ComplianceStatus, Count, DescriptionVO, FilePath, FilePathList, Score,
};
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::maintenance::IMaintenanceCheckerProtocol;
use shared::maintenance::MaintenanceStatsVO;
use shared::maintenance::{
    DependencyInfo, DependencyReport, DoctorResultVO, HealthCheckAdapterVO, HealthCheckResult,
    SecurityFinding, SecurityScanReport, ToolStatus, ToolchainDiagnostics,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct MaintenanceChecker {
    filesystem: Arc<dyn IFilesystemAggregate>,
}

impl MaintenanceChecker {
    pub fn new(filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
        Self { filesystem }
    }

    fn check_tool(&self, name: &str, args: &[&str], required: bool) -> ToolStatus {
        let (stdout, _, success) = self.filesystem.run_external_command_in(name, args, ".");
        let (status, version) = if success {
            let ver = stdout.lines().next().unwrap_or("").trim().to_string();
            ("OK".to_string(), ver)
        } else if required {
            ("FAIL".to_string(), "NOT FOUND".to_string())
        } else {
            ("WARN".to_string(), "NOT FOUND".to_string())
        };
        ToolStatus {
            name: name.to_string(),
            status,
            version,
        }
    }
}

impl IMaintenanceCheckerProtocol for MaintenanceChecker {
    fn diagnose_toolchain(&self) -> ToolchainDiagnostics {
        let mut rust_tools = vec![self.check_tool("rustc", &["--version"], true)];
        rust_tools.push(self.check_tool("cargo", &["--version"], true));
        let mut clippy_status = self.check_tool("cargo", &["clippy", "--version"], true);
        clippy_status.name = "clippy".to_string();
        rust_tools.push(clippy_status);
        rust_tools.push(self.check_tool("rustfmt", &["--version"], true));
        let python_tools = vec![
            self.check_tool("python3", &["--version"], false),
            self.check_tool("ruff", &["--version"], false),
            self.check_tool("mypy", &["--version"], false),
        ];
        let mut js_tools = vec![self.check_tool("node", &["--version"], false)];
        js_tools.push(self.check_tool("eslint", &["--version"], false));
        let vcs_tools = vec![self.check_tool("git", &["--version"], true)];
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

    fn health_check(&self) -> HealthCheckResult {
        // FRD FR-004: all 9 adapters must be checked
        let mut adapters = Vec::new();
        for (name, bin, args, lang) in &[
            (
                "clippy",
                "cargo",
                &["clippy", "--version"] as &[&str],
                "Rust",
            ),
            ("rustfmt", "rustfmt", &["--version"] as &[&str], "Rust"),
            (
                "cargo-audit",
                "cargo",
                &["audit", "--version"] as &[&str],
                "Rust",
            ),
            ("ruff", "ruff", &["--version"] as &[&str], "Python"),
            ("mypy", "mypy", &["--version"] as &[&str], "Python"),
            ("bandit", "bandit", &["--version"] as &[&str], "Python"),
            ("eslint", "eslint", &["--version"] as &[&str], "JS/TS"),
            ("prettier", "prettier", &["--version"] as &[&str], "JS/TS"),
            ("tsc", "tsc", &["--version"] as &[&str], "JS/TS"),
        ] {
            let status = self.check_tool(bin, args, false);
            adapters.push(HealthCheckAdapterVO {
                name: name.to_string(),
                language: lang.to_string(),
                available: status.status == "OK",
            });
        }
        HealthCheckResult { adapters }
    }

    fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport {
        let root = &project_path.value;
        let cargo_lock = std::path::Path::new(root).join("Cargo.lock");
        if cargo_lock.exists() {
            let (s, _, _) =
                self.filesystem
                    .run_external_command_in("cargo", &["audit", "--json"], root);
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
            SecurityScanReport {
                language: "Rust".to_string(),
                tool_name: "cargo-audit".to_string(),
                findings,
                tool_installed: true,
            }
        } else {
            SecurityScanReport {
                language: "Unknown".to_string(),
                tool_name: "none".to_string(),
                findings: Vec::new(),
                tool_installed: false,
            }
        }
    }

    fn run_dependency_report(&self, project_path: &FilePath) -> Result<DependencyReport, String> {
        let root = &project_path.value;
        let cargo_lock = std::path::Path::new(root).join("Cargo.lock");
        if cargo_lock.exists() {
            let content = self
                .filesystem
                .read_to_string(&cargo_lock)
                .map_err(|e| e.to_string())?;
            let mut dependencies = Vec::new();
            let mut in_package = false;
            let mut pkg_name = String::new();
            let mut pkg_version = String::new();
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed == "[[package]]" {
                    if !pkg_name.is_empty() && !pkg_version.is_empty() {
                        dependencies.push(DependencyInfo {
                            name: pkg_name.clone(),
                            version: pkg_version.clone(),
                            dep_type: "transitive".to_string(),
                        });
                    }
                    pkg_name.clear();
                    pkg_version.clear();
                    in_package = true;
                    continue;
                }
                if in_package {
                    if let Some(v) = trimmed.strip_prefix("name = ") {
                        pkg_name = v.trim_matches('"').to_string();
                    } else if let Some(v) = trimmed.strip_prefix("version = ") {
                        pkg_version = v.trim_matches('"').to_string();
                    }
                }
            }
            if !pkg_name.is_empty() && !pkg_version.is_empty() {
                dependencies.push(DependencyInfo {
                    name: pkg_name,
                    version: pkg_version,
                    dep_type: "transitive".to_string(),
                });
            }
            Ok(DependencyReport {
                language: "Rust".to_string(),
                dependencies,
            })
        } else {
            Err("No Cargo.lock found".to_string())
        }
    }

    fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO {
        let root = &project_path.value;
        let root_path = std::path::Path::new(root);
        let mut total_files = 0u64;
        let mut test_files = 0u64;
        let mut python_files = 0u64;
        let mut rust_files = 0u64;
        let mut js_files = 0u64;
        if let Ok(entries) = std::fs::read_dir(root_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    total_files += 1;
                    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if name.contains("test") || name.contains("spec") {
                        test_files += 1;
                    }
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        match ext {
                            "rs" => rust_files += 1,
                            "py" => python_files += 1,
                            "ts" | "js" | "jsx" | "tsx" => js_files += 1,
                            _ => {}
                        }
                    }
                }
            }
        }
        let source_count = rust_files + python_files + js_files;
        MaintenanceStatsVO {
            project_path: project_path.clone(),
            total_files: Count::new(total_files as i64),
            test_files: Count::new(test_files as i64),
            test_ratio: Score::new(if source_count > 0 {
                test_files as f64 / source_count as f64
            } else {
                0.0
            }),
            python_files: Count::new(python_files as i64),
            rust_files: Count::new(rust_files as i64),
            js_files: Count::new(js_files as i64),
        }
    }

    fn clean(&self) {
        for dir in &[
            ".pytest_cache",
            "__pycache__",
            "node_modules/.cache",
            "target",
        ] {
            let path = std::path::Path::new(".").join(dir);
            if path.exists() {
                let _ = std::fs::remove_dir_all(&path);
            }
        }
    }

    fn update(&self) {
        let _ = self.filesystem.run_external_command_in(
            "pip",
            &["install", "--upgrade", "ruff", "mypy", "bandit"],
            ".",
        );
    }

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
        let mut adapter_statuses = HashMap::new();
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
            is_installed: if all_ok {
                ComplianceStatus::new(true)
            } else {
                ComplianceStatus::new(false)
            },
            config_found: FilePathList::new(Vec::new()),
            adapter_statuses,
            issues: Vec::new(),
            healthy: if all_ok {
                ComplianceStatus::new(true)
            } else {
                ComplianceStatus::new(false)
            },
        }
    }
}
