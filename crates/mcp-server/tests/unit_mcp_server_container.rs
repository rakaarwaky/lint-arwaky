// PURPOSE: Unit tests for McpContainer — DI wiring struct

use mcp_server_lint_arwaky::root_mcp_container::McpContainer;

// ─── Struct field accessibility ──────────────────────────────────────

#[test]
fn mcp_container_has_all_required_fields() {
    // Compile-time check: if any field is missing or renamed, this won't compile.
    // We can't call new_default() in unit tests (requires real filesystem),
    // but we verify the struct shape.
    fn _assert_fields(_c: &McpContainer) {
        let _ = &_c.code_analysis_linter;
        let _ = &_c.import_orchestrator;
        let _ = &_c.naming_orchestrator;
        let _ = &_c.orphan_orchestrator;
        let _ = &_c.external_lint;
        let _ = &_c.role_orchestrator;
        let _ = &_c.config_orchestrator;
    }
    // If this compiles, all fields are public and accessible.
}

#[test]
fn mcp_container_fields_are_arc_dyn_traits() {
    use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
    use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
    use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
    use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
    use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
    use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
    use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
    use std::sync::Arc;

    fn _assert_arc_types(_c: &McpContainer) {
        let _: &Arc<dyn ICodeAnalysisAggregate> = &_c.code_analysis_linter;
        let _: &Arc<dyn IImportRunnerAggregate> = &_c.import_orchestrator;
        let _: &Arc<dyn INamingRunnerAggregate> = &_c.naming_orchestrator;
        let _: &Arc<dyn IOrphanAggregate> = &_c.orphan_orchestrator;
        let _: &Arc<dyn IExternalLintAggregate> = &_c.external_lint;
        let _: &Arc<dyn IRoleRunnerAggregate> = &_c.role_orchestrator;
        let _: &Arc<dyn IConfigOrchestratorAggregate> = &_c.config_orchestrator;
    }
}
