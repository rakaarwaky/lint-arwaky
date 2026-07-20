// PURPOSE: ImportContainer — wiring for import-rules feature (root layer, wiring only)
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use std::sync::Arc;

pub struct ImportContainer {
    config: ArchitectureConfig,
}

impl ImportContainer {
    pub fn new(_source_parser: Arc<dyn shared::common::contract_parser_protocol::ISourceParserProtocol>) -> Self {
        Self::new_with_config(shared::config_system::taxonomy_config_vo::default_aes_config())
    }

    pub fn new_with_config(config: ArchitectureConfig) -> Self {
        Self { config }
    }

    pub fn new_default() -> Self {
        Self::new_with_config(shared::config_system::taxonomy_config_vo::default_aes_config())
    }

    pub fn mandatory(&self) -> Arc<dyn shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol> {
        Arc::new(crate::capabilities_import_mandatory_checker::ArchImportMandatoryChecker::new())
    }

    pub fn forbidden(&self) -> Arc<dyn shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol> {
        Arc::new(crate::capabilities_import_forbidden_checker::ArchImportForbiddenChecker::new())
    }

    pub fn dummy(&self) -> Arc<dyn shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol> {
        Arc::new(crate::capabilities_dummy_import_checker::DummyImportChecker::new())
    }

    pub fn unused(&self) -> Arc<dyn shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol> {
        Arc::new(crate::capabilities_import_unused_checker::UnusedImportRuleChecker::new())
    }

    pub fn cycle(&self) -> Arc<dyn shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol> {
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
        ))
    }
}

pub struct NullSourceParser;
impl shared::common::contract_parser_protocol::ISourceParserProtocol for NullSourceParser {
    fn extract_imports(&self, _: &FilePath) -> Result<shared::code_analysis::taxonomy_import_source_vo::ImportInfoList, shared::common::taxonomy_parser_error::SourceParserError> { Ok(Default::default()) }
    fn get_raw_symbols(&self, _: &FilePath) -> Result<shared::mcp_server::taxonomy_job_vo::ResponseData, shared::common::taxonomy_parser_error::SourceParserError> { Ok(Default::default()) }
    fn get_class_attributes(&self, _: &FilePath) -> shared::mcp_server::taxonomy_job_vo::ResponseData { Default::default() }
    fn has_all_export(&self, _: &FilePath) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus { Default::default() }
    fn find_primitive_violations(&self, _: &FilePath, _: &shared::common::taxonomy_naming_list_vo::PrimitiveTypeList) -> shared::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList { Default::default() }
    fn find_unused_imports(&self, _: &FilePath) -> shared::code_analysis::taxonomy_import_source_vo::ImportInfoList { Default::default() }
    fn get_class_definitions(&self, _: &FilePath) -> Result<shared::common::taxonomy_suggestion_vo::MetadataVO, shared::common::taxonomy_parser_error::SourceParserError> { Ok(Default::default()) }
    fn get_function_definitions(&self, _: &FilePath) -> shared::common::taxonomy_suggestion_vo::MetadataVO { Default::default() }
    fn is_symbol_exported(&self, _: &FilePath, _: &shared::common::taxonomy_name_vo::SymbolName) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus { Default::default() }
    fn get_class_methods(&self, _: &FilePath) -> shared::common::taxonomy_suggestion_vo::MetadataVO { Default::default() }
    fn get_class_bases_map(&self, _: &FilePath) -> shared::common::taxonomy_suggestion_vo::MetadataVO { Default::default() }
    fn get_assignment_targets(&self, _: &FilePath) -> shared::common::taxonomy_suggestion_vo::MetadataVO { Default::default() }
    fn get_control_flow_count(&self, _: &FilePath) -> shared::common::taxonomy_common_vo::Count { Default::default() }
    fn is_barrel_file(&self, _: &FilePath) -> shared::common::taxonomy_common_vo::BooleanVO { Default::default() }
    fn get_stem(&self, _: &FilePath) -> shared::common::taxonomy_name_vo::SymbolName { Default::default() }
    fn is_entry_point(&self, _: &FilePath) -> shared::common::taxonomy_common_vo::BooleanVO { Default::default() }
    fn get_supported_extensions(&self) -> shared::common::taxonomy_common_vo::PatternList { Default::default() }
}
