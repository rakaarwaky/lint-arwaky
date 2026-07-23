// PURPOSE: ImportContainer — wiring for import-rules feature (root layer, wiring only)
use crate::agent_import_orchestrator::{ImportOrchestrator, ImportOrchestratorDeps};
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
        let config = orchestrator.load_config_sync(project_root);
        Self::new_with_config(config)
    }

    pub fn orchestrator(&self) -> Arc<dyn IImportRunnerAggregate> {
        Arc::new(ImportOrchestrator::new(ImportOrchestratorDeps {
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
            dummy: Arc::new(crate::capabilities_dummy_import_checker::DummyImportChecker::new()),
            config: self.config.clone(),
        }))
    }
}
