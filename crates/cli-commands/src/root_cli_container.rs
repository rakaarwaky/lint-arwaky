// PURPOSE: CliContainer — DI wiring for CLI binary aggregates
use std::sync::Arc;

use shared::config_system::IConfigOrchestratorAggregate;
use shared::external_lint::IExternalLintAggregate;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::import_rules::IImportRunnerAggregate;
use shared::naming_rules::INamingRunnerAggregate;
use shared::orphan_rules::IOrphanAggregate;
use shared::quality_rules::ICodeAnalysisAggregate;
use shared::report_formatter::IReportFormatterAggregate;

pub struct CliContainer {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub report_formatter: Arc<dyn IReportFormatterAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
}

impl CliContainer {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
        import_orchestrator: Arc<dyn IImportRunnerAggregate>,
        naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
        external_lint: Arc<dyn IExternalLintAggregate>,
        orphan_orchestrator: Arc<dyn IOrphanAggregate>,
        config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
        report_formatter: Arc<dyn IReportFormatterAggregate>,
        filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> Self {
        Self {
            code_analysis_linter,
            import_orchestrator,
            naming_orchestrator,
            external_lint,
            orphan_orchestrator,
            config_orchestrator,
            report_formatter,
            filesystem,
        }
    }
}
