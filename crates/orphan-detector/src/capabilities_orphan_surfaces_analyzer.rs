// PURPOSE: SurfacesOrphanAnalyzer — ISurfacesOrphanProtocol for orphan surface detection
use shared::code_analysis::{OrphanIndicatorResult, ReachabilityResult};

use shared::common::{FilePath, Severity};

use shared::orphan_detector::{AesOrphanViolation, ISurfacesOrphanProtocol};

use shared::common::LayerDefinition;
use shared::orphan_detector::utility_orphan_filename::{file_basename, file_stem, file_suffix};

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
        // Import graph only — no identifier-based fallback
        let is_reachable = alive_files.paths.contains(f);
        if is_reachable {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        let fp_val = f.value();
        let basename = file_basename(fp_val);
        let stem = file_stem(fp_val);
        let suffix = Self::get_surface_suffix(&basename);
        let category = Self::surface_category(&suffix);

        OrphanIndicatorResult::new(
            true,
            AesOrphanViolation::SurfaceOrphan {
                category,
                stem,
                reason: Some("Surface is unreachable from entry points.".into()),
            }
            .to_string(),
            Severity::HIGH,
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
            "command" | "controller" | "page" => "smart",
            "hook" | "store" | "action" | "screen" | "router" => "utility",
            "component" | "view" | "layout" => "passive",
            _ => "unknown",
        }
    }
}
