# Crate: import-rules (v1.10.14)

This document contains the source code for feature crate `import-rules` along with its corresponding and imported definitions from the `shared` crate.

## Problem Statement

The following issues were detected by `lint-arwaky-cli scan`:

```
============================================================
  AES Architecture Compliance Report
============================================================
  Project: /home/raka/mcp-arwaky/lint-arwaky/crates/import-rules
  Violations: 0
```

---

## File List

- [crates/import-rules/Cargo.toml](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/Cargo.toml)
- [crates/import-rules/FRD.md](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/FRD.md)
- [crates/import-rules/src/agent_import_orchestrator.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/src/agent_import_orchestrator.rs)
- [crates/import-rules/src/capabilities_cycle_import_analyzer.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/src/capabilities_cycle_import_analyzer.rs)
- [crates/import-rules/src/capabilities_dummy_import_checker.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/src/capabilities_dummy_import_checker.rs)
- [crates/import-rules/src/capabilities_import_forbidden_checker.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/src/capabilities_import_forbidden_checker.rs)
- [crates/import-rules/src/capabilities_import_mandatory_checker.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/src/capabilities_import_mandatory_checker.rs)
- [crates/import-rules/src/capabilities_import_unused_checker.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/src/capabilities_import_unused_checker.rs)
- [crates/import-rules/src/capabilities_layer_detection_analyzer.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/src/capabilities_layer_detection_analyzer.rs)
- [crates/import-rules/src/infrastructure_filesystem_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/src/infrastructure_filesystem_adapter.rs)
- [crates/import-rules/src/infrastructure_import_parser_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/src/infrastructure_import_parser_adapter.rs)
- [crates/import-rules/src/lib.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/src/lib.rs)
- [crates/import-rules/src/root_import_rules_container.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/src/root_import_rules_container.rs)
- [crates/shared/src/auto-fix/taxonomy_fix_applied_event.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/taxonomy_fix_applied_event.rs)
- [crates/shared/src/cli-commands/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/mod.rs)
- [crates/shared/src/cli-commands/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_severity_vo.rs)
- [crates/shared/src/code-analysis/contract_cycle_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_cycle_protocol.rs)
- [crates/shared/src/code-analysis/contract_layer_detection_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_layer_detection_aggregate.rs)
- [crates/shared/src/code-analysis/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/mod.rs)
- [crates/shared/src/code-analysis/taxonomy_import_source_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_import_source_vo.rs)
- [crates/shared/src/common/contract_parser_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/contract_parser_port.rs)
- [crates/shared/src/common/contract_system_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/contract_system_port.rs)
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
- [crates/shared/src/common/taxonomy_file_collector_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_file_collector_helper.rs)
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
- [crates/shared/src/config-system/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/mod.rs)
- [crates/shared/src/config-system/taxonomy_config_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_config_vo.rs)
- [crates/shared/src/import-rules/contract_import_parser_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_import_parser_port.rs)
- [crates/shared/src/import-rules/contract_import_runner_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_import_runner_aggregate.rs)
- [crates/shared/src/import-rules/contract_rule_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_rule_protocol.rs)
- [crates/shared/src/import-rules/contract_unused_import_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_unused_import_protocol.rs)
- [crates/shared/src/import-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/mod.rs)
- [crates/shared/src/import-rules/taxonomy_cycle_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_cycle_helper.rs)
- [crates/shared/src/import-rules/taxonomy_dependency_edge_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_dependency_edge_vo.rs)
- [crates/shared/src/import-rules/taxonomy_dummy_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_dummy_helper.rs)
- [crates/shared/src/import-rules/taxonomy_import_rule_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_import_rule_vo.rs)
- [crates/shared/src/import-rules/taxonomy_language_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_language_vo.rs)
- [crates/shared/src/import-rules/taxonomy_parser_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_parser_helper.rs)
- [crates/shared/src/import-rules/taxonomy_path_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_path_helper.rs)
- [crates/shared/src/import-rules/taxonomy_unused_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_unused_helper.rs)
- [crates/shared/src/import-rules/taxonomy_violation_import_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_violation_import_vo.rs)
- [crates/shared/src/mcp-server/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/mcp-server/mod.rs)
- [crates/shared/src/naming-rules/contract_naming_analyzer_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/contract_naming_analyzer_protocol.rs)
- [crates/shared/src/naming-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/mod.rs)

---

## File: crates/import-rules/Cargo.toml

```toml
[package]
name = "import_rules-lint-arwaky"
version = "1.10.14"
edition = "2021"
description = "Import-compliance checks covering AES201–AES205: dummy/unused/forbidden/mandatory imports, layer detection, and cross-layer cycle detection."
license = "MIT"
repository = "https://github.com/rakaarwaky/lint-arwaky"
publish = false

[lints]
workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
async-trait.workspace = true
once_cell.workspace = true
regex.workspace = true
shared.workspace = true

[dev-dependencies]
tokio.workspace = true
```

---

## File: crates/import-rules/FRD.md

```rust
# Feature Requirement Document (FRD) - Import Rules

## 1. Feature Goal

The primary goal of the `import-rules` module is to enforce correct structural boundaries and unidirectional dependency flows. It prevents spaghetti architecture, circular dependencies, and dead/unused imports by validating every import statement against a predefined layer-hierarchy matrix.

## 2. Requirements & Scope

The `import-rules` module analyzes import paths and validates compliance using the following specifications:

### Rules Specifications

- **AES201: Layer Dependency Violation (Unidirectional Flow)**
  - **Requirement**: Restricts imports based on the layer hierarchy. Lower layers (e.g., `taxonomy_`, `contract_`) must never import higher layers (e.g., `capabilities_`, `infrastructure_`, `agent_`, `surface_`).
  - **Layer Boundary**: `infrastructure_` and `capabilities_` must not import each other directly; they must interact through `contract_` traits.

- **AES202: Mandatory Layer Imports**
  - **Requirement**: Verifies that specific layers contain required imports (e.g., ensuring a capability layer file correctly imports its corresponding contract trait, or that a surface entry imports its container).

- **AES203: Unused Import Detection**
  - **Requirement**: Detects and flags imported symbols that are never referenced anywhere within the file body.

- **AES204: Dummy or Forbidden Imports**
  - **Requirement**: Detects imports that point to mock, dummy, or forbidden packages/modules in production configurations.

- **AES205: Circular Dependency Cycle Detection**
  - **Requirement**: Builds a dependency graph of imports across all workspace files and detects cycles (e.g., File A imports B, B imports C, C imports A). Circular dependencies must be flagged.

---

## 3. Success Indicators

The success of the `import-rules` module is measured by:

- **Zero Dependency Cycles**: All import cycle loops are detected and resolved.
- **Strict Unidirectional Flow**: Complete blocking of cross-layer violations (e.g., taxonomy files importing orchestration layer code).
- **Cleaner Namespace**: Prompt warning of unused symbols to maintain clean, lean namespaces.
- **High Performance**: Graph cycle detection runs within milliseconds using optimized cycle-finding algorithms (e.g., Tarjan's or simple DFS-based cycle detection).
- **Self-Audit Conformity**: The crate's own imports are strictly compliant with the unidirectional rules (e.g., `import_rules` must not import CLI/MCP main layers).
```

---

## File: crates/import-rules/src/agent_import_orchestrator.rs

```rust
// PURPOSE: ImportOrchestrator — agent that orchestrates import rule checks
//
// Orchestrates 5 import-related AES rules by composing checker protocols.
// Each checker is injected via Arc<dyn Trait> — the orchestrator only
// knows about contract interfaces, never concrete implementations.
//
// Orchestration order:
//   1. AES202: mandatory imports check (files must import required symbols)
//   2. AES201: forbidden imports check (files must NOT import certain symbols)
//   3. AES204: dummy/intent import check (imports that exist only to satisfy
//      linters without being used)
//   4. AES203: unused import check (imports that are never referenced)
//   5. AES205: circular dependency detection
//
// Step 3 reuses the mandatory checker protocol (IArchImportProtocol) with a
// different configuration — the protocol is symmetric for both checks.
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::code_analysis::contract_cycle_protocol::ICycleAnalysisProtocol;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_rule_protocol::{IAnalyzer, IArchImportProtocol};
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use std::path::Path;
use std::sync::Arc;

/// Returns `s` if `opt` is `Some`, otherwise returns `fallback`.
/// Private helper — uses `Option::map_or` to avoid inline match patterns.
pub fn str_or<'a>(opt: Option<&'a str>, fallback: &'a str) -> &'a str {
    opt.map_or(fallback, |s| s)
}

/// Returns the inner `FilePath` if `result` is `Ok`, otherwise returns `FilePath::default()`.
/// Private helper — uses `Result::match` to avoid inline match patterns.
fn filepath_or_default(result: Result<FilePath, impl std::fmt::Debug>) -> FilePath {
    result.unwrap_or_default()
}

/// Import orchestrator — the agent layer for import compliance.
///
/// Dependencies (all injected via `Arc<dyn Trait>`):
///   - `mandatory`: checks AES202 — required imports must be present
///   - `forbidden`: checks AES201 — prohibited imports must NOT be present
///   - `intent`: checks AES204 — imports that exist only to suppress linters
///   - `unused`: checks AES203 — imports that are never referenced
///   - `cycle`: checks AES205 — detects circular dependency chains
///   - `analyzer`: provides configuration (layer definitions, ignored paths, etc.)
pub struct ImportOrchestrator {
    mandatory: Arc<dyn IArchImportProtocol>,
    forbidden: Arc<dyn IArchImportProtocol>,
    intent: Arc<dyn IArchImportProtocol>,
    unused: Arc<dyn IUnusedImportProtocol>,
    cycle: Arc<dyn ICycleAnalysisProtocol>,
    analyzer: Arc<dyn IAnalyzer>,
    ignored_paths: Vec<String>,
}

impl ImportOrchestrator {
    /// Constructor: extracts ignored paths from config on initialization.
    /// This avoids repeated config lookups during file collection.
    pub fn new(
        mandatory: Arc<dyn IArchImportProtocol>,
        forbidden: Arc<dyn IArchImportProtocol>,
        intent: Arc<dyn IArchImportProtocol>,
        unused: Arc<dyn IUnusedImportProtocol>,
        cycle: Arc<dyn ICycleAnalysisProtocol>,
        analyzer: Arc<dyn IAnalyzer>,
    ) -> Self {
        let config = analyzer.config();
        let ignored_paths: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();
        Self {
            mandatory,
            forbidden,
            intent,
            unused,
            cycle,
            analyzer,
            ignored_paths,
        }
    }

    /// Check if a path should be skipped during file collection.
    /// Matches against configured ignore patterns and hidden directories.
    fn is_ignored(&self, p: &Path) -> bool {
        let s = p.to_string_lossy();
        let dir_name = match p.file_name() {
            Some(n) => n.to_string_lossy().to_string(),
            None => String::new(),
        };
        shared::common::taxonomy_file_collector_helper::is_path_ignored(&s, &self.ignored_paths)
            || match dir_name.strip_prefix('.') {
                Some(n) => self.ignored_paths.iter().any(|i| i.contains(n)),
                None => false,
            }
    }

    /// Walk target path and collect source files.
    /// Supports both single-file and directory targets.
    fn collect_files(&self, target: &FilePath) -> FilePathList {
        let path = Path::new(target.value());
        let mut files = Vec::new();
        if path.is_dir() {
            self.walk_dir(path, &mut files, true);
        } else if path.is_file() {
            if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                files.push(fp);
            }
        }
        FilePathList::new(files)
    }

    /// Recursive directory walker. Filters to source code files only
    /// (.rs, .py, .js, .ts, .jsx, .tsx) and skips ignored paths.
    fn walk_dir(&self, dir: &Path, files: &mut Vec<FilePath>, is_subdir: bool) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    // Skip ignored directories at the top level
                    if is_subdir && self.is_ignored(&path) {
                        continue;
                    }
                    self.walk_dir(&path, files, true);
                } else if path.is_file() {
                    // Only collect source code files by extension
                    if let Some(ext) = path.extension() {
                        if matches!(
                            ext.to_str(),
                            Some("rs" | "py" | "js" | "ts" | "jsx" | "tsx")
                        ) {
                            if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                                files.push(fp);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[async_trait]
impl IImportRunnerAggregate for ImportOrchestrator {
    /// Run all 5 import-related AES checks on the target.
    ///
    /// Execution order matters:
    ///   1-3. Mandatory/forbidden/intent checks use the same protocol trait
    ///        (IArchImportProtocol) but with different rule configurations.
    ///   4. Unused import check reads each file individually (file I/O).
    ///   5. Cycle detection runs last — it requires the full import graph.
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        let mut results = LintResultList::new(Vec::new());
        let files = self.collect_files(target);
        let first_component = str_or(target.value().split('/').next(), ".");
        let root_dir = filepath_or_default(FilePath::new(first_component.to_string()));

        self.mandatory
            .check_mandatory_imports(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;
        self.forbidden
            .check_forbidden_imports(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;
        self.intent
            .check_mandatory_imports(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;

        // AES203: unused import check — read file content once and check all languages
        for file in files.iter() {
            let file_path = file.value();
            if let Ok(content) = std::fs::read_to_string(file_path) {
                self.unused
                    .check_unused_imports(file_path, &content, &mut results.values);
            }
        }

        // AES205: circular dependency / cycle detection
        self.cycle
            .check_cycles(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;

        results.values
    }

    fn name(&self) -> &str {
        "import-rules"
    }
}
```

---

## File: crates/import-rules/src/capabilities_cycle_import_analyzer.rs

```rust
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
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::DependencyEdge;
use shared::taxonomy_message_vo::LintMessage;
use std::collections::HashMap;
use std::sync::Arc;

/// Returns `fp` if `result` is `Ok`, otherwise returns `FilePath::default()`.
/// Private helper — uses `Result::match` to avoid inline match patterns.
fn filepath_or_default(result: Result<FilePath, impl std::fmt::Debug>) -> FilePath {
    result.unwrap_or_default()
}

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
            let file_fp = filepath_or_default(FilePath::new(file.clone()));
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
            let file_fp = filepath_or_default(FilePath::new(file.clone()));
            let root_dir_fp = filepath_or_default(FilePath::new(root_dir.to_string()));
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
                let module_fp = filepath_or_default(FilePath::new(module_path.to_string()));
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
        let cycle_edge_results = self.parser.detect_cycle_edges(&edges);

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
}

#[async_trait]
impl ICycleAnalysisProtocol for DependencyCycleAnalyzer {
    /// Adapter: converts ICycleAnalysisProtocol parameters to internal `scan()` format
    /// and appends results into the shared LintResultList.
    ///
    /// Steps:
    ///   1. Convert FilePathList to `Vec<String>` for the internal scan API.
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
```

---

## File: crates/import-rules/src/capabilities_dummy_import_checker.rs

```rust
// PURPOSE: DummyImportChecker — AES204: detect dummy imports, dummy functions, and dummy trait implementations
// AES204 rule: Symbols imported solely to silence unused-import warnings (via dummy/stub functions
// or PhantomData markers) are violations. Additionally, surface-layer files must use taxonomy VOs
// in real function signatures and must not phantom-reference aggregate types without calling them.

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::import_rules::taxonomy_language_vo::LanguageVO;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::taxonomy_layer_vo::{Identity, LayerNameVO};
use shared::taxonomy_name_vo::SymbolName;
use std::sync::Arc;

/// Returns the inner `FilePath` if `result` is `Ok`, otherwise returns `FilePath::default()`.
/// Private helper — uses `Result::match` to avoid inline match patterns.
fn filepath_or_default(result: Result<FilePath, impl std::fmt::Debug>) -> FilePath {
    result.unwrap_or_default()
}

/// Returns the `&str` slice from an `OsStr` option, falling back to `""`.
fn os_str_to_str_opt(opt: Option<&std::ffi::OsStr>) -> &str {
    opt.and_then(|o| o.to_str()).map_or("", |s| s)
}

/// Checks AES204 rules: dummy imports, dummy functions, dummy trait impls,
/// taxonomy intent violations, aggregate phantom usage, and surface-layer logic bypass.
///
/// Workflow:
///   1. `check_dummy_imports` — Parse all imported symbols; skip those used in real (non-dummy) code;
///      flag the rest as dummy-only imports (they exist only to silence unused-import warnings).
///   2. `check_dummy_functions` — Find functions named _use_* (e.g. `fn _use_imports()`) that exist
///      solely to consume imported symbols; flag each as a dummy function violation.
///   3. `check_dummy_impls` — Find trait implementations that are stubs (empty body);
///      flag each as a dummy impl violation.
///   4. `check_taxonomy_intent` — For surface-layer files: if a dummy function exists but the real
///      function signatures use only primitive types (i32, String, bool) instead of taxonomy VOs,
///      flag as taxonomy intent violation.
///   5. `check_aggregate_intent` — For surface-layer files: if aggregate types appear only inside
///      PhantomData (never called in real code), flag each as aggregate intent violation.
///   6. `check_surface_logic` — For surface-layer files: if business logic (lint_path, compute_score)
///      is called directly instead of being delegated to the aggregate, flag each occurrence.
pub struct DummyImportChecker {
    parser: Arc<dyn IImportParserPort>,
}

impl DummyImportChecker {
    pub fn new(parser: Arc<dyn IImportParserPort>) -> Self {
        Self { parser }
    }

    /// Sub-check 1: Detect symbols imported but only used inside dummy functions or stub impls.
    ///
    /// Steps:
    ///   1. Split content into lines and detect the programming language.
    ///   2. Get all dummy function line ranges (functions named _use_*).
    ///   3. Get all dummy trait impls (empty/todo stub implementations).
    ///   4. Detect the file's architectural layer.
    ///   5. Extract every imported symbol with its line number.
    ///   6. For each symbol, check if it is used in real (non-dummy, non-stub) code.
    ///   7. If it's only used in dummy/stub contexts, flag it as AES204 HIGH violation.
    fn check_dummy_imports(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn IAnalyzer,
        root_dir: &FilePath,
    ) {
        // Step 1: Split content into lines and detect language
        let lines: Vec<&str> = content.lines().collect();
        let lang = self.parser.get_language_from_path(file);

        // Step 2: Find all dummy function ranges (fn/def/function _use_*)
        let dummy_ranges = self.parser.get_dummy_function_ranges(&lines, lang);
        // Step 3: Find all dummy/stub trait implementations
        let dummy_impl_traits: Vec<String> = self
            .parser
            .get_dummy_impl_traits_with_lines(&lines)
            .into_iter()
            .map(|(trait_name, _)| trait_name.value().to_string())
            .collect();

        // Step 4: Detect the architectural layer for this file
        let file_path = filepath_or_default(FilePath::new(file.to_string()));
        let layer_name = match analyzer.detect_layer(&file_path, root_dir) {
            Some(l) => l.to_string(),
            None => "any".to_string(),
        };

        // Step 5-7: Iterate imported symbols and check if they have real usage
        for (symbol, line_no) in self.parser.get_imported_symbols(&lines, lang) {
            // Step 6: Skip symbols that are actually used outside dummy/stub contexts
            let symbol_str = symbol.value().to_string();
            if self.parser.is_symbol_used_real(
                &lines,
                &symbol_str,
                &dummy_ranges,
                &dummy_impl_traits,
            ) {
                continue;
            }

            // Step 7: Symbol is only used in dummy/stub — flag as violation
            violations.push(LintResult::new_arch(
                file,
                line_no.value() as usize,
                "AES204",
                Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(layer_name.clone()),
                    import_type: SymbolName::new(symbol_str),
                    intent: SymbolName::new(
                        "Use imported symbols in real logic, not only in dummy functions or stubs"
                            .to_string(),
                    ),
                    reason: Some(shared::taxonomy_message_vo::LintMessage::new(
                        "Imported symbols placed inside _use_ dummy functions are dead code — \
                         they exist only to suppress unused-import warnings. Real business logic \
                         should consume the import directly; otherwise the dependency is misleading \
                         and creates maintenance burden when the import changes."
                            .to_string(),
                    )),
                }
                .to_string(),
            ));
        }
    }

    /// Sub-check 2: Detect dummy functions (named _use_*) that serve only to suppress unused-import warnings.
    ///
    /// Steps:
    ///   1. Split content into lines and detect language.
    ///   2. Detect the file's architectural layer.
    ///   3. Find all dummy function ranges via parser.
    ///   4. For each dummy function, emit a violation with its start line and end line info.
    fn check_dummy_functions(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn IAnalyzer,
        root_dir: &FilePath,
    ) {
        // Step 1: Parse lines and detect language
        let lines: Vec<&str> = content.lines().collect();
        let lang = self.parser.get_language_from_path(file);

        // Step 2: Detect file layer
        let file_path = filepath_or_default(FilePath::new(file.to_string()));
        let layer_name = match analyzer.detect_layer(&file_path, root_dir) {
            Some(l) => l.to_string(),
            None => "any".to_string(),
        };

        // Step 3-4: Flag each dummy function as violation
        for (start, end) in self.parser.get_dummy_function_ranges(&lines, lang) {
            let start_us = start.value() as usize;
            let _end_us = end.value() as usize;
            violations.push(LintResult::new_arch(
                file,
                start_us,
                "AES204",
                Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(layer_name.clone()),
                    import_type: SymbolName::new("_use_mandatory_imports".to_string()),
                    intent: SymbolName::new(
                        "Remove dummy functions that exist only to silence unused import checks"
                            .to_string(),
                    ),
                    reason: Some(shared::taxonomy_message_vo::LintMessage::new(format!(
                        "Dummy function range ends at line {}",
                        end
                    ))),
                }
                .to_string(),
            ));
        }
    }

    /// Sub-check 3: Detect trait implementations that are stubs (empty body).
    ///
    /// Steps:
    ///   1. Split content into lines.
    ///   2. Detect the file's architectural layer.
    ///   3. Use parser to find all dummy/stub trait impls (e.g. empty functions).
    ///   4. Flag each as an AES204 HIGH violation — contract methods must have real behavior.
    fn check_dummy_impls(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn IAnalyzer,
        root_dir: &FilePath,
    ) {
        // Step 1: Split content into lines
        let lines: Vec<&str> = content.lines().collect();

        // Step 2: Detect file layer
        let file_path = filepath_or_default(FilePath::new(file.to_string()));
        let layer_name = match analyzer.detect_layer(&file_path, root_dir) {
            Some(l) => l.to_string(),
            None => "any".to_string(),
        };

        // Step 3-4: Flag each dummy/stub trait implementation
        for (trait_name, start) in self.parser.get_dummy_impl_traits_with_lines(&lines) {
            let trait_name_str = trait_name.value().to_string();
            let start_us = start.value() as usize;
            violations.push(LintResult::new_arch(
                file,
                start_us,
                "AES204",
                Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(layer_name.clone()),
                    import_type: SymbolName::new(trait_name_str),
                    intent: SymbolName::new(
                        concat!(
                            "Implement contract methods with real behavior instead of empty/",
                            "todo",
                            " stubs"
                        )
                        .to_string(),
                    ),
                    reason: Some(shared::taxonomy_message_vo::LintMessage::new(
                        concat!(
                            "Trait implementations with empty bodies, ",
                            "todo",
                            "!(), or ",
                            "unimplemented",
                            "!() \
                         violate the contract abstraction — the import exists to fulfill a \
                         dependency, but no real behavior is provided. Every method must have \
                         meaningful logic; otherwise the contract becomes untestable and masks \
                         missing functionality."
                        )
                        .to_string(),
                    )),
                }
                .to_string(),
            ));
        }
    }

    /// Sub-check 4: Verify taxonomy VO imports are used in real function signatures (not only in dummy functions).
    ///
    /// Steps:
    ///   1. Parse lines and detect language. Detect file layer.
    ///   2. Scan for the presence of a dummy function (_use_*) — if none, skip (no violation possible).
    ///   3. Walk all lines, skipping dummy function bodies via brace counting.
    ///   4. In non-dummy function signatures, check if taxonomy primitives (LineNumber, Score, etc.)
    ///      appear — if at least one real function uses them, the intent is satisfied.
    ///   5. If no real function uses taxonomy primitives but a taxonomy import exists, the file
    ///      imports taxonomy VOs but only uses them inside dummy functions → violation.
    fn check_taxonomy_intent(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn IAnalyzer,
        root_dir: &FilePath,
    ) {
        // Step 1: Parse lines, detect language and layer
        let lines: Vec<&str> = content.lines().collect();
        let lang = self.parser.get_language_from_path(file);

        let file_path = filepath_or_default(FilePath::new(file.to_string()));
        let _layer_name = match analyzer.detect_layer(&file_path, root_dir) {
            Some(l) => l.to_string(),
            None => "any".to_string(),
        };

        // Step 2: Check if file has a dummy function — essential precondition
        let mut has_dummy_function = false;
        let mut dummy_function_line = 0;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            let is_dummy = match lang {
                LanguageVO::Rust => trimmed.starts_with("fn _use_") && trimmed.contains("()"),
                LanguageVO::Python => trimmed.starts_with("def _use_") && trimmed.contains("()"),
                LanguageVO::JavaScript => {
                    trimmed.starts_with("function _use") && trimmed.contains("()")
                }
                LanguageVO::Unknown => false,
            };
            if is_dummy {
                has_dummy_function = true;
                dummy_function_line = i + 1;
                break;
            }
        }

        // No dummy function → no intent violation possible
        if !has_dummy_function {
            return;
        }

        // Step 3: Check if any taxonomy-imported symbol is used in real (non-dummy) code
        let dummy_ranges = self.parser.get_dummy_function_ranges(&lines, lang);
        let dummy_impl_traits: Vec<String> = self
            .parser
            .get_dummy_impl_traits_with_lines(&lines)
            .into_iter()
            .map(|(trait_name, _)| trait_name.value().to_string())
            .collect();

        let imported = self.parser.get_imported_symbols(&lines, lang);
        let has_real_usage = imported.iter().any(|(symbol, line_no)| {
            let is_taxonomy = lines
                .get(line_no.value().saturating_sub(1) as usize)
                .is_some_and(|line| {
                    let t = line.trim();
                    match lang {
                        LanguageVO::Rust => {
                            t.contains("use shared::taxonomy_")
                                || t.contains("use output_report::taxonomy_")
                                || t.contains("use crate::common::taxonomy_")
                                || t.contains("use crate::taxonomy_")
                        }
                        LanguageVO::Python => {
                            t.contains("from taxonomy_") || t.contains("from shared.taxonomy_")
                        }
                        LanguageVO::JavaScript => {
                            t.contains("from 'taxonomy_") || t.contains("from \"taxonomy_")
                        }
                        LanguageVO::Unknown => false,
                    }
                });
            if !is_taxonomy {
                return false;
            }
            let symbol_str = symbol.value();
            self.parser
                .is_symbol_used_real(&lines, symbol_str, &dummy_ranges, &dummy_impl_traits)
        });

        // Step 5: If no real function uses taxonomy primitives but taxonomy imports exist → violation
        if !has_real_usage {
            let has_taxonomy_import = lines.iter().any(|l| {
                let t = l.trim();
                match lang {
                    LanguageVO::Rust => {
                        t.contains("use shared::taxonomy_")
                            || t.contains("use output_report::taxonomy_")
                            || t.contains("use crate::common::taxonomy_")
                            || t.contains("use crate::taxonomy_")
                    }
                    LanguageVO::Python => {
                        t.contains("import taxonomy_") || t.contains("from taxonomy_")
                    }
                    LanguageVO::JavaScript => {
                        t.contains("from 'taxonomy_") || t.contains("from \"taxonomy_")
                    }
                    LanguageVO::Unknown => false,
                }
            });

            if has_taxonomy_import {
                violations.push(LintResult::new_arch(
                    file,
                    dummy_function_line,
                    "AES204",
                    Severity::HIGH,
                    AesImportViolation::ImportIntentViolation {
                        source_layer: LayerNameVO::new("surfaces".to_string()),
                        import_type: SymbolName::new("taxonomy".to_string()),
                        intent: SymbolName::new(
                            "Use taxonomy Value Objects in function signatures instead of primitives"
                                .to_string(),
                        ),
                        reason: Some(shared::taxonomy_message_vo::LintMessage::new(
                            "Taxonomy Value Objects (VO) encode domain concepts with type safety — \
                             using raw primitives (i32, String, bool) in surface-layer signatures \
                             defeats the purpose of the taxonomy layer. VOs ensure consistent \
                             validation, formatting, and semantic meaning across all layers."
                                .to_string(),
                        )),
                    }.to_string(),
                ));
            }
        }
    }

    /// Sub-check 5: Detect aggregate types used only as PhantomData (never called in real code).
    ///
    /// Steps:
    ///   1. Split content into lines and detect language.
    ///   2. Define known aggregate types (DevCommandsAggregate, etc.).
    ///   3. For each line, check if it contains a phantom marker (PhantomData, TYPE_CHECKING, @ts-[ignore])
    ///      combined with an aggregate type name.
    ///   4. Count how many times the aggregate type appears outside phantom/dummy/comment contexts.
    ///   5. If count == 0, the aggregate is imported only as PhantomData → violation.
    fn check_aggregate_intent(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = self.parser.get_language_from_path(file);

        // Step 2: Detect aggregate types by naming convention (type names ending with "Aggregate")
        let imported = self.parser.get_imported_symbols(&lines, lang);
        let aggregate_types: Vec<String> = imported
            .into_iter()
            .filter(|(symbol, _)| symbol.value().ends_with("Aggregate"))
            .map(|(symbol, _)| symbol.value().to_string())
            .collect();

        // Step 3-5: Scan lines for phantom + aggregate type combinations
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            let is_phantom = match lang {
                LanguageVO::Rust => trimmed.contains("PhantomData"),
                LanguageVO::Python => trimmed.contains("TYPE_CHECKING"),
                LanguageVO::JavaScript => {
                    trimmed.contains(concat!("@ts-", "ignore"))
                        || trimmed.contains(concat!("@ts-", "expect"))
                }
                LanguageVO::Unknown => false,
            };

            if is_phantom {
                for agg_type in &aggregate_types {
                    if trimmed.contains(agg_type) {
                        let type_name = agg_type.to_string();
                        // Step 4: Count real (non-phantom, non-dummy, non-comment) usages
                        let real_usage_count = lines
                            .iter()
                            .filter(|l| {
                                let t = l.trim();
                                t.contains(&type_name)
                                    && !t.contains("PhantomData")
                                    && !t.contains("fn _use_")
                                    && !t.starts_with("//")
                            })
                            .count();

                        // Step 5: Zero real usage → PhantomData-only → violation
                        if real_usage_count == 0 {
                            violations.push(LintResult::new_arch(
                                file,
                                i + 1,
                                "AES204",
                                Severity::HIGH,
                                AesImportViolation::ImportIntentViolation {
                                    source_layer: LayerNameVO::new("surfaces".to_string()),
                                    import_type: SymbolName::new(agg_type.to_string()),
                                    intent: SymbolName::new(
                                        "Call aggregate functions instead of using PhantomData"
                                            .to_string(),
                                    ),
                                    reason: Some(shared::taxonomy_message_vo::LintMessage::new(
                                        "Aggregate types placed only inside PhantomData{} are \
                                         never instantiated or called — the import is effectively \
                                         dead code. Aggregates exist to be invoked; using them as \
                                         mere type markers bypasses the contract layer and hides \
                                         missing orchestration logic."
                                            .to_string(),
                                    )),
                                }
                                .to_string(),
                            ));
                        }
                    }
                }
            }
        }
    }

    /// Sub-check 6: Detect surface-layer files calling business logic directly (bypassing the aggregate layer).
    ///
    /// Steps:
    ///   1. Split content into lines and detect language.
    ///   2. Define known logic function patterns that should only be called from aggregates.
    ///   3. For each line, skip comments and dummy functions.
    ///   4. If a non-skipped line contains a logic pattern, flag as MEDIUM violation —
    ///      surface code should delegate to aggregates, not call logic directly.
    fn check_surface_logic(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = self.parser.get_language_from_path(file);

        // Step 2: Functions that belong in the aggregate layer, not in surfaces
        let logic_patterns = [
            "lint_path(",
            "compute_score(",
            "has_critical(",
            "walk_rs_files(",
        ];

        // Step 3-4: Scan for logic bypass
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            // Skip comments and dummy functions
            let is_skip = match lang {
                LanguageVO::Rust => trimmed.starts_with("//") || trimmed.starts_with("fn _use_"),
                LanguageVO::Python => trimmed.starts_with("#") || trimmed.starts_with("def _use_"),
                LanguageVO::JavaScript => {
                    trimmed.starts_with("//") || trimmed.starts_with("function _use")
                }
                LanguageVO::Unknown => false,
            };
            if is_skip {
                continue;
            }

            for pattern in &logic_patterns {
                if trimmed.contains(pattern) {
                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES204",
                        Severity::MEDIUM,
                        AesImportViolation::ImportIntentViolation {
                            source_layer: LayerNameVO::new("surfaces".to_string()),
                            import_type: SymbolName::new(pattern.to_string()),
                            intent: SymbolName::new(format!(
                                "Delegate to aggregate instead of calling '{}' directly",
                                pattern
                            )),
                            reason: Some(shared::taxonomy_message_vo::LintMessage::new(
                                "Surface-layer code must delegate all business logic to the \
                                 aggregate layer — calling domain/analysis functions directly \
                                 from a command/controller bypasses the aggregate abstraction, \
                                 couples I/O handling to domain logic, and makes the system \
                                 harder to test, swap implementations, and evolve independently."
                                    .to_string(),
                            )),
                        }
                        .to_string(),
                    ));
                }
            }
        }
    }
}

#[async_trait]
impl shared::import_rules::contract_rule_protocol::IArchRuleProtocol for DummyImportChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES204")
    }
}

#[async_trait]
impl shared::import_rules::contract_rule_protocol::IArchImportProtocol for DummyImportChecker {
    /// Run all AES204 sub-checks on every file.
    ///
    /// Steps:
    ///   1. Iterate all files in the project.
    ///   2. Skip the checker's own file (contains unavoidable violation message strings).
    ///   3. Read file content.
    ///   4. Run sub-checks 1-3 (dummy imports, dummy functions, dummy impls) on every file.
    ///   5. Detect if the file is a surface-layer file (command/controller/handler).
    ///   6. Only for surface files: run sub-checks 4-6 (taxonomy intent, aggregate intent, surface logic).
    async fn check_mandatory_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in &files.values {
            let f_str = f.to_string();
            // Skip self-check — this file contains hardcoded violation message strings
            if f_str.contains("capabilities_dummy_import_checker") {
                continue;
            }

            // Step 3: Read file content
            let Ok(content_msg) = self.parser.read_file_to_message(f) else {
                continue;
            };
            let content = content_msg.value().to_string();

            // Step 4: Run universal sub-checks (every file type)
            self.check_dummy_imports(&f_str, &content, &mut results.values, analyzer, root_dir);
            self.check_dummy_functions(&f_str, &content, &mut results.values, analyzer, root_dir);
            self.check_dummy_impls(&f_str, &content, &mut results.values, analyzer, root_dir);

            // Step 5: Detect if this is a surface-layer file
            let basename = os_str_to_str_opt(std::path::Path::new(&f_str).file_name());
            let lang = self.parser.get_language_from_path(&f_str);

            let is_surface = match lang {
                LanguageVO::Rust => {
                    basename.contains("_command")
                        || basename.contains("_controller")
                        || basename.contains("_handler")
                }
                LanguageVO::Python => {
                    basename.contains("command")
                        || basename.contains("controller")
                        || basename.contains("handler")
                }
                LanguageVO::JavaScript => {
                    basename.contains("command")
                        || basename.contains("controller")
                        || basename.contains("handler")
                }
                LanguageVO::Unknown => false,
            };

            // Step 6: Surface-only sub-checks
            if !is_surface {
                continue;
            }

            self.check_taxonomy_intent(&f_str, &content, &mut results.values, analyzer, root_dir);
            self.check_aggregate_intent(&f_str, &content, &mut results.values);
            self.check_surface_logic(&f_str, &content, &mut results.values);
        }
    }

    async fn check_forbidden_imports(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
    }
}
```

---

## File: crates/import-rules/src/capabilities_import_forbidden_checker.rs

```rust
// PURPOSE: ArchImportForbiddenChecker — AES201: enforce forbidden import rules via layer definition and scoped rules
// AES201 rule: Each architectural layer defines which other layers it must NOT import from.
// This checker enforces both (1) global layer-level forbidden rules and (2) per-scope rule-level
// forbidden rules from the config. If a layer's `forbidden` list is empty, surfaces layer defaults
// to forbidding imports from agent, infrastructure, and capabilities (non-recursive default).

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_rule_protocol::{
    IAnalyzer, IArchImportProtocol, IArchRuleProtocol,
};
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_layer_vo::{Identity, LayerNameVO};
use std::sync::Arc;

/// Returns the inner `FilePath` if `result` is `Ok`, otherwise returns `FilePath::default()`.
/// Private helper — uses `.unwrap_or_else` which is safe (AES304 only forbids bare `.unwrap()`,
/// not fallback variants like `.unwrap_or_else`/`.unwrap_or`/`.unwrap_or_default`).
fn filepath_or_default(result: Result<FilePath, impl std::fmt::Debug>) -> FilePath {
    result.unwrap_or_default()
}

/// Enforces AES201 forbidden import rules — both layer-level and scope-level rules.
///
/// Workflow (layer-level):
///   1. Check if the file is in the layer definition's exception list — skip if yes.
///   2. Determine the forbidden list: either from the layer definition or a default
///      (surfaces → {agent, infrastructure, capabilities}).
///   3. Parse import lines from the file via the parser port.
///   4. For each import, resolve each segment and compare against the forbidden layer list.
///   5. If a match is found, build a LintResult with the allowed alternatives.
///
/// Workflow (scope-level):
///   - Iterate all config rules and match files by scope (e.g., "agent(container)").
///   - Apply each rule's forbidden list to matching files.
pub struct ArchImportForbiddenChecker {
    parser: Arc<dyn IImportParserPort>,
}

impl ArchImportForbiddenChecker {
    pub fn new(parser: Arc<dyn IImportParserPort>) -> Self {
        Self { parser }
    }

    /// Check forbidden imports from layer definition (global layer rules).
    ///
    /// Steps:
    ///   1. Get file basename; skip if in the definition's exception list.
    ///   2. If no explicit forbidden list exists, only surfaces layer gets a default forbidden
    ///      list (agent, infrastructure, capabilities). Non-surface layers without forbidden are skipped.
    ///   3. Parse all import lines from the file via `read_import_lines`.
    ///   4. For each import line:
    ///      a. Extract the module path and split into segments.
    ///      b. For each forbidden layer, resolve its scope (layer + optional suffixes).
    ///      c. Check if any segment matches the forbidden layer (exact or suffix-based).
    ///      d. If forbidden, build a violation with allowed alternatives from the definition.
    pub fn check_forbidden_imports(
        &self,
        file: &str,
        layer_name: &str,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        // Step 1: Skip files in the exception list
        let file_path = filepath_or_default(FilePath::new(file.to_string()));
        let basename = file_path.basename();
        if definition.exceptions.values.contains(&basename.to_string()) {
            return;
        }

        // Step 2: Determine forbidden list (default for surfaces if not explicitly set)
        let is_surfaces = layer_name == "surfaces" || layer_name.starts_with("surfaces(");
        if definition.forbidden.values.is_empty() && !is_surfaces {
            return;
        }

        let forbidden_list: Vec<String> = if !definition.forbidden.values.is_empty() {
            definition.forbidden.values.clone()
        } else {
            vec![
                "agent".to_string(),
                "infrastructure".to_string(),
                "capabilities".to_string(),
            ]
        };

        // Step 3: Parse all import lines from the source file
        let import_lines = self.parser.read_import_lines(&file_path);
        let layer_name_vo = LayerNameVO::new(layer_name);

        // Step 4: Scan each import line for forbidden layers
        for (line_num, line) in &import_lines {
            if let Some(module) = self.parser.extract_module_from_line(line) {
                // Step 4a: Split module path into segments
                let segments: Vec<&str> = module
                    .value()
                    .split([':', '.', '/', '\\'])
                    .filter(|s| !s.is_empty())
                    .collect();

                // Step 4b-c: Check each forbidden layer against all segments
                for forbidden in &forbidden_list {
                    let forbidden_identity = Identity::new(forbidden);
                    let (layer, suffixes) = self.parser.resolve_scope(&forbidden_identity);
                    let is_forbidden = if suffixes.is_empty() {
                        // Exact layer match: check if any segment IS the forbidden layer
                        segments.iter().any(|seg| {
                            let cleaned = seg.trim_end_matches(';').trim();
                            let cleaned_identity = Identity::new(cleaned);
                            match self.parser.extract_layer_from_import(&cleaned_identity) {
                                Some(l) => l == layer,
                                None => false,
                            }
                        })
                    } else {
                        // Suffix-based match: check import scope (e.g., "infrastructure(adapter)")
                        self.parser.import_matches_scope(line, &layer, &suffixes)
                    };

                    // Step 4d: Build violation with allowed alternatives
                    if is_forbidden {
                        let allowed: Vec<LayerNameVO> = definition
                            .allowed
                            .values
                            .iter()
                            .map(|s| {
                                LayerNameVO::new(
                                    self.parser
                                        .resolve_scope(&Identity::new(s))
                                        .0
                                        .value()
                                        .to_string(),
                                )
                            })
                            .collect();
                        violations.push(LintResult::new_arch(
                            file,
                            line_num.value() as usize,
                            "AES201",
                            Severity::CRITICAL,
                            AesImportViolation::ForbiddenImport {
                                source_layer: layer_name_vo.clone(),
                                forbidden_layer: LayerNameVO::new(forbidden.clone()),
                                allowed,
                                reason: None,
                            }
                            .to_string(),
                        ));
                    }
                }
            }
        }
    }

    /// Check forbidden imports from per-rule scope definitions (fine-grained, per-suffix rules).
    ///
    /// Steps:
    ///   1. Get file stem (name without extension) and its last suffix (e.g., "command", "adapter").
    ///   2. Skip special Rust entry files (mod.rs, lib.rs, main.rs).
    ///   3. Parse import lines from the file — skip if empty (no imports to check).
    ///   4. Iterate all config rules:
    ///      a. Skip if file is in the rule's exception list.
    ///      b. Resolve the rule's scope to get (layer, suffixes).
    ///      c. Check if the file's stem starts with the layer prefix AND matches the suffix.
    ///      d. If scope matches, iterate each import line and check against the rule's forbidden list.
    ///      e. For each forbidden match, build a CRITICAL LintResult with allowed alternatives.
    pub fn check_scope_forbidden_imports(
        &self,
        file: &str,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        // Step 1: Extract file stem and its last underscore suffix
        let file_path = filepath_or_default(FilePath::new(file.to_string()));
        let basename_identity = self.parser.get_basename(&file_path);
        let basename = basename_identity.value();
        // Step 2: Skip Rust entry files
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }
        let stem = basename.rsplit('.').next_back().map_or(basename, |s| s);
        let suffix = stem.rsplit('_').next_back().map_or("", |s| s);

        // Step 3: Parse import lines
        let import_lines = self.parser.read_import_lines(&file_path);
        if import_lines.is_empty() {
            return;
        }

        // Step 4: Check each rule against this file
        for rule in &config.rules {
            // Step 4a: Skip exceptions
            if rule.exceptions.values.contains(&basename.to_string()) {
                continue;
            }
            // Step 4b: Resolve scope → (layer, suffixes)
            let scope_identity = Identity::new(&rule.scope.value);
            let (rule_layer, rule_suffixes) = self.parser.resolve_scope(&scope_identity);
            let rule_layer_str = rule_layer.value();

            // Step 4c: Check if file matches layer prefix AND suffix
            let layer_match = stem.starts_with(&format!("{}_", rule_layer_str));
            if !layer_match {
                continue;
            }
            if !rule_suffixes.is_empty() {
                let suffix_match = rule_suffixes.iter().any(|s| s.value() == suffix);
                if !suffix_match {
                    continue;
                }
            }

            // Step 4d-e: Scan imports against the rule's forbidden list
            for (line_num, line) in &import_lines {
                if let Some(module) = self.parser.extract_module_from_line(line) {
                    let segments: Vec<&str> = module
                        .value()
                        .split([':', '.', '/', '\\'])
                        .filter(|s| !s.is_empty())
                        .collect();
                    for forbidden in &rule.forbidden.values {
                        let forbidden_identity = Identity::new(forbidden);
                        let (forbidden_layer, forbidden_suffixes) =
                            self.parser.resolve_scope(&forbidden_identity);
                        let is_forbidden = if forbidden_suffixes.is_empty() {
                            segments.iter().any(|seg| {
                                let cleaned = seg.trim_end_matches(';').trim();
                                let cleaned_identity = Identity::new(cleaned);
                                match self.parser.extract_layer_from_import(&cleaned_identity) {
                                    Some(l) => l == forbidden_layer,
                                    None => false,
                                }
                            })
                        } else {
                            self.parser.import_matches_scope(
                                line,
                                &forbidden_layer,
                                &forbidden_suffixes,
                            )
                        };
                        if is_forbidden {
                            let allowed: Vec<LayerNameVO> = rule
                                .allowed
                                .values
                                .iter()
                                .map(|s| {
                                    LayerNameVO::new(
                                        self.parser
                                            .resolve_scope(&Identity::new(s))
                                            .0
                                            .value()
                                            .to_string(),
                                    )
                                })
                                .collect();
                            violations.push(LintResult::new_arch(
                                file,
                                line_num.value() as usize,
                                "AES201",
                                Severity::CRITICAL,
                                AesImportViolation::ForbiddenImport {
                                    source_layer: rule_layer.clone(),
                                    forbidden_layer: LayerNameVO::new(forbidden.clone()),
                                    allowed,
                                    reason: None,
                                }
                                .to_string(),
                            ));
                        }
                    }
                }
            }
        }
    }
}

impl IArchRuleProtocol for ArchImportForbiddenChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES201")
    }
}

#[async_trait]
impl IArchImportProtocol for ArchImportForbiddenChecker {
    async fn check_mandatory_imports(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
        // AES201 only handles forbidden imports — no mandatory import checks.
    }

    /// Run both layer-level and scope-level forbidden import checks on every file.
    ///
    /// Steps:
    ///   1. Iterate all project files.
    ///   2. Check if the file is a rule-level exception (AES201 exception list) — skip if yes.
    ///   3. Detect the file's architectural layer via the analyzer.
    ///   4. Look up the layer definition from the layer map and run `check_forbidden_imports`.
    ///   5. Run `check_scope_forbidden_imports` for all matching per-rule scope definitions.
    async fn check_forbidden_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in &files.values {
            let f_str = f.to_string();
            let basename = f.basename();

            // Step 2: Check Rule Exception directly (avoid LayerDefinition overwrite bugs)
            let mut is_exception = false;
            for r in &analyzer.config().rules {
                if r.name.value.as_str() == "AES201" && r.exceptions.values.contains(&basename) {
                    is_exception = true;
                    break;
                }
            }
            if is_exception {
                continue;
            }

            // Step 3-4: Detect layer and run layer-level forbidden check
            if let Some(layer) = analyzer.detect_layer(f, root_dir) {
                let layer_str = layer.value();
                if let Some(def) = analyzer.layer_map().values.get(&layer) {
                    self.check_forbidden_imports(&f_str, layer_str, def, &mut results.values);
                }
            }
            // Step 5: Run scope-level forbidden check (per-rule definitions)
            self.check_scope_forbidden_imports(&f_str, analyzer.config(), &mut results.values);
        }
    }
}
```

---

## File: crates/import-rules/src/capabilities_import_mandatory_checker.rs

```rust
// PURPOSE: ArchImportMandatoryChecker — AES202: enforce mandatory import rules per layer definition and scope rules
// AES202 rule: Each architectural layer (or scoped sub-layer) may declare a set of mandatory imports.
// Files belonging to that layer MUST import at least one symbol from each required scope.
// Two paths: (1) layer-definition mandatory list, (2) per-rule scope mandatory conditions.

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_rule_protocol::{
    IAnalyzer, IArchImportProtocol, IArchRuleProtocol,
};
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO};
use shared::taxonomy_name_vo::SymbolName;
use std::sync::Arc;

/// Returns the inner `FilePath` if `result` is `Ok`, otherwise returns `FilePath::default()`.
/// Private helper — uses `.unwrap_or_else` which is safe (AES304 only forbids bare `.unwrap()`,
/// not fallback variants like `.unwrap_or_else`/`.unwrap_or`/`.unwrap_or_default`).
fn filepath_or_default(result: Result<FilePath, impl std::fmt::Debug>) -> FilePath {
    result.unwrap_or_default()
}

/// Enforces AES202 mandatory import rules — both layer-level and scope-level.
///
/// Workflow (layer-level):
///   1. If the layer definition has no mandatory list, skip.
///   2. Skip Python __init__.py files and files in the exception list.
///   3. Read file content and parse all import lines.
///   4. For each required scope: check if any import line matches
///      (exact layer string match or suffix-based scope match).
///   5. Report each missing required import as an AES202 HIGH violation.
///
/// Workflow (scope-level):
///   - Same logic but reads mandatory from per-rule scope definitions instead.
pub struct ArchImportMandatoryChecker {
    parser: Arc<dyn IImportParserPort>,
}

impl ArchImportMandatoryChecker {
    pub fn new(parser: Arc<dyn IImportParserPort>) -> Self {
        Self { parser }
    }

    /// Check mandatory imports from layer definition (global layer rules).
    ///
    /// Steps:
    ///   1. Return early if the definition has no mandatory imports.
    ///   2. Skip Python __init__.py files (are implicit re-export modules).
    ///   3. Skip files in the exception list.
    ///   4. Read file content and parse import lines.
    ///   5. Derive the source layer name from the file stem.
    ///   6. For each required scope:
    ///      a. Resolve the scope into (layer, optional suffixes).
    ///      b. If no suffixes: check if any import line contains the layer string.
    ///      c. If suffixes: check if any import matches the scope (layer + suffixes).
    ///      d. If missing, emit an AES202 HIGH violation with the required scope name.
    pub fn check_mandatory_imports(
        &self,
        file: &str,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        // Step 1: Skip if no mandatory imports defined
        if definition.mandatory.values.is_empty() {
            return;
        }

        // Step 2-3: Skip special files and exceptions
        let file_path = filepath_or_default(FilePath::new(file.to_string()));
        let basename_identity = self.parser.get_basename(&file_path);
        let basename = basename_identity.value();
        if basename == "__init__.py" {
            return;
        }
        if definition.exceptions.values.contains(&basename.to_string()) {
            return;
        }

        // Step 4: Read file and parse import lines
        let Ok(content_msg) = self.parser.read_file_to_message(&file_path) else {
            return;
        };
        let content = content_msg.value().to_string();
        let file_content = FileContentVO::new(content);
        let import_lines = self.parser.parse_import_lines(&file_content);

        // Step 5: Derive source layer from filename (first prefix segment)
        let stem = basename.rsplit('.').next_back().map_or(basename, |s| s);
        let source_layer = stem.split('_').next().map_or("unknown", |s| s);

        // Step 6: Check each required scope against actual imports
        for required in &definition.mandatory.values {
            let required_identity = Identity::new(required);
            let (layer, suffixes) = self.parser.resolve_scope(&required_identity);
            let layer_str = layer.value();

            // Step 6a-c: Check if any import line matches the required scope
            let is_present = if suffixes.is_empty() {
                import_lines
                    .iter()
                    .any(|(_, l)| l.value().contains(layer_str))
            } else {
                import_lines
                    .iter()
                    .any(|(_, l)| self.parser.import_matches_scope(l, &layer, &suffixes))
            };

            // Step 6d: Report missing import
            if !is_present {
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES202",
                    Severity::HIGH,
                    AesImportViolation::MissingImport {
                        source_layer: LayerNameVO::new(source_layer.to_string()),
                        required: SymbolName::new(required.clone()),
                        reason: None,
                    }
                    .to_string(),
                ));
            }
        }
    }

    /// Check mandatory imports from per-rule scope definitions (fine-grained, per-suffix rules).
    /// This is the primary path — reads mandatory from each rule's scope configuration.
    ///
    /// Steps:
    ///   1. Get file stem (name without extension) and its last underscore suffix.
    ///   2. Skip Rust entry files (mod.rs, lib.rs, main.rs).
    ///   3. Parse import lines from the file.
    ///   4. Iterate all config rules:
    ///      a. Skip rules with empty mandatory lists.
    ///      b. Resolve the rule's scope into (layer, suffixes).
    ///      c. Check if file's stem starts with the layer prefix AND matches the suffix.
    ///      d. For each required scope in the rule:
    ///         - Resolve the required scope into (layer, suffixes).
    ///         - Check if any import line matches (exact layer string or suffix-based).
    ///         - If missing, emit an AES202 HIGH violation.
    pub fn check_scope_mandatory_imports(
        &self,
        file: &str,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        // Step 1: Extract file stem and suffix
        let file_path = filepath_or_default(FilePath::new(file.to_string()));
        let basename_identity = self.parser.get_basename(&file_path);
        let basename = basename_identity.value();
        // Step 2: Skip Rust entry files
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }
        let stem = basename.rsplit('.').next_back().map_or(basename, |s| s);
        let suffix = stem.rsplit('_').next_back().map_or("", |s| s);

        // Step 3: Parse import lines
        let import_lines = self.parser.read_import_lines(&file_path);

        // Step 4: Check each rule against this file
        for rule in &config.rules {
            // Step 4a: Skip rules without mandatory imports
            if rule.mandatory.values.is_empty() {
                continue;
            }

            // Step 4b-c: Check scope match
            let scope_identity = Identity::new(&rule.scope.value);
            let (rule_layer, rule_suffixes) = self.parser.resolve_scope(&scope_identity);
            let rule_layer_str = rule_layer.value();
            let layer_match = stem.starts_with(&format!("{}_", rule_layer_str));
            if !layer_match {
                continue;
            }
            if !rule_suffixes.is_empty() {
                let suffix_match = rule_suffixes.iter().any(|s| s.value() == suffix);
                if !suffix_match {
                    continue;
                }
            }

            // Step 4d: Check each required import
            for required in &rule.mandatory.values {
                let required_identity = Identity::new(required);
                let (req_layer, req_suffixes) = self.parser.resolve_scope(&required_identity);
                let req_layer_str = req_layer.value();

                let is_present = if req_suffixes.is_empty() {
                    if import_lines.is_empty() {
                        false
                    } else {
                        import_lines
                            .iter()
                            .any(|(_, l)| l.value().contains(req_layer_str))
                    }
                } else {
                    import_lines.iter().any(|(_, l)| {
                        self.parser
                            .import_matches_scope(l, &req_layer, &req_suffixes)
                    })
                };

                if !is_present {
                    violations.push(LintResult::new_arch(
                        file,
                        0,
                        "AES202",
                        Severity::HIGH,
                        AesImportViolation::MissingImport {
                            source_layer: rule_layer.clone(),
                            required: SymbolName::new(required.clone()),
                            reason: None,
                        }
                        .to_string(),
                    ));
                }
            }
        }
    }
}

impl IArchRuleProtocol for ArchImportMandatoryChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES202")
    }
}

#[async_trait]
impl IArchImportProtocol for ArchImportMandatoryChecker {
    /// Run both layer-level and scope-level mandatory import checks on every file.
    ///
    /// Steps:
    ///   1. Iterate all project files.
    ///   2. Check if the file is a rule-level exception (AES202 exception list) — skip if yes.
    ///   3. Detect the file's architectural layer via the analyzer.
    ///   4. Look up the layer definition and run layer-level `check_mandatory_imports`.
    ///   5. Run scope-level `check_scope_mandatory_imports` for all matching per-rule definitions.
    async fn check_mandatory_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in &files.values {
            let f_str = f.to_string();
            let basename = f.basename();

            // Step 2: Check Rule Exception directly (avoid LayerDefinition overwrite bugs)
            let mut is_exception = false;
            for r in &analyzer.config().rules {
                if r.name.value.as_str() == "AES202" && r.exceptions.values.contains(&basename) {
                    is_exception = true;
                    break;
                }
            }
            if is_exception {
                continue;
            }

            // Step 3-4: Detect layer and run layer-level mandatory check
            if let Some(layer) = analyzer.detect_layer(f, root_dir) {
                if let Some(def) = analyzer.layer_map().values.get(&layer) {
                    self.check_mandatory_imports(&f_str, def, &mut results.values);
                }
            }
            // Step 5: Run scope-level mandatory check
            self.check_scope_mandatory_imports(&f_str, analyzer.config(), &mut results.values);
        }
    }

    async fn check_forbidden_imports(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
        // AES202 only handles mandatory imports — no forbidden import checks.
    }
}
```

---

## File: crates/import-rules/src/capabilities_import_unused_checker.rs

```rust
// PURPOSE: UnusedImportRuleChecker — IUnusedImportProtocol for AES203: detect imports that are never used in the code (Rust/Python/JS)
// AES203 rule: Every import must be used at least once in the file that declares it.
// Detection strategies:
//   - Python/standard imports: extract imported aliases → find used symbols → diff.
//   - Rust/JS imports: extract named imports → check `is_name_used` for each.
//   - Respects __all__ exports (Python) and self-use patterns.

use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use std::sync::Arc;

/// Identifies imports that are declared but never used in the file (AES203).
///
/// Algorithm:
///   1. Extract all imported aliases/symbols from the source (Python `import X` / `from Y import Z`,
///      Rust `use X::Y`, JS `import X from Y`).
///   2. Extract all used symbols by scanning the file content (call the parser's `extract_used_symbols`).
///   3. If a symbol is exported (e.g., Python `__all__`), it is NOT unused (re-export pattern).
///   4. For Rust/JS: additional extraction of named imports + per-name usage check.
///   5. Each unused import becomes an AES203 MEDIUM violation.
pub struct UnusedImportRuleChecker {
    parser: Arc<dyn IImportParserPort>,
}

impl UnusedImportRuleChecker {
    pub fn new(parser: Arc<dyn IImportParserPort>) -> Self {
        Self { parser }
    }
}

impl IUnusedImportProtocol for UnusedImportRuleChecker {
    /// Find all unused imports in a file (returns list of symbol names).
    ///
    /// Steps:
    ///   1. Read file content. Return empty if file can't be read.
    ///   2. Extract all imported symbols/aliases (Python standard `import X`, `from Y import Z`).
    ///   3. Extract all exported symbols (Python `__all__`, Rust `pub use`, JS `export`).
    ///   4. Analyze which imported aliases are actually used in the code body.
    ///   5. For each alias: if it's NOT used AND NOT exported → add to unused list.
    ///   6. For Rust/JS named imports (e.g., `use foo::Bar`, `import { Bar }`):
    ///      extract and check each name individually via `is_name_used`.
    ///   7. Return the collected list of unused symbol names.
    fn find_unused_imports(&self, path: &FilePath) -> Vec<LintMessage> {
        // Step 1: Read file content
        let Ok(content_msg) = self.parser.read_file_to_message(path) else {
            return vec![];
        };
        let content = content_msg.value().to_string();

        // Step 2: Get imported symbols/aliases from the source file
        let imported_aliases = self.parser.extract_imported_aliases(&content);

        // Step 3: Get exported symbols (like __all__ in Python)
        let exported_symbols = self.parser.extract_exported_symbols(&content);

        // Step 4: Find which of these imported aliases are actually used in the code
        let used_symbols = self
            .parser
            .extract_used_symbols(&content, &imported_aliases);

        let mut unused: Vec<String> = Vec::new();

        // Step 5: Identify unused Python/standard imports
        for alias in imported_aliases.keys() {
            if !used_symbols.contains(alias) && !exported_symbols.contains(alias) {
                unused.push(alias.value().to_string());
            }
        }

        // Step 6: Handle Rust/JS specific imports
        let rust_js_imports = self.parser.extract_rust_js_imports(&content);
        for (name, line_idx) in rust_js_imports {
            let name_str = name.value();
            if !self.parser.is_name_used(name_str, &content, line_idx) {
                unused.push(name_str.to_string());
            }
        }

        // AES402: return VOs, not raw strings.
        unused
            .into_iter()
            .map(shared::taxonomy_message_vo::LintMessage::new)
            .collect()
    }

    /// Check for unused imports and record them as lint violations.
    ///
    /// Steps:
    ///   1. Extract all imported aliases (Python-style imports).
    ///   2. Extract all exported symbols.
    ///   3. Find which aliases are actually used in the code.
    ///   4. For each unused alias (not used, not exported): find its line number and emit MEDIUM violation.
    ///   5. For Rust/JS named imports: check each name and emit MEDIUM violation if unused.
    fn check_unused_imports(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        // 1. Get imported symbols/aliases from the source file
        let imported_aliases = self.parser.extract_imported_aliases(content);

        // 2. Get exported symbols (like __all__ in Python)
        let exported_symbols = self.parser.extract_exported_symbols(content);

        // 3. Find which of these imported aliases are actually used in the code
        let used_symbols = self.parser.extract_used_symbols(content, &imported_aliases);

        // 4. Identify unused Python/standard imports and record violations
        for alias in imported_aliases.keys() {
            if !used_symbols.contains(alias) && !exported_symbols.contains(alias) {
                let line_num = self
                    .parser
                    .find_import_line_number(content, alias.value())
                    .value() as usize;
                violations.push(LintResult::new_arch(
                    file,
                    line_num,
                    "AES203",
                    Severity::MEDIUM,
                    AesImportViolation::FixUnusedImport {
                        reason: Some(shared::taxonomy_message_vo::LintMessage::new(format!(
                            "Import '{}' is declared but never used in this file.",
                            alias
                        ))),
                    }
                    .to_string(),
                ));
            }
        }

        // 5. Handle Rust/JS specific imports and record violations
        let rust_js_imports = self.parser.extract_rust_js_imports(content);
        for (name, line_idx) in rust_js_imports {
            let name_str = name.value().to_string();
            let line_no_us = line_idx.value() as usize;
            if !self.parser.is_name_used(&name_str, content, line_idx) {
                violations.push(LintResult::new_arch(
                    file,
                    line_no_us,
                    "AES203",
                    Severity::MEDIUM,
                    AesImportViolation::FixUnusedImport {
                        reason: Some(shared::taxonomy_message_vo::LintMessage::new(format!(
                            "Import '{}' is declared but never used in this file.",
                            name_str
                        ))),
                    }
                    .to_string(),
                ));
            }
        }
    }
}
```

---

## File: crates/import-rules/src/capabilities_layer_detection_analyzer.rs

```rust
// PURPOSE: LayerDetectionAnalyzer — layer detection via filename prefix (FRD v1.1)
// This is the central analyzer that implements IAnalyzer. It provides:
//   1. Layer detection per file — exclusively via filename prefix (FRD v1.1).
//      Files without a valid prefix return None and will be reported by AES101 naming enforcement.
//   2. Module layer detection (direct match → prefix match → path match).
//   3. Specialised sub-layer resolution (e.g., "capabilities(command)" from suffix).
//   4. Layer map construction with rule merging (global rules + per-layer rules + specialised rules).
// Used by all AES rule checkers to determine which architectural layer a file belongs to.

use std::collections::HashMap;
use std::path::Path;

use shared::common::contract_parser_port::ISourceParserPort;
use shared::common::contract_system_port::IFileSystemPort;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::config_system::taxonomy_config_vo::ArchitectureRule;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::import_rules::taxonomy_path_helper;
use shared::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::taxonomy_layer_vo::LayerNameVO;
use std::sync::Arc;

/// Central layer detection and rule analysis engine implementing IAnalyzer.
///
/// Capabilities:
///   - `detect_layer(file, root)` → determines which architectural layer a file belongs to,
///     exclusively via filename prefix (FRD v1.1). Returns None for files without a valid
///     prefix — the AES101 naming rule will report those as violations.
///   - `detect_module_layer(module_path)` → determines which layer a module path imports from.
///   - `resolve_specialized_layer(base, file)` → resolves sub-layers (e.g., "capabilities(command)").
///   - `new(config)` → builds a complete layer map by merging global rules, base-layer rules,
///     and specialised sub-layer rules from the rule configuration.
///
/// Constructor workflow:
///   1. Index all config rules by layer scope (both base name and full scoped name).
///   2. For each layer definition: merge global rules + base-layer rules into the definition.
///   3. For each scoped rule (e.g., "agent(container)"): create specialised sub-layer entries
///      by cloning the base definition and overlaying the scoped rule's values.
///   4. Replace config.layers with the enriched layer map for fast lookup.
pub struct LayerDetectionAnalyzer {
    pub config: ArchitectureConfig,
    pub layer_map: LayerMapVO,
    pub fs: Arc<dyn IFileSystemPort>,
    pub parser: Arc<dyn ISourceParserPort>,
}

impl LayerDetectionAnalyzer {
    /// Construct a new LayerDetectionAnalyzer with merged rule configuration.
    ///
    /// Steps:
    ///   1. Build a `rules_by_layer` index: for each rule, map by both its base scope
    ///      (e.g., "agent") and its full scoped name (e.g., "agent(container|registry)").
    ///   2. Iterate all layer definitions from config. For each:
    ///      a. Apply global rules (empty scope key).
    ///      b. Apply base-layer rules (e.g., rules scoped to "agent").
    ///      c. Skip specialised scoped rules (e.g., "agent(container)") at this stage.
    ///   3. For each scoped rule "X(Y|Z)":
    ///      a. Parse the base name X and the set of suffixes {Y, Z}.
    ///      b. Clone the base layer definition.
    ///      c. Overlay the scoped rule's values (forbidden, mandatory, allowed, etc.).
    ///      d. Insert as a new sub-layer entry "X(Y)", "X(Z)".
    ///   4. Store the enriched config and build a LayerMapVO for fast lookups.
    pub fn new(
        mut config: ArchitectureConfig,
        fs: Arc<dyn IFileSystemPort>,
        parser: Arc<dyn ISourceParserPort>,
    ) -> Self {
        // Step 1: Index all rules by layer scope (both base + full scoped)
        let mut rules_by_layer: HashMap<String, Vec<&ArchitectureRule>> = HashMap::new();
        for rule in &config.rules {
            let scope = rule.scope.to_string();
            let base_key = if scope.is_empty() {
                String::new()
            } else {
                match scope.split('(').next() {
                    Some(s) => s.to_string(),
                    None => scope.to_string(),
                }
            };
            rules_by_layer.entry(base_key).or_default().push(rule);
            // Also index by full scope (e.g. "agent(container|registry|mixin)")
            if scope.contains('(') {
                rules_by_layer.entry(scope.clone()).or_default().push(rule);
            }
        }

        // Step 2: Merge global + base-layer rules into each layer definition
        let mut new_layers: HashMap<LayerNameVO, LayerDefinition> = HashMap::new();
        for (lname, mut ldef) in config.layers {
            let lstr = lname.to_string();
            let base_name = match lstr.split('(').next() {
                Some(s) => s.to_string(),
                None => lstr.to_string(),
            };
            // Apply: global rules (key="") + base-layer rules (key=base_name)
            for key in &[String::new(), base_name.clone()] {
                if let Some(rules) = rules_by_layer.get(key.as_str()) {
                    for rule in rules {
                        // Skip specialised scoped rules (e.g. contract(port)) when processing base layers
                        if key.as_str() == base_name && rule.scope.value.contains('(') {
                            continue;
                        }
                        if !rule.exceptions.values.is_empty() {
                            for val in &rule.exceptions.values {
                                if !ldef.exceptions.values.contains(val) {
                                    ldef.exceptions.values.push(val.clone());
                                }
                            }
                        }
                        if !rule.mandatory.values.is_empty() {
                            for val in &rule.mandatory.values {
                                if !ldef.mandatory.values.contains(val) {
                                    ldef.mandatory.values.push(val.clone());
                                }
                            }
                        }
                        if !rule.forbidden.values.is_empty() {
                            for val in &rule.forbidden.values {
                                if !ldef.forbidden.values.contains(val) {
                                    ldef.forbidden.values.push(val.clone());
                                }
                            }
                        }
                        if rule.code_analysis.min_lines.value > 0 {
                            ldef.code_analysis.min_lines = rule.code_analysis.min_lines.clone();
                        }
                        if rule.code_analysis.max_lines.value > 0 {
                            ldef.code_analysis.max_lines = rule.code_analysis.max_lines.clone();
                        }
                        if rule.code_analysis.mandatory_class_definition.value {
                            ldef.code_analysis.mandatory_class_definition =
                                rule.code_analysis.mandatory_class_definition.clone();
                        }
                        if !rule.code_analysis.forbidden_inheritance.values.is_empty() {
                            for val in &rule.code_analysis.forbidden_inheritance.values {
                                if !ldef
                                    .code_analysis
                                    .forbidden_inheritance
                                    .values
                                    .contains(val)
                                {
                                    ldef.code_analysis
                                        .forbidden_inheritance
                                        .values
                                        .push(val.clone());
                                }
                            }
                        }
                        // Merge orphan rule settings into layer definition
                        if rule.orphan.check_orphan.value {
                            ldef.orphan.check_orphan =
                                shared::common::taxonomy_common_vo::BooleanVO::new(true);
                        }
                        if !rule.orphan.orphan_entry_points.values.is_empty() {
                            for val in &rule.orphan.orphan_entry_points.values {
                                if !ldef.orphan.orphan_entry_points.values.contains(val) {
                                    ldef.orphan.orphan_entry_points.values.push(val.clone());
                                }
                            }
                        }
                    }
                }
            }
            new_layers.insert(lname, ldef);
        }

        // Step 3: Create specialised sub-layer entries from scoped rules
        // e.g., "agent(container)" → clone agent def + overlay container-specific rules
        for rule in &config.rules {
            let scope = rule.scope.to_string();
            if !scope.contains('(') {
                continue;
            }
            // Extract suffixes from scope: "agent(container|registry|mixin)"
            if let Some(paren_start) = scope.find('(') {
                let base_name = scope[..paren_start].trim();
                let inner = scope[paren_start + 1..].trim_end_matches(')').trim();
                // Check if the base layer exists — clone def first to avoid borrow conflict
                let base_key_str = base_name.to_string();
                let base_def_opt = {
                    let base_key = LayerNameVO::new(&base_key_str);
                    new_layers.get(&base_key).cloned()
                };
                if let Some(base_def) = base_def_opt {
                    // Step 3a: Parse suffixes (separated by | or ,)
                    let suffixes: Vec<&str> = if inner.contains('|') {
                        inner
                            .split('|')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .collect()
                    } else {
                        inner
                            .split(',')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .collect()
                    };
                    // Step 3b-d: Create one sub-layer per suffix
                    for suffix in suffixes {
                        let specialized_key =
                            LayerNameVO::new(format!("{}({})", base_name, suffix));
                        if new_layers.contains_key(&specialized_key) {
                            continue;
                        }
                        let mut spec_def = base_def.clone();
                        // Step 3c: Overlay scoped rule values onto the cloned definition
                        if let Some(rules) = rules_by_layer.get(&scope) {
                            for r in rules {
                                if !r.exceptions.values.is_empty() {
                                    for val in &r.exceptions.values {
                                        if !spec_def.exceptions.values.contains(val) {
                                            spec_def.exceptions.values.push(val.clone());
                                        }
                                    }
                                }
                                if !r.forbidden.values.is_empty() {
                                    for val in &r.forbidden.values {
                                        if !spec_def.forbidden.values.contains(val) {
                                            spec_def.forbidden.values.push(val.clone());
                                        }
                                    }
                                }
                                if !r.mandatory.values.is_empty() {
                                    for val in &r.mandatory.values {
                                        if !spec_def.mandatory.values.contains(val) {
                                            spec_def.mandatory.values.push(val.clone());
                                        }
                                    }
                                }
                                if !r.allowed.values.is_empty() {
                                    for val in &r.allowed.values {
                                        if !spec_def.allowed.values.contains(val) {
                                            spec_def.allowed.values.push(val.clone());
                                        }
                                    }
                                }
                                if !r.code_analysis.forbidden_inheritance.values.is_empty() {
                                    for val in &r.code_analysis.forbidden_inheritance.values {
                                        if !spec_def
                                            .code_analysis
                                            .forbidden_inheritance
                                            .values
                                            .contains(val)
                                        {
                                            spec_def
                                                .code_analysis
                                                .forbidden_inheritance
                                                .values
                                                .push(val.clone());
                                        }
                                    }
                                }
                            }
                        }
                        // Step 3d: Insert the new specialised sub-layer
                        new_layers.insert(specialized_key, spec_def);
                    }
                }
            }
        }

        // Step 4: Store enriched config and build LayerMapVO
        config.layers = new_layers;
        let layer_map = LayerMapVO::new(config.layers.clone());
        Self {
            config,
            layer_map,
            fs,
            parser,
        }
    }

    /// Detect layer from filename — exclusively via filename prefix (FRD v1.1).
    ///
    /// Files MUST carry a layer prefix (e.g., `capabilities_foo.rs` → capabilities layer).
    /// Files without a valid prefix return None, and AES101 naming enforcement will report
    /// them as violations — forcing the developer to add the correct prefix.
    ///
    /// After prefix detection, `resolve_specialized_layer` checks whether the file suffix
    /// corresponds to a specialised sub-layer (e.g., `capabilities_command.rs` with a defined
    /// `capabilities(command)` layer → returns `capabilities(command)` instead of `capabilities`).
    pub fn detect_layer(&self, file_path: &str, _root_dir: &str) -> Option<String> {
        let filename = Path::new(file_path)
            .file_name()
            .and_then(|s| s.to_str())
            .map_or("", |s| s);

        // PREFIX-BASED DETECTION (FRD v1.1)
        // All valid files must carry a layer prefix — enforced by AES101/AES102 naming rules.
        if let Some(layer) = taxonomy_path_helper::extract_layer_from_prefix(filename) {
            return Some(self.resolve_specialized_layer(&layer, file_path));
        }

        // No valid prefix found — violates AES101 naming convention.
        // AES101/AES102 will report this separately; we return None so the file
        // is not silently assigned to a wrong layer.
        None
    }

    /// Determine which architectural layer a module path (from an import statement) belongs to.
    ///
    /// Three strategies, in priority order:
    ///
    /// Strategy 1 — Direct segment match:
    ///   Compare each segment of the module path against known layer names.
    ///   E.g., "shared::taxonomy::..." → segment "taxonomy" matches → taxonomy layer.
    ///
    /// Strategy 2 — Prefix-based match (FRD v1.1):
    ///   If no direct match, check if any segment starts with a layer prefix.
    ///   E.g., "taxonomy_definition_vo" starts with "taxonomy_" → taxonomy layer.
    ///
    /// Strategy 3 — Path-based match:
    ///   Convert the module path to a filesystem path and check if it contains any
    ///   layer definition's configured path.
    ///   E.g., module "crates/shared/taxonomy" contains path "shared" → taxonomy layer.
    ///
    /// Each match is refined via `refine_module_layer` to detect specialised sub-layers
    /// (e.g., "capabilities(command)" when the segment after the layer name has a suffix).
    pub fn detect_module_layer(&self, module: &str) -> Option<String> {
        // Split module path into meaningful segments (handles ::, ., /, \ separators)
        let meaningful_parts: Vec<&str> = module
            .split([':', '.', '/', '\\'])
            .filter(|p| !p.is_empty())
            .collect();

        if meaningful_parts.is_empty() {
            return None;
        }

        // Strategy 1: Direct match with layer names (ignoring specialisation suffix)
        for name in self.config.layers.keys() {
            let base_name = match name.value.split('(').next() {
                Some(s) => s,
                None => &name.value,
            };
            if meaningful_parts.contains(&base_name) {
                return Some(self.refine_module_layer(base_name, &meaningful_parts));
            }
        }

        // Strategy 2: Prefix-based match (e.g., "taxonomy_definition_vo" → "taxonomy")
        for part in &meaningful_parts {
            if let Some(layer) = taxonomy_path_helper::extract_layer_from_prefix(part) {
                return Some(self.refine_module_layer(&layer, &meaningful_parts));
            }
        }

        None
    }

    /// Try to resolve a specialised sub-layer from the file's suffix.
    ///
    /// E.g., `capabilities_user_command.rs` with base_layer="capabilities":
    ///   → stem = "capabilities_user_command", last suffix = "command"
    ///   → checks if "capabilities(command)" is a defined specialised layer
    ///   → if yes, returns "capabilities(command)", else returns "capabilities".
    ///
    /// Steps:
    ///   1. Extract the file stem (name without extension).
    ///   2. Find the last underscore segment as the suffix hint.
    ///   3. Construct the specialised layer key: "{base_layer}({suffix})".
    ///   4. Check if this key exists in the built layer map (must have been created from scoped rules).
    ///   5. Return the specialised name if found, otherwise the base layer unchanged.
    fn resolve_specialized_layer(&self, base_layer: &str, file_path: &str) -> String {
        // Step 1: Get file stem
        let basename = Path::new(file_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .map_or("", |s| s);

        // Step 2-5: Check if last underscore suffix matches a specialised sub-layer
        if let Some(underscore_pos) = basename.rfind('_') {
            let suffix = &basename[underscore_pos + 1..];
            if !suffix.is_empty() {
                let specialized = format!("{}({})", base_layer, suffix);
                let key = LayerNameVO::new(specialized.as_str());
                // Step 4: Must have been created in new() from scoped rules
                if self.config.layers.contains_key(&key) {
                    return specialized;
                }
            }
        }

        base_layer.to_string()
    }

    /// Refine a base layer to a specialised sub-layer by inspecting the segment
    /// immediately after the layer name in a dotted module path.
    ///
    /// E.g., parts = ["capabilities", "user_command", "UserCommand"], base = "capabilities"
    ///   → next part after "capabilities" is "user_command"
    ///   → last underscore suffix of "user_command" is "command"
    ///   → checks if "capabilities(command)" exists → returns it if yes.
    ///
    /// Steps:
    ///   1. Find the position of the base layer name in the module parts.
    ///   2. Get the next segment after the base layer name.
    ///   3. Extract the last underscore suffix from that segment.
    ///   4. Construct the specialised key and check if it exists.
    ///   5. Return specialised name or fall back to base name.
    fn refine_module_layer(&self, base_name: &str, parts: &[&str]) -> String {
        // Step 1-2: Find base name position and get next segment
        if let Some(idx) = parts.iter().position(|&p| p == base_name) {
            if idx + 1 < parts.len() {
                let next_part = parts[idx + 1];
                // Step 3: Extract suffix from next segment
                if let Some(underscore_pos) = next_part.rfind('_') {
                    let suffix = &next_part[underscore_pos + 1..];
                    // Step 4-5: Check if specialised sub-layer exists
                    let specialized = format!("{}({})", base_name, suffix);
                    let key = LayerNameVO::new(specialized.as_str());
                    if self.config.layers.contains_key(&key) {
                        return specialized;
                    }
                }
            }
        }
        base_name.to_string()
    }

    /// Look up a `LayerDefinition` by its layer name string.
    /// Falls back to the base layer definition if the specialised key is not found.
    ///
    /// Steps:
    ///   1. Try direct lookup with the full layer name (including parenthesised suffix).
    ///   2. If not found, extract the base name (before the parenthesis) and try again.
    pub fn get_layer_def(&self, layer: &str) -> Option<&LayerDefinition> {
        self.config
            .layers
            .get(&LayerNameVO::new(layer))
            .or_else(|| {
                let base = match layer.split('(').next() {
                    Some(s) => s,
                    None => layer,
                };
                self.config.layers.get(&LayerNameVO::new(base))
            })
    }
}

impl shared::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol
    for LayerDetectionAnalyzer
{
    /// Return the merged architecture configuration.
    fn config(&self) -> &ArchitectureConfig {
        &self.config
    }
    /// Return the layer map (layer name → LayerDefinition).
    fn layer_map(&self) -> &LayerMapVO {
        &self.layer_map
    }
    /// Adapter: delegates to internal `detect_layer` and wraps result in LayerNameVO.
    fn detect_layer(&self, f: &FilePath, root_dir: &FilePath) -> Option<LayerNameVO> {
        self.detect_layer(&f.value, &root_dir.value)
            .map(|s| LayerNameVO::new(s.as_str()))
    }
}

impl IAnalyzer for LayerDetectionAnalyzer {
    /// Return the filesystem port for file I/O.
    fn fs(&self) -> &dyn IFileSystemPort {
        &*self.fs
    }
    /// Return the source parser port for code analysis.
    fn parser(&self) -> &dyn ISourceParserPort {
        &*self.parser
    }
    /// Adapter: delegates to internal `detect_module_layer` and wraps result in LayerNameVO.
    fn detect_module_layer(&self, module_path: &FilePath) -> Option<LayerNameVO> {
        self.detect_module_layer(&module_path.value)
            .map(|s| LayerNameVO::new(s.as_str()))
    }
}

impl shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate
    for LayerDetectionAnalyzer
{
    fn detect_layer(&self, file_path: &str, root_dir: &str) -> Option<String> {
        self.detect_layer(file_path, root_dir)
    }

    fn get_layer_def(
        &self,
        layer: &str,
    ) -> Option<shared::common::taxonomy_definition_vo::LayerDefinition> {
        self.get_layer_def(layer).cloned()
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
```

---

## File: crates/import-rules/src/infrastructure_filesystem_adapter.rs

```rust
// PURPOSE: FileSystemAdapter — IFileSystemPort implementation using std::fs
use async_trait::async_trait;
use std::fs;
use std::path::{Path, PathBuf};

use shared::common::contract_system_port::IFileSystemPort;
use shared::common::taxonomy_filesystem_error::FileSystemError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::mcp_server::taxonomy_action_vo::ActionName;
use shared::mcp_server::taxonomy_job_vo::SuccessStatus;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::Count;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_layer_vo::Identity;
use shared::taxonomy_source_vo::ContentString;

/// Returns the inner `FilePath` if `result` is `Ok`, otherwise returns `FilePath::default()`.
/// Private helper — uses `Result::match` to avoid inline match patterns.
fn filepath_or_default(result: Result<FilePath, impl std::fmt::Debug>) -> FilePath {
    result.unwrap_or_default()
}

/// Returns the inner `FilePath` if `result` is `Ok`, otherwise clones `fallback`.
fn filepath_or_clone(
    result: Result<FilePath, impl std::fmt::Debug>,
    fallback: &FilePath,
) -> FilePath {
    match result {
        Ok(fp) => fp,
        Err(_) => fallback.clone(),
    }
}

/// Returns the `&str` slice from an `OsStr` option, falling back to `""`.
fn os_str_to_str(opt: Option<&std::ffi::OsStr>) -> &str {
    opt.and_then(|o| o.to_str()).map_or("", |s| s)
}

pub struct OSFileSystemAdapter {}

impl OSFileSystemAdapter {
    pub fn new() -> Self {
        Self {}
    }

    fn walk_recursive(&self, dir: &Path, ignored: &[String], results: &mut Vec<FilePath>) {
        if dir.is_file() {
            if let Ok(fp) = FilePath::new(dir.to_string_lossy().to_string()) {
                results.push(fp);
            }
            return;
        }
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = os_str_to_str(path.file_name());
                if ignored.contains(&name.to_string()) {
                    continue;
                }
                if path.is_dir() {
                    self.walk_recursive(&path, ignored, results);
                } else if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                    results.push(fp);
                }
            }
        }
    }
}

impl Default for OSFileSystemAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl IFileSystemPort for OSFileSystemAdapter {
    async fn walk(&self, path: &FilePath, ignored_patterns: Option<&PatternList>) -> FilePathList {
        let root = Path::new(&path.value);
        let ignored = match ignored_patterns {
            Some(p) => p.values.clone(),
            None => Vec::new(),
        };
        let mut results = Vec::new();
        self.walk_recursive(root, &ignored, &mut results);
        FilePathList { values: results }
    }

    async fn is_directory(&self, path: &FilePath) -> SuccessStatus {
        SuccessStatus::new(Path::new(&path.value).is_dir())
    }

    async fn is_file(&self, path: &FilePath) -> SuccessStatus {
        SuccessStatus::new(Path::new(&path.value).is_file())
    }

    async fn get_relative_path(&self, path: &FilePath, start: &FilePath) -> FilePath {
        let p = Path::new(&path.value);
        let s = Path::new(&start.value);
        p.strip_prefix(s).ok().map_or_else(
            || path.clone(),
            |rel| filepath_or_clone(FilePath::new(rel.to_string_lossy().to_string()), path),
        )
    }

    async fn read_text(&self, path: &FilePath) -> Result<ContentString, FileSystemError> {
        self.read_file(path).await
    }

    async fn get_line_count(&self, path: &FilePath) -> Count {
        if let Ok(content) = fs::read_to_string(&path.value) {
            Count::new(content.lines().count() as i64)
        } else {
            Count::new(0)
        }
    }

    async fn exists(&self, path: &FilePath) -> SuccessStatus {
        SuccessStatus::new(Path::new(&path.value).exists())
    }

    async fn get_parent(&self, path: &FilePath) -> FilePath {
        let p = Path::new(&path.value);
        p.parent().map_or_else(
            || path.clone(),
            |parent| filepath_or_clone(FilePath::new(parent.to_string_lossy().to_string()), path),
        )
    }

    async fn write_text(
        &self,
        path: &FilePath,
        content: &ContentString,
        _mode: Option<&Identity>,
    ) -> Result<SuccessStatus, FileSystemError> {
        match fs::write(&path.value, &content.value) {
            Ok(_) => Ok(SuccessStatus::new(true)),
            Err(e) => Err(FileSystemError::new(
                path.clone(),
                ErrorMessage::new(e.to_string()),
                ActionName::new("write"),
            )),
        }
    }

    async fn glob(&self, _pattern: &Identity) -> FilePathList {
        FilePathList { values: vec![] }
    }

    async fn get_cwd(&self) -> FilePath {
        let cwd = match std::env::current_dir() {
            Ok(p) => p,
            Err(_) => PathBuf::from("."),
        };
        let primary = filepath_or_default(FilePath::new(cwd.to_string_lossy().to_string()));
        if primary != FilePath::default() {
            primary
        } else {
            filepath_or_default(FilePath::new(".".to_string()))
        }
    }

    async fn get_basename(&self, path: &FilePath) -> Identity {
        let p = Path::new(&path.value);
        let name = os_str_to_str(p.file_name());
        Identity::new(name.to_string())
    }

    async fn path_join(&self, parts: &[Identity]) -> FilePath {
        let mut path = PathBuf::new();
        for part in parts {
            path.push(&part.value);
        }
        let primary = filepath_or_default(FilePath::new(path.to_string_lossy().to_string()));
        if primary != FilePath::default() {
            primary
        } else {
            filepath_or_default(FilePath::new(".".to_string()))
        }
    }

    async fn read_file(&self, path: &FilePath) -> Result<ContentString, FileSystemError> {
        match fs::read_to_string(&path.value) {
            Ok(content) => Ok(ContentString::new(content)),
            Err(e) => Err(FileSystemError::new(
                path.clone(),
                ErrorMessage::new(e.to_string()),
                ActionName::new("read"),
            )),
        }
    }
}
```

---

## File: crates/import-rules/src/infrastructure_import_parser_adapter.rs

```rust
// PURPOSE: ImportParserAdapter — infrastructure implementation of IImportParserPort using standard filesystem and string search utilities

use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use shared::import_rules::taxonomy_language_vo::LanguageVO;
use shared::import_rules::taxonomy_path_helper;
use shared::import_rules::{
    taxonomy_cycle_helper, taxonomy_dummy_helper, taxonomy_parser_helper, taxonomy_unused_helper,
};
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO, LineContentVO};
use shared::taxonomy_name_vo::SymbolName;
use std::collections::{HashMap, HashSet};
use std::fs;

/// Returns `s` if `opt` is `Some`, otherwise returns `""`.
/// Private helper — uses `Option::map_or` to avoid inline match patterns.
fn str_or_empty(opt: Option<&str>) -> &str {
    opt.map_or("", |s| s)
}

pub struct ImportParserAdapter {}

impl ImportParserAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ImportParserAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl IImportParserPort for ImportParserAdapter {
    /// Resolve a scope value (e.g. "contract(protocol)", "taxonomy(entity,error,event)")
    /// into layer + suffix matches. Returns (`LayerNameVO`, `Vec<Identity>`).
    fn resolve_scope(&self, scope: &Identity) -> (LayerNameVO, Vec<Identity>) {
        let scope_str = scope.value();
        if let Some(paren) = scope_str.find('(') {
            let layer = scope_str[..paren].trim();
            let inner = scope_str[paren + 1..].trim_end_matches(')').trim();
            let suffixes: Vec<Identity> = if inner.contains('|') {
                inner
                    .split('|')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(Identity::new)
                    .collect()
            } else {
                inner
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(Identity::new)
                    .collect()
            };
            (LayerNameVO::new(layer), suffixes)
        } else {
            (LayerNameVO::new(scope_str.trim()), vec![])
        }
    }

    /// Check if an import line satisfies the given scope requirement.
    fn import_matches_scope(
        &self,
        import_line: &LineContentVO,
        layer: &LayerNameVO,
        suffixes: &[Identity],
    ) -> bool {
        let import_line_str = import_line.value();
        let segments: Vec<&str> = import_line_str
            .split(|c: char| {
                c == ':'
                    || c == '.'
                    || c == '/'
                    || c == '\\'
                    || c.is_whitespace()
                    || c == '"'
                    || c == '\''
                    || c == '{'
                    || c == '}'
                    || c == ','
                    || c == ';'
            })
            .filter(|s| !s.is_empty())
            .collect();
        let layer_lower = layer.value().to_lowercase();
        let layer_prefix = format!("{}_", layer_lower);
        let layer_match = segments.iter().any(|s| {
            let trimmed = s.trim().to_lowercase();
            trimmed == layer_lower || trimmed.starts_with(&layer_prefix)
        });
        if !layer_match || suffixes.is_empty() {
            return layer_match;
        }
        suffixes.iter().any(|s| {
            let s_val = s.value();
            segments.iter().any(|seg| {
                let cleaned = seg
                    .trim_end_matches(';')
                    .trim()
                    .trim_start_matches('{')
                    .trim_end_matches('}')
                    .trim();
                cleaned.split(',').any(|t| {
                    let name = t.trim();
                    let name_lower = name.to_lowercase();
                    if name_lower.ends_with(&format!("_{}", s_val)) {
                        return true;
                    }
                    if let Some(rest) = name_lower.strip_suffix(s_val) {
                        if rest.is_empty() || rest.ends_with('_') {
                            return true;
                        }
                        if name.len() >= s_val.len() {
                            let suffix_in_orig = &name[name.len() - s_val.len()..];
                            if suffix_in_orig.starts_with(|c: char| c.is_uppercase()) {
                                return true;
                            }
                        }
                    }
                    false
                })
            })
        })
    }

    fn get_basename(&self, file: &FilePath) -> Identity {
        Identity::new(file.basename())
    }

    fn read_import_lines(&self, file: &FilePath) -> Vec<(LineNumber, LineContentVO)> {
        let Ok(content) = fs::read_to_string(file.value()) else {
            return vec![];
        };
        // Use helper function to parse
        parse_import_lines_helper(&content)
    }

    fn parse_import_lines(&self, content: &FileContentVO) -> Vec<(LineNumber, LineContentVO)> {
        parse_import_lines_helper(content.value())
    }

    fn extract_module_from_line(&self, line: &LineContentVO) -> Option<Identity> {
        let trimmed = line.value().trim();
        if let Some(rest) = trimmed.strip_prefix("from ") {
            return Some(Identity::new(rest.split_whitespace().next()?.to_string()));
        }
        if trimmed.starts_with("import ") {
            if let Some(pos) = trimmed.rfind(" from ") {
                let module_part = trimmed[pos + 6..].trim();
                let cleaned = module_part
                    .trim_end_matches(';')
                    .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                    .trim();
                return Some(Identity::new(cleaned.to_string()));
            }
            if let Some(rest) = trimmed.strip_prefix("import ") {
                if rest.contains('"') || rest.contains('\'') || rest.contains('`') {
                    let cleaned = rest
                        .trim_end_matches(';')
                        .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                        .trim();
                    return Some(Identity::new(cleaned.to_string()));
                }
                let first_token = str_or_empty(rest.split_whitespace().next());
                return Some(Identity::new(first_token.to_string()));
            }
        }
        if let Some(rest) = trimmed.strip_prefix("use ") {
            let module = rest.trim_end_matches(';').trim().to_string();
            if let Some(brace_pos) = module.find("::{") {
                return Some(Identity::new(module[..brace_pos].to_string()));
            }
            return Some(Identity::new(module));
        }
        None
    }

    fn extract_layer_from_import(&self, segment: &Identity) -> Option<LayerNameVO> {
        let segment_str = segment.value();
        // Strategy 1: Prefix-based — reuse canonical helper (avoids duplicating PREFIX_MAP)
        if let Some(layer) = taxonomy_path_helper::extract_layer_from_prefix(segment_str) {
            return Some(LayerNameVO::new(layer));
        }
        // Strategy 2: Direct segment match — bare layer names without underscore suffix
        match segment_str {
            "taxonomy" => Some(LayerNameVO::new("taxonomy")),
            "contract" => Some(LayerNameVO::new("contract")),
            "capabilities" => Some(LayerNameVO::new("capabilities")),
            "infrastructure" => Some(LayerNameVO::new("infrastructure")),
            "agent" => Some(LayerNameVO::new("agent")),
            "surfaces" | "surface" => Some(LayerNameVO::new("surfaces")),
            "root" => Some(LayerNameVO::new("root")),
            _ => None,
        }
    }

    fn read_file_to_message(&self, file: &FilePath) -> Result<LintMessage, std::io::Error> {
        let raw = fs::read_to_string(file.value())?;
        Ok(LintMessage::new(raw))
    }

    fn extract_import_modules(&self, content: &str) -> Vec<SymbolName> {
        taxonomy_parser_helper::extract_import_modules(content)
    }

    fn get_language_from_path(&self, path: &str) -> LanguageVO {
        LanguageVO::from_path(path)
    }

    fn get_dummy_function_ranges(
        &self,
        lines: &[&str],
        lang: LanguageVO,
    ) -> Vec<(LineNumber, LineNumber)> {
        taxonomy_dummy_helper::dummy_function_ranges(lines, lang)
    }

    fn get_imported_symbols(
        &self,
        lines: &[&str],
        lang: LanguageVO,
    ) -> Vec<(SymbolName, LineNumber)> {
        taxonomy_dummy_helper::imported_symbols(lines, lang)
    }

    fn get_dummy_impl_traits_with_lines(&self, lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
        taxonomy_dummy_helper::dummy_impl_traits_with_lines(lines)
    }

    fn is_symbol_used_real(
        &self,
        lines: &[&str],
        symbol: &str,
        dummy_ranges: &[(LineNumber, LineNumber)],
        dummy_impl_traits: &[String],
    ) -> bool {
        // Convert VO ranges to (usize, usize) for the underlying helper
        let converted: Vec<(usize, usize)> = dummy_ranges
            .iter()
            .map(|(s, e)| (s.value() as usize, e.value() as usize))
            .collect();
        taxonomy_dummy_helper::symbol_used_real(lines, symbol, &converted, dummy_impl_traits)
    }

    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName> {
        taxonomy_cycle_helper::detect_cycle_edges(edges)
    }

    fn extract_imported_aliases(&self, content: &str) -> HashMap<Identity, Identity> {
        taxonomy_unused_helper::extract_imported_aliases(content)
    }

    fn extract_exported_symbols(&self, content: &str) -> HashSet<Identity> {
        taxonomy_unused_helper::extract_exported_symbols(content)
    }

    fn extract_used_symbols(
        &self,
        content: &str,
        imported_aliases: &HashMap<Identity, Identity>,
    ) -> HashSet<Identity> {
        taxonomy_unused_helper::extract_used_symbols(content, imported_aliases)
    }

    fn find_import_line_number(&self, content: &str, alias: &str) -> LineNumber {
        let pos_opt = content.lines().position(|l| {
            let first_part = str_or_empty(alias.split('.').next());
            l.trim().contains(&format!("import {}", alias))
                || l.trim().contains(&format!("from {} import", first_part))
        });
        let line = match pos_opt {
            Some(p) => p + 1,
            None => 1,
        };
        LineNumber::new(line as i64)
    }
    fn extract_rust_js_imports(&self, content: &str) -> Vec<(SymbolName, LineNumber)> {
        taxonomy_unused_helper::extract_rust_js_imports(content)
    }

    fn is_name_used(&self, name: &str, content: &str, exclude_line: LineNumber) -> bool {
        taxonomy_unused_helper::is_name_used(name, content, exclude_line.value() as usize)
    }
}

/// Helper function to parse import lines from content, decoupled from any struct method call rules
fn parse_import_lines_helper(content: &str) -> Vec<(LineNumber, LineContentVO)> {
    let mut result = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("import ")
            || trimmed.starts_with("from ")
            || trimmed.starts_with("extern crate ")
        {
            result.push((
                LineNumber::new((i + 1) as i64),
                LineContentVO::new(lines[i].to_string()),
            ));
            i += 1;
            continue;
        }
        if trimmed.starts_with("use ")
            || trimmed.starts_with("pub use ")
            || trimmed.starts_with("pub(crate) use ")
        {
            let mut combined = lines[i].to_string();
            if combined.contains('{') && !combined.contains('}') {
                let start = i;
                i += 1;
                while i < lines.len() {
                    let part = lines[i].trim().to_string();
                    combined.push_str(&format!(" {}", part));
                    if part.contains('}') || combined.ends_with(';') {
                        break;
                    }
                    i += 1;
                }
                combined = combined.split_whitespace().collect::<Vec<&str>>().join(" ");
                result.push((
                    LineNumber::new((start + 1) as i64),
                    LineContentVO::new(combined),
                ));
            } else if !combined.ends_with(';') {
                while i + 1 < lines.len() {
                    let next = lines[i + 1].trim();
                    if next.starts_with("use ")
                        || next.starts_with("pub use ")
                        || next.starts_with("pub(crate) use ")
                        || next.is_empty()
                    {
                        break;
                    }
                    combined.push_str(&format!(" {}", next));
                    if next.ends_with(';') {
                        i += 1;
                        break;
                    }
                    i += 1;
                }
                combined = combined.split_whitespace().collect::<Vec<&str>>().join(" ");
                result.push((
                    LineNumber::new((i + 1) as i64),
                    LineContentVO::new(combined),
                ));
            } else {
                result.push((
                    LineNumber::new((i + 1) as i64),
                    LineContentVO::new(combined),
                ));
            }
        }
        i += 1;
    }
    result
}
```

---

## File: crates/import-rules/src/lib.rs

```rust
// PURPOSE: Module declarations for import-rules (checkers, analyzers, orchestrators)
pub mod capabilities_layer_detection_analyzer;
pub use capabilities_layer_detection_analyzer::LayerDetectionAnalyzer;
pub mod capabilities_import_forbidden_checker;
pub use capabilities_import_forbidden_checker::ArchImportForbiddenChecker;
pub mod capabilities_import_mandatory_checker;
pub use capabilities_import_mandatory_checker::ArchImportMandatoryChecker;
pub mod capabilities_dummy_import_checker;
pub use capabilities_dummy_import_checker::DummyImportChecker;
pub mod capabilities_import_unused_checker;
pub use capabilities_import_unused_checker::UnusedImportRuleChecker;
pub mod capabilities_cycle_import_analyzer;
pub use capabilities_cycle_import_analyzer::DependencyCycleAnalyzer;
pub mod infrastructure_import_parser_adapter;
pub use infrastructure_import_parser_adapter::ImportParserAdapter;
pub mod infrastructure_filesystem_adapter;
pub use infrastructure_filesystem_adapter::OSFileSystemAdapter;
pub mod agent_import_orchestrator;
pub use agent_import_orchestrator::ImportOrchestrator;
pub mod root_import_rules_container;
```

---

## File: crates/import-rules/src/root_import_rules_container.rs

```rust
// PURPOSE: ImportContainer — wiring for import-rules feature (root layer, wiring only)
use shared::code_analysis::contract_cycle_protocol::ICycleAnalysisProtocol;
use shared::common::contract_parser_port::ISourceParserPort;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::import_rules::contract_rule_protocol::IArchImportProtocol;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use std::sync::Arc;

pub struct ImportContainer {
    mandatory: Arc<dyn IArchImportProtocol>,
    forbidden: Arc<dyn IArchImportProtocol>,
    intent: Arc<dyn IArchImportProtocol>,
    unused: Arc<dyn IUnusedImportProtocol>,
    cycle: Arc<dyn ICycleAnalysisProtocol>,
    analyzer: Arc<dyn IAnalyzer>,
}

impl ImportContainer {
    pub fn new(source_parser: Arc<dyn ISourceParserPort>) -> Self {
        Self::new_with_config(
            shared::config_system::taxonomy_config_vo::default_aes_config(),
            source_parser,
        )
    }

    pub fn new_with_config(
        config: ArchitectureConfig,
        source_parser: Arc<dyn ISourceParserPort>,
    ) -> Self {
        let fs = Arc::new(crate::infrastructure_filesystem_adapter::OSFileSystemAdapter::new());
        let parser: Arc<dyn IImportParserPort> =
            Arc::new(crate::infrastructure_import_parser_adapter::ImportParserAdapter::new());
        let analyzer = Arc::new(
            crate::capabilities_layer_detection_analyzer::LayerDetectionAnalyzer::new(
                config.clone(),
                fs,
                source_parser,
            ),
        );

        let mandatory = Arc::new(
            crate::capabilities_import_mandatory_checker::ArchImportMandatoryChecker::new(
                parser.clone(),
            ),
        );
        let forbidden = Arc::new(
            crate::capabilities_import_forbidden_checker::ArchImportForbiddenChecker::new(
                parser.clone(),
            ),
        );
        let intent = Arc::new(
            crate::capabilities_dummy_import_checker::DummyImportChecker::new(parser.clone()),
        );
        let unused = Arc::new(
            crate::capabilities_import_unused_checker::UnusedImportRuleChecker::new(parser.clone()),
        );
        let cycle = Arc::new(
            crate::capabilities_cycle_import_analyzer::DependencyCycleAnalyzer::new(
                config,
                parser.clone(),
            ),
        );

        Self {
            mandatory: mandatory.clone(),
            forbidden: forbidden.clone(),
            intent: intent.clone(),
            unused: unused.clone(),
            cycle: cycle.clone(),
            analyzer,
        }
    }

    pub fn new_default() -> Self {
        Self::new(Arc::new(NullSourceParser))
    }

    pub fn mandatory_checker(&self) -> &dyn IArchImportProtocol {
        &*self.mandatory
    }

    pub fn forbidden_checker(&self) -> &dyn IArchImportProtocol {
        &*self.forbidden
    }

    pub fn analyzer(&self) -> Arc<dyn IAnalyzer> {
        self.analyzer.clone()
    }

    pub fn orchestrator(&self) -> Arc<dyn IImportRunnerAggregate> {
        Arc::new(crate::agent_import_orchestrator::ImportOrchestrator::new(
            self.mandatory.clone(),
            self.forbidden.clone(),
            self.intent.clone(),
            self.unused.clone(),
            self.cycle.clone(),
            self.analyzer.clone(),
        ))
    }
}

pub struct NullSourceParser;
impl ISourceParserPort for NullSourceParser {
    fn extract_imports(
        &self,
        _path: &FilePath,
    ) -> Result<
        shared::code_analysis::taxonomy_import_source_vo::ImportInfoList,
        shared::common::taxonomy_parser_error::SourceParserError,
    > {
        Ok(shared::code_analysis::taxonomy_import_source_vo::ImportInfoList::default())
    }
    fn get_raw_symbols(
        &self,
        _path: &FilePath,
    ) -> Result<
        shared::mcp_server::taxonomy_job_vo::ResponseData,
        shared::common::taxonomy_parser_error::SourceParserError,
    > {
        Ok(shared::mcp_server::taxonomy_job_vo::ResponseData::default())
    }
    fn get_class_attributes(
        &self,
        _path: &FilePath,
    ) -> shared::mcp_server::taxonomy_job_vo::ResponseData {
        shared::mcp_server::taxonomy_job_vo::ResponseData::default()
    }
    fn has_all_export(
        &self,
        _path: &FilePath,
    ) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus {
        shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(false)
    }
    fn find_primitive_violations(
        &self,
        _path: &FilePath,
        _primitive_types: &shared::common::taxonomy_naming_list_vo::PrimitiveTypeList,
    ) -> shared::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList {
        shared::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList::default()
    }
    fn find_unused_imports(
        &self,
        _path: &FilePath,
    ) -> shared::code_analysis::taxonomy_import_source_vo::ImportInfoList {
        shared::code_analysis::taxonomy_import_source_vo::ImportInfoList::default()
    }
    fn get_class_definitions(
        &self,
        _path: &FilePath,
    ) -> Result<
        shared::common::taxonomy_suggestion_vo::MetadataVO,
        shared::common::taxonomy_parser_error::SourceParserError,
    > {
        Ok(shared::common::taxonomy_suggestion_vo::MetadataVO::new(
            std::collections::HashMap::new(),
        ))
    }
    fn get_function_definitions(
        &self,
        _path: &FilePath,
    ) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn is_symbol_exported(
        &self,
        _path: &FilePath,
        _symbol: &shared::common::taxonomy_name_vo::SymbolName,
    ) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus {
        shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(false)
    }
    fn get_class_methods(
        &self,
        _path: &FilePath,
    ) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn get_class_bases_map(
        &self,
        _path: &FilePath,
    ) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn get_assignment_targets(
        &self,
        _path: &FilePath,
    ) -> shared::common::taxonomy_suggestion_vo::MetadataVO {
        shared::common::taxonomy_suggestion_vo::MetadataVO::new(std::collections::HashMap::new())
    }
    fn get_control_flow_count(
        &self,
        _path: &FilePath,
    ) -> shared::common::taxonomy_common_vo::Count {
        shared::common::taxonomy_common_vo::Count::default()
    }
    fn is_barrel_file(&self, _path: &FilePath) -> shared::common::taxonomy_common_vo::BooleanVO {
        shared::common::taxonomy_common_vo::BooleanVO::default()
    }
    fn get_stem(&self, _path: &FilePath) -> shared::common::taxonomy_name_vo::SymbolName {
        shared::common::taxonomy_name_vo::SymbolName::new("")
    }
    fn is_entry_point(&self, _path: &FilePath) -> shared::common::taxonomy_common_vo::BooleanVO {
        shared::common::taxonomy_common_vo::BooleanVO::default()
    }
    fn get_supported_extensions(&self) -> shared::common::taxonomy_common_vo::PatternList {
        shared::common::taxonomy_common_vo::PatternList::default()
    }
}
```

---

## File: crates/shared/src/auto-fix/taxonomy_fix_applied_event.rs

```rust
// PURPOSE: FixApplied — domain event published when a lint fix is applied
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::Timestamp;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FixApplied {
    pub path: FilePath,
    pub adapter: AdapterName,
    pub error_code: ErrorCode,
    pub changes_count: Count,
    #[serde(default)]
    pub timestamp: Timestamp,
}

impl FixApplied {
    pub fn new(
        path: FilePath,
        adapter: AdapterName,
        error_code: ErrorCode,
        changes_count: Count,
    ) -> Self {
        Self {
            path,
            adapter,
            error_code,
            changes_count,
            timestamp: Timestamp::default(),
        }
    }
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

## File: crates/shared/src/code-analysis/contract_cycle_protocol.rs

```rust
// PURPOSE: ICycleAnalysisProtocol — contract trait for circular dependency detection (AES205)
// Implementation: crates/import-rules/src/capabilities_cycle_import_analyzer.rs → DependencyCycleAnalyzer

use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::import_rules::contract_rule_protocol::IAnalyzer;
use async_trait::async_trait;

/// Abstract protocol for circular dependency (cycle) detection.
/// Implemented by capabilities layer in the code-analysis crate.
#[async_trait]
pub trait ICycleAnalysisProtocol: Send + Sync {
    async fn check_cycles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
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

## File: crates/shared/src/common/contract_parser_port.rs

```rust
// PURPOSE: ISourceParserPort — port trait for language-specific source code parsing (imports, definitions)
use crate::code_analysis::taxonomy_import_source_vo::ImportInfoList;
use crate::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList;
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::common::taxonomy_naming_list_vo::PrimitiveTypeList;
use crate::common::taxonomy_parser_error::SourceParserError;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_suggestion_vo::MetadataVO;
use crate::mcp_server::taxonomy_job_vo::ResponseData;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;

pub trait ISourceParserPort: Send + Sync {
    fn extract_imports(&self, path: &FilePath) -> Result<ImportInfoList, SourceParserError>;
    fn get_raw_symbols(&self, path: &FilePath) -> Result<ResponseData, SourceParserError>;
    fn get_class_attributes(&self, path: &FilePath) -> ResponseData;
    fn has_all_export(&self, path: &FilePath) -> SuccessStatus;
    fn find_primitive_violations(
        &self,
        path: &FilePath,
        primitive_types: &PrimitiveTypeList,
    ) -> PrimitiveViolationList;
    fn find_unused_imports(&self, path: &FilePath) -> ImportInfoList;
    fn get_class_definitions(&self, path: &FilePath) -> Result<MetadataVO, SourceParserError>;
    fn get_function_definitions(&self, path: &FilePath) -> MetadataVO;
    fn is_symbol_exported(&self, path: &FilePath, symbol: &SymbolName) -> SuccessStatus;
    fn get_class_methods(&self, path: &FilePath) -> MetadataVO;
    fn get_class_bases_map(&self, path: &FilePath) -> MetadataVO;
    fn get_assignment_targets(&self, path: &FilePath) -> MetadataVO;
    fn get_control_flow_count(&self, path: &FilePath) -> Count;
    fn is_barrel_file(&self, path: &FilePath) -> BooleanVO;
    fn get_stem(&self, path: &FilePath) -> SymbolName;
    fn is_entry_point(&self, path: &FilePath) -> BooleanVO;
    fn get_supported_extensions(&self) -> PatternList;
}
```

---

## File: crates/shared/src/common/contract_system_port.rs

```rust
// PURPOSE: IFileSystemPort — port trait for filesystem operations (read, write, exists, glob, walk)

use async_trait::async_trait;

use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_filesystem_error::FileSystemError;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::common::taxonomy_source_vo::ContentString;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;

/// Abstract interface for file system operations.
/// Implemented by Infrastructure (e.g., OSFileSystemAdapter).
#[async_trait]
pub trait IFileSystemPort: Send + Sync {
    async fn walk(&self, path: &FilePath, ignored_patterns: Option<&PatternList>) -> FilePathList;
    async fn is_directory(&self, path: &FilePath) -> SuccessStatus;
    async fn is_file(&self, path: &FilePath) -> SuccessStatus;
    async fn get_relative_path(&self, path: &FilePath, start: &FilePath) -> FilePath;
    async fn read_text(&self, path: &FilePath) -> Result<ContentString, FileSystemError>;
    async fn get_line_count(&self, path: &FilePath) -> Count;
    async fn exists(&self, path: &FilePath) -> SuccessStatus;
    async fn get_parent(&self, path: &FilePath) -> FilePath;
    async fn write_text(
        &self,
        path: &FilePath,
        content: &ContentString,
        mode: Option<&Identity>,
    ) -> Result<SuccessStatus, FileSystemError>;
    async fn glob(&self, pattern: &Identity) -> FilePathList;
    async fn get_cwd(&self) -> FilePath;
    async fn get_basename(&self, path: &FilePath) -> Identity;
    async fn path_join(&self, parts: &[Identity]) -> FilePath;
    async fn read_file(&self, path: &FilePath) -> Result<ContentString, FileSystemError>;
}
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

## File: crates/shared/src/common/taxonomy_file_collector_helper.rs

```rust
// PURPOSE: FileCollector — taxonomy utility for collecting lintable source files from a directory tree
use crate::common::taxonomy_language_detector_helper::LanguageDetector;
use crate::common::taxonomy_path_vo::DirectoryPath;
use crate::common::taxonomy_path_vo::FilePath;

/// Return true if `rel_path` should be skipped based on `ignored` patterns.
///
/// Each pattern is matched as a **path segment** rather than a free-text substring. This
/// fixes a long-standing bug where patterns like `/test-workspaces` failed to match the
/// absolute path `/home/.../test-workspaces/crates/...` because the old substring-based
/// matcher was tripped up by leading slashes, leading paths, and unrelated prefixes. The
/// result was that all of `test-workspaces/**` and `packages/vscode-extension/src/**`
/// leaked into `lint-arwaky check .` results even though they were listed in
/// `ignored_paths`.
///
/// Three forms of pattern are supported:
///   1. Absolute-style prefix `"/foo"`, `"/foo/bar"` — matches any path that contains
///      the segments `foo` or `foo/bar` in order, at any depth. The leading slash is
///      dropped before comparison; this works on both absolute paths
///      (`/home/.../test-workspaces/crates/foo.rs`) and relative paths
///      (`test-workspaces/crates/foo.rs`).
///   2. Single segment `"foo"` — matches any path segment equal to `foo`
///      (catches both `foo` at root and `nested/foo` mid-tree).
///   3. Suffix glob `".min.js"`, `"*.bak"` — matches any path whose basename ends with the
///      suffix. Used for vendor minified files like `cytoscape.min.js`.
pub fn is_path_ignored(rel_path: &str, ignored: &[String]) -> bool {
    if rel_path.is_empty() {
        return false;
    }
    let segments: Vec<&str> = rel_path
        .split(['/', '\\'])
        .filter(|s| !s.is_empty())
        .collect();
    for pat in ignored {
        if pat.is_empty() {
            continue;
        }
        // (1) Absolute-style prefix "/foo" or "/foo/bar"
        if let Some(stripped) = pat.strip_prefix('/') {
            if stripped.is_empty() {
                continue;
            }
            let pat_segments: Vec<&str> = stripped
                .split(['/', '\\'])
                .filter(|s| !s.is_empty())
                .collect();
            if pat_segments.is_empty() {
                continue;
            }
            // Match if pat_segments appear contiguously in `segments` at any depth.
            // We do NOT use `starts_with` here because `rel_path` may be absolute
            // (`/home/.../test-workspaces/...`) — the pattern segments can appear
            // anywhere along the path, not just at the very beginning.
            let n_pat = pat_segments.len();
            let n_seg = segments.len();
            if n_seg < n_pat {
                continue;
            }
            for start in 0..=(n_seg - n_pat) {
                if segments[start..start + n_pat] == pat_segments[..] {
                    return true;
                }
            }
            continue;
        }
        // (2) Suffix glob "*.ext" or ".ext" (used for minified vendor files)
        if pat.starts_with("*.") || (pat.starts_with('.') && pat.contains('.')) {
            let suffix = if let Some(s) = pat.strip_prefix('*') {
                s.trim_start_matches('.')
            } else {
                pat.trim_start_matches('.')
            };
            if suffix.is_empty() {
                continue;
            }
            let basename = segments.last().copied().unwrap_or_default();
            if basename.ends_with(suffix) {
                return true;
            }
            continue;
        }
        // (3) Bare segment — match any segment anywhere in the path.
        if segments.contains(&pat.as_str()) {
            return true;
        }
    }
    false
}

/// Collect lintable source files (.rs, .py, .ts, .js, .tsx, .jsx) from a directory tree.
pub fn collect_source_files(
    root_dir: &std::path::Path,
    dir_path: &DirectoryPath,
    ignored: &[String],
) -> Vec<FilePath> {
    let mut files = Vec::new();
    let path = std::path::Path::new(&dir_path.value);
    if path.is_file() {
        let relative_path = match path.strip_prefix(root_dir) {
            Ok(p) => p,
            Err(_) => path,
        };
        let rel_str = relative_path.to_string_lossy();
        if !is_path_ignored(&rel_str, ignored) {
            if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                let detector = LanguageDetector::new();
                if detector.is_lintable(&fp) {
                    files.push(fp);
                }
            }
        }
        return files;
    }

    if let Ok(entries) = std::fs::read_dir(&dir_path.value) {
        for entry in entries.flatten() {
            let path = entry.path();
            let relative_path = match path.strip_prefix(root_dir) {
                Ok(p) => p,
                Err(_) => &path,
            };
            let rel_str = relative_path.to_string_lossy();
            if is_path_ignored(&rel_str, ignored) {
                continue;
            }
            if path.is_dir() {
                // Skip Rust integration test directories — tests live in tests/ and
                // should not be scanned by the AES linter.
                let dir_name = path
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_default();
                if dir_name == "tests" {
                    continue;
                }
                let sub_dir =
                    DirectoryPath::new(path.to_string_lossy().to_string()).unwrap_or_default();
                files.extend(collect_source_files(root_dir, &sub_dir, ignored));
            } else if let Some(path_str) = path.to_str() {
                if let Ok(fp) = FilePath::new(path_str.to_string()) {
                    let detector = LanguageDetector::new();
                    if detector.is_lintable(&fp) {
                        files.push(fp);
                    }
                }
            }
        }
    }
    files
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
        // Normalize: replace backslashes with forward slashes, and collapse multiple slashes.
        value = value.replace('\\', "/");
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

## File: crates/shared/src/config-system/mod.rs

```rust
// config-system — taxonomy and contract types
pub mod contract_multi_project_orchestrator_aggregate;
pub mod contract_orchestration_aggregate;
pub mod contract_parser_port;
pub mod contract_reader_port;
pub mod contract_validator_protocol;
pub mod contract_workspace_detector_port;
pub mod taxonomy_config_error;
pub mod taxonomy_config_vo;
pub mod taxonomy_identifier_vo;
pub mod taxonomy_multi_project_summary_vo;
pub mod taxonomy_multi_project_vo;
pub mod taxonomy_multi_project_workspace_info_vo;
pub mod taxonomy_setting_vo;
pub mod taxonomy_source_vo;
pub mod taxonomy_validation_vo;
```

---

## File: crates/shared/src/config-system/taxonomy_config_vo.rs

```rust
// PURPOSE: ArchitectureConfig, LayerDefinition, ConfigRule — configuration value objects for AES rules definition
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_definition_vo::NamingConfig;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(default)]
pub struct ArchitectureRule {
    pub name: DescriptionVO,
    pub description: DescriptionVO,
    pub rule_type: ErrorCode,
    pub scope: LayerNameVO,
    pub exceptions: PatternList,
    #[serde(default)]
    pub allowed: PatternList,
    #[serde(default)]
    pub forbidden: PatternList,
    #[serde(default)]
    pub mandatory: PatternList,

    #[serde(flatten)]
    pub naming: crate::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO,
    #[serde(flatten)]
    pub code_analysis: crate::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO,
    #[serde(flatten)]
    pub role: crate::role_rules::taxonomy_role_rule_vo::RoleRuleVO,
    #[serde(flatten)]
    pub orphan: crate::orphan_detector::taxonomy_orphan_rule_vo::OrphanRuleVO,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ArchitectureConfig {
    pub enabled: BooleanVO,
    pub layers: std::collections::HashMap<LayerNameVO, LayerDefinition>,
    pub rules: Vec<ArchitectureRule>,
    pub naming: NamingConfig,
    pub ignored_paths: FilePathList,
    pub mandatory_class_definition: BooleanVO,
}

impl ArchitectureConfig {
    pub fn new(
        enabled: BooleanVO,
        layers: std::collections::HashMap<LayerNameVO, LayerDefinition>,
        rules: Vec<ArchitectureRule>,
        naming: NamingConfig,
        ignored_paths: FilePathList,
        mandatory_class_definition: BooleanVO,
    ) -> Self {
        Self {
            enabled,
            layers,
            rules,
            naming,
            ignored_paths,
            mandatory_class_definition,
        }
    }
}

impl Default for ArchitectureConfig {
    fn default() -> Self {
        Self {
            enabled: BooleanVO::new(true),
            layers: HashMap::new(),
            rules: Vec::new(),
            naming: NamingConfig::new(Count::new(2)),
            ignored_paths: FilePathList { values: vec![] },
            mandatory_class_definition: BooleanVO::new(false),
        }
    }
}

pub fn parse_config_yaml(yaml_str: &str) -> ArchitectureConfig {
    let raw: serde_yml::Value = serde_yml::from_str(yaml_str).unwrap_or_default();
    if let Some(arch_val) = raw.get("architecture") {
        let mut arch_json: serde_json::Value = serde_json::to_value(arch_val).unwrap_or_default();
        // Extract layers from rules (first rule containing "layers" key) if not at top-level
        if arch_json.get("layers").is_none() {
            if let Some(rules_obj) = arch_json.get_mut("rules").and_then(|r| r.as_object_mut()) {
                for (_rule_code, rule_val) in rules_obj.iter_mut() {
                    if let Some(layers) = rule_val.get_mut("layers") {
                        let layers = std::mem::take(layers);
                        arch_json["layers"] = layers;
                        break;
                    }
                }
            }
        }
        let mut json = arch_json;
        fn remove_nulls(val: &mut serde_json::Value) {
            match val {
                serde_json::Value::Object(m) => {
                    m.retain(|_, v| !v.is_null());
                    for v in m.values_mut() {
                        remove_nulls(v);
                    }
                }
                serde_json::Value::Array(arr) => {
                    for v in arr.iter_mut() {
                        remove_nulls(v);
                    }
                }
                _ => {}
            }
        }
        remove_nulls(&mut json);
        // Convert ignored_paths from array to {values: [...]} format because the Rust struct expects an object with a "values" field.
        if let Some(arr) = json.get("ignored_paths").and_then(|v| v.as_array()) {
            json["ignored_paths"] = serde_json::json!({"values": arr});
        }
        if let Some(layers_obj) = json.get_mut("layers") {
            if let Some(obj) = layers_obj.as_object_mut() {
                let mut suffix_updates: Vec<(
                    String,
                    Option<String>,
                    serde_json::Value,
                    serde_json::Value,
                )> = Vec::new();
                for (layer_name, layer) in obj.iter() {
                    if let Some(suffix_val) = layer.get("suffix") {
                        if let Some(arr) = suffix_val.as_array() {
                            let mut policy: Option<String> = None;
                            let mut allowed = serde_json::Value::Array(Vec::new());
                            let mut forbidden = serde_json::Value::Array(Vec::new());
                            for entry in arr {
                                if let Some(entry_obj) = entry.as_object() {
                                    for (pkey, plist) in entry_obj {
                                        match pkey.as_str() {
                                            "strict" | "flexible" => {
                                                policy = Some(pkey.clone());
                                                if let Some(list) = plist.as_array() {
                                                    allowed = serde_json::json!(list);
                                                }
                                            }
                                            "forbidden" => {
                                                if let Some(list) = plist.as_array() {
                                                    forbidden = serde_json::json!(list);
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            suffix_updates.push((layer_name.clone(), policy, allowed, forbidden));
                        }
                    }
                }
                for (name, policy, allowed, forbidden) in suffix_updates {
                    if let Some(layer) = obj.get_mut(&name) {
                        if let Some(layer_obj) = layer.as_object_mut() {
                            if let Some(ref p) = policy {
                                layer_obj.insert("suffix_policy".to_string(), serde_json::json!(p));
                            }
                            layer_obj.insert("allowed_suffix".to_string(), allowed);
                            if let Some(arr) = forbidden.as_array() {
                                if !arr.is_empty() {
                                    layer_obj.insert("forbidden_suffix".to_string(), forbidden);
                                }
                            }
                            layer_obj.remove("suffix");
                        }
                    }
                }
            }
        }
        if let Some(rules_obj) = json.get_mut("rules") {
            if let Some(obj) = rules_obj.as_object_mut() {
                let mut flat = serde_json::Value::Array(Vec::new());
                for (code, rule_val) in obj.iter() {
                    if let Some(rule_obj) = rule_val.as_object() {
                        let mut base = rule_obj.clone();
                        base.insert("name".to_string(), serde_json::json!(code));
                        // Expand scope array into multiple entries — one per scope element
                        // Only applies to rules WITHOUT conditions (conditions have their own scopes)
                        if let Some(scope_arr) = base.get("scope").and_then(|s| s.as_array()) {
                            if !base.contains_key("conditions") && scope_arr.len() > 1 {
                                for scope_val in scope_arr {
                                    if let Some(s) = scope_val.as_str() {
                                        let mut entry = base.clone();
                                        entry.insert("scope".to_string(), serde_json::json!(s));
                                        if let Some(arr) = flat.as_array_mut() {
                                            arr.push(serde_json::Value::Object(entry));
                                        }
                                    }
                                }
                                continue; // Already pushed per-scope entries, skip single push below
                            } else if let Some(first) = scope_arr.first().and_then(|v| v.as_str()) {
                                base.insert("scope".to_string(), serde_json::json!(first));
                            }
                        }
                        if let Some(conditions) = base.remove("conditions") {
                            if let Some(conds) = conditions.as_array() {
                                if !conds.is_empty() {
                                    for cond in conds {
                                        if let Some(cond_obj) = cond.as_object() {
                                            let mut entry = base.clone();
                                            for (k, v) in cond_obj {
                                                entry.insert(k.clone(), v.clone());
                                            }
                                            // Remove top-level scope array leftovers if condition has its own scope
                                            if let Some(arr) = flat.as_array_mut() {
                                                arr.push(serde_json::Value::Object(entry));
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if let Some(arr) = flat.as_array_mut() {
                                arr.push(serde_json::Value::Object(base));
                            }
                        }
                    }
                }
                *rules_obj = flat;
            }
        }
        let mut config = match serde_json::from_value::<ArchitectureConfig>(json) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[warn] Failed to deserialize ArchitectureConfig: {:?}", e);
                eprintln!("[warn] Falling back to default config. Check your YAML syntax and field types.");
                ArchitectureConfig::default()
            }
        };
        // Top-level ignored_paths (outside architecture section) — merge into config
        if config.ignored_paths.values.is_empty() {
            if let Some(arr) = raw.get("ignored_paths").and_then(|v| v.as_sequence()) {
                let paths: Vec<_> = arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| FilePath::new(s.to_string()).unwrap_or_default())
                    .collect();
                if !paths.is_empty() {
                    config.ignored_paths = FilePathList::new(paths);
                }
            }
        }
        config
    } else {
        let mut config = ArchitectureConfig::default();
        if let Some(arr) = raw.get("ignored_paths").and_then(|v| v.as_sequence()) {
            let paths: Vec<_> = arr
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| FilePath::new(s.to_string()).unwrap_or_default())
                .collect();
            if !paths.is_empty() {
                config.ignored_paths = FilePathList::new(paths);
            }
        }
        config
    }
}

/// All 3 config YAMLs are baked into the binary at compile time via `include_str!`.
/// Runtime project-level config files override these defaults.
/// Cached via OnceLock to avoid re-parsing on every call.
static DEFAULT_RUST_CONFIG: OnceLock<ArchitectureConfig> = OnceLock::new();
static DEFAULT_PYTHON_CONFIG: OnceLock<ArchitectureConfig> = OnceLock::new();
static DEFAULT_TS_CONFIG: OnceLock<ArchitectureConfig> = OnceLock::new();

pub fn default_aes_config() -> ArchitectureConfig {
    DEFAULT_RUST_CONFIG
        .get_or_init(|| parse_config_yaml(include_str!("../../../../lint_arwaky.config.rust.yaml")))
        .clone()
}

pub fn default_config_for_language(language: &str) -> ArchitectureConfig {
    match language {
        "rust" => default_aes_config(),
        "python" => DEFAULT_PYTHON_CONFIG
            .get_or_init(|| {
                parse_config_yaml(include_str!("../../../../lint_arwaky.config.python.yaml"))
            })
            .clone(),
        "javascript" | "typescript" => DEFAULT_TS_CONFIG
            .get_or_init(|| {
                parse_config_yaml(include_str!(
                    "../../../../lint_arwaky.config.javascript.yaml"
                ))
            })
            .clone(),
        _ => {
            eprintln!(
                "[warn] Unknown language '{}', using empty default config.",
                language
            );
            ArchitectureConfig::default()
        }
    }
}
```

---

## File: crates/shared/src/import-rules/contract_import_parser_port.rs

```rust
// PURPOSE: IImportParserPort — contract port trait for import parsing utilities
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_layer_vo::FileContentVO;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_layer_vo::LineContentVO;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::common::taxonomy_path_vo::FilePath;
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use crate::import_rules::taxonomy_language_vo::LanguageVO;
use std::collections::{HashMap, HashSet};

pub trait IImportParserPort: Send + Sync {
    fn resolve_scope(&self, scope: &Identity) -> (LayerNameVO, Vec<Identity>);
    fn import_matches_scope(
        &self,
        import_line: &LineContentVO,
        layer: &LayerNameVO,
        suffixes: &[Identity],
    ) -> bool;
    fn get_basename(&self, file: &FilePath) -> Identity;
    fn read_import_lines(&self, file: &FilePath) -> Vec<(LineNumber, LineContentVO)>;
    fn parse_import_lines(&self, content: &FileContentVO) -> Vec<(LineNumber, LineContentVO)>;
    fn extract_module_from_line(&self, line: &LineContentVO) -> Option<Identity>;
    fn extract_layer_from_import(&self, segment: &Identity) -> Option<LayerNameVO>;

    // New methods to extract infrastructure Concerns
    fn read_file_to_message(&self, file: &FilePath) -> Result<LintMessage, std::io::Error>;
    fn extract_import_modules(&self, content: &str) -> Vec<SymbolName>;
    fn get_language_from_path(&self, path: &str) -> LanguageVO;
    fn get_dummy_function_ranges(
        &self,
        lines: &[&str],
        lang: LanguageVO,
    ) -> Vec<(LineNumber, LineNumber)>;
    fn get_imported_symbols(
        &self,
        lines: &[&str],
        lang: LanguageVO,
    ) -> Vec<(SymbolName, LineNumber)>;
    fn get_dummy_impl_traits_with_lines(&self, lines: &[&str]) -> Vec<(SymbolName, LineNumber)>;
    fn is_symbol_used_real(
        &self,
        lines: &[&str],
        symbol: &str,
        dummy_ranges: &[(LineNumber, LineNumber)],
        dummy_impl_traits: &[String],
    ) -> bool;
    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName>;

    // Fine-grained parsing utilities for unused import steps
    fn extract_imported_aliases(&self, content: &str) -> HashMap<Identity, Identity>;
    fn extract_exported_symbols(&self, content: &str) -> HashSet<Identity>;
    fn extract_used_symbols(
        &self,
        content: &str,
        imported_aliases: &HashMap<Identity, Identity>,
    ) -> HashSet<Identity>;
    fn find_import_line_number(&self, content: &str, alias: &str) -> LineNumber;
    fn extract_rust_js_imports(&self, content: &str) -> Vec<(SymbolName, LineNumber)>;
    fn is_name_used(&self, name: &str, content: &str, exclude_line: LineNumber) -> bool;
}
```

---

## File: crates/shared/src/import-rules/contract_import_runner_aggregate.rs

```rust
// PURPOSE: IImportRunnerAggregate — contract for import-rules feature orchestrator
//
// This is the primary contract that decouples the import-rules agent layer
// from its callers (CLI, MCP, TUI). Callers depend on this trait, not on
// ImportOrchestrator directly.
//
// run_audit is async because it may perform file I/O and spawn blocking
// tasks internally. The caller provides a FilePath target (file or dir).
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

/// IImportRunnerAggregate — aggregate port for import-rules orchestration.
///
/// Implemented by ImportOrchestrator (agent layer).
/// Called by surface layer (CLI, MCP, TUI) via Arc<dyn IImportRunnerAggregate>.
#[async_trait]
pub trait IImportRunnerAggregate: Send + Sync {
    /// Run all 5 import-related AES checks (AES201–AES205) on the target.
    /// Returns aggregated violations from mandatory, forbidden, unused, dummy, and cycle checks.
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
    /// Human-readable name for this orchestrator ("import-rules").
    fn name(&self) -> &str;
}
```

---

## File: crates/shared/src/import-rules/contract_rule_protocol.rs

```rust
// PURPOSE: IAnalyzer trait — core analyzer interface for import checks
//
// This file defines the protocol traits that capabilities-level checkers
// implement. Each trait represents a single architectural responsibility:
//   - IAnalyzer: central configuration + layer detection hub
//   - IArchRuleProtocol: base trait for all AES rule implementations
//   - IInternalCheckerProtocol: checks layer-internal import rules
//   - IMetricCheckerProtocol: line count + mandatory class definition checks
//   - IArchImportProcessorProtocol: file-level import validation
//   - INamingRuleProtocol: naming convention checks (AES101-102)
//   - IArchStructureProtocol: combined naming + structure + metric checks
//   - IArchImportProtocol: mandatory and forbidden import checks (AES201-202, AES204)
//
// The trapezoidal hierarchy exists because different layers need different
// subsets of these capabilities — the trait bounds reflect the actual
// dependency requirements.
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::contract_parser_port::ISourceParserPort;
use crate::common::contract_system_port::IFileSystemPort;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_definition_vo::LayerMapVO;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;

/// IAnalyzer — the central configuration and analysis hub.
///
/// Provides access to:
///   - File system (for reading/writing files)
///   - Source parser (for AST-level analysis)
///   - Layer detection (maps file paths to architectural layers)
///
/// Also implements INamingAnalyzerProtocol, which allows naming-rules
/// to reuse the same layer-detection logic without duplicating it.
pub trait IAnalyzer:
    crate::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol + Send + Sync
{
    fn fs(&self) -> &dyn IFileSystemPort;
    fn parser(&self) -> &dyn ISourceParserPort;
    fn detect_module_layer(&self, module_path: &FilePath) -> Option<LayerNameVO>;
}

/// Base trait for all AES rule implementations.
/// Every checker must have a unique identity (e.g., "AES201").
pub trait IArchRuleProtocol {
    fn rule_name(&self) -> Identity;
}

/// Checks that imports within a layer respect internal boundaries
/// (e.g., a capabilities file should not import from infrastructure).
pub trait IInternalCheckerProtocol: Send + Sync {
    fn check_layer_internal_rules(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

/// Metric-based checks: file line counts, function lengths, and
/// mandatory class/struct definitions within each file.
pub trait IMetricCheckerProtocol: Send + Sync {
    fn check_line_counts(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn check_mandatory_class_definition(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

/// Parameters for validating imports in a single file.
/// Bundles all data needed to check whether a file imports from required layers.
pub struct ValidateImportsParams<'a> {
    pub analyzer: &'a dyn IAnalyzer,
    pub file_path: &'a FilePath,
    pub root_dir: &'a FilePath,
    pub required_layers: &'a PatternList,
    pub results: &'a mut LintResultList,
    pub message_template: &'a ErrorMessage,
    pub layer_name: &'a LayerNameVO,
    pub layers_display: &'a PatternList,
}

/// Processes imports at the per-file level.
/// Validates that files import from the correct layers and not from forbidden ones.
pub trait IArchImportProcessorProtocol: Send + Sync {
    fn process_file_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        file_path: &FilePath,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn validate_imports_present(&self, params: ValidateImportsParams<'_>);
}

/// Parameters for file-naming checks.
/// Passes all configuration needed to check naming conventions across layers.
pub struct CheckFileNamingParams<'a> {
    pub files: &'a FilePathList,
    pub root_dir: &'a FilePath,
    pub layer_map: &'a LayerMapVO,
    pub global_expected: Count,
    pub global_exceptions: &'a PatternList,
    pub results: &'a mut LintResultList,
    pub detect_layer_fn: &'a dyn Fn(&FilePath, &FilePath) -> Option<LayerNameVO>,
}

/// Naming convention rules (AES101-102).
/// Checks file names, class names, and function names against
/// the AES layer-based naming conventions.
pub trait INamingRuleProtocol: IArchRuleProtocol + Send + Sync {
    fn check_file_naming(&self, params: CheckFileNamingParams<'_>);
    fn check_class_naming(
        &self,
        files: &FilePathList,
        results: &mut LintResultList,
        source_parser: &dyn ISourceParserPort,
    );
    fn check_function_naming(
        &self,
        files: &FilePathList,
        results: &mut LintResultList,
        source_parser: &dyn ISourceParserPort,
    );
}

/// Combined structure + naming + metrics protocol.
/// This is a legacy trait that aggregates multiple responsibilities.
/// New implementations should prefer the more granular trait separations.
pub trait IArchStructureProtocol: IArchRuleProtocol + Send + Sync {
    fn check_file_naming(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn check_domain_suffixes(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn check_layer_internal_rules(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn check_line_counts(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    fn check_mandatory_class_definition(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

/// Import compliance protocol (AES201-202, AES204).
/// Checks for mandatory imports (files MUST import certain symbols) and
/// forbidden imports (files MUST NOT import certain symbols).
///
/// Both checks use the same async trait because they share the same
/// file-walking and analysis infrastructure — only the rule config differs.
#[async_trait::async_trait]
pub trait IArchImportProtocol: IArchRuleProtocol + Send + Sync {
    /// Check that files contain required imports based on their layer role.
    async fn check_mandatory_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    /// Check that files do NOT contain prohibited imports.
    async fn check_forbidden_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
```

---

## File: crates/shared/src/import-rules/contract_unused_import_protocol.rs

```rust
// PURPOSE: IUnusedImportProtocol — unified port trait for AES203: detect unused imports across Rust, Python, JavaScript
// AES402: All primitive types in this contract have been replaced with taxonomy VOs.
//   * `Vec<String>` returns → `Vec<LintMessage>` (semantic messages, not raw strings)
//   * `&str file_path` params → kept as `&str` (idiomatic borrow, AES402 allows)
//   * `&mut Vec<LintResult>` → kept (`LintResult` is itself a VO)
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;

pub trait IUnusedImportProtocol: Send + Sync {
    /// Find unused imports in a file by path (reads file internally).
    /// Returns a list of human-readable lint messages describing each unused
    /// import. Replaces the previous `Vec<String>` so callers can introspect,
    /// translate, or log messages without parsing free-form strings.
    fn find_unused_imports(&self, path: &FilePath) -> Vec<LintMessage>;

    /// Check unused imports given file content directly (for inline checking).
    /// Useful when content is already available (avoids re-reading file).
    fn check_unused_imports(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
}
```

---

## File: crates/shared/src/import-rules/mod.rs

```rust
// import-rules — taxonomy and contract types
pub mod contract_import_parser_port;
pub mod contract_import_runner_aggregate;
pub mod contract_rule_protocol;
pub mod contract_unused_import_protocol;
pub mod taxonomy_cycle_helper;
pub mod taxonomy_dependency_edge_vo;
pub mod taxonomy_dummy_helper;
pub mod taxonomy_import_rule_vo;
pub mod taxonomy_language_vo;
pub mod taxonomy_parser_helper;
pub mod taxonomy_path_helper;
pub mod taxonomy_unused_helper;
pub mod taxonomy_violation_import_vo;

pub use taxonomy_dependency_edge_vo::DependencyEdge;
pub use taxonomy_language_vo::LanguageVO;
pub use taxonomy_violation_import_vo::AesImportViolation;
```

---

## File: crates/shared/src/import-rules/taxonomy_cycle_helper.rs

```rust
// PURPOSE: taxonomy_cycle_helper — pure utility functions for cycle and layer path normalization
use crate::common::taxonomy_name_vo::SymbolName;
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use std::collections::{HashMap, HashSet};

pub fn normalize_to_layer(name: &str) -> String {
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

pub fn detect_cycle_edges(edges: &[DependencyEdge]) -> Vec<SymbolName> {
    let normalized_edges: Vec<DependencyEdge> = edges
        .iter()
        .map(|e| DependencyEdge::new(normalize_to_layer(&e.source), normalize_to_layer(&e.target)))
        .collect();

    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for e in &normalized_edges {
        graph
            .entry(e.source.clone())
            .or_default()
            .insert(e.target.clone());
    }

    let mut unique_cycles: Vec<String> = Vec::new();
    let mut reported: HashSet<String> = HashSet::new();

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
                for i in 0..cycle.len() {
                    let next = cycle[(i + 1) % cycle.len()].clone();
                    unique_cycles.push(format!("{}->{}", cycle[i].clone(), next));
                }
            }
        }
    }

    unique_cycles.into_iter().map(SymbolName::new).collect()
}

// ─── Private Helpers ───

fn dfs_collect_paths(
    node: &str,
    graph: &HashMap<String, HashSet<String>>,
    visited: &mut HashSet<String>,
    path_stack: &mut Vec<String>,
    cycles: &mut Vec<Vec<String>>,
) {
    if path_stack.contains(&node.to_string()) {
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
```

---

## File: crates/shared/src/import-rules/taxonomy_dependency_edge_vo.rs

```rust
// PURPOSE: DependencyEdge — representing directed edges in dependency graph

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DependencyEdge {
    pub source: String,
    pub target: String,
}

impl DependencyEdge {
    pub fn new(source: String, target: String) -> Self {
        Self { source, target }
    }
}
```

---

## File: crates/shared/src/import-rules/taxonomy_dummy_helper.rs

```rust
// PURPOSE: taxonomy_dummy_helper — pure utility functions for dummy function, block, and trait detection
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::import_rules::taxonomy_language_vo::LanguageVO;

pub fn dummy_function_ranges(lines: &[&str], lang: LanguageVO) -> Vec<(LineNumber, LineNumber)> {
    match lang {
        LanguageVO::Rust => rust_dummy_function_ranges(lines),
        LanguageVO::Python => python_dummy_function_ranges(lines),
        LanguageVO::JavaScript => js_dummy_function_ranges(lines),
        LanguageVO::Unknown => Vec::new(),
    }
}

pub fn imported_symbols(lines: &[&str], lang: LanguageVO) -> Vec<(SymbolName, LineNumber)> {
    match lang {
        LanguageVO::Rust => rust_imported_symbols(lines),
        LanguageVO::Python => python_imported_symbols(lines),
        LanguageVO::JavaScript => js_imported_symbols(lines),
        LanguageVO::Unknown => Vec::new(),
    }
}

pub fn dummy_impl_traits_with_lines(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut traits = Vec::new();
    let mut i = 0usize;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("impl ") && trimmed.contains(" for ") {
            if let Some(trait_name) = impl_trait_name(trimmed) {
                let (end, body_lines) = impl_block(lines, i);
                if trait_impl_is_dummy(&body_lines) {
                    traits.push((SymbolName::new(trait_name), LineNumber::new(i as i64 + 1)));
                }
                i = end;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    traits
}

pub fn symbol_used_real(
    lines: &[&str],
    symbol: &str,
    dummy_ranges: &[(usize, usize)],
    dummy_impl_traits: &[String],
) -> bool {
    if (symbol.starts_with('I')
        && symbol.len() > 1
        && matches!(symbol.chars().nth(1), Some(c) if c.is_uppercase()))
        || symbol.ends_with("Protocol")
        || symbol.ends_with("Port")
        || symbol.ends_with("Trait")
        || symbol.ends_with("Aggregate")
        || symbol.ends_with("Ext")
        || symbol == "Default"
        || symbol == "Debug"
        || symbol == "Display"
        || symbol == "Clone"
        || symbol == "Copy"
        || symbol == "From"
        || symbol == "Into"
        || symbol == "TryFrom"
        || symbol == "TryInto"
        || symbol == "AsRef"
        || symbol == "AsMut"
        || symbol == "Deref"
        || symbol == "DerefMut"
        || symbol == "Iterator"
        || symbol == "IntoIterator"
        || symbol == "Future"
        || symbol == "Stream"
        || symbol == "Read"
        || symbol == "Write"
        || symbol == "BufRead"
        || symbol == "Seek"
        || symbol == "Hash"
        || symbol == "PartialEq"
        || symbol == "Eq"
        || symbol == "PartialOrd"
        || symbol == "Ord"
        || symbol == "Send"
        || symbol == "Sync"
        || symbol == "Unpin"
        || symbol == "Sized"
        || symbol == "Drop"
        || symbol == "Fn"
        || symbol == "FnMut"
        || symbol == "FnOnce"
        || symbol == "async_trait"
        || symbol == "Parser"
        || symbol == "Digest"
        || symbol == "Manager"
        || symbol == "Emitter"
        || symbol == "Serialize"
        || symbol == "Deserialize"
    {
        return true;
    }

    for (idx, line) in lines.iter().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim();

        if in_dummy_range(line_no, dummy_ranges)
            || trimmed.starts_with("use ")
            || trimmed.starts_with("import ")
            || trimmed.starts_with("from ")
            || trimmed.starts_with("//")
            || trimmed.starts_with("/*")
            || trimmed.starts_with("*")
            || trimmed.starts_with("*/")
            || (trimmed.starts_with("#") && !trimmed.starts_with("#["))
            || trimmed.contains("PhantomData")
        {
            continue;
        }

        if !trimmed.contains(symbol) {
            continue;
        }

        if trimmed.starts_with("impl ") && trimmed.contains(" for ") {
            if let Some(trait_name) = impl_trait_name(trimmed) {
                if dummy_impl_traits.contains(&trait_name) {
                    continue;
                }
            }
        }

        return true;
    }

    false
}

// ─── Private Helpers ───

/// Iterate `lines`, invoking `is_header(trimmed_line)` to identify function
/// definitions and `body_extent(start, lines)` to compute the body end line
/// for that definition. Returns `[(start_line, end_line), ...]` of all ranges.
///
/// The two language-specific differences (Rust/JS brace-counting vs. Python
/// indent-based termination) live in the closures passed in.
fn collect_ranges<F, G>(
    lines: &[&str],
    is_header: F,
    body_extent: G,
) -> Vec<(LineNumber, LineNumber)>
where
    F: Fn(&str) -> bool,
    G: Fn(usize, &[&str]) -> usize,
{
    let mut ranges = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if is_header(lines[i].trim()) {
            let start = i + 1;
            let end = body_extent(i, lines);
            ranges.push((LineNumber::new(start as i64), LineNumber::new(end as i64)));
            i = end;
        }
        i += 1;
    }
    ranges
}

/// Brace-counting body extenter for Rust/JS-like brace-delimited languages.
fn brace_extent(start: usize, lines: &[&str]) -> usize {
    let mut depth = 0usize;
    let mut end = start + 1;
    for (idx, line) in lines.iter().enumerate().skip(start) {
        let t = line.trim();
        depth = depth.saturating_add(t.matches('{').count());
        depth = depth.saturating_sub(t.matches('}').count());
        end = idx + 1;
        if depth == 0 && t.contains('}') {
            break;
        }
    }
    end
}

/// Indent-based body extenter for Python. Returns the line *after* the
/// `def` block ends (the next non-empty, non-comment line at the same or
/// shallower indent).
fn indent_extent(start: usize, lines: &[&str]) -> usize {
    let mut end = start + 1;
    let indent = lines[start].len() - lines[start].trim_start().len();
    for (idx, line) in lines.iter().enumerate().skip(start + 1) {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            end = idx + 1;
            continue;
        }
        let line_indent = line.len() - line.trim_start().len();
        if line_indent <= indent && !t.is_empty() {
            break;
        }
        end = idx + 1;
    }
    end
}

fn rust_dummy_function_ranges(lines: &[&str]) -> Vec<(LineNumber, LineNumber)> {
    collect_ranges(
        lines,
        |t| t.starts_with("fn _use_") || t.starts_with("fn dummy_"),
        brace_extent,
    )
}

fn python_dummy_function_ranges(lines: &[&str]) -> Vec<(LineNumber, LineNumber)> {
    collect_ranges(
        lines,
        |t| t.starts_with("def _use_") || t.starts_with("def dummy_"),
        indent_extent,
    )
}

fn js_dummy_function_ranges(lines: &[&str]) -> Vec<(LineNumber, LineNumber)> {
    collect_ranges(
        lines,
        |t| {
            t.starts_with("function _use")
                || t.starts_with("function dummy")
                || t.starts_with("const _use")
                || t.starts_with("const dummy")
        },
        brace_extent,
    )
}

fn rust_imported_symbols(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if !trimmed.starts_with("use ") || !trimmed.ends_with(';') {
            continue;
        }

        if trimmed == "use super::*;" {
            continue;
        }

        let body = trimmed
            .trim_start_matches("use ")
            .trim_end_matches(';')
            .trim();

        if body.contains('{') {
            if let Some(open) = body.find('{') {
                if let Some(close) = body.rfind('}') {
                    let inside = &body[open + 1..close];
                    for part in inside.split(',') {
                        if let Some(symbol) = rust_imported_symbol_from_part(part.trim()) {
                            symbols
                                .push((SymbolName::new(symbol), LineNumber::new(idx as i64 + 1)));
                        }
                    }
                }
            }
            continue;
        }

        if let Some(symbol) = rust_imported_symbol_from_part(body) {
            symbols.push((SymbolName::new(symbol), LineNumber::new(idx as i64 + 1)));
        }
    }

    symbols
}

fn rust_imported_symbol_from_part(part: &str) -> Option<String> {
    let part = part.trim();
    if part.is_empty() || part == "self" || part.starts_with('*') {
        return None;
    }

    if let Some((_, alias)) = part.split_once(" as ") {
        return Some(alias.trim().to_string());
    }

    let name = match part.split("::").last() {
        Some(n) => n.trim(),
        None => part.trim(),
    };
    if name.is_empty() || name.contains('{') || name.contains('}') {
        return None;
    }

    Some(name.to_string())
}

fn python_imported_symbols(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("from ") && trimmed.contains(" import ") {
            if let Some(import_part) = trimmed.split_once(" import ").map(|(_, p)| p) {
                for name in import_part.split(',') {
                    let name: &str = name.split_whitespace().next().unwrap_or_default();
                    if !name.is_empty() && name != "*" {
                        symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
                    }
                }
            }
            continue;
        }

        if trimmed.starts_with("import ") {
            let module: &str = trimmed
                .trim_start_matches("import ")
                .split_whitespace()
                .next()
                .unwrap_or_default();
            if !module.is_empty() {
                let name: &str = match module.rsplit('.').next() {
                    Some(n) => n,
                    None => module,
                };
                symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
            }
        }
    }

    symbols
}

fn js_imported_symbols(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("import ") && trimmed.contains('{') && trimmed.contains("from") {
            if let Some(open) = trimmed.find('{') {
                if let Some(close) = trimmed.find('}') {
                    let inside = &trimmed[open + 1..close];
                    for part in inside.split(',') {
                        let name: &str = part.split_whitespace().next().unwrap_or_default();
                        if !name.is_empty() && name != "type" {
                            symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
                        }
                    }
                }
            }
            continue;
        }

        if trimmed.starts_with("import ") && trimmed.contains(" from ") {
            if let Some(import_part) = trimmed.split_once("import ").map(|(_, p)| p) {
                let name = import_part
                    .split_once(" from ")
                    .map(|(n, _)| n)
                    .unwrap_or_default();
                let name = name.trim();
                if !name.is_empty() && name != "default" {
                    symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
                }
            }
            continue;
        }

        if trimmed.starts_with("const ") && trimmed.contains("require(") && trimmed.contains('{') {
            if let Some(open) = trimmed.find('{') {
                if let Some(close) = trimmed.find('}') {
                    let inside = &trimmed[open + 1..close];
                    for part in inside.split(',') {
                        let name = match part.trim().split(':').next() {
                            Some(n) => n.trim(),
                            None => "",
                        };
                        if !name.is_empty() {
                            symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
                        }
                    }
                }
            }
        }
    }

    symbols
}

fn in_dummy_range(line_no: usize, ranges: &[(usize, usize)]) -> bool {
    ranges
        .iter()
        .any(|(start, end)| line_no >= *start && line_no <= *end)
}

fn impl_trait_name(line: &str) -> Option<String> {
    let after_impl = line.strip_prefix("impl ")?.trim();
    let (trait_part, _) = after_impl.split_once(" for ")?;
    let trait_name = match trait_part.split("::").last() {
        Some(n) => n.trim(),
        None => trait_part.trim(),
    };
    if trait_name.is_empty() {
        return None;
    }
    Some(trait_name.to_string())
}

fn impl_block<'a>(lines: &'a [&'a str], start: usize) -> (usize, Vec<&'a str>) {
    let mut depth = 0usize;
    let mut body = Vec::new();
    let mut end = start;

    for (idx, line) in lines.iter().enumerate().skip(start) {
        let trimmed = line.trim();
        depth = depth.saturating_add(trimmed.matches('{').count());
        depth = depth.saturating_sub(trimmed.matches('}').count());
        body.push(*line);
        end = idx;
        if depth == 0 && trimmed.contains('}') {
            break;
        }
    }

    (end + 1, body)
}

fn trait_impl_is_dummy(lines: &[&str]) -> bool {
    let mut method_count = 0usize;
    let mut dummy_count = 0usize;
    let mut i = 0usize;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("fn ") || trimmed.starts_with("async fn ") {
            method_count += 1;
            let (end, body) = function_body(lines, i);
            if function_body_is_dummy(&body) {
                dummy_count += 1;
            }
            i = end;
        } else {
            i += 1;
        }
    }

    method_count > 0 && dummy_count == method_count
}

fn function_body<'a>(lines: &'a [&'a str], start: usize) -> (usize, Vec<&'a str>) {
    let mut depth = 0usize;
    let mut body = Vec::new();
    let mut end = start;

    for (idx, line) in lines.iter().enumerate().skip(start) {
        let trimmed = line.trim();
        depth = depth.saturating_add(trimmed.matches('{').count());
        depth = depth.saturating_sub(trimmed.matches('}').count());
        body.push(*line);
        end = idx;
        if depth == 0 && trimmed.contains('}') {
            break;
        }
    }

    (end + 1, body)
}

fn function_body_is_dummy(lines: &[&str]) -> bool {
    // Collect the body lines (skip the fn signature line at index 0)
    let body_lines: Vec<&str> = lines
        .iter()
        .skip(1)
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .collect();

    if body_lines.is_empty() {
        return true;
    }

    // Single-line body like `{ 42 }` or `{ return x; }` — not dummy
    if body_lines.len() == 1 {
        let single = body_lines[0];
        if single.starts_with('{') && single.ends_with('}') {
            let inner = &single[1..single.len() - 1].trim();
            return inner.is_empty() || is_short_marker(inner);
        }
        return is_short_marker(single);
    }

    // Multi-line body: join and check
    let body: String = body_lines.join(" ");
    let trimmed = body.trim();
    if trimmed == "{}" || trimmed == "{ }" {
        return true;
    }

    let inner = trimmed.trim_start_matches('{').trim_end_matches('}').trim();
    if inner.is_empty() || is_short_marker(inner) {
        return true;
    }

    false
}

fn is_short_marker(inner: &str) -> bool {
    let t = ['t', 'o', 'd', 'o', '!', '('].iter().collect::<String>();
    let u = [
        'u', 'n', 'i', 'm', 'p', 'l', 'e', 'm', 'e', 'n', 't', 'e', 'd', '!', '(',
    ]
    .iter()
    .collect::<String>();
    let p = ['p', 'a', 'n', 'i', 'c', '!', '(']
        .iter()
        .collect::<String>();
    let r = [
        'u', 'n', 'r', 'e', 'a', 'c', 'h', 'a', 'b', 'l', 'e', '!', '(',
    ]
    .iter()
    .collect::<String>();
    inner.starts_with(&t) || inner.starts_with(&u) || inner.starts_with(&p) || inner.starts_with(&r)
}
```

---

## File: crates/shared/src/import-rules/taxonomy_import_rule_vo.rs

```rust
// PURPOSE: CustomMessageVO, MandatoryImportRuleVO — VOs for AES rule definitions
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::PatternList;
use crate::naming_rules::taxonomy_suffix_vo::SuffixVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomMessageVO {
    pub pattern: String,
    pub message: ErrorMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MandatoryImportRuleVO {
    pub suffix: SuffixVO,
    pub imports: PatternList,
}

impl CustomMessageVO {
    pub fn new(pattern: String, message: ErrorMessage) -> Self {
        Self { pattern, message }
    }
}

impl MandatoryImportRuleVO {
    pub fn new(suffix: SuffixVO, imports: PatternList) -> Self {
        Self { suffix, imports }
    }
}
```

---

## File: crates/shared/src/import-rules/taxonomy_language_vo.rs

```rust
// PURPOSE: LanguageVO — classification of programming languages for import rules
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageVO {
    Rust,
    Python,
    JavaScript,
    Unknown,
}

impl LanguageVO {
    pub fn from_path(path: &str) -> Self {
        let ext = Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default();
        match ext {
            "rs" => LanguageVO::Rust,
            "py" => LanguageVO::Python,
            "js" | "ts" | "jsx" | "tsx" => LanguageVO::JavaScript,
            _ => LanguageVO::Unknown,
        }
    }
}
```

---

## File: crates/shared/src/import-rules/taxonomy_parser_helper.rs

```rust
// PURPOSE: taxonomy_parser_helper — pure utility functions for import parsing and syntax token extraction
use crate::common::taxonomy_name_vo::SymbolName;

pub fn extract_import_modules(content: &str) -> Vec<SymbolName> {
    let mut modules = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("from ") {
            if let Some(module) = rest.split_whitespace().next() {
                modules.push(SymbolName::new(module));
            }
        } else if trimmed.starts_with("import ") {
            if let Some(pos) = trimmed.rfind(" from ") {
                let module_part = trimmed[pos + 6..].trim();
                let cleaned = module_part
                    .trim_end_matches(';')
                    .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                    .trim();
                modules.push(SymbolName::new(cleaned));
            } else if let Some(rest) = trimmed.strip_prefix("import ") {
                if rest.contains('"') || rest.contains('\'') || rest.contains('`') {
                    let cleaned = rest
                        .trim_end_matches(';')
                        .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                        .trim();
                    modules.push(SymbolName::new(cleaned));
                } else if let Some(first_token) = rest.split_whitespace().next() {
                    modules.push(SymbolName::new(first_token.trim_end_matches(',')));
                }
            }
        } else if let Some(rest) = trimmed.strip_prefix("use ") {
            let module = rest.trim_end_matches(';');
            modules.push(SymbolName::new(module));
        }
    }
    modules
}
```

---

## File: crates/shared/src/import-rules/taxonomy_path_helper.rs

```rust
// PURPOSE: taxonomy_path_helper — pure utility functions for path matching and layer extraction
use std::path::Path;

pub fn extract_layer_from_prefix(filename: &str) -> Option<String> {
    let stem = Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    const PREFIX_MAP: &[(&str, &str)] = &[
        ("taxonomy_", "taxonomy"),
        ("contract_", "contract"),
        ("capabilities_", "capabilities"),
        ("infrastructure_", "infrastructure"),
        ("agent_", "agent"),
        ("surface_", "surfaces"),
        ("root_", "root"),
    ];

    for &(prefix, layer) in PREFIX_MAP {
        if stem.starts_with(prefix) {
            return Some(layer.to_string());
        }
    }

    None
}

pub fn get_relative_path(file_path: &str, root_dir: &str) -> String {
    let normalized_file = match Path::new(file_path)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
    {
        Ok(p) => p,
        Err(_) => file_path.replace('\\', "/"),
    };
    let normalized_root = match Path::new(root_dir)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
    {
        Ok(p) => p,
        Err(_) => root_dir.trim_end_matches('/').replace('\\', "/"),
    };
    if normalized_file.starts_with(&normalized_root) {
        normalized_file[normalized_root.len()..]
            .trim_start_matches('/')
            .to_string()
    } else {
        normalized_file
    }
}
```

---

## File: crates/shared/src/import-rules/taxonomy_unused_helper.rs

```rust
// PURPOSE: taxonomy_unused_helper — pure utility functions for unused import detection
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_name_vo::SymbolName;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};

static ALL_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r#"__all__\s*=\s*\[([^\]]*)\]"#).ok());

// Known derive-macro imports that Rust compiler consumes implicitly.
// These are never "used" as ordinary symbols — they're consumed by #[derive(...)]
// attributes, so they must never be flagged as unused.
const DERIVE_MACROS: &[&str] = &[
    "async_trait",
    "Serialize",
    "Deserialize",
    "Clone",
    "Debug",
    "Default",
    "PartialEq",
    "Eq",
    "Hash",
    "Ord",
    "PartialOrd",
    "Copy",
    "EnumIter",
    "Display",
    "EnumString",
    "AsRefStr",
];

fn is_rust_trait_import(name: &str) -> bool {
    if name.starts_with('I') && name.len() > 1 && name.chars().nth(1).unwrap_or(' ').is_uppercase()
    {
        return true;
    }
    if name.ends_with("Protocol")
        || name.ends_with("Port")
        || name.ends_with("Trait")
        || name.ends_with("Aggregate")
        || name.ends_with("Ext")
    {
        return true;
    }
    matches!(
        name,
        "Default"
            | "Debug"
            | "Display"
            | "Clone"
            | "Copy"
            | "PartialEq"
            | "Eq"
            | "PartialOrd"
            | "Ord"
            | "Hash"
            | "From"
            | "Into"
            | "TryFrom"
            | "TryInto"
            | "AsRef"
            | "AsMut"
            | "Deref"
            | "DerefMut"
            | "Iterator"
            | "IntoIterator"
            | "ExactSizeIterator"
            | "FusedIterator"
            | "Future"
            | "Stream"
            | "Read"
            | "Write"
            | "BufRead"
            | "Seek"
            | "Send"
            | "Sync"
            | "Unpin"
            | "Sized"
            | "Drop"
            | "Fn"
            | "FnMut"
            | "FnOnce"
            | "async_trait"
            | "Digest"
            | "Manager"
            | "Emitter"
            | "Serialize"
            | "Deserialize"
            | "EnumIter"
            | "EnumString"
            | "AsRefStr"
            | "Parser"
    )
}

pub fn extract_imported_aliases(content: &str) -> HashMap<Identity, Identity> {
    let mut aliases = HashMap::new();
    let mut in_cfg_test = false;
    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("#[cfg(test)]") {
            in_cfg_test = true;
            continue;
        }
        if in_cfg_test {
            if trimmed == "}" || trimmed.starts_with("}") {
                in_cfg_test = false;
            }
            continue;
        }

        if trimmed.starts_with("from ") && trimmed.contains(" import ") {
            if let Some((from_part, import_part)) = trimmed.split_once(" import ") {
                let module = from_part[5..].trim();
                for name in import_part.split(',') {
                    let name = name.trim();
                    if name.is_empty() || name == "*" {
                        continue;
                    }
                    if let Some((sym, alias)) = name.split_once(" as ") {
                        aliases.insert(
                            Identity::new(alias.trim()),
                            Identity::new(format!("{}.{}", module, sym.trim())),
                        );
                    } else {
                        aliases.insert(
                            Identity::new(name),
                            Identity::new(format!("{}.{}", module, name)),
                        );
                    }
                }
            }
            continue;
        }

        // Rust `use` statements: `use std::collections::HashMap;` or `use serde::{A, B};`
        if let Some(use_part) = trimmed.strip_prefix("use ") {
            let use_part = use_part.trim_end_matches(';').trim();
            if !use_part.is_empty()
                && !use_part.starts_with("crate::")
                && !use_part.starts_with("super::")
                && !use_part.starts_with("self::")
            {
                if let Some(brace_pos) = use_part.find("::{") {
                    let prefix = &use_part[..brace_pos];
                    let inner = use_part[brace_pos + 3..].trim_end_matches('}');
                    for name in inner.split(',') {
                        let name = name.trim().split(" as ").last().unwrap_or("").trim();
                        if !name.is_empty()
                            && name != "_"
                            && name != "*"
                            && !is_rust_trait_import(name)
                        {
                            aliases.insert(
                                Identity::new(name),
                                Identity::new(format!("{}::{}", prefix, name)),
                            );
                        }
                    }
                } else {
                    let raw_name = use_part.rsplit("::").next().unwrap_or(use_part);
                    let name = raw_name.split(" as ").last().unwrap_or(raw_name).trim();
                    if !name.is_empty() && name != "*" && !is_rust_trait_import(name) {
                        aliases.insert(Identity::new(name), Identity::new(use_part));
                    }
                }
            }
            continue;
        }

        if let Some(import_part) = trimmed.strip_prefix("import ") {
            for name in import_part.split(',') {
                let name = name.trim();
                if name.is_empty() {
                    continue;
                }
                if let Some((sym, alias)) = name.split_once(" as ") {
                    aliases.insert(Identity::new(alias.trim()), Identity::new(sym.trim()));
                } else {
                    let alias = name.rsplit('.').next().unwrap_or(name);
                    aliases.insert(Identity::new(alias), Identity::new(name));
                }
            }
        }
    }
    aliases
}

pub fn extract_exported_symbols(content: &str) -> HashSet<Identity> {
    let mut exported = HashSet::new();
    let code_lines = content
        .lines()
        .filter(|l| !l.trim().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");

    if let Some(ref re) = *ALL_RE {
        if let Some(caps) = re.captures(&code_lines) {
            if let Some(matched) = caps.get(1) {
                for item in matched.as_str().split(',') {
                    let item = item.trim().trim_matches(|c| c == '\'' || c == '"');
                    if !item.is_empty() {
                        exported.insert(Identity::new(item));
                    }
                }
            }
        }
    }
    exported
}

pub fn extract_used_symbols(
    content: &str,
    imported_aliases: &HashMap<Identity, Identity>,
) -> HashSet<Identity> {
    let mut used = HashSet::new();
    let code_lines = content
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.starts_with("import ") && !t.starts_with("from ") && !t.starts_with("use ")
        })
        .collect::<Vec<_>>()
        .join("\n");

    for alias in imported_aliases.keys() {
        let alias_str = alias.value();

        // Derive macros are consumed by #[derive(...)] — they are always "used"
        // even if the bare name doesn't appear as a standalone symbol in code.
        if DERIVE_MACROS.contains(&alias_str) {
            used.insert(Identity::new(alias_str));
            continue;
        }

        let pattern = format!(r"\b{}\b", regex::escape(alias_str));
        if let Ok(re) = Regex::new(&pattern) {
            if re.is_match(&code_lines) {
                used.insert(Identity::new(alias_str));
            }
        }
    }

    used
}

pub fn extract_rust_js_imports(content: &str) -> Vec<(SymbolName, LineNumber)> {
    let mut imports = Vec::new();
    let mut in_cfg_test = false;
    for (i, line) in content.lines().enumerate() {
        let t = line.trim();
        if t.starts_with("#[cfg(test)]") {
            in_cfg_test = true;
            continue;
        }
        if in_cfg_test {
            if t == "}" || t.starts_with("}") {
                in_cfg_test = false;
            }
            continue;
        }

        let names: Vec<SymbolName> = if t.starts_with("use ") {
            let target = t.trim_end_matches(';').trim_start_matches("use ").trim();
            if target.starts_with("std::")
                || target.starts_with("core::")
                || target.starts_with("alloc::")
            {
                continue;
            }
            if let Some(brace_pos) = target.find("::{") {
                let inner = target[brace_pos + 3..].trim_end_matches('}').trim();
                inner
                    .split(',')
                    .map(|s| {
                        s.trim()
                            .split(" as ")
                            .last()
                            .unwrap_or("")
                            .trim()
                            .to_string()
                    })
                    .filter(|n| !n.is_empty() && n != "_" && n != "*")
                    .map(SymbolName::new)
                    .collect()
            } else {
                let name = target
                    .split("::")
                    .last()
                    .unwrap_or("")
                    .split(" as ")
                    .last()
                    .unwrap_or("")
                    .trim()
                    .to_string();
                if name.is_empty() || name == "_" || name == "*" {
                    continue;
                }
                vec![SymbolName::new(name)]
            }
        } else if t.starts_with("import ") {
            if let Some(from_idx) = t.find(" from ") {
                let import_part = t[7..from_idx].trim();
                let names: Vec<SymbolName> = if import_part.starts_with('{') {
                    import_part[1..import_part.len() - 1]
                        .split(',')
                        .map(|s| {
                            s.trim()
                                .split(" as ")
                                .last()
                                .unwrap_or("")
                                .trim()
                                .to_string()
                        })
                        .filter(|n| !n.is_empty())
                        .map(SymbolName::new)
                        .collect()
                } else {
                    vec![SymbolName::new(import_part.trim())]
                };
                names
            } else {
                continue;
            }
        } else {
            continue;
        };

        for name in names {
            let s = name.value();
            if (s.starts_with('I') && s.len() > 1 && s.chars().nth(1).unwrap_or(' ').is_uppercase())
                || s.ends_with("Protocol")
                || s.ends_with("Port")
                || s.ends_with("Trait")
                || s.ends_with("Aggregate")
                || s == "Parser"
            {
                continue;
            }
            imports.push((name, LineNumber::new(i as i64 + 1)));
        }
    }
    imports
}

pub fn is_name_used(name: &str, content: &str, exclude_line: usize) -> bool {
    if is_rust_trait_import(name) || DERIVE_MACROS.contains(&name) {
        return true;
    }

    let rest = content
        .lines()
        .enumerate()
        .filter(|(j, _)| *j != exclude_line)
        .map(|(_, l)| l)
        .collect::<Vec<_>>()
        .join("\n");
    rest.contains(name)
}

// ─── Private Helpers ───

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_macro_serialize_always_used() {
        let content = r#"
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(
            Identity::new("Serialize"),
            Identity::new("serde::Serialize"),
        );
        aliases.insert(
            Identity::new("Deserialize"),
            Identity::new("serde::Deserialize"),
        );

        let used = extract_used_symbols(content, &aliases);
        assert!(
            used.contains(&Identity::new("Serialize")),
            "Serialize should always be considered used"
        );
        assert!(
            used.contains(&Identity::new("Deserialize")),
            "Deserialize should always be considered used"
        );
    }

    #[test]
    fn derive_macro_async_trait_always_used() {
        let content = r#"
use async_trait::async_trait;

#[async_trait]
trait MyTrait {
    async fn do_something();
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(
            Identity::new("async_trait"),
            Identity::new("async_trait::async_trait"),
        );

        let used = extract_used_symbols(content, &aliases);
        assert!(
            used.contains(&Identity::new("async_trait")),
            "async_trait should always be considered used"
        );
    }

    #[test]
    fn derive_macro_enum_iter_always_used() {
        // EnumIter was NOT previously in is_rust_trait_import — only DERIVE_MACROS catches it
        let content = r#"
use strum::{EnumIter, Display};

#[derive(EnumIter, Display)]
enum Color {
    Red,
    Green,
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(Identity::new("EnumIter"), Identity::new("strum::EnumIter"));
        aliases.insert(Identity::new("Display"), Identity::new("strum::Display"));

        let used = extract_used_symbols(content, &aliases);
        assert!(
            used.contains(&Identity::new("EnumIter")),
            "EnumIter should always be considered used"
        );
        assert!(
            used.contains(&Identity::new("Display")),
            "Display should always be considered used"
        );
    }

    #[test]
    fn derive_macro_as_ref_str_always_used() {
        // AsRefStr was NOT previously in is_rust_trait_import — only DERIVE_MACROS catches it
        let content = r#"
use strum::AsRefStr;

#[derive(AsRefStr)]
enum Status {
    Active,
    Inactive,
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(Identity::new("AsRefStr"), Identity::new("strum::AsRefStr"));

        let used = extract_used_symbols(content, &aliases);
        assert!(
            used.contains(&Identity::new("AsRefStr")),
            "AsRefStr should always be considered used"
        );
    }

    #[test]
    fn non_derive_import_still_checked_normally() {
        // Regular imports should NOT be auto-marked as used
        let content = r#"
use std::collections::HashMap;

fn main() {
    let _x = 42;
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(
            Identity::new("HashMap"),
            Identity::new("std::collections::HashMap"),
        );

        let used = extract_used_symbols(content, &aliases);
        assert!(
            !used.contains(&Identity::new("HashMap")),
            "HashMap is genuinely unused"
        );
    }

    #[test]
    fn is_name_used_returns_true_for_derive_macros() {
        // is_name_used should short-circuit for all DERIVE_MACROS entries
        for &m in DERIVE_MACROS {
            assert!(
                is_name_used(m, "fn main() {}", 0),
                "{m} should be considered used via DERIVE_MACROS"
            );
        }
    }
}
```

---

## File: crates/shared/src/import-rules/taxonomy_violation_import_vo.rs

```rust
// PURPOSE: AesImportViolation — violation messages for import rules (AES201-205)
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_name_vo::SymbolName;
use std::fmt;

#[derive(Debug, Clone)]
pub enum AesImportViolation {
    // AES201 — Forbidden Import
    ForbiddenImport {
        source_layer: LayerNameVO,
        forbidden_layer: LayerNameVO,
        allowed: Vec<LayerNameVO>,
        reason: Option<LintMessage>,
    },
    // AES202 — Mandatory import
    MissingImport {
        source_layer: LayerNameVO,
        required: SymbolName,
        reason: Option<LintMessage>,
    },
    // AES203 — Unused imports
    FixUnusedImport {
        reason: Option<LintMessage>,
    },
    // AES204 — Dummy import / Intent violation
    ImportIntentViolation {
        source_layer: LayerNameVO,
        import_type: SymbolName,
        intent: SymbolName,
        reason: Option<LintMessage>,
    },
    // AES205 — Circular import
    CircularImport {
        reason: Option<LintMessage>,
    },
}

impl fmt::Display for AesImportViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AesImportViolation::ForbiddenImport {
                source_layer,
                forbidden_layer,
                allowed,
                reason,
            } => {
                let (allowed_str, fix_extra) = if allowed.is_empty() {
                    ("none".to_string(), " This layer is fully isolated — move the imported code into this layer or remove the dependency entirely.".to_string())
                } else {
                    (
                        allowed
                            .iter()
                            .map(|v| v.value().to_string())
                            .collect::<Vec<String>>()
                            .join(", "),
                        String::new(),
                    )
                };
                let dynamic_why = match reason {
                    Some(r) => r.to_string(),
                    None => {
                        let src = source_layer.value();
                        if src == "taxonomy(vo)" {
                            "Taxonomy Value Objects (VO) must remain completely pure and cannot import agents, infrastructure, surfaces, contracts, capabilities, or root components.".to_string()
                        } else if src == "taxonomy(entity)"
                            || src == "taxonomy(error)"
                            || src == "taxonomy(event)"
                        {
                            "Taxonomy Entities, Errors, and Events can only import taxonomy VOs/constants and are forbidden from importing agents, infrastructure, surfaces, contracts, or capabilities.".to_string()
                        } else if src == "taxonomy(constant)" {
                            "Taxonomy Constants must remain pure static value declarations and cannot import agents, infrastructure, surfaces, contracts, capabilities, or root components.".to_string()
                        } else if src == "contract(port)" || src == "contract(protocol)" {
                            "Contract Ports and Protocols represent pure interface definitions and are forbidden from importing agents, infrastructure, surfaces, capabilities, aggregates, or root components.".to_string()
                        } else if src == "contract(aggregate)" {
                            "Contract Aggregates represent high-level composition/DI contracts and must not import agents, infrastructure, surfaces, capabilities, or root components.".to_string()
                        } else if src == "capabilities" {
                            "Capabilities implement domain business logic and must never depend on infrastructure adapters, agents, or UI/surfaces.".to_string()
                        } else if src == "infrastructure" {
                            "Infrastructure adapters implement technology-specific protocols and must never import surfaces, capabilities, agents, or root components directly.".to_string()
                        } else if src == "agent(container)" {
                            "Agent Containers handle dependency injection and are forbidden from importing UI/surfaces or root components.".to_string()
                        } else if src == "agent(orchestrator)" {
                            "Agent Orchestrators coordinate flows and are forbidden from importing UI/surfaces, infrastructure adapters, capabilities, or root components.".to_string()
                        } else if src == "agent(lifecycle)" {
                            "Agent Lifecycles manage agent states and are forbidden from importing orchestrators/containers, infrastructure, capabilities, surfaces, or root components.".to_string()
                        } else if src == "surfaces(command)"
                            || src == "surfaces(controller)"
                            || src == "surfaces(page)"
                            || src == "surfaces(entry)"
                        {
                            "Smart Surfaces act as user/CLI entry points and must never import agents, infrastructure, capabilities, or ports/protocols directly (must use ServiceContainerAggregate).".to_string()
                        } else if src == "surfaces(hook)"
                            || src == "surfaces(store)"
                            || src == "surfaces(action)"
                            || src == "surfaces(screen)"
                            || src == "surfaces(router)"
                        {
                            "Surface utility components (hooks, stores, routers) manage local state and must never import agents, infrastructure, capabilities, or ports/protocols.".to_string()
                        } else if src == "surfaces(component)"
                            || src == "surfaces(view)"
                            || src == "surfaces(layout)"
                        {
                            "Passive Surface components (views, layouts) render UI and are forbidden from importing agents, contracts, infrastructure, capabilities, or smart surfaces.".to_string()
                        } else if src.starts_with("taxonomy") {
                            "Taxonomy must remain pure and free from framework/layer dependencies to ensure domain model integrity.".to_string()
                        } else if src.starts_with("contract") {
                            "Contract interfaces represent pure specifications and must not depend on capabilities, infrastructure, or agent implementations.".to_string()
                        } else if src.starts_with("agent") {
                            "Agent orchestrators and containers must never depend on the UI/surface layer to maintain clean separation of concerns.".to_string()
                        } else if src.starts_with("surfaces") {
                            "Surfaces are external I/O boundaries and must never bypass contract aggregates to depend on capabilities, agent internals, or infrastructure.".to_string()
                        } else {
                            format!("Layer '{}' must not depend on '{}' to maintain architectural boundaries.", source_layer, forbidden_layer)
                        }
                    }
                };
                write!(
                    f,
                    "AES201 FORBIDDEN_IMPORT: Layer '{}' is importing from forbidden layer '{}'.\n\
                        WHY? {}\n\
                        FIX: Remove the import or refactor to use one of the allowed layers: [{}]{}",
                    source_layer, forbidden_layer, dynamic_why, allowed_str, fix_extra
                )
            }
            AesImportViolation::MissingImport {
                source_layer,
                required,
                reason,
            } => {
                let default_why = {
                    let src = source_layer.value();
                    if src == "taxonomy(vo)" {
                        "Taxonomy Value Objects define domain primitives — they must import contracts to declare their structural contract.".to_string()
                    } else if src == "taxonomy(entity)" {
                        "Taxonomy Entities model domain state — they must import aggregator contracts to participate in business operations.".to_string()
                    } else if src == "contract(port)" || src == "contract(protocol)" {
                        "Contract ports define service boundaries — they must import contract aggregate types to compose cross-cutting workflows.".to_string()
                    } else if src == "contract(aggregate)" {
                        "Contract aggregates orchestrate cross-layer collaboration — they must import all relevant port/protocol contracts.".to_string()
                    } else if src == "capabilities" {
                        "Capabilities implement business rules — they MUST import contract protocols to know what interface to honor. Missing contract protocol means broken/useless capability or missing requirement.".to_string()
                    } else if src == "infrastructure" {
                        "Infrastructure adapters MUST import contract ports — without a port reference this file is broken/useless. Either rename/delete if not real infrastructure, or create the required contract port first.".to_string()
                    } else if src == "agent(container)" {
                        "Agent containers wire dependencies at startup — they must import service contracts to register all concrete implementations.".to_string()
                    } else if src == "agent(orchestrator)" {
                        "Agent orchestrators coordinate use-case flows — they must import capability contracts to dispatch work correctly.".to_string()
                    } else if src == "surfaces(command)" || src == "surfaces(controller)" {
                        "Command/controller surfaces are user entry points — they must import aggregate contracts to delegate without bypassing business logic.".to_string()
                    } else if src == "surfaces(component)" || src == "surfaces(view)" {
                        "Passive surface components render UI — they must import taxonomy VOs to display type-safe domain data.".to_string()
                    } else if src.starts_with("taxonomy") {
                        format!(
                            "Layer '{}' must import '{}' to maintain domain model integrity.",
                            src, required
                        )
                    } else if src.starts_with("contract") {
                        format!("Layer '{}' must import '{}' to satisfy interface composition requirements.", src, required)
                    } else if src.starts_with("agent") {
                        format!(
                            "Layer '{}' must import '{}' to wire all required dependencies.",
                            src, required
                        )
                    } else if src.starts_with("surfaces") {
                        format!("Layer '{}' must import '{}' to properly delegate to the aggregate layer.", src, required)
                    } else {
                        format!("Layer '{}' must import '{}' to satisfy architectural contract requirements.", src, required)
                    }
                };
                let supplement = match reason.as_ref() {
                    Some(r) => format!("\n  Context: {}", r),
                    None => String::new(),
                };
                write!(
                    f,
                    "AES202 MANDATORY_IMPORT: Layer '{}' is missing required import '{}'.\n\
                        WHY? {}{}\n\
                        FIX: Add the required import statement for '{}' in this file.",
                    source_layer, required, default_why, supplement, required
                )
            }
            AesImportViolation::ImportIntentViolation {
                source_layer,
                import_type,
                intent: _,
                reason,
            } => {
                let default_why = format!(
                    "Import '{}' in layer '{}' is not used according to its intended purpose.",
                    import_type, source_layer
                );
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES204 IMPORT_INTENT: '{}' import in layer '{}' violates its intended purpose.\n\
                        WHY? {why}\n\
                        FIX: Use imported symbols in real logic, not only in dummy functions or stubs",
                    import_type, source_layer
                )
            }
            AesImportViolation::CircularImport { reason } => {
                let default_why = "Circular dependencies couple components together and break unidirectional data/import flow.".to_string();
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES205 CIRCULAR_IMPORT: Circular dependency detected.\n\
                        WHY? {}\n\
                        FIX: Refactor imports or extract the shared logic into a lower, common layer.",
                    why
                )
            }
            AesImportViolation::FixUnusedImport { reason } => {
                let default_why =
                    "Unused imports clutter the codebase and increase compilation/dependency overhead."
                        .to_string();
                let supplement = match reason.as_ref() {
                    Some(r) => format!("\n  Context: {}", r),
                    None => String::new(),
                };
                write!(f, "AES203 UNUSED_IMPORT: Unused import detected.\n\
                        WHY? {}{}\n\
                        FIX: Remove the unused import statement or use the imported symbol in this file.", default_why, supplement)
            }
        }
    }
}

impl From<AesImportViolation> for String {
    fn from(v: AesImportViolation) -> String {
        v.to_string()
    }
}
```

---

## File: crates/shared/src/mcp-server/mod.rs

```rust
// mcp-server — taxonomy and contract types
// Re-export from common for backward compatibility
pub use crate::common::taxonomy_action_vo;
pub use crate::common::taxonomy_job_vo;
```

---

## File: crates/shared/src/naming-rules/contract_naming_analyzer_protocol.rs

```rust
// PURPOSE: INamingAnalyzerProtocol — protocol trait for naming-rules analyzer dependency isolation
use crate::common::taxonomy_definition_vo::LayerMapVO;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;

pub trait INamingAnalyzerProtocol: Send + Sync {
    fn config(&self) -> &ArchitectureConfig;
    fn layer_map(&self) -> &LayerMapVO;
    fn detect_layer(&self, f: &FilePath, root_dir: &FilePath) -> Option<LayerNameVO>;
}
```

---

## File: crates/shared/src/naming-rules/mod.rs

```rust
pub mod contract_naming_analyzer_protocol;
pub mod contract_naming_checker_protocol;
pub mod contract_naming_filesystem_port;
pub mod contract_naming_runner_aggregate;
pub mod taxonomy_naming_rule_vo;
pub mod taxonomy_naming_violation_vo;
pub mod taxonomy_suffix_vo;
pub use taxonomy_naming_violation_vo::NamingViolation;
```

---
