// PURPOSE: ICapabilitiesOrphanProtocol — orphan indicator for the capabilities layer (AES503)
use crate::common::taxonomy_path_vo::FilePath;
use crate::quality_rules::taxonomy_analysis_vo::{OrphanIndicatorResult, ReachabilityResult};
use std::collections::HashMap;

pub trait ICapabilitiesOrphanProtocol: Send + Sync {
    /// Detect capabilities-layer orphans: checkers never wired into a container.
    fn is_capabilities_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
        content_map: &HashMap<String, String>,
        workspace_root: &std::path::Path,
    ) -> OrphanIndicatorResult;
}
