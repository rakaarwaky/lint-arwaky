// PURPOSE: CycleImportAnalyzer — ICycleImportProtocol for AES205: circular dependency detection
// AES205 rule: Detect circular dependencies between architectural layers.
// Algorithm: Parse all files → extract import modules → detect source & target layers
// → build cross-layer dependency edges → run DFS 3-color cycle detection
// → report each cycle edge as a CRITICAL violation.

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_name_vo::SymbolName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::taxonomy_message_vo::LintMessage;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Unified cycle import analyzer — combines scanning logic and core cycle detection (AES205).
///
/// Workflow:
///   1. Scan receives the full file list and an `IAnalyzer` reference.
///   2. For each file, extract its layer (via filename prefix) and parse all import statements.
///   3. For each import, determine the target layer → build a directed edge (source_layer → target_layer).
///   4. Pass all edges to `detect_cycle_edges` (DFS 3-color algorithm internally).
///   5. Every edge that participates in a cycle is reported as a CRITICAL LintResult.
pub struct CycleImportAnalyzer {
    _config: ArchitectureConfig,
    parser: Arc<dyn IImportParserPort>,
}

/// Color enum for DFS 3-coloring cycle detection.
#[derive(Clone, Copy, PartialEq)]
enum Color {
    White,
    Gray,
    Black,
}                    

#[async_trait]
impl ICycleImportProtocol for CycleImportAnalyzer {
    /// Returns `fp` if `result` is `Ok`, otherwise returns `FilePath::default()`.
    fn pure_filepath_or_default(result: Result<FilePath, impl std::fmt::Debug>) -> FilePath {
        result.unwrap_or_default()
    }

    /// Create a new CycleImportAnalyzer instance.
    pub fn new(config: ArchitectureConfig, parser: Arc<dyn IImportParserPort>) -> Self {
        Self {
            _config: config,
            parser,
        }
    }

    /// Scan all files for circular dependency cycles (AES205).
    fn scan(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult> {
        self.do_scan(analyzer, files, root_dir)
    }

    /// Adapter: converts ICycleImportProtocol parameters to internal `scan()` format
    /// and appends results into the shared LintResultList.
    async fn check_cycles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let file_strs: Vec<String> = files.values.iter().map(|f| f.to_string()).collect();
        let cycle_violations = self.scan(analyzer, &file_strs, &root_dir.to_string());
        results.values.extend(cycle_violations);
    }

    /// Detect cycle edges in a directed graph using DFS 3-coloring.
    fn pure_detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName> {
        self.do_detect_cycle_edges(edges)
    }

    /// Normalize a file/module name to its architectural layer name.
    fn pure_normalize_to_layer(&self, name: &str) -> String {
        self.do_normalize_to_layer(name)
    }

    /// Scan all files for circular dependency cycles (AES205).
    ///
    /// Steps:
    ///   1. Check if the architecture analysis is globally enabled — return empty if disabled.
    ///   2. Locate the AES205 rule config to read exception lists (files to skip).
    ///   3. For each file in the project:
    ///      a. Check if the filename is in the AES205 exception list — skip if yes.
    ///      b. Read file content through the parser port.
    ///      c. Detect the file's architectural layer via filename prefix / path fallback.
    ///      d. Record one representative file path per layer (for error reporting).
    ///      e. Parse all import module paths from the file.
    ///      f. For each import, detect the target layer via module-path analysis.
    ///      g. If the target layer differs from source layer, record a cross-layer edge.
    ///   4. Delegate cycle detection to `do_detect_cycle_edges()` (DFS 3-color SCC).
    ///   5. Transform each cycle edge string into a CRITICAL LintResult.
    fn do_scan(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult> {
        // Step 1: Skip analysis if the architecture checker is globally disabled
        let config = analyzer.config();
        if !config.enabled.value {
            return vec![];
        }

        // Step 2: Find AES205 rule to access exception list (files allowed to have cycles)
        let aes205_rule = config.rules.iter().find(|r| r.name.value == "AES205");

        let mut edges = Vec::new();
        let mut file_by_layer: HashMap<String, String> = HashMap::new();

        // Step 3: Iterate every file in the project
        for file in files {
            // Step 3a: Skip files exempted via rule exceptions
            let file_fp = Self::pure_filepath_or_default(FilePath::new(file.clone()));
            let basename = file_fp.basename();
            if let Some(rule) = aes205_rule {
                if rule.exceptions.values.contains(&basename.to_string()) {
                    continue;
                }
            }

            // Step 3b: Read the raw file content
            let Ok(content_msg) = self.parser.read_file_to_message(&file_fp) else {
                continue;
            };
            let content = content_msg.value().to_string();

            // Step 3c: Detect the file's architectural layer (strip scoped suffix)
            let file_fp = Self::pure_filepath_or_default(FilePath::new(file.clone()));
            let root_dir_fp = Self::pure_filepath_or_default(FilePath::new(root_dir.to_string()));
            let file_layer = match analyzer.detect_layer(&file_fp, &root_dir_fp) {
                Some(l) => {
                    let val = l.value();
                    let s = match val.split('(').next() {
                        Some(part) => part,
                        None => val,
                    };
                    s.to_string()
                }
                None => continue,
            };

            // Step 3e: Parse every import statement in the file
            let modules = self.parser.extract_import_modules(&content);
            // Step 3f: For each import, resolve its target layer (strip scoped suffix)
            let mut has_cross_layer = false;
            for module in modules {
                let module_value = module.value();
                // For crate:: imports, check if the first segment is a layer name
                // (e.g., crate::contract::foo → contract layer = cross-layer)
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
                // Skip crate:: imports that don't reference a layer prefix
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
                let module_fp = Self::pure_filepath_or_default(FilePath::new(module_path.to_string()));
                if let Some(target_layer) = analyzer.detect_module_layer(&module_fp) {
                    let val = target_layer.value();
                    let target_layer_str = match val.split('(').next() {
                        Some(part) => part,
                        None => val,
                    }
                    .to_string();
                    // Step 3g: Only record cross-layer edges (same-layer edges cannot cause cycles)
                    if target_layer_str != file_layer {
                        edges.push(DependencyEdge::new(file_layer.clone(), target_layer_str));
                        has_cross_layer = true;
                    }
                }
            }
            // Step 3d: Only store files that contribute cross-layer edges as representatives
            if has_cross_layer {
                file_by_layer
                    .entry(file_layer.clone())
                    .or_insert_with(|| file.clone());
            }
        }

        // Step 4: Run cycle detection algorithm on the directed graph of layer edges
        let cycle_edge_results = self.do_detect_cycle_edges(&edges);

        // Step 5: Convert each detected cycle edge into a CRITICAL LintResult
        cycle_edge_results
            .into_iter()
            .map(|sn| {
                let edge_key = sn.value;
                let parts: Vec<&str> = edge_key.split("->").collect();
                let source = parts[0];
                let target = parts[1];
                let file = match file_by_layer.get(source) {
                    Some(f) => f.clone(),
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
            .map(|e| DependencyEdge::new(self.do_normalize_to_layer(&e.source), self.do_normalize_to_layer(&e.target)))
            .collect();

        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        for e in &normalized_edges {
            graph
                .entry(e.source.clone())
                .or_default()
                .push(e.target.clone());
            graph.entry(e.target.clone()).or_default();
        }

        let mut color: HashMap<String, Color> = HashMap::new();
        let mut parent: HashMap<String, String> = HashMap::new();
        let mut cycle_edges_set: HashSet<(String, String)> = HashSet::new();

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
                sorted_cycle.sort();
                let dedup_key = sorted_cycle.join("->");
                if reported.insert(dedup_key) {
                    for i in 0..cycle.len() {
                        let next = cycle[(i + 1) % cycle.len()].clone();
                        unique_cycles.push(format!("{}->{}", cycle[i], next));
                    }
                }
            }
        }

        unique_cycles.into_iter().map(SymbolName::new).collect()
    }

    /// Normalize a file/module name to its architectural layer name.
    fn do_normalize_to_layer(&self, name: &str) -> String {
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
                return prefix.trim_end_matches('_').to_string();
            }
        }
        name.to_string()
    }

    /// DFS 3-coloring traversal to detect back-edges (cycles).
    fn dfs_3color(
        &self,
        node: &str,
        graph: &HashMap<String, Vec<String>>,
        color: &mut HashMap<String, Color>,
        parent: &mut HashMap<String, String>,
        cycle_edges: &mut HashSet<(String, String)>,
    ) {
        color.insert(node.to_string(), Color::Gray);

        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if *color.get(neighbor).unwrap_or(&Color::White) == Color::Gray {
                    cycle_edges.insert((node.to_string(), neighbor.clone()));
                } else if *color.get(neighbor).unwrap_or(&Color::White) == Color::White {
                    parent.insert(neighbor.clone(), node.to_string());
                    self.dfs_3color(neighbor, graph, color, parent, cycle_edges);
                }
            }
        }

        color.insert(node.to_string(), Color::Black);
    }

    /// Extract cycle nodes from source to target using parent tracking.
    fn extract_cycle_nodes(
        &self,
        src: &str,
        tgt: &str,
        parent: &HashMap<String, String>,
    ) -> Option<Vec<String>> {
        let mut path = Vec::new();
        let mut cur = src;
        path.push(cur.to_string());

        while cur != tgt {
            let p = parent.get(cur)?;
            cur = p;
            path.push(cur.to_string());
        }

        path.reverse();
        Some(path)
    }
}


