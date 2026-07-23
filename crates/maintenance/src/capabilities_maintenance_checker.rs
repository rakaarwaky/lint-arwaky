use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_error::ErrorMessage;
use shared::common::taxonomy_common_vo::{Count, Score};
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_suggestion_vo::DescriptionVO;

// PURPOSE: MaintenanceChecker — business logic capabilities for running audits and checking toolchains
//
// Implements IMaintenanceCheckerProtocol with health-check operations:
//
//   1. diagnose_toolchain: checks for installation of Rust, Python, JS, VCS tools.
//   2. run_security_scan: runs cargo-audit or bandit depending on project type.
//   3. run_dependency_report: parses lock files to list dependencies.
//   4. stats: count Python files and test files, compute ratio.
//   5. clean: remove cache directories.
//   6. update: upgrade pip packages.
//   7. doctor: check tool installations and config presence.

use shared::common::utility_command_runner as proc_io;
use shared::maintenance::utility_dependency_io as dep_io;
use shared::maintenance::contract_maintenance_protocol::IMaintenanceCheckerProtocol;
use shared::maintenance::taxonomy_doctor_vo::{
    DependencyInfo, DependencyReport, DoctorResultVO, SecurityFinding, SecurityScanReport,
    ToolStatus, ToolchainDiagnostics,
};
use shared::maintenance::taxonomy_stats_vo::MaintenanceStatsVO;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct MaintenanceChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait::async_trait]
impl IMaintenanceCheckerProtocol for MaintenanceChecker {
    async fn diagnose_toolchain(&self) -> ToolchainDiagnostics {
        let check_tool = &|name: &str, args: &[&str], required: bool| {
            let (stdout, _, success) = proc_io::run_command(name, args);
            let (status, version) = if success {
                let ver = match stdout.lines().next() {
                    Some(v) => v.trim().to_string(),
                    None => String::new(),
                };
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
        let eslint_local = "node_modules/.bin/eslint";
        let eslint_status = if shared::common::utility_file_handler::is_file(eslint_local) {
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

        let prettier_local = "node_modules/.bin/prettier";
        let prettier_status = if shared::common::utility_file_handler::is_file(prettier_local) {
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

        let tsc_local = "node_modules/.bin/tsc";
        let tsc_status = if shared::common::utility_file_handler::is_file(tsc_local) {
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

    async fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport {
        let root = &project_path.value;
        let cargo_lock = std::path::Path::new(root).join("Cargo.lock");
        if cargo_lock.exists() {
            let (s, _, _) = dep_io::run_external_command_in("cargo", &["audit", "--json"], root);
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
        } else {
            let (s, _, _) =
                dep_io::run_external_command_in("bandit", &["-r", "--format", "json", root], root);
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
                        let severity = match r.get("issue_severity").and_then(|s| s.as_str()) {
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
    }

    async fn run_dependency_report(
        &self,
        project_path: &FilePath,
    ) -> Result<DependencyReport, String> {
        let root = &project_path.value;
        let cargo_lock = std::path::Path::new(root).join("Cargo.lock");
        if cargo_lock.exists() {
            let content = dep_io::read_dependency_file(&cargo_lock).map_err(|e| e.to_string())?;
            let mut in_package = false;
            let mut pkg_name = String::new();
            let mut pkg_version = String::new();
            let mut dependencies = Vec::new();

            let cargo_toml = std::path::Path::new(root).join("Cargo.toml");
            let mut direct_deps = std::collections::HashSet::new();
            if let Ok(toml_content) = dep_io::read_dependency_file(&cargo_toml) {
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

            Ok(DependencyReport {
                language: "Rust".to_string(),
                dependencies,
            })
        } else {
            let pyproject = std::path::Path::new(root).join("pyproject.toml");
            if pyproject.exists() {
                let content =
                    dep_io::read_dependency_file(&pyproject).map_err(|e| e.to_string())?;
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
                    let content = dep_io::read_dependency_file(&reqs).map_err(|e| e.to_string())?;
                    let mut dependencies = Vec::new();
                    for line in content.lines() {
                        let t = line.trim();
                        if !t.is_empty() && !t.starts_with('#') {
                            let parts: Vec<&str> = t.splitn(2, ['=', '>', '<', '~']).collect();
                            let name = parts[0].trim().to_string();
                            let version = if parts.len() > 1 {
                                parts[1].trim_start_matches('=').trim().to_string()
                            } else {
                                String::new()
                            };
                            dependencies.push(DependencyInfo {
                                name,
                                version,
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

    async fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO {
        let root = std::path::Path::new(&project_path.value);
        let mut py_files = Vec::new();
        Self::walk_dir(root, &mut py_files);
        let py_count = py_files.len() as i64;
        let test_count = py_files
            .iter()
            .filter(|f| {
                f.file_name()
                    .map(|n| n.to_string_lossy().starts_with("test_"))
                    .unwrap_or_default()
            })
            .count() as i64;
        let ratio = if py_count > 0 {
            test_count as f64 / py_count as f64
        } else {
            0.0
        };

        MaintenanceStatsVO {
            project_path: project_path.clone(),
            total_files: Count::new(py_count),
            test_files: Count::new(test_count),
            test_ratio: Score::new(ratio),
            python_files: Count::new(py_count),
        }
    }

    async fn clean(&self) {
        let cwd = std::env::current_dir().ok();
        if let Some(cwd) = cwd {
            let cache_dirs = [
                ".pytest_cache",
                ".mypy_cache",
                ".ruff_cache",
                "__pycache__",
                ".lint_arwaky_cache",
            ];
            let mut found_dirs = Vec::new();
            Self::find_cache_dirs(&cwd, &cache_dirs, &mut found_dirs);
            for entry in found_dirs {
                let _ = std::fs::remove_dir_all(&entry);
            }
        }
    }

    async fn update(&self) {
        let adapters = ["ruff", "mypy", "bandit", "radon"];
        for adapter in &adapters {
            let _ = std::process::Command::new("pip")
                .args(["install", "--upgrade", adapter])
                .output();
        }
    }

    async fn doctor(&self) -> DoctorResultVO {
        let mut issues: Vec<ErrorMessage> = Vec::new();
        let mut adapter_statuses: std::collections::HashMap<AdapterName, String> =
            std::collections::HashMap::new();

        let py_ver = DescriptionVO::new("3.12");

        let is_installed = match std::process::Command::new("pip")
            .args(["show", "lint-arwaky"])
            .output()
        {
            Ok(o) => o.status.success(),
            Err(_) => false,
        };

        let mut config_found_paths = Vec::new();
        for cfg in &[
            ".lint_arwaky.json",
            "lint_arwaky.config.yaml",
            "pyproject.toml",
        ] {
            if std::path::Path::new(cfg).exists() {
                if let Ok(fp) = FilePath::new(cfg.to_string()) {
                    config_found_paths.push(fp);
                }
            }
        }
        let config_found = FilePathList::new(config_found_paths);
        if config_found.is_empty() {
            issues.push(ErrorMessage::new("No configuration file found"));
        }

        for adapter in &["ruff", "mypy", "bandit", "radon"] {
            let found = match std::process::Command::new("which").arg(adapter).output() {
                Ok(o) => o.status.success(),
                Err(_) => false,
            };
            if let Ok(name) = AdapterName::new(adapter.to_string()) {
                adapter_statuses.insert(
                    name.clone(),
                    if found {
                        "found".to_string()
                    } else {
                        "MISSING".to_string()
                    },
                );
                if !found {
                    issues.push(ErrorMessage::new(format!(
                        "Linter adapter '{}' is not installed",
                        adapter
                    )));
                }
            }
        }

        let healthy = ComplianceStatus::new(issues.is_empty());

        DoctorResultVO {
            python_version: py_ver,
            is_installed: ComplianceStatus::new(is_installed),
            config_found,
            adapter_statuses,
            issues,
            healthy,
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl MaintenanceChecker {
    pub fn new() -> Self {
        Self
    }

    fn walk_dir(dir: &std::path::Path, py_files: &mut Vec<std::path::PathBuf>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or_default();
                    if name != "target"
                        && name != ".git"
                        && name != "node_modules"
                        && name != ".venv"
                    {
                        Self::walk_dir(&path, py_files);
                    }
                } else if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("py")
                {
                    py_files.push(path);
                }
            }
        }
    }

    fn find_cache_dirs(
        dir: &std::path::Path,
        cache_names: &[&str],
        found_dirs: &mut Vec<std::path::PathBuf>,
    ) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or_default();
                    if cache_names.contains(&name) {
                        found_dirs.push(path.clone());
                    } else if name != "target" && name != ".git" && name != "node_modules" {
                        Self::find_cache_dirs(&path, cache_names, found_dirs);
                    }
                }
            }
        }
    }
}

impl Default for MaintenanceChecker {
    fn default() -> Self {
        Self::new()
    }
}
