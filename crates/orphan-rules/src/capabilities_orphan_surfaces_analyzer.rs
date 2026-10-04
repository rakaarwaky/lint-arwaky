use shared_common::taxonomy_definition_vo::LayerDefinition;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_orphan_rules::contract_orphan_protocol::ISurfacesOrphanProtocol;
use shared_orphan_rules::utility_orphan_filename::{file_basename, file_stem, file_suffix};
use shared_quality_rules::taxonomy_quality_rules_vo::{
    InboundLinkMap, OrphanIndicatorResult, ReachabilityResult,
};

// ─── Block 1: Struct Definition ────────────────────────────

pub struct SurfacesOrphanAnalyzer;

// ─── Block 2: Protocol Trait Implementation ────────────────

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
        let suffix = file_suffix(&basename);
        let category = Self::surface_category(&suffix);

        // FR-009: Unclassifiable suffixes → skipped (no orphan check)
        if category == "unknown" {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Surface is the outermost layer — only 1 condition: reachability from _entry
        let is_reachable = alive_files.paths.contains(f);
        if is_reachable {
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
            format!("{} surface '{}' is not reachable.", category, stem),
            severity,
        )
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl Default for SurfacesOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SurfacesOrphanAnalyzer {
    pub fn new() -> Self {
        Self
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
