// PURPOSE: ImportContainer — wiring for import-rules feature (root layer, wiring only)
use shared::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use std::sync::Arc;

pub struct ImportContainer {
    mandatory: Arc<dyn IImportMandatoryProtocol>,
    forbidden: Arc<dyn IImportForbiddenProtocol>,
    intent: Arc<dyn IDummyImportCheckerProtocol>,
    unused: Arc<dyn IUnusedImportProtocol>,
    cycle: Arc<dyn ICycleImportProtocol>,
    analyzer: Arc<dyn ILayerDetectionProtocol>,
}

impl ImportContainer {
    pub fn new() -> Self {
        Self::new_with_config(shared::config_system::taxonomy_config_vo::default_aes_config())
    }

    pub fn new_with_config(config: ArchitectureConfig) -> Self {
        let parser: Arc<dyn IImportParserPort> =
            Arc::new(crate::infrastructure_import_parser_adapter::ImportParserAdapter::new());

        let analyzer: Arc<dyn ILayerDetectionProtocol> = Arc::new(
            crate::capabilities_layer_detection_analyzer::LayerDetectionAnalyzer::new(
                config.clone(),
            ),
        );

        let mandatory = Arc::new(
            crate::capabilities_import_mandatory_checker::ArchImportMandatoryChecker::new(
                parser.clone(),
            ),
        );
        let forbidden = Arc::new(
            crate::capabilities_import_forbidden_checker::ImportForbiddenChecker::new(
                parser.clone(),
            ),
        );
        let intent: Arc<
            dyn shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol,
        > = Arc::new(
            crate::capabilities_dummy_import_checker::DummyImportChecker::new(parser.clone()),
        );
        let unused = Arc::new(
            crate::capabilities_import_unused_checker::UnusedImportRuleChecker::new(parser.clone()),
        );
        let cycle = Arc::new(
            crate::capabilities_cycle_import_analyzer::CycleImportAnalyzer::new(
                config,
                parser.clone(),
            ),
        );

        Self {
            mandatory: mandatory.clone(),
            forbidden: forbidden.clone(),
            intent: intent.clone(),
            unused: unused.clone(),
            cycle: cycle.clone(),
            analyzer,
        }
    }

    pub fn new_default() -> Self {
        Self::new()
    }

    pub fn mandatory_checker(&self) -> &dyn IImportMandatoryProtocol {
        &*self.mandatory
    }

    pub fn forbidden_checker(&self) -> &dyn IImportForbiddenProtocol {
        &*self.forbidden
    }

    pub fn analyzer(&self) -> Arc<dyn ILayerDetectionProtocol> {
        self.analyzer.clone()
    }

    pub fn orchestrator(&self) -> Arc<dyn IImportRunnerAggregate> {
        Arc::new(crate::agent_import_orchestrator::ImportOrchestrator::new(
            self.mandatory.clone(),
            self.forbidden.clone(),
            self.intent.clone(),
            self.unused.clone(),
            self.cycle.clone(),
            self.analyzer.clone(),
        ))
    }
}
