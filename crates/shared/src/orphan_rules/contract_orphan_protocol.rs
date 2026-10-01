// PURPOSE: orphan-domain capability contracts (AES102 `_protocol`).
//
// One file for the orphan feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs. One trait per FR-OrphanRules-002..008.
//
// FR-OrphanRules-001 (graph context), FR-OrphanRules-002 (entry points), and
// FR-OrphanRules-003 (reachability) deliberately have no trait here. They are
// orchestration steps the orphan-rules agent owns: it receives the context
// from the filesystem aggregate and delegates the pure work to shared
// utilities. An agent must not implement a contract protocol (AES405), and no
// other feature consumes those steps, so a seam would have exactly one
// implementer — the agent — which is the shape the rule forbids.

use crate::taxonomy_orphan_rules_vo::FileParseResultVO;
use shared_common::taxonomy_definition_vo::LayerDefinition;
use shared_common::taxonomy_path_vo::FilePath;
use shared_quality_rules::taxonomy_quality_rules_vo::InheritanceMap;
use shared_quality_rules::taxonomy_quality_rules_vo::{
    InboundLinkMap, OrphanIndicatorResult, ReachabilityResult,
};
use std::collections::HashMap;

/// FR-OrphanRules-004 (AES501): taxonomy files must have a higher-layer importer
/// or be reachable from an entry point.
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

/// FR-OrphanRules-005 (AES502): contract files must be reachable and have both
/// an implementation and a caller.
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

/// FR-OrphanRules-006 (AES503): capability files must be reachable and wired
/// into a root container.
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

/// FR-OrphanRules-007 (AES504): utility files must be reachable and imported by
/// at least one consumer layer.
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

/// FR-OrphanRules-008 (AES505): agent files must be reachable and have their
/// aggregate traits wired into a composition root.
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

/// FR-OrphanRules-009 (AES506): surface files must be reachable, checked per
/// their Smart / Utility / Passive group classification.
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

/// Language-parser seam behind the content scans: routes a file to the parser
/// for its extension and reports whether that extension is supported.
pub trait IOrphanParserProtocol: Send + Sync {
    /// Parse a file based on its extension, routing to the correct language parser.
    fn parse_file(&self, path: &str, content: &str) -> FileParseResultVO;

    /// Check if a file extension is supported by any parser.
    fn is_supported(&self, path: &str) -> bool;
}
