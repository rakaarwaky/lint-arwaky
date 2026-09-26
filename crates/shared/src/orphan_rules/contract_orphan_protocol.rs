// PURPOSE: orphan-domain capability contracts (AES102 `_protocol`).
//
// One file for the orphan feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs.

use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_path_vo::FilePath;
use crate::orphan_rules::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use crate::quality_rules::taxonomy_analysis_vo::InheritanceMap;
use crate::quality_rules::taxonomy_analysis_vo::{
    InboundLinkMap, OrphanIndicatorResult, ReachabilityResult,
};
use std::collections::HashMap;

pub trait IAgentOrphanProtocol: Send + Sync {
    /// Detect agent-layer orphans: orchestrators never composed into a root container.
    fn is_agent_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        all_files: &[String],
        content_map: &HashMap<String, String>,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult;
}

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

pub trait IContractOrphanProtocol: Send + Sync {
    /// Detect contract-layer orphans: traits/aggregates never implemented or exported.
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        inheritance_map: &InheritanceMap,
        all_files: &[String],
        content_map: &HashMap<String, String>,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult;
}

pub trait IOrphanParserProtocol: Send + Sync {
    /// Parse a file based on its extension, routing to the correct language parser.
    fn parse_file(&self, path: &str, content: &str) -> FileParseResultVO;

    /// Check if a file extension is supported by any parser.
    fn is_supported(&self, path: &str) -> bool;
}

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
