use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_layer_detector;
use shared::orphan_rules::IUtilityOrphanProtocol;
use shared::orphan_rules::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use shared::quality_rules::taxonomy_analysis_vo::{
    InboundLinkMap, OrphanIndicatorResult, ReachabilityResult,
};
use std::collections::HashMap;

const CONSUMER_LAYERS: &[&str] = &["capabilities", "agent", "surface", "surfaces", "root"];

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

    pub fn is_module_imported(file_path: &str, content: &str, module_name: &str) -> bool {
        match shared::common::parse_file_content(file_path, content) {
            FileParseResultVO::Rust(result) => {
                let in_imports = result.imports.iter().any(|imp| {
                    imp.segments.iter().any(|seg| {
                        seg == module_name || seg.starts_with(&format!("{module_name}_"))
                    })
                });
                let in_usage = result.used_identifiers.iter().any(|id| id == module_name);
                in_imports || in_usage
            }
            FileParseResultVO::Python(result) => result.imports.iter().any(|imp| {
                imp.raw_path.contains(module_name)
                    || imp.segments.iter().any(|seg| seg == module_name)
            }),
            FileParseResultVO::TypeScript(result) => result.imports.iter().any(|imp| {
                imp.raw_path.contains(module_name)
                    || imp.segments.iter().any(|seg| seg == module_name)
            }),
            FileParseResultVO::Unsupported => false,
        }
    }
}

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
        let module_name = match std::path::Path::new(fp)
            .file_stem()
            .and_then(|s| s.to_str())
        {
            Some(name) => name.to_string(),
            None => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        };

        // Condition 1: not reachable from any _entry file
        let is_reachable = alive_files.paths.contains(f);

        // Condition 2: not imported by capabilities/agent/surface
        let mut consumer_importers: Vec<String> = Vec::new();
        let mut utility_importers: Vec<String> = Vec::new();

        if let Some(importers) = inbound_links.get_importers(fp) {
            for importer in importers.iter().filter(|i| *i != fp) {
                let filename = utility_layer_detector::extract_filename(importer);
                let is_consumer = utility_layer_detector::detect_layer_from_prefix(filename)
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
                let filename = utility_layer_detector::extract_filename(other_file);
                let is_consumer = utility_layer_detector::detect_layer_from_prefix(filename)
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

        let has_consumer_importers = !consumer_importers.is_empty();
        let has_any_importers = has_consumer_importers || !utility_importers.is_empty();

        // Both conditions must be satisfied for non-orphan
        if is_reachable && has_consumer_importers {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Build diagnostic message
        let reason = if !is_reachable && !has_any_importers {
            format!(
                "AES504 UTILITY_ORPHAN: '{}' is not reachable and has no importers.\nWHY? Utility file '{}' is not reachable from any _entry file AND not imported by any capabilities_*, agent_*, or surface_* file.\nFIX: Import '{}' from a _entry file AND a consumer layer file.",
                module_name, module_name, module_name
            )
        } else if !is_reachable && !has_consumer_importers {
            let imported_by_str = utility_importers.join(", ");
            format!(
                "AES504 UTILITY_ORPHAN: '{}' is not reachable and not imported by consumer layer.\nWHY? Utility file '{}' is not reachable from any _entry file AND only imported by other utility files ({}).\nFIX: Import '{}' from a _entry file AND a capabilities_* file.",
                module_name, module_name, imported_by_str, module_name
            )
        } else if !is_reachable {
            format!(
                "AES504 UTILITY_ORPHAN: '{}' is not reachable.\nWHY? Utility file '{}' is not reachable from any _entry file.\nFIX: Import '{}' from a _entry file.",
                module_name, module_name, module_name
            )
        } else if !has_any_importers {
            format!(
                "AES504 UTILITY_ORPHAN: '{}' is not imported by any consumer layer.\nWHY? Utility file '{}' is not imported by any capabilities_*, agent_*, or surface_* file.\nFIX: Import '{}' in a capabilities_* file.",
                module_name, module_name, module_name
            )
        } else {
            let imported_by_str = utility_importers.join(", ");
            format!(
                "AES504 UTILITY_ORPHAN: '{}' is not imported by any consumer layer.\nWHY? Utility file '{}' is only imported by other utility files ({}), not by capabilities_*, agent_*, or surface_* files.\nFIX: Import '{}' in a capabilities_* file.",
                module_name, module_name, imported_by_str, module_name
            )
        };

        OrphanIndicatorResult::new(true, reason, Severity::MEDIUM)
    }
}
