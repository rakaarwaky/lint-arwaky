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
        let severity = match category {
            "smart" => Severity::HIGH,
            "utility" => Severity::MEDIUM,
            "passive" => Severity::LOW,
            _ => Severity::MEDIUM,
        };

        let (reason_line, fix_line) = match category {
            "smart" => (
                format!(
                    "the {} surface '{}' is not imported by any entry point or container such as root_*_entry.py/rs/ts.",
                    category, stem
                ),
                format!(
                    "Import '{}' at the entry point. If this surface is dead code, delete the file and its module declaration. Consider moving it to utility surface (_hook/_store/_action/_screen) or passive (surface _component/_view/_layout) if it is in the wrong role.",
                    stem
                ),
            ),
            "utility" => (
                format!(
                    "the {} surface '{}' is not imported by any smart surface (command, controller, page, router).",
                    category, stem
                ),
                format!(
                    "Import '{}' by a smart surface (command, controller, page, router) or an entry point. If this surface is dead code, delete the file and its module declaration. Consider moving it to passive (surface _component/_view/_layout) if it is in the wrong role.",
                    stem
                ),
            ),
            "passive" => (
                format!(
                    "the passive surface '{}' is not imported by any smart or utility surface.",
                    stem
                ),
                format!(
                    "Import '{}' by a smart or utility surface. If this surface is dead code, delete the file and its module declaration.",
                    stem
                ),
            ),
            _ => (
                format!(
                    "the unknown surface '{}' is not imported by any appropriate importer.",
                    stem
                ),
                format!(
                    "Import '{}' in an appropriate importer file. If this surface is dead code, delete the file and its module declaration.",
                    stem
                ),
            ),
        };

        OrphanIndicatorResult::new(
            true,
            AesOrphanViolation::SurfaceOrphan {
                category,
                stem: stem.clone(),
                reason: Some(reason_line.into()),
            }
            .to_string()
                + "\nFIX: "
                + &fix_line,
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
