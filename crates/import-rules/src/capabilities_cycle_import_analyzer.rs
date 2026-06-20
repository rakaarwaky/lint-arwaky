// PURPOSE: DependencyCycleAnalyzer — ICycleAnalysisProtocol for AES205: circular dependency detection

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_cycle_protocol::ICycleAnalysisProtocol;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_lint_vo::ScopeRef;
use shared::taxonomy_message_vo::LintMessage;
use shared::taxonomy_name_vo::SymbolName;
use shared::taxonomy_suggestion_vo::DescriptionVO;
use std::collections::{HashMap, HashSet};
use std::fs;

fn aes205_circular_import(source: &str, target: &str) -> String {
    format!(
        "AES205 CIRCULAR_IMPORT: Circular dependency detected: '{}' -> '{}'.",
        source, target
    )
}

/// Represents a single edge in a dependency graph (normalized by layer prefix).
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DependencyEdge {
    pub source: String,
    pub target: String,
}

impl DependencyEdge {
    pub fn new(source: String, target: String) -> Self {
        Self {
            source: normalize_to_layer(&source),
            target: normalize_to_layer(&target),
        }
    }
}

/// Normalize a file path or module name to its layer prefix.
/// Files in the same layer (e.g., capabilities_checker, capabilities_analyzer)
/// are grouped under the common layer prefix.
fn normalize_to_layer(name: &str) -> String {
    let layer_prefixes = [
        "taxonomy_",
        "contract_",
        "capabilities_",
        "infrastructure_",
        "agent_",
        "surface_",
    ];
    // Extract the last component (file name or module name)
    let base = name.rsplit('/').next().unwrap_or(name);
    for prefix in &layer_prefixes {
        if base.starts_with(prefix) {
            return prefix.trim_end_matches('_').to_string();
        }
    }
    name.to_string()
}

/// Detect cycles with deduplication. Each unique cycle is reported once.
pub fn detect_cycle_edges(edges: &[DependencyEdge]) -> Vec<SymbolName> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for e in edges {
        graph
            .entry(e.source.clone())
            .or_default()
            .insert(e.target.clone());
    }

    let mut unique_cycles: Vec<String> = Vec::new();
    let mut reported: HashSet<String> = HashSet::new();

    fn dfs_collect_paths(
        node: &str,
        graph: &HashMap<String, HashSet<String>>,
        visited: &mut HashSet<String>,
        path_stack: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        if path_stack.contains(&node.to_string()) {
            // Found a cycle — extract the cycle path
            if let Some(pos) = path_stack.iter().position(|n| n == node) {
                let cycle: Vec<String> = path_stack[pos..].to_vec();
                cycles.push(cycle);
            }
            return;
        }
        if visited.contains(node) {
            return;
        }
        visited.insert(node.to_string());
        path_stack.push(node.to_string());

        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                dfs_collect_paths(neighbor, graph, visited, path_stack, cycles);
            }
        }

        path_stack.pop();
    }

    let nodes: Vec<String> = graph.keys().cloned().collect();
    for node in &nodes {
        let mut local_visited: HashSet<String> = HashSet::new();
        let mut path_stack: Vec<String> = Vec::new();
        let mut cycles: Vec<Vec<String>> = Vec::new();
        dfs_collect_paths(
            node,
            &graph,
            &mut local_visited,
            &mut path_stack,
            &mut cycles,
        );
        for cycle in cycles {
            let mut sorted_cycle = cycle.clone();
            sorted_cycle.sort();
            let dedup_key = sorted_cycle.join("->");
            if reported.insert(dedup_key) {
                // Report the first edge of the cycle
                for i in 0..cycle.len() {
                    let next = cycle[(i + 1) % cycle.len()].clone();
                    unique_cycles.push(format!("{}->{}", cycle[i].clone(), next));
                }
            }
        }
    }

    unique_cycles.into_iter().map(SymbolName::new).collect()
}

/// Detects circular imports and dependency cycles (Capability).
pub struct DependencyCycleAnalyzer {
    _config: ArchitectureConfig,
}

impl DependencyCycleAnalyzer {
    pub fn new(config: ArchitectureConfig) -> Self {
        Self { _config: config }
    }

    fn make_result(file: &str, msg: &str) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(1),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw("AES205"),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: Severity::CRITICAL,
            enclosing_scope: Some(ScopeRef {
                name: DescriptionVO::new(String::new()),
                kind: DescriptionVO::new(String::new()),
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
            } else if trimmed.starts_with("import ") {
                if let Some(pos) = trimmed.rfind(" from ") {
                    let module_part = trimmed[pos + 6..].trim();
                    let cleaned = module_part
                        .trim_end_matches(';')
                        .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                        .trim();
                    modules.push(cleaned.to_string());
                } else if let Some(rest) = trimmed.strip_prefix("import ") {
                    if rest.contains('"') || rest.contains('\'') || rest.contains('`') {
                        let cleaned = rest
                            .trim_end_matches(';')
                            .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                            .trim();
                        modules.push(cleaned.to_string());
                    } else if let Some(first_token) = rest.split_whitespace().next() {
                        modules.push(first_token.trim_end_matches(',').to_string());
                    }
                }
            } else if let Some(rest) = trimmed.strip_prefix("use ") {
                let module = rest.trim_end_matches(';');
                modules.push(module.to_string());
            }
        }
        modules
    }

    pub fn scan(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult> {
        let config = analyzer.config();
        if !config.enabled.value {
            return vec![];
        }

        let aes205_rule = config.rules.iter().find(|r| r.name.value == "AES205");

        let mut edges = Vec::new();
        let mut file_by_layer: HashMap<String, String> = HashMap::new();

        for file in files {
            let file_fp = FilePath::new(file.clone()).unwrap_or_default();
            let basename = file_fp.basename();
            if let Some(rule) = aes205_rule {
                if rule.exceptions.values.contains(&basename.to_string()) {
                    continue;
                }
            }

            let Ok(content) = fs::read_to_string(file) else {
                continue;
            };
            let file_fp = FilePath::new(file.clone()).unwrap_or_default();
            let file_layer = match analyzer.detect_layer(
                &file_fp,
                &FilePath::new(root_dir.to_string()).unwrap_or_default(),
            ) {
                Some(l) => l.value().to_string(),
                None => continue,
            };

            file_by_layer
                .entry(file_layer.clone())
                .or_insert_with(|| file.clone());

            let modules = Self::extract_import_modules(&content);
            for module in modules {
                let module_fp = FilePath::new(module.clone()).unwrap_or_default();
                if let Some(target_layer) = analyzer.detect_module_layer(&module_fp) {
                    let target_layer_str = target_layer.value().to_string();
                    if target_layer_str != file_layer {
                        edges.push(DependencyEdge::new(file_layer.clone(), target_layer_str));
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
                let msg = aes205_circular_import(source, target);
                Self::make_result(&file, &msg)
            })
            .collect()
    }
}

#[async_trait]
impl ICycleAnalysisProtocol for DependencyCycleAnalyzer {
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
}
