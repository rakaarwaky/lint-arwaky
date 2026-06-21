// PURPOSE: DependencyCycleAnalyzer — ICycleAnalysisProtocol for AES205: circular dependency detection
// AES205 rule: Detect circular dependencies between architectural layers.
// Algorithm: Parse all files → extract import modules → detect source & target layers
// → build cross-layer dependency edges → run Floyd-Warshall/Tarjan cycle detection
// → report each cycle edge as a CRITICAL violation.

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_cycle_protocol::ICycleAnalysisProtocol;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::DependencyEdge;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use shared::taxonomy_message_vo::LintMessage;
use std::collections::HashMap;
use std::sync::Arc;

/// Detects circular imports and dependency cycles (Capability) — AES205.
///
/// Workflow:
///   1. Scan receives the full file list and an `IAnalyzer` reference.
///   2. For each file, extract its layer (via filename prefix) and parse all import statements.
///   3. For each import, determine the target layer → build a directed edge (source_layer → target_layer).
///   4. Pass all edges to `detect_cycle_edges` (Tarjan's SCC algorithm internally).
///   5. Every edge that participates in a cycle is reported as a CRITICAL LintResult.
pub struct DependencyCycleAnalyzer {
    _config: ArchitectureConfig,
    parser: Arc<dyn IImportParserPort>,
}

impl DependencyCycleAnalyzer {
    pub fn new(config: ArchitectureConfig, parser: Arc<dyn IImportParserPort>) -> Self {
        Self {
            _config: config,
            parser,
        }
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
    ///   4. Delegate cycle detection to `parser.detect_cycle_edges()` (Tarjan's SCC).
    ///   5. Transform each cycle edge string ("A->B") into a CRITICAL LintResult.
    pub fn scan(
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
            let file_fp = FilePath::new(file.clone()).unwrap_or_default();
            let basename = file_fp.basename();
            if let Some(rule) = aes205_rule {
                if rule.exceptions.values.contains(&basename.to_string()) {
                    continue;
                }
            }

            // Step 3b: Read the raw file content
            let Ok(content) = self.parser.read_file_to_string(&file_fp) else {
                continue;
            };

            // Step 3c: Detect the file's architectural layer
            let file_fp = FilePath::new(file.clone()).unwrap_or_default();
            let file_layer = match analyzer.detect_layer(
                &file_fp,
                &FilePath::new(root_dir.to_string()).unwrap_or_default(),
            ) {
                Some(l) => l.value().to_string(),
                None => continue,
            };

            // Step 3d: Store one representative file path for this layer (for error reporting)
            file_by_layer
                .entry(file_layer.clone())
                .or_insert_with(|| file.clone());

            // Step 3e: Parse every import statement in the file
            let modules = self.parser.extract_import_modules(&content);

            // Step 3f: For each import, resolve its target layer
            for module in modules {
                let module_fp = FilePath::new(module.clone()).unwrap_or_default();
                if let Some(target_layer) = analyzer.detect_module_layer(&module_fp) {
                    let target_layer_str = target_layer.value().to_string();
                    // Step 3g: Only record cross-layer edges (same-layer edges cannot cause cycles)
                    if target_layer_str != file_layer {
                        edges.push(DependencyEdge::new(file_layer.clone(), target_layer_str));
                    }
                }
            }
        }

        // Step 4: Run cycle detection algorithm on the directed graph of layer edges
        let cycle_edge_results = self.parser.detect_cycle_edges(&edges);

        // Step 5: Convert each detected cycle edge into a CRITICAL LintResult
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
}

#[async_trait]
impl ICycleAnalysisProtocol for DependencyCycleAnalyzer {
    /// Adapter: converts ICycleAnalysisProtocol parameters to internal `scan()` format
    /// and appends results into the shared LintResultList.
    ///
    /// Steps:
    ///   1. Convert FilePathList to Vec<String> for the internal scan API.
    ///   2. Call scan() to detect all circular dependency violations.
    ///   3. Extend the shared results list with any found violations.
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
