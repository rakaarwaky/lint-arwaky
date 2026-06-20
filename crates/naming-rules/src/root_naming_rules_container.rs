// PURPOSE: NamingContainer — wiring for naming-rules feature (root layer, wiring only)
use shared::file_system::contract_system_port::IFileSystemPort;
use shared::import_rules::contract_rule_protocol::{IAnalyzer, INamingCheckerProtocol};
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use std::sync::Arc;

pub struct NamingContainer {
    checker: Arc<dyn INamingCheckerProtocol>,
    analyzer: Arc<dyn IAnalyzer>,
    fs: Arc<dyn IFileSystemPort>,
}

impl NamingContainer {
    pub fn new() -> Self {
        Self::new_with_config(shared::config_system::taxonomy_config_vo::default_aes_config())
    }

    pub fn new_with_config(config: shared::config_system::taxonomy_config_vo::ArchitectureConfig) -> Self {
        let fs: Arc<dyn IFileSystemPort> =
            Arc::new(file_system::infrastructure_filesystem_adapter::OSFileSystemAdapter::new());
        let source_parser = Arc::new(
            source_parsing::infrastructure_parser_adapter::SourceParserOrchestrator::new(
                Box::new(source_parsing::infrastructure_py_scanner::ASTPythonParserAdapter::new()),
                Box::new(source_parsing::infrastructure_rust_scanner::ASTRustParserAdapter::new()),
                Box::new(source_parsing::infrastructure_js_scanner::ASTJSParserAdapter::new()),
            ),
        );
        let analyzer = Arc::new(
            import_rules::capabilities_layer_detection_analyzer::LayerDetectionAnalyzer::new(
                config,
                fs.clone(),
                source_parser,
            ),
        );
        let checker: Arc<dyn INamingCheckerProtocol> =
            Arc::new(crate::capabilities_naming_checker::ArchNamingChecker::new());
        Self {
            checker,
            analyzer,
            fs,
        }
    }

    pub fn checker(&self) -> &Arc<dyn INamingCheckerProtocol> {
        &self.checker
    }

    pub fn analyzer(&self) -> Arc<dyn IAnalyzer> {
        self.analyzer.clone()
    }

    pub fn orchestrator(&self) -> Arc<dyn INamingRunnerAggregate> {
        Arc::new(crate::agent_naming_orchestrator::NamingOrchestrator::new(
            self.checker.clone(),
            self.analyzer.clone(),
            self.fs.clone(),
        ))
    }
}

impl Default for NamingContainer {
    fn default() -> Self {
        Self::new()
    }
}
