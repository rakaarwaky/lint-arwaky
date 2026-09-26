// PURPOSE: IUtilityOrphanProtocol — orphan indicator for the utility layer (AES504)
use crate::common::taxonomy_path_vo::FilePath;
use crate::quality_rules::taxonomy_analysis_vo::{
    InboundLinkMap, OrphanIndicatorResult, ReachabilityResult,
};
use std::collections::HashMap;

pub trait IUtilityOrphanProtocol: Send + Sync {
    /// Detect utility-layer orphans: helper functions imported by nobody.
    fn is_utility_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        all_files: &[String],
        inbound_links: &InboundLinkMap,
        content_map: &HashMap<String, String>,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult;
}
