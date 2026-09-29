// PURPOSE: orphan-rules constants — consumer layers consumed by orphan analysis
// Used by: orphan-rules (UtilityOrphanAnalyzer) to identify layers that consume utilities

/// Layers that are valid consumers of utility files.
pub const CONSUMER_LAYERS: &[&str] = &["capabilities", "agent", "surface", "surfaces", "root"];
