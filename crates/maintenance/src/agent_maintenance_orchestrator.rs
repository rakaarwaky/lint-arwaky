// PURPOSE: MaintenanceCommandsOrchestrator — implements MaintenanceCommandsAggregate for env diagnostics, stats, cleanup
//
// The maintenance crate provides background health operations:
//   - doctor: check environment for required tools (ruff, mypy, bandit, git, etc.)
//   - stats: count and ratio of Python files vs test files in a project
//   - clean: remove cache directories (.pytest_cache, __pycache__, etc.)
//   - update: upgrade external linter tools via pip
//   - security_scan: run dependency vulnerability scans
//   - dependency_report: analyze project dependencies
//   - diagnose_toolchain: check Rust/Python/Node.js toolchain versions
//
// This is the least "lint-like" crate — it handles ops, not code quality.
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::mcp_server::taxonomy_action_vo::JobId;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use shared::project_setup::taxonomy_doctor_vo::{
    DependencyReport, DoctorResultVO, SecurityScanReport, ToolchainDiagnostics,
};
use shared::project_setup::taxonomy_stats_vo::MaintenanceStatsVO;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::Count;
use shared::taxonomy_common_vo::Score;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_suggestion_vo::DescriptionVO;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// ─── Block 1: Struct Definition ───────────────────────────
pub struct MaintenanceCommandsOrchestrator {}

use async_trait::async_trait;

// ─── Block 2: Aggregate Trait Implementation ──────────────
#[async_trait]
impl MaintenanceCommandsAggregate for MaintenanceCommandsOrchestrator {
    /// Count Python files and test files in the project, compute test ratio.
    async fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO {
        let root = Path::new(&project_path.value);
        let mut py_files = Vec::new();
        walk_dir(root, &mut py_files);
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

    /// Delete cache dirs (.pytest_cache, .mypy_cache, .ruff_cache, __pycache__, .lint_arwaky_cache).
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
            find_cache_dirs(&cwd, &cache_dirs, &mut found_dirs);
            for entry in found_dirs {
                let _ = std::fs::remove_dir_all(&entry);
            }
        }
    }

    /// Upgrade Python linter tools via pip (ruff, mypy, bandit, radon).
    async fn update(&self) {
        let adapters = ["ruff", "mypy", "bandit", "radon"];
        for adapter in &adapters {
            let _ = std::process::Command::new("pip")
                .args(["install", "--upgrade", adapter])
                .output();
        }
    }

    /// Run health check: verify tool installations and config file presence.
    async fn doctor(&self) -> DoctorResultVO {
        let mut issues: Vec<ErrorMessage> = Vec::new();
        let mut adapter_statuses: HashMap<AdapterName, String> = HashMap::new();

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

    async fn cancel(&self, _job_id: JobId) {}

    async fn diagnose_toolchain(&self) -> ToolchainDiagnostics {
        let checker = crate::capabilities_maintenance_checker::MaintenanceChecker::new();
        checker.diagnose_toolchain().await
    }

    async fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport {
        let checker = crate::capabilities_maintenance_checker::MaintenanceChecker::new();
        checker.run_security_scan(project_path).await
    }

    async fn run_dependency_report(
        &self,
        project_path: &FilePath,
    ) -> Result<DependencyReport, String> {
        let checker = crate::capabilities_maintenance_checker::MaintenanceChecker::new();
        checker.run_dependency_report(project_path).await
    }
}

fn walk_dir(dir: &Path, py_files: &mut Vec<PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                if name != "target" && name != ".git" && name != "node_modules" && name != ".venv" {
                    walk_dir(&path, py_files);
                }
            } else if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("py") {
                py_files.push(path);
            }
        }
    }
}

fn find_cache_dirs(dir: &Path, cache_names: &[&str], found_dirs: &mut Vec<PathBuf>) {
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
                    find_cache_dirs(&path, cache_names, found_dirs);
                }
            }
        }
    }
}

fn walk_dir(dir: &Path, py_files: &mut Vec<PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                if name != "target" && name != ".git" && name != "node_modules" && name != ".venv" {
                    walk_dir(&path, py_files);
                }
            } else if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("py") {
                py_files.push(path);
            }
        }
    }
}

fn find_cache_dirs(dir: &Path, cache_names: &[&str], found_dirs: &mut Vec<PathBuf>) {
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
                    find_cache_dirs(&path, cache_names, found_dirs);
                }
            }
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl Default for MaintenanceCommandsOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl MaintenanceCommandsOrchestrator {
    pub fn new() -> Self {
        Self {}
    }
}
