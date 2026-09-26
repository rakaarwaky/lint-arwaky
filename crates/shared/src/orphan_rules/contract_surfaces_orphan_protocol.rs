// PURPOSE: ISurfacesOrphanProtocol — orphan indicator for the surfaces layer (AES506)
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_path_vo::FilePath;
use crate::quality_rules::taxonomy_analysis_vo::{
    InboundLinkMap, OrphanIndicatorResult, ReachabilityResult,
};

pub trait ISurfacesOrphanProtocol: Send + Sync {
    /// Detect surface-layer orphans: UI/command surfaces never reachable from an entry point.
    fn is_surface_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
        inbound_links: &InboundLinkMap,
        definition: Option<&LayerDefinition>,
    ) -> OrphanIndicatorResult;
}
