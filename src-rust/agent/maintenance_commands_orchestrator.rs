// maintenance_commands_orchestrator — Orchestrator for maintenance-related domain logic.
use crate::contract::MaintenanceCommandsAggregate;
use crate::taxonomy::{DoctorResultVO, FilePath, JobId, MaintenanceStatsVO};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct MaintenanceCommandsOrchestrator;

impl MaintenanceCommandsAggregate for MaintenanceCommandsOrchestrator {}

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

    pub fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO {
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
            total_files: py_count,
            test_files: test_count,
            test_ratio: ratio,
            python_files: py_count,
        }
    }

    pub fn clean(&self) {
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

    pub fn update(&self) {
        let adapters = ["ruff", "mypy", "bandit", "radon"];
        for adapter in &adapters {
            let _ = std::process::Command::new("pip")
                .args(["install", "--upgrade", adapter])
                .output();
        }
    }

    pub fn doctor(&self) -> DoctorResultVO {
        let mut issues = Vec::new();
        let mut adapter_statuses = HashMap::new();

        let py_ver = "3.12".to_string();

        let is_installed = std::process::Command::new("pip")
            .args(["show", "lint-arwaky"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        let mut config_found = Vec::new();
        for cfg in &[
            ".lint_arwaky.json",
            "lint_arwaky.config.yaml",
            "pyproject.toml",
        ] {
            if std::path::Path::new(cfg).exists() {
                config_found.push(cfg.to_string());
            }
        }
        if config_found.is_empty() {
            issues.push("No configuration file found".to_string());
        }

        for adapter in &["ruff", "mypy", "bandit", "radon"] {
            let found = std::process::Command::new("which")
                .arg(adapter)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);
            adapter_statuses.insert(
                adapter.to_string(),
                if found {
                    "found".to_string()
                } else {
                    "MISSING".to_string()
                },
            );
            if !found {
                issues.push(format!("Linter adapter '{}' is not installed", adapter));
            }
        }

        let healthy = issues.is_empty();

        DoctorResultVO {
            python_version: py_ver,
            is_installed,
            config_found,
            adapter_statuses,
            issues,
            healthy,
        }
    }

    pub async fn cancel(&self, _job_id: JobId) {
        // Cancel a running lint job
    }
}
