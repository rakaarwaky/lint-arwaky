use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_layer_detector;
use shared::orphan_rules::taxonomy_orphan_parse_result_vo::FileParseResultVO;
use shared::orphan_rules::IUtilityOrphanProtocol;
use shared::quality_rules::taxonomy_analysis_vo::{InboundLinkMap, OrphanIndicatorResult};
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
        match shared::orphan_rules::taxonomy_parser_dispatcher::parse_file_content(
            file_path, content,
        ) {
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
            if !consumer_importers.is_empty() {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        }

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

        if !consumer_importers.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        if !utility_importers.is_empty() {
            let imported_by_str = utility_importers.join(", ");
            return OrphanIndicatorResult::new(
                true,
                format!(
                    "AES504 UTILITY_DEAD_CODE: '{}' has no consumers in capability/agent/surfaces layers.\nWHY? Utility file '{}' is only imported by other utility files ({}), not by capability, agent, or surfaces layers.\nFIX: Import '{}' in a capabilities_* file.",
                    module_name, module_name, imported_by_str, module_name
                ),
                Severity::MEDIUM,
            );
        }

        OrphanIndicatorResult::new(
            true,
            format!(
                "AES504 UTILITY_ORPHAN: '{}' is not imported.\nWHY? Utility file '{}' is not imported by any capabilities or other layer file.\nFIX: Import '{}' in a capabilities_* file.",
                module_name, module_name, module_name
            ),
            Severity::MEDIUM,
        )
    }
}
