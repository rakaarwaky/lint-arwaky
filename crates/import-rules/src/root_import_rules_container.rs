// PURPOSE: ImportContainer — wiring for import-rules feature (root layer, wiring only)
use crate::agent_import_orchestrator::{ImportOrchestrator, ImportOrchestratorDeps};
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use std::sync::Arc;

pub struct ImportContainer {
    config: ArchitectureConfig,
}

impl ImportContainer {
    pub fn new_with_config(config: ArchitectureConfig) -> Self {
        Self { config }
    }

    /// Create from config orchestrator — the canonical way per AES architecture.
    pub fn from_orchestrator(
        orchestrator: &Arc<dyn IConfigOrchestratorAggregate>,
        project_root: &str,
    ) -> Self {
        let fp = FilePath::new(project_root.to_string()).unwrap_or_default();
        let config = orchestrator.load_config_sync(&fp);
        Self::new_with_config(config)
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
                    crate::capabilities_import_mandatory_checker::ArchImportMandatoryChecker::new(
                    ),
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
            },
            self.config.clone(),
            ignored_paths,
        ))
    }
}
