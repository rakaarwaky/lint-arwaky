// PURPOSE: DependencyCycleAnalyzer — AES205: circular dependency detection
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_layer_detector;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::{utility_cycle_detector, utility_import_module_parser};
use shared::taxonomy_definition_vo::LayerMapVO;
use shared::taxonomy_layer_vo::LayerNameVO;
use shared::taxonomy_message_vo::LintMessage;
use shared::taxonomy_name_vo::SymbolName;
use std::collections::HashMap;

// ─── Block 1: Struct Definition ───────────────────────────

#[derive(Default)]
pub struct DependencyCycleAnalyzer {}

type ScannedFileEdges = (Vec<DependencyEdge>, Option<(String, String)>);

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl ICycleImportProtocol for DependencyCycleAnalyzer {
    fn scan(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &[FilePath],
        root_dir: &FilePath,
    ) -> Vec<LintResult> {
        let file_strs: Vec<String> = files.iter().map(|f| f.to_string()).collect();
        let root_str = root_dir.to_string();
        self._scan(config, layer_map, &file_strs, &root_str)
    }

    async fn check_cycles(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let file_strs: Vec<String> = files.values.iter().map(|f| f.to_string()).collect();
        let cycle_violations = self._scan(config, layer_map, &file_strs, &root_dir.to_string());
        results.values.extend(cycle_violations);
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
        _root_dir: &str,
    ) -> Vec<LintResult> {
        if !config.enabled.value {
            return vec![];
        }
        let aes205_rule = config.rules.iter().find(|r| r.name.value == "AES205");
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();

        let file_results: Vec<ScannedFileEdges> = files
            .iter()
            .filter_map(|file| {
                let file_fp = FilePath::new(file.clone()).ok()?;
                let basename = file_fp.basename();
                if let Some(rule) = aes205_rule {
                    if rule.exceptions.values.contains(&basename.to_string()) {
                        return None;
                    }
                }
                let content = shared::common::utility_file_handler::read_file_generic(file).ok()?;

                let filename = utility_layer_detector::extract_filename(file);
                let file_layer = match utility_layer_detector::detect_layer_from_prefix(filename) {
                    Some(l) => {
                        let specialized = utility_layer_detector::resolve_specialized_layer(
                            &l,
                            file,
                            &layer_keys,
                        );
                        match specialized.split('(').next() {
                            Some(p) => p.to_string(),
                            None => specialized,
                        }
                    }
                    None => return None,
                };

                let modules = utility_import_module_parser::extract_import_modules(&content);
                let mut local_edges = Vec::new();
                let mut has_cross_layer = false;
                for module in modules {
                    let module_value = module.value();
                    let is_crate_import = module_value.starts_with("crate::")
                        || module_value.starts_with("lint_arwaky::");
                    let is_cross_layer_crate = if is_crate_import {
                        let stripped = module_value
                            .strip_prefix("crate::")
                            .or_else(|| module_value.strip_prefix("lint_arwaky::"))
                            .unwrap_or("");
                        let first_segment = stripped.split("::").next().unwrap_or("");
                        layer_keys.iter().any(|k| {
                            let prefix = format!("{}_", k);
                            stripped.starts_with(&prefix)
                        }) || layer_keys.iter().any(|k| k == first_segment)
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
                    if let Some(target_layer) =
                        utility_layer_detector::detect_module_layer(module_path, &layer_keys)
                    {
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
        let mut file_by_layer: HashMap<String, String> = HashMap::new();
        for (local_edges, layer_mapping) in file_results {
            edges.extend(local_edges);
            if let Some((fl, f)) = layer_mapping {
                file_by_layer.entry(fl).or_insert(f);
            }
        }

        let cycle_edge_results = utility_cycle_detector::detect_cycle_edges(&edges);
        cycle_edge_results.into_iter().map(|sn| {
            let edge_key = sn.value;
            let parts: Vec<&str> = edge_key.split("->").collect();
            let source = parts[0];
            let target = parts[1];
            let file = file_by_layer.get(source).cloned().unwrap_or_else(|| source.to_string());
            LintResult::new_arch(&file, 1, "AES205", Severity::CRITICAL,
                AesImportViolation::CircularImport {
                    reason: Some(LintMessage::new(format!(
                        "Circular dependency between layers '{}' and '{}' creates implicit bidirectional coupling.",
                        source, target
                    ))),
                }.to_string(),
            )
        }).collect()
    }
}
