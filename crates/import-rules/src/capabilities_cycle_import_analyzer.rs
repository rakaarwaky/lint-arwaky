// PURPOSE: DependencyCycleAnalyzer — AES205: circular dependency detection
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use shared::cli_commands::LintResult;
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_name_vo::SymbolName;
use shared::common::utility_layer_detector;
use shared::common::{FilePath, FilePathList, Severity};
use shared::filesystem::taxonomy_filesystem_vo::ImportEntry;

use crate::utility_cycle_detector;
use crate::utility_import_module_parser;
use shared::config_system::ArchitectureConfig;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use shared::import_rules::taxonomy_import_error::ImportError;

use std::collections::HashMap;

// ─── Block 1: Struct Definition ───────────────────────────

#[derive(Default)]
pub struct DependencyCycleAnalyzer {}

type ScannedFileEdges = (Vec<DependencyEdge>, Option<(String, String)>);

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ICycleImportProtocol for DependencyCycleAnalyzer {
    fn scan(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &[FilePath],
        root_dir: &FilePath,
        content_map: &HashMap<String, String>,
        imports_map: &HashMap<String, Vec<ImportEntry>>,
    ) -> Vec<LintResult> {
        let file_strs: Vec<String> = files.iter().map(|f| f.to_string()).collect();
        let root_str = root_dir.to_string();
        self._scan(
            config,
            layer_map,
            &file_strs,
            &root_str,
            content_map,
            imports_map,
        )
    }

    fn check_cycles(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        content_map: &HashMap<String, String>,
        imports_map: &HashMap<String, Vec<ImportEntry>>,
    ) -> Result<Vec<LintResult>, ImportError> {
        let file_strs: Vec<String> = files.values.iter().map(|f| f.to_string()).collect();
        let cycle_violations = self._scan(
            config,
            layer_map,
            &file_strs,
            &root_dir.to_string(),
            content_map,
            imports_map,
        );
        Ok(cycle_violations)
    }

    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName> {
        utility_cycle_detector::detect_cycle_edges(edges)
    }

    fn normalize_to_layer(&self, name: &str) -> LayerNameVO {
        LayerNameVO::new(name.split('_').next().unwrap_or(name))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl DependencyCycleAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    fn _scan(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &[String],
        root_dir: &str,
        content_map: &HashMap<String, String>,
        imports_map: &HashMap<String, Vec<ImportEntry>>,
    ) -> Vec<LintResult> {
        if !config.enabled.value {
            return vec![];
        }
        let aes205_rule = config.rules.iter().find(|r| r.name.value == "AES205");
        // Skip if AES205 rule is explicitly disabled
        if let Some(rule) = aes205_rule {
            if !rule.enabled.value {
                return vec![];
            }
        }
        let mut layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();
        // Fallback: if config.layers is empty, use canonical AES layer names.
        // config.layers is only populated when the YAML defines explicit layer overrides;
        // file-prefix detection always works regardless.
        if layer_keys.is_empty() {
            layer_keys = vec![
                "taxonomy".into(),
                "contract".into(),
                "utility".into(),
                "capabilities".into(),
                "agent".into(),
                "surface".into(),
                "root".into(),
            ];
        }
        let layer_prefixes: Vec<String> = layer_keys.iter().map(|k| format!("{}_", k)).collect();

        let file_results: Vec<ScannedFileEdges> =
            ParallelIterator::filter_map(IntoParallelRefIterator::par_iter(files), |file| {
                let file_fp = FilePath::new(file.clone()).ok()?;
                let basename = file_fp.basename();
                if let Some(rule) = aes205_rule
                    && rule.exceptions.values.contains(&basename.to_string())
                {
                    return None;
                }
                content_map.get(file)?;

                let filename = utility_layer_detector::extract_filename(file);
                let file_layer = match utility_layer_detector::detect_layer_from_prefix(filename) {
                    Some(l) => {
                        let specialized = utility_layer_detector::resolve_specialized_layer(
                            &l,
                            file,
                            &layer_keys,
                        );
                        let base_part = specialized
                            .find('(')
                            .map(|i| &specialized[..i])
                            .unwrap_or(&specialized);
                        base_part.to_string()
                    }
                    None => return None,
                };

                // Use ImportEntry from filesystem's AST parser (resolved_path already set)
                let resolved_modules = match imports_map.get(file) {
                    Some(entries) => {
                        utility_import_module_parser::extract_import_modules_from_entries_resolved(
                            entries,
                        )
                    }
                    None => return None,
                };

                let modules: Vec<SymbolName> = resolved_modules
                    .into_iter()
                    .map(|(_, resolved)| resolved)
                    .collect();
                let mut local_edges = Vec::new();
                let mut has_cross_layer = false;
                for module in modules {
                    let module_value = module.value();
                    // Skip empty module paths (e.g. re-export entries parsed with no raw path) —
                    // they cannot represent a real dependency and would falsely resolve to a layer
                    // via the filesystem fallback (resolve_module_path_to_layer).
                    if module_value.is_empty() {
                        continue;
                    }
                    let is_crate_import = module_value.starts_with("crate::")
                        || module_value.starts_with("lint_arwaky::");
                    let is_cross_layer_crate = if is_crate_import {
                        let stripped = module_value
                            .strip_prefix("crate::")
                            .or_else(|| module_value.strip_prefix("lint_arwaky::"))
                            .unwrap_or("");
                        let first_segment = stripped.split("::").next().unwrap_or("");
                        layer_prefixes
                            .iter()
                            .any(|prefix| stripped.starts_with(prefix))
                            || layer_keys.iter().any(|k| k == first_segment)
                    } else {
                        false
                    };
                    if is_crate_import && !is_cross_layer_crate {
                        continue;
                    }
                    let module_path = if is_crate_import {
                        module_value
                            .strip_prefix("crate::")
                            .or_else(|| module_value.strip_prefix("lint_arwaky::"))
                            .unwrap_or(module_value)
                    } else {
                        module_value
                    };
                    let target_layer =
                        utility_layer_detector::detect_module_layer(module_path, &layer_keys)
                            .or_else(|| {
                                utility_layer_detector::resolve_module_path_to_layer(
                                    module_path,
                                    root_dir,
                                )
                            });

                    if let Some(target_layer) = target_layer {
                        let target_layer_str = match target_layer.split('(').next() {
                            Some(p) => p.to_string(),
                            None => target_layer,
                        };
                        if target_layer_str != file_layer {
                            local_edges
                                .push(DependencyEdge::new(file_layer.clone(), target_layer_str));
                            has_cross_layer = true;
                        }
                    }
                }
                let layer_mapping = if has_cross_layer {
                    Some((file_layer, file.clone()))
                } else {
                    None
                };
                Some((local_edges, layer_mapping))
            })
            .collect();

        let mut edges = Vec::new();
        let mut edge_to_file: HashMap<(String, String), String> = HashMap::new();
        for (local_edges, layer_mapping) in file_results {
            if let Some((_, ref f)) = layer_mapping {
                for e in &local_edges {
                    edge_to_file
                        .entry((e.source.clone(), e.target.clone()))
                        .or_insert_with(|| f.clone());
                }
            }
            edges.extend(local_edges);
        }

        let cycle_edge_results = utility_cycle_detector::detect_cycle_edges(&edges);
        cycle_edge_results.into_iter().map(|sn| {
            let edge_key = sn.value;
            let parts: Vec<&str> = edge_key.split("->").collect();
            let source = parts[0];
            let target = parts[1];
            let file = edge_to_file
                .get(&(source.to_string(), target.to_string()))
                .cloned()
                .unwrap_or_else(|| source.to_string());
            LintResult::new_arch(&file, 1, "AES205", Severity::CRITICAL,
                format!(
                    "AES205 CIRCULAR_IMPORT: Circular dependency.\n\
                     WHY? Circular dependency between layers '{}' and '{}' creates implicit bidirectional coupling.\n\
                     FIX: Extract shared types to taxonomy, define a contract protocol, or restructure the dependency direction.",
                    source, target
                ),
            )
        }).collect()
    }
}
