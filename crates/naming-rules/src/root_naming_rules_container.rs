// PURPOSE: NamingContainer — wiring for naming-rules feature (root layer, wiring only)
use shared::naming_rules::contract_naming_analyzer_port::INamingAnalyzerPort;
use shared::naming_rules::contract_naming_checker_protocol::INamingCheckerProtocol;
use shared::naming_rules::contract_naming_filesystem_port::INamingFileSystemPort;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use std::sync::Arc;

pub struct NamingContainer {
    naming_convention_checker: Arc<dyn INamingCheckerProtocol>,
    suffix_prefix_checker: Arc<dyn INamingCheckerProtocol>,
    analyzer: Arc<dyn INamingAnalyzerPort>,
    fs: Arc<dyn INamingFileSystemPort>,
}

impl NamingContainer {
    pub fn new(analyzer: Arc<dyn INamingAnalyzerPort>) -> Self {
        let naming_convention_checker: Arc<dyn INamingCheckerProtocol> =
            Arc::new(crate::capabilities_naming_convention_checker::NamingConventionChecker::new());
        let suffix_prefix_checker: Arc<dyn INamingCheckerProtocol> =
            Arc::new(crate::capabilities_suffix_prefix_checker::SuffixPrefixChecker::new());
        let fs: Arc<dyn INamingFileSystemPort> =
            Arc::new(crate::infrastructure_filesystem_adapter::OSFileSystemAdapter::new());
        Self {
            naming_convention_checker,
            suffix_prefix_checker,
            analyzer,
            fs,
        }
    }

    pub fn naming_convention_checker(&self) -> &Arc<dyn INamingCheckerProtocol> {
        &self.naming_convention_checker
    }

    pub fn suffix_prefix_checker(&self) -> &Arc<dyn INamingCheckerProtocol> {
        &self.suffix_prefix_checker
    }

    pub fn analyzer(&self) -> Arc<dyn INamingAnalyzerPort> {
        self.analyzer.clone()
    }


    pub fn orchestrator(&self) -> Arc<dyn INamingRunnerAggregate> {
        Arc::new(crate::agent_naming_orchestrator::NamingOrchestrator::new(
            self.naming_convention_checker.clone(),
            self.suffix_prefix_checker.clone(),
            self.analyzer.clone(),
            self.fs.clone(),
        ))
    }
}
