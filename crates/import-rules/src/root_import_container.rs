// PURPOSE: ImportContainer — wiring for import-rules feature (root layer, wiring only)
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::import_rules::contract_rule_protocol::IArchImportProtocol;
use std::sync::Arc;

pub struct ImportContainer {
    mandatory: Arc<dyn IArchImportProtocol>,
    forbidden: Arc<dyn IArchImportProtocol>,
    intent: Arc<dyn IArchImportProtocol>,
    analyzer: Arc<dyn IAnalyzer>,
}

impl ImportContainer {
    pub fn new() -> Self {
        let config = shared::config_system::taxonomy_config_vo::default_aes_config();
        let fs =
            Arc::new(file_system::infrastructure_filesystem_adapter::OSFileSystemAdapter::new());
        let source_parser = Arc::new(
            source_parsing::infrastructure_parser_adapter::SourceParserOrchestrator::new(
                Box::new(source_parsing::infrastructure_py_scanner::ASTPythonParserAdapter::new()),
                Box::new(source_parsing::infrastructure_rust_scanner::ASTRustParserAdapter::new()),
                Box::new(source_parsing::infrastructure_js_scanner::ASTJSParserAdapter::new()),
            ),
        );
        let parser: Arc<dyn IImportParserPort> =
            Arc::new(crate::infrastructure_import_parser_adapter::ImportParserAdapter::new());
        let analyzer = Arc::new(
            crate::capabilities_layer_detection_analyzer::LayerDetectionAnalyzer::new(
                config,
                fs,
                source_parser,
            ),
        );

        let mandatory = Arc::new(
            crate::capabilities_import_mandatory_checker::ArchImportMandatoryChecker::new(
                parser.clone(),
            ),
        );
        let forbidden = Arc::new(
            crate::capabilities_import_forbidden_checker::ArchImportForbiddenChecker::new(
                parser.clone(),
            ),
        );
        let intent =
            Arc::new(crate::capabilities_dummy_import_checker::DummyImportChecker::new(parser));

        Self {
            mandatory: mandatory.clone(),
            forbidden: forbidden.clone(),
            intent: intent.clone(),
            analyzer,
        }
    }

    pub fn mandatory_checker(&self) -> &dyn IArchImportProtocol {
        &*self.mandatory
    }

    pub fn forbidden_checker(&self) -> &dyn IArchImportProtocol {
        &*self.forbidden
    }

    pub fn analyzer(&self) -> Arc<dyn IAnalyzer> {
        self.analyzer.clone()
    }

    pub fn orchestrator(&self) -> Arc<dyn IImportRunnerAggregate> {
        Arc::new(crate::agent_import_orchestrator::ImportOrchestrator::new(
            self.mandatory.clone(),
            self.forbidden.clone(),
            self.intent.clone(),
            self.analyzer.clone(),
        ))
    }
}

impl Default for ImportContainer {
    fn default() -> Self {
        Self::new()
    }
}
