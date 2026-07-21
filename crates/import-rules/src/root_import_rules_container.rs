// PURPOSE: ImportContainer — wiring for import-rules feature (root layer, wiring only)
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::config_system::utility_config_defaults::default_aes_config;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use std::sync::Arc;

pub struct ImportContainer {
    config: ArchitectureConfig,
}

impl ImportContainer {
    pub fn new_with_config(config: ArchitectureConfig) -> Self {
        Self { config }
    }

    pub fn new_default() -> Self {
        Self::new_with_config(default_aes_config())
    }

    pub fn mandatory(&self) -> Arc<dyn IImportMandatoryProtocol> {
        Arc::new(crate::capabilities_import_mandatory_checker::ArchImportMandatoryChecker::new())
    }

    pub fn forbidden(&self) -> Arc<dyn IImportForbiddenProtocol> {
        Arc::new(crate::capabilities_import_forbidden_checker::ArchImportForbiddenChecker::new())
    }

    pub fn dummy(&self) -> Arc<dyn IDummyImportCheckerProtocol> {
        Arc::new(crate::capabilities_dummy_import_checker::DummyImportChecker::new())
    }

    pub fn unused(&self) -> Arc<dyn IUnusedImportProtocol> {
        Arc::new(crate::capabilities_import_unused_checker::UnusedImportRuleChecker::new())
    }

    pub fn cycle(&self) -> Arc<dyn ICycleImportProtocol> {
        Arc::new(crate::capabilities_cycle_import_analyzer::DependencyCycleAnalyzer::new())
    }

    pub fn config(&self) -> &ArchitectureConfig {
        &self.config
    }

    pub fn orchestrator(&self) -> Arc<dyn IImportRunnerAggregate> {
        Arc::new(crate::agent_import_orchestrator::ImportOrchestrator::new(
            self.mandatory(),
            self.forbidden(),
            self.unused(),
            self.cycle(),
            self.dummy(),
            self.config.clone(),
        ))
    }
}
