// PURPOSE: LintDependenciesVO — grouped linter dependencies for pipeline constructors
//
// Groups all 6 linter group dependencies + config + format into a single struct
// to avoid clippy::too_many_arguments warnings in constructor signatures.
use std::sync::Arc;

use crate::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use crate::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use crate::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use crate::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use crate::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use crate::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use crate::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

/// Grouped linter dependencies for pipeline constructors.
/// Contains all 6 linter group deps + config + format to avoid too_many_arguments.
pub struct LintDependencies {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
}

impl LintDependencies {
    pub fn new(
        code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
        naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
        import_orchestrator: Arc<dyn IImportRunnerAggregate>,
        external_lint: Arc<dyn IExternalLintAggregate>,
        role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
        orphan_orchestrator: Arc<dyn IOrphanAggregate>,
        config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    ) -> Self {
        Self {
            code_analysis_linter,
            naming_orchestrator,
            import_orchestrator,
            external_lint,
            role_orchestrator,
            orphan_orchestrator,
            config_orchestrator,
        }
    }
}
