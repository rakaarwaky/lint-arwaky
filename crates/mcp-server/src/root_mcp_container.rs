// PURPOSE: McpContainer — DI wiring for MCP server aggregates
use std::sync::Arc;

use crate::agent_mcp_server_orchestrator::{McpServerDependencies, McpServerOrchestrator};
use auto_fix::root_auto_fix_container::AutoFixContainer;
use git_hooks::root_git_hooks_container::GitContainer;
use maintenance::root_maintenance_container::MaintenanceContainer;
use project_setup::root_project_setup_container::SetupContainer;
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::maintenance::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

pub struct McpContainer {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub fix_orchestrator: Arc<dyn LintFixOrchestratorAggregate>,
    pub git_hooks_aggregate: Arc<dyn GitHooksAggregate>,
    pub maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    pub setup_orchestrator: Arc<dyn SetupManagementAggregate>,
}

impl McpContainer {
    /// Get the MCP server orchestrator with all dependencies wired.
    pub fn orchestrator(&self) -> McpServerOrchestrator {
        let deps = McpServerDependencies {
            code_analysis_linter: self.code_analysis_linter.clone(),
            fix_orchestrator: self.fix_orchestrator.clone(),
            orphan_orchestrator: self.orphan_orchestrator.clone(),
            maintenance_orchestrator: self.maintenance_orchestrator.clone(),
            git_hooks_aggregate: self.git_hooks_aggregate.clone(),
            setup_orchestrator: self.setup_orchestrator.clone(),
            config_orchestrator: self.config_orchestrator.clone(),
            external_lint: self.external_lint.clone(),
            import_orchestrator: self.import_orchestrator.clone(),
            naming_orchestrator: self.naming_orchestrator.clone(),
            role_orchestrator: self.role_orchestrator.clone(),
        };
        McpServerOrchestrator::new(deps)
    }

    pub fn new_default() -> Self {
        // Create config orchestrator — single source of truth for config
        let config_container = config_system::root_config_system_container::ConfigContainer::new();
        let orchestrator = config_container.orchestrator();

        // All containers get config from orchestrator
        let code_analysis_linter =
            code_analysis::root_code_analysis_container::CodeAnalysisContainer::from_orchestrator(
                &orchestrator,
                ".",
            )
            .code_analysis_linter();

        let import_container =
            import_rules::root_import_rules_container::ImportContainer::from_orchestrator(
                &orchestrator,
                ".",
            );
        let import_orchestrator = import_container.orchestrator();

        let naming_container =
            naming_rules::root_naming_rules_container::NamingContainer::from_orchestrator(
                &orchestrator,
                ".",
            );
        let naming_orchestrator = naming_container.orchestrator();

        let orphan_container =
            orphan_detector::root_orphan_detector_container::OrphanContainer::from_orchestrator(
                &orchestrator,
                ".",
            );
        let orphan_orchestrator = orphan_container.analyzer();

        let ext_container =
            external_lint::root_external_lint_container::ExternalLintContainer::new_default();
        let external_lint = ext_container.aggregate();

        let role_container =
            role_rules::root_role_rules_container::RoleContainer::from_orchestrator(
                &orchestrator,
                ".",
            );
        let role_orchestrator = role_container.orchestrator();

        // Auto-fix orchestrator (uses same code analysis linter)
        let auto_fix_container =
            AutoFixContainer::new(code_analysis_linter.clone());
        let fix_orchestrator = auto_fix_container.orchestrator(false);

        // Git hooks aggregate
        let git_hooks_container = GitContainer::new_default();
        let git_hooks_aggregate = git_hooks_container.aggregate();

        // Maintenance orchestrator (doctor, security, dependencies)
        let maintenance_container = MaintenanceContainer::new();
        let maintenance_orchestrator = maintenance_container.orchestrator();

        // Setup orchestrator (init, install, mcp-config)
        let setup_container = SetupContainer::new();
        let setup_orchestrator = setup_container.aggregate();

        Self {
            code_analysis_linter,
            import_orchestrator,
            naming_orchestrator,
            orphan_orchestrator,
            external_lint,
            role_orchestrator,
            config_orchestrator: orchestrator,
            fix_orchestrator,
            git_hooks_aggregate,
            maintenance_orchestrator,
            setup_orchestrator,
        }
    }
}
