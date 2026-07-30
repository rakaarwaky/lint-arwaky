// PURPOSE: SurfacesOrphanAnalyzer — ISurfacesOrphanProtocol for orphan surface detection
use shared::code_analysis::{OrphanIndicatorResult, ReachabilityResult};
use shared::common::{FilePath, LayerDefinition, Severity};
use shared::orphan_detector::utility_orphan_filename::{file_basename, file_stem, file_suffix};
use shared::orphan_detector::{AesOrphanViolation, ISurfacesOrphanProtocol};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct SurfacesOrphanAnalyzer {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ISurfacesOrphanProtocol for SurfacesOrphanAnalyzer {
    fn is_surface_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        alive_files: &ReachabilityResult,
        _definition: Option<&LayerDefinition>,
    ) -> OrphanIndicatorResult {
        let is_reachable = alive_files.paths.contains(f);
        let fp_val = f.value();
        let basename = file_basename(fp_val);
        let stem = file_stem(fp_val);
        let suffix = Self::get_surface_suffix(&basename);
        let category = Self::surface_category(&suffix);

        // FR-009: Category-aware orphan detection
        // Dependency chain: Entry → Smart → Utility → Passive
        if is_reachable {
            // Even if BFS-reachable, validate the import chain per FR-009:
            // - Utility surfaces must be imported by a Smart surface (not just any file)
            // - Passive surfaces must be imported by Smart OR Utility (not just Passive)
            //
            // NOTE: Full chain validation requires inbound_links (not available in
            // current trait signature). Current implementation trusts BFS reachability
            // as a necessary approximation. When trait is extended with inbound_links,
            // add per-category importer validation here.
            //
            // TODO(FR-009): Extend ISurfacesOrphanProtocol with inbound_links param
            // to enforce: passive imported only by passive → orphan.
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Not BFS-reachable — determine severity by category
        // Message formatting is handled by Display impl in taxonomy_violation_orphan_vo.rs
        let severity = match category {
            "smart" => Severity::HIGH,
            "utility" => Severity::MEDIUM,
            "passive" => Severity::LOW,
            _ => Severity::MEDIUM,
        };

        OrphanIndicatorResult::new(
            true,
            AesOrphanViolation::SurfaceOrphan {
                category,
                stem: stem.clone(),
                reason: None, // Display impl generates WHY/FIX from category
            }
            .to_string(),
            severity,
        )
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for SurfacesOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SurfacesOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    /// Get surface suffix from filename
    fn get_surface_suffix(basename: &str) -> String {
        file_suffix(basename)
    }

    /// Surface category
    fn surface_category(suffix: &str) -> &'static str {
        match suffix {
            "command" | "controller" | "page" | "router" => "smart",
            "hook" | "store" | "action" | "screen" => "utility",
            "component" | "view" | "layout" => "passive",
            _ => "unknown",
        }
    }
}
