// PURPOSE: NamingOrchestrator — agent that orchestrates naming rule checks
use crate::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub struct NamingOrchestrator {}

impl NamingOrchestrator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl INamingRunnerAggregate for NamingOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        let mut results = Vec::new();
        // Naming checks will be orchestrated here
        results
    }

    fn name(&self) -> &str { "naming-rules" }
}
