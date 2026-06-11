// PURPOSE: ImportContainer — wiring for import-rules feature (root layer, wiring only)
use std::sync::Arc;
use crate::import_rules::contract_import_parser_port::IImportParserPort;
use crate::import_rules::contract_rule_protocol::IArchImportProtocol;

pub struct ImportContainer {
    mandatory: Arc<dyn IArchImportProtocol>,
    forbidden: Arc<dyn IArchImportProtocol>,
}

impl ImportContainer {
    pub fn new() -> Self {
        let parser: Arc<dyn IImportParserPort> = Arc::new(
            crate::import_rules::infrastructure_import_parser_adapter::ImportParserAdapter::new(),
        );
        Self {
            mandatory: Arc::new(
                crate::import_rules::capabilities_import_mandatory_checker::ArchImportMandatoryChecker::new(parser.clone()),
            ),
            forbidden: Arc::new(
                crate::import_rules::capabilities_import_forbidden_checker::ArchImportForbiddenChecker::new(parser),
            ),
        }
    }

    pub fn mandatory_checker(&self) -> &dyn IArchImportProtocol {
        &*self.mandatory
    }

    pub fn forbidden_checker(&self) -> &dyn IArchImportProtocol {
        &*self.forbidden
    }

    pub fn orchestrator(&self) -> Arc<dyn crate::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate> {
        Arc::new(crate::import_rules::agent_import_orchestrator::ImportOrchestrator::new())
    }
}
