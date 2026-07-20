use std::sync::Arc;

use shared::orphan_detector::taxonomy_orphan_analysis_policy_vo::OrphanAnalysisPolicy;
use shared::orphan_detector::taxonomy_orphan_file_cache_protocol::IOrphanFileCacheProtocol;
use shared::orphan_detector::taxonomy_orphan_filename_extractor_protocol::IOrphanFilenameExtractorProtocol;
use shared::orphan_detector::taxonomy_capabilities_orphan_protocol::ICapabilitiesOrphanProtocol;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct CapabilitiesOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
    cache: Arc<dyn IOrphanFileCacheProtocol>,
    policy: OrphanAnalysisPolicy,
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
impl ICapabilitiesOrphanProtocol for CapabilitiesOrphanAnalyzer {
    fn analyze(&self, path: &FilePath) -> Vec<LintResult> {
        let mut violations = Vec::new();
        // domain logic using injected dependencies
        violations
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl CapabilitiesOrphanAnalyzer {
    pub fn new(
        extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
        cache: Arc<dyn IOrphanFileCacheProtocol>,
        policy: OrphanAnalysisPolicy,
    ) -> Self {
        Self { extractor, cache, policy }
    }
}
