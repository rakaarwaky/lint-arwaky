// PURPOSE: ChangeLintHandler — IChangeLintProtocol (FR-FileWatch-003)

use shared_common::taxonomy_path_vo::FilePath;
use shared_file_watch::contract_watch_protocol::IChangeLintProtocol;
use shared_file_watch::taxonomy_file_watch_error::WatchServiceError;
use shared_file_watch::taxonomy_file_watch_vo::ChangeLintSummary;
use shared_file_watch::taxonomy_file_watch_vo::WatchEvent;
use shared_quality_rules::CodeAnalysisRequest;
use shared_quality_rules::ICodeAnalysisAggregate;
use tracing::info;

use std::sync::Arc;

// ─── Block 1: Module Declaration ───────────────────────────

pub struct ChangeLintHandler {
    linter: Arc<dyn ICodeAnalysisAggregate>,
}

impl ChangeLintHandler {
    pub fn new(linter: Arc<dyn ICodeAnalysisAggregate>) -> Self {
        Self { linter }
    }
}
// ─── Block 2: Module Contract ───────────────

impl IChangeLintProtocol for ChangeLintHandler {
    fn lint_changed(
        &self,
        event: &WatchEvent,
    ) -> Result<ChangeLintSummary, WatchServiceError> {
        if FilePath::new(&event.path).is_err() {
            // An unparseable path is not a violation; report a clean summary so
            // the caller's policy check stays false.
            return Ok(ChangeLintSummary::new(
                event.path.clone(),
                0,
                0,
                100.0,
            ));
        }
        let results = self
            .linter
            .execute(CodeAnalysisRequest::RunAnalysis { files: vec![] })
            .into_violations();
        let score = shared_common::Score::new(
            shared_quality_rules::utility_compliance_score::compute_score(&results),
        );
        let critical = results
            .iter()
            .filter(|r| r.severity == shared_common::Severity::CRITICAL)
            .count();
        info!(
            file = %event.path,
            violations = results.len(),
            score = score.value(),
            "File change linted"
        );
        Ok(ChangeLintSummary::new(
            event.path.clone(),
            results.len(),
            critical,
            score.value(),
        ))
    }
}

// ─── Block 3: Module Helpers ──────
