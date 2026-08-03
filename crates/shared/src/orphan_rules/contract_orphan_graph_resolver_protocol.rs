// PURPOSE: IOrphanGraphResolverProtocol — contract trait for orphan entry point identification
// Graph building has been migrated to filesystem.build_orphan_graph_context().
// This trait now only handles entry point pattern matching (business logic).
use crate::orphan_rules::taxonomy_orphan_contract_vo::{
    OrphanEntryPatternListVO, OrphanFileListVO,
};

pub trait IOrphanGraphResolverProtocol: Send + Sync {
    /// Identify which of the supplied files count as entry points. A file
    /// is an entry point if its path matches any of the configured patterns
    /// (substring or suffix match). Returns the filtered list as a
    /// strongly-typed VO.
    fn identify_entry_points(
        &self,
        files: &[OrphanFileListVO],
        configured: &[OrphanEntryPatternListVO],
    ) -> OrphanFileListVO;
}
