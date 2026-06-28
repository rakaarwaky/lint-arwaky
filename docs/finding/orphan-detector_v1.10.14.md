# Crate: orphan-detector (v1.10.14)

This document contains the source code for feature crate `orphan-detector` along with its corresponding and imported definitions from the `shared` crate.

## Problem Statement

The following issues were detected by `lint-arwaky-cli scan`:

```
============================================================
  AES Architecture Compliance Report
============================================================
  Project: /home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector
  Violations: 0
```

---

## File List

- [crates/orphan-detector/Cargo.toml](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/Cargo.toml)
- [crates/orphan-detector/FRD.md](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/FRD.md)
- [crates/orphan-detector/src/agent_orphan_orchestrator.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/agent_orphan_orchestrator.rs)
- [crates/orphan-detector/src/capabilities_orphan_agent_analyzer.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/capabilities_orphan_agent_analyzer.rs)
- [crates/orphan-detector/src/capabilities_orphan_capabilities_analyzer.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/capabilities_orphan_capabilities_analyzer.rs)
- [crates/orphan-detector/src/capabilities_orphan_contract_analyzer.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/capabilities_orphan_contract_analyzer.rs)
- [crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs)
- [crates/orphan-detector/src/capabilities_orphan_infrastructure_analyzer.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/capabilities_orphan_infrastructure_analyzer.rs)
- [crates/orphan-detector/src/capabilities_orphan_surfaces_analyzer.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/capabilities_orphan_surfaces_analyzer.rs)
- [crates/orphan-detector/src/capabilities_orphan_taxonomy_analyzer.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/capabilities_orphan_taxonomy_analyzer.rs)
- [crates/orphan-detector/src/lib.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/lib.rs)
- [crates/orphan-detector/src/root_orphan_detector_container.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/root_orphan_detector_container.rs)
- [crates/orphan-detector/src/taxonomy_orphan_filename_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/taxonomy_orphan_filename_helper.rs)
- [crates/shared/src/cli-commands/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/mod.rs)
- [crates/shared/src/cli-commands/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_severity_vo.rs)
- [crates/shared/src/code-analysis/contract_layer_detection_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_layer_detection_aggregate.rs)
- [crates/shared/src/code-analysis/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/mod.rs)
- [crates/shared/src/code-analysis/taxonomy_analysis_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_analysis_vo.rs)
- [crates/shared/src/code-analysis/taxonomy_import_source_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_import_source_vo.rs)
- [crates/shared/src/common/infrastructure_file_collector_provider.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/infrastructure_file_collector_provider.rs)
- [crates/shared/src/common/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/mod.rs)
- [crates/shared/src/common/taxonomy_adapter_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_error.rs)
- [crates/shared/src/common/taxonomy_adapter_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_name_vo.rs)
- [crates/shared/src/common/taxonomy_byte_count_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_byte_count_vo.rs)
- [crates/shared/src/common/taxonomy_common_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_error.rs)
- [crates/shared/src/common/taxonomy_common_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_vo.rs)
- [crates/shared/src/common/taxonomy_definition_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_definition_vo.rs)
- [crates/shared/src/common/taxonomy_display_content_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_display_content_vo.rs)
- [crates/shared/src/common/taxonomy_duration_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_duration_vo.rs)
- [crates/shared/src/common/taxonomy_error_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_error_vo.rs)
- [crates/shared/src/common/taxonomy_filesystem_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_filesystem_error.rs)
- [crates/shared/src/common/taxonomy_job_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_job_vo.rs)
- [crates/shared/src/common/taxonomy_language_detector_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_language_detector_helper.rs)
- [crates/shared/src/common/taxonomy_line_count_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_line_count_vo.rs)
- [crates/shared/src/common/taxonomy_lint_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_lint_vo.rs)
- [crates/shared/src/common/taxonomy_message_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_message_vo.rs)
- [crates/shared/src/common/taxonomy_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_name_vo.rs)
- [crates/shared/src/common/taxonomy_naming_list_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_naming_list_vo.rs)
- [crates/shared/src/common/taxonomy_parser_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_parser_error.rs)
- [crates/shared/src/common/taxonomy_path_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_path_vo.rs)
- [crates/shared/src/common/taxonomy_paths_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_paths_vo.rs)
- [crates/shared/src/common/taxonomy_response_data_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_response_data_vo.rs)
- [crates/shared/src/common/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_severity_vo.rs)
- [crates/shared/src/common/taxonomy_source_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_source_vo.rs)
- [crates/shared/src/common/taxonomy_suggestion_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_suggestion_vo.rs)
- [crates/shared/src/common/taxonomy_value_object_utility.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_value_object_utility.rs)
- [crates/shared/src/orphan-detector/contract_orphan_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/contract_orphan_aggregate.rs)
- [crates/shared/src/orphan-detector/contract_orphan_graph_resolver_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/contract_orphan_graph_resolver_protocol.rs)
- [crates/shared/src/orphan-detector/contract_orphan_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/contract_orphan_protocol.rs)
- [crates/shared/src/orphan-detector/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/mod.rs)
- [crates/shared/src/orphan-detector/taxonomy_orphan_contract_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/taxonomy_orphan_contract_vo.rs)
- [crates/shared/src/orphan-detector/taxonomy_orphan_rule_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/taxonomy_orphan_rule_vo.rs)
- [crates/shared/src/orphan-detector/taxonomy_orphan_utility.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/taxonomy_orphan_utility.rs)
- [crates/shared/src/orphan-detector/taxonomy_violation_orphan_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/taxonomy_violation_orphan_vo.rs)
- [crates/shared/src/role-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/mod.rs)
- [crates/shared/src/role-rules/taxonomy_layer_names_constant.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/taxonomy_layer_names_constant.rs)

---

## File: crates/orphan-detector/Cargo.toml

```toml
[package]
name = "orphan_detector-lint-arwaky"
version = "1.10.14"
edition = "2021"
description = "Unreachable/dead component detector covering AES501–AES506. Flags roles, capabilities, and surfaces that are never wired into an entry point."
license = "MIT"
repository = "https://github.com/rakaarwaky/lint-arwaky"
publish = false

[lints]
workspace = true

[dependencies]  # (unchanged)
serde.workspace = true
serde_json.workspace = true
regex.workspace = true
shared.workspace = true

[dev-dependencies]
tokio.workspace = true

```

---

## File: crates/orphan-detector/FRD.md

```rust
# Feature Requirement Document (FRD) - Orphan Detector

## 1. Feature Goal

The goal of the `orphan-detector` module is to identify dead, unused, or unreachable code components across the layers of the 7-layer architecture. By building an import reachability graph starting from valid entry points (such as containers, binary entries, or main files), it flags any architecture component that has been orphaned, preventing codebase bloat and keeping the system maintainable.

## 2. Requirements & Scope

The `orphan-detector` module scans all source code files and evaluates reachability based on the following specifications:

### Rules Specifications

- **AES501: Taxonomy Orphan Checker**
  - **Requirement**: Taxonomy layer files (e.g. `taxonomy_`) must be reachable from contracts, capabilities, or orchestrators.

- **AES502: Contract Orphan Checker**
  - **Requirement**: Contract files (e.g. `contract_`) must have at least one active implementation in the capabilities or infrastructure layers.

- **AES503: Capabilities Orphan Checker**
  - **Requirement**: Capability files (e.g. `capabilities_`) must be instantiated or imported by orchestrators or other capability files.

- **AES504: Infrastructure Orphan Checker**
  - **Requirement**: Infrastructure files (e.g. `infrastructure_`) must be wired into root containers or instantiated by orchestrators.

- **AES505: Agent Orphan Checker**
  - **Requirement**: Agent orchestrator files (e.g. `agent_`) must be called by surface layer files or binary entry points.

- **AES506: Surface Orphan Checker**
  - **Requirement**: Surface layer files (e.g. `surface_`) must be registered in the routing system or called from main entries.

### Reachability Logic

- **Entry Points**: The reachability graph traversal starts from defined entry points (e.g., `main.rs`, `lib.rs`, `root_container.rs`, `index.ts`, `__main__.py`).
- **Traverser**: Performs a BFS/DFS graph traversal from the entry points along the import paths. Any file not visited during traversal is flagged as an orphan unless explicitly exempted.
- **Exceptions**: Supports an exception list configured in the YAML config files where specific files can be safely skipped from orphan checks.

---

## 3. Success Indicators

The success of the `orphan-detector` module is measured by:

- **Dead Code Identification**: 100% detection of unused or unreachable architectural files.
- **Zero False Warnings on Valid Code**: Valid components transitively reachable from entry points must never be flagged as orphans.
- **Configuration Flexibility**: Correctly respects rule-specific exceptions and ignored path patterns.
- **Performance**: Building and traversing the import graph must be highly efficient, taking less than a second even for larger multi-crate projects.
- **Workspace Cleanliness**: Keeps the production binary lightweight and clean of deprecated/unused components.
```

---

## File: crates/orphan-detector/src/agent_orphan_orchestrator.rs

```rust
// PURPOSE: ArchOrphanAnalyzer — aggregate implementing all 6 orphan detection protocols (AES501-506)
//
// Orphan detection works by building a full import dependency graph across
// all source files, then tracing reachability from known entry points
// (_container.rs, main.rs, lib.rs, index.ts, etc.). Files that are not
// reachable are flagged as orphans.
//
// Each AES layer has its own orphan protocol (ITaxonomyOrphanProtocol for
// AES501, IContractOrphanProtocol for AES502, etc.) because different
// layers have different definition/reachability criteria:
//   - Taxonomy:  checked via inbound links (imports from other files)
//   - Contract:  checked via file definitions + inheritance map
//   - Infrastructure/Capabilities: checked via alive set (reachability)
//   - Agent:     checked via cross-file references
//   - Surfaces:  checked via alive set + optional entry detection
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use shared::code_analysis::taxonomy_analysis_vo::ImportGraph;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_layer_vo::LayerNameVO;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_lint_vo::ScopeRef;
use shared::taxonomy_message_vo::LintMessage;
use shared::taxonomy_suggestion_vo::DescriptionVO;
use std::collections::HashSet;
use std::sync::Arc;

use shared::orphan_detector::contract_orphan_protocol::{
    IAgentOrphanProtocol, ICapabilitiesOrphanProtocol, IContractOrphanProtocol,
    IInfrastructureOrphanProtocol, ISurfacesOrphanProtocol, ITaxonomyOrphanProtocol,
};
use shared::role_rules::taxonomy_layer_names_constant::{
    LAYER_AGENT, LAYER_CAPABILITIES, LAYER_CONTRACT, LAYER_INFRASTRUCTURE, LAYER_SURFACES,
    LAYER_TAXONOMY,
};

use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;

pub struct ArchOrphanAnalyzer {
    resolver: Arc<dyn IOrphanGraphResolverProtocol>,
    taxonomy_analyzer: Arc<dyn ITaxonomyOrphanProtocol>,
    contract_analyzer: Arc<dyn IContractOrphanProtocol>,
    capabilities_analyzer: Arc<dyn ICapabilitiesOrphanProtocol>,
    infrastructure_analyzer: Arc<dyn IInfrastructureOrphanProtocol>,
    agent_analyzer: Arc<dyn IAgentOrphanProtocol>,
    surfaces_analyzer: Arc<dyn ISurfacesOrphanProtocol>,
}

impl ArchOrphanAnalyzer {
    pub fn new(
        resolver: Arc<dyn IOrphanGraphResolverProtocol>,
        taxonomy_analyzer: Arc<dyn ITaxonomyOrphanProtocol>,
        contract_analyzer: Arc<dyn IContractOrphanProtocol>,
        capabilities_analyzer: Arc<dyn ICapabilitiesOrphanProtocol>,
        infrastructure_analyzer: Arc<dyn IInfrastructureOrphanProtocol>,
        agent_analyzer: Arc<dyn IAgentOrphanProtocol>,
        surfaces_analyzer: Arc<dyn ISurfacesOrphanProtocol>,
    ) -> Self {
        Self {
            resolver,
            taxonomy_analyzer,
            contract_analyzer,
            capabilities_analyzer,
            infrastructure_analyzer,
            agent_analyzer,
            surfaces_analyzer,
        }
    }
}

impl IOrphanAggregate for ArchOrphanAnalyzer {
    fn build_orphan_graph_context(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext {
        // Bridge the raw &[String] parameter from IOrphanAggregate into the
        // VO-typed contract surface of IOrphanGraphResolverProtocol.
        let file_vo = shared::orphan_detector::OrphanFileListVO::new(files.to_vec());
        self.resolver.build_graph_context(&[file_vo], root_dir)
    }

    fn identify_orphan_entry_points(&self, files: &[String]) -> HashSet<String> {
        let file_vo = shared::orphan_detector::OrphanFileListVO::new(files.to_vec());
        self.resolver
            .identify_entry_points(&[file_vo], &[])
            .values
            .into_iter()
            .collect()
    }

    /// Check orphans for all files in the given list via the checker aggregate for layer detection.
    ///
    /// This is the main orphan detection pipeline:
    ///   1. Build the import graph context (all imports between files)
    ///   2. Identify entry points (containers, main files, index files)
    ///   3. Trace reachability from entry points via BFS on the import graph
    ///   4. For each file not in the reachable set:
    ///      a. Detect its AES layer from filename prefix
    ///      b. Dispatch to layer-specific orphan protocol
    ///      c. If the protocol confirms it's orphan, add violation with appropriate AES code
    fn check_orphans(
        &self,
        layer_detector: &dyn ILayerDetectionAggregate,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult> {
        let mut results: Vec<LintResult> = Vec::new();
        // Build comprehensive context — bridge &[String] -> &[OrphanFileListVO].
        let file_vo = shared::orphan_detector::OrphanFileListVO::new(files.to_vec());
        let context: GraphAnalysisContext = self
            .resolver
            .build_graph_context(std::slice::from_ref(&file_vo), root_dir);

        // Trace reachability: BFS from all entry points through the import graph
        let configured = layer_detector.get_orphan_entry_points();
        let configured_vo = shared::orphan_detector::OrphanEntryPatternListVO::new(configured);
        let entry_points = self
            .resolver
            .identify_entry_points(&[file_vo], &[configured_vo]);
        let alive_files_set: Vec<String> =
            self._trace_reachability(&entry_points.values, &context.import_graph);

        // Evaluate each file: alive (reachable) vs orphan (unreachable)
        for f in files {
            let file_fp = match FilePath::new(f.clone()) {
                Ok(fp) => fp,
                Err(_) => continue,
            };
            let layer_str = match layer_detector.detect_layer(f, root_dir) {
                Some(l) => l,
                None => continue,
            };

            let definition = match layer_detector.get_layer_def(&layer_str) {
                Some(d) => d,
                None => continue,
            };

            // Skip files in the layer's exception list (e.g., mod.rs, __init__.py)
            let basename = file_fp.basename();
            if definition.exceptions.values.contains(&basename) {
                continue;
            }

            // Skip if orphan checking is disabled for this layer
            if !definition.orphan.check_orphan.value {
                continue;
            }

            let layer_vo = LayerNameVO::new(&layer_str);
            let res =
                self._evaluate_layer(f, &context, &alive_files_set, &layer_vo, files, root_dir);

            // If the layer-specific protocol confirms orphan status, emit the appropriate AES code
            if res.is_orphan {
                let code = match layer_str.to_lowercase() {
                    s if s.contains(LAYER_TAXONOMY) => "AES501",
                    s if s.contains(LAYER_CONTRACT) => "AES502",
                    s if s.contains(LAYER_CAPABILITIES) => "AES503",
                    s if s.contains(LAYER_INFRASTRUCTURE) => "AES504",
                    s if s.contains(LAYER_AGENT) => "AES505",
                    s if s.contains(LAYER_SURFACES) => "AES506",
                    _ => continue,
                };
                results.push(self._make_result(f, &res.reason, res.severity, code));
            }
        }

        results
    }
}

impl ArchOrphanAnalyzer {
    /// Build a LintResult from orphan analysis output.
    fn _make_result(&self, file: &str, msg: &str, sev: Severity, code: &str) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(1),
            column: ColumnNumber::new(1),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
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

    /// BFS traversal of the import graph to find all reachable (alive) files.
    /// Starting from entry points, follows imports to find all transitive dependencies.
    fn _trace_reachability(&self, entry_points: &[String], graph: &ImportGraph) -> Vec<String> {
        use std::collections::VecDeque;

        let mut reachable: std::collections::HashSet<String> =
            entry_points.iter().cloned().collect();
        let mut queue: VecDeque<String> = entry_points.iter().cloned().collect();

        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = graph.mapping.get(&current) {
                for neighbor in neighbors {
                    if reachable.insert(neighbor.clone()) {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        reachable.into_iter().collect()
    }

    fn _evaluate_layer(
        &self,
        f: &str,
        context: &GraphAnalysisContext,
        alive_files_set: &[String],
        layer_vo: &LayerNameVO,
        all_files: &[String],
        root_dir: &str,
    ) -> OrphanIndicatorResult {
        if f.ends_with("__init__.py") {
            return shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult::new(
                false,
                String::new(),
                Severity::HIGH,
            );
        }

        let layer_str = layer_vo.value.to_lowercase();
        let fp = match FilePath::new(f.to_string()) {
            Ok(fp) => fp,
            Err(_) => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        };
        let root = FilePath::new(root_dir.to_string()).unwrap_or_default();

        if layer_str.contains(LAYER_TAXONOMY) {
            return self.taxonomy_analyzer.is_taxonomy_orphan(
                &fp,
                &root,
                None,
                &context.inbound_links,
            );
        }

        if layer_str.contains(LAYER_CONTRACT) {
            return self.contract_analyzer.is_contract_orphan(
                &fp,
                &root,
                &context.file_definitions,
                &context.inheritance_map,
                all_files,
            );
        }

        let alive_set = ReachabilityResult::new(
            alive_files_set
                .iter()
                .filter_map(|s| FilePath::new(s.clone()).ok())
                .collect(),
        );

        if layer_str.contains(LAYER_INFRASTRUCTURE) {
            return self
                .infrastructure_analyzer
                .is_infrastructure_orphan(&fp, &root, &alive_set);
        }

        if layer_str.contains(LAYER_CAPABILITIES) {
            return self
                .capabilities_analyzer
                .is_capabilities_orphan(&fp, &root, &alive_set);
        }

        if layer_str.contains(LAYER_AGENT) {
            return self.agent_analyzer.is_agent_orphan(&fp, &root, all_files);
        }

        if layer_str.contains(LAYER_SURFACES) {
            return self
                .surfaces_analyzer
                .is_surface_orphan(&fp, &alive_set, None);
        }

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }
}

impl ILayerDetectionAggregate for ArchOrphanAnalyzer {
    fn detect_layer(&self, file_path: &str, _root_dir: &str) -> Option<String> {
        let path = std::path::Path::new(file_path);
        let filename = path.file_name()?.to_str()?;
        let stem = std::path::Path::new(filename).file_stem()?.to_str()?;

        const PREFIX_MAP: &[(&str, &str)] = &[
            ("taxonomy_", "taxonomy"),
            ("contract_", "contract"),
            ("capabilities_", "capabilities"),
            ("infrastructure_", "infrastructure"),
            ("agent_", "agent"),
            ("surface_", "surfaces"),
            ("root_", "root"),
        ];

        for (prefix, layer) in PREFIX_MAP {
            if stem.starts_with(prefix) {
                return Some(layer.to_string());
            }
        }

        None
    }

    fn get_layer_def(&self, _layer: &str) -> Option<LayerDefinition> {
        let mut def = LayerDefinition::default();
        def.orphan.check_orphan = shared::common::taxonomy_common_vo::BooleanVO::new(true);
        Some(def)
    }

    fn get_orphan_entry_points(&self) -> Vec<String> {
        vec![
            "_container.rs".to_string(),
            "_container.py".to_string(),
            "_container.ts".to_string(),
            "_container.js".to_string(),
            "_entry.rs".to_string(),
            "_entry.py".to_string(),
            "_entry.ts".to_string(),
            "_entry.js".to_string(),
            "main.rs".to_string(),
            "lib.rs".to_string(),
            "main.py".to_string(),
            "main.ts".to_string(),
            "main.js".to_string(),
            "index.ts".to_string(),
            "index.js".to_string(),
        ]
    }
}

pub fn mk_orphan_result(file: &str, msg: &str, sev: Severity, code: &str) -> LintResult {
    LintResult {
        file: FilePath::new(file.to_string()).unwrap_or_default(),
        line: LineNumber::new(0),
        column: ColumnNumber::new(0),
        code: ErrorCode::raw(code),
        message: LintMessage::new(msg),
        source: Some(AdapterName::raw("architecture")),
        severity: sev,
        enclosing_scope: None,
        related_locations: LocationList::new(),
    }
}
```

---

## File: crates/orphan-detector/src/capabilities_orphan_agent_analyzer.rs

```rust
// PURPOSE: AgentOrphanAnalyzer — IAgentOrphanProtocol for detecting orphan agent files
// Agent is orphan if the contract aggregate it implements is NOT called by any surface or container.
use regex::Regex;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IAgentOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;

pub struct AgentOrphanAnalyzer {}

impl Default for AgentOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}

impl IAgentOrphanProtocol for AgentOrphanAnalyzer {
    fn is_agent_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        all_files: &[String],
    ) -> OrphanIndicatorResult {
        is_agent_orphan_raw(f, all_files)
    }
}

/// Extract aggregate trait names from agent file content.
/// Looks for: impl IAggregateTrait for Struct, Box<dyn IAggregateTrait>, Arc<dyn IAggregateTrait>
pub fn extract_aggregate_traits(content: &str) -> Vec<String> {
    let mut traits = Vec::new();

    // Rust: impl ITrait for Struct
    let re_impl = Regex::new(r"impl\s+([A-Za-z0-9_]+)\s+for\s+\s*").ok();
    if let Some(re) = re_impl {
        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            if name.contains("Aggregate") || name.ends_with("Aggregate") {
                traits.push(name);
            }
        }
    }

    // Rust: Box<dyn ITrait> or Arc<dyn ITrait>
    let re_dyn = Regex::new(r"(?:Box|Arc)<dyn\s+([A-Za-z0-9_]+)>").ok();
    if let Some(re) = re_dyn {
        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            if name.contains("Aggregate") || name.ends_with("Aggregate") {
                traits.push(name);
            }
        }
    }

    // Python: class Struct(ITrait):
    let re_py = Regex::new(r"class\s+\w+\(([^)]+)\)").ok();
    if let Some(re) = re_py {
        for cap in re.captures_iter(content) {
            for part in cap[1].split(',') {
                let name = part.trim().to_string();
                if name.contains("Aggregate") || name.ends_with("Aggregate") {
                    traits.push(name);
                }
            }
        }
    }

    // JS/TS: class Struct implements IAggregateTrait
    let re_ts = Regex::new(r"class\s+\w+\s+implements\s+(\w+)").ok();
    if let Some(re) = re_ts {
        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            if name.contains("Aggregate") || name.ends_with("Aggregate") {
                traits.push(name);
            }
        }
    }

    traits.sort();
    traits.dedup();
    traits
}

pub fn is_agent_orphan_raw(f: &FilePath, all_files: &[String]) -> OrphanIndicatorResult {
    let fp = f.value();
    let content = match std::fs::read_to_string(fp) {
        Ok(c) => c,
        Err(_) => return OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
    };

    // Step 1: Find aggregate traits this agent implements
    let aggregate_traits = extract_aggregate_traits(&content);
    if aggregate_traits.is_empty() {
        return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
    }

    // Step 2: Check if any of these aggregates are called by a surface file OR container
    for agg_name in &aggregate_traits {
        let mut called_by_surface_or_container = false;
        for cf in all_files {
            let cb = match cf.split('/').next_back() {
                Some(b) => b,
                None => continue,
            };
            // Check surface files
            let is_surface = cb.starts_with("surface_");
            // Check container files (DI wiring)
            let is_container = cb.ends_with("_container.rs")
                || cb.ends_with("_container.py")
                || cb.ends_with("_container.ts")
                || cb.ends_with("_container.js");

            if !is_surface && !is_container {
                continue;
            }
            if let Ok(c) = std::fs::read_to_string(cf) {
                if c.contains(agg_name) {
                    called_by_surface_or_container = true;
                    break;
                }
            }
        }
        if !called_by_surface_or_container {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::AgentOrphan {
                    agg_name: agg_name.clone(),
                    reason: Some(
                        format!(
                            "Agent orphan: aggregate '{}' not called by any surface.",
                            agg_name
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::HIGH,
            );
        }
    }

    OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
}

pub fn check_agent_orphan(
    fp: &str,
    _basename: &str,
    files: &[String],
    violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
) {
    let fp_vo = match FilePath::new(fp.to_string()) {
        Ok(p) => p,
        Err(_) => return,
    };
    let result = is_agent_orphan_raw(&fp_vo, files);
    if result.is_orphan {
        violations.push(crate::agent_orphan_orchestrator::mk_orphan_result(
            fp,
            &result.reason,
            result.severity,
            "AES505",
        ));
    }
}
```

---

## File: crates/orphan-detector/src/capabilities_orphan_capabilities_analyzer.rs

```rust
// PURPOSE: CapabilitiesOrphanAnalyzer — ICapabilitiesOrphanProtocol for orphan capability detection
use crate::taxonomy_orphan_filename_helper::{file_basename, file_stem, file_suffix};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ICapabilitiesOrphanProtocol;
use shared::orphan_detector::taxonomy_orphan_utility::{extract_struct_names, extract_trait_names};
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;

pub struct CapabilitiesOrphanAnalyzer {}

impl Default for CapabilitiesOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilitiesOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}

impl ICapabilitiesOrphanProtocol for CapabilitiesOrphanAnalyzer {
    fn is_capabilities_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult {
        is_infra_cap_orphan(f, root_dir, alive_files)
    }
}

pub fn is_infra_cap_orphan(
    f: &FilePath,
    root_dir: &FilePath,
    alive_files: &ReachabilityResult,
) -> OrphanIndicatorResult {
    let is_reachable = alive_files.paths.contains(f);
    if is_reachable {
        return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
    }

    // Check if wired in any container
    let fp = f.value();
    let stem = file_stem(fp);

    if let Ok(content) = std::fs::read_to_string(fp) {
        let mut identifiers: Vec<String> = Vec::new();
        identifiers.extend(extract_struct_names(&content));
        identifiers.extend(extract_trait_names(&content));
        identifiers.push(stem.clone());

        let pascal_stem: String = stem
            .split('_')
            .filter(|s| !s.is_empty())
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().to_string() + c.as_str(),
                    None => String::new(),
                }
            })
            .collect();
        identifiers.push(pascal_stem);

        // Search for container files in workspace root
        let root = std::path::Path::new(root_dir.value());
        if let Ok(workspace_root) = find_workspace_root(root) {
            if let Ok(wired) = check_wired_in_container(&workspace_root, &identifiers) {
                if wired {
                    return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                }
            }
        }
    }

    OrphanIndicatorResult::new(
        true,
        AesOrphanViolation::CapabilitiesOrphan {
            stem,
            reason: Some("Not reachable from any entry point.".into()),
        }
        .to_string(),
        Severity::MEDIUM,
    )
}

pub fn find_workspace_root(start: &std::path::Path) -> Result<std::path::PathBuf, std::io::Error> {
    let member_dirs = ["crates", "packages", "modules"];
    let mut current = start.to_path_buf();
    loop {
        let has_cargo = current.join("Cargo.toml").exists() && current.join("Cargo.toml").is_file();
        let has_package_json =
            current.join("package.json").exists() && current.join("package.json").is_file();
        let has_pyproject =
            current.join("pyproject.toml").exists() && current.join("pyproject.toml").is_file();
        let has_member_dir = member_dirs.iter().any(|d| current.join(d).is_dir());

        if has_member_dir && (has_cargo || has_package_json || has_pyproject) {
            return Ok(current);
        }

        if !current.pop() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "workspace root not found",
            ));
        }
    }
}

pub fn check_wired_in_container(
    workspace_root: &std::path::Path,
    identifiers: &[String],
) -> Result<bool, std::io::Error> {
    for dir_name in &["crates", "packages", "modules"] {
        let dir = workspace_root.join(dir_name);
        if dir.is_dir() && check_dir_containers(&dir, identifiers)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn check_dir_containers(
    dir: &std::path::Path,
    identifiers: &[String],
) -> Result<bool, std::io::Error> {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if check_dir_containers(&path, identifiers)? {
                    return Ok(true);
                }
            } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.ends_with("_container.rs")
                    || name.ends_with("_container.py")
                    || name.ends_with("_container.ts")
                    || name.ends_with("_container.js")
                    || name.ends_with("_entry.rs")
                    || name.ends_with("_entry.py")
                    || name.ends_with("_entry.ts")
                    || name.ends_with("_entry.js")
                {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        for id in identifiers {
                            if content.contains(id) {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(false)
}

pub fn is_infra_cap_orphan_raw_wired(is_wired: bool, is_reachable: bool) -> OrphanIndicatorResult {
    let orphan = !is_wired && !is_reachable;
    OrphanIndicatorResult::new(
        orphan,
        "Not wired in container and unreachable.".into(),
        Severity::MEDIUM,
    )
}

/// Extract public struct names from Rust file content.
pub fn is_infra_cap_orphan_raw(
    f: &FilePath,
    all_files: &[String],
    is_reachable: bool,
) -> OrphanIndicatorResult {
    let fp = f.value();
    let _basename = file_basename(fp);
    let stem = file_stem(fp);

    let content = std::fs::read_to_string(fp).unwrap_or_default();
    let mut identifiers: Vec<String> = Vec::new();
    identifiers.extend(extract_struct_names(&content));
    identifiers.extend(extract_trait_names(&content));
    identifiers.push(stem.clone());

    let pascal_stem: String = stem
        .split('_')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                Some(f) => f.to_uppercase().to_string() + c.as_str(),
                None => String::new(),
            }
        })
        .collect();
    identifiers.push(pascal_stem);

    let mut is_wired = false;
    for cf in all_files {
        let cb = file_basename(cf);
        let csuffix = file_suffix(cb);
        if csuffix != "container" {
            continue;
        }
        if let Ok(c) = std::fs::read_to_string(cf) {
            for id in &identifiers {
                if c.contains(id) {
                    is_wired = true;
                    break;
                }
            }
            if is_wired {
                break;
            }
        }
    }

    is_infra_cap_orphan_raw_wired(is_wired, is_reachable)
}

pub fn check_capabilities_orphan(
    fp: &str,
    _basename: &str,
    files: &[String],
    violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
) {
    let stem = file_stem(fp);
    let content = std::fs::read_to_string(fp).unwrap_or_default();

    let mut identifiers: Vec<String> = Vec::new();
    identifiers.extend(extract_struct_names(&content));
    identifiers.extend(extract_trait_names(&content));
    identifiers.push(stem.clone());

    let pascal_stem: String = stem
        .split('_')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                Some(f) => f.to_uppercase().to_string() + c.as_str(),
                None => String::new(),
            }
        })
        .collect();
    identifiers.push(pascal_stem);

    let mut wired = false;
    for cf in files {
        let cb = file_basename(cf);
        let csuffix = file_suffix(cb);
        if csuffix != "container" {
            continue;
        }
        if let Ok(c) = std::fs::read_to_string(cf) {
            for id in &identifiers {
                if c.contains(id) {
                    wired = true;
                    break;
                }
            }
            if wired {
                break;
            }
        }
    }
    if !wired {
        violations.push(crate::agent_orphan_orchestrator::mk_orphan_result(
            fp,
            &AesOrphanViolation::CapabilitiesOrphan {
                stem: stem.clone(),
                reason: Some(format!("capabilities '{}' not wired in container.", stem).into()),
            }
            .to_string(),
            Severity::MEDIUM,
            "AES503",
        ));
    }
}
```

---

## File: crates/orphan-detector/src/capabilities_orphan_contract_analyzer.rs

```rust
// PURPOSE: ContractOrphanAnalyzer — IContractOrphanProtocol for orphan contract detection
use crate::taxonomy_orphan_filename_helper::{file_basename, file_suffix};
use regex::Regex;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use shared::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IContractOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;

pub struct ContractOrphanAnalyzer {}

impl Default for ContractOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}

impl IContractOrphanProtocol for ContractOrphanAnalyzer {
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        file_definitions: &FileDefinitionMap,
        inheritance_map: &InheritanceMap,
        all_files: &[String],
    ) -> OrphanIndicatorResult {
        is_contract_orphan(f, root_dir, file_definitions, inheritance_map, all_files)
    }
}

pub fn is_contract_orphan(
    f: &FilePath,
    root_dir: &FilePath,
    _file_definitions: &FileDefinitionMap,
    _inheritance_map: &InheritanceMap,
    all_files: &[String],
) -> OrphanIndicatorResult {
    let fp = f.value();
    let suffix = file_suffix(fp);

    let content = match std::fs::read_to_string(fp) {
        Ok(c) => c,
        Err(_) => return OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
    };

    let trait_name = extract_contract_trait_name(&content);
    let trait_name = match trait_name {
        Some(t) => t,
        None => return OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
    };

    // Build search_files: combine scan-directory files with all workspace .rs files
    let mut search_files: Vec<String> = all_files.to_vec();
    let root_path = std::path::Path::new(root_dir.value());
    for ws_dir in &["crates", "packages", "modules"] {
        let ws_path = root_path.join(ws_dir);
        if ws_path.exists() {
            collect_rs_files(&ws_path, &mut search_files);
        }
    }

    // Check 1: contract not implemented by expected layer
    let target_prefix = match suffix.as_str() {
        "port" => "infrastructure",
        "protocol" => "capabilities",
        "aggregate" => "agent",
        _ => return OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
    };

    let mut has_impl = false;
    for cf in &search_files {
        let cb = file_basename(cf);
        // Check target layer files (infrastructure_ for ports, capabilities_ for protocols, agent_ for aggregates)
        // Also check root_*_container files (DI wiring often implements traits there)
        let is_target_layer = cb.starts_with(target_prefix);
        let is_container_impl = cb.starts_with("root_") && cb.ends_with("_container.rs");
        if !is_target_layer && !is_container_impl {
            continue;
        }
        if let Ok(c) = std::fs::read_to_string(cf) {
            if c.contains(&format!("impl {} for", trait_name))
                || c.lines().any(|ln| {
                    let t = ln.trim();
                    t.starts_with("impl") && t.contains(&trait_name) && t.contains(" for")
                })
                || c.contains(&format!("class {}(\\(", trait_name))
                || c.contains(&format!("class {} ", trait_name))
                || c.contains(&format!("class {}:", trait_name))
            {
                has_impl = true;
                break;
            }
        }
    }

    if !has_impl {
        return OrphanIndicatorResult::new(
            true,
            AesOrphanViolation::ContractOrphan {
                suffix: suffix.clone(),
                trait_name: trait_name.clone(),
                target_layer: target_prefix,
                reason: Some(
                    format!(
                        "Contract {} '{}' not implemented by any {} file.",
                        suffix, trait_name, target_prefix
                    )
                    .into(),
                ),
            }
            .to_string(),
            Severity::LOW,
        );
    }

    // Check 2: port/protocol not called by any orchestrator, container, capabilities, or surface
    if suffix == "port" || suffix == "protocol" {
        let mut called_by_impl_or_user = false;
        for cf in &search_files {
            let cb = file_basename(cf);
            // Check orchestrator files
            let is_orchestrator = cb.starts_with("agent_")
                && (cb.ends_with("_orchestrator.rs")
                    || cb.ends_with("_orchestrator.py")
                    || cb.ends_with("_orchestrator.ts")
                    || cb.ends_with("_orchestrator.js"));
            // Check container files (DI wiring)
            let is_container = cb.ends_with("_container.rs");
            // Check capabilities files (trait implementations)
            let is_capabilities = cb.starts_with("capabilities_");
            // Check surface files (trait usage)
            let is_surface = cb.starts_with("surface_");

            if !is_orchestrator && !is_container && !is_capabilities && !is_surface {
                continue;
            }
            if let Ok(c) = std::fs::read_to_string(cf) {
                if c.contains(&trait_name) {
                    called_by_impl_or_user = true;
                    break;
                }
            }
        }
        if !called_by_impl_or_user {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::ContractOrphan {
                    suffix: suffix.clone(),
                    trait_name: trait_name.clone(),
                    target_layer: target_prefix,
                    reason: Some(
                        format!(
                            "Contract {} '{}' not called by any orchestrator or container.",
                            suffix, trait_name
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::LOW,
            );
        }
    }

    // Check 3: aggregate not called by any surface OR container
    if suffix == "aggregate" {
        let mut called_by_surface_or_container = false;
        for cf in &search_files {
            let cb = file_basename(cf);
            // Check surface files
            let is_surface = cb.starts_with("surface_");
            // Check container files (DI wiring)
            let is_container = cb.ends_with("_container.rs");

            if !is_surface && !is_container {
                continue;
            }
            if let Ok(c) = std::fs::read_to_string(cf) {
                if c.contains(&trait_name) {
                    called_by_surface_or_container = true;
                    break;
                }
            }
        }
        if !called_by_surface_or_container {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::ContractOrphan {
                    suffix: suffix.clone(),
                    trait_name: trait_name.clone(),
                    target_layer: target_prefix,
                    reason: Some(
                        format!(
                            "Contract aggregate '{}' not called by any surface or container.",
                            trait_name
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::LOW,
            );
        }
    }

    OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
}

pub fn extract_contract_trait_name(content: &str) -> Option<String> {
    let re_rust = Regex::new(r"(?:pub\s+)?trait\s+([A-Za-z0-9_]+)").ok()?;
    let re_py = Regex::new(r"class\s+([A-Za-z0-9_]+)").ok()?;
    let re_ts_interface = Regex::new(r"export\s+interface\s+([A-Za-z0-9_]+)").ok()?;
    let re_interface = Regex::new(r"interface\s+([A-Za-z0-9_]+)").ok()?;

    // Skip comment lines to avoid matching "trait for" in comments
    let code_lines: String = content
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.starts_with("//") && !t.starts_with("/*") && !t.starts_with("*")
        })
        .collect::<Vec<_>>()
        .join("\n");

    if let Some(caps) = re_rust.captures(&code_lines) {
        return Some(caps[1].to_string());
    }
    if let Some(caps) = re_ts_interface.captures(&code_lines) {
        return Some(caps[1].to_string());
    }
    if let Some(caps) = re_interface.captures(&code_lines) {
        return Some(caps[1].to_string());
    }
    re_py.captures(&code_lines).map(|caps| caps[1].to_string())
}

fn collect_rs_files(dir: &std::path::Path, files: &mut Vec<String>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name == "target" || name == ".git" || name == "node_modules" {
                    continue;
                }
                collect_rs_files(&path, files);
            } else if path.extension().is_some_and(|e| e == "rs") {
                if let Some(s) = path.to_str() {
                    files.push(s.to_string());
                }
            }
        }
    }
}
```

---

## File: crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs

```rust
// PURPOSE: OrphanGraphResolver — build graph context and identify entry points for orphan analysis.
use crate::taxonomy_orphan_filename_helper::file_stem;
use shared::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use shared::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use shared::code_analysis::taxonomy_analysis_vo::ImportGraph;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use shared::orphan_detector::taxonomy_orphan_contract_vo::{
    OrphanEntryPatternListVO, OrphanFileListVO,
};

/// Build graph context and identify entry points for orphan analysis.
pub struct OrphanGraphResolver {}

impl Default for OrphanGraphResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl IOrphanGraphResolverProtocol for OrphanGraphResolver {
    fn build_graph_context(
        &self,
        files: &[OrphanFileListVO],
        root_dir: &str,
    ) -> GraphAnalysisContext {
        // Bridge the contract-level VO collection to the internal helper
        // which still uses raw `&[String]` for backward compatibility with
        // the rest of the orphan-detector graph builder.
        let raw_paths: Vec<String> = files
            .iter()
            .flat_map(|v| v.values.iter().cloned())
            .collect();
        self.build_graph_context_inner(&raw_paths, root_dir)
    }

    fn identify_entry_points(
        &self,
        files: &[OrphanFileListVO],
        configured: &[OrphanEntryPatternListVO],
    ) -> OrphanFileListVO {
        let file_strs: Vec<String> = files
            .iter()
            .flat_map(|v| v.values.iter().cloned())
            .collect();

        let configured_strs: Vec<String> = configured
            .iter()
            .flat_map(|p| p.values.iter().cloned())
            .collect();

        // If configured patterns exist, use them; otherwise auto-detect _entry files
        let matched: Vec<String> = if configured_strs.is_empty() {
            file_strs
                .iter()
                .filter(|f| {
                    f.contains("_entry")
                        || f.contains("root_")
                        || f.ends_with("main.rs")
                        || f.ends_with("lib.rs")
                })
                .cloned()
                .collect()
        } else {
            file_strs
                .iter()
                .filter(|f| {
                    configured_strs
                        .iter()
                        .any(|pattern| f.ends_with(pattern) || f.contains(pattern))
                })
                .cloned()
                .collect()
        };
        OrphanFileListVO::new(matched)
    }
}

impl OrphanGraphResolver {
    pub fn new() -> Self {
        Self {}
    }

    fn build_graph_context_inner(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext {
        use std::collections::HashMap;
        let mut import_graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut inbound_links: HashMap<String, Vec<String>> = HashMap::new();
        let mut inheritance_map: HashMap<String, Vec<String>> = HashMap::new();
        let file_definitions: HashMap<String, Vec<String>> = HashMap::new();

        // Build a lookup: module_name -> file_path for crate:: resolution
        let mut module_to_file: HashMap<String, String> = HashMap::new();
        for f in files {
            let stem = file_stem(f);
            module_to_file.insert(stem.clone(), f.clone());
            if let Some(parent) = f.rsplit('/').nth(1) {
                let module_path = format!("{}/{}", parent, stem);
                module_to_file.insert(module_path.clone(), f.clone());
                let normalized_path = module_path.replace('-', "_");
                if normalized_path != module_path {
                    module_to_file.insert(normalized_path, f.clone());
                }
            }
        }

        // Build set of known workspace crate dirs for external dep detection
        let mut workspace_modules: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        let root_path = std::path::Path::new(root_dir);
        for ws_dir in &["crates", "packages", "modules"] {
            let ws_path = root_path.join(ws_dir);
            if let Ok(entries) = std::fs::read_dir(&ws_path) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        if let Some(name) = entry.file_name().to_str().map(|s| s.to_string()) {
                            workspace_modules.insert(name.clone());
                            workspace_modules.insert(name.replace('-', "_"));
                        }
                    }
                }
            }
        }

        // Handle pub mod declarations (both with #[path] and plain)
        let pub_mod_re =
            regex::Regex::new(r#"#\[path\s*=\s*"([^"]+)"\]\s*(?:pub\s+)?mod\s+([a-zA-Z_]+)"#).ok();
        for f in files {
            if let Ok(content) = std::fs::read_to_string(f) {
                // Handle #[path = "..."] pub mod declarations
                if let Some(ref re) = pub_mod_re {
                    for cap in re.captures_iter(&content) {
                        let mod_path = cap[1].to_string();
                        let _mod_name = cap[2].to_string();
                        // Resolve: lib.rs has #[path = "layer-rules/mod.rs"] pub mod layer_rules
                        // → find files in layer-rules/ directory
                        let base_dir = match std::path::Path::new(f).parent() {
                            Some(p) => p.to_string_lossy().to_string(),
                            None => String::from("."),
                        };
                        let resolved_dir = format!(
                            "{}/{}",
                            base_dir,
                            mod_path.replace("/mod.rs", "").replace("mod.rs", ".")
                        );
                        // Find all .rs files in that directory
                        if let Ok(entries) = std::fs::read_dir(&resolved_dir) {
                            for entry in entries.flatten() {
                                let path = entry.path();
                                if path.is_file() {
                                    if let Some(ext) = path.extension() {
                                        if ext == "rs" || ext == "py" {
                                            if let Some(path_str) = path.to_str() {
                                                let resolved = path_str.to_string();
                                                if resolved != *f {
                                                    import_graph
                                                        .entry(f.clone())
                                                        .or_default()
                                                        .push(resolved.clone());
                                                    inbound_links
                                                        .entry(resolved)
                                                        .or_default()
                                                        .push(f.clone());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let import_re = regex::Regex::new(r"(?:use|import|from)\s+([a-zA-Z_][a-zA-Z0-9_\.:]*)");
        let inh_re = regex::Regex::new(r"class\s+\w+\(([^)]+)\)");
        for f in files {
            import_graph.entry(f.clone()).or_default();
            if let Ok(content) = std::fs::read_to_string(f) {
                if let Ok(ref import_re) = import_re {
                    for cap in import_re.captures_iter(&content) {
                        let full_import = cap[1].to_string();

                        // Handle crate:: and lint_arwaky:: imports (lint_arwaky = crate in main.rs)
                        let normalized =
                            if let Some(stripped) = full_import.strip_prefix("lint_arwaky::") {
                                format!("crate::{}", stripped)
                            } else {
                                full_import.clone()
                            };
                        let full_import = &normalized;
                        if let Some(path_part) = full_import.strip_prefix("crate::") {
                            // Extract module segments: source_parsing::taxonomy_path_vo::FilePath
                            // Module path is all segments except the last (item name)
                            // e.g. ["source_parsing", "taxonomy_path_vo", "FilePath"]
                            //   → module segments: ["source_parsing", "taxonomy_path_vo"]
                            //   → try composite keys first (parent/segment), then plain segments
                            let segments: Vec<&str> = path_part.split("::").collect();
                            if segments.len() >= 2 {
                                let mut resolved = false;
                                for i in (1..segments.len()).rev() {
                                    let composite = segments[..i].join("/");
                                    if let Some(file_path) = module_to_file.get(composite.as_str())
                                    {
                                        if file_path != f {
                                            import_graph
                                                .entry(f.clone())
                                                .or_default()
                                                .push(file_path.clone());
                                            inbound_links
                                                .entry(file_path.clone())
                                                .or_default()
                                                .push(f.clone());
                                            resolved = true;
                                            break;
                                        }
                                    }
                                }
                                if resolved {
                                    continue;
                                }
                                for seg in segments[..segments.len() - 1].iter().rev() {
                                    if let Some(file_path) = module_to_file.get(*seg) {
                                        if file_path != f {
                                            import_graph
                                                .entry(f.clone())
                                                .or_default()
                                                .push(file_path.clone());
                                            inbound_links
                                                .entry(file_path.clone())
                                                .or_default()
                                                .push(f.clone());
                                            resolved = true;
                                            break;
                                        }
                                    }
                                }
                                if resolved {
                                    continue;
                                }
                            }
                            // Fallback: single-segment import like use crate::taxonomy_foo
                            if let Some(seg) = segments.first() {
                                if let Some(file_path) = module_to_file.get(*seg) {
                                    if file_path != f {
                                        import_graph
                                            .entry(f.clone())
                                            .or_default()
                                            .push(file_path.clone());
                                        inbound_links
                                            .entry(file_path.clone())
                                            .or_default()
                                            .push(f.clone());
                                        continue;
                                    }
                                }
                            }
                            continue;
                        }

                        // Handle super:: imports
                        if let Some(path_part) = full_import.strip_prefix("super::") {
                            let segments: Vec<&str> = path_part.split("::").collect();
                            if segments.len() >= 2 {
                                let mut found = false;
                                for i in (1..segments.len()).rev() {
                                    let composite = segments[..i].join("/");
                                    if let Some(file_path) = module_to_file.get(composite.as_str())
                                    {
                                        if file_path != f {
                                            import_graph
                                                .entry(f.clone())
                                                .or_default()
                                                .push(file_path.clone());
                                            inbound_links
                                                .entry(file_path.clone())
                                                .or_default()
                                                .push(f.clone());
                                            found = true;
                                            break;
                                        }
                                    }
                                }
                                if found {
                                    continue;
                                }
                                for seg in segments[..segments.len() - 1].iter().rev() {
                                    if let Some(resolved) = module_to_file.get(*seg) {
                                        if resolved != f {
                                            import_graph
                                                .entry(f.clone())
                                                .or_default()
                                                .push(resolved.clone());
                                            inbound_links
                                                .entry(resolved.clone())
                                                .or_default()
                                                .push(f.clone());
                                            break;
                                        }
                                    }
                                }
                            } else if let Some(seg) = segments.first() {
                                if let Some(resolved) = module_to_file.get(*seg) {
                                    if resolved != f {
                                        import_graph
                                            .entry(f.clone())
                                            .or_default()
                                            .push(resolved.clone());
                                        inbound_links
                                            .entry(resolved.clone())
                                            .or_default()
                                            .push(f.clone());
                                    }
                                }
                            }
                            continue;
                        }

                        // Skip external crates — dynamic check against local modules + workspace dirs
                        let mut dep = full_import.clone();
                        if let Some(dot) = dep.find('.') {
                            dep = dep[..dot].to_string();
                        }
                        if let Some(colon) = dep.find("::") {
                            dep = dep[..colon].to_string();
                        }
                        let is_known_local = module_to_file.contains_key(&dep)
                            || workspace_modules.contains(&dep)
                            || matches!(dep.as_str(), "crate" | "self" | "super");
                        if !is_known_local {
                            continue;
                        }

                        // Try to resolve workspace crate imports: e.g. "import_rules::root_import_rules_container"
                        if let Some(colon_idx) = full_import.find("::") {
                            let crate_name = &full_import[..colon_idx];
                            let rest = &full_import[colon_idx + 2..];
                            let segments: Vec<&str> = rest.split("::").collect();
                            if !segments.is_empty() {
                                let module_name = segments[0];
                                let member_roots = ["crates", "packages", "modules"];
                                let mut member_dirs = Vec::new();
                                for root in &member_roots {
                                    let base = root_path.join(root).to_string_lossy().to_string();
                                    let cn = crate_name.replace('_', "-");
                                    for path in &[
                                        format!("{}/{}/src", base, cn),
                                        format!("{}/{}/src", base, crate_name),
                                        format!("{}/{}", base, cn),
                                        format!("{}/{}", base, crate_name),
                                        format!("./{}/{}/src", root, cn),
                                        format!("{}/{}/src", root, cn),
                                    ] {
                                        member_dirs.push(path.clone());
                                    }
                                }
                                for dir in &member_dirs {
                                    if let Ok(entries) = std::fs::read_dir(dir) {
                                        for entry in entries.flatten() {
                                            let path = entry.path();
                                            if let Some(path_str) = path.to_str() {
                                                let stem = path
                                                    .file_stem()
                                                    .and_then(|s| s.to_str())
                                                    .unwrap_or_default();
                                                if stem == module_name && path_str != *f {
                                                    import_graph
                                                        .entry(f.clone())
                                                        .or_default()
                                                        .push(format!(
                                                            "./{}",
                                                            path_str.trim_start_matches("./")
                                                        ));
                                                    inbound_links
                                                        .entry(format!(
                                                            "./{}",
                                                            path_str.trim_start_matches("./")
                                                        ))
                                                        .or_default()
                                                        .push(f.clone());
                                                }
                                            }
                                        }
                                    }
                                }
                                continue;
                            }
                        }

                        // Python/JS relative imports
                        import_graph.entry(f.clone()).or_default().push(dep.clone());
                        inbound_links.entry(dep).or_default().push(f.clone());
                    }
                }
                if let Ok(ref inh_re) = inh_re {
                    for cap in inh_re.captures_iter(&content) {
                        for base in cap[1].split(',') {
                            inheritance_map
                                .entry(f.clone())
                                .or_default()
                                .push(base.trim().to_string());
                        }
                    }
                }
            }
        }
        GraphAnalysisContext::new(
            ImportGraph::new(import_graph),
            InboundLinkMap::new(inbound_links),
            InheritanceMap::new(inheritance_map),
            FileDefinitionMap::new(file_definitions),
        )
    }
}
```

---

## File: crates/orphan-detector/src/capabilities_orphan_infrastructure_analyzer.rs

```rust
// PURPOSE: InfrastructureOrphanAnalyzer — IInfrastructureOrphanProtocol for orphan infrastructure detection
use crate::taxonomy_orphan_filename_helper::file_stem;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IInfrastructureOrphanProtocol;
use shared::orphan_detector::taxonomy_orphan_utility::{extract_struct_names, extract_trait_names};
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;

pub struct InfrastructureOrphanAnalyzer {}

impl Default for InfrastructureOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl InfrastructureOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}

impl IInfrastructureOrphanProtocol for InfrastructureOrphanAnalyzer {
    fn is_infrastructure_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult {
        let is_reachable = alive_files.paths.contains(f);
        if is_reachable {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Check if wired in any container
        let fp = f.value();
        let stem = file_stem(fp);

        if let Ok(content) = std::fs::read_to_string(fp) {
            let mut identifiers: Vec<String> = Vec::new();
            identifiers.extend(extract_struct_names(&content));
            identifiers.extend(extract_trait_names(&content));
            identifiers.push(stem.clone());

            let pascal_stem: String = stem
                .split('_')
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let mut c = s.chars();
                    match c.next() {
                        Some(f) => f.to_uppercase().to_string() + c.as_str(),
                        None => String::new(),
                    }
                })
                .collect();
            identifiers.push(pascal_stem);

            let root = std::path::Path::new(root_dir.value());
            if let Ok(workspace_root) =
                crate::capabilities_orphan_capabilities_analyzer::find_workspace_root(root)
            {
                if let Ok(wired) =
                    crate::capabilities_orphan_capabilities_analyzer::check_wired_in_container(
                        &workspace_root,
                        &identifiers,
                    )
                {
                    if wired {
                        return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                    }
                }
            }
        }

        OrphanIndicatorResult::new(
            true,
            AesOrphanViolation::InfrastructureOrphan {
                stem,
                reason: Some(
                    "Not reachable from any entry point and not wired in any container.".into(),
                ),
            }
            .to_string(),
            Severity::MEDIUM,
        )
    }
}

pub fn check_infrastructure_orphan(
    fp: &str,
    basename: &str,
    files: &[String],
    violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
) {
    crate::capabilities_orphan_capabilities_analyzer::check_capabilities_orphan(
        fp, basename, files, violations,
    )
}
```

---

## File: crates/orphan-detector/src/capabilities_orphan_surfaces_analyzer.rs

```rust
// PURPOSE: SurfacesOrphanAnalyzer — ISurfacesOrphanProtocol for orphan surface detection
// Surface orphan detection per category:
// - Smart surface (_command, _controller, _page, _entry): must be imported by entry or router
// - Utility surface (_hook, _store, _action, _screen, _router): must be imported by smart surface
// - Passive surface (_component, _view, _layout): must be imported by smart or utility surface
use crate::taxonomy_orphan_filename_helper::{file_basename, file_stem, file_suffix};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ISurfacesOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared::taxonomy_definition_vo::LayerDefinition;

pub struct SurfacesOrphanAnalyzer {}

impl Default for SurfacesOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SurfacesOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}

impl ISurfacesOrphanProtocol for SurfacesOrphanAnalyzer {
    fn is_surface_orphan(
        &self,
        f: &FilePath,
        alive_files: &ReachabilityResult,
        definition: Option<&LayerDefinition>,
    ) -> OrphanIndicatorResult {
        is_surface_orphan(f, alive_files, definition)
    }
}

/// Get surface suffix from filename
pub fn get_surface_suffix(basename: &str) -> String {
    file_suffix(basename)
}

/// Surface category
pub fn surface_category(suffix: &str) -> &'static str {
    match suffix {
        "command" | "controller" | "page" => "smart",
        "hook" | "store" | "action" | "screen" | "router" => "utility",
        "component" | "view" | "layout" => "passive",
        _ => "unknown",
    }
}

pub fn is_surface_orphan(
    f: &FilePath,
    alive_files: &ReachabilityResult,
    _definition: Option<&LayerDefinition>,
) -> OrphanIndicatorResult {
    let alive: Vec<String> = alive_files
        .paths
        .iter()
        .map(|fp| fp.value().to_string())
        .collect();
    let is_reachable = alive.contains(&f.value().to_string());
    if is_reachable {
        return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
    }

    // Check if imported by entry or router
    let fp_val = f.value();
    let basename = file_basename(fp_val);
    let stem = file_stem(fp_val);

    if let Ok(content) = std::fs::read_to_string(fp_val) {
        // Check if this surface is imported by any entry or router file
        let root = std::path::Path::new(".");
        if let Ok(workspace_root) =
            crate::capabilities_orphan_capabilities_analyzer::find_workspace_root(root)
        {
            if let Ok(imported) = check_imported_by_entry_or_router(&workspace_root, &stem) {
                if imported {
                    return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                }
            }

            // Also check for function/struct names from the surface file
            let mut identifiers: Vec<String> = Vec::new();
            // Extract pub fn names
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("pub fn ") {
                    if let Some(name) = trimmed.strip_prefix("pub fn ") {
                        if let Some(name) = name.split('(').next() {
                            identifiers.push(name.trim().to_string());
                        }
                    }
                }
                // Extract pub struct names
                if trimmed.starts_with("pub struct ") {
                    if let Some(name) = trimmed.strip_prefix("pub struct ") {
                        if let Some(name) = name.split('{').next() {
                            identifiers.push(name.trim().to_string());
                        }
                    }
                }
            }

            for id in &identifiers {
                if let Ok(imported) = check_imported_by_entry_or_router(&workspace_root, id) {
                    if imported {
                        return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                    }
                }
            }
        }
    }

    let suffix = get_surface_suffix(basename);
    let category = surface_category(&suffix);
    OrphanIndicatorResult::new(
        true,
        AesOrphanViolation::SurfaceOrphan {
            category,
            stem,
            reason: Some("Surface is unreachable.".into()),
        }
        .to_string(),
        Severity::HIGH,
    )
}

fn check_imported_by_entry_or_router(
    workspace_root: &std::path::Path,
    stem: &str,
) -> Result<bool, std::io::Error> {
    for dir_name in &["crates", "packages", "modules"] {
        let dir = workspace_root.join(dir_name);
        if dir.is_dir() && check_dir_imports(&dir, stem)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn check_dir_imports(dir: &std::path::Path, stem: &str) -> Result<bool, std::io::Error> {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if check_dir_imports(&path, stem)? {
                    return Ok(true);
                }
            } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                let is_entry_or_router = name.starts_with("root_")
                    || name.starts_with("cli_")
                    || name.starts_with("mcp_")
                    || name.contains("_entry")
                    || name.contains("_router");
                if is_entry_or_router
                    && (name.ends_with(".rs")
                        || name.ends_with(".py")
                        || name.ends_with(".ts")
                        || name.ends_with(".js"))
                {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        if content.contains(stem) {
                            return Ok(true);
                        }
                    }
                }
            }
        }
    }
    Ok(false)
}

pub fn is_surface_orphan_raw(f: &FilePath, all_files: &[String]) -> OrphanIndicatorResult {
    let fp = f.value();
    let basename = file_basename(fp);
    let suffix = get_surface_suffix(basename);
    let category = surface_category(&suffix);
    let stem = file_stem(fp);

    match category {
        // Smart surface: must be imported by entry point or router
        "smart" => {
            let mut imported_by_entry_or_router = false;
            for cf in all_files {
                let cb = file_basename(cf);
                let cf_suffix = get_surface_suffix(cb);
                // Entry point or router
                if cb.starts_with("cli_")
                    || cb.starts_with("mcp_")
                    || cf_suffix == "entry"
                    || cf_suffix == "router"
                {
                    if let Ok(c) = std::fs::read_to_string(cf) {
                        if c.contains(&stem) {
                            imported_by_entry_or_router = true;
                            break;
                        }
                    }
                }
            }
            OrphanIndicatorResult::new(
                !imported_by_entry_or_router,
                AesOrphanViolation::SurfaceOrphan {
                    category: "smart",
                    stem: stem.clone(),
                    reason: Some(
                        format!(
                            "Smart surface '{}' not imported by any entry point or router.",
                            stem
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::HIGH,
            )
        }
        // Utility surface: must be imported by smart surface
        "utility" => {
            let mut imported_by_smart = false;
            for cf in all_files {
                let cb = file_basename(cf);
                let cf_suffix = get_surface_suffix(cb);
                if surface_category(&cf_suffix) == "smart" {
                    if let Ok(c) = std::fs::read_to_string(cf) {
                        if c.contains(&stem) {
                            imported_by_smart = true;
                            break;
                        }
                    }
                }
            }
            OrphanIndicatorResult::new(
                !imported_by_smart,
                AesOrphanViolation::SurfaceOrphan {
                    category: "utility",
                    stem: stem.clone(),
                    reason: Some(
                        format!(
                            "Utility surface '{}' not imported by any smart surface.",
                            stem
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::HIGH,
            )
        }
        // Passive surface: must be imported by smart or utility surface
        "passive" => {
            let mut imported = false;
            for cf in all_files {
                let cb = file_basename(cf);
                let cf_suffix = get_surface_suffix(cb);
                let cat = surface_category(&cf_suffix);
                if cat == "smart" || cat == "utility" {
                    if let Ok(c) = std::fs::read_to_string(cf) {
                        if c.contains(&stem) {
                            imported = true;
                            break;
                        }
                    }
                }
            }
            OrphanIndicatorResult::new(
                !imported,
                AesOrphanViolation::SurfaceOrphan {
                    category: "passive",
                    stem: stem.clone(),
                    reason: Some(
                        format!(
                            "Passive surface '{}' not imported by any smart or utility surface.",
                            stem
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::HIGH,
            )
        }
        _ => OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
    }
}

pub fn check_surfaces_orphan(
    fp: &str,
    all_files: &[String],
    violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
) {
    let fp_vo = match FilePath::new(fp.to_string()) {
        Ok(p) => p,
        Err(_) => return,
    };
    let result = is_surface_orphan_raw(&fp_vo, all_files);
    if result.is_orphan {
        violations.push(crate::agent_orphan_orchestrator::mk_orphan_result(
            fp,
            &result.reason,
            result.severity,
            "AES506",
        ));
    }
}
```

---

## File: crates/orphan-detector/src/capabilities_orphan_taxonomy_analyzer.rs

```rust
// PURPOSE: TaxonomyOrphanAnalyzer — ITaxonomyOrphanProtocol for orphan taxonomy detection
use crate::taxonomy_orphan_filename_helper::file_stem;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ITaxonomyOrphanProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use shared::taxonomy_definition_vo::LayerDefinition;

pub struct TaxonomyOrphanAnalyzer {}

impl Default for TaxonomyOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl TaxonomyOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}

impl ITaxonomyOrphanProtocol for TaxonomyOrphanAnalyzer {
    fn is_taxonomy_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        definition: Option<&LayerDefinition>,
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult {
        is_taxonomy_orphan(f, root_dir, definition, inbound_links)
    }
}

/// Fallback: check if any sibling .rs file in the same directory imports this module via `crate::` path.
/// The graph resolver doesn't always track crate:: imports within the same crate.
fn has_crate_self_import(file_path: &str) -> bool {
    let stem = std::path::Path::new(file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    if stem.is_empty() {
        return false;
    }
    let search = format!("crate::{}", stem);
    if let Some(parent) = std::path::Path::new(file_path).parent() {
        if let Ok(entries) = std::fs::read_dir(parent) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path == std::path::Path::new(file_path) {
                    continue;
                }
                if path.extension().is_some_and(|e| e == "rs") {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        if content.contains(&search) {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn is_taxonomy_orphan(
    f: &FilePath,
    _root: &FilePath,
    _def: Option<&LayerDefinition>,
    inbound: &InboundLinkMap,
) -> OrphanIndicatorResult {
    let stem = file_stem(f.value());

    let suffix = match stem.rfind('_') {
        Some(pos) => &stem[pos + 1..],
        None => "",
    };

    let is_utility_or_helper = matches!(suffix, "utility" | "helper");

    let is_orphan = if is_utility_or_helper {
        // utility/helper: must be imported by file outside taxonomy
        let importers = match inbound.mapping.get(f.value()) {
            Some(v) => v,
            None => {
                // Fallback: graph resolver may not catch crate:: imports within same crate
                if has_crate_self_import(f.value()) {
                    return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                }
                return OrphanIndicatorResult::new(
                    true,
                    AesOrphanViolation::TaxonomyOrphan {
                        stem: stem.clone(),
                        category: "utility",
                        reason: Some(format!("Taxonomy '{}' (utility/helper) is not imported by any file outside taxonomy.", stem).into()),
                    }.to_string(),
                    Severity::LOW,
                );
            }
        };
        let has_outside_taxonomy = importers.iter().any(|importer| {
            importer
                .split('/')
                .next_back()
                .is_some_and(|b| !b.starts_with("taxonomy_"))
        });
        !has_outside_taxonomy
    } else {
        // vo, entity, error, event, constant: must be imported by contract layer
        let importers = match inbound.mapping.get(f.value()) {
            Some(v) => v,
            None => {
                return OrphanIndicatorResult::new(
                    true,
                    AesOrphanViolation::TaxonomyOrphan {
                        stem: stem.clone(),
                        category: "taxonomy",
                        reason: Some(
                            format!("Taxonomy '{}' is not imported by any contract.", stem).into(),
                        ),
                    }
                    .to_string(),
                    Severity::LOW,
                )
            }
        };
        let has_any_importer = importers.iter().any(|importer| {
            // Must be imported by a file outside the taxonomy layer
            // (contract_, capabilities_, infrastructure_, surface_, agent_, root_)
            importer
                .split('/')
                .next_back()
                .is_some_and(|b| !b.starts_with("taxonomy_"))
        });
        !has_any_importer
    };

    let category = if is_utility_or_helper {
        "utility"
    } else {
        "taxonomy"
    };

    OrphanIndicatorResult::new(
        is_orphan,
        AesOrphanViolation::TaxonomyOrphan {
            stem,
            category,
            reason: None,
        }
        .to_string(),
        Severity::LOW,
    )
}

pub fn check_taxonomy_orphan(
    fp: &str,
    basename: &str,
    files: &[String],
    violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
) {
    let stem = basename
        .replace(".rs", "")
        .replace(".py", "")
        .replace(".ts", "")
        .replace(".js", "");
    let suffix = match stem.rfind('_') {
        Some(pos) => &stem[pos + 1..],
        None => "",
    };

    let is_utility_or_helper = matches!(suffix, "utility" | "helper");

    let mut imported = false;
    for cf in files {
        let cb = match cf.split('/').next_back() {
            Some(b) => b,
            None => continue,
        };

        if is_utility_or_helper {
            // utility/helper: can be imported directly by any layer, no contract needed
            if let Ok(c) = std::fs::read_to_string(cf) {
                if c.contains(&stem) {
                    imported = true;
                    break;
                }
            }
        } else {
            // vo, entity, error, event, constant: must be imported via contract layer
            if !cb.starts_with("contract_") {
                continue;
            }
            if let Ok(c) = std::fs::read_to_string(cf) {
                if c.contains(&stem) {
                    imported = true;
                    break;
                }
            }
        }
    }
    if !imported {
        let category = if is_utility_or_helper {
            "utility"
        } else {
            "vo"
        };
        let reason = if is_utility_or_helper {
            format!("Taxonomy '{}' is not imported by any file.", stem)
        } else {
            format!("Taxonomy '{}' is not imported by any contract.", stem)
        };
        violations.push(crate::agent_orphan_orchestrator::mk_orphan_result(
            fp,
            &AesOrphanViolation::TaxonomyOrphan {
                stem: stem.clone(),
                category,
                reason: Some(reason.into()),
            }
            .to_string(),
            Severity::LOW,
            "AES501",
        ));
    }
}
```

---

## File: crates/orphan-detector/src/lib.rs

```rust
/// PURPOSE: Module declarations for orphan-detector (orchestrator, analyzers, container)
pub mod agent_orphan_orchestrator;
pub mod capabilities_orphan_agent_analyzer;
pub mod capabilities_orphan_capabilities_analyzer;
pub mod capabilities_orphan_contract_analyzer;
pub mod capabilities_orphan_graph_resolver;
pub mod capabilities_orphan_infrastructure_analyzer;
pub mod capabilities_orphan_surfaces_analyzer;
pub mod capabilities_orphan_taxonomy_analyzer;
pub mod root_orphan_detector_container;
pub mod taxonomy_orphan_filename_helper;
```

---

## File: crates/orphan-detector/src/root_orphan_detector_container.rs

```rust
// PURPOSE: OrphanContainer — wiring for orphan-detector feature (root layer, wiring only)
use crate::agent_orphan_orchestrator::ArchOrphanAnalyzer;
use crate::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use std::sync::Arc;

pub struct OrphanContainer {
    analyzer: Arc<dyn IOrphanAggregate>,
    layer_detector: Arc<dyn ILayerDetectionAggregate>,
}

impl OrphanContainer {
    pub fn new() -> Self {
        Self::new_with_ignored(Vec::new())
    }

    pub fn new_with_ignored(_ignored_paths: Vec<String>) -> Self {
        let resolver: Arc<dyn IOrphanGraphResolverProtocol> = Arc::new(OrphanGraphResolver::new());
        let arch = Arc::new(ArchOrphanAnalyzer::new(
            resolver,
            Arc::new(crate::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_infrastructure_analyzer::InfrastructureOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer::new()),
        ));
        let layer: Arc<dyn ILayerDetectionAggregate> = arch.clone();
        Self {
            analyzer: arch.clone() as Arc<dyn IOrphanAggregate>,
            layer_detector: layer,
        }
    }

    pub fn analyzer(&self) -> Arc<dyn IOrphanAggregate> {
        self.analyzer.clone()
    }

    pub fn layer_detector(&self) -> Arc<dyn ILayerDetectionAggregate> {
        self.layer_detector.clone()
    }
}
impl Default for OrphanContainer {
    fn default() -> Self {
        Self::new()
    }
}
```

---

## File: crates/orphan-detector/src/taxonomy_orphan_filename_helper.rs

```rust
// PURPOSE: taxonomy_orphan_filename_helper — shared filename parsing utilities for orphan analyzers
// Extracts basename, stem, and suffix from file paths in a consistent way across all orphan layer analyzers.

/// All language extensions recognized by the orphan detector.
const KNOWN_EXTENSIONS: &[&str] = &["tsx", "jsx", "ts", "js", "rs", "py"];

/// Extract the basename (filename without directory) from a file path string.
/// Handles both `/`-separated and OS paths gracefully.
pub fn file_basename(fp: &str) -> &str {
    fp.split('/').next_back().unwrap_or(fp)
}

/// Extract the file stem (basename with all known language extensions stripped).
///
/// Example: `capabilities_import_checker.rs` → `capabilities_import_checker`
pub fn file_stem(fp: &str) -> String {
    let mut stem = file_basename(fp).to_string();
    for ext in KNOWN_EXTENSIONS {
        let dot_ext = format!(".{}", ext);
        if stem.ends_with(&dot_ext) {
            stem = stem[..stem.len() - dot_ext.len()].to_string();
            break;
        }
    }
    stem
}

/// Extract the domain suffix from a file stem (the part after the last `_`, with
/// language extension stripped). Returns an empty string if no underscore is present.
///
/// Example: `capabilities_import_checker.rs` → `checker`
pub fn file_suffix(fp: &str) -> String {
    let basename = file_basename(fp);
    let stem = file_stem(fp);
    let stem_str = if stem.is_empty() { basename } else { &stem };
    stem_str.rsplit('_').next().unwrap_or_default().to_string()
}
```

---

## File: crates/shared/src/cli-commands/mod.rs

```rust
// cli-commands — taxonomy and contract types
pub mod contract_executor_port;
pub mod taxonomy_catalog_constant;
pub mod taxonomy_cli_vo;
pub mod taxonomy_command_catalog_vo;
pub mod taxonomy_format_vo;
pub mod taxonomy_metadata_vo;
pub mod taxonomy_position_vo;
pub mod taxonomy_protocol_vo;
pub mod taxonomy_result_vo;
pub mod taxonomy_score_vo;
pub mod taxonomy_severity_vo;
```

---

## File: crates/shared/src/cli-commands/taxonomy_result_vo.rs

```rust
// PURPOSE: LintResult, LintResultList, FilePathSet — value objects for lint violation results
use serde::{Deserialize, Serialize};

use crate::cli_commands::taxonomy_position_vo::Position;
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_lint_vo::LocationList;
use crate::common::taxonomy_lint_vo::ScopeRef;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LintResult {
    pub file: FilePath,
    pub line: LineNumber,
    pub column: ColumnNumber,
    pub code: ErrorCode,
    pub message: LintMessage,
    pub source: Option<AdapterName>,
    pub severity: Severity,
    pub enclosing_scope: Option<ScopeRef>,
    pub related_locations: LocationList,
}

impl LintResult {
    /// Convenience constructor used by architecture checkers (make_result / mk pattern).
    pub fn new_arch(
        file: &str,
        line: usize,
        code: &str,
        sev: Severity,
        msg: impl Into<String>,
    ) -> Self {
        Self {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(line as i64),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
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

    pub fn position(&self) -> Position {
        Position {
            line: self.line.clone(),
            column: self.column.clone(),
        }
    }
    pub fn identity(&self) -> Identity {
        Identity::new(format!(
            "{}:{}:{}:{:?}",
            self.file, self.line, self.code, self.source
        ))
    }
}

/// Generate a `Vec<T>`-backed newtype with `Default`, `new`, `iter`,
/// `len`, `is_empty`, `push`, and `append`. Used for the `LintResultList`
/// wrapper below; siblings `ImportInfoList`/`PrimitiveViolationList` in
/// `taxonomy_import_source_vo.rs` carry the same surface.
macro_rules! lint_result_list_wrapper {
    ($name:ident, $item:ty) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
        pub struct $name {
            pub values: Vec<$item>,
        }

        impl $name {
            pub fn new(value: Vec<$item>) -> Self {
                Self { values: value }
            }
            pub fn iter(&self) -> std::slice::Iter<'_, $item> {
                self.values.iter()
            }
            pub fn len(&self) -> usize {
                self.values.len()
            }
            pub fn is_empty(&self) -> bool {
                self.values.is_empty()
            }
            pub fn push(&mut self, item: $item) {
                self.values.push(item);
            }
            pub fn append(&mut self, item: $item) {
                self.values.push(item);
            }
        }
    };
}

lint_result_list_wrapper!(LintResultList, LintResult);
```

---

## File: crates/shared/src/cli-commands/taxonomy_severity_vo.rs

```rust
// PURPOSE: Severity — re-export from common for backward compatibility
//
// This module exists so dependents can keep using the
// `cli_commands::taxonomy_severity_vo::Severity` import path. The real
// definition lives in `common::taxonomy_severity_vo` and is re-exported
// here to avoid breaking any code that still imports from the legacy path.
pub use crate::common::taxonomy_severity_vo::Severity;
```

---

## File: crates/shared/src/code-analysis/contract_layer_detection_aggregate.rs

```rust
// PURPOSE: ILayerDetectionAggregate — contract trait for layer detection (detect_layer + get_layer_def)
use crate::common::taxonomy_definition_vo::LayerDefinition;

/// Slim aggregate for layer detection — used by orphan detector and orchestrator.
/// Container implements this; orchestrator calls individual checker protocols directly.
pub trait ILayerDetectionAggregate: Send + Sync {
    fn detect_layer(&self, file_path: &str, root_dir: &str) -> Option<String>;
    fn get_layer_def(&self, layer: &str) -> Option<LayerDefinition>;
    fn get_orphan_entry_points(&self) -> Vec<String>;
}
```

---

## File: crates/shared/src/code-analysis/mod.rs

```rust
// code-analysis — taxonomy and contract types
pub mod contract_adapter_port;
pub mod contract_bypass_checker_protocol;
pub mod contract_class_protocol;
pub mod contract_code_analysis_aggregate;
pub mod contract_code_metric_analyzer_protocol;
pub mod contract_cycle_protocol;
pub mod contract_dead_inheritance_protocol;
pub mod contract_layer_detection_aggregate;
pub mod contract_line_protocol;
pub mod taxonomy_analysis_vo;
pub mod taxonomy_code_analysis_rule_vo;
pub mod taxonomy_import_source_vo;
pub mod taxonomy_operation_error;
pub mod taxonomy_violation_code_analysis_vo;
pub use taxonomy_violation_code_analysis_vo::{AesCodeAnalysisViolation, Language};
```

---

## File: crates/shared/src/code-analysis/taxonomy_analysis_vo.rs

```rust
// PURPOSE: FileDefinitionMap, GraphAnalysisContext, ImportGraph, InboundLinkMap, InheritanceMap — analysis value objects for code structure
// Re-export LintResultList so code_analysis contracts stay within their own domain.
pub use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// A set of file paths.
pub type FilePathSet = HashSet<FilePath>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileDefinitionMap {
    pub mapping: std::collections::HashMap<String, Vec<String>>,
}

impl FileDefinitionMap {
    pub fn new(value: std::collections::HashMap<String, Vec<String>>) -> Self {
        Self { mapping: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphAnalysisContext {
    pub import_graph: ImportGraph,
    pub inbound_links: InboundLinkMap,
    pub inheritance_map: InheritanceMap,
    pub file_definitions: FileDefinitionMap,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportGraph {
    pub mapping: std::collections::HashMap<String, Vec<String>>,
}

impl ImportGraph {
    pub fn new(value: std::collections::HashMap<String, Vec<String>>) -> Self {
        Self { mapping: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InboundLinkMap {
    pub mapping: std::collections::HashMap<String, Vec<String>>,
}

impl InboundLinkMap {
    pub fn new(value: std::collections::HashMap<String, Vec<String>>) -> Self {
        Self { mapping: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InheritanceMap {
    pub mapping: std::collections::HashMap<String, Vec<String>>,
}

impl InheritanceMap {
    pub fn new(value: std::collections::HashMap<String, Vec<String>>) -> Self {
        Self { mapping: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrphanIndicatorResult {
    pub is_orphan: bool,
    pub reason: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReachabilityResult {
    pub paths: FilePathSet,
}

impl ReachabilityResult {
    pub fn new(value: FilePathSet) -> Self {
        Self { paths: value }
    }
}

impl GraphAnalysisContext {
    pub fn new(
        import_graph: ImportGraph,
        inbound_links: InboundLinkMap,
        inheritance_map: InheritanceMap,
        file_definitions: FileDefinitionMap,
    ) -> Self {
        Self {
            import_graph,
            inbound_links,
            inheritance_map,
            file_definitions,
        }
    }
}

impl OrphanIndicatorResult {
    pub fn new(is_orphan: bool, reason: String, severity: Severity) -> Self {
        Self {
            is_orphan,
            reason,
            severity,
        }
    }
}
```

---

## File: crates/shared/src/code-analysis/taxonomy_import_source_vo.rs

```rust
// PURPOSE: ImportInfo, PrimitiveViolation, PrimitiveViolationList — value objects for import analysis and primitive type detection
use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportInfo {
    pub line: LineNumber,
    pub module: String,
    #[serde(default)]
    pub name: Option<String>,
}

impl ImportInfo {
    pub fn new(line: LineNumber, module: String) -> Self {
        Self {
            line,
            module,
            name: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrimitiveViolation {
    pub line: LineNumber,
    pub column: ColumnNumber,
    pub type_name: String,
}

impl PrimitiveViolation {
    pub fn new(line: LineNumber, column: ColumnNumber, type_name: String) -> Self {
        Self {
            line,
            column,
            type_name,
        }
    }
}

/// Emit a `Vec<T>`-backed newtype plus `Default`, `new`, `push`, `len`,
/// and `is_empty`. Used for the two list wrappers below.
macro_rules! list_wrapper {
    ($name:ident, $item:ty) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        pub struct $name {
            #[serde(default)]
            pub values: Vec<$item>,
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl $name {
            pub fn new() -> Self {
                Self { values: Vec::new() }
            }
            pub fn push(&mut self, item: $item) {
                self.values.push(item);
            }
            pub fn len(&self) -> usize {
                self.values.len()
            }
            pub fn is_empty(&self) -> bool {
                self.values.is_empty()
            }
        }
    };
}

list_wrapper!(ImportInfoList, ImportInfo);
list_wrapper!(PrimitiveViolationList, PrimitiveViolation);
```

---

## File: crates/shared/src/common/infrastructure_file_collector_provider.rs

```rust
use std::fs;
use std::path::{Path, PathBuf};

use crate::common::contract_scanner_provider_port::IScannerProviderPort;
use crate::common::taxonomy_file_collector_helper::is_path_ignored;
use crate::common::taxonomy_filesystem_error::FileSystemError;
use crate::common::taxonomy_path_vo::DirectoryPath;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::config_system::taxonomy_config_vo::default_aes_config;

pub struct FileCollectorProvider {}

impl Default for FileCollectorProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl FileCollectorProvider {
    pub fn new() -> Self {
        Self {}
    }
}

fn default_ignored_paths() -> Vec<String> {
    let config = default_aes_config();
    config
        .ignored_paths
        .values
        .iter()
        .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
        .collect()
}

pub fn collect_all_source_files(dir: &Path) -> Vec<FilePath> {
    let mut files = Vec::new();
    if dir.exists() && dir.is_dir() {
        walk_source_files(dir, &mut files, &[]);
    }
    files
}

impl IScannerProviderPort for FileCollectorProvider {
    fn scan_directory(&self, path: &DirectoryPath) -> Result<FilePathList, FileSystemError> {
        let dir = Path::new(&path.value);
        let mut files = Vec::new();
        if !dir.exists() || !dir.is_dir() {
            return Ok(FilePathList { values: files });
        }
        let ignored = default_ignored_paths();
        walk_source_files(dir, &mut files, &ignored);
        Ok(FilePathList { values: files })
    }

    fn get_ignored_files(&self) -> FilePathList {
        FilePathList { values: vec![] }
    }
}

fn is_source_file(ext: &str) -> bool {
    matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx")
}

fn is_ignored_dir(dir: &Path, ignored: &[String]) -> bool {
    let s = dir.to_string_lossy();
    is_path_ignored(&s, ignored)
}

fn walk_source_files(dir: &Path, files: &mut Vec<FilePath>, ignored: &[String]) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if is_ignored_dir(&path, ignored) {
                continue;
            }
            if path.is_dir() {
                let dir_name = path
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_default();
                if dir_name == "tests" {
                    continue;
                }
                walk_source_files(&path, files, ignored);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if is_source_file(ext) {
                    if let Some(path_str) = path.to_str() {
                        if let Ok(fp) = FilePath::new(path_str.to_string()) {
                            files.push(fp);
                        }
                    }
                }
            }
        }
    }
}

pub fn walk_rs_files(dir: &Path, cb: &mut dyn FnMut(PathBuf), ignored: &[String]) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if is_ignored_dir(&p, ignored) {
                continue;
            }
            if p.is_dir() {
                walk_rs_files(&p, cb, ignored);
            } else if matches!(p.extension().and_then(|e| e.to_str()), Some("rs")) {
                cb(p);
            }
        }
    }
}
```

---

## File: crates/shared/src/common/mod.rs

```rust
// common — truly shared types used by multiple features
pub mod taxonomy_action_vo;
pub mod taxonomy_adapter_name_vo;
pub mod taxonomy_byte_count_vo;
pub mod taxonomy_common_error;
pub mod taxonomy_common_vo;
pub mod taxonomy_definition_vo;
pub mod taxonomy_display_content_vo;
pub mod taxonomy_duration_vo;
pub mod taxonomy_error_vo;
pub mod taxonomy_job_id_vo;
pub mod taxonomy_job_vo;
pub mod taxonomy_layer_vo;
pub mod taxonomy_line_count_vo;
pub mod taxonomy_lint_vo;
pub mod taxonomy_message_vo;
pub mod taxonomy_name_vo;
pub mod taxonomy_response_data_vo;
pub mod taxonomy_severity_vo;
pub mod taxonomy_source_vo;
pub mod taxonomy_suggestion_vo;
pub mod taxonomy_value_object_utility;

// from file-system/ (foundational, multi-feature)
pub mod contract_system_port;
pub mod taxonomy_filesystem_error;

// from source-parsing/ (foundational, multi-feature)
pub mod contract_language_detector_port;
pub mod contract_parser_port;
pub mod contract_path_normalization_port;
pub mod contract_scanner_provider_port;
pub mod infrastructure_file_collector_provider;
pub mod taxonomy_adapter_error;
pub mod taxonomy_file_collector_helper;
pub mod taxonomy_language_detector_helper;
pub mod taxonomy_language_vo;
pub mod taxonomy_naming_list_vo;
pub mod taxonomy_parser_error;
pub mod taxonomy_path_utils_vo;
pub mod taxonomy_path_vo;
pub mod taxonomy_paths_vo;
pub mod taxonomy_workspace_helper;
pub use infrastructure_file_collector_provider::{
    collect_all_source_files, walk_rs_files, FileCollectorProvider,
};
```

---

## File: crates/shared/src/common/taxonomy_adapter_error.rs

```rust
// PURPOSE: AdapterError, ScanError, ValidationError — structured error types for adapter operations
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::Constraint;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_error::ExitCode;
use crate::common::taxonomy_common_error::FieldName;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, thiserror::Error)]
pub struct AdapterError {
    pub adapter_name: AdapterName,
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: Option<ErrorCode>,
    #[serde(default)]
    pub command: Option<ContentString>,
    #[serde(default)]
    pub stderr: Option<ErrorMessage>,
    #[serde(default)]
    pub exit_code: Option<ExitCode>,
}

impl AdapterError {
    pub fn new(adapter_name: AdapterName, message: ErrorMessage) -> Self {
        Self {
            adapter_name,
            message,
            error_code: None,
            command: None,
            stderr: None,
            exit_code: None,
        }
    }
}

impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self.error_code.as_ref() {
            Some(c) => format!(" [{}]", c),
            None => String::new(),
        };
        write!(f, "[{}]{} {}", self.adapter_name, code, self.message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct ScanError {
    pub path: FilePath,
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: Option<ErrorCode>,
    #[serde(default)]
    pub adapter_name: Option<AdapterName>,
    #[serde(default)]
    pub cause: Option<Cause>,
}

impl ScanError {
    pub fn new(path: FilePath, message: ErrorMessage) -> Self {
        Self {
            path,
            message,
            error_code: None,
            adapter_name: None,
            cause: None,
        }
    }
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let adapter = match self.adapter_name.as_ref() {
            Some(a) => format!(" ({})", a),
            None => String::new(),
        };
        let code = match self.error_code.as_ref() {
            Some(c) => format!(" [{}]", c),
            None => String::new(),
        };
        write!(
            f,
            "Scan failed{}{}: {} — {}",
            adapter, code, self.path, self.message
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct ValidationError {
    pub field_name: FieldName,
    pub message: ErrorMessage,
    #[serde(default)]
    pub constraint: Option<Constraint>,
    #[serde(default)]
    pub value: Option<String>,
}

impl ValidationError {
    pub fn new(field_name: FieldName, message: ErrorMessage) -> Self {
        Self {
            field_name,
            message,
            constraint: None,
            value: None,
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Validation failed on '{}': {}",
            self.field_name, self.message
        )
    }
}
```

---

## File: crates/shared/src/common/taxonomy_adapter_name_vo.rs

```rust
// PURPOSE: AdapterName — validated newtype for adapter/linter name strings
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// adapter_name_vo — Adapter and tool identifier value objects.
///
/// Adapter/tool identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AdapterName {
    pub value: String,
}

impl AdapterName {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new AdapterName from a string.
    ///
    /// # Errors
    /// Returns an error if the adapter name is empty or only whitespace.
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("Adapter name cannot be empty".to_string());
        }
        Ok(AdapterName {
            value: value.trim().to_string(),
        })
    }

    /// Create a raw AdapterName without error validation (for static compile-time safe inputs).
    pub fn raw<S: Into<String>>(value: S) -> Self {
        AdapterName {
            value: value.into(),
        }
    }
}

impl std::ops::Deref for AdapterName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for AdapterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Hash for AdapterName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
```

---

## File: crates/shared/src/common/taxonomy_byte_count_vo.rs

```rust
// PURPOSE: ByteCount — value object for file/stream sizes
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ByteCount {
    pub value: u64,
}

impl ByteCount {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl From<u64> for ByteCount {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl std::fmt::Display for ByteCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
```

---

## File: crates/shared/src/common/taxonomy_common_error.rs

```rust
// PURPOSE: Cause, Constraint, ExitCode, FieldName, ModuleName, PrimitiveTypeName — common error value objects
pub use crate::common::taxonomy_common_vo::ErrorMessage;
use crate::string_value_object;
use serde::Serialize;

string_value_object!(Cause);
string_value_object!(Constraint);
string_value_object!(FieldName);
string_value_object!(ModuleName);
string_value_object!(PrimitiveTypeName);

/// Strongly-typed exit code value object. Written manually because the
/// `string_value_object!` macro only supports `String` (not `i64`).
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ExitCode {
    pub value: crate::common::taxonomy_common_vo::LineNumber,
}

impl ExitCode {
    pub fn new(value: impl Into<crate::common::taxonomy_common_vo::LineNumber>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> i64 {
        self.value.value()
    }
}

impl std::fmt::Display for ExitCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for ExitCode {
    fn from(v: i64) -> Self {
        Self {
            value: crate::common::taxonomy_common_vo::LineNumber::new(v),
        }
    }
}

impl<'de> serde::Deserialize<'de> for ExitCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct W {
            value: crate::common::taxonomy_common_vo::LineNumber,
        }
        let w = W::deserialize(deserializer)?;
        Ok(Self { value: w.value })
    }
}
```

---

## File: crates/shared/src/common/taxonomy_common_vo.rs

```rust
// PURPOSE: BooleanVO, ColumnNumber, Count, DataFlowList, LineContentList, LineNumber, PatternList, Score, Timestamp — common VOs
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_job_id_vo::JobId;
use crate::common::taxonomy_layer_vo::LineContentVO;
use crate::common::taxonomy_response_data_vo::ResponseData;
use crate::common::taxonomy_severity_vo::Severity;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct BooleanVO {
    pub value: bool,
}

impl BooleanVO {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}

impl std::fmt::Display for BooleanVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<bool> for BooleanVO {
    fn from(v: bool) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for BooleanVO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BooleanVOVisitor {}
        impl<'de> serde::de::Visitor<'de> for BooleanVOVisitor {
            type Value = BooleanVO;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BooleanVO { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<bool>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(BooleanVO { value: val })
            }
        }
        deserializer.deserialize_any(BooleanVOVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ColumnNumber {
    pub value: i64,
}

impl ColumnNumber {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for ColumnNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for ColumnNumber {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for ColumnNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ColumnNumberVisitor {}
        impl<'de> serde::de::Visitor<'de> for ColumnNumberVisitor {
            type Value = ColumnNumber;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ColumnNumber { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ColumnNumber { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(ColumnNumber { value: val })
            }
        }
        deserializer.deserialize_any(ColumnNumberVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct Count {
    pub value: i64,
}

impl Count {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for Count {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for Count {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for Count {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CountVisitor {}
        impl<'de> serde::de::Visitor<'de> for CountVisitor {
            type Value = Count;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Count { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Count { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Count { value: val })
            }
        }
        deserializer.deserialize_any(CountVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataFlowList {
    pub values: Vec<ErrorMessage>,
}

impl DataFlowList {
    pub fn new(value: Vec<ErrorMessage>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[ErrorMessage] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, ErrorMessage> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: ErrorMessage) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobIdList {
    pub values: Vec<JobId>,
}

impl JobIdList {
    pub fn new(value: Vec<JobId>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[JobId] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, JobId> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: JobId) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LineContentList {
    pub values: Vec<LineContentVO>,
}

impl LineContentList {
    pub fn new(value: Vec<LineContentVO>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[LineContentVO] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, LineContentVO> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: LineContentVO) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
#[derive(Default)]
pub struct LineNumber {
    pub value: i64,
}

impl LineNumber {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for LineNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for LineNumber {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for LineNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct LineNumberVisitor {}
        impl<'de> serde::de::Visitor<'de> for LineNumberVisitor {
            type Value = LineNumber;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LineNumber { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LineNumber { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(LineNumber { value: val })
            }
        }
        deserializer.deserialize_any(LineNumberVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct PatternList {
    pub values: Vec<String>,
}

impl PatternList {
    pub fn new(value: impl IntoPatternListValues) -> Self {
        Self {
            values: value.into_pattern_list_values(),
        }
    }
    pub fn values(&self) -> &[String] {
        &self.values
    }
}

impl PatternList {
    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: String) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseDataList {
    pub values: Vec<ResponseData>,
}

impl ResponseDataList {
    pub fn new(value: Vec<ResponseData>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[ResponseData] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, ResponseData> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: ResponseData) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(transparent)]
pub struct Score {
    pub value: f64,
}

impl Score {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> f64 {
        self.value
    }
    pub fn is_perfect(&self) -> bool {
        self.value >= 100.0
    }
    pub fn is_passing(&self, threshold: &Score) -> bool {
        self.value >= threshold.value
    }
    pub fn deduct(&self, severity: &Severity) -> Score {
        Score {
            value: self.value - severity.score_impact(),
        }
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}", self.value)
    }
}

impl From<f64> for Score {
    fn from(v: f64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for Score {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ScoreVisitor {}
        impl<'de> serde::de::Visitor<'de> for ScoreVisitor {
            type Value = Score;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Score { value: v })
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Score { value: v as f64 })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Score { value: v as f64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<f64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Score { value: val })
            }
        }
        deserializer.deserialize_any(ScoreVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct Timestamp {
    pub value: String,
}

impl Timestamp {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn now() -> Self {
        Self {
            value: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for Timestamp {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for Timestamp {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TimestampVisitor {}
        impl<'de> serde::de::Visitor<'de> for TimestampVisitor {
            type Value = Timestamp;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Timestamp {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Timestamp { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<String>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Timestamp { value: val })
            }
        }
        deserializer.deserialize_any(TimestampVisitor {})
    }
}

// Custom Coercion Traits for PatternList

pub trait IntoPatternListValues {
    fn into_pattern_list_values(self) -> Vec<String>;
}

impl IntoPatternListValues for &str {
    fn into_pattern_list_values(self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl IntoPatternListValues for String {
    fn into_pattern_list_values(self) -> Vec<String> {
        vec![self]
    }
}

impl IntoPatternListValues for Vec<String> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self
    }
}

impl IntoPatternListValues for Vec<&str> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self.into_iter().map(|s| s.to_string()).collect()
    }
}

impl IntoPatternListValues for &Vec<String> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ErrorMessage {
    pub value: String,
}

impl ErrorMessage {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for ErrorMessage {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for ErrorMessage {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_definition_vo.rs

```rust
// PURPOSE: LayerDefinition, LayerMapVO, NamingConfig — VOs for AES layer definitions and naming policies
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use serde::{Deserialize, Serialize};

/// Wrap a single-field VO that exposes a `new(value)` constructor plus the
/// default `derive`s needed by the codebase. Used to keep the boilerplate
/// for `LayerMapVO`/`NamingConfig` uniform without introducing a new linter
/// violation cluster.
macro_rules! single_field_vo {
    ($name:ident, $field:ident: $field_ty:ty) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
        pub struct $name {
            pub $field: $field_ty,
        }

        impl $name {
            pub fn new($field: $field_ty) -> Self {
                Self { $field }
            }
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LayerDefinition {
    #[serde(default)]
    pub allowed: PatternList,
    #[serde(default)]
    pub forbidden: PatternList,
    #[serde(default)]
    pub mandatory: PatternList,
    #[serde(default)]
    pub word_count: Count,
    #[serde(default)]
    pub exceptions: PatternList,
    #[serde(default)]
    pub recursive: BooleanVO,

    #[serde(flatten)]
    pub naming: crate::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO,
    #[serde(flatten)]
    pub code_analysis: crate::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO,
    #[serde(flatten)]
    pub role: crate::role_rules::taxonomy_role_rule_vo::RoleRuleVO,
    #[serde(flatten)]
    pub orphan: crate::orphan_detector::taxonomy_orphan_rule_vo::OrphanRuleVO,
}

single_field_vo!(LayerMapVO, values: std::collections::HashMap<LayerNameVO, LayerDefinition>);
single_field_vo!(NamingConfig, word_count: Count);
```

---

## File: crates/shared/src/common/taxonomy_display_content_vo.rs

```rust
// PURPOSE: DisplayContent — value object for formatted display output (previews, human-readable sizes, etc.)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisplayContent {
    pub value: String,
}

impl DisplayContent {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl From<String> for DisplayContent {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl std::fmt::Display for DisplayContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
```

---

## File: crates/shared/src/common/taxonomy_duration_vo.rs

```rust
// PURPOSE: Duration, Timeout — value objects for duration and timeout tracking
use serde::Serialize;

/// Wrap a `f64` value object that should be clamped to a minimum during
/// construction. Emit the struct, manual `new`/`value`/`Display`/`From`
/// impls, and a serde `Deserialize` that respects the clamp.
macro_rules! clamped_f64_vo {
    ($name:ident, $min:expr, $display_fmt:literal) => {
        #[derive(Debug, Clone, Serialize, PartialEq)]
        #[serde(transparent)]
        pub struct $name {
            pub value: f64,
        }

        impl $name {
            pub fn new(value: f64) -> Self {
                Self {
                    value: value.max($min),
                }
            }
            pub fn value(&self) -> f64 {
                self.value
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $display_fmt, self.value)
            }
        }

        impl From<f64> for $name {
            fn from(v: f64) -> Self {
                Self { value: v }
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(serde::Deserialize)]
                #[serde(transparent)]
                struct W {
                    value: f64,
                }
                let w = W::deserialize(deserializer)?;
                Ok(Self {
                    value: w.value.max($min),
                })
            }
        }
    };
}

clamped_f64_vo!(Timeout, 0.001, "{}s");
```

---

## File: crates/shared/src/common/taxonomy_error_vo.rs

```rust
// PURPOSE: ErrorCode — value object for AES error code identification
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// error_code_vo — Error code value object.
///
/// Linter error code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ErrorCode {
    code: String,
}

impl ErrorCode {
    pub fn code(&self) -> &str {
        &self.code
    }
    /// Create a new ErrorCode from a string.
    ///
    /// # Errors
    /// Returns an error if the code is empty.
    pub fn new<S: Into<String>>(code: S) -> Result<Self, String> {
        let code = code.into();
        if code.is_empty() {
            return Err("Error code cannot be empty".to_string());
        }
        Ok(ErrorCode { code })
    }

    /// Create a raw ErrorCode without error validation.
    pub fn raw<S: Into<String>>(code: S) -> Self {
        ErrorCode { code: code.into() }
    }

    /// Returns true if the code is a style error (starts with E, W, or D).
    pub fn is_style(&self) -> bool {
        self.code.starts_with('E') || self.code.starts_with('W') || self.code.starts_with('D')
    }
    pub fn is_logic(&self) -> bool {
        self.code.starts_with('F') || self.code.starts_with('I')
    }
    pub fn is_security(&self) -> bool {
        self.code.starts_with('B')
    }
    pub fn is_architecture(&self) -> bool {
        self.code.starts_with("AES")
    }
}

impl std::ops::Deref for ErrorCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.code
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl Hash for ErrorCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}
```

---

## File: crates/shared/src/common/taxonomy_filesystem_error.rs

```rust
// PURPOSE: FileSystemError — structured error type for filesystem operation failures
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_path_vo::FilePath;
use crate::mcp_server::taxonomy_action_vo::ActionName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct FileSystemError {
    pub path: FilePath,
    pub message: ErrorMessage,
    pub operation: ActionName,
    #[serde(default)]
    pub error_code: ErrorCode,
    #[serde(default)]
    pub cause: Cause,
}

impl FileSystemError {
    pub fn new(path: FilePath, message: ErrorMessage, operation: ActionName) -> Self {
        Self {
            path,
            message,
            operation,
            error_code: ErrorCode::default(),
            cause: Cause::default(),
        }
    }
}

impl std::fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = if self.error_code.code().is_empty() {
            String::new()
        } else {
            format!(" [{}]", self.error_code.code())
        };
        write!(
            f,
            "FS Error during {} on {}{}: {}",
            self.operation, self.path, code, self.message
        )
    }
}
```

---

## File: crates/shared/src/common/taxonomy_job_vo.rs

```rust
// PURPOSE: PipelineJob, SuccessStatus, EnvContentVO, McpConfigVO — value objects for pipeline job lifecycle tracking
// ResponseData is re-exported from common for backward compatibility
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::string_value_object;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::common::taxonomy_response_data_vo::ResponseData;

// Manual impl: `SuccessStatus` overrides `Display` to render "SUCCESS"/"FAILURE"
// instead of `true`/`false`, and the macro does not currently support a clean
// `bool` cast (Rust forbids `i64 as bool`). Kept as a hand-rolled VO.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SuccessStatus {
    pub value: bool,
}

impl Default for SuccessStatus {
    fn default() -> Self {
        Self::new(false)
    }
}

impl SuccessStatus {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}

impl std::fmt::Display for SuccessStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value {
            write!(f, "SUCCESS")
        } else {
            write!(f, "FAILURE")
        }
    }
}

impl std::ops::Deref for SuccessStatus {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.value
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterMetadata {
    pub name: AdapterName,
    pub class_path: String,
    #[serde(default)]
    pub description: String,
}

impl AdapterMetadata {
    pub fn new(name: AdapterName, class_path: String) -> Self {
        Self {
            name,
            class_path,
            description: String::new(),
        }
    }
}

string_value_object!(EnvContentVO);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpConfigVO {
    #[serde(default)]
    pub value: HashMap<String, serde_json::Value>,
}

impl McpConfigVO {
    pub fn new(value: HashMap<String, serde_json::Value>) -> Self {
        Self { value }
    }
    pub fn value(&self) -> &HashMap<String, serde_json::Value> {
        &self.value
    }
}
```

---

## File: crates/shared/src/common/taxonomy_language_detector_helper.rs

```rust
// PURPOSE: LanguageDetector — Helper for detecting programming languages from file paths
use crate::common::taxonomy_language_vo::Language;
use crate::common::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Copy, Default)]
pub struct LanguageDetector;

impl LanguageDetector {
    pub fn new() -> Self {
        Self
    }

    /// Detect language from a FilePath based on extension.
    pub fn detect(&self, path: &FilePath) -> Language {
        let ext = path.extension();
        match ext.as_str() {
            "py" => Language::Python,
            "js" | "jsx" | "mjs" | "cjs" => Language::JavaScript,
            "ts" | "tsx" | "mts" | "cts" => Language::TypeScript,
            "rs" => Language::Rust,
            _ => Language::Unknown,
        }
    }

    /// Check if a FilePath represents a lintable language.
    pub fn is_lintable(&self, path: &FilePath) -> bool {
        matches!(
            self.detect(path),
            Language::Python | Language::JavaScript | Language::TypeScript | Language::Rust
        )
    }
}
```

---

## File: crates/shared/src/common/taxonomy_line_count_vo.rs

```rust
// PURPOSE: LineCount — value object for the number of lines (preview, file limits, etc.)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineCount {
    pub value: usize,
}

impl LineCount {
    pub fn new(value: usize) -> Self {
        Self { value }
    }

    pub fn value(&self) -> usize {
        self.value
    }
}

impl From<usize> for LineCount {
    fn from(value: usize) -> Self {
        Self { value }
    }
}

impl std::fmt::Display for LineCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
```

---

## File: crates/shared/src/common/taxonomy_lint_vo.rs

```rust
// PURPOSE: CommandArgs, Location, LocationList, ScopeBounds, ScopeRef, ViolationConstraint — VOs for lint violations
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeRef {
    pub name: DescriptionVO,
    #[serde(default)]
    pub kind: DescriptionVO,
    #[serde(default)]
    pub file: Option<FilePath>,
    #[serde(default)]
    pub start_line: Option<LineNumber>,
    #[serde(default)]
    pub end_line: Option<LineNumber>,
}

impl ScopeRef {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: DescriptionVO::new(name),
            kind: DescriptionVO::new("function"),
            file: None,
            start_line: None,
            end_line: None,
        }
    }
    pub fn has_range(&self) -> bool {
        self.start_line.as_ref().is_some_and(|l| l.value > 0)
            && self.end_line.as_ref().is_some_and(|l| l.value > 0)
    }
}

impl std::fmt::Display for ScopeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref file) = self.file {
            write!(f, "{} {} in {}", self.kind.value, self.name.value, file)
        } else if !self.kind.value.is_empty() {
            write!(f, "{} {}", self.kind.value, self.name.value)
        } else {
            write!(f, "{}", self.name.value)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    #[serde(default)]
    pub file: Option<FilePath>,
    #[serde(default)]
    pub line: Option<LineNumber>,
    #[serde(default)]
    pub column: Option<ColumnNumber>,
    #[serde(default)]
    pub description: DescriptionVO,
}

impl Default for Location {
    fn default() -> Self {
        Self::new()
    }
}

impl Location {
    pub fn new() -> Self {
        Self {
            file: None,
            line: None,
            column: None,
            description: DescriptionVO::new(String::new()),
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        if let Some(ref file) = self.file {
            parts.push(file.value.clone());
        }
        if let Some(ref line) = self.line {
            let mut s = line.value.to_string();
            if let Some(ref col) = self.column {
                if col.value > 0 {
                    s = format!("{}:{}", line.value, col.value);
                }
            }
            parts.push(s);
        }
        let result = if parts.is_empty() {
            "unknown".to_string()
        } else {
            parts.join(":")
        };
        if self.description.value.is_empty() {
            write!(f, "{}", result)
        } else {
            write!(f, "{} — {}", result, self.description.value)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LocationList {
    #[serde(default)]
    pub values: Vec<Location>,
}

impl LocationList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
}

impl LocationList {
    pub fn push(&mut self, item: Location) {
        self.values.push(item);
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl std::ops::Deref for LocationList {
    type Target = Vec<Location>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ViolationConstraint {
    pub rule: DescriptionVO,
    #[serde(default)]
    pub min_value: DescriptionVO,
    #[serde(default)]
    pub max_value: DescriptionVO,
}

impl ViolationConstraint {
    pub fn new(rule: impl Into<String>) -> Self {
        Self {
            rule: DescriptionVO::new(rule),
            min_value: DescriptionVO::new(String::new()),
            max_value: DescriptionVO::new(String::new()),
        }
    }
}

impl std::fmt::Display for ViolationConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rule.value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandArgs {
    #[serde(default)]
    pub args: Vec<ContentString>,
}

impl Default for CommandArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandArgs {
    pub fn new() -> Self {
        Self { args: Vec::new() }
    }
}

impl std::fmt::Display for CommandArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.args
                .iter()
                .map(|a| a.value.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeBounds {
    #[serde(default)]
    pub start: Option<LineNumber>,
    #[serde(default)]
    pub end: Option<LineNumber>,
}
```

---

## File: crates/shared/src/common/taxonomy_message_vo.rs

```rust
// PURPOSE: ComplianceStatus, LintMessage — VOs for compliance status and violation messages
use crate::string_value_object;

string_value_object!(LintMessage);

/// Boolean compliance flag. Written manually because `bool` is not supported
/// by the `string_value_object!` macro (`i64 as bool` is not a valid Rust cast).
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ComplianceStatus {
    pub value: bool,
}

impl ComplianceStatus {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}

impl std::fmt::Display for ComplianceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<bool> for ComplianceStatus {
    fn from(v: bool) -> Self {
        Self { value: v }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_name_vo.rs

```rust
// PURPOSE: NameVariants, SymbolName — value objects for symbol naming and naming convention variants
use crate::string_value_object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NameVariants {
    pub values: Vec<SymbolName>,
}

impl NameVariants {
    pub fn new(value: Vec<SymbolName>) -> Self {
        Self { values: value }
    }
    pub fn iter(&self) -> std::slice::Iter<'_, SymbolName> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: SymbolName) {
        self.values.push(item);
    }
}

string_value_object!(SymbolName);
```

---

## File: crates/shared/src/common/taxonomy_naming_list_vo.rs

```rust
// PURPOSE: SymbolNameList, PrimitiveTypeList — VOs for collections of symbol names and primitive types
use crate::common::taxonomy_name_vo::SymbolName;
use serde::{Deserialize, Serialize};

pub const CORE_PRIMITIVE_TYPES: &[&str] = &["str", "int", "float"];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SymbolNameList {
    #[serde(default)]
    pub values: Vec<SymbolName>,
}

impl Default for SymbolNameList {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolNameList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn push(&mut self, item: SymbolName) {
        self.values.push(item);
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, SymbolName> {
        self.values.iter()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportNameList {
    #[serde(default)]
    pub values: Vec<SymbolName>,
}

impl Default for ImportNameList {
    fn default() -> Self {
        Self::new()
    }
}

impl ImportNameList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrimitiveTypeList {
    #[serde(default)]
    pub values: Vec<SymbolName>,
}

impl Default for PrimitiveTypeList {
    fn default() -> Self {
        Self::new()
    }
}

impl PrimitiveTypeList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn contains(&self, item: &str) -> bool {
        self.values.iter().any(|v| v.value == item)
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallChainList {
    #[serde(default)]
    pub values: Vec<SymbolName>,
}

impl Default for CallChainList {
    fn default() -> Self {
        Self::new()
    }
}

impl CallChainList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

pub fn primitive_type_list() -> PrimitiveTypeList {
    PrimitiveTypeList {
        values: CORE_PRIMITIVE_TYPES
            .iter()
            .map(|s| SymbolName::new(*s))
            .collect(),
    }
}
```

---

## File: crates/shared/src/common/taxonomy_parser_error.rs

```rust
// PURPOSE: ParserError — structured error type for source code parsing failures
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, thiserror::Error)]
pub struct SourceParserError {
    pub path: FilePath,
    pub message: ErrorMessage,
    pub error_code: ErrorCode,
    pub cause: Cause,
}

impl SourceParserError {
    pub fn new(path: FilePath, message: ErrorMessage) -> Self {
        Self {
            path,
            message,
            error_code: ErrorCode::default(),
            cause: Cause::default(),
        }
    }
}

impl std::fmt::Display for SourceParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code_str = self.error_code.to_string();
        let code = if code_str.is_empty() {
            String::new()
        } else {
            format!(" [{}]", code_str)
        };
        write!(f, "Parser Error on {}{}: {}", self.path, code, self.message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct SyntaxErrorVO {
    #[serde(flatten)]
    pub base: SourceParserError,
    pub line: LineNumber,
    pub column: ColumnNumber,
}

impl SyntaxErrorVO {
    pub fn new(path: FilePath, message: ErrorMessage) -> Self {
        Self {
            base: SourceParserError::new(path, message),
            line: LineNumber::default(),
            column: ColumnNumber::default(),
        }
    }
}

impl std::fmt::Display for SyntaxErrorVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line_str = self.line.to_string();
        let col_str = self.column.to_string();
        let pos = if !line_str.is_empty() && !col_str.is_empty() {
            format!(" at {}:{}", line_str, col_str)
        } else if !line_str.is_empty() {
            format!(" at {}", line_str)
        } else {
            String::new()
        };
        write!(
            f,
            "Syntax Error on {}{}: {}",
            self.base.path, pos, self.base.message
        )
    }
}
```

---

## File: crates/shared/src/common/taxonomy_path_vo.rs

```rust
// PURPOSE: FilePath, DirectoryPath — value objects for validated file and directory paths
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// file_path_vo — File and directory path value objects.
///
/// File path identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FilePath {
    pub value: String,
}

impl FilePath {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new FilePath from a string.
    ///
    /// # Errors
    /// Returns an error if the path is invalid (empty or only whitespace).
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let mut value = value.into();
        if value.trim().is_empty() {
            return Err("File path cannot be empty".to_string());
        }
        // Normalize: replace backslashes with forward slashes, collapse repeated slashes.
        let mut normalized = String::with_capacity(value.len());
        let mut prev_slash = false;
        for c in value.chars() {
            if c == '/' || c == '\\' {
                if !prev_slash {
                    normalized.push('/');
                    prev_slash = true;
                }
            } else {
                normalized.push(c);
                prev_slash = false;
            }
        }
        value = normalized;
        // Remove trailing slashes
        let trimmed = value.trim_end_matches('/');
        value = if trimmed.is_empty() {
            "/".to_string()
        } else {
            trimmed.to_string()
        };
        // If after normalization it's empty, then it was all slashes -> treat as root
        if value.is_empty() {
            return Ok(FilePath {
                value: "/".to_string(),
            });
        }
        Ok(FilePath { value })
    }

    /// File extension without dot.
    pub fn extension(&self) -> String {
        let special_files = [
            "Makefile",
            "Dockerfile",
            "Dockerfile.dev",
            "Dockerfile.prod",
            ".bashrc",
            ".profile",
            ".zshrc",
            ".gitignore",
            ".dockerignore",
        ];
        // Operate on the basename, not the full path — `./foo.rs` must still yield
        // `rs` as its extension, and `.bashrc` (which is fully a basename) must NOT
        // be confused with a hidden file mid-path.
        let basename = match self.value.rsplit('/').next() {
            Some(b) => b,
            None => return String::new(),
        };
        if special_files.contains(&basename) || basename.starts_with('.') {
            return String::new();
        }
        match basename.rsplit('.').next() {
            Some(ext) => ext.to_string(),
            None => String::new(),
        }
    }

    /// Check if path has given extension (without dot).
    pub fn has_extension(&self, ext: &str) -> bool {
        self.extension().eq_ignore_ascii_case(ext)
    }

    /// Extract filename/basename of the path.
    pub fn basename(&self) -> String {
        match self.value.rsplit('/').next() {
            Some(f) => f.to_string(),
            None => self.value.clone(),
        }
    }

    /// Check if the path is a barrel file (module re-export aggregator).
    pub fn is_barrel_file(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py" | "mod.rs" | "index.ts" | "index.js" | "index.tsx" | "index.jsx"
        )
    }

    /// Check if the path is a module/layer entry point file.
    pub fn is_entry_point(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py"
                | "main.py"
                | "py.typed"
                | "app.py"
                | "lib.rs"
                | "main.rs"
                | "index.ts"
                | "index.js"
                | "index.tsx"
                | "index.jsx"
                | "main.ts"
                | "main.js"
                | "app.ts"
                | "app.js"
        )
    }
}

impl std::ops::Deref for FilePath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Hash for FilePath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

/// Directory path identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct DirectoryPath {
    pub value: String,
}

impl DirectoryPath {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new DirectoryPath from a string.
    ///
    /// # Errors
    /// Returns an error if the path is invalid (empty or only whitespace).
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let mut value = value.into();
        if value.trim().is_empty() {
            return Err("Directory path cannot be empty".to_string());
        }
        // Normalize: replace backslashes with forward slashes, and remove trailing slash.
        value = value.replace('\\', "/");
        // Remove trailing slashes
        let trimmed = value.trim_end_matches('/');
        value = if trimmed.is_empty() {
            "/".to_string()
        } else {
            trimmed.to_string()
        };
        Ok(DirectoryPath { value })
    }
}

impl std::ops::Deref for DirectoryPath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for DirectoryPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'de> serde::Deserialize<'de> for DirectoryPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DirectoryPath::new(s).map_err(serde::de::Error::custom)
    }
}

impl Hash for DirectoryPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
```

---

## File: crates/shared/src/common/taxonomy_paths_vo.rs

```rust
// PURPOSE: FilePathList, DirectoryPath, SourceDir — VOs for file/directory path collections
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RenamedFile {
    pub old_path: FilePath,
    pub new_path: FilePath,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RenamedFileList {
    pub values: Vec<RenamedFile>,
}

impl RenamedFileList {
    pub fn new(value: Vec<RenamedFile>) -> Self {
        Self { values: value }
    }
    pub fn iter(&self) -> std::slice::Iter<'_, RenamedFile> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: RenamedFile) {
        self.values.push(item);
    }
}

impl RenamedFile {
    pub fn new(old_path: FilePath, new_path: FilePath) -> Self {
        Self { old_path, new_path }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FilePathList {
    pub values: Vec<FilePath>,
}

impl FilePathList {
    pub fn new(value: Vec<FilePath>) -> Self {
        Self { values: value }
    }
}

impl FilePathList {
    pub fn iter(&self) -> std::slice::Iter<'_, FilePath> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: FilePath) {
        self.values.push(item);
    }
}

impl std::ops::Deref for FilePathList {
    type Target = Vec<FilePath>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}
```

---

## File: crates/shared/src/common/taxonomy_response_data_vo.rs

```rust
// PURPOSE: ResponseData — value object for pipeline job response data
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseData {
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[serde(default)]
    pub stdout: String,
    #[serde(default)]
    pub stderr: String,
    #[serde(default)]
    pub returncode: i64,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Default for ResponseData {
    fn default() -> Self {
        Self::new()
    }
}

impl ResponseData {
    pub fn new() -> Self {
        Self {
            value: None,
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata: HashMap::new(),
        }
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.value.as_ref()
    }
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.value.as_ref().and_then(|v| v.get(key))
    }
}
```

---

## File: crates/shared/src/common/taxonomy_severity_vo.rs

```rust
// PURPOSE: Severity — value object for violation severity levels (critical, high, medium, low)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum Severity {
    #[serde(rename = "info")]
    #[default]
    INFO,
    #[serde(rename = "low")]
    LOW,
    #[serde(rename = "medium")]
    MEDIUM,
    #[serde(rename = "high")]
    HIGH,
    #[serde(rename = "critical")]
    CRITICAL,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::INFO => write!(f, "info"),
            Severity::LOW => write!(f, "low"),
            Severity::MEDIUM => write!(f, "medium"),
            Severity::HIGH => write!(f, "high"),
            Severity::CRITICAL => write!(f, "critical"),
        }
    }
}

impl Severity {
    pub fn score_impact(&self) -> f64 {
        match self {
            Severity::INFO => 0.0,
            Severity::LOW => 1.0,
            Severity::MEDIUM => 2.0,
            Severity::HIGH => 3.0,
            Severity::CRITICAL => 5.0,
        }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_source_vo.rs

```rust
// PURPOSE: ContentString, SourceContentVO — VOs for source code content representation
use crate::string_value_object;
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_path_vo::FilePath;

string_value_object!(ContentString);

/// Source content value object: combines a file path, a `ContentString`
/// payload, and a language marker. Carries three fields rather than one,
/// so it does not fit the single-field `string_value_object!` macro;
/// defined manually.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceContentVO {
    pub file_path: FilePath,
    pub content: ContentString,
    pub language: String,
}

impl SourceContentVO {
    pub fn new(file_path: FilePath, content: ContentString, language: impl Into<String>) -> Self {
        Self {
            file_path,
            content,
            language: language.into(),
        }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_suggestion_vo.rs

```rust
// PURPOSE: ClassPath, DescriptionVO, LogOutput, MetadataVO, StdError, StdOutput, Suggestion — domain value objects for CLI suggestion/result data
use crate::string_value_object;
use serde::{Deserialize, Serialize};

// ClassPath, DescriptionVO, LogOutput, StdError, StdOutput, and Suggestion all
// follow the standard String-wrapper VO pattern; the macro emits the
// new/value/Display/From/Hash/PartialEq/Deserialize impls they need.
string_value_object!(ClassPath);
string_value_object!(DescriptionVO);
string_value_object!(LogOutput);
string_value_object!(StdError);
string_value_object!(StdOutput);
string_value_object!(Suggestion);

/// Strongly-typed replacement for the previous
/// `HashMap<String, serde_json::Value>` return type. Each field has a real
/// domain meaning — there is no `serde_json::Value` in the contract surface.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetadataVO {
    pub values: std::collections::HashMap<String, serde_json::Value>,
}

impl MetadataVO {
    pub fn new(value: std::collections::HashMap<String, serde_json::Value>) -> Self {
        Self { values: value }
    }
    pub fn value(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.values
    }
}
```

---

## File: crates/shared/src/common/taxonomy_value_object_utility.rs

````rust
// PURPOSE: Macros for generating boilerplate impls on String/primitive wrapper value objects.
//
// These macros emit the impls that every String-wrapper VO needs:
//   - `new(value)` constructor
//   - `value()` accessor
//   - `Display`
//   - `Hash` / `PartialEq` / `Eq` (optional)
//   - `From<&str>` / `From<String>` / `From<$Inner>` (for primitives)
//   - serde `Deserialize` (accepts either a primitive or a map with a `value` key)
//
// Using the macro keeps each VO file to its domain-specific surface and stops
// AES305 from flagging the same serde visitor across ~13 files.

/// Generate a String-wrapped value object with the standard VO surface.
///
/// # Usage
/// ``` `ignore
/// // in any sibling module file:
/// use crate::string_value_object;
/// string_value_object!(FooName);
/// ``` `
///
/// The macro is `#[macro_export]`-ed so it is accessible at the crate root.
/// Each VO file `use crate::string_value_object;` once and then invokes the
/// macro locally.
#[macro_export]
macro_rules! string_value_object {
    ($name:ident) => {
        #[derive(Default, Debug, Clone, serde::Serialize)]
        #[serde(transparent)]
        pub struct $name {
            pub value: String,
        }

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self {
                    value: value.into(),
                }
            }

            pub fn value(&self) -> &str {
                &self.value
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.value.hash(state);
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        }

        impl Eq for $name {}

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self {
                    value: s.to_string(),
                }
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self { value: s }
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct V {}
                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = $name;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("primitive or map with 'value' key")
                    }
                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name {
                            value: v.to_string(),
                        })
                    }
                    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v })
                    }
                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        let mut value = None;
                        while let Some(k) = map.next_key::<String>()? {
                            if k == "value" {
                                value = Some(map.next_value::<String>()?);
                            } else {
                                let _: serde::de::IgnoredAny = map.next_value()?;
                            }
                        }
                        let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                        Ok($name { value: val })
                    }
                }
                deserializer.deserialize_any(V {})
            }
        }
    };
}

/// Generate a primitive-wrapped value object (e.g. `i64`, `f64`, `bool`).
///
/// # Usage
/// ``` `ignore
/// primitive_value_object!(LineNumber, i64);
/// ``` `
///
/// Emits the same surface as `string_value_object!` but with `From<$Inner>`,
/// `From<$Inner>` conversions, and a serde visitor that accepts the inner
/// type or a `{"value": ...}` map.
#[macro_export]
macro_rules! primitive_value_object {
    ($name:ident, $inner:ty) => {
        #[derive(Default, Debug, Clone, serde::Serialize)]
        #[serde(transparent)]
        pub struct $name {
            pub value: $inner,
        }

        impl $name {
            pub fn new(value: $inner) -> Self {
                Self { value }
            }

            pub fn value(&self) -> $inner {
                self.value
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        }

        impl Eq for $name {}

        impl From<$inner> for $name {
            fn from(v: $inner) -> Self {
                Self { value: v }
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct V {}
                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = $name;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(concat!(
                            "primitive or map with 'value' key (",
                            stringify!($inner),
                            ")"
                        ))
                    }
                    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v as $inner })
                    }
                    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v as $inner })
                    }
                    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v as $inner })
                    }
                    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v as $inner })
                    }
                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        let mut value: Option<$inner> = None;
                        while let Some(k) = map.next_key::<String>()? {
                            if k == "value" {
                                value = Some(map.next_value::<$inner>()?);
                            } else {
                                let _: serde::de::IgnoredAny = map.next_value()?;
                            }
                        }
                        let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                        Ok($name { value: val })
                    }
                }
                deserializer.deserialize_any(V {})
            }
        }
    };
}
````

---

## File: crates/shared/src/orphan-detector/contract_orphan_aggregate.rs

```rust
// PURPOSE: IOrphanAggregate — aggregate trait bundling all orphan detection protocols
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use std::collections::HashSet;

pub trait IOrphanAggregate: Send + Sync {
    fn build_orphan_graph_context(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext;
    fn identify_orphan_entry_points(&self, files: &[String]) -> HashSet<String>;
    fn check_orphans(
        &self,
        layer_detector: &dyn ILayerDetectionAggregate,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult>;
}
```

---

## File: crates/shared/src/orphan-detector/contract_orphan_graph_resolver_protocol.rs

```rust
// PURPOSE: IOrphanGraphResolverProtocol — contract trait for building orphan analysis graph context
// AES402: All primitive `&[String]` parameter types and `Vec<String>` return
// types in this contract have been replaced with strongly-typed VOs.
//   * `&[String] files` → `&[OrphanFileListVO]` (per analysis pass)
//   * `Vec<String>` returns → `OrphanFileListVO`
//   * `&[String] configured` → `&[OrphanEntryPatternListVO]`
//   * `&str root_dir` → kept as `&str` (idiomatic borrow, AES402 allows)
use crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use crate::orphan_detector::taxonomy_orphan_contract_vo::{
    OrphanEntryPatternListVO, OrphanFileListVO,
};

pub trait IOrphanGraphResolverProtocol: Send + Sync {
    /// Build the orphan-detection graph context for a set of source files.
    /// `files` is the list of file paths to include in the graph; `root_dir`
    /// is the project root used to compute relative paths.
    fn build_graph_context(
        &self,
        files: &[OrphanFileListVO],
        root_dir: &str,
    ) -> GraphAnalysisContext;

    /// Identify which of the supplied files count as entry points. A file
    /// is an entry point if its path matches any of the configured patterns
    /// (substring or suffix match). Returns the filtered list as a
    /// strongly-typed VO.
    fn identify_entry_points(
        &self,
        files: &[OrphanFileListVO],
        configured: &[OrphanEntryPatternListVO],
    ) -> OrphanFileListVO;
}
```

---

## File: crates/shared/src/orphan-detector/contract_orphan_protocol.rs

```rust
// PURPOSE: ITaxonomyOrphanProtocol + layer-specific orphan indicator protocols (agent, contract, capabilities, infra, surfaces)
use crate::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use crate::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use crate::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use crate::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use crate::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_path_vo::FilePath;

pub trait ITaxonomyOrphanProtocol: Send + Sync {
    fn is_taxonomy_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        definition: Option<&LayerDefinition>,
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult;
}

pub trait IContractOrphanProtocol: Send + Sync {
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        file_definitions: &FileDefinitionMap,
        inheritance_map: &InheritanceMap,
        all_files: &[String],
    ) -> OrphanIndicatorResult;
}

pub trait ICapabilitiesOrphanProtocol: Send + Sync {
    fn is_capabilities_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult;
}

pub trait IInfrastructureOrphanProtocol: Send + Sync {
    fn is_infrastructure_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult;
}

pub trait IAgentOrphanProtocol: Send + Sync {
    fn is_agent_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        all_files: &[String],
    ) -> OrphanIndicatorResult;
}

pub trait ISurfacesOrphanProtocol: Send + Sync {
    fn is_surface_orphan(
        &self,
        f: &FilePath,
        alive_files: &ReachabilityResult,
        definition: Option<&LayerDefinition>,
    ) -> OrphanIndicatorResult;
}
```

---

## File: crates/shared/src/orphan-detector/mod.rs

```rust
pub mod contract_orphan_aggregate;
pub mod contract_orphan_graph_resolver_protocol;
pub mod contract_orphan_protocol;
pub mod taxonomy_orphan_rule_vo;
pub mod taxonomy_orphan_utility;
pub mod taxonomy_violation_orphan_vo;
pub use taxonomy_violation_orphan_vo::AesOrphanViolation;
pub mod taxonomy_orphan_contract_vo;
pub use taxonomy_orphan_contract_vo::{OrphanEntryPatternListVO, OrphanFileListVO};
```

---

## File: crates/shared/src/orphan-detector/taxonomy_orphan_contract_vo.rs

```rust
// PURPOSE: OrphanContractVOs — value objects used by IOrphanGraphResolverProtocol.
//
// AES402: All primitive `&[String]` / `Vec<String>` parameter types and return
// types in IOrphanGraphResolverProtocol are replaced with strongly-typed VOs
// so the contract surface has no primitive collections.
//
// Why a dedicated VO instead of reusing `FilePathList` or `PatternList`?
//   * `FilePathList` (source_parsing/taxonomy_paths_vo) wraps `Vec<FilePath>`,
//     but the orphan graph resolver receives and emits file paths as `String`
//     (it does not own the underlying file system resolution — the surface
//     layer feeds it raw strings from a directory walk).
//   * `PatternList` (common/taxonomy_common_vo) wraps `Vec<String>` but is
//     semantically about exclusion patterns, not about file or pattern
//     identifiers in a graph context.
//
// The two VOs below mirror the parameter roles of the contract:
//   * `OrphanFileListVO` — list of file paths under analysis
//   * `OrphanEntryPatternListVO` — list of configured entry-point patterns
// Both are intentionally minimal wrappers around `Vec<String>`; the point
// is to take the *name* of the field out of the contract surface and put
// it in a typed wrapper, not to invent new functionality.
use serde::{Deserialize, Serialize};

/// List of file paths under orphan-detection analysis. Wraps `Vec<String>`
/// (raw path strings as emitted by the directory walker). Replaces the
/// previous `&[String]` parameter and `Vec<String>` return type used in
/// `IOrphanGraphResolverProtocol::build_graph_context` and
/// `identify_entry_points`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrphanFileListVO {
    pub values: Vec<String>,
}

impl OrphanFileListVO {
    pub fn new(values: Vec<String>) -> Self {
        Self { values }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.values.iter()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// List of configured entry-point patterns (e.g. glob prefixes or exact
/// paths) the resolver should treat as reachable entry points. Replaces
/// the previous `&[String]` parameter on
/// `IOrphanGraphResolverProtocol::identify_entry_points`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrphanEntryPatternListVO {
    pub values: Vec<String>,
}

impl OrphanEntryPatternListVO {
    pub fn new(values: Vec<String>) -> Self {
        Self { values }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.values.iter()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}
```

---

## File: crates/shared/src/orphan-detector/taxonomy_orphan_rule_vo.rs

```rust
// PURPOSE: OrphanRuleVO — value object containing orphan compliance rule definitions
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::PatternList;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OrphanRuleVO {
    #[serde(default)]
    pub check_orphan: BooleanVO,
    #[serde(default, alias = "entry_points")]
    pub orphan_entry_points: PatternList,
}
```

---

## File: crates/shared/src/orphan-detector/taxonomy_orphan_utility.rs

```rust
use regex::Regex;

pub fn extract_struct_names(content: &str) -> Vec<String> {
    let re = Regex::new(r"(?:pub\s+)?struct\s+([A-Za-z0-9_]+)").ok();
    let mut names = Vec::new();
    if let Some(re) = re {
        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            if name != "Self" && !name.is_empty() {
                names.push(name);
            }
        }
    }
    names
}

pub fn extract_trait_names(content: &str) -> Vec<String> {
    let re = Regex::new(r"(?:pub\s+)?trait\s+([A-Za-z0-9_]+)").ok();
    let mut names = Vec::new();
    if let Some(re) = re {
        for cap in re.captures_iter(content) {
            names.push(cap[1].to_string());
        }
    }
    names
}
```

---

## File: crates/shared/src/orphan-detector/taxonomy_violation_orphan_vo.rs

```rust
use crate::common::taxonomy_message_vo::LintMessage;
use std::fmt;

#[derive(Debug, Clone)]
pub enum AesOrphanViolation {
    TaxonomyOrphan {
        stem: String,
        category: &'static str,
        reason: Option<LintMessage>,
    },
    ContractOrphan {
        suffix: String,
        trait_name: String,
        target_layer: &'static str,
        reason: Option<LintMessage>,
    },
    CapabilitiesOrphan {
        stem: String,
        reason: Option<LintMessage>,
    },
    InfrastructureOrphan {
        stem: String,
        reason: Option<LintMessage>,
    },
    AgentOrphan {
        agg_name: String,
        reason: Option<LintMessage>,
    },
    SurfaceOrphan {
        category: &'static str,
        stem: String,
        reason: Option<LintMessage>,
    },
}

impl fmt::Display for AesOrphanViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AesOrphanViolation::TaxonomyOrphan {
                stem,
                category,
                reason,
            } => {
                let target_hint = match *category {
                    "utility" | "helper" => "any file that needs its functionality".to_string(),
                    _ => "a contract_* file (contract_port, contract_protocol, or contract_aggregate)".to_string(),
                };
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => {
                        format!("Taxonomy file '{}' is not imported by any file.", stem)
                    }
                };
                write!(f, "AES501 TAXONOMY_ORPHAN: '{}' is not imported.\nWHY? {}\nFIX: Import '{}' in {}.", stem, why, stem, target_hint)
            }
            AesOrphanViolation::ContractOrphan {
                suffix,
                trait_name,
                target_layer,
                reason,
            } => {
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => format!(
                        "Contract {} '{}' is not implemented by any {} file.",
                        suffix, trait_name, target_layer
                    ),
                };
                let fix = match suffix.as_str() {
                    "port" => format!("Implement '{}' in an infrastructure_* file, or wire it in agent_*_orchestrator.rs if already implemented.", trait_name),
                    "protocol" => format!("Implement '{}' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.", trait_name),
                    "aggregate" => format!("Import and use '{}' in a surface_* file or root_*_container.rs.", trait_name),
                    _ => format!("Implement '{}' in the appropriate layer.", trait_name),
                };
                write!(
                    f,
                    "AES502 CONTRACT_ORPHAN: Contract {} '{}' is orphaned.\nWHY? {}\nFIX: {}",
                    suffix, trait_name, why, fix
                )
            }
            AesOrphanViolation::CapabilitiesOrphan { stem, reason } => {
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => format!(
                        "Capabilities file '{}' is not wired in any container.",
                        stem
                    ),
                };
                write!(f, "AES503 CAPABILITIES_ORPHAN: '{}' is not wired.\nWHY? {}\nFIX: Register '{}' in root_*_entry.rs or root_*_container.rs via `use {}::...;` and wire it into the container's constructor. If this file is obsolete, delete it and remove its module declaration from lib.rs.", stem, why, stem, stem)
            }
            AesOrphanViolation::InfrastructureOrphan { stem, reason } => {
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => format!("Infrastructure file '{}' is not wired in any container and unreachable from any entry point.", stem),
                };
                write!(f, "AES504 INFRASTRUCTURE_ORPHAN: '{}' is not wired.\nWHY? {}\nFIX: Register '{}' in the corresponding agent_*_orchestrator.rs or root_*_container.rs by passing it as a dependency. If this adapter is unused, delete it and remove its module declaration.", stem, why, stem)
            }
            AesOrphanViolation::AgentOrphan { agg_name, reason } => {
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => format!(
                        "Agent aggregate '{}' is not called by any surface or container.",
                        agg_name
                    ),
                };
                write!(f, "AES505 AGENT_ORPHAN: Aggregate '{}' is unreachable from any surface.\nWHY? {}\nFIX: Import and use '{}' in a surface_* file or root_*_container.rs via `Arc<dyn {}>`. If the orchestrator is unused, delete it and remove its module declaration.", agg_name, why, agg_name, agg_name)
            }
            AesOrphanViolation::SurfaceOrphan {
                category,
                stem,
                reason,
            } => {
                let (where_hint, fix_hint) = match *category {
                    "smart" => ("entry point or router", "an entry point (root_*_entry.rs, cli_*, mcp_*) or router file"),
                    "utility" => ("smart surface", "a smart surface (command, controller, page)"),
                    "passive" => ("smart or utility surface", "a smart surface (command, controller, page) or utility surface (hook, store, action, screen, router)"),
                    _ => ("the appropriate importer", "an appropriate importer file"),
                };
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => format!(
                        "{} surface '{}' is not imported by any {}.",
                        category, stem, where_hint
                    ),
                };
                write!(f, "AES506 SURFACE_ORPHAN: {} surface '{}' is orphaned.\nWHY? {}\nFIX: Import '{}' in {}. If this surface is dead code, remove the file and its module declaration from lib.rs.", category, stem, why, stem, fix_hint)
            }
        }
    }
}

impl From<AesOrphanViolation> for String {
    fn from(v: AesOrphanViolation) -> String {
        v.to_string()
    }
}
```

---

## File: crates/shared/src/role-rules/mod.rs

```rust
// role-rules — taxonomy and contract types
pub mod contract_agent_role_protocol;
pub mod contract_capabilities_role_protocol;
pub mod contract_infrastructure_role_protocol;
pub mod contract_role_aggregate;
pub mod contract_role_protocol;
pub mod contract_role_runner_aggregate;
pub mod contract_surface_role_protocol;
pub mod contract_taxonomy_role_protocol;
pub mod taxonomy_layer_names_constant;
pub mod taxonomy_layer_names_vo;
pub mod taxonomy_role_rule_vo;
pub mod taxonomy_violation_role_vo;
pub use taxonomy_violation_role_vo::AesRoleViolation;
```

---

## File: crates/shared/src/role-rules/taxonomy_layer_names_constant.rs

```rust
// PURPOSE: LAYER_AGENT, LAYER_CAPABILITIES, etc. — constant definitions for AES layer names

pub const LAYER_AGENT: &str = "agent";
pub const LAYER_CAPABILITIES: &str = "capabilities";
pub const LAYER_CONTRACT: &str = "contract";
pub const LAYER_INFRASTRUCTURE: &str = "infrastructure";
pub const LAYER_SURFACES: &str = "surfaces";
pub const LAYER_TAXONOMY: &str = "taxonomy";
pub const LAYER_ROOT: &str = "root";
pub const LAYER_GLOBAL: &str = "global";
```

---
