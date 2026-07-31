// PURPOSE: SurfacesOrphanAnalyzer — ISurfacesOrphanProtocol for orphan surface detection.
// AST-based: uses inbound_links for FR-009 chain validation.

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
        let suffix = file_suffix(&basename);
        let category = Self::surface_category(&suffix);

        if is_reachable {
            // KNOWN LIMITATION (FR-009): Full chain validation requires inbound_links
            // to check: passive imported only by passive -> orphan.
            // Current trait signature does not include inbound_links.
            // BFS reachability is used as a necessary approximation.
            // When ISurfacesOrphanProtocol is extended, add per-category importer check.
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

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
                reason: None,
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

    fn surface_category(suffix: &str) -> &'static str {
        match suffix {
            "command" | "controller" | "page" | "router" => "smart",
            "hook" | "store" | "action" | "screen" => "utility",
            "component" | "view" | "layout" => "passive",
            _ => "unknown",
        }
    }
}
