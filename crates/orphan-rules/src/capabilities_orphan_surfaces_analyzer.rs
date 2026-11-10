use shared_common::taxonomy_layer_vo::LayerDefinition;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_orphan_rules::contract_orphan_protocol::ISurfacesOrphanProtocol;
use shared_orphan_rules::utility_orphan_filename::{file_basename, file_stem};
use shared_quality_rules::taxonomy_quality_rules_vo::{
    InboundLinkMap, OrphanIndicatorResult, ReachabilityResult,
};

// ─── Block 1: Module Declaration ────────────────────────────

pub struct SurfacesOrphanAnalyzer;

impl Default for SurfacesOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SurfacesOrphanAnalyzer {
    pub fn new() -> Self {
        Self
    }
}

// Severity is defect-keyed, not extension-keyed: AES506's defect is
// "not reachable from entry point" — every surface orphan carries the same
// severity regardless of file extension. The surface category stays in the
// message text (informative only) and never influences severity.
const SURFACE_SEVERITY: Severity = Severity::HIGH;

// ─── Block 2: Module Contract ────────────────

impl ISurfacesOrphanProtocol for SurfacesOrphanAnalyzer {
    fn is_surface_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        alive_files: &ReachabilityResult,
        _inbound_links: &InboundLinkMap,
        _definition: Option<&LayerDefinition>,
    ) -> OrphanIndicatorResult {
        let fp_val = f.value();
        let basename = file_basename(fp_val);
        let stem = file_stem(fp_val);

        // FR-009: Unclassifiable extensions → skipped (no orphan check)
        let extension = basename.rsplit('.').next().unwrap_or("");
        if !matches!(extension, "rs" | "py" | "ts" | "js" | "md" | "txt") {
            return OrphanIndicatorResult::not_orphan(Severity::LOW);
        }

        // Surface is the outermost layer — only 1 condition: reachability from _entry
        let is_reachable = alive_files.paths.contains(f);
        if is_reachable {
            return OrphanIndicatorResult::not_orphan(Severity::LOW);
        }

        let severity = SURFACE_SEVERITY;

        OrphanIndicatorResult::orphan(
            "SURFACE_ORPHAN",
            format!("'{}' surface is not reachable from any entry point.", stem),
            "A surface without reachability cannot handle any command.",
            format!("Import '{}' in a root_*_entry file.", stem),
            severity,
        )
    }
}

// ─── Block 3: Module Helpers ────────────
