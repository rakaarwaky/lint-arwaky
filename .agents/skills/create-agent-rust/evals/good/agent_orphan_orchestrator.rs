use std::sync::Arc;

use shared::code_analysis::taxonomy_file_path_vo::FilePath;
use shared::code_analysis::taxonomy_lint_result_vo::LintResult;
use shared::orphan_detector::taxonomy_orphan_analyzer_protocol::IOrphanAnalyzerProtocol;
use shared::orphan_detector::taxonomy_orphan_orchestrator_aggregate::IOrphanOrchestratorAggregate;
use shared::scan::taxonomy_scan_request_vo::ScanRequest;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct OrphanOrchestrator {
    analyzer: Arc<dyn IOrphanAnalyzerProtocol>,
}

// ─── Block 2: Public Contract (domain aggregate ONLY) ─────
impl IOrphanOrchestratorAggregate for OrphanOrchestrator {
    fn execute(&self, request: &ScanRequest) -> Vec<LintResult> {
        let mut violations = Vec::new();
        for file in request.files() {
            match self.analyzer.analyze(file) {
                Ok(result) => violations.extend(result.into_violations()),
                Err(err) => violations.push(LintResult::from_analysis_error(file, err)),
            }
        }
        violations
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl OrphanOrchestrator {
    pub fn new(analyzer: Arc<dyn IOrphanAnalyzerProtocol>) -> Self {
        Self { analyzer }
    }
}
