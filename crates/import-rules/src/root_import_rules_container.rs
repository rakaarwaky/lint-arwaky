// PURPOSE: ImportContainer — wiring for import-rules feature (root layer, wiring only)
use crate::agent_import_orchestrator::{ImportOrchestrator, ImportOrchestratorDeps};
use shared::common::FilePath;
use shared::config_system::{ArchitectureConfig, IConfigOrchestratorAggregate};

use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::import_rules::IImportRunnerAggregate;
use std::sync::Arc;

pub struct ImportContainer {
    config: ArchitectureConfig,
    filesystem: Arc<dyn IFilesystemAggregate>,
}

impl ImportContainer {
    pub fn new_with_config(
        config: ArchitectureConfig,
        filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> Self {
        Self { config, filesystem }
    }

    /// Create from config orchestrator — the canonical way per AES architecture.
    pub fn from_orchestrator(
        orchestrator: &Arc<dyn IConfigOrchestratorAggregate>,
        project_root: &str,
        filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> Self {
        let fp = FilePath::new(project_root.to_string()).unwrap_or_default();
        let config = orchestrator.load_config_sync(&fp);
        Self::new_with_config(config, filesystem)
    }

    pub fn orchestrator(&self) -> Arc<dyn IImportRunnerAggregate> {
        let ignored_paths: Vec<String> = self
            .config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.clone())
            .collect();
        Arc::new(ImportOrchestrator::new(
            ImportOrchestratorDeps {
                mandatory: Arc::new(
                    crate::capabilities_import_mandatory_checker::ArchImportMandatoryChecker::new(),
                ),
                forbidden: Arc::new(
                    crate::capabilities_import_forbidden_checker::ArchImportForbiddenChecker::new(),
                ),
                unused: Arc::new(
                    crate::capabilities_import_unused_checker::UnusedImportRuleChecker::new(),
                ),
                cycle: Arc::new(
                    crate::capabilities_cycle_import_analyzer::DependencyCycleAnalyzer::new(),
                ),
                dummy: Arc::new(
                    crate::capabilities_dummy_import_checker::DummyImportChecker::new(),
                ),
                filesystem: self.filesystem.clone(),
            },
            self.config.clone(),
            ignored_paths,
        ))
    }
}
