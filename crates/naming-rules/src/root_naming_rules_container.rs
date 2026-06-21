// PURPOSE: NamingContainer — wiring for naming-rules feature (root layer, wiring only)
use shared::file_system::contract_system_port::IFileSystemPort;
use shared::import_rules::contract_rule_protocol::{IAnalyzer, INamingCheckerProtocol};
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use std::sync::Arc;

pub struct NamingContainer {
    naming_convention_checker: Arc<dyn INamingCheckerProtocol>,
    suffix_prefix_checker: Arc<dyn INamingCheckerProtocol>,
    analyzer: Arc<dyn IAnalyzer>,
    fs: Arc<dyn IFileSystemPort>,
}

impl NamingContainer {
    pub fn new() -> Self {
        Self::new_with_config(shared::config_system::taxonomy_config_vo::default_aes_config())
    }

    pub fn new_with_config(
        config: shared::config_system::taxonomy_config_vo::ArchitectureConfig,
    ) -> Self {
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
        let naming_convention_checker: Arc<dyn INamingCheckerProtocol> =
            Arc::new(crate::capabilities_naming_convention_checker::NamingConventionChecker::new());
        let suffix_prefix_checker: Arc<dyn INamingCheckerProtocol> =
            Arc::new(crate::capabilities_suffix_prefix_checker::SuffixPrefixChecker::new());
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

    pub fn analyzer(&self) -> Arc<dyn IAnalyzer> {
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

impl Default for NamingContainer {
    fn default() -> Self {
        Self::new()
    }
}
