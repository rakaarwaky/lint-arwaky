// PURPOSE: ImportOrchestrator — agent that orchestrates import rule checks
use crate::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub struct ImportOrchestrator {}

impl ImportOrchestrator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl IImportRunnerAggregate for ImportOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        // TODO: Wire import checkers here
        Vec::new()
    }

    fn name(&self) -> &str { "import-rules" }
}
