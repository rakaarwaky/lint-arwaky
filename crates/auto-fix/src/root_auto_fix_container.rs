// PURPOSE: AutoFixContainer — wiring for auto-fix feature (root layer, wiring only)
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
#[derive(Clone)]
pub struct AutoFixContainer {
    code_analysis_linter:
        Arc<dyn shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate>,
}

// ─── Block 2: Public Contract ─────────────────────────────
// (No trait impl — root container is wiring only)

// ─── Block 3: Constructors & Helpers ──────────────────────
impl AutoFixContainer {
    pub fn new(
        code_analysis_linter: Arc<
            dyn shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate,
        >,
    ) -> Self {
        Self {
            code_analysis_linter,
        }
    }

    pub fn orchestrator(&self, dry_run: bool) -> Arc<dyn LintFixOrchestratorAggregate> {
        let fix_protocol = crate::capabilities_fix_processor::LintFixProcessor::with_dry_run(
            dry_run,
            self.code_analysis_linter.clone(),
        );
        Arc::new(crate::agent_fix_orchestrator::FixOrchestrator::new(
            Arc::new(fix_protocol),
        ))
    }
}
