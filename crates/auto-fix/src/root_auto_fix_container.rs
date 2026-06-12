// PURPOSE: AutoFixContainer — wiring for auto-fix feature (root layer, wiring only)
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use std::sync::Arc;

pub struct AutoFixContainer {
    arch_linter: Arc<dyn crate::code_analysis::contract_lint_protocol::IArchLintProtocol>,
}

impl AutoFixContainer {
    pub fn new(
        arch_linter: Arc<dyn crate::code_analysis::contract_lint_protocol::IArchLintProtocol>,
    ) -> Self {
        Self { arch_linter }
    }

    pub fn orchestrator(&self, dry_run: bool) -> Arc<dyn LintFixOrchestratorAggregate> {
        let fix_protocol =
            crate::auto_fix::capabilities_fix_processor::LintFixProcessor::with_dry_run(
                dry_run,
                self.arch_linter.clone(),
            );
        Arc::new(
            crate::auto_fix::agent_fix_orchestrator::FixOrchestrator::new(Arc::new(fix_protocol)),
        )
    }
}
