// PURPOSE: NamingContainer — wiring for naming-rules feature (root layer, wiring only)
use crate::import_rules::contract_rule_protocol::IAnalyzer;
use crate::naming_rules::capabilities_naming_checker::ArchNamingChecker;
use crate::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use std::sync::Arc;

pub struct NamingContainer {
    checker: ArchNamingChecker,
    analyzer: Arc<dyn IAnalyzer>,
}

impl NamingContainer {
    pub fn new() -> Self {
        let config = crate::config_system::taxonomy_config_vo::default_aes_config();
        let fs = Arc::new(
            crate::file_system::infrastructure_filesystem_adapter::OSFileSystemAdapter::new(),
        );
        let source_parser = Arc::new(
            crate::source_parsing::infrastructure_parser_adapter::SourceParserOrchestrator::new(
                Box::new(
                    crate::source_parsing::infrastructure_py_scanner::ASTPythonParserAdapter::new(),
                ),
                Box::new(
                    crate::source_parsing::infrastructure_rust_scanner::ASTRustParserAdapter::new(),
                ),
                Box::new(
                    crate::source_parsing::infrastructure_js_scanner::ASTJSParserAdapter::new(),
                ),
            ),
        );
        let analyzer = Arc::new(
            crate::import_rules::capabilities_layer_detection_analyzer::LayerDetectionAnalyzer::new(
                config,
                fs,
                source_parser,
            ),
        );
        Self {
            checker: ArchNamingChecker::new(),
            analyzer,
        }
    }

    pub fn checker(&self) -> &ArchNamingChecker {
        &self.checker
    }

    pub fn analyzer(&self) -> Arc<dyn IAnalyzer> {
        self.analyzer.clone()
    }

    pub fn orchestrator(&self) -> Arc<dyn INamingRunnerAggregate> {
        Arc::new(
            crate::naming_rules::agent_naming_orchestrator::NamingOrchestrator::new(
                self.checker.clone(),
                self.analyzer.clone(),
            ),
        )
    }
}

impl Default for NamingContainer {
    fn default() -> Self {
        Self::new()
    }
}