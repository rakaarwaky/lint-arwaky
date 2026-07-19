// PURPOSE: MaintenanceChecker — IBypassCheckerProtocol for maintenance: toolchain diagnostics, security scan, dependency report
// Zero I/O: all external tool execution and file reading delegated to ports via DI.
use std::sync::Arc;

use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_filesystem_maintenance_port::IFileSystemMaintenancePort;
use shared::project_setup::contract_maintenance_protocol::IMaintenanceCheckerProtocol;
use shared::project_setup::contract_tool_executor_port::IToolExecutorPort;
use shared::project_setup::taxonomy_doctor_vo::{
    DependencyInfo, DependencyReport, SecurityFinding, SecurityScanReport, ToolStatus,
    ToolchainDiagnostics,
};

// Block 1: struct Definition
pub struct MaintenanceChecker {
    tool_executor: Arc<dyn IToolExecutorPort>,
    fs: Arc<dyn IFileSystemMaintenancePort>,
}

// Block 2: impl Trait for Struct (Public Contract)
#[async_trait::async_trait]
impl IMaintenanceCheckerProtocol for MaintenanceChecker {
    async fn diagnose_toolchain(&self) -> ToolchainDiagnostics {
        let mut rust_tools = vec![self.check_tool_status("cargo", true).await];
        let mut clippy_status = self.check_tool_status("cargo", true).await;
        clippy_status.name = "clippy".to_string();
        rust_tools.push(clippy_status);
        rust_tools.push(self.check_tool_status("rustfmt", true).await);

        let python_tools = vec![
            self.check_tool_status("python3", false).await,
            self.check_tool_status("ruff", false).await,
            self.check_tool_status("mypy", false).await,
            self.check_tool_status("bandit", false).await,
        ];

        let mut js_tools = vec![self.check_tool_status("node", false).await];

        let eslint_local = self.fs.file_exists("node_modules/.bin/eslint").await;
        let eslint_status = if eslint_local {
            ToolStatus {
                name: "eslint (local)".to_string(),
                status: "OK".to_string(),
                version: "local".to_string(),
            }
        } else {
            let mut st = self.check_tool_status("eslint", false).await;
            st.name = "eslint".to_string();
            st
        };
        js_tools.push(eslint_status);

        let prettier_local = self.fs.file_exists("node_modules/.bin/prettier").await;
        let prettier_status = if prettier_local {
            ToolStatus {
                name: "prettier (local)".to_string(),
                status: "OK".to_string(),
                version: "local".to_string(),
            }
        } else {
            let mut st = self.check_tool_status("prettier", false).await;
            st.name = "prettier".to_string();
            st
        };
        js_tools.push(prettier_status);

        let tsc_local = self.fs.file_exists("node_modules/.bin/tsc").await;
        let tsc_status = if tsc_local {
            ToolStatus {
                name: "tsc (local)".to_string(),
                status: "OK".to_string(),
                version: "local".to_string(),
            }
        } else {
            let mut st = self.check_tool_status("tsc", false).await;
            st.name = "tsc".to_string();
            st
        };
        js_tools.push(tsc_status);

        let vcs_tools = vec![
            self.check_tool_status("git", true).await,
            self.check_tool_status("jj", false).await,
        ];

        let binary_path = self.tool_executor.get_binary_path().await;

        ToolchainDiagnostics {
            rust_tools,
            python_tools,
            js_tools,
            vcs_tools,
            binary_path,
        }
    }

    async fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport {
        let root = &project_path.value;
        let cargo_lock = format!("{}/Cargo.lock", root);

        if self.fs.file_exists(&cargo_lock).await {
            let output = self
                .tool_executor
                .run_tool_in_dir("cargo", &["audit", "--json"], root)
                .await;
            if !output.success {
                return SecurityScanReport {
                    language: "Rust".to_string(),
                    tool_name: "cargo-audit".to_string(),
                    findings: Vec::new(),
                    tool_installed: false,
                };
            }
            return parse_cargo_audit(&output.stdout);
        }

        let output = self
            .tool_executor
            .run_tool_in_dir("bandit", &["-r", "--format", "json", "."], root)
            .await;
        if !output.success && output.stdout.is_empty() {
            return SecurityScanReport {
                language: "Python".to_string(),
                tool_name: "bandit".to_string(),
                findings: Vec::new(),
                tool_installed: false,
            };
        }
        parse_bandit_output(&output.stdout)
    }

    async fn run_dependency_report(
        &self,
        project_path: &FilePath,
    ) -> Result<DependencyReport, String> {
        let root = &project_path.value;

        let cargo_lock = format!("{}/Cargo.lock", root);
        if self.fs.file_exists(&cargo_lock).await {
            let content = self.fs.read_file(&cargo_lock).await?;
            let cargo_toml = format!("{}/Cargo.toml", root);
            let direct_deps = if self.fs.file_exists(&cargo_toml).await {
                let toml_content = self.fs.read_file(&cargo_toml).await.unwrap_or_default();
                parse_cargo_toml_direct_deps(&toml_content)
            } else {
                std::collections::HashSet::new()
            };
            return Ok(parse_cargo_lock(&content, &direct_deps));
        }

        let pyproject = format!("{}/pyproject.toml", root);
        if self.fs.file_exists(&pyproject).await {
            let content = self.fs.read_file(&pyproject).await?;
            return Ok(parse_pyproject(&content));
        }

        let reqs = format!("{}/requirements.txt", root);
        if self.fs.file_exists(&reqs).await {
            let content = self.fs.read_file(&reqs).await?;
            return Ok(parse_requirements(&content));
        }

        Err("No dependency files found (Cargo.lock, pyproject.toml, requirements.txt)".to_string())
    }
}

// Block 3: constructors & helpers
impl MaintenanceChecker {
    pub fn new(
        tool_executor: Arc<dyn IToolExecutorPort>,
        fs: Arc<dyn IFileSystemMaintenancePort>,
    ) -> Self {
        Self { tool_executor, fs }
    }

    async fn check_tool_status(&self, name: &str, required: bool) -> ToolStatus {
        let output = self.tool_executor.run_tool(name, &["--version"]).await;
        let (status, version) = if output.success {
            let ver = output
                .stdout
                .lines()
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
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

impl Default for MaintenanceChecker {
    fn default() -> Self {
        use crate::infrastructure_filesystem_maintenance_adapter::FileSystemMaintenanceAdapter;
        use crate::infrastructure_tool_executor_adapter::ToolExecutorAdapter;
        Self::new(
            Arc::new(ToolExecutorAdapter::new()),
            Arc::new(FileSystemMaintenanceAdapter::new()),
        )
    }
}

// --- Pure business logic helpers (no I/O) ---

fn parse_cargo_audit(stdout: &str) -> SecurityScanReport {
    let mut findings = Vec::new();
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(stdout) {
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
}

fn parse_bandit_output(stdout: &str) -> SecurityScanReport {
    let mut findings = Vec::new();
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(stdout) {
        if let Some(results) = json.get("results").and_then(|r| r.as_array()) {
            for r in results {
                let test_id = r
                    .get("test_id")
                    .and_then(|t| t.as_str())
                    .unwrap_or("")
                    .to_string();
                let issue = r
                    .get("issue_text")
                    .and_then(|t| t.as_str())
                    .unwrap_or("")
                    .to_string();
                let severity = r
                    .get("issue_severity")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();
                let fname = r
                    .get("filename")
                    .and_then(|f| f.as_str())
                    .unwrap_or("")
                    .to_string();
                let line = r
                    .get("line_number")
                    .and_then(|l| l.as_u64())
                    .unwrap_or_default();
                findings.push(SecurityFinding {
                    severity,
                    test_id,
                    file: fname,
                    line,
                    issue,
                });
            }
        }
    }
    SecurityScanReport {
        language: "Python".to_string(),
        tool_name: "bandit".to_string(),
        findings,
        tool_installed: true,
    }
}

fn parse_cargo_toml_direct_deps(toml_content: &str) -> std::collections::HashSet<String> {
    let mut direct_deps = std::collections::HashSet::new();
    let mut in_deps = false;
    for line in toml_content.lines() {
        if line.trim().starts_with("[dependencies]") {
            in_deps = true;
            continue;
        }
        if in_deps && line.trim().starts_with('[') {
            in_deps = false;
        }
        if in_deps {
            if let Some(eq) = line.find('=') {
                let name = line[..eq].trim().to_string();
                if !name.is_empty() && !name.starts_with('#') {
                    direct_deps.insert(name);
                }
            }
        }
    }
    direct_deps
}

fn parse_cargo_lock(
    content: &str,
    direct_deps: &std::collections::HashSet<String>,
) -> DependencyReport {
    let mut in_package = false;
    let mut pkg_name = String::new();
    let mut pkg_version = String::new();
    let mut dependencies = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[[package]]" {
            if !pkg_name.is_empty() && !pkg_version.is_empty() {
                let dep_type = if direct_deps.contains(pkg_name.as_str()) {
                    "direct".to_string()
                } else {
                    "transitive".to_string()
                };
                dependencies.push(DependencyInfo {
                    name: pkg_name.clone(),
                    version: pkg_version.clone(),
                    dep_type,
                });
            }
            pkg_name.clear();
            pkg_version.clear();
            in_package = true;
            continue;
        }
        if in_package {
            if let Some(name_val) = trimmed.strip_prefix("name = ") {
                pkg_name = name_val.trim_matches('"').to_string();
            } else if let Some(ver_val) = trimmed.strip_prefix("version = ") {
                pkg_version = ver_val.trim_matches('"').to_string();
            }
        }
    }
    if !pkg_name.is_empty() && !pkg_version.is_empty() {
        let dep_type = if direct_deps.contains(pkg_name.as_str()) {
            "direct".to_string()
        } else {
            "transitive".to_string()
        };
        dependencies.push(DependencyInfo {
            name: pkg_name,
            version: pkg_version,
            dep_type,
        });
    }

    DependencyReport {
        language: "Rust".to_string(),
        dependencies,
    }
}

fn parse_pyproject(content: &str) -> DependencyReport {
    let mut dependencies = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if t.contains('=') && !t.starts_with('[') && !t.starts_with('#') {
            if let Some(eq) = t.find('=') {
                let name = t[..eq].trim().to_string();
                let version = t[eq + 1..]
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string();
                dependencies.push(DependencyInfo {
                    name,
                    version,
                    dep_type: "python".to_string(),
                });
            }
        }
    }
    DependencyReport {
        language: "Python".to_string(),
        dependencies,
    }
}

fn parse_requirements(content: &str) -> DependencyReport {
    let mut dependencies = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if !t.is_empty() && !t.starts_with('#') {
            dependencies.push(DependencyInfo {
                name: t.to_string(),
                version: String::new(),
                dep_type: "python".to_string(),
            });
        }
    }
    DependencyReport {
        language: "Python".to_string(),
        dependencies,
    }
}
