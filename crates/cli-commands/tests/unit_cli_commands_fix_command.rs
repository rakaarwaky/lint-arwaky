//! Unit tests for surface_fix_command — FixCommandsSurface construction and dry-run logic.

use cli_commands_lint_arwaky::surface_fix_action::FixCommandsSurface;
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use std::sync::Arc;

/// Verify FixCommandsSurface::new accepts the correct types.
#[test]
fn fix_surface_construction_compiles() {
    // This is a compile-time contract test disguised as unit test.
    // We verify the constructor signature matches expectations.
    #[allow(clippy::type_complexity)]
    fn assert_constructor(
        _f: fn(
            Arc<dyn ICodeAnalysisAggregate>,
            Arc<dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync>,
        ) -> FixCommandsSurface,
    ) {
    }
    assert_constructor(FixCommandsSurface::new);
}
