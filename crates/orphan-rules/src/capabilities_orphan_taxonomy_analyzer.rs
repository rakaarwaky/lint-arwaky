use shared_common::taxonomy_definition_vo::LayerDefinition;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_common::utility_layer_detector;
use shared_orphan_rules::contract_orphan_protocol::ITaxonomyOrphanProtocol;
use shared_orphan_rules::utility_orphan_filename::file_stem;
use shared_quality_rules::taxonomy_quality_rules_vo::{
    InboundLinkMap, OrphanIndicatorResult, ReachabilityResult,
};
use std::collections::HashMap;

// ─── Block 1: Struct Definition ────────────────────────────

pub struct TaxonomyOrphanAnalyzer;

// ─── Block 2: Protocol Trait Implementation ────────────────

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
                        let imp_filename =
                            shared_common::utility_layer_detector::extract_filename(importer);
                        imp_filename.starts_with("contract_")
                            || imp_filename.starts_with("capabilities_")
                            || imp_filename.starts_with("agent_")
                            || imp_filename.starts_with("surface_")
                            || imp_filename.starts_with("root_")
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
                        let imp_filename =
                            shared_common::utility_layer_detector::extract_filename(importer);
                        imp_filename.starts_with("contract_")
                            || imp_filename.starts_with("capabilities_")
                            || imp_filename.starts_with("agent_")
                            || imp_filename.starts_with("surface_")
                            || imp_filename.starts_with("root_")
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
                let filename = utility_layer_detector::extract_filename(other_file);
                let is_higher = filename.starts_with("contract_")
                    || filename.starts_with("capabilities_")
                    || filename.starts_with("agent_")
                    || filename.starts_with("surface_")
                    || filename.starts_with("root_");
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
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Build diagnostic message
        let reason = if !is_reachable && !has_higher_layer_importer {
            match &importers {
                None => format!(
                    "AES501 TAXONOMY_ORPHAN: '{}' is a taxonomy file with no importer.\nWHY: A taxonomy file must be imported by a contract or capability to be discoverable.\nFIX: Add an import from a higher-layer file.",
                    stem
                ),
                Some(v) if v.is_empty() => format!(
                    "AES501 TAXONOMY_ORPHAN: '{}' is a taxonomy file with no importer.\nWHY: A taxonomy file must be imported by a contract or capability to be discoverable.\nFIX: Add an import from a higher-layer file.",
                    stem
                ),
                Some(v) => {
                    let low_layer: Vec<String> = v
                        .iter()
                        .filter(|i| *i != f.value())
                        .map(|i| {
                            shared_common::utility_layer_detector::extract_filename(i).to_string()
                        })
                        .collect();
                    format!(
                        "AES501 TAXONOMY_ORPHAN: '{}' is a taxonomy file only imported by lower-layer files ({}).\nWHY: Taxonomy files must be imported by contract, agent, capability, surface, or root layers.\nFIX: Import '{}' from a higher-layer file.",
                        stem,
                        low_layer.join(", "),
                        stem
                    )
                }
            }
        } else if !is_reachable {
            format!(
                "AES501 TAXONOMY_ORPHAN: '{}' is a taxonomy file not reachable from any entry.\nWHY: Taxonomy files must be reachable from the module barrel.\nFIX: Ensure the barrel mod.rs re-exports this file.",
                stem
            )
        } else {
            format!(
                "AES501 TAXONOMY_ORPHAN: '{}' is a taxonomy file not imported by any higher-layer file.\nWHY: Taxonomy files must be imported by contract, agent, capability, surface, or root layers.\nFIX: Add an import from a higher-layer file.",
                stem
            )
        };

        OrphanIndicatorResult::new(true, reason, Severity::LOW)
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

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
