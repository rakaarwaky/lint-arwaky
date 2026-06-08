// maintenance_commands_orchestrator — Orchestrator for maintenance-related domain logic.
use crate::contract::maintenance_commands_aggregate::MaintenanceCommandsAggregate;
use crate::taxonomy::{
    AdapterName, ComplianceStatus, Count, DescriptionVO, DoctorResultVO, ErrorMessage, FilePath,
    FilePathList, JobId, MaintenanceStatsVO, Score,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct MaintenanceCommandsOrchestrator;

use async_trait::async_trait;

#[async_trait]
impl MaintenanceCommandsAggregate for MaintenanceCommandsOrchestrator {
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
                    .unwrap_or(false)
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
            find_cache_dirs(&cwd, &cache_dirs, &mut found_dirs);
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
        let mut adapter_statuses: HashMap<AdapterName, String> = HashMap::new();

        let py_ver = DescriptionVO::new("3.12");

        let is_installed = std::process::Command::new("pip")
            .args(["show", "lint-arwaky"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

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
            let found = std::process::Command::new("which")
                .arg(adapter)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);
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
}

fn walk_dir(dir: &Path, py_files: &mut Vec<PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
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
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if cache_names.contains(&name) {
                    found_dirs.push(path.clone());
                } else if name != "target" && name != ".git" && name != "node_modules" {
                    find_cache_dirs(&path, cache_names, found_dirs);
                }
            }
        }
    }
}

impl Default for MaintenanceCommandsOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl MaintenanceCommandsOrchestrator {
    pub fn new() -> Self {
        Self
    }
}
