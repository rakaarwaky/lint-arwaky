// PURPOSE: orphan-rules constants — consumer layers consumed by orphan analysis
// Used by: orphan-rules (UtilityOrphanAnalyzer) to identify layers that consume utilities

/// Layers that are valid consumers of utility files.
pub const CONSUMER_LAYERS: &[&str] = &["capabilities", "agent", "surface", "root"];

/// Layers that rescue a taxonomy file (importing one means the file is alive).
/// These are filename prefixes (with trailing underscore), e.g. `contract_foo.rs`.
pub const TAXONOMY_RESCUERS: &[&str] =
    &["contract_", "capabilities_", "agent_", "surface_", "root_"];
