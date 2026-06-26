// PURPOSE: CheckContext — shared orchestrator context for check/scan commands
//
// DI container that wires all analysis subsystems together:
//   - Creates ImportContainer, NamingContainer, RoleContainer, CodeAnalysisContainer
//   - Wires ExternalLintContainer, OrphanContainer, LayerDetectionAnalyzer
//   - Creates FileCollectorProvider (scanner), CliLanguageDetector
//
// This is the main wiring point for the CLI — all 9 analysis components are
// assembled here and handed to CheckCommandsSurface for execution.
use std::sync::Arc;

use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

pub struct CheckContext {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub scanner_provider:
        Arc<dyn shared::common::contract_scanner_provider_port::IScannerProviderPort>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub layer_detector: Arc<dyn ILayerDetectionAggregate>,
    pub language_detector:
        Arc<dyn shared::common::contract_language_detector_port::ILanguageDetectorPort>,
}

impl CheckContext {
    pub fn new_default() -> Self {
        let import_container =
            import_rules::root_import_rules_container::ImportContainer::new_default();
        let analyzer = import_container.analyzer();
        let import_orchestrator = import_container.orchestrator();

        let checker_container =
            code_analysis::root_code_analysis_container::CodeAnalysisCheckerContainer::new(
                analyzer.clone(),
            );
        code_analysis::agent_code_analysis_orchestrator::init_global_checker(Arc::new(
            checker_container,
        ));

        let naming_container =
            naming_rules::root_naming_rules_container::NamingContainer::new(analyzer.clone());
        let naming_orchestrator = naming_container.orchestrator();
        let role_container = role_rules::root_role_rules_container::RoleContainer::new();
        let role_orchestrator = role_container.orchestrator();
        let code_analysis_container =
            code_analysis::root_code_analysis_container::CodeAnalysisContainer::new_with_analyzer(
                analyzer,
            );
        let code_analysis_linter = code_analysis_container.code_analysis_linter();
        let external_lint_container =
            external_lint::root_external_lint_container::ExternalLintContainer::new_default();
        let external_lint = external_lint_container.aggregate();
        let orphan_container =
            orphan_detector::root_orphan_detector_container::OrphanContainer::new();
        let orphan_orchestrator = orphan_container.analyzer();
        let aes_config = shared::config_system::taxonomy_config_vo::default_aes_config();
        let fs: Arc<dyn shared::common::contract_system_port::IFileSystemPort> =
            Arc::new(import_rules::infrastructure_filesystem_adapter::OSFileSystemAdapter::new());
        let parser: Arc<dyn shared::common::contract_parser_port::ISourceParserPort> =
            Arc::new(import_rules::root_import_rules_container::NullSourceParser);
        let layer_detector: Arc<dyn ILayerDetectionAggregate> = Arc::new(
            import_rules::capabilities_layer_detection_analyzer::LayerDetectionAnalyzer::new(
                aes_config, fs, parser,
            ),
        );
        let scanner_provider: Arc<
            dyn shared::common::contract_scanner_provider_port::IScannerProviderPort,
        > = Arc::new(
            shared::common::infrastructure_file_collector_provider::FileCollectorProvider::new(),
        );
        let language_detector: Arc<
            dyn shared::common::contract_language_detector_port::ILanguageDetectorPort,
        > = Arc::new(crate::infrastructure_language_detector::CliLanguageDetector::new());
        Self {
            code_analysis_linter,
            import_orchestrator,
            naming_orchestrator,
            external_lint,
            role_orchestrator,
            scanner_provider,
            orphan_orchestrator,
            layer_detector,
            language_detector,
        }
    }
}
