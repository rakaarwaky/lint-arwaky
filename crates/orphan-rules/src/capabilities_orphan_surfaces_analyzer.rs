use crate::utility_orphan_filename::{file_basename, file_stem, file_suffix};
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::orphan_rules::ISurfacesOrphanProtocol;
use shared::quality_rules::taxonomy_analysis_vo::{
    InboundLinkMap, OrphanIndicatorResult, ReachabilityResult,
};

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

    fn surface_category(suffix: &str) -> &'static str {
        match suffix {
            "command" | "controller" | "page" | "router" => "smart",
            "hook" | "store" | "action" | "screen" => "utility",
            "component" | "view" | "layout" => "passive",
            _ => "unknown",
        }
    }
}

impl ISurfacesOrphanProtocol for SurfacesOrphanAnalyzer {
    fn is_surface_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        alive_files: &ReachabilityResult,
        inbound_links: &InboundLinkMap,
        _definition: Option<&LayerDefinition>,
    ) -> OrphanIndicatorResult {
        let is_reachable = alive_files.paths.contains(f);
        let fp_val = f.value();
        let basename = file_basename(fp_val);
        let stem = file_stem(fp_val);
        let suffix = file_suffix(&basename);
        let category = Self::surface_category(&suffix);

        // FR-009: Unclassifiable suffixes → skipped (no orphan check)
        if category == "unknown" {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        if !is_reachable {
            let severity = match category {
                "smart" => Severity::HIGH,
                "utility" => Severity::MEDIUM,
                "passive" => Severity::LOW,
                _ => Severity::MEDIUM,
            };
            return OrphanIndicatorResult::new(
                true,
                format!(
                    "AES506 SURFACE_ORPHAN: {} surface '{}' is not reachable.\nWHY? Surface '{}' is not reachable from any _entry file.\nFIX: Import '{}' from a _entry file.",
                    category, stem, stem, stem
                ),
                severity,
            );
        }

        // FR-009 surface chain validation using inbound_links
        if let Some(importers) = inbound_links.get_importers(fp_val) {
            let non_self_importers: Vec<&String> =
                importers.iter().filter(|imp| *imp != fp_val).collect();

            if category == "utility" && !non_self_importers.is_empty() {
                let has_valid_consumer = non_self_importers.iter().any(|imp| {
                    let imp_b = file_basename(imp);
                    let imp_suf = file_suffix(&imp_b);
                    let imp_cat = Self::surface_category(&imp_suf);
                    imp_cat == "smart"
                        || imp_b.ends_with("_container.rs")
                        || imp_b.ends_with("_container.py")
                        || imp_b.ends_with("_container.ts")
                        || imp_b.ends_with("_container.js")
                        || imp_b.ends_with("_entry.rs")
                        || imp_b.ends_with("_entry.py")
                        || imp_b.ends_with("_entry.ts")
                        || imp_b.ends_with("_entry.js")
                        || imp_b.starts_with("root_")
                        || imp_b == "main.rs"
                        || imp_b == "main.py"
                        || imp_b == "__main__.py"
                        || imp_b == "main.ts"
                        || imp_b == "main.js"
                        // Barrel files (single source: shared::common::DEFAULT_RULE_EXCEPTIONS)
                        || shared::common::DEFAULT_RULE_EXCEPTIONS.contains(&imp_b.as_str())
                });
                if !has_valid_consumer {
                    return OrphanIndicatorResult::new(
                        true,
                        format!(
                            "AES506 SURFACE_ORPHAN: {} surface '{}' is orphaned.\nWHY? The {} surface '{}' is not imported by any appropriate importer.\nFIX: Import '{}' in an appropriate importer file.",
                            category, stem, category, stem, stem
                        ),
                        Severity::MEDIUM,
                    );
                }
            } else if category == "passive" && !non_self_importers.is_empty() {
                let has_valid_consumer = non_self_importers.iter().any(|imp| {
                    let imp_b = file_basename(imp);
                    let imp_suf = file_suffix(&imp_b);
                    let imp_cat = Self::surface_category(&imp_suf);
                    imp_cat == "smart"
                        || imp_cat == "utility"
                        || imp_b.ends_with("_container.rs")
                        || imp_b.ends_with("_container.py")
                        || imp_b.ends_with("_container.ts")
                        || imp_b.ends_with("_container.js")
                        || imp_b.ends_with("_entry.rs")
                        || imp_b.ends_with("_entry.py")
                        || imp_b.ends_with("_entry.ts")
                        || imp_b.ends_with("_entry.js")
                        || imp_b.starts_with("root_")
                        || imp_b == "main.rs"
                        || imp_b == "main.py"
                        || imp_b == "__main__.py"
                        || imp_b == "main.ts"
                        || imp_b == "main.js"
                        // Barrel files (single source: shared::common::DEFAULT_RULE_EXCEPTIONS)
                        || shared::common::DEFAULT_RULE_EXCEPTIONS.contains(&imp_b.as_str())
                });
                if !has_valid_consumer {
                    return OrphanIndicatorResult::new(
                        true,
                        format!(
                            "AES506 SURFACE_ORPHAN: {} surface '{}' is orphaned.\nWHY? The {} surface '{}' is not imported by any appropriate importer.\nFIX: Import '{}' in an appropriate importer file.",
                            category, stem, category, stem, stem
                        ),
                        Severity::LOW,
                    );
                }
            }
        }

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }
}
