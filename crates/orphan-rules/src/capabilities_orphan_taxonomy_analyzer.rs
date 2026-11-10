use shared_common::taxonomy_layer_vo::LayerDefinition;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_common::utility_architecture_layer_detector::extract_filename;
use shared_orphan_rules::contract_orphan_protocol::ITaxonomyOrphanProtocol;
use shared_orphan_rules::taxonomy_orphan_rules_constant::TAXONOMY_RESCUERS;
use shared_orphan_rules::utility_orphan_filename::file_stem;
use shared_orphan_rules::utility_orphan_paths::summarize_names;
use shared_quality_rules::taxonomy_quality_rules_vo::{
    InboundLinkMap, OrphanIndicatorResult, ReachabilityResult,
};
use std::collections::HashMap;

// ─── Block 1: Module Declaration ────────────────────────────

pub struct TaxonomyOrphanAnalyzer;

impl Default for TaxonomyOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl TaxonomyOrphanAnalyzer {
    pub fn new() -> Self {
        Self
    }
}
// ─── Block 2: Module Contract ────────────────

impl ITaxonomyOrphanProtocol for TaxonomyOrphanAnalyzer {
    fn is_taxonomy_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        _definition: Option<&LayerDefinition>,
        inbound_links: &InboundLinkMap,
        all_files: &[String],
        content_map: &HashMap<String, String>,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult {
        let stem = file_stem(f.value());

        // Condition 1: not reachable from any _entry file
        let is_reachable = alive_files.paths.contains(f);

        // Condition 2: not imported by any contract_ file
        let importers = inbound_links.get_importers(f.value());
        let has_higher_layer_importer = importers
            .as_ref()
            .map(|v| {
                v.iter()
                    .filter(|importer| *importer != f.value())
                    .any(|importer| {
                        let imp_filename = extract_filename(importer);
                        imp_filename.starts_with(TAXONOMY_RESCUERS[0])
                            || imp_filename.starts_with(TAXONOMY_RESCUERS[1])
                            || imp_filename.starts_with(TAXONOMY_RESCUERS[2])
                            || imp_filename.starts_with(TAXONOMY_RESCUERS[3])
                            || imp_filename.starts_with(TAXONOMY_RESCUERS[4])
                    })
            })
            .unwrap_or(false);

        // Fallback: barrel re-exports — if a sibling mod.rs re-exports this file
        // and mod.rs has higher-layer importers, consider this file as imported.
        let has_barrel_importer = if !has_higher_layer_importer {
            let file_path = std::path::Path::new(f.value());
            if let Some(parent) = file_path.parent() {
                let mod_rs = parent.join("mod.rs");
                if let Some(mod_importers) =
                    inbound_links.get_importers(mod_rs.to_str().unwrap_or(""))
                {
                    mod_importers.iter().any(|importer| {
                        let imp_filename = extract_filename(importer);
                        imp_filename.starts_with(TAXONOMY_RESCUERS[0])
                            || imp_filename.starts_with(TAXONOMY_RESCUERS[1])
                            || imp_filename.starts_with(TAXONOMY_RESCUERS[2])
                            || imp_filename.starts_with(TAXONOMY_RESCUERS[3])
                            || imp_filename.starts_with(TAXONOMY_RESCUERS[4])
                    })
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        // Fallback: content-based scan — check if any higher-layer file imports this module
        // or imports from the parent module that re-exports it.
        let has_content_importer = if !has_higher_layer_importer && !has_barrel_importer {
            // Get parent module name for barrel re-export detection
            let parent_module = std::path::Path::new(f.value())
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            all_files.iter().any(|other_file| {
                if other_file == f.value() {
                    return false;
                }
                let filename = extract_filename(other_file);
                let is_higher = TAXONOMY_RESCUERS.iter().any(|p| filename.starts_with(p));
                if !is_higher {
                    return false;
                }
                let content = content_map.get(other_file).cloned().unwrap_or_default();
                // Check direct module name or parent module (barrel re-export)
                content.contains(&stem)
                    || (!parent_module.is_empty()
                        && content.contains(&format!("::{parent_module}")))
            })
        } else {
            false
        };

        // All conditions must be satisfied for non-orphan
        if is_reachable
            && (has_higher_layer_importer || has_barrel_importer || has_content_importer)
        {
            return OrphanIndicatorResult::not_orphan(Severity::LOW);
        }

        // Build diagnostic message — one sentence ≤72 visible chars, no code prefix, no WHY:/FIX:.
        let stem_owned = stem.to_string();
        let message = if !is_reachable && !has_higher_layer_importer {
            format!("'{}' is a taxonomy file with no importer.", stem_owned)
        } else if !is_reachable {
            format!(
                "'{}' is a taxonomy file not reachable from any entry.",
                stem_owned
            )
        } else {
            format!("'{}' is not imported by any higher-layer file.", stem_owned)
        };
        let why = if !is_reachable && !has_higher_layer_importer {
            match &importers {
                None => "A taxonomy file must be imported by a contract or capability to be discoverable."
                    .to_string(),
                Some(v) if v.is_empty() => {
                    "A taxonomy file must be imported by a contract or capability to be discoverable."
                        .to_string()
                }
                Some(v) => {
                    let low_layer: Vec<String> = v
                        .iter()
                        .filter(|i| *i != f.value())
                        .map(|i| extract_filename(i).to_string())
                        .collect();
                    format!(
                        "Taxonomy files must be imported by contract, agent, capability, surface, or root layers; only lower layers import it ({}).",
                        summarize_names(&low_layer, 3)
                    )
                }
            }
        } else if !is_reachable {
            "Taxonomy files must be reachable from the module barrel.".to_string()
        } else {
            "Taxonomy files must be imported by contract, agent, capability, surface, or root layers.".to_string()
        };

        let fix = if !is_reachable && !has_higher_layer_importer {
            "Add an import from a higher-layer file.".to_string()
        } else if !is_reachable {
            "Ensure the barrel mod.rs re-exports this file.".to_string()
        } else {
            "Add an import from a higher-layer file.".to_string()
        };

        OrphanIndicatorResult::orphan("TAXONOMY_ORPHAN", message, why, fix, Severity::LOW)
    }
}

// ─── Block 3: Module Helpers ───
