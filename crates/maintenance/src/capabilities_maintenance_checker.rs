// PURPOSE: MaintenanceChecker — business logic capabilities for running audits and checking toolchains
use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_maintenance_protocol::IMaintenanceCheckerProtocol;
use shared::project_setup::taxonomy_doctor_vo::{
    DependencyInfo, DependencyReport, SecurityFinding, SecurityScanReport, ToolStatus,
    ToolchainDiagnostics,
};

pub struct MaintenanceChecker;

impl Default for MaintenanceChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl MaintenanceChecker {
    pub fn new() -> Self {
        Self
    }

    pub async fn diagnose_toolchain(&self) -> ToolchainDiagnostics {
        let check_tool = |name: &str, args: &[&str], required: bool| -> ToolStatus {
            let output = std::process::Command::new(name).args(args).output();
            let (status, version) = match output {
                Ok(o) if o.status.success() => {
                    let ver = match String::from_utf8_lossy(&o.stdout).lines().next() {
                        Some(v) => v.trim().to_string(),
                        None => String::new(),
                    };
                    ("OK".to_string(), ver)
                }
                _ => {
                    if required {
                        ("FAIL".to_string(), "NOT FOUND".to_string())
                    } else {
                        ("WARN".to_string(), "NOT FOUND".to_string())
                    }
                }
            };
            ToolStatus {
                name: name.to_string(),
                status,
                version,
            }
        };

        let mut rust_tools = vec![check_tool("cargo", &["--version"], true)];
        let mut clippy_status = check_tool("cargo", &["clippy", "--version"], true);
        clippy_status.name = "clippy".to_string();
        rust_tools.push(clippy_status);
        rust_tools.push(check_tool("rustfmt", &["--version"], true));

        let python_tools = vec![
            check_tool("python3", &["--version"], false),
            check_tool("ruff", &["--version"], false),
            check_tool("mypy", &["--version"], false),
            check_tool("bandit", &["--version"], false),
        ];

        let mut js_tools = vec![check_tool("node", &["--version"], false)];
        let eslint_local = std::path::Path::new("node_modules/.bin/eslint");
        let eslint_status = if eslint_local.exists() {
            ToolStatus {
                name: "eslint (local)".to_string(),
                status: "OK".to_string(),
                version: "local".to_string(),
            }
        } else {
            let mut st = check_tool("eslint", &["--version"], false);
            st.name = "eslint".to_string();
            st
        };
        js_tools.push(eslint_status);

        let prettier_local = std::path::Path::new("node_modules/.bin/prettier");
        let prettier_status = if prettier_local.exists() {
            ToolStatus {
                name: "prettier (local)".to_string(),
                status: "OK".to_string(),
                version: "local".to_string(),
            }
        } else {
            let mut st = check_tool("prettier", &["--version"], false);
            st.name = "prettier".to_string();
            st
        };
        js_tools.push(prettier_status);

        let tsc_local = std::path::Path::new("node_modules/.bin/tsc");
        let tsc_status = if tsc_local.exists() {
            ToolStatus {
                name: "tsc (local)".to_string(),
                status: "OK".to_string(),
                version: "local".to_string(),
            }
        } else {
            let mut st = check_tool("tsc", &["--version"], false);
            st.name = "tsc".to_string();
            st
        };
        js_tools.push(tsc_status);

        let vcs_tools = vec![
            check_tool("git", &["--version"], true),
            check_tool("jj", &["--version"], false),
        ];

        let binary_path = match std::env::current_exe() {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(_) => String::new(),
        };

        ToolchainDiagnostics {
            rust_tools,
            python_tools,
            js_tools,
            vcs_tools,
            binary_path,
        }
    }

    pub async fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport {
        let root = &project_path.value;
        let cargo_lock = std::path::Path::new(root).join("Cargo.lock");
        if cargo_lock.exists() {
            let output = std::process::Command::new("cargo")
                .args(["audit", "--json"])
                .current_dir(root)
                .output();
            match output {
                Ok(o) => {
                    let s = String::from_utf8_lossy(&o.stdout);
                    let mut findings = Vec::new();
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&s) {
                        if let Some(list) = json
                            .get("vulnerabilities")
                            .and_then(|v| v.get("list"))
                            .and_then(|l| l.as_array())
                        {
                            for adv in list {
                                let pkg = match adv
                                    .get("package")
                                    .and_then(|p| p.get("name"))
                                    .and_then(|n| n.as_str())
                                {
                                    Some(s) => s.to_string(),
                                    None => "unknown".to_string(),
                                };
                                let severity = match adv.get("severity").and_then(|s| s.as_str()) {
                                    Some(s) => s.to_string(),
                                    None => "unknown".to_string(),
                                };
                                let cve = match adv
                                    .get("advisory")
                                    .and_then(|a| a.get("id"))
                                    .and_then(|i| i.as_str())
                                {
                                    Some(s) => s.to_string(),
                                    None => "unknown".to_string(),
                                };
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
                Err(_) => SecurityScanReport {
                    language: "Rust".to_string(),
                    tool_name: "cargo-audit".to_string(),
                    findings: Vec::new(),
                    tool_installed: false,
                },
            }
        } else {
            let output = std::process::Command::new("bandit")
                .args(["-r", "--format", "json", root])
                .output();
            match output {
                Ok(o) => {
                    let s = String::from_utf8_lossy(&o.stdout);
                    let mut findings = Vec::new();
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&s) {
                        if let Some(results) = json.get("results").and_then(|r| r.as_array()) {
                            for r in results {
                                let test_id = match r.get("test_id").and_then(|t| t.as_str()) {
                                    Some(s) => s.to_string(),
                                    None => String::new(),
                                };
                                let issue = match r.get("issue_text").and_then(|t| t.as_str()) {
                                    Some(s) => s.to_string(),
                                    None => String::new(),
                                };
                                let severity =
                                    match r.get("issue_severity").and_then(|s| s.as_str()) {
                                        Some(s) => s.to_string(),
                                        None => String::new(),
                                    };
                                let fname = match r.get("filename").and_then(|f| f.as_str()) {
                                    Some(s) => s.to_string(),
                                    None => String::new(),
                                };
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
                Err(_) => SecurityScanReport {
                    language: "Python".to_string(),
                    tool_name: "bandit".to_string(),
                    findings: Vec::new(),
                    tool_installed: false,
                },
            }
        }
    }

    pub async fn run_dependency_report(
        &self,
        project_path: &FilePath,
    ) -> Result<DependencyReport, String> {
        let root = &project_path.value;
        let cargo_lock = std::path::Path::new(root).join("Cargo.lock");
        if cargo_lock.exists() {
            let content = std::fs::read_to_string(&cargo_lock).map_err(|e| e.to_string())?;
            let mut in_package = false;
            let mut pkg_name = String::new();
            let mut pkg_version = String::new();
            let mut dependencies = Vec::new();

            let cargo_toml = std::path::Path::new(root).join("Cargo.toml");
            let mut direct_deps = std::collections::HashSet::new();
            if let Ok(toml_content) = std::fs::read_to_string(&cargo_toml) {
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
            }

            let add_pkg = |pkg_name: &str,
                           pkg_version: &str,
                           direct_deps: &std::collections::HashSet<String>,
                           dependencies: &mut Vec<DependencyInfo>| {
                if !pkg_name.is_empty() && !pkg_version.is_empty() {
                    let dep_type = if direct_deps.contains(pkg_name) {
                        "direct".to_string()
                    } else {
                        "transitive".to_string()
                    };
                    dependencies.push(DependencyInfo {
                        name: pkg_name.to_string(),
                        version: pkg_version.to_string(),
                        dep_type,
                    });
                }
            };

            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed == "[[package]]" {
                    add_pkg(&pkg_name, &pkg_version, &direct_deps, &mut dependencies);
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
            add_pkg(&pkg_name, &pkg_version, &direct_deps, &mut dependencies);

            Ok(DependencyReport {
                language: "Rust".to_string(),
                dependencies,
            })
        } else {
            let pyproject = std::path::Path::new(root).join("pyproject.toml");
            if pyproject.exists() {
                let content = std::fs::read_to_string(&pyproject).map_err(|e| e.to_string())?;
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
                Ok(DependencyReport {
                    language: "Python".to_string(),
                    dependencies,
                })
            } else {
                let reqs = std::path::Path::new(root).join("requirements.txt");
                if reqs.exists() {
                    let content = std::fs::read_to_string(&reqs).map_err(|e| e.to_string())?;
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
                    Ok(DependencyReport {
                        language: "Python".to_string(),
                        dependencies,
                    })
                } else {
                    Err(
                        "No dependency files found (Cargo.lock, pyproject.toml, requirements.txt)"
                            .to_string(),
                    )
                }
            }
        }
    }
}

#[async_trait::async_trait]
impl IMaintenanceCheckerProtocol for MaintenanceChecker {
    async fn diagnose_toolchain(&self) -> ToolchainDiagnostics {
        MaintenanceChecker::diagnose_toolchain(self).await
    }

    async fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport {
        MaintenanceChecker::run_security_scan(self, project_path).await
    }

    async fn run_dependency_report(
        &self,
        project_path: &FilePath,
    ) -> Result<DependencyReport, String> {
        MaintenanceChecker::run_dependency_report(self, project_path).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[tokio::test]
    async fn test_diagnose_toolchain_returns_cargo() {
        let checker = MaintenanceChecker::new();
        let diag = checker.diagnose_toolchain().await;
        assert!(!diag.rust_tools.is_empty());
        let cargo = &diag.rust_tools[0];
        assert_eq!(cargo.name, "cargo");
        assert_eq!(cargo.status, "OK");
    }

    #[tokio::test]
    async fn test_diagnose_toolchain_has_git() {
        let checker = MaintenanceChecker::new();
        let diag = checker.diagnose_toolchain().await;
        let git = diag.vcs_tools.iter().find(|t| t.name == "git");
        assert!(git.is_some());
        assert_eq!(git.unwrap().status, "OK");
    }

    #[tokio::test]
    async fn test_diagnose_toolchain_returns_non_empty_sections() {
        let checker = MaintenanceChecker::new();
        let diag = checker.diagnose_toolchain().await;
        assert!(!diag.python_tools.is_empty());
        assert!(!diag.js_tools.is_empty());
    }

    #[tokio::test]
    async fn test_dependency_report_no_lockfile() {
        let checker = MaintenanceChecker::new();
        let path = FilePath::new("/nonexistent_path".to_string()).unwrap_or_default();
        let result = checker.run_dependency_report(&path).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_dependency_report_with_requirements_txt() {
        let dir = std::env::temp_dir().join("lint_arwaky_test_maint");
        let _ = fs::create_dir_all(&dir);
        let reqs = dir.join("requirements.txt");
        fs::write(&reqs, "requests==2.28.0\nflask>=2.0\n# comment\n").unwrap();

        let checker = MaintenanceChecker::new();
        let path = FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
        let result = checker.run_dependency_report(&path).await;
        assert!(result.is_ok());
        let report = result.unwrap();
        assert_eq!(report.language, "Python");
        assert_eq!(report.dependencies.len(), 2);
        assert_eq!(report.dependencies[0].name, "requests==2.28.0");

        let _ = fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn test_dependency_report_with_cargo_lock() {
        let dir = std::env::temp_dir().join("lint_arwaky_test_maint_cargo");
        let _ = fs::create_dir_all(&dir);
        let cargo_toml = dir.join("Cargo.toml");
        fs::write(
            &cargo_toml,
            "[dependencies]\nserde = \"1.0\"\ntokio = \"1.0\"\n",
        )
        .unwrap();
        let cargo_lock = dir.join("Cargo.lock");
        fs::write(
            &cargo_lock,
            "[[package]]\nname = \"serde\"\nversion = \"1.0.0\"\n\n[[package]]\nname = \"tokio\"\nversion = \"1.0.0\"\n\n[[package]]\nname = \"autocfg\"\nversion = \"1.0.0\"\n",
        )
        .unwrap();

        let checker = MaintenanceChecker::new();
        let path = FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
        let result = checker.run_dependency_report(&path).await;
        assert!(result.is_ok());
        let report = result.unwrap();
        assert_eq!(report.language, "Rust");
        assert_eq!(report.dependencies.len(), 3);

        let serde = report
            .dependencies
            .iter()
            .find(|d| d.name == "serde")
            .unwrap();
        assert_eq!(serde.dep_type, "direct");

        let autocfg = report
            .dependencies
            .iter()
            .find(|d| d.name == "autocfg")
            .unwrap();
        assert_eq!(autocfg.dep_type, "transitive");

        let _ = fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn test_security_scan_returns_report() {
        let checker = MaintenanceChecker::new();
        let path = FilePath::new("/nonexistent_path".to_string()).unwrap_or_default();
        let report = checker.run_security_scan(&path).await;
        assert_eq!(report.language, "Python");
        assert_eq!(report.tool_name, "bandit");
    }
}
