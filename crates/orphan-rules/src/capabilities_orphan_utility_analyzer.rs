use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_common::utility_architecture_layer_detector;
use shared_orphan_rules::contract_orphan_protocol::IUtilityOrphanProtocol;
use shared_orphan_rules::taxonomy_orphan_rules_constant::CONSUMER_LAYERS;
use shared_orphan_rules::utility_orphan_paths::summarize_names;
use shared_quality_rules::taxonomy_quality_rules_vo::{
    InboundLinkMap, OrphanIndicatorResult, ReachabilityResult,
};
use std::collections::HashMap;

// ─── Block 1: Module Declaration ────────────────────────────

pub struct UtilityOrphanAnalyzer;

impl Default for UtilityOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
impl UtilityOrphanAnalyzer {
    pub fn new() -> Self {
        Self
    }
}
// ─── Block 2: Module Contract ────────────────

impl IUtilityOrphanProtocol for UtilityOrphanAnalyzer {
    fn is_utility_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        all_files: &[String],
        inbound_links: &InboundLinkMap,
        content_map: &HashMap<String, String>,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult {
        let fp = f.value();
        let content = content_map.get(fp).cloned().unwrap_or_default();
        if content.is_empty() {
            return OrphanIndicatorResult::not_orphan(Severity::LOW);
        }
        let module_name = match std::path::Path::new(fp)
            .file_stem()
            .and_then(|s| s.to_str())
        {
            Some(name) => name.to_string(),
            None => {
                return OrphanIndicatorResult::not_orphan(Severity::LOW);
            }
        };

        // Condition 1: not reachable from any _entry file
        let is_reachable = alive_files.paths.contains(f);

        // Condition 2: not imported by capabilities/agent/surface
        let mut consumer_importers: Vec<String> = Vec::new();
        let mut utility_importers: Vec<String> = Vec::new();

        if let Some(importers) = inbound_links.get_importers(fp) {
            for importer in importers.iter().filter(|i| *i != fp) {
                let filename = utility_architecture_layer_detector::extract_filename(importer);
                let is_consumer =
                    utility_architecture_layer_detector::detect_layer_from_prefix(filename)
                        .map(|layer| CONSUMER_LAYERS.contains(&layer.as_str()))
                        .unwrap_or(false);
                let stem = std::path::Path::new(importer)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                if is_consumer {
                    consumer_importers.push(stem);
                } else {
                    utility_importers.push(stem);
                }
            }
        }

        if consumer_importers.is_empty() {
            for other_file in all_files {
                if other_file == fp {
                    continue;
                }
                let filename = utility_architecture_layer_detector::extract_filename(other_file);
                let is_consumer =
                    utility_architecture_layer_detector::detect_layer_from_prefix(filename)
                        .map(|layer| CONSUMER_LAYERS.contains(&layer.as_str()))
                        .unwrap_or(false);
                if !is_consumer {
                    continue;
                }

                let other_content = content_map.get(other_file).cloned().unwrap_or_default();
                if other_content.is_empty() {
                    continue;
                }

                if Self::is_module_imported(other_file, &other_content, &module_name) {
                    let stem = std::path::Path::new(other_file)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string();
                    consumer_importers.push(stem);
                }
            }
        }

        // Fallback: barrel re-exports — if a sibling mod.rs or __init__.py re-exports this file
        // and the barrel has consumer-layer importers, consider this file as imported.
        if consumer_importers.is_empty() {
            let file_path = std::path::Path::new(fp);
            if let Some(parent) = file_path.parent() {
                // Try mod.rs (Rust) and __init__.py (Python)
                let barrel_names = ["mod.rs", "__init__.py"];
                for barrel in &barrel_names {
                    let barrel_path = parent.join(barrel);
                    if let Some(mod_importers) =
                        inbound_links.get_importers(barrel_path.to_str().unwrap_or(""))
                    {
                        for importer in mod_importers.iter().filter(|i| *i != fp) {
                            let imp_filename =
                                utility_architecture_layer_detector::extract_filename(importer);
                            let is_consumer =
                                utility_architecture_layer_detector::detect_layer_from_prefix(
                                    imp_filename,
                                )
                                .map(|layer| CONSUMER_LAYERS.contains(&layer.as_str()))
                                .unwrap_or(false);
                            if is_consumer {
                                let stem = std::path::Path::new(importer)
                                    .file_stem()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("unknown")
                                    .to_string();
                                consumer_importers.push(stem);
                            }
                        }
                    }
                }
            }
        }

        let has_consumer_importers = !consumer_importers.is_empty();

        // Both conditions must be satisfied for non-orphan
        if is_reachable && has_consumer_importers {
            return OrphanIndicatorResult::not_orphan(Severity::LOW);
        }

        // Build diagnostic message — one sentence ≤72 visible chars, no code prefix, no WHY:/FIX:.
        let imported_by_str = if utility_importers.is_empty() {
            String::new()
        } else {
            format!(" (only by {})", summarize_names(&utility_importers, 3))
        };

        let message = if !is_reachable && !has_consumer_importers {
            format!(
                "'{}' unreachable; no consumer{}.",
                module_name, imported_by_str
            )
        } else if !is_reachable {
            format!("'{}' is not reachable from any entry point.", module_name)
        } else {
            format!(
                "'{}' not imported by consumer layer{}.",
                module_name, imported_by_str
            )
        };

        let why = if !is_reachable && !has_consumer_importers {
            "A utility with no consumer-layer importer and no entry-point reachability is dead code."
                .to_string()
        } else if !is_reachable {
            "An unreachable utility provides no value.".to_string()
        } else {
            "A utility without consumers is dead code.".to_string()
        };

        let fix = if !is_reachable && !has_consumer_importers {
            format!(
                "Import '{}' in a consumer layer or remove the orphaned utility.",
                module_name
            )
        } else if !is_reachable {
            format!(
                "Import '{}' in a surface, capability, agent, or entry file.",
                module_name
            )
        } else {
            format!(
                "Import '{}' in a capabilities, agent, or surface file.",
                module_name
            )
        };

        OrphanIndicatorResult::orphan("UTILITY_ORPHAN", message, why, fix, Severity::MEDIUM)
    }
}

// ─── Block 3: Module Helpers ────────────

impl UtilityOrphanAnalyzer {
    pub fn is_module_imported(_file_path: &str, content: &str, module_name: &str) -> bool {
        content.lines().any(|line| {
            line.contains(&format!("use {module_name}"))
                || line.contains(&format!("from {module_name}"))
        })
    }
}
