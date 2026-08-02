use shared::common::{AdapterName, ErrorMessage};

use shared::common::{ComplianceStatus, DescriptionVO, FilePath, FilePathList};
use shared::common::{Count, Score};
// PURPOSE: MaintenanceChecker — business logic capabilities for running audits and checking toolchains
//
// Implements IMaintenanceCheckerProtocol with health-check operations:
//
//   1. doctor: check tool installations, config presence, language versions.
//   2. diagnose_toolchain: checks for installation of Rust, Python, JS, VCS tools.
//   3. run_security_scan: runs cargo-audit, bandit, or npm audit depending on project type.
//   4. run_dependency_report: parses lock files to list dependencies.
//   5. stats: count source files and test files across all languages, compute ratio.
//   6. clean: remove cache directories.
//   7. update: upgrade linter tools via pip/npm.

use shared::maintenance::IMaintenanceCheckerProtocol;
use shared::maintenance::MaintenanceStatsVO;
use shared::maintenance::{
    DependencyInfo, DependencyReport, DoctorResultVO, SecurityFinding, SecurityScanReport,
    ToolStatus, ToolchainDiagnostics,
};

use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct MaintenanceChecker {
    filesystem: Arc<dyn IFilesystemAggregate>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait::async_trait]
impl IMaintenanceCheckerProtocol for MaintenanceChecker {
    async fn diagnose_toolchain(&self) -> ToolchainDiagnostics {
        let check_tool = &|name: &str, args: &[&str], required: bool| {
            let (stdout, _, success) = self.filesystem.run_external_command_in(name, args, ".");
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

        // FR-005: Rust tools — all required
        let mut rust_tools = vec![check_tool("rustc", &["--version"], true)];
        rust_tools.push(check_tool("cargo", &["--version"], true));
        let mut clippy_status = check_tool("cargo", &["clippy", "--version"], true);
        clippy_status.name = "clippy".to_string();
        rust_tools.push(clippy_status);
        rust_tools.push(check_tool("rustfmt", &["--version"], true));

        // FR-005: Python tools — all optional
        let python_tools = vec![
            check_tool("python3", &["--version"], false),
            check_tool("ruff", &["--version"], false),
            check_tool("mypy", &["--version"], false),
            check_tool("bandit", &["--version"], false),
            check_tool("radon", &["--version"], false),
        ];

        // FR-005: JS tools — all optional; local node_modules/.bin/ preferred
        let mut js_tools = vec![check_tool("node", &["--version"], false)];
        let eslint_local = "node_modules/.bin/eslint";
        let eslint_status = if self.filesystem.is_file(std::path::Path::new(eslint_local)) {
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
        let prettier_status = if self
            .filesystem
            .is_file(std::path::Path::new(prettier_local))
        {
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
        let tsc_status = if self.filesystem.is_file(std::path::Path::new(tsc_local)) {
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

        // FR-005: VCS tools — git required
        let vcs_tools = vec![check_tool("git", &["--version"], true)];

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
        let package_json = std::path::Path::new(root).join("package.json");

        // FR-006: Language detection — Cargo.lock → Rust, package.json → JS/TS, else Python
        if cargo_lock.exists() {
            // FR-006: Rust — cargo audit
            let tool_available = self
                .filesystem
                .run_external_command_in("cargo", &["audit", "--version"], ".")
                .2;
            if !tool_available {
                return SecurityScanReport {
                    language: "Rust".to_string(),
                    tool_name: "cargo-audit".to_string(),
                    findings: Vec::new(),
                    tool_installed: false,
                };
            }
            let (s, _, _) =
                self.filesystem
                    .run_external_command_in("cargo", &["audit", "--json"], root);
            let mut findings = Vec::new();
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&s)
                && let Some(list) = json
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
            SecurityScanReport {
                language: "Rust".to_string(),
                tool_name: "cargo-audit".to_string(),
                findings,
                tool_installed: true,
            }
        } else if package_json.exists() {
            // FR-006: JS/TS — npm audit
            let tool_available = self
                .filesystem
                .run_external_command_in("npm", &["--version"], ".")
                .2;
            if !tool_available {
                return SecurityScanReport {
                    language: "JavaScript".to_string(),
                    tool_name: "npm-audit".to_string(),
                    findings: Vec::new(),
                    tool_installed: false,
                };
            }
            let (s, _, _) =
                self.filesystem
                    .run_external_command_in("npm", &["audit", "--json"], root);
            let mut findings = Vec::new();
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&s)
                && let Some(vulns) = json.get("vulnerabilities")
                && let Some(obj) = vulns.as_object()
            {
                for (name, detail) in obj {
                    let severity = detail
                        .get("severity")
                        .and_then(|s| s.as_str())
                        .unwrap_or("unknown")
                        .to_string();
                    let via = detail.get("via").and_then(|v| v.as_array());
                    let issue = match via {
                        Some(arr) if !arr.is_empty() => {
                            if let Some(v) = arr.first() {
                                if let Some(title) = v.get("title").and_then(|t| t.as_str()) {
                                    title.to_string()
                                } else {
                                    "Advisory vulnerability".to_string()
                                }
                            } else {
                                "Advisory vulnerability".to_string()
                            }
                        }
                        _ => "Transitive vulnerability".to_string(),
                    };
                    findings.push(SecurityFinding {
                        severity,
                        test_id: "npm-advisory".to_string(),
                        file: name.clone(),
                        line: 0,
                        issue,
                    });
                }
            }
            SecurityScanReport {
                language: "JavaScript".to_string(),
                tool_name: "npm-audit".to_string(),
                findings,
                tool_installed: true,
            }
        } else {
            // FR-006: Python — bandit
            let tool_available = self
                .filesystem
                .run_external_command_in("bandit", &["--version"], ".")
                .2;
            if !tool_available {
                return SecurityScanReport {
                    language: "Python".to_string(),
                    tool_name: "bandit".to_string(),
                    findings: Vec::new(),
                    tool_installed: false,
                };
            }
            let (s, _, _) = self.filesystem.run_external_command_in(
                "bandit",
                &["-r", "--format", "json", root],
                root,
            );
            let mut findings = Vec::new();
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&s)
                && let Some(results) = json.get("results").and_then(|r| r.as_array())
            {
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
        let package_json = std::path::Path::new(root).join("package.json");

        if cargo_lock.exists() {
            // FR-007: Rust — Cargo.lock + Cargo.toml
            let content = self
                .filesystem
                .read_to_string(&cargo_lock)
                .map_err(|e| e.to_string())?;
            let mut in_package = false;
            let mut pkg_name = String::new();
            let mut pkg_version = String::new();
            let mut dependencies = Vec::new();

            let cargo_toml = std::path::Path::new(root).join("Cargo.toml");
            let mut direct_deps = std::collections::HashSet::new();
            if let Ok(toml_content) = self.filesystem.read_to_string(&cargo_toml) {
                let mut in_deps = false;
                for line in toml_content.lines() {
                    if line.trim().starts_with("[dependencies]") {
                        in_deps = true;
                        continue;
                    }
                    if in_deps && line.trim().starts_with('[') {
                        in_deps = false;
                    }
                    if in_deps && let Some(eq) = line.find('=') {
                        let name = line[..eq].trim().to_string();
                        if !name.is_empty() && !name.starts_with('#') {
                            direct_deps.insert(name);
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
        } else if package_json.exists() {
            // FR-007: JS/TS — package.json (dependencies + devDependencies)
            let content = self
                .filesystem
                .read_to_string(&package_json)
                .map_err(|e| e.to_string())?;
            let json: serde_json::Value =
                serde_json::from_str(&content).map_err(|e| e.to_string())?;
            let mut dependencies = Vec::new();

            // Direct dependencies
            if let Some(deps) = json.get("dependencies").and_then(|d| d.as_object()) {
                for (name, ver) in deps {
                    let version = ver.as_str().unwrap_or("").to_string();
                    dependencies.push(DependencyInfo {
                        name: name.clone(),
                        version,
                        dep_type: "direct".to_string(),
                    });
                }
            }
            // Dev dependencies
            if let Some(dev_deps) = json.get("devDependencies").and_then(|d| d.as_object()) {
                for (name, ver) in dev_deps {
                    let version = ver.as_str().unwrap_or("").to_string();
                    dependencies.push(DependencyInfo {
                        name: name.clone(),
                        version,
                        dep_type: "direct".to_string(),
                    });
                }
            }

            Ok(DependencyReport {
                language: "JavaScript".to_string(),
                dependencies,
            })
        } else {
            let pyproject = std::path::Path::new(root).join("pyproject.toml");
            if pyproject.exists() {
                // FR-007: Python — pyproject.toml
                let content = self
                    .filesystem
                    .read_to_string(&pyproject)
                    .map_err(|e| e.to_string())?;
                let mut dependencies = Vec::new();
                for line in content.lines() {
                    let t = line.trim();
                    if t.contains('=')
                        && !t.starts_with('[')
                        && !t.starts_with('#')
                        && let Some(eq) = t.find('=')
                    {
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
                Ok(DependencyReport {
                    language: "Python".to_string(),
                    dependencies,
                })
            } else {
                let reqs = std::path::Path::new(root).join("requirements.txt");
                if reqs.exists() {
                    // FR-007: Python — requirements.txt (fallback)
                    let content = self
                        .filesystem
                        .read_to_string(&reqs)
                        .map_err(|e| e.to_string())?;
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
                        "No dependency files found (Cargo.lock, package.json, pyproject.toml, requirements.txt)"
                            .to_string(),
                    )
                }
            }
        }
    }

    async fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO {
        let root = std::path::Path::new(&project_path.value);
        let mut all_files = Vec::new();
        let mut py_files = Vec::new();
        let mut rs_files = Vec::new();
        let mut js_files = Vec::new();
        self.walk_dir(
            root,
            &mut all_files,
            &mut py_files,
            &mut rs_files,
            &mut js_files,
        );

        let total_count = all_files.len() as i64;
        let py_count = py_files.len() as i64;
        let rs_count = rs_files.len() as i64;
        let js_count = js_files.len() as i64;

        // FR-002: Test file detection per language
        let test_count = {
            let mut count = 0i64;
            // Rust tests: *_test.rs, test_*.rs, files inside tests/
            for f in &rs_files {
                let name = f.file_name().and_then(|n| n.to_str()).unwrap_or("");
                let parent = f
                    .parent()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                if name.starts_with("test_") || name.ends_with("_test.rs") || parent == "tests" {
                    count += 1;
                }
            }
            // Python tests: test_*.py, *_test.py, files inside tests/
            for f in &py_files {
                let name = f.file_name().and_then(|n| n.to_str()).unwrap_or("");
                let parent = f
                    .parent()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                if name.starts_with("test_") || name.ends_with("_test.py") || parent == "tests" {
                    count += 1;
                }
            }
            // JS/TS tests: *.test.*, *.spec.*, files inside tests/ or __tests__/
            for f in &js_files {
                let name = f.file_name().and_then(|n| n.to_str()).unwrap_or("");
                let parent = f
                    .parent()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                if name.contains(".test.")
                    || name.contains(".spec.")
                    || parent == "tests"
                    || parent == "__tests__"
                {
                    count += 1;
                }
            }
            count
        };

        let ratio = if total_count > 0 {
            test_count as f64 / total_count as f64
        } else {
            0.0
        };

        MaintenanceStatsVO {
            project_path: project_path.clone(),
            total_files: Count::new(total_count),
            test_files: Count::new(test_count),
            test_ratio: Score::new(ratio),
            python_files: Count::new(py_count),
            rust_files: Count::new(rs_count),
            js_files: Count::new(js_count),
        }
    }

    async fn clean(&self) {
        let cwd = std::env::current_dir().ok();
        if let Some(cwd) = cwd {
            // FR-003: Cache targets including .eslintcache, .tsc-cache
            let cache_dirs = [
                ".pytest_cache",
                ".mypy_cache",
                ".ruff_cache",
                "__pycache__",
                ".lint_arwaky_cache",
                ".eslintcache",
                ".tsc-cache",
            ];
            let mut found_dirs = Vec::new();
            self.find_cache_dirs(&cwd, &cache_dirs, &mut found_dirs);
            for entry in found_dirs {
                let _ = self
                    .filesystem
                    .remove_dir_all(&std::path::Path::new(&entry));
            }
        }
    }

    async fn update(&self) {
        // FR-004: Python tools — pip upgrade
        let python_tools = ["ruff", "mypy", "bandit"];
        for tool in &python_tools {
            let _ = std::process::Command::new("pip")
                .args(["install", "--upgrade", tool])
                .output();
        }
        // FR-004: JS/TS tools — npm global upgrade
        let js_tools = ["eslint", "prettier", "typescript"];
        for tool in &js_tools {
            let _ = std::process::Command::new("npm")
                .args(["install", "-g", tool])
                .output();
        }
        // FR-004: Rust tools — print suggestion
        eprintln!("Run `rustup update` to update Rust tools");
    }

    async fn doctor(&self) -> DoctorResultVO {
        let mut issues: Vec<ErrorMessage> = Vec::new();
        let mut adapter_statuses: std::collections::HashMap<AdapterName, String> =
            std::collections::HashMap::new();

        // FR-001: Language runtime versions
        let py_ver = {
            let (stdout, _, success) =
                self.filesystem
                    .run_external_command_in("python3", &["--version"], ".");
            if success {
                stdout
                    .lines()
                    .next()
                    .unwrap_or("unknown")
                    .trim()
                    .to_string()
            } else {
                "not installed".to_string()
            }
        };
        let rust_ver = {
            let (stdout, _, success) =
                self.filesystem
                    .run_external_command_in("rustc", &["--version"], ".");
            if success {
                stdout
                    .lines()
                    .next()
                    .unwrap_or("unknown")
                    .trim()
                    .to_string()
            } else {
                "not installed".to_string()
            }
        };
        let node_ver = {
            let (stdout, _, success) =
                self.filesystem
                    .run_external_command_in("node", &["--version"], ".");
            if success {
                stdout
                    .lines()
                    .next()
                    .unwrap_or("unknown")
                    .trim()
                    .to_string()
            } else {
                "not installed".to_string()
            }
        };

        let is_installed = match std::process::Command::new("pip")
            .args(["show", "lint-arwaky"])
            .output()
        {
            Ok(o) => o.status.success(),
            Err(_) => false,
        };

        // FR-001: Config files — 7 per FRD
        let mut config_found_paths = Vec::new();
        for cfg in &[
            "lint_arwaky.config.rust.yaml",
            "lint_arwaky.config.python.yaml",
            "lint_arwaky.config.typescript.yaml",
            "lint_arwaky.config.javascript.yaml",
            "pyproject.toml",
            "Cargo.toml",
            "package.json",
        ] {
            if std::path::Path::new(cfg).exists()
                && let Ok(fp) = FilePath::new(cfg.to_string())
            {
                config_found_paths.push(fp);
            }
        }
        let config_found = FilePathList::new(config_found_paths);
        if config_found.is_empty() {
            issues.push(ErrorMessage::new("No configuration file found"));
        }

        // FR-001: 9 adapters
        for adapter in &[
            "clippy",
            "rustfmt",
            "cargo-audit",
            "ruff",
            "mypy",
            "bandit",
            "eslint",
            "prettier",
            "tsc",
        ] {
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
            python_version: DescriptionVO::new(&py_ver),
            rust_version: DescriptionVO::new(&rust_ver),
            node_version: DescriptionVO::new(&node_ver),
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
        Self {
            filesystem: Arc::new(filesystem::FilesystemOrchestrator::new()),
        }
    }

    /// Walk directory tree, collecting all source files and per-language files.
    /// FR-002: Excludes target/, .git/, node_modules/, .venv/, __pycache__/, dist/, build/
    fn walk_dir(
        &self,
        dir: &std::path::Path,
        all_files: &mut Vec<std::path::PathBuf>,
        py_files: &mut Vec<std::path::PathBuf>,
        rs_files: &mut Vec<std::path::PathBuf>,
        js_files: &mut Vec<std::path::PathBuf>,
    ) {
        let dir_entries = self.filesystem.scan_directory(dir);
        for path in dir_entries {
            if path.is_dir() {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                if name != "target"
                    && name != ".git"
                    && name != "node_modules"
                    && name != ".venv"
                    && name != "__pycache__"
                    && name != "dist"
                    && name != "build"
                {
                    self.walk_dir(&path, all_files, py_files, rs_files, js_files);
                }
            } else if path.is_file() {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                match ext {
                    "py" => {
                        all_files.push(path.clone());
                        py_files.push(path);
                    }
                    "rs" => {
                        all_files.push(path.clone());
                        rs_files.push(path);
                    }
                    "js" | "jsx" | "ts" | "tsx" => {
                        all_files.push(path.clone());
                        js_files.push(path);
                    }
                    _ => {}
                }
            }
        }
    }

    fn find_cache_dirs(
        &self,
        dir: &std::path::Path,
        cache_names: &[&str],
        found_dirs: &mut Vec<std::path::PathBuf>,
    ) {
        let dir_entries = self.filesystem.scan_directory(dir);
        for path in dir_entries {
            if path.is_dir() {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                if cache_names.contains(&name) {
                    found_dirs.push(path.clone());
                } else if name != "target" && name != ".git" && name != "node_modules" {
                    self.find_cache_dirs(&path, cache_names, found_dirs);
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
