// PURPOSE: ChangeLintHandler — IChangeLintProtocol (FR-FileWatch-003)

use shared_common::taxonomy_path_vo::FilePath;
use shared_file_watch::contract_watch_protocol::IChangeLintProtocol;
use shared_file_watch::taxonomy_file_watch_error::WatchServiceError;
use shared_file_watch::taxonomy_file_watch_vo::WatchEvent;
use shared_quality_rules::CodeAnalysisRequest;
use shared_quality_rules::ICodeAnalysisAggregate;
use tracing::info;

use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ChangeLintHandler {
    linter: Arc<dyn ICodeAnalysisAggregate>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IChangeLintProtocol for ChangeLintHandler {
    fn lint_changed(&self, event: &WatchEvent) -> Result<(), WatchServiceError> {
        if FilePath::new(&event.path).is_err() {
            return Ok(());
        }
        let results = self
            .linter
            .execute(CodeAnalysisRequest::RunAnalysis { files: vec![] })
            .into_violations();
        let score = shared_common::Score::new(
            shared_quality_rules::utility_compliance_score::compute_score(&results),
        );
        info!(
            file = %event.path,
            violations = results.len(),
            score = score.value(),
            "File change linted"
        );
        Ok(())
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl ChangeLintHandler {
    pub fn new(linter: Arc<dyn ICodeAnalysisAggregate>) -> Self {
        Self { linter }
    }
}
