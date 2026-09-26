// PURPOSE: ITaxonomyOrphanProtocol — orphan indicator for the taxonomy layer (AES501)
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_path_vo::FilePath;
use crate::quality_rules::taxonomy_analysis_vo::{
    InboundLinkMap, OrphanIndicatorResult, ReachabilityResult,
};
use std::collections::HashMap;

pub trait ITaxonomyOrphanProtocol: Send + Sync {
    /// Detect taxonomy-layer orphans: files under taxonomy/ that nothing imports.
    fn is_taxonomy_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        definition: Option<&LayerDefinition>,
        inbound_links: &InboundLinkMap,
        all_files: &[String],
        content_map: &HashMap<String, String>,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult;
}
