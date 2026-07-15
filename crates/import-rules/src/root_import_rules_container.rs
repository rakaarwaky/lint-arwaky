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
        Self::new_with_config_and_parser(config, parser)
    }

    pub fn new_with_config_and_parser(
        config: ArchitectureConfig,
        parser: Arc<dyn IImportParserPort>,
    ) -> Self {
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

impl Default for ImportContainer {
    fn default() -> Self {
        Self::new()
    }
}

/// Null source parser — placeholder for testing or when no parser is needed
pub struct NullSourceParser;

impl shared::common::contract_parser_port::ISourceParserPort for NullSourceParser {
    fn extract_imports(
        &self,
        _path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> Result<
        shared::code_analysis::taxonomy_import_source_vo::ImportInfoList,
        shared::common::taxonomy_parser_error::SourceParserError,
    > {
        Ok(shared::code_analysis::taxonomy_import_source_vo::ImportInfoList::default())
    }
    fn get_raw_symbols(
        &self,
        _path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> Result<
        shared::mcp_server::taxonomy_job_vo::ResponseData,
        shared::common::taxonomy_parser_error::SourceParserError,
    > {
        Ok(shared::mcp_server::taxonomy_job_vo::ResponseData::default())
    }
    fn get_class_attributes(
        &self,
        _path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::mcp_server::taxonomy_job_vo::ResponseData {
        shared::mcp_server::taxonomy_job_vo::ResponseData::default()
    }
    fn has_all_export(
        &self,
        _path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus {
        shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(false)
    }
    fn find_primitive_violations(
        &self,
        _path: &shared::common::taxonomy_path_vo::FilePath,
        _primitive_types: &shared::common::taxonomy_naming_list_vo::PrimitiveTypeList,
    ) -> shared::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList {
        shared::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList::default()
    }
    fn find_unused_imports(
        &self,
        _path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::code_analysis::taxonomy_import_source_vo::ImportInfoList {
        shared::code_analysis::taxonomy_import_source_vo::ImportInfoList::default()
    }
    fn get_class_definitions(
        &self,
        _path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> Result<
        shared::common::taxonomy_suggestion_vo::MetadataVO,
        shared::common::taxonomy_parser_error::SourceParserError,
    > {
        Ok(shared::common::taxonomy_suggestion_vo::MetadataVO::new(
            std::collections::HashMap::new(),
        ))
    }
    fn get_function_definitions(
        &self,
        _path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn is_symbol_exported(
        &self,
        _path: &shared::common::taxonomy_path_vo::FilePath,
        _symbol: &shared::common::taxonomy_name_vo::SymbolName,
    ) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus {
        shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(false)
    }
    fn get_class_methods(
        &self,
        _path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn get_class_bases_map(
        &self,
        _path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn get_assignment_targets(
        &self,
        _path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn get_control_flow_count(
        &self,
        _path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::common::taxonomy_common_vo::Count {
        shared::common::taxonomy_common_vo::Count::default()
    }
    fn is_barrel_file(
        &self,
        _path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::common::taxonomy_common_vo::BooleanVO {
        shared::common::taxonomy_common_vo::BooleanVO::default()
    }
    fn get_stem(
        &self,
        _path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::common::taxonomy_name_vo::SymbolName {
        shared::common::taxonomy_name_vo::SymbolName::new("")
    }
    fn is_entry_point(
        &self,
        _path: &shared::common::taxonomy_path_vo::FilePath,
    ) -> shared::common::taxonomy_common_vo::BooleanVO {
        shared::common::taxonomy_common_vo::BooleanVO::default()
    }
    fn get_supported_extensions(&self) -> shared::common::taxonomy_common_vo::PatternList {
        shared::common::taxonomy_common_vo::PatternList::default()
    }
}
