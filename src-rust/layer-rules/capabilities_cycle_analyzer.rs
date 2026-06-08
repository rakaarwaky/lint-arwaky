// dependency_cycle_analyzer — Detects circular imports and dependency cycles.
// Implements ICycleAnalysisProtocol: scan files for circular import violations.

use crate::shared_common::taxonomy_name_vo::AdapterName;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use /* UNKNOWN: LayerNameVO */ crate::shared_common::taxonomy_layer_vo::LayerNameVO;
use /* UNKNOWN: LineNumber */ crate::shared_common::taxonomy_common_vo::LineNumber;
use /* UNKNOWN: LintMessage */ crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::output_report::taxonomy_result_vo::LintResult;
use /* UNKNOWN: LocationList */ crate::shared_common::taxonomy_lint_vo::LocationList;
use /* UNKNOWN: ScopeRef */ crate::shared_common::taxonomy_lint_vo::ScopeRef;
use crate::output_report::taxonomy_severity_vo::Severity;
use /* UNKNOWN: SymbolName */ crate::naming_rules::taxonomy_symbol_vo::SymbolName;
use std::collections::{HashMap, HashSet};
use std::fs;

/// Represents a single edge in a dependency graph.
pub struct DependencyEdge {
    pub source: String,
    pub target: String,
}

impl DependencyEdge {
    pub fn new(source: String, target: String) -> Self {
        Self { source, target }
    }
}

/// Detects cycles in a directed graph adjacency list.
/// Returns list of "source->target" edge keys that are part of cycles.
pub fn detect_cycle_edges(edges: &[DependencyEdge]) -> Vec<SymbolName> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for e in edges {
        graph
            .entry(e.source.clone())
            .or_default()
            .insert(e.target.clone());
    }

    let mut cycle_edges: Vec<String> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut path_stack: HashSet<String> = HashSet::new();

    fn dfs(
        node: &str,
        graph: &HashMap<String, HashSet<String>>,
        visited: &mut HashSet<String>,
        path_stack: &mut HashSet<String>,
        cycle_edges: &mut Vec<String>,
    ) {
        visited.insert(node.to_string());
        path_stack.insert(node.to_string());

        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if path_stack.contains(neighbor) {
                    cycle_edges.push(format!("{}->{}", node, neighbor));
                } else if !visited.contains(neighbor) {
                    dfs(neighbor, graph, visited, path_stack, cycle_edges);
                }
            }
        }

        path_stack.remove(node);
    }

    let nodes: Vec<String> = graph.keys().cloned().collect();
    for node in nodes {
        if !visited.contains(&node) {
            dfs(
                &node,
                &graph,
                &mut visited,
                &mut path_stack,
                &mut cycle_edges,
            );
        }
    }

    cycle_edges
        .into_iter()
        .map(|s| SymbolName::new(s))
        .collect()
}

/// Detects circular imports and dependency cycles (Capability).
pub struct DependencyCycleAnalyzer {
    config: ArchitectureConfig,
}

impl DependencyCycleAnalyzer {
    pub fn new(config: ArchitectureConfig) -> Self {
        Self { config }
    }

    fn make_result(file: &str, msg: &str) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(1),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw("AES020"),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: Severity::CRITICAL,
            enclosing_scope: Some(ScopeRef {
                name: crate::shared_common::taxonomy_suggestion_vo::DescriptionVO::new(String::new()),
                kind: crate::shared_common::taxonomy_suggestion_vo::DescriptionVO::new(String::new()),
                file: None,
                start_line: None,
                end_line: None,
            }),
            related_locations: LocationList::new(),
        }
    }

    fn extract_import_modules(content: &str) -> Vec<String> {
        let mut modules = Vec::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("from ") {
                if let Some(module) = rest.split_whitespace().next() {
                    modules.push(module.to_string());
                }
            } else if let Some(rest) = trimmed.strip_prefix("import ") {
                if let Some(module) = rest.split_whitespace().next() {
                    modules.push(module.trim_end_matches(',').to_string());
                }
            } else if let Some(rest) = trimmed.strip_prefix("use ") {
                let module = rest.trim_end_matches(';');
                modules.push(module.to_string());
            }
        }
        modules
    }

    fn detect_file_layer(&self, file: &str, root_dir: &str) -> Option<String> {
        let rel = file
            .strip_prefix(root_dir)
            .unwrap_or(file)
            .trim_start_matches('/');
        let mut layers: Vec<(&LayerNameVO, &crate::layer_rules::taxonomy_definition_vo::LayerDefinition)> =
            self.config.layers.iter().collect();
        layers.sort_by(|a, b| b.1.path.value.len().cmp(&a.1.path.value.len()));

        for (name, def) in layers {
            let layer_path = def.path.value.as_str();
            if rel.starts_with(layer_path) {
                return Some(name.value.clone());
            }
        }
        None
    }

    fn detect_module_layer(&self, module: &str) -> Option<String> {
        let parts: Vec<&str> = module.split('.').collect();
        for part in &parts {
            for (name, _def) in &self.config.layers {
                if *part == name.value.as_str() {
                    return Some(name.value.clone());
                }
            }
        }
        None
    }

    pub fn scan(&self, files: &[String], root_dir: &str) -> Vec<LintResult> {
        if !self.config.enabled.value {
            return vec![];
        }

        let mut edges = Vec::new();
        let mut file_by_layer: HashMap<String, String> = HashMap::new();

        for file in files {
            let Ok(content) = fs::read_to_string(file) else {
                continue;
            };
            let file_layer = match self.detect_file_layer(file, root_dir) {
                Some(l) => l,
                None => continue,
            };

            file_by_layer
                .entry(file_layer.clone())
                .or_insert_with(|| file.clone());

            let modules = Self::extract_import_modules(&content);
            for module in modules {
                if let Some(target_layer) = self.detect_module_layer(&module) {
                    if target_layer != file_layer {
                        edges.push(DependencyEdge::new(file_layer.clone(), target_layer));
                    }
                }
            }
        }

        let cycle_edge_results = detect_cycle_edges(&edges);

        cycle_edge_results
            .into_iter()
            .map(|sn| {
                let edge_key = sn.value;
                let parts: Vec<&str> = edge_key.split("->").collect();
                let source = parts[0];
                let target = parts[1];
                let file = file_by_layer
                    .get(source)
                    .cloned()
                    .unwrap_or_else(|| source.to_string());
                let msg = format!(
                    "[AES Circular Import] Dependency cycle detected: '{}' -> '{}'.",
                    source, target
                );
                Self::make_result(&file, &msg)
            })
            .collect()
    }
}
