// PURPOSE: CliContainer — DI wiring for CLI binary aggregates
use std::sync::Arc;

use shared::cli_commands::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

pub struct CliContainer {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub git_aggregate: Arc<dyn GitHooksAggregate>,
    pub multi_project_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub report_formatter: Arc<dyn IReportFormatterAggregate>,
}

impl CliContainer {
    pub fn new_default() -> Self {
        // Create config orchestrator — single source of truth for config
        let config_container = config_system::root_config_system_container::ConfigContainer::new();
        let multi_project_orchestrator = config_container.orchestrator();

        // All containers get config from orchestrator
        let code_analysis_linter =
            code_analysis::root_code_analysis_container::CodeAnalysisContainer::from_orchestrator(
                &multi_project_orchestrator,
                ".",
            )
            .code_analysis_linter();

        let import_container =
            import_rules::root_import_rules_container::ImportContainer::from_orchestrator(
                &multi_project_orchestrator,
                ".",
            );
        let import_orchestrator = import_container.orchestrator();

        let role_container =
            role_rules::root_role_rules_container::RoleContainer::from_orchestrator(
                &multi_project_orchestrator,
                ".",
            );
        let role_orchestrator = role_container.orchestrator();

        let naming_container =
            naming_rules::root_naming_rules_container::NamingContainer::from_orchestrator(
                &multi_project_orchestrator,
                ".",
            );
        let naming_orchestrator = naming_container.orchestrator();

        let external_lint_container =
            external_lint::root_external_lint_container::ExternalLintContainer::new_default();
        let external_lint = external_lint_container.aggregate();

        let orphan_container =
            orphan_detector::root_orphan_detector_container::OrphanContainer::from_orchestrator(
                &multi_project_orchestrator,
                ".",
            );
        let orphan_orchestrator = orphan_container.analyzer();

        let git_container = git_hooks::root_git_hooks_container::GitContainer::new_default();
        let git_aggregate = git_container.aggregate();

        // Wire up report formatter capabilities → aggregate
        let text_formatter: Arc<dyn IReportFormatterProtocol> = Arc::new(
            report_formatter::TextFormatter::new(code_analysis_linter.clone()),
        );
        let json_formatter: Arc<dyn IReportFormatterProtocol> =
            Arc::new(report_formatter::JsonFormatter::new());
        let sarif_formatter: Arc<dyn IReportFormatterProtocol> =
            Arc::new(report_formatter::SarifFormatter::new());
        let junit_formatter: Arc<dyn IReportFormatterProtocol> =
            Arc::new(report_formatter::JunitFormatter::new());
        let report_formatter_agg: Arc<dyn IReportFormatterAggregate> =
            Arc::new(report_formatter::ReportFormatterOrchestrator::new(
                report_formatter::ReportFormatterDeps {
                    text: text_formatter,
                    json: json_formatter,
                    sarif: sarif_formatter,
                    junit: junit_formatter,
                },
            ));

        Self {
            code_analysis_linter,
            import_orchestrator,
            naming_orchestrator,
            role_orchestrator,
            external_lint,
            orphan_orchestrator,
            git_aggregate,
            multi_project_orchestrator,
            report_formatter: report_formatter_agg,
        }
    }

    pub fn fix_orchestrator_factory(
        &self,
    ) -> std::sync::Arc<
        dyn Fn(
                bool,
            ) -> std::sync::Arc<
                dyn shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate,
            > + Send
            + Sync,
    > {
        let fix_linter = self.code_analysis_linter.clone();
        Arc::new(move |dry_run| {
            auto_fix::root_auto_fix_container::AutoFixContainer::new(fix_linter.clone())
                .orchestrator(dry_run)
        })
    }
}
