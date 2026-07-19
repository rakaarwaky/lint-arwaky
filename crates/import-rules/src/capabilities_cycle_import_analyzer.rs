// PURPOSE: CycleImportAnalyzer — ICycleImportProtocol for AES205: circular dependency detection
// AES205 rule: Detect circular dependencies between architectural layers.
// Algorithm: Parse all files → extract import modules → detect source & target layers
// → build cross-layer dependency edges → run DFS 3-color cycle detection
// → report each cycle edge as a CRITICAL violation.

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_name_vo::SymbolName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::taxonomy_cycle_color_vo::Color;
use shared::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::taxonomy_message_vo::LintMessage;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct CycleImportAnalyzer {
    _config: ArchitectureConfig,
    parser: Arc<dyn IImportParserPort>,
}

// ─── Block 2: Public Contract (ICycleImportProtocol) ───

#[async_trait]
impl ICycleImportProtocol for CycleImportAnalyzer {
    fn scan(
        &self,
        analyzer: &dyn ILayerDetectionProtocol,
        files: &[FilePath],
        root_dir: &FilePath,
    ) -> Vec<LintResult> {
        self.do_scan(analyzer, files, root_dir)
    }

    async fn check_cycles(
        &self,
        analyzer: &dyn ILayerDetectionProtocol,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let cycle_violations = self.scan(analyzer, &files.values, root_dir);
        results.values.extend(cycle_violations);
    }

    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName> {
        self.do_detect_cycle_edges(edges)
    }

    fn normalize_to_layer(&self, name: &str) -> LayerNameVO {
        self.do_normalize_to_layer(name)
    }
}

// ─── Block 3: Constructors & Private Helpers ───

impl CycleImportAnalyzer {
    pub fn new(config: ArchitectureConfig, parser: Arc<dyn IImportParserPort>) -> Self {
        Self {
            _config: config,
            parser,
        }
    }

    /// Scan all files for circular dependency cycles (AES205).
    fn do_scan(
        &self,
        analyzer: &dyn ILayerDetectionProtocol,
        files: &[FilePath],
        root_dir: &FilePath,
    ) -> Vec<LintResult> {
        let config = analyzer.config();
        if !config.enabled.value {
            return vec![];
        }

        let aes205_rule = config.rules.iter().find(|r| r.name.value == "AES205");

        let mut edges = Vec::new();
        let mut file_by_layer: HashMap<String, FilePath> = HashMap::new();

        for file in files {
            let basename = file.basename();
            if let Some(rule) = aes205_rule {
                if rule.exceptions.values.contains(&basename.to_string()) {
                    continue;
                }
            }

            let Ok(content_msg) = self.parser.read_file_to_message(file) else {
                continue;
            };
            let content = content_msg.value().to_string();

            let file_layer = match analyzer.detect_layer(file, root_dir) {
                Some(l) => {
                    let s = match l.value.split('(').next() {
                        Some(part) => part,
                        None => &l.value,
                    };
                    s.to_string()
                }
                None => continue,
            };

            let modules = self.parser.extract_import_modules(&content);
            let mut has_cross_layer = false;
            for module in modules {
                let module_value = module.value();
                let is_crate_import = module_value.starts_with("crate::")
                    || module_value.starts_with("lint_arwaky::");
                let layer_prefixes = [
                    "taxonomy_",
                    "contract_",
                    "capabilities_",
                    "infrastructure_",
                    "agent_",
                    "surface_",
                ];
                let layer_names = [
                    "taxonomy",
                    "contract",
                    "capabilities",
                    "infrastructure",
                    "agent",
                    "surface",
                ];
                let is_cross_layer_crate = if is_crate_import {
                    let stripped = module_value
                        .strip_prefix("crate::")
                        .or_else(|| module_value.strip_prefix("lint_arwaky::"))
                        .unwrap_or("");
                    let first_segment = stripped.split("::").next().unwrap_or("");
                    layer_prefixes.iter().any(|p| stripped.starts_with(p))
                        || layer_names.contains(&first_segment)
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
                if let Some(target_layer) = analyzer.detect_module_layer(module_path) {
                    let target_layer_str = match target_layer.value.split('(').next() {
                        Some(part) => part,
                        None => &target_layer.value,
                    }
                    .to_string();
                    if target_layer_str != file_layer {
                        edges.push(DependencyEdge::new(file_layer.clone(), target_layer_str));
                        has_cross_layer = true;
                    }
                }
            }
            if has_cross_layer {
                file_by_layer
                    .entry(file_layer.clone())
                    .or_insert_with(|| file.clone());
            }
        }

        let cycle_edge_results = self.do_detect_cycle_edges(&edges);

        cycle_edge_results
            .into_iter()
            .map(|sn| {
                let edge_key = sn.value;
                let parts: Vec<&str> = edge_key.split("->").collect();
                let source = parts[0];
                let target = parts[1];
                let file = match file_by_layer.get(source) {
                    Some(f) => f.value.clone(),
                    None => source.to_string(),
                };
                LintResult::new_arch(
                    &file,
                    1,
                    "AES205",
                    Severity::CRITICAL,
                    AesImportViolation::CircularImport {
                        reason: Some(LintMessage::new(format!(
                            "Circular dependency between layers '{}' and '{}' creates an implicit bidirectional coupling. \
                             Architectural layers must form a Directed Acyclic Graph (DAG) — every cycle \
                             prevents independent testing, deployment, and reasoning about each layer.",
                            source, target
                        ))),
                    }
                    .to_string(),
                )
            })
            .collect()
    }

    /// Detect cycle edges in a directed graph using DFS 3-coloring.
    fn do_detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName> {
        let normalized_edges: Vec<DependencyEdge> = edges
            .iter()
            .map(|e| {
                DependencyEdge::new(
                    self.do_normalize_to_layer(&e.source).value,
                    self.do_normalize_to_layer(&e.target).value,
                )
            })
            .collect();

        let mut graph: HashMap<LayerNameVO, Vec<LayerNameVO>> = HashMap::new();
        for e in &normalized_edges {
            graph
                .entry(LayerNameVO::new(e.source.clone()))
                .or_default()
                .push(LayerNameVO::new(e.target.clone()));
            graph.entry(LayerNameVO::new(e.target.clone())).or_default();
        }

        let mut color: HashMap<LayerNameVO, Color> = HashMap::new();
        let mut parent: HashMap<LayerNameVO, LayerNameVO> = HashMap::new();
        let mut cycle_edges_set: HashSet<(LayerNameVO, LayerNameVO)> = HashSet::new();

        for node in graph.keys() {
            color.entry(node.clone()).or_insert(Color::White);
        }

        for node in graph.keys().cloned().collect::<Vec<_>>() {
            if color[&node] == Color::White {
                self.dfs_3color(&node, &graph, &mut color, &mut parent, &mut cycle_edges_set);
            }
        }

        let mut unique_cycles: Vec<String> = Vec::new();
        let mut reported: HashSet<String> = HashSet::new();

        for (src, tgt) in &cycle_edges_set {
            let cycle_nodes = self.extract_cycle_nodes(src, tgt, &parent);
            if let Some(cycle) = cycle_nodes {
                let mut sorted_cycle = cycle.clone();
                sorted_cycle.sort_by(|a, b| a.value.cmp(&b.value));
                let dedup_key: String = sorted_cycle
                    .iter()
                    .map(|n| n.value.clone())
                    .collect::<Vec<_>>()
                    .join("->");
                if reported.insert(dedup_key) {
                    for i in 0..cycle.len() {
                        let next = cycle[(i + 1) % cycle.len()].clone();
                        unique_cycles.push(format!("{}->{}", cycle[i].value, next.value));
                    }
                }
            }
        }

        unique_cycles.into_iter().map(SymbolName::new).collect()
    }

    /// Normalize a file/module name to its architectural layer name.
    fn do_normalize_to_layer(&self, name: &str) -> LayerNameVO {
        let layer_prefixes = [
            "taxonomy_",
            "contract_",
            "capabilities_",
            "infrastructure_",
            "agent_",
            "surface_",
        ];
        let base = match name.rsplit('/').next() {
            Some(b) => b,
            None => name,
        };
        for prefix in &layer_prefixes {
            if base.starts_with(prefix) {
                return LayerNameVO::new(prefix.trim_end_matches('_').to_string());
            }
        }
        LayerNameVO::new(name.to_string())
    }

    /// DFS 3-coloring traversal to detect back-edges (cycles).
    fn dfs_3color(
        &self,
        node: &LayerNameVO,
        graph: &HashMap<LayerNameVO, Vec<LayerNameVO>>,
        color: &mut HashMap<LayerNameVO, Color>,
        parent: &mut HashMap<LayerNameVO, LayerNameVO>,
        cycle_edges: &mut HashSet<(LayerNameVO, LayerNameVO)>,
    ) {
        color.insert(node.clone(), Color::Gray);

        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if *color.get(neighbor).unwrap_or(&Color::White) == Color::Gray {
                    cycle_edges.insert((node.clone(), neighbor.clone()));
                } else if *color.get(neighbor).unwrap_or(&Color::White) == Color::White {
                    parent.insert(neighbor.clone(), node.clone());
                    self.dfs_3color(neighbor, graph, color, parent, cycle_edges);
                }
            }
        }

        color.insert(node.clone(), Color::Black);
    }

    /// Extract cycle nodes from source to target using parent tracking.
    fn extract_cycle_nodes(
        &self,
        src: &LayerNameVO,
        tgt: &LayerNameVO,
        parent: &HashMap<LayerNameVO, LayerNameVO>,
    ) -> Option<Vec<LayerNameVO>> {
        let mut path = Vec::new();
        let mut cur = src.clone();
        path.push(cur.clone());

        while cur != *tgt {
            let p = parent.get(&cur)?;
            cur = p.clone();
            path.push(cur.clone());
        }

        path.reverse();
        Some(path)
    }
}
