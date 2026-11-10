// PURPOSE: orphan-rules constants — consumer layers consumed by orphan analysis
// Used by: orphan-rules (UtilityOrphanAnalyzer) to identify layers that consume utilities

use shared_common::taxonomy_severity_vo::Severity;

/// Layers that are valid consumers of utility files.
pub const CONSUMER_LAYERS: &[&str] = &["capabilities", "agent", "surface", "root"];

/// Layers that rescue a taxonomy file (importing one means the file is alive).
/// These are filename prefixes (with trailing underscore), e.g. `contract_foo.rs`.
pub const TAXONOMY_RESCUERS: &[&str] =
    &["contract_", "capabilities_", "agent_", "surface_", "root_"];

/// Severity for a surface orphan (AES506). Defect-keyed: "not reachable from
/// entry point" is HIGH for every surface orphan regardless of file extension.
/// Lives here so every layer shares one policy value (not a private const in a
/// capability file).
pub const SURFACE_SEVERITY: Severity = Severity::HIGH;
