// PURPOSE: MaintenanceCommandsOrchestrator — implements MaintenanceCommandsAggregate for env diagnostics, stats, cleanup
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::mcp_server::taxonomy_action_vo::JobId;
use shared::project_setup::contract_filesystem_maintenance_port::IFileSystemMaintenancePort;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use shared::project_setup::contract_tool_executor_port::IToolExecutorPort;
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

// Block 1: struct Definition
pub struct MaintenanceCommandsOrchestrator {
    tool_executor: Arc<dyn IToolExecutorPort>,
    fs: Arc<dyn IFileSystemMaintenancePort>,
    checker:
        Arc<dyn shared::project_setup::contract_maintenance_protocol::IMaintenanceCheckerProtocol>,
}

// Block 3: constructors
impl MaintenanceCommandsOrchestrator {
    pub fn new(
        tool_executor: Arc<dyn IToolExecutorPort>,
        fs: Arc<dyn IFileSystemMaintenancePort>,
        checker: Arc<
            dyn shared::project_setup::contract_maintenance_protocol::IMaintenanceCheckerProtocol,
        >,
    ) -> Self {
        Self {
            tool_executor,
            fs,
            checker,
        }
    }
}

impl Default for MaintenanceCommandsOrchestrator {
    fn default() -> Self {
        use crate::infrastructure_filesystem_maintenance_adapter::FileSystemMaintenanceAdapter;
        use crate::infrastructure_tool_executor_adapter::ToolExecutorAdapter;
        let te: Arc<dyn IToolExecutorPort> = Arc::new(ToolExecutorAdapter::new());
        let fs: Arc<dyn IFileSystemMaintenancePort> = Arc::new(FileSystemMaintenanceAdapter::new());
        let checker: Arc<
            dyn shared::project_setup::contract_maintenance_protocol::IMaintenanceCheckerProtocol,
        > = Arc::new(
            crate::capabilities_maintenance_checker::MaintenanceChecker::new(
                te.clone(),
                fs.clone(),
            ),
        );
        Self::new(te, fs, checker)
    }
}

// Block 2: impl Trait for Struct (Public Contract)
#[async_trait]
impl MaintenanceCommandsAggregate for MaintenanceCommandsOrchestrator {
    async fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO {
        let root = &project_path.value;
        let py_files = self.fs.walk_py_files(root).await;
        let py_count = py_files.len() as i64;
        let test_count = py_files
            .iter()
            .filter(|f| {
                std::path::Path::new(f)
                    .file_name()
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
            let cwd_str = cwd.to_string_lossy().to_string();
            let found_dirs = self.fs.find_cache_dirs(&cwd_str, &cache_dirs).await;
            for entry in found_dirs {
                let _ = self.fs.remove_dir_all(&entry).await;
            }
        }
    }

    async fn update(&self) {
        let adapters = ["ruff", "mypy", "bandit", "radon"];
        for adapter in &adapters {
            let _ = self
                .tool_executor
                .run_tool("pip", &["install", "--upgrade", adapter])
                .await;
        }
    }

    async fn doctor(&self) -> DoctorResultVO {
        let mut issues: Vec<ErrorMessage> = Vec::new();
        let mut adapter_statuses: HashMap<AdapterName, String> = HashMap::new();

        let py_ver = DescriptionVO::new("3.12");

        let pip_output = self
            .tool_executor
            .run_tool("pip", &["show", "lint-arwaky"])
            .await;
        let is_installed = pip_output.success;

        let mut config_found_paths = Vec::new();
        for cfg in &[
            ".lint_arwaky.json",
            "lint_arwaky.config.yaml",
            "pyproject.toml",
        ] {
            if self.fs.file_exists(cfg).await {
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
            let found = self.tool_executor.tool_exists(adapter).await;
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
        self.checker.diagnose_toolchain().await
    }

    async fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport {
        self.checker.run_security_scan(project_path).await
    }

    async fn run_dependency_report(
        &self,
        project_path: &FilePath,
    ) -> Result<DependencyReport, String> {
        self.checker.run_dependency_report(project_path).await
    }
}
