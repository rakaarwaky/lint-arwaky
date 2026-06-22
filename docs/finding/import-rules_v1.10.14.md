# Crate: import-rules (v1.10.14)

This document contains the source code for feature crate `import-rules` along with its corresponding and imported definitions from the `shared` crate.

## File List

- [crates/import-rules/Cargo.toml](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/Cargo.toml)
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
- [crates/shared/src/auto-fix/taxonomy_fix_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/taxonomy_fix_vo.rs)
- [crates/shared/src/cli-commands/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/mod.rs)
- [crates/shared/src/cli-commands/taxonomy_metadata_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_metadata_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_position_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_position_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_protocol_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_protocol_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_score_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_score_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_severity_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_transport_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_transport_error.rs)
- [crates/shared/src/code-analysis/contract_cycle_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_cycle_protocol.rs)
- [crates/shared/src/code-analysis/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/mod.rs)
- [crates/shared/src/code-analysis/taxonomy_analysis_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_analysis_vo.rs)
- [crates/shared/src/code-analysis/taxonomy_governance_entity.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_governance_entity.rs)
- [crates/shared/src/code-analysis/taxonomy_import_source_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_import_source_vo.rs)
- [crates/shared/src/common/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/mod.rs)
- [crates/shared/src/common/taxonomy_action_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_action_vo.rs)
- [crates/shared/src/common/taxonomy_adapter_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_name_vo.rs)
- [crates/shared/src/common/taxonomy_common_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_error.rs)
- [crates/shared/src/common/taxonomy_common_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_vo.rs)
- [crates/shared/src/common/taxonomy_definition_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_definition_vo.rs)
- [crates/shared/src/common/taxonomy_duration_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_duration_vo.rs)
- [crates/shared/src/common/taxonomy_error_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_error_vo.rs)
- [crates/shared/src/common/taxonomy_job_id_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_job_id_vo.rs)
- [crates/shared/src/common/taxonomy_job_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_job_vo.rs)
- [crates/shared/src/common/taxonomy_layer_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_layer_vo.rs)
- [crates/shared/src/common/taxonomy_lint_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_lint_vo.rs)
- [crates/shared/src/common/taxonomy_message_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_message_vo.rs)
- [crates/shared/src/common/taxonomy_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_name_vo.rs)
- [crates/shared/src/common/taxonomy_response_data_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_response_data_vo.rs)
- [crates/shared/src/common/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_severity_vo.rs)
- [crates/shared/src/common/taxonomy_source_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_source_vo.rs)
- [crates/shared/src/common/taxonomy_suggestion_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_suggestion_vo.rs)
- [crates/shared/src/config-system/contract_parser_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_parser_port.rs)
- [crates/shared/src/config-system/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/mod.rs)
- [crates/shared/src/config-system/taxonomy_adapter_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_adapter_vo.rs)
- [crates/shared/src/config-system/taxonomy_config_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_config_error.rs)
- [crates/shared/src/config-system/taxonomy_config_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_config_vo.rs)
- [crates/shared/src/config-system/taxonomy_identifier_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_identifier_vo.rs)
- [crates/shared/src/config-system/taxonomy_setting_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_setting_vo.rs)
- [crates/shared/src/config-system/taxonomy_source_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_source_vo.rs)
- [crates/shared/src/file-system/contract_system_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-system/contract_system_port.rs)
- [crates/shared/src/file-system/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-system/mod.rs)
- [crates/shared/src/file-system/taxonomy_filesystem_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-system/taxonomy_filesystem_error.rs)
- [crates/shared/src/file-watch/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/mod.rs)
- [crates/shared/src/file-watch/taxonomy_diff_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/taxonomy_diff_result_vo.rs)
- [crates/shared/src/file-watch/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/taxonomy_result_vo.rs)
- [crates/shared/src/file-watch/taxonomy_service_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/taxonomy_service_error.rs)
- [crates/shared/src/file-watch/taxonomy_watch_event_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/taxonomy_watch_event_vo.rs)
- [crates/shared/src/git-hooks/taxonomy_hook_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/taxonomy_hook_error.rs)
- [crates/shared/src/git-hooks/taxonomy_installed_event.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/taxonomy_installed_event.rs)
- [crates/shared/src/git-hooks/taxonomy_ref_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/taxonomy_ref_vo.rs)
- [crates/shared/src/git-hooks/taxonomy_removed_event.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/taxonomy_removed_event.rs)
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
- [crates/shared/src/lib.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/lib.rs)
- [crates/shared/src/mcp-server/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/mcp-server/mod.rs)
- [crates/shared/src/multi-project/taxonomy_summary_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/multi-project/taxonomy_summary_vo.rs)
- [crates/shared/src/multi-project/taxonomy_workspace_info_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/multi-project/taxonomy_workspace_info_vo.rs)
- [crates/shared/src/naming-rules/taxonomy_suffix_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/taxonomy_suffix_vo.rs)
- [crates/shared/src/project-setup/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/mod.rs)
- [crates/shared/src/project-setup/taxonomy_doctor_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/taxonomy_doctor_vo.rs)
- [crates/shared/src/project-setup/taxonomy_language_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/taxonomy_language_vo.rs)
- [crates/shared/src/project-setup/taxonomy_stats_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/taxonomy_stats_vo.rs)
- [crates/shared/src/source-parsing/contract_parser_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/contract_parser_port.rs)
- [crates/shared/src/source-parsing/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/mod.rs)
- [crates/shared/src/source-parsing/taxonomy_adapter_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_adapter_error.rs)
- [crates/shared/src/source-parsing/taxonomy_language_detector_helper.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_language_detector_helper.rs)
- [crates/shared/src/source-parsing/taxonomy_naming_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_naming_error.rs)
- [crates/shared/src/source-parsing/taxonomy_naming_list_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_naming_list_vo.rs)
- [crates/shared/src/source-parsing/taxonomy_parser_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_parser_error.rs)
- [crates/shared/src/source-parsing/taxonomy_path_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_path_vo.rs)
- [crates/shared/src/source-parsing/taxonomy_paths_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_paths_vo.rs)
- [crates/shared/src/source-parsing/taxonomy_semantic_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_semantic_error.rs)

---

## File: crates/import-rules/Cargo.toml

```toml
[package]
name = "import_rules-lint-arwaky"
version = "1.10.14"
edition = "2021"

[dependencies]
serde.workspace = true
serde_json.workspace = true
async-trait.workspace = true
once_cell.workspace = true
regex.workspace = true
shared.workspace = true
```

---

## File: crates/import-rules/src/agent_import_orchestrator.rs

```rust
// PURPOSE: ImportOrchestrator — agent that orchestrates import rule checks
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::code_analysis::contract_cycle_protocol::ICycleAnalysisProtocol;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_rule_protocol::{IAnalyzer, IArchImportProtocol};
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use std::path::Path;
use std::sync::Arc;

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

    fn is_ignored(&self, p: &Path) -> bool {
        let s = p.to_string_lossy();
        let dir_name = p
            .file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_default();
        self.ignored_paths.iter().any(|ignored| {
            s.contains(ignored.as_str()) || dir_name.contains(ignored.trim_start_matches('/'))
        })
    }

    fn collect_files(&self, target: &FilePath) -> FilePathList {
        let path = Path::new(target.value());
        let mut files = Vec::new();
        if path.is_dir() {
            self.walk_dir(path, &mut files, true);
        } else if path.is_file() {
            files.push(FilePath::new(path.to_string_lossy().to_string()).unwrap_or_default());
        }
        FilePathList::new(files)
    }

    fn walk_dir(&self, dir: &Path, files: &mut Vec<FilePath>, is_subdir: bool) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if is_subdir && self.is_ignored(&path) {
                        continue;
                    }
                    self.walk_dir(&path, files, true);
                } else if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if matches!(
                            ext.to_str(),
                            Some("rs" | "py" | "js" | "ts" | "jsx" | "tsx")
                        ) {
                            files.push(
                                FilePath::new(path.to_string_lossy().to_string())
                                    .unwrap_or_default(),
                            );
                        }
                    }
                }
            }
        }
    }
}

#[async_trait]
impl IImportRunnerAggregate for ImportOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        let mut results = LintResultList::new(Vec::new());
        let files = self.collect_files(target);
        let root_dir = FilePath::new(target.value().split('/').next().unwrap_or(".").to_string())
            .unwrap_or_default();

        self.mandatory
            .check_mandatory_imports(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;
        self.forbidden
            .check_forbidden_imports(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;
        self.intent
            .check_mandatory_imports(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;

        // AES203: unused import check - read file content once and check all languages
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
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::import_rules::taxonomy_language_vo::LanguageVO;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use shared::taxonomy_layer_vo::{Identity, LayerNameVO};
use shared::taxonomy_name_vo::SymbolName;
use std::sync::Arc;

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
            .map(|(trait_name, _)| trait_name)
            .collect();

        // Step 4: Detect the architectural layer for this file
        let layer_name = analyzer
            .detect_layer(
                &FilePath::new(file.to_string()).unwrap_or_default(),
                root_dir,
            )
            .map(|l| l.to_string())
            .unwrap_or_else(|| "any".to_string());

        // Step 5-7: Iterate imported symbols and check if they have real usage
        for (symbol, line_no) in self.parser.get_imported_symbols(&lines, lang) {
            // Step 6: Skip symbols that are actually used outside dummy/stub contexts
            if self
                .parser
                .is_symbol_used_real(&lines, &symbol, &dummy_ranges, &dummy_impl_traits)
            {
                continue;
            }

            // Step 7: Symbol is only used in dummy/stub — flag as violation
            violations.push(LintResult::new_arch(
                file,
                line_no,
                "AES204",
                Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(layer_name.clone()),
                    import_type: SymbolName::new(symbol),
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
        let layer_name = analyzer
            .detect_layer(
                &FilePath::new(file.to_string()).unwrap_or_default(),
                root_dir,
            )
            .map(|l| l.to_string())
            .unwrap_or_else(|| "any".to_string());

        // Step 3-4: Flag each dummy function as violation
        for (start, end) in self.parser.get_dummy_function_ranges(&lines, lang) {
            violations.push(LintResult::new_arch(
                file,
                start,
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
        let layer_name = analyzer
            .detect_layer(
                &FilePath::new(file.to_string()).unwrap_or_default(),
                root_dir,
            )
            .map(|l| l.to_string())
            .unwrap_or_else(|| "any".to_string());

        // Step 3-4: Flag each dummy/stub trait implementation
        for (trait_name, start) in self.parser.get_dummy_impl_traits_with_lines(&lines) {
            violations.push(LintResult::new_arch(
                file,
                start,
                "AES204",
                Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(layer_name.clone()),
                    import_type: SymbolName::new(trait_name),
                    intent: SymbolName::new(
                        "Implement contract methods with real behavior instead of empty/todo stubs"
                            .to_string(),
                    ),
                    reason: Some(shared::taxonomy_message_vo::LintMessage::new(
                        "Trait implementations with empty bodies, todo!(), or unimplemented!() \
                         violate the contract abstraction — the import exists to fulfill a \
                         dependency, but no real behavior is provided. Every method must have \
                         meaningful logic; otherwise the contract becomes untestable and masks \
                         missing functionality."
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

        let _layer_name = analyzer
            .detect_layer(
                &FilePath::new(file.to_string()).unwrap_or_default(),
                root_dir,
            )
            .map(|l| l.to_string())
            .unwrap_or_else(|| "any".to_string());

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
            .map(|(trait_name, _)| trait_name)
            .collect();

        let imported = self.parser.get_imported_symbols(&lines, lang);
        let has_real_usage = imported.iter().any(|(symbol, line_no)| {
            let is_taxonomy = lines.get(line_no.saturating_sub(1)).is_some_and(|line| {
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
            self.parser
                .is_symbol_used_real(&lines, symbol, &dummy_ranges, &dummy_impl_traits)
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
    ///   3. For each line, check if it contains a phantom marker (PhantomData, TYPE_CHECKING, @ts-ignore)
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
            .filter(|(symbol, _)| symbol.ends_with("Aggregate"))
            .map(|(symbol, _)| symbol)
            .collect();

        // Step 3-5: Scan lines for phantom + aggregate type combinations
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            let is_phantom = match lang {
                LanguageVO::Rust => trimmed.contains("PhantomData"),
                LanguageVO::Python => trimmed.contains("TYPE_CHECKING"),
                LanguageVO::JavaScript => {
                    trimmed.contains("@ts-ignore") || trimmed.contains("@ts-expect")
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
            let Ok(content) = self.parser.read_file_to_string(f) else {
                continue;
            };

            // Step 4: Run universal sub-checks (every file type)
            self.check_dummy_imports(&f_str, &content, &mut results.values, analyzer, root_dir);
            self.check_dummy_functions(&f_str, &content, &mut results.values, analyzer, root_dir);
            self.check_dummy_impls(&f_str, &content, &mut results.values, analyzer, root_dir);

            // Step 5: Detect if this is a surface-layer file
            let basename = std::path::Path::new(&f_str)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
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
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_rule_protocol::{
    IAnalyzer, IArchImportProtocol, IArchRuleProtocol,
};
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_layer_vo::{Identity, LayerNameVO};
use std::sync::Arc;

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
        let file_path = FilePath::new(file.to_string()).unwrap_or_default();
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
                            self.parser
                                .extract_layer_from_import(&cleaned_identity)
                                .map(|l| l == layer)
                                .unwrap_or(false)
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
        let file_path = FilePath::new(file.to_string()).unwrap_or_default();
        let basename_identity = self.parser.get_basename(&file_path);
        let basename = basename_identity.value();
        // Step 2: Skip Rust module entry files
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }
        let stem = basename.rsplit('.').next_back().unwrap_or(basename);
        let suffix = stem.rsplit('_').next().unwrap_or("");

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
                                self.parser
                                    .extract_layer_from_import(&cleaned_identity)
                                    .map(|l| l == forbidden_layer)
                                    .unwrap_or(false)
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
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_rule_protocol::{
    IAnalyzer, IArchImportProtocol, IArchRuleProtocol,
};
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO};
use shared::taxonomy_name_vo::SymbolName;
use std::sync::Arc;

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
        let file_path = FilePath::new(file.to_string()).unwrap_or_default();
        let basename_identity = self.parser.get_basename(&file_path);
        let basename = basename_identity.value();
        if basename == "__init__.py" {
            return;
        }
        if definition.exceptions.values.contains(&basename.to_string()) {
            return;
        }

        // Step 4: Read file and parse import lines
        let Ok(content) = self.parser.read_file_to_string(&file_path) else {
            return;
        };
        let file_content = FileContentVO::new(content);
        let import_lines = self.parser.parse_import_lines(&file_content);

        // Step 5: Derive source layer from filename (first prefix segment)
        let stem = basename.rsplit('.').next_back().unwrap_or(basename);
        let source_layer = stem.split('_').next().unwrap_or("unknown");

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
        let file_path = FilePath::new(file.to_string()).unwrap_or_default();
        let basename_identity = self.parser.get_basename(&file_path);
        let basename = basename_identity.value();
        // Step 2: Skip Rust entry files
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }
        let stem = basename.rsplit('.').next_back().unwrap_or(basename);
        let suffix = stem.rsplit('_').next().unwrap_or("");

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
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::source_parsing::taxonomy_path_vo::FilePath;
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
    fn find_unused_imports(&self, path: &FilePath) -> Vec<String> {
        // Step 1: Read file content
        let Ok(content) = self.parser.read_file_to_string(path) else {
            return vec![];
        };

        // Step 2: Get imported symbols/aliases from the source file
        let imported_aliases = self.parser.extract_imported_aliases(&content);

        // Step 3: Get exported symbols (like __all__ in Python)
        let exported_symbols = self.parser.extract_exported_symbols(&content);

        // Step 4: Find which of these imported aliases are actually used in the code
        let used_symbols = self
            .parser
            .extract_used_symbols(&content, &imported_aliases);

        let mut unused = Vec::new();

        // Step 5: Identify unused Python/standard imports
        for alias in imported_aliases.keys() {
            if !used_symbols.contains(alias) && !exported_symbols.contains(alias) {
                unused.push(alias.clone());
            }
        }

        // Step 6: Handle Rust/JS specific imports
        let rust_js_imports = self.parser.extract_rust_js_imports(&content);
        for (name, line_idx) in rust_js_imports {
            if !self.parser.is_name_used(&name, &content, line_idx) {
                unused.push(name);
            }
        }

        unused
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
                let line_num = self.parser.find_import_line_number(content, alias);
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
            if !self.parser.is_name_used(&name, content, line_idx) {
                violations.push(LintResult::new_arch(
                    file,
                    line_idx + 1,
                    "AES203",
                    Severity::MEDIUM,
                    AesImportViolation::FixUnusedImport {
                        reason: Some(shared::taxonomy_message_vo::LintMessage::new(format!(
                            "Import '{}' is declared but never used in this file.",
                            name
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

use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::config_system::taxonomy_config_vo::ArchitectureRule;
use shared::file_system::contract_system_port::IFileSystemPort;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::import_rules::taxonomy_path_helper;
use shared::source_parsing::contract_parser_port::ISourceParserPort;
use shared::source_parsing::taxonomy_path_vo::FilePath;
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
                scope.split('(').next().unwrap_or(&scope).to_string()
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
            let base_name = lstr.split('(').next().unwrap_or(&lstr).to_string();
            // Apply: global rules (key="") + base-layer rules (key=base_name)
            for key in &[String::new(), base_name.clone()] {
                if let Some(rules) = rules_by_layer.get(key.as_str()) {
                    for rule in rules {
                        // Skip specialised scoped rules (e.g. contract(port)) when processing base layers
                        if key.as_str() == base_name && rule.scope.value.contains('(') {
                            continue;
                        }
                        if !rule.exceptions.values.is_empty() {
                            ldef.exceptions = rule.exceptions.clone();
                        }
                        if !rule.mandatory.values.is_empty() {
                            ldef.mandatory = rule.mandatory.clone();
                        }
                        if !rule.forbidden.values.is_empty() {
                            ldef.forbidden = rule.forbidden.clone();
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
                            ldef.code_analysis.forbidden_inheritance =
                                rule.code_analysis.forbidden_inheritance.clone();
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
                                    spec_def.exceptions = r.exceptions.clone();
                                }
                                if !r.forbidden.values.is_empty() {
                                    spec_def.forbidden = r.forbidden.clone();
                                }
                                if !r.mandatory.values.is_empty() {
                                    spec_def.mandatory = r.mandatory.clone();
                                }
                                if !r.allowed.values.is_empty() {
                                    spec_def.allowed = r.allowed.clone();
                                }
                                if !r.code_analysis.forbidden_inheritance.values.is_empty() {
                                    spec_def.code_analysis.forbidden_inheritance =
                                        r.code_analysis.forbidden_inheritance.clone();
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
            .unwrap_or("");

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
            let base_name = name.value.split('(').next().unwrap_or(&name.value);
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
            .unwrap_or("");

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
                let base = layer.split('(').next().unwrap_or(layer);
                self.config.layers.get(&LayerNameVO::new(base))
            })
    }
}

impl IAnalyzer for LayerDetectionAnalyzer {
    /// Return the merged architecture configuration.
    fn config(&self) -> &ArchitectureConfig {
        &self.config
    }
    /// Return the layer map (layer name → LayerDefinition).
    fn layer_map(&self) -> &LayerMapVO {
        &self.layer_map
    }
    /// Return the filesystem port for file I/O.
    fn fs(&self) -> &dyn IFileSystemPort {
        &*self.fs
    }
    /// Return the source parser port for code analysis.
    fn parser(&self) -> &dyn ISourceParserPort {
        &*self.parser
    }
    /// Adapter: delegates to internal `detect_layer` and wraps result in LayerNameVO.
    fn detect_layer(&self, f: &FilePath, root_dir: &FilePath) -> Option<LayerNameVO> {
        self.detect_layer(&f.value, &root_dir.value)
            .map(|s| LayerNameVO::new(s.as_str()))
    }
    /// Adapter: delegates to internal `detect_module_layer` and wraps result in LayerNameVO.
    fn detect_module_layer(&self, module_path: &FilePath) -> Option<LayerNameVO> {
        self.detect_module_layer(&module_path.value)
            .map(|s| LayerNameVO::new(s.as_str()))
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

use shared::file_system::contract_system_port::IFileSystemPort;
use shared::file_system::taxonomy_filesystem_error::FileSystemError;
use shared::mcp_server::taxonomy_action_vo::ActionName;
use shared::mcp_server::taxonomy_job_vo::SuccessStatus;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::Count;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_layer_vo::Identity;
use shared::taxonomy_source_vo::ContentString;

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
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
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
        let ignored = ignored_patterns
            .map(|p| p.values.clone())
            .unwrap_or_default();
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
        match p.strip_prefix(s) {
            Ok(rel) => {
                FilePath::new(rel.to_string_lossy().to_string()).unwrap_or_else(|_| path.clone())
            }
            Err(_) => path.clone(),
        }
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
        match p.parent() {
            Some(parent) => {
                FilePath::new(parent.to_string_lossy().to_string()).unwrap_or_else(|_| path.clone())
            }
            None => path.clone(),
        }
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
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        FilePath::new(cwd.to_string_lossy().to_string())
            .unwrap_or_else(|_| FilePath::new(".".to_string()).unwrap_or_default())
    }

    async fn get_basename(&self, path: &FilePath) -> Identity {
        let p = Path::new(&path.value);
        Identity::new(
            p.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string(),
        )
    }

    async fn path_join(&self, parts: &[Identity]) -> FilePath {
        let mut path = PathBuf::new();
        for part in parts {
            path.push(&part.value);
        }
        FilePath::new(path.to_string_lossy().to_string())
            .unwrap_or_else(|_| FilePath::new(".".to_string()).unwrap_or_default())
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

use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use shared::import_rules::taxonomy_language_vo::LanguageVO;
use shared::import_rules::taxonomy_path_helper;
use shared::import_rules::{
    taxonomy_cycle_helper, taxonomy_dummy_helper, taxonomy_parser_helper, taxonomy_unused_helper,
};
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO, LineContentVO};
use shared::taxonomy_name_vo::SymbolName;
use std::fs;

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
    /// into layer + suffix matches. Returns (LayerNameVO, Vec<Identity>).
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
                let first_token = rest.split_whitespace().next().unwrap_or("");
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

    fn read_file_to_string(&self, file: &FilePath) -> Result<String, std::io::Error> {
        fs::read_to_string(file.value())
    }

    fn extract_import_modules(&self, content: &str) -> Vec<String> {
        taxonomy_parser_helper::extract_import_modules(content)
    }

    fn get_language_from_path(&self, path: &str) -> LanguageVO {
        LanguageVO::from_path(path)
    }

    fn get_dummy_function_ranges(&self, lines: &[&str], lang: LanguageVO) -> Vec<(usize, usize)> {
        taxonomy_dummy_helper::dummy_function_ranges(lines, lang)
    }

    fn get_imported_symbols(&self, lines: &[&str], lang: LanguageVO) -> Vec<(String, usize)> {
        taxonomy_dummy_helper::imported_symbols(lines, lang)
    }

    fn get_dummy_impl_traits_with_lines(&self, lines: &[&str]) -> Vec<(String, usize)> {
        taxonomy_dummy_helper::dummy_impl_traits_with_lines(lines)
    }

    fn is_symbol_used_real(
        &self,
        lines: &[&str],
        symbol: &str,
        dummy_ranges: &[(usize, usize)],
        dummy_impl_traits: &[String],
    ) -> bool {
        taxonomy_dummy_helper::symbol_used_real(lines, symbol, dummy_ranges, dummy_impl_traits)
    }

    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName> {
        taxonomy_cycle_helper::detect_cycle_edges(edges)
    }

    fn extract_imported_aliases(&self, content: &str) -> std::collections::HashMap<String, String> {
        taxonomy_unused_helper::extract_imported_aliases(content)
    }

    fn extract_exported_symbols(&self, content: &str) -> std::collections::HashSet<String> {
        taxonomy_unused_helper::extract_exported_symbols(content)
    }

    fn extract_used_symbols(
        &self,
        content: &str,
        imported_aliases: &std::collections::HashMap<String, String>,
    ) -> std::collections::HashSet<String> {
        taxonomy_unused_helper::extract_used_symbols(content, imported_aliases)
    }

    fn find_import_line_number(&self, content: &str, alias: &str) -> usize {
        content
            .lines()
            .position(|l| {
                l.trim().contains(&format!("import {}", alias))
                    || l.trim().contains(&format!(
                        "from {} import",
                        alias.split('.').next().unwrap_or("")
                    ))
            })
            .map(|p| p + 1)
            .unwrap_or(1)
    }

    fn extract_rust_js_imports(&self, content: &str) -> Vec<(String, usize)> {
        taxonomy_unused_helper::extract_rust_js_imports(content)
    }

    fn is_name_used(&self, name: &str, content: &str, exclude_line: usize) -> bool {
        taxonomy_unused_helper::is_name_used(name, content, exclude_line)
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
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::import_rules::contract_rule_protocol::IArchImportProtocol;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::source_parsing::contract_parser_port::ISourceParserPort;
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
```

---

## File: crates/shared/src/auto-fix/taxonomy_fix_applied_event.rs

```rust
// PURPOSE: FixApplied — domain event published when a lint fix is applied
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::Timestamp;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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

## File: crates/shared/src/auto-fix/taxonomy_fix_vo.rs

```rust
// PURPOSE: FixResult — value object capturing fix application outcome
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FixResult {
    pub output: DescriptionVO,
    #[serde(default)]
    pub error: Option<ErrorMessage>,
}

impl FixResult {
    pub fn new(output: DescriptionVO, error: Option<ErrorMessage>) -> Self {
        Self { output, error }
    }
    pub fn is_success(&self) -> bool {
        self.error.is_none()
    }
}

impl std::fmt::Display for FixResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.error {
            Some(e) => write!(f, "{}", e),
            None => write!(f, "{}", self.output),
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
pub mod taxonomy_metadata_vo;
pub mod taxonomy_position_vo;
pub mod taxonomy_protocol_vo;
pub mod taxonomy_result_vo;
pub mod taxonomy_score_vo;
pub mod taxonomy_severity_vo;
pub mod taxonomy_transport_error;
```

---

## File: crates/shared/src/cli-commands/taxonomy_metadata_vo.rs

```rust
// PURPOSE: CommandMetadataVO — value object wrapping description + usage example for each CLI command
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::common::taxonomy_suggestion_vo::Suggestion;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandMetadataVO {
    pub description: DescriptionVO,
    pub example: Suggestion,
}

impl CommandMetadataVO {
    pub fn new(description: DescriptionVO, example: Suggestion) -> Self {
        Self {
            description,
            example,
        }
    }
}

impl std::fmt::Display for CommandMetadataVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.description, self.example)
    }
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_position_vo.rs

```rust
// PURPOSE: Position — value object for source code position tracking (file, line, column)
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub line: LineNumber,
    #[serde(default)]
    pub column: ColumnNumber,
}

impl Position {
    pub fn new(line: LineNumber) -> Self {
        Self {
            line,
            column: ColumnNumber::new(0),
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.column.value > 0 {
            write!(f, "{}:{}", self.line, self.column)
        } else {
            write!(f, "{}", self.line)
        }
    }
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_protocol_vo.rs

```rust
// PURPOSE: TransportEndpoint, TransportProtocol, TransportUrlVO — value objects for transport endpoint configuration
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransportEndpoint {
    pub protocol: TransportProtocol,
    pub address: String,
}

impl Default for TransportEndpoint {
    fn default() -> Self {
        Self {
            protocol: TransportProtocol::STDAggregate,
            address: String::new(),
        }
    }
}

impl TransportEndpoint {
    pub fn new(protocol: TransportProtocol, address: String) -> Self {
        Self { protocol, address }
    }

    pub fn display_name(&self) -> String {
        match self.protocol {
            TransportProtocol::HTTP => format!("HTTP({})", self.address),
            TransportProtocol::UnixSocket => format!("Socket({})", self.address),
            TransportProtocol::STDAggregate => "Stdio(direct)".to_string(),
        }
    }
    pub fn from_url(url: &str) -> Self {
        if url.starts_with("http://") || url.starts_with("https://") {
            Self {
                protocol: TransportProtocol::HTTP,
                address: url.to_string(),
            }
        } else if url == "stdio" {
            Self {
                protocol: TransportProtocol::STDAggregate,
                address: "stdio".to_string(),
            }
        } else if url.starts_with("/") || url.starts_with(".") {
            Self {
                protocol: TransportProtocol::UnixSocket,
                address: url.to_string(),
            }
        } else {
            Self {
                protocol: TransportProtocol::STDAggregate,
                address: "stdio".to_string(),
            }
        }
    }
}

impl std::fmt::Display for TransportEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.protocol, self.address)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TransportProtocol {
    #[serde(rename = "HTTP")]
    HTTP,
    #[serde(rename = "UnixSocket")]
    UnixSocket,
    #[serde(rename = "Stdio")]
    STDAggregate,
}

impl std::fmt::Display for TransportProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportProtocol::HTTP => write!(f, "HTTP"),
            TransportProtocol::UnixSocket => write!(f, "UnixSocket"),
            TransportProtocol::STDAggregate => write!(f, "Stdio"),
        }
    }
}

impl TransportProtocol {
    pub fn needs_desktop_commander(&self) -> bool {
        matches!(
            self,
            TransportProtocol::HTTP | TransportProtocol::UnixSocket
        )
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct TransportUrlVO {
    pub value: String,
}

impl TransportUrlVO {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for TransportUrlVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for TransportUrlVO {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for TransportUrlVO {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for TransportUrlVO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TransportUrlVOVisitor {}
        impl<'de> serde::de::Visitor<'de> for TransportUrlVOVisitor {
            type Value = TransportUrlVO;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(TransportUrlVO {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(TransportUrlVO { value: v })
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
                Ok(TransportUrlVO { value: val })
            }
        }
        deserializer.deserialize_any(TransportUrlVOVisitor {})
    }
}
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
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LintResultList {
    pub values: Vec<LintResult>,
}

impl LintResultList {
    pub fn new(value: Vec<LintResult>) -> Self {
        Self { values: value }
    }
    pub fn iter(&self) -> std::slice::Iter<'_, LintResult> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: LintResult) {
        self.values.push(item);
    }
    pub fn append(&mut self, item: LintResult) {
        self.values.push(item);
    }
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_score_vo.rs

```rust
// PURPOSE: Score, FileFormat, ScoreMap — value objects for compliance scoring and file format enums
use serde::Serialize;

use crate::cli_commands::taxonomy_result_vo::LintResult;

pub fn compute_score(results: &[LintResult]) -> f64 {
    let penalty: f64 = results.iter().map(|r| r.severity.score_impact()).sum();
    (100.0 - penalty).max(0.0)
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct FileFormat {
    pub name: String,
}

impl FileFormat {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn new(value: impl Into<String>) -> Self {
        Self { name: value.into() }
    }
    pub fn is_structured(&self) -> bool {
        matches!(self.name.as_ref(), "json" | "sarif" | "junit")
    }
}

impl std::fmt::Display for FileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<&str> for FileFormat {
    fn from(s: &str) -> Self {
        Self {
            name: s.to_string(),
        }
    }
}

impl From<String> for FileFormat {
    fn from(s: String) -> Self {
        Self { name: s }
    }
}

impl<'de> serde::Deserialize<'de> for FileFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FileFormatVisitor {}
        impl<'de> serde::de::Visitor<'de> for FileFormatVisitor {
            type Value = FileFormat;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(FileFormat {
                    name: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(FileFormat { name: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "name" || k == "value" {
                        value = Some(map.next_value::<String>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                Ok(FileFormat { name: val })
            }
        }
        deserializer.deserialize_any(FileFormatVisitor {})
    }
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_severity_vo.rs

```rust
// PURPOSE: Severity — re-export from common for backward compatibility
pub use crate::common::taxonomy_severity_vo::Severity;
```

---

## File: crates/shared/src/cli-commands/taxonomy_transport_error.rs

```rust
// PURPOSE: TransportError — structured error type wrapping protocol, message, endpoint, and underlying error
use crate::cli_commands::taxonomy_protocol_vo::TransportEndpoint;
use crate::cli_commands::taxonomy_protocol_vo::TransportProtocol;
use crate::common::taxonomy_common_error::ErrorMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct TransportError {
    pub protocol: TransportProtocol,
    pub message: ErrorMessage,
    pub endpoint: TransportEndpoint,
    pub underlying_error: ErrorMessage,
}

impl TransportError {
    pub fn new(protocol: TransportProtocol, message: ErrorMessage) -> Self {
        Self {
            protocol,
            message,
            endpoint: TransportEndpoint::default(),
            underlying_error: ErrorMessage::default(),
        }
    }
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ep_str = self.endpoint.to_string();
        let ep = if ep_str.is_empty() {
            String::new()
        } else {
            format!(" {}", ep_str)
        };
        write!(f, "[{}]{} {}", self.protocol, ep, self.message)
    }
}
```

---

## File: crates/shared/src/code-analysis/contract_cycle_protocol.rs

```rust
// PURPOSE: ICycleAnalysisProtocol + DefaultCycleAnalysisProtocol — port trait and default impl for circular dependency detection (AES205)
use std::collections::{HashMap, HashSet};

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_lint_vo::ScopeRef;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::import_rules::contract_rule_protocol::IAnalyzer;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use async_trait::async_trait;

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

pub struct DefaultCycleAnalysisProtocol {}

fn find_rust_crate_root(source_file: &str) -> Option<std::path::PathBuf> {
    let mut current = std::path::Path::new(source_file).parent()?;
    while !current.join("Cargo.toml").exists() {
        current = current.parent()?;
    }
    Some(current.join("src"))
}

fn try_resolve_candidates(
    base_path: &str,
    module_path: &str,
    file_set: &HashSet<String>,
) -> Option<String> {
    let exts = ["rs", "py", "ts", "js"];
    for ext in &exts {
        let candidate = format!("{}/{}.{}", base_path, module_path, ext);
        if file_set.contains(&candidate) {
            return Some(candidate);
        }
    }
    for ext in &exts {
        let candidate = format!("{}/{}/mod.{}", base_path, module_path, ext);
        if file_set.contains(&candidate) {
            return Some(candidate);
        }
    }
    for ext in &exts {
        let candidate = format!("{}/{}/__init__.{}", base_path, module_path, ext);
        if file_set.contains(&candidate) {
            return Some(candidate);
        }
    }
    for ext in &exts {
        let candidate = format!("{}/{}/index.{}", base_path, module_path, ext);
        if file_set.contains(&candidate) {
            return Some(candidate);
        }
    }
    // Also check bare module path (no base prefix) for flat file sets
    for ext in &exts {
        let candidate = format!("{}.{}", module_path, ext);
        if file_set.contains(&candidate) {
            return Some(candidate);
        }
    }
    None
}

fn resolve_import_to_file(
    module: &str,
    source_file: &FilePath,
    root_dir: &FilePath,
    file_set: &HashSet<String>,
) -> Option<String> {
    let source_dir = std::path::Path::new(source_file.value())
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    // 1. Handle relative imports starting with dots (Python, JS/TS)
    if module.starts_with('.') {
        let mut current_dir = std::path::PathBuf::from(&source_dir);
        let mut remaining = module;

        if remaining.starts_with("./") || remaining.starts_with("../") {
            // JS/TS style
            if let Some(r) = remaining.strip_prefix("./") {
                remaining = r;
            }
            while let Some(r) = remaining.strip_prefix("../") {
                remaining = r;
                if let Some(parent) = current_dir.parent() {
                    current_dir = parent.to_path_buf();
                }
            }
            let remaining_path = remaining.replace('\\', "/");
            let resolved_str = current_dir.to_string_lossy().to_string();

            let exts = ["rs", "py", "ts", "js"];
            for ext in &exts {
                let candidate = format!("{}/{}.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            for ext in &exts {
                let candidate = format!("{}/{}/mod.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            for ext in &exts {
                let candidate = format!("{}/{}/__init__.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            for ext in &exts {
                let candidate = format!("{}/{}/index.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            return None;
        } else {
            // Python style (count leading dots)
            let mut dots_count = 0;
            while remaining.starts_with('.') {
                dots_count += 1;
                remaining = &remaining[1..];
            }
            if dots_count > 1 {
                for _ in 0..(dots_count - 1) {
                    if let Some(parent) = current_dir.parent() {
                        current_dir = parent.to_path_buf();
                    }
                }
            }
            let remaining_path = remaining.replace('.', "/");
            let resolved_str = current_dir.to_string_lossy().to_string();

            let exts = ["rs", "py", "ts", "js"];
            for ext in &exts {
                let candidate = format!("{}/{}.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            for ext in &exts {
                let candidate = format!("{}/{}/mod.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            for ext in &exts {
                let candidate = format!("{}/{}/__init__.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            for ext in &exts {
                let candidate = format!("{}/{}/index.{}", resolved_str, remaining_path, ext);
                if file_set.contains(&candidate) {
                    return Some(candidate);
                }
            }
            return None;
        }
    }

    // 2. Handle Rust-specific imports
    let is_rust = source_file.value().ends_with(".rs");
    if is_rust {
        if let Some(crate_root) = find_rust_crate_root(source_file.value()) {
            let mut normalized = module.to_string();
            let mut resolved_base = crate_root.clone();

            if normalized.starts_with("crate::") {
                normalized = normalized.trim_start_matches("crate::").to_string();
            } else if normalized.starts_with("self::") {
                normalized = normalized.trim_start_matches("self::").to_string();
                resolved_base = std::path::PathBuf::from(&source_dir);
            } else if normalized.starts_with("super::") {
                let mut current_dir = std::path::PathBuf::from(&source_dir);
                while normalized.starts_with("super::") {
                    normalized = normalized.trim_start_matches("super::").to_string();
                    if let Some(parent) = current_dir.parent() {
                        current_dir = parent.to_path_buf();
                    }
                }
                resolved_base = current_dir;
            }

            let segments: Vec<&str> = normalized
                .split("::")
                .flat_map(|s| s.split('.'))
                .filter(|s| !s.is_empty())
                .collect();

            if !segments.is_empty() {
                // Check if it's a cross-crate import in workspace
                let first_seg = segments[0];
                let workspace_crate_src = std::path::Path::new(root_dir.value())
                    .join("crates")
                    .join(first_seg.replace('_', "-"))
                    .join("src");
                if workspace_crate_src.exists() {
                    let sub_segments = &segments[1..];
                    for len in (1..=sub_segments.len()).rev() {
                        let module_path = sub_segments[..len].join("/");
                        if let Some(target) = try_resolve_candidates(
                            &workspace_crate_src.to_string_lossy(),
                            &module_path,
                            file_set,
                        ) {
                            return Some(target);
                        }
                    }
                }
            }

            // Fallback: resolve relative to the resolved_base directory
            for len in (1..=segments.len()).rev() {
                let module_path = segments[..len].join("/");
                if let Some(target) =
                    try_resolve_candidates(&resolved_base.to_string_lossy(), &module_path, file_set)
                {
                    return Some(target);
                }
            }
        }
    }

    // 3. Fallback standard module resolution (Python or other language standard import)
    let segments: Vec<&str> = module.split('.').filter(|s| !s.is_empty()).collect();
    for len in (1..=segments.len()).rev() {
        let module_path = segments[..len].join("/");
        if let Some(target) = try_resolve_candidates(root_dir.value(), &module_path, file_set) {
            return Some(target);
        }
    }

    None
}

fn find_cycle_dfs(
    node: &str,
    adjacency: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    in_stack: &mut HashSet<String>,
    path: &mut Vec<String>,
) -> Option<Vec<String>> {
    if in_stack.contains(node) {
        let cycle_start = path.iter().position(|n| n == node);
        if let Some(start) = cycle_start {
            let mut cycle = path[start..].to_vec();
            cycle.push(node.to_string());
            return Some(cycle);
        }
    }
    if visited.contains(node) {
        return None;
    }

    visited.insert(node.to_string());
    in_stack.insert(node.to_string());
    path.push(node.to_string());

    if let Some(neighbors) = adjacency.get(node) {
        for neighbor in neighbors {
            if let Some(cycle) = find_cycle_dfs(neighbor, adjacency, visited, in_stack, path) {
                return Some(cycle);
            }
        }
    }

    path.pop();
    in_stack.remove(node);
    None
}

#[async_trait]
impl ICycleAnalysisProtocol for DefaultCycleAnalysisProtocol {
    async fn check_cycles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let file_set: HashSet<String> =
            files.values.iter().map(|f| f.value().to_string()).collect();

        let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();

        for file in &files.values {
            let imports = match analyzer.parser().extract_imports(file) {
                Ok(imp) => imp,
                Err(_) => continue,
            };

            for imp in imports.values {
                if let Some(target) = resolve_import_to_file(&imp.module, file, root_dir, &file_set)
                {
                    adjacency
                        .entry(file.value().to_string())
                        .or_default()
                        .push(target);
                }
            }
        }

        let mut global_visited: HashSet<String> = HashSet::new();
        let mut reported_cycles: HashSet<String> = HashSet::new();

        for file in &files.values {
            let file_str = file.value().to_string();
            if global_visited.contains(&file_str) {
                continue;
            }

            let mut in_stack: HashSet<String> = HashSet::new();
            let mut path: Vec<String> = Vec::new();

            if let Some(cycle) = find_cycle_dfs(
                &file_str,
                &adjacency,
                &mut global_visited,
                &mut in_stack,
                &mut path,
            ) {
                let mut unique_nodes = cycle[..cycle.len() - 1].to_vec();

                if !unique_nodes.is_empty() {
                    let min_idx = unique_nodes
                        .iter()
                        .enumerate()
                        .min_by_key(|&(_, val)| val)
                        .map(|(idx, _)| idx)
                        .unwrap_or(0);

                    unique_nodes.rotate_left(min_idx);
                    unique_nodes.push(unique_nodes[0].clone());

                    let cycle_display = unique_nodes.join(" -> ");

                    if reported_cycles.insert(cycle_display.clone()) {
                        if let Ok(cycle_file) = FilePath::new(unique_nodes[0].clone()) {
                            results.push(LintResult {
                                file: cycle_file,
                                line: crate::common::taxonomy_common_vo::LineNumber::new(1),
                                column: crate::common::taxonomy_common_vo::ColumnNumber::new(0),
                                code: ErrorCode::raw("AES205"),
                                message: LintMessage::new(format!(
                                    "Circular dependency detected: {}",
                                    cycle_display
                                )),
                                source: Some(AdapterName::raw("architecture")),
                                severity: Severity::CRITICAL,
                                enclosing_scope: Some(ScopeRef {
                                    name: DescriptionVO::new(String::new()),
                                    kind: DescriptionVO::new(String::new()),
                                    file: None,
                                    start_line: None,
                                    end_line: None,
                                }),
                                related_locations:
                                    crate::common::taxonomy_lint_vo::LocationList::new(),
                            });
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli_commands::taxonomy_result_vo::LintResultList;
    use crate::code_analysis::taxonomy_import_source_vo::{
        ImportInfo, ImportInfoList, PrimitiveViolationList,
    };
    use crate::common::taxonomy_common_vo::{BooleanVO, Count, LineNumber, PatternList};
    use crate::common::taxonomy_definition_vo::LayerMapVO;
    use crate::common::taxonomy_layer_vo::LayerNameVO;
    use crate::common::taxonomy_name_vo::SymbolName;
    use crate::common::taxonomy_suggestion_vo::MetadataVO;
    use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
    use crate::file_system::contract_system_port::IFileSystemPort;
    use crate::import_rules::contract_rule_protocol::IAnalyzer;
    use crate::mcp_server::taxonomy_job_vo::{ResponseData, SuccessStatus};
    use crate::source_parsing::contract_parser_port::ISourceParserPort;
    use crate::source_parsing::taxonomy_naming_list_vo::PrimitiveTypeList;
    use crate::source_parsing::taxonomy_parser_error::SourceParserError;
    use crate::source_parsing::taxonomy_path_vo::FilePath;
    use crate::source_parsing::taxonomy_paths_vo::FilePathList;
    use std::collections::HashMap;
    use std::fs;

    struct MockSourceParserPort {
        imports: HashMap<String, Vec<String>>,
    }

    impl ISourceParserPort for MockSourceParserPort {
        fn extract_imports(&self, path: &FilePath) -> Result<ImportInfoList, SourceParserError> {
            let mut list = ImportInfoList::new();
            if let Some(imp_list) = self.imports.get(path.value()) {
                for imp in imp_list {
                    list.push(ImportInfo::new(LineNumber::new(1), imp.clone()));
                }
            }
            Ok(list)
        }
        fn get_raw_symbols(&self, _path: &FilePath) -> Result<ResponseData, SourceParserError> {
            Ok(ResponseData {
                value: None,
                stdout: String::new(),
                stderr: String::new(),
                returncode: 0,
                metadata: HashMap::new(),
            })
        }
        fn get_class_attributes(&self, _path: &FilePath) -> ResponseData {
            ResponseData {
                value: None,
                stdout: String::new(),
                stderr: String::new(),
                returncode: 0,
                metadata: HashMap::new(),
            }
        }
        fn has_all_export(&self, _path: &FilePath) -> SuccessStatus {
            SuccessStatus::new(false)
        }
        fn find_primitive_violations(
            &self,
            _path: &FilePath,
            _primitive_types: &PrimitiveTypeList,
        ) -> PrimitiveViolationList {
            PrimitiveViolationList::new()
        }
        fn find_unused_imports(&self, _path: &FilePath) -> ImportInfoList {
            ImportInfoList::new()
        }
        fn get_class_definitions(&self, _path: &FilePath) -> Result<MetadataVO, SourceParserError> {
            Ok(MetadataVO::new(HashMap::new()))
        }
        fn get_function_definitions(&self, _path: &FilePath) -> MetadataVO {
            MetadataVO::new(HashMap::new())
        }
        fn is_symbol_exported(&self, _path: &FilePath, _symbol: &SymbolName) -> SuccessStatus {
            SuccessStatus::new(false)
        }
        fn get_class_methods(&self, _path: &FilePath) -> MetadataVO {
            MetadataVO::new(HashMap::new())
        }
        fn get_class_bases_map(&self, _path: &FilePath) -> MetadataVO {
            MetadataVO::new(HashMap::new())
        }
        fn get_assignment_targets(&self, _path: &FilePath) -> MetadataVO {
            MetadataVO::new(HashMap::new())
        }
        fn get_control_flow_count(&self, _path: &FilePath) -> Count {
            Count::new(0)
        }
        fn is_barrel_file(&self, _path: &FilePath) -> BooleanVO {
            BooleanVO::new(false)
        }
        fn get_stem(&self, _path: &FilePath) -> SymbolName {
            SymbolName::new(String::new())
        }
        fn is_entry_point(&self, _path: &FilePath) -> BooleanVO {
            BooleanVO::new(false)
        }
        fn get_supported_extensions(&self) -> PatternList {
            PatternList { values: vec![] }
        }
    }

    struct MockFileSystemPort {
        _dummy: bool,
    }
    #[async_trait::async_trait]
    impl IFileSystemPort for MockFileSystemPort {
        async fn walk(
            &self,
            _path: &FilePath,
            _ignored_patterns: Option<&PatternList>,
        ) -> FilePathList {
            FilePathList { values: vec![] }
        }
        async fn is_directory(&self, _path: &FilePath) -> SuccessStatus {
            SuccessStatus::new(false)
        }
        async fn is_file(&self, _path: &FilePath) -> SuccessStatus {
            SuccessStatus::new(false)
        }
        async fn get_relative_path(&self, _path: &FilePath, _start: &FilePath) -> FilePath {
            FilePath::new(String::new()).unwrap()
        }
        async fn read_text(
            &self,
            _path: &FilePath,
        ) -> Result<
            crate::common::taxonomy_source_vo::ContentString,
            crate::file_system::taxonomy_filesystem_error::FileSystemError,
        > {
            Ok(crate::common::taxonomy_source_vo::ContentString::new(
                String::new(),
            ))
        }
        async fn get_line_count(&self, _path: &FilePath) -> Count {
            Count::new(0)
        }
        async fn exists(&self, _path: &FilePath) -> SuccessStatus {
            SuccessStatus::new(false)
        }
        async fn get_parent(&self, _path: &FilePath) -> FilePath {
            FilePath::new(String::new()).unwrap()
        }
        async fn write_text(
            &self,
            _path: &FilePath,
            _content: &crate::common::taxonomy_source_vo::ContentString,
            _mode: Option<&crate::common::taxonomy_layer_vo::Identity>,
        ) -> Result<SuccessStatus, crate::file_system::taxonomy_filesystem_error::FileSystemError>
        {
            Ok(SuccessStatus::new(true))
        }
        async fn glob(
            &self,
            _pattern: &crate::common::taxonomy_layer_vo::Identity,
        ) -> FilePathList {
            FilePathList { values: vec![] }
        }
        async fn get_cwd(&self) -> FilePath {
            FilePath::new(String::new()).unwrap()
        }
        async fn get_basename(
            &self,
            _path: &FilePath,
        ) -> crate::common::taxonomy_layer_vo::Identity {
            crate::common::taxonomy_layer_vo::Identity::new("")
        }
        async fn path_join(
            &self,
            _parts: &[crate::common::taxonomy_layer_vo::Identity],
        ) -> FilePath {
            FilePath::new(String::new()).unwrap()
        }
        async fn read_file(
            &self,
            _path: &FilePath,
        ) -> Result<
            crate::common::taxonomy_source_vo::ContentString,
            crate::file_system::taxonomy_filesystem_error::FileSystemError,
        > {
            Ok(crate::common::taxonomy_source_vo::ContentString::new(
                String::new(),
            ))
        }
    }

    struct MockAnalyzer {
        parser: MockSourceParserPort,
        config: ArchitectureConfig,
        layer_map: LayerMapVO,
        fs: MockFileSystemPort,
    }

    impl IAnalyzer for MockAnalyzer {
        fn config(&self) -> &ArchitectureConfig {
            &self.config
        }
        fn layer_map(&self) -> &LayerMapVO {
            &self.layer_map
        }
        fn fs(&self) -> &dyn IFileSystemPort {
            &self.fs
        }
        fn parser(&self) -> &dyn ISourceParserPort {
            &self.parser
        }
        fn detect_layer(&self, _f: &FilePath, _root_dir: &FilePath) -> Option<LayerNameVO> {
            None
        }
        fn detect_module_layer(&self, _module_path: &FilePath) -> Option<LayerNameVO> {
            None
        }
    }

    #[tokio::test]
    async fn test_check_cycles_detection() {
        let mut imports = HashMap::new();
        imports.insert("/src/a.rs".to_string(), vec!["./b".to_string()]);
        imports.insert("/src/b.rs".to_string(), vec!["./a".to_string()]);

        let parser = MockSourceParserPort { imports };
        let analyzer = MockAnalyzer {
            parser,
            config: ArchitectureConfig::default(),
            layer_map: LayerMapVO::new(HashMap::new()),
            fs: MockFileSystemPort { _dummy: false },
        };

        let files = FilePathList {
            values: vec![
                FilePath::new("/src/a.rs".to_string()).unwrap(),
                FilePath::new("/src/b.rs".to_string()).unwrap(),
            ],
        };
        let root_dir = FilePath::new("/".to_string()).unwrap();
        let mut results = LintResultList::default();

        let protocol = DefaultCycleAnalysisProtocol {};
        protocol
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert!(!results.values.is_empty());
        assert_eq!(&*results.values[0].code, "AES205");
    }

    #[tokio::test]
    async fn test_check_cycles_self_cycle() {
        let mut imports = HashMap::new();
        imports.insert("/src/a.rs".to_string(), vec!["./a".to_string()]);

        let parser = MockSourceParserPort { imports };
        let analyzer = MockAnalyzer {
            parser,
            config: ArchitectureConfig::default(),
            layer_map: LayerMapVO::new(HashMap::new()),
            fs: MockFileSystemPort { _dummy: false },
        };

        let files = FilePathList {
            values: vec![FilePath::new("/src/a.rs".to_string()).unwrap()],
        };
        let root_dir = FilePath::new("/".to_string()).unwrap();
        let mut results = LintResultList::default();

        let protocol = DefaultCycleAnalysisProtocol {};
        protocol
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert!(
            !results.values.is_empty(),
            "Should detect self circular dependency"
        );
        let result = &results.values[0];
        assert_eq!(&*result.code, "AES205");
        assert!(result.message.value().contains("/src/a.rs -> /src/a.rs"));
    }

    struct MockParserForCycle {
        imports: HashMap<String, Vec<String>>,
    }

    impl ISourceParserPort for MockParserForCycle {
        fn extract_imports(&self, path: &FilePath) -> Result<ImportInfoList, SourceParserError> {
            let mut list = ImportInfoList::new();
            if let Some(modules) = self.imports.get(path.value()) {
                for (i, module) in modules.iter().enumerate() {
                    list.push(ImportInfo::new(
                        LineNumber::new((i + 1) as i64),
                        module.clone(),
                    ));
                }
            }
            Ok(list)
        }
        fn get_raw_symbols(
            &self,
            _path: &FilePath,
        ) -> Result<crate::mcp_server::taxonomy_job_vo::ResponseData, SourceParserError> {
            todo!()
        }
        fn get_class_attributes(
            &self,
            _path: &FilePath,
        ) -> crate::mcp_server::taxonomy_job_vo::ResponseData {
            todo!()
        }
        fn has_all_export(
            &self,
            _path: &FilePath,
        ) -> crate::mcp_server::taxonomy_job_vo::SuccessStatus {
            todo!()
        }
        fn find_primitive_violations(
            &self,
            _path: &FilePath,
            _primitive_types: &crate::source_parsing::taxonomy_naming_list_vo::PrimitiveTypeList,
        ) -> crate::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList {
            todo!()
        }
        fn find_unused_imports(&self, _path: &FilePath) -> ImportInfoList {
            todo!()
        }
        fn get_class_definitions(
            &self,
            _path: &FilePath,
        ) -> Result<crate::common::taxonomy_suggestion_vo::MetadataVO, SourceParserError> {
            todo!()
        }
        fn get_function_definitions(
            &self,
            _path: &FilePath,
        ) -> crate::common::taxonomy_suggestion_vo::MetadataVO {
            todo!()
        }
        fn is_symbol_exported(
            &self,
            _path: &FilePath,
            _symbol: &crate::common::taxonomy_name_vo::SymbolName,
        ) -> crate::mcp_server::taxonomy_job_vo::SuccessStatus {
            todo!()
        }
        fn get_class_methods(
            &self,
            _path: &FilePath,
        ) -> crate::common::taxonomy_suggestion_vo::MetadataVO {
            todo!()
        }
        fn get_class_bases_map(
            &self,
            _path: &FilePath,
        ) -> crate::common::taxonomy_suggestion_vo::MetadataVO {
            todo!()
        }
        fn get_assignment_targets(
            &self,
            _path: &FilePath,
        ) -> crate::common::taxonomy_suggestion_vo::MetadataVO {
            todo!()
        }
        fn get_control_flow_count(
            &self,
            _path: &FilePath,
        ) -> crate::common::taxonomy_common_vo::Count {
            todo!()
        }
        fn is_barrel_file(&self, _path: &FilePath) -> crate::common::taxonomy_common_vo::BooleanVO {
            todo!()
        }
        fn get_stem(&self, _path: &FilePath) -> crate::common::taxonomy_name_vo::SymbolName {
            todo!()
        }
        fn is_entry_point(&self, _path: &FilePath) -> crate::common::taxonomy_common_vo::BooleanVO {
            todo!()
        }
        fn get_supported_extensions(&self) -> crate::common::taxonomy_common_vo::PatternList {
            todo!()
        }
    }

    struct MockAnalyzerForCycle {
        parser: MockParserForCycle,
    }

    impl IAnalyzer for MockAnalyzerForCycle {
        fn config(&self) -> &ArchitectureConfig {
            todo!()
        }
        fn layer_map(&self) -> &LayerMapVO {
            todo!()
        }
        fn fs(&self) -> &dyn IFileSystemPort {
            todo!()
        }
        fn parser(&self) -> &dyn ISourceParserPort {
            &self.parser
        }
        fn detect_layer(&self, _f: &FilePath, _root_dir: &FilePath) -> Option<LayerNameVO> {
            todo!()
        }
        fn detect_module_layer(&self, _module_path: &FilePath) -> Option<LayerNameVO> {
            todo!()
        }
    }

    #[tokio::test]
    async fn test_happy_path_no_cycles() {
        let mut imports = HashMap::new();
        imports.insert("/src/a.rs".to_string(), vec!["./b".to_string()]);
        imports.insert("/src/b.rs".to_string(), vec!["./c".to_string()]);
        imports.insert("/src/c.rs".to_string(), vec![]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![
                FilePath::new("/src/a.rs".to_string()).unwrap(),
                FilePath::new("/src/b.rs".to_string()).unwrap(),
                FilePath::new("/src/c.rs".to_string()).unwrap(),
            ],
        };
        let root_dir = FilePath::new("/".to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert!(
            results.values.is_empty(),
            "Expected no cycles, found: {:?}",
            results.values
        );
    }

    #[tokio::test]
    async fn test_self_cycle() {
        let mut imports = HashMap::new();
        imports.insert("/src/a.rs".to_string(), vec!["./a".to_string()]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![FilePath::new("/src/a.rs".to_string()).unwrap()],
        };
        let root_dir = FilePath::new("/".to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert_eq!(results.values.len(), 1);
        let msg = &results.values[0].message.value;
        assert!(
            msg.contains("/src/a.rs -> /src/a.rs"),
            "Expected A->A cycle, got: {}",
            msg
        );
    }

    #[tokio::test]
    async fn test_simple_cycle() {
        let mut imports = HashMap::new();
        imports.insert("/src/a.rs".to_string(), vec!["./b".to_string()]);
        imports.insert("/src/b.rs".to_string(), vec!["./a".to_string()]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![
                FilePath::new("/src/a.rs".to_string()).unwrap(),
                FilePath::new("/src/b.rs".to_string()).unwrap(),
            ],
        };
        let root_dir = FilePath::new("/".to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert_eq!(results.values.len(), 1);
        let msg = &results.values[0].message.value;
        assert!(
            msg.contains("/src/a.rs -> /src/b.rs -> /src/a.rs")
                || msg.contains("/src/b.rs -> /src/a.rs -> /src/b.rs"),
            "Got message: {}",
            msg
        );
    }

    #[tokio::test]
    async fn test_complex_cycle() {
        let mut imports = HashMap::new();
        imports.insert("/src/a.rs".to_string(), vec!["./b".to_string()]);
        imports.insert("/src/b.rs".to_string(), vec!["./c".to_string()]);
        imports.insert("/src/c.rs".to_string(), vec!["./a".to_string()]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![
                FilePath::new("/src/a.rs".to_string()).unwrap(),
                FilePath::new("/src/b.rs".to_string()).unwrap(),
                FilePath::new("/src/c.rs".to_string()).unwrap(),
            ],
        };
        let root_dir = FilePath::new("/".to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert_eq!(results.values.len(), 1);
    }

    #[tokio::test]
    async fn test_js_ts_relative_and_barrel_imports() {
        let temp_dir = std::env::temp_dir().join("js_ts_cycle_test");
        let src_dir = temp_dir.join("src");
        let components_dir = src_dir.join("components");
        let utils_dir = src_dir.join("utils");

        fs::create_dir_all(&components_dir).unwrap();
        fs::create_dir_all(&utils_dir).unwrap();

        let button_file = components_dir.join("button.ts");
        let index_file = utils_dir.join("index.ts");
        let helper_file = utils_dir.join("helper.ts");

        fs::write(&button_file, "").unwrap();
        fs::write(&index_file, "").unwrap();
        fs::write(&helper_file, "").unwrap();

        let button_str = button_file.to_string_lossy().to_string();
        let index_str = index_file.to_string_lossy().to_string();
        let helper_str = helper_file.to_string_lossy().to_string();

        let mut imports = HashMap::new();
        imports.insert(button_str.clone(), vec!["../utils".to_string()]);
        imports.insert(index_str.clone(), vec!["./helper".to_string()]);
        imports.insert(helper_str.clone(), vec!["../components/button".to_string()]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![
                FilePath::new(button_str.clone()).unwrap(),
                FilePath::new(index_str.clone()).unwrap(),
                FilePath::new(helper_str.clone()).unwrap(),
            ],
        };
        let root_dir = FilePath::new(temp_dir.to_string_lossy().to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert_eq!(
            results.values.len(),
            1,
            "Expected exactly 1 cycle, found: {:?}",
            results.values
        );
        let msg = &results.values[0].message.value;
        assert!(
            msg.contains("button.ts") && msg.contains("index.ts") && msg.contains("helper.ts"),
            "Got message: {}",
            msg
        );

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[tokio::test]
    async fn test_python_relative_imports() {
        let temp_dir = std::env::temp_dir().join("python_cycle_test");
        let pkg_dir = temp_dir.join("pkg");
        let sub_dir = pkg_dir.join("sub");

        fs::create_dir_all(&sub_dir).unwrap();

        let init_file = pkg_dir.join("__init__.py");
        let main_file = pkg_dir.join("main.py");
        let sub_init_file = sub_dir.join("__init__.py");
        let sub_module_file = sub_dir.join("module.py");

        fs::write(&init_file, "").unwrap();
        fs::write(&main_file, "").unwrap();
        fs::write(&sub_init_file, "").unwrap();
        fs::write(&sub_module_file, "").unwrap();

        let main_str = main_file.to_string_lossy().to_string();
        let sub_module_str = sub_module_file.to_string_lossy().to_string();

        let mut imports = HashMap::new();
        imports.insert(main_str.clone(), vec![".sub.module".to_string()]);
        imports.insert(sub_module_str.clone(), vec!["..main".to_string()]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![
                FilePath::new(main_str.clone()).unwrap(),
                FilePath::new(sub_module_str.clone()).unwrap(),
            ],
        };
        let root_dir = FilePath::new(temp_dir.to_string_lossy().to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert_eq!(
            results.values.len(),
            1,
            "Expected cycle, got: {:?}",
            results.values
        );

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[tokio::test]
    async fn test_rust_crate_absolute_and_super_imports() {
        let temp_dir = std::env::temp_dir().join("rust_cycle_test");
        let src_dir = temp_dir.join("src");
        let sub_dir = src_dir.join("sub");

        fs::create_dir_all(&sub_dir).unwrap();
        fs::write(temp_dir.join("Cargo.toml"), "").unwrap();

        let main_file = src_dir.join("main.rs");
        let sub_mod_file = sub_dir.join("mod.rs");
        let helper_file = sub_dir.join("helper.rs");

        fs::write(&main_file, "").unwrap();
        fs::write(&sub_mod_file, "").unwrap();
        fs::write(&helper_file, "").unwrap();

        let main_str = main_file.to_string_lossy().to_string();
        let sub_mod_str = sub_mod_file.to_string_lossy().to_string();
        let helper_str = helper_file.to_string_lossy().to_string();

        let mut imports = HashMap::new();
        imports.insert(main_str.clone(), vec!["crate::sub::helper".to_string()]);
        imports.insert(helper_str.clone(), vec!["crate::sub".to_string()]);
        imports.insert(sub_mod_str.clone(), vec!["crate::main".to_string()]);

        let analyzer = MockAnalyzerForCycle {
            parser: MockParserForCycle { imports },
        };
        let files = FilePathList {
            values: vec![
                FilePath::new(main_str.clone()).unwrap(),
                FilePath::new(helper_str.clone()).unwrap(),
                FilePath::new(sub_mod_str.clone()).unwrap(),
            ],
        };
        let root_dir = FilePath::new(temp_dir.to_string_lossy().to_string()).unwrap();
        let mut results = LintResultList::default();

        let checker = DefaultCycleAnalysisProtocol {};
        checker
            .check_cycles(&analyzer, &files, &root_dir, &mut results)
            .await;

        assert_eq!(
            results.values.len(),
            1,
            "Expected cycle, got: {:?}",
            results.values
        );

        fs::remove_dir_all(&temp_dir).unwrap();
    }
}
```

---

## File: crates/shared/src/code-analysis/mod.rs

```rust
// code-analysis — taxonomy and contract types
pub mod contract_adapter_port;
pub mod contract_bypass_checker_protocol;
pub mod contract_class_protocol;
pub mod contract_code_metric_analyzer_protocol;
pub mod contract_cycle_protocol;
pub mod contract_dead_inheritance_protocol;
pub mod contract_layer_detection_aggregate;
pub mod contract_line_protocol;
pub mod contract_lint_protocol;
pub mod taxonomy_analysis_vo;
pub mod taxonomy_code_analysis_rule_vo;
pub mod taxonomy_governance_entity;
pub mod taxonomy_import_source_vo;
pub mod taxonomy_operation_error;
pub mod taxonomy_violation_code_analysis_vo;
pub use taxonomy_violation_code_analysis_vo::{AesCodeAnalysisViolation, Language};
```

---

## File: crates/shared/src/code-analysis/taxonomy_analysis_vo.rs

```rust
// PURPOSE: FileDefinitionMap, GraphAnalysisContext, ImportGraph, InboundLinkMap, InheritanceMap — analysis value objects for code structure
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
pub struct ModuleToFileMap {
    pub mapping: std::collections::HashMap<String, String>,
}

impl ModuleToFileMap {
    pub fn new(value: std::collections::HashMap<String, String>) -> Self {
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

## File: crates/shared/src/code-analysis/taxonomy_governance_entity.rs

```rust
// PURPOSE: ArchitectureGovernanceEntity — domain entity for architecture governance (scores, issues, dates)
use serde::{Deserialize, Serialize};

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::Score;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_message_vo::ComplianceStatus;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchitectureGovernanceEntity {
    #[serde(default)]
    pub id: Identity,
    #[serde(default)]
    pub results: LintResultList,
    #[serde(default = "default_score")]
    pub score: Score,
    #[serde(default = "default_compliance")]
    pub is_passing: ComplianceStatus,
}

fn default_score() -> Score {
    Score::new(100.0)
}
fn default_compliance() -> ComplianceStatus {
    ComplianceStatus::new(true)
}

impl ArchitectureGovernanceEntity {
    pub fn new() -> Self {
        Self {
            id: Identity::new("default"),
            results: LintResultList::default(),
            score: Score::new(100.0),
            is_passing: ComplianceStatus::new(true),
        }
    }
    pub fn add_result(&mut self, result: LintResult) {
        self.score = self.score.deduct(&result.severity);
        self.results.push(result);
    }
    pub fn update_compliance(&mut self, threshold: &Score) {
        let is_p = self.score.value >= threshold.value;
        let has_critical = self
            .results
            .values
            .iter()
            .any(|r| r.severity == Severity::CRITICAL);
        self.is_passing = ComplianceStatus::new(is_p && !has_critical);
    }
    pub fn results_by_source(&self, source: &AdapterName) -> LintResultList {
        LintResultList {
            values: self
                .results
                .values
                .iter()
                .filter(|r| r.source.as_ref() == Some(source))
                .cloned()
                .collect(),
        }
    }
    pub fn violation_count(&self) -> Count {
        Count::new(
            self.results
                .values
                .iter()
                .filter(|r| r.severity.score_impact() > 0.0)
                .count() as i64,
        )
    }
}

impl Default for ArchitectureGovernanceEntity {
    fn default() -> Self {
        Self::new()
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportInfoList {
    #[serde(default)]
    pub values: Vec<ImportInfo>,
}

impl Default for ImportInfoList {
    fn default() -> Self {
        Self::new()
    }
}

impl ImportInfoList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn push(&mut self, item: ImportInfo) {
        self.values.push(item);
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrimitiveViolationList {
    #[serde(default)]
    pub values: Vec<PrimitiveViolation>,
}

impl Default for PrimitiveViolationList {
    fn default() -> Self {
        Self::new()
    }
}

impl PrimitiveViolationList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn push(&mut self, item: PrimitiveViolation) {
        self.values.push(item);
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

## File: crates/shared/src/common/mod.rs

```rust
// common — truly shared types used by multiple features
pub mod taxonomy_action_vo;
pub mod taxonomy_adapter_name_vo;
pub mod taxonomy_common_error;
pub mod taxonomy_common_vo;
pub mod taxonomy_definition_vo;
pub mod taxonomy_duration_vo;
pub mod taxonomy_error_vo;
pub mod taxonomy_job_id_vo;
pub mod taxonomy_job_vo;
pub mod taxonomy_layer_vo;
pub mod taxonomy_lint_vo;
pub mod taxonomy_message_vo;
pub mod taxonomy_name_vo;
pub mod taxonomy_response_data_vo;
pub mod taxonomy_severity_vo;
pub mod taxonomy_source_vo;
pub mod taxonomy_suggestion_vo;
```

---

## File: crates/shared/src/common/taxonomy_action_vo.rs

```rust
// PURPOSE: ActionName, ActionArgs — value objects for pipeline job actions
// JobId is re-exported from common for backward compatibility
use serde::{Deserialize, Serialize};

/* UNKNOWN: MetadataVO */
use crate::common::taxonomy_suggestion_vo::MetadataVO;

pub use crate::common::taxonomy_job_id_vo::JobId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionArgs {
    pub value: MetadataVO,
}

impl ActionArgs {
    pub fn new(value: MetadataVO) -> Self {
        Self { value }
    }
    pub fn value(&self) -> &MetadataVO {
        &self.value
    }
}

#[derive(Debug, Clone, Serialize, Eq)]
#[serde(transparent)]
pub struct ActionName {
    pub value: String,
}

impl ActionName {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for ActionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::hash::Hash for ActionName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for ActionName {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl From<&str> for ActionName {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for ActionName {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for ActionName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ActionNameVisitor {}
        impl<'de> serde::de::Visitor<'de> for ActionNameVisitor {
            type Value = ActionName;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ActionName {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ActionName { value: v })
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
                Ok(ActionName { value: val })
            }
        }
        deserializer.deserialize_any(ActionNameVisitor {})
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

#[cfg(test)]
mod tests {
    use super::AdapterName;

    #[test]
    fn test_adapter_name_new() {
        let name = AdapterName::new("ruff").unwrap_or_default();
        assert_eq!(name.value, "ruff");

        // Test trimming
        let name = AdapterName::new("  ruff  ").unwrap_or_default();
        assert_eq!(name.value, "ruff");

        // Test that internal spaces are preserved
        let name = AdapterName::new("my adapter").unwrap_or_default();
        assert_eq!(name.value, "my adapter");
    }

    #[test]
    fn test_adapter_name_invalid() {
        assert!(AdapterName::new("").is_err());
        assert!(AdapterName::new("   ").is_err());
        assert!(AdapterName::new("\t\n  ").is_err());
    }
}
```

---

## File: crates/shared/src/common/taxonomy_common_error.rs

```rust
// PURPOSE: Cause, Constraint, ErrorMessage, ExitCode, FieldName, ModuleName, PrimitiveTypeName — common error value objects
pub use crate::common::taxonomy_common_vo::ErrorMessage;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct Cause {
    pub value: String,
}

impl Cause {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for Cause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for Cause {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for Cause {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for Cause {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CauseVisitor {}
        impl<'de> serde::de::Visitor<'de> for CauseVisitor {
            type Value = Cause;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Cause {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Cause { value: v })
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
                Ok(Cause { value: val })
            }
        }
        deserializer.deserialize_any(CauseVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct Constraint {
    pub value: String,
}

impl Constraint {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for Constraint {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for Constraint {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for Constraint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ConstraintVisitor {}
        impl<'de> serde::de::Visitor<'de> for ConstraintVisitor {
            type Value = Constraint;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Constraint {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Constraint { value: v })
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
                Ok(Constraint { value: val })
            }
        }
        deserializer.deserialize_any(ConstraintVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct ExitCode {
    pub value: i64,
}

impl ExitCode {
    pub fn new(value: impl Into<i64>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for ExitCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for ExitCode {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for ExitCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ExitCodeVisitor {}
        impl<'de> serde::de::Visitor<'de> for ExitCodeVisitor {
            type Value = ExitCode;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ExitCode { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ExitCode { value: v as i64 })
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
                Ok(ExitCode { value: val })
            }
        }
        deserializer.deserialize_any(ExitCodeVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct FieldName {
    pub value: String,
}

impl FieldName {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for FieldName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for FieldName {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for FieldName {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for FieldName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FieldNameVisitor {}
        impl<'de> serde::de::Visitor<'de> for FieldNameVisitor {
            type Value = FieldName;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(FieldName {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(FieldName { value: v })
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
                Ok(FieldName { value: val })
            }
        }
        deserializer.deserialize_any(FieldNameVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct ModuleName {
    pub value: String,
}

impl ModuleName {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for ModuleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for ModuleName {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for ModuleName {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for ModuleName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ModuleNameVisitor {}
        impl<'de> serde::de::Visitor<'de> for ModuleNameVisitor {
            type Value = ModuleName;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ModuleName {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ModuleName { value: v })
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
                Ok(ModuleName { value: val })
            }
        }
        deserializer.deserialize_any(ModuleNameVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct PrimitiveTypeName {
    pub value: String,
}

impl PrimitiveTypeName {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for PrimitiveTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for PrimitiveTypeName {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for PrimitiveTypeName {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for PrimitiveTypeName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PrimitiveTypeNameVisitor {}
        impl<'de> serde::de::Visitor<'de> for PrimitiveTypeNameVisitor {
            type Value = PrimitiveTypeName;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(PrimitiveTypeName {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(PrimitiveTypeName { value: v })
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
                Ok(PrimitiveTypeName { value: val })
            }
        }
        deserializer.deserialize_any(PrimitiveTypeNameVisitor {})
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

#[derive(Debug, Clone, Serialize, PartialEq)]
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
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_layer_vo::LayerNameVO;

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LayerMapVO {
    pub values: std::collections::HashMap<LayerNameVO, LayerDefinition>,
}

impl LayerMapVO {
    pub fn new(value: std::collections::HashMap<LayerNameVO, LayerDefinition>) -> Self {
        Self { values: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct NamingConfig {
    pub word_count: Count,
}

impl NamingConfig {
    pub fn new(word_count: Count) -> Self {
        Self { word_count }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_duration_vo.rs

```rust
// PURPOSE: Duration, Timeout — value objects for duration and timeout tracking
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Duration {
    pub value: f64,
}

impl Duration {
    pub fn new(value: f64) -> Self {
        Self {
            value: value.max(0.0),
        }
    }
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}ms", self.value)
    }
}

impl<'de> serde::Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DurationVisitor {}
        impl<'de> serde::de::Visitor<'de> for DurationVisitor {
            type Value = Duration;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("number or map with 'value' key")
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Duration { value: v.max(0.0) })
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Duration {
                    value: (v as f64).max(0.0),
                })
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
                Ok(Duration {
                    value: value.unwrap_or(0.0).max(0.0),
                })
            }
        }
        deserializer.deserialize_any(DurationVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Timeout {
    pub value: f64,
}

impl Timeout {
    pub fn new(value: f64) -> Self {
        Self {
            value: value.max(0.001),
        }
    }
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl std::fmt::Display for Timeout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}s", self.value)
    }
}

impl<'de> serde::Deserialize<'de> for Timeout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TimeoutVisitor {}
        impl<'de> serde::de::Visitor<'de> for TimeoutVisitor {
            type Value = Timeout;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("number or map with 'value' key")
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Timeout {
                    value: v.max(0.001),
                })
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
                Ok(Timeout {
                    value: value.unwrap_or(30.0),
                })
            }
        }
        deserializer.deserialize_any(TimeoutVisitor {})
    }
}
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

#[cfg(test)]
mod tests {
    use super::ErrorCode;

    #[test]
    fn test_error_code_new() {
        let ec = ErrorCode::new("E123").unwrap_or_default();
        assert_eq!(ec.code, "E123");
        assert!(ec.is_style());
        assert!(!ec.is_logic());
        assert!(!ec.is_security());
        assert!(!ec.is_architecture());

        let ec = ErrorCode::new("W999").unwrap_or_default();
        assert!(ec.is_style());

        let ec = ErrorCode::new("D404").unwrap_or_default();
        assert!(ec.is_style());

        let ec = ErrorCode::new("F001").unwrap_or_default();
        assert!(ec.is_logic());

        let ec = ErrorCode::new("I999").unwrap_or_default();
        assert!(ec.is_logic());

        let ec = ErrorCode::new("B001").unwrap_or_default();
        assert!(ec.is_security());

        let ec = ErrorCode::new("AES123").unwrap_or_default();
        assert!(ec.is_architecture());
    }

    #[test]
    fn test_error_code_invalid() {
        assert!(ErrorCode::new("").is_err());
    }
}
```

---

## File: crates/shared/src/common/taxonomy_job_id_vo.rs

```rust
// PURPOSE: JobId — value object for pipeline job identifiers
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(transparent)]
pub struct JobId {
    pub value: String,
}

impl JobId {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for JobId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::hash::Hash for JobId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for JobId {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for JobId {}

impl From<&str> for JobId {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for JobId {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for JobId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct JobIdVisitor {}
        impl<'de> serde::de::Visitor<'de> for JobIdVisitor {
            type Value = JobId;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(JobId {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(JobId { value: v })
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
                Ok(JobId { value: val })
            }
        }
        deserializer.deserialize_any(JobIdVisitor {})
    }
}
```

---

## File: crates/shared/src/common/taxonomy_job_vo.rs

```rust
// PURPOSE: PipelineJob, SuccessStatus, EnvContentVO, McpConfigVO — value objects for pipeline job lifecycle tracking
// ResponseData is re-exported from common for backward compatibility
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::common::taxonomy_adapter_name_vo::AdapterName;

pub use crate::common::taxonomy_response_data_vo::ResponseData;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum JobStatus {
    #[serde(rename = "pending")]
    PENDING,
    #[serde(rename = "running")]
    RUNNING,
    #[serde(rename = "completed")]
    COMPLETED,
    #[serde(rename = "failed")]
    FAILED,
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::PENDING => write!(f, "pending"),
            JobStatus::RUNNING => write!(f, "running"),
            JobStatus::COMPLETED => write!(f, "completed"),
            JobStatus::FAILED => write!(f, "failed"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SuccessStatus {
    pub value: bool,
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
pub struct LintStatusActionArgs {
    #[serde(default)]
    pub value: HashMap<String, serde_json::Value>,
}

impl Default for LintStatusActionArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl LintStatusActionArgs {
    pub fn new() -> Self {
        Self {
            value: HashMap::new(),
        }
    }
    pub fn value(&self) -> &HashMap<String, serde_json::Value> {
        &self.value
    }
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.value.get(key)
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnvContentVO {
    pub value: String,
}

impl EnvContentVO {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for EnvContentVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

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

## File: crates/shared/src/common/taxonomy_layer_vo.rs

```rust
// PURPOSE: FileContentVO, Identity, LayerNameVO, LineContentVO — VOs for layer identity and file content
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Eq)]
#[serde(transparent)]
pub struct FileContentVO {
    pub value: String,
}

impl FileContentVO {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for FileContentVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::hash::Hash for FileContentVO {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for FileContentVO {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl From<&str> for FileContentVO {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for FileContentVO {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for FileContentVO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FileContentVOVisitor {}
        impl<'de> serde::de::Visitor<'de> for FileContentVOVisitor {
            type Value = FileContentVO;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(FileContentVO {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(FileContentVO { value: v })
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
                Ok(FileContentVO { value: val })
            }
        }
        deserializer.deserialize_any(FileContentVOVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, Eq)]
#[serde(transparent)]
#[derive(Default)]
pub struct Identity {
    pub value: String,
}

impl Identity {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for Identity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::hash::Hash for Identity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for Identity {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl From<&str> for Identity {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for Identity {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for Identity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct IdentityVisitor {}
        impl<'de> serde::de::Visitor<'de> for IdentityVisitor {
            type Value = Identity;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Identity {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Identity { value: v })
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
                Ok(Identity { value: val })
            }
        }
        deserializer.deserialize_any(IdentityVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Default)]
#[serde(transparent)]
pub struct LayerNameVO {
    pub value: String,
}

impl LayerNameVO {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for LayerNameVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::hash::Hash for LayerNameVO {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for LayerNameVO {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl From<&str> for LayerNameVO {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for LayerNameVO {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

#[derive(Debug, Clone, Serialize, Eq)]
#[serde(transparent)]
pub struct LineContentVO {
    pub value: String,
}

impl LineContentVO {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for LineContentVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::hash::Hash for LineContentVO {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for LineContentVO {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl From<&str> for LineContentVO {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for LineContentVO {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for LineContentVO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct LineContentVOVisitor {}
        impl<'de> serde::de::Visitor<'de> for LineContentVOVisitor {
            type Value = LineContentVO;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LineContentVO {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LineContentVO { value: v })
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
                Ok(LineContentVO { value: val })
            }
        }
        deserializer.deserialize_any(LineContentVOVisitor {})
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
use crate::common::taxonomy_source_vo::ContentString;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;

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
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ComplianceStatus {
    pub value: bool,
}

impl ComplianceStatus {
    pub fn value(&self) -> bool {
        self.value
    }
    pub fn new(value: bool) -> Self {
        Self { value }
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

impl<'de> serde::Deserialize<'de> for ComplianceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ComplianceStatusVisitor {}
        impl<'de> serde::de::Visitor<'de> for ComplianceStatusVisitor {
            type Value = ComplianceStatus;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ComplianceStatus { value: v })
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
                Ok(ComplianceStatus { value: val })
            }
        }
        deserializer.deserialize_any(ComplianceStatusVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct LintMessage {
    pub value: String,
}

impl LintMessage {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for LintMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for LintMessage {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for LintMessage {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for LintMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct LintMessageVisitor {}
        impl<'de> serde::de::Visitor<'de> for LintMessageVisitor {
            type Value = LintMessage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LintMessage {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LintMessage { value: v })
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
                Ok(LintMessage { value: val })
            }
        }
        deserializer.deserialize_any(LintMessageVisitor {})
    }
}
```

---

## File: crates/shared/src/common/taxonomy_name_vo.rs

```rust
// PURPOSE: NameVariants, SymbolName — value objects for symbol naming and naming convention variants
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

#[derive(Debug, Clone, Serialize, Eq)]
#[serde(transparent)]
pub struct SymbolName {
    pub value: String,
}

impl SymbolName {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for SymbolName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::hash::Hash for SymbolName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for SymbolName {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl From<&str> for SymbolName {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for SymbolName {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for SymbolName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SymbolNameVisitor {}
        impl<'de> serde::de::Visitor<'de> for SymbolNameVisitor {
            type Value = SymbolName;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(SymbolName {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(SymbolName { value: v })
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
                Ok(SymbolName { value: val })
            }
        }
        deserializer.deserialize_any(SymbolNameVisitor {})
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
use serde::{Deserialize, Serialize};

use crate::source_parsing::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Serialize, Eq)]
#[serde(transparent)]
pub struct ContentString {
    pub value: String,
}

impl ContentString {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for ContentString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::hash::Hash for ContentString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for ContentString {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl From<&str> for ContentString {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for ContentString {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for ContentString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ContentStringVisitor {}
        impl<'de> serde::de::Visitor<'de> for ContentStringVisitor {
            type Value = ContentString;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ContentString {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ContentString { value: v })
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
                Ok(ContentString { value: val })
            }
        }
        deserializer.deserialize_any(ContentStringVisitor {})
    }
}

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
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct ClassPath {
    pub value: String,
}

impl ClassPath {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for ClassPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for ClassPath {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for ClassPath {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for ClassPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ClassPathVisitor {}
        impl<'de> serde::de::Visitor<'de> for ClassPathVisitor {
            type Value = ClassPath;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ClassPath {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ClassPath { value: v })
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
                Ok(ClassPath { value: val })
            }
        }
        deserializer.deserialize_any(ClassPathVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct DescriptionVO {
    pub value: String,
}

impl DescriptionVO {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for DescriptionVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for DescriptionVO {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for DescriptionVO {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for DescriptionVO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DescriptionVOVisitor {}
        impl<'de> serde::de::Visitor<'de> for DescriptionVOVisitor {
            type Value = DescriptionVO;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(DescriptionVO {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(DescriptionVO { value: v })
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
                Ok(DescriptionVO { value: val })
            }
        }
        deserializer.deserialize_any(DescriptionVOVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct LogOutput {
    pub value: String,
}

impl LogOutput {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for LogOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for LogOutput {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for LogOutput {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for LogOutput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct LogOutputVisitor {}
        impl<'de> serde::de::Visitor<'de> for LogOutputVisitor {
            type Value = LogOutput;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LogOutput {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LogOutput { value: v })
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
                Ok(LogOutput { value: val })
            }
        }
        deserializer.deserialize_any(LogOutputVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetadataVO {
    pub value: std::collections::HashMap<String, serde_json::Value>,
}

impl MetadataVO {
    pub fn new(value: std::collections::HashMap<String, serde_json::Value>) -> Self {
        Self { value }
    }
    pub fn value(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.value
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct StdError {
    pub value: String,
}

impl StdError {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for StdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for StdError {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for StdError {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for StdError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StdErrorVisitor {}
        impl<'de> serde::de::Visitor<'de> for StdErrorVisitor {
            type Value = StdError;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(StdError {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(StdError { value: v })
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
                Ok(StdError { value: val })
            }
        }
        deserializer.deserialize_any(StdErrorVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct StdOutput {
    pub value: String,
}

impl StdOutput {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for StdOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for StdOutput {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for StdOutput {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for StdOutput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StdOutputVisitor {}
        impl<'de> serde::de::Visitor<'de> for StdOutputVisitor {
            type Value = StdOutput;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(StdOutput {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(StdOutput { value: v })
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
                Ok(StdOutput { value: val })
            }
        }
        deserializer.deserialize_any(StdOutputVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
pub struct Suggestion {
    pub value: String,
}

impl Suggestion {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for Suggestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for Suggestion {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for Suggestion {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for Suggestion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SuggestionVisitor {}
        impl<'de> serde::de::Visitor<'de> for SuggestionVisitor {
            type Value = Suggestion;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Suggestion {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Suggestion { value: v })
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
                Ok(Suggestion { value: val })
            }
        }
        deserializer.deserialize_any(SuggestionVisitor {})
    }
}
```

---

## File: crates/shared/src/config-system/contract_parser_port.rs

```rust
// PURPOSE: IConfigParserPort — contract for config parser provider (YAML and TOML)
use crate::config_system::taxonomy_config_error::ConfigError;
use crate::config_system::taxonomy_setting_vo::ProjectConfig;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IConfigParserPort: Send + Sync {
    fn parse_yaml_config(&self, path: &FilePath) -> Result<ProjectConfig, ConfigError>;
    fn parse_toml_config(&self, path: &FilePath) -> Option<Result<ProjectConfig, ConfigError>>;
}
```

---

## File: crates/shared/src/config-system/mod.rs

```rust
// config-system — taxonomy and contract types
pub mod contract_orchestration_aggregate;
pub mod contract_parser_port;
pub mod contract_reader_port;
pub mod contract_validator_protocol;
pub mod contract_workspace_detector_port;
pub mod taxonomy_adapter_vo;
pub mod taxonomy_app_vo;
pub mod taxonomy_config_error;
pub mod taxonomy_config_vo;
pub mod taxonomy_identifier_vo;
pub mod taxonomy_setting_vo;
pub mod taxonomy_source_vo;
pub mod taxonomy_validation_vo;
```

---

## File: crates/shared/src/config-system/taxonomy_adapter_vo.rs

```rust
// PURPOSE: AdapterClassMap, AdapterMetadataList, AdapterNameList — VOs for adapter registration metadata
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::mcp_server::taxonomy_job_vo::AdapterMetadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterMetadataList {
    #[serde(default)]
    pub values: Vec<AdapterMetadata>,
}

impl Default for AdapterMetadataList {
    fn default() -> Self {
        Self::new()
    }
}

impl AdapterMetadataList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn push(&mut self, item: AdapterMetadata) {
        self.values.push(item);
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl std::ops::Deref for AdapterMetadataList {
    type Target = Vec<AdapterMetadata>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterNameList {
    #[serde(default)]
    pub values: Vec<AdapterName>,
}

impl Default for AdapterNameList {
    fn default() -> Self {
        Self::new()
    }
}

impl AdapterNameList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn push(&mut self, item: AdapterName) {
        self.values.push(item);
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl std::ops::Deref for AdapterNameList {
    type Target = Vec<AdapterName>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterClassMap {
    #[serde(default)]
    pub values: std::collections::HashMap<String, String>,
}

impl Default for AdapterClassMap {
    fn default() -> Self {
        Self::new()
    }
}

impl AdapterClassMap {
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
        }
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
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

## File: crates/shared/src/config-system/taxonomy_config_error.rs

```rust
// PURPOSE: ConfigError, ConfigErrorKind — structured error types for configuration loading failures
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::config_system::taxonomy_identifier_vo::ConfigKey;
use crate::config_system::taxonomy_setting_vo::ActualValue;
use crate::config_system::taxonomy_setting_vo::ExpectedValue;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, thiserror::Error)]
pub struct ConfigError {
    pub key: ConfigKey,
    pub message: ErrorMessage,
    pub expected: ExpectedValue,
    pub actual: ActualValue,
    pub config_file: FilePath,
}

impl ConfigError {
    pub fn new(key: ConfigKey, message: ErrorMessage) -> Self {
        Self {
            key,
            message,
            expected: ExpectedValue::default(),
            actual: ActualValue::default(),
            config_file: FilePath::default(),
        }
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file_str = self.config_file.to_string();
        let file_info = if file_str.is_empty() {
            String::new()
        } else {
            format!(" in {}", file_str)
        };
        write!(
            f,
            "Config error on '{}'{}: {}",
            self.key, file_info, self.message
        )
    }
}
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
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use std::collections::HashMap;

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
    let raw: serde_yaml::Value = serde_yaml::from_str(yaml_str).unwrap_or_default();
    if let Some(arch_val) = raw.get("architecture") {
        let mut arch_json = serde_json::to_value(arch_val).unwrap_or_default();
        // Extract layers from rules.AES102.layers if not at top-level layers
        if arch_json
            .get("rules")
            .and_then(|r| r.get("AES102"))
            .and_then(|a| a.get("layers"))
            .is_some()
            && arch_json.get("layers").is_none()
        {
            if let Some(rules_obj) = arch_json.get_mut("rules").and_then(|r| r.as_object_mut()) {
                if let Some(aes102) = rules_obj.get_mut("AES102").and_then(|a| a.as_object_mut()) {
                    if let Some(layers) = aes102.remove("layers") {
                        arch_json["layers"] = layers;
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
                println!("[debug] serde_json from_value error: {:?}", e);
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
pub fn default_aes_config() -> ArchitectureConfig {
    parse_config_yaml(include_str!("../../../../lint_arwaky.config.rust.yaml"))
}

pub fn default_config_for_language(language: &str) -> ArchitectureConfig {
    match language {
        "python" => parse_config_yaml(include_str!("../../../../lint_arwaky.config.python.yaml")),
        "javascript" | "typescript" => parse_config_yaml(include_str!(
            "../../../../lint_arwaky.config.javascript.yaml"
        )),
        _ => default_aes_config(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_parsing() {
        let config = default_config_for_language("typescript");
        println!("typescript layers: {:?}", config.layers.keys());
        assert!(!config.layers.is_empty());
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_identifier_vo.rs

```rust
// PURPOSE: ConfigIdentifier — value object for named configuration identifiers
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Eq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ConfigKey {
    pub value: String,
}

impl ConfigKey {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn parts(&self) -> Vec<String> {
        self.value.split('.').map(|s| s.to_string()).collect()
    }
    pub fn parent(&self) -> String {
        let parts = self.parts();
        if parts.len() > 1 {
            parts[..parts.len() - 1].join(".")
        } else {
            String::new()
        }
    }
    pub fn leaf(&self) -> String {
        self.parts()
            .last()
            .cloned()
            .unwrap_or_else(|| self.value.clone())
    }
}

impl std::fmt::Display for ConfigKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::hash::Hash for ConfigKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for ConfigKey {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl From<&str> for ConfigKey {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for ConfigKey {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for ConfigKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ConfigKeyVisitor {}
        impl<'de> serde::de::Visitor<'de> for ConfigKeyVisitor {
            type Value = ConfigKey;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ConfigKey {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ConfigKey { value: v })
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
                Ok(ConfigKey { value: val })
            }
        }
        deserializer.deserialize_any(ConfigKeyVisitor {})
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_setting_vo.rs

```rust
// PURPOSE: SettingsConfigVO — value object for application-wide settings configuration

use serde::{Deserialize, Serialize};

use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_common_vo::Score;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ActualValue {
    pub value: String,
}

impl ActualValue {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for ActualValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for ActualValue {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for ActualValue {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for ActualValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ActualValueVisitor {}
        impl<'de> serde::de::Visitor<'de> for ActualValueVisitor {
            type Value = ActualValue;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive string or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ActualValue {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ActualValue { value: v })
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
                Ok(ActualValue { value: val })
            }
        }
        deserializer.deserialize_any(ActualValueVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ExpectedValue {
    pub value: String,
}

impl ExpectedValue {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for ExpectedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for ExpectedValue {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for ExpectedValue {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for ExpectedValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ExpectedValueVisitor {}
        impl<'de> serde::de::Visitor<'de> for ExpectedValueVisitor {
            type Value = ExpectedValue;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive string or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ExpectedValue {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ExpectedValue { value: v })
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
                Ok(ExpectedValue { value: val })
            }
        }
        deserializer.deserialize_any(ExpectedValueVisitor {})
    }
}

/// Scoring thresholds.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Thresholds {
    pub score: Score,
    pub complexity: Count,
    pub max_file_lines: Count,
}

impl Thresholds {
    pub fn new(score: Score, complexity: Count, max_file_lines: Count) -> Self {
        Self {
            score,
            complexity,
            max_file_lines,
        }
    }
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            score: Score::new(80.0),
            complexity: Count::new(10),
            max_file_lines: Count::new(500),
        }
    }
}

/// Adapter status enum.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum AdapterStatus {
    #[default]
    Enabled,
    Disabled,
    NotInstalled,
}

impl AdapterStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AdapterStatus::Enabled => "enabled",
            AdapterStatus::Disabled => "disabled",
            AdapterStatus::NotInstalled => "not_installed",
        }
    }
}

impl std::fmt::Display for AdapterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Single adapter configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterEntry {
    pub name: AdapterName,
    #[serde(default)]
    pub status: AdapterStatus,
    #[serde(default = "default_weight")]
    pub weight: f64,
}

fn default_weight() -> f64 {
    1.0
}

impl AdapterEntry {
    pub fn new(name: AdapterName, status: AdapterStatus, weight: f64) -> Self {
        Self {
            name,
            status,
            weight,
        }
    }

    pub fn enabled(name: AdapterName) -> Self {
        Self::new(name, AdapterStatus::Enabled, 1.0)
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, AdapterStatus::Enabled)
    }
}

/// Project configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ProjectConfig {
    #[serde(default = "default_project_name")]
    pub project_name: DescriptionVO,
    #[serde(default)]
    pub thresholds: Thresholds,
    #[serde(default)]
    pub adapters: Vec<AdapterEntry>,
    #[serde(default)]
    pub ignored_paths: FilePathList,
    #[serde(default)]
    pub ignored_rules: PatternList,
    #[serde(default)]
    pub layer_map: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub output_dir: Option<DirectoryPath>,
    #[serde(default)]
    pub architecture: ArchitectureConfig,
}

fn default_project_name() -> DescriptionVO {
    DescriptionVO::new("lint-arwaky")
}

impl ProjectConfig {
    /// Returns a ProjectConfig with default linter adapters enabled.
    pub fn defaults() -> Self {
        Self {
            project_name: default_project_name(),
            thresholds: Thresholds::default(),
            adapters: vec![
                AdapterEntry::enabled(AdapterName::raw("ruff")),
                AdapterEntry::enabled(AdapterName::raw("mypy")),
                AdapterEntry::enabled(AdapterName::raw("bandit")),
                AdapterEntry::enabled(AdapterName::raw("radon")),
            ],
            ignored_paths: FilePathList::default(),
            ignored_rules: PatternList::default(),
            layer_map: std::collections::HashMap::new(),
            output_dir: None,
            architecture: ArchitectureConfig::default(),
        }
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_source_vo.rs

```rust
// PURPOSE: ConfigResult, ConfigSource for config-system
pub use crate::common::taxonomy_source_vo::ContentString;
pub use crate::common::taxonomy_source_vo::SourceContentVO;

use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

/// Represents a configuration source with its language, path, and raw content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfigSource {
    pub language: String,
    pub path: FilePath,
    pub raw_content: String,
}

impl ConfigSource {
    pub fn new(
        language: impl Into<String>,
        path: impl Into<String>,
        raw_content: impl Into<String>,
    ) -> Self {
        Self {
            language: language.into(),
            path: FilePath::new(path.into()).unwrap_or_default(),
            raw_content: raw_content.into(),
        }
    }
}

/// Result type for config loading operations containing the parsed config, source info, and warnings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfigResult {
    pub config: ArchitectureConfig,
    pub source: ConfigSource,
    pub warnings: Vec<String>,
}

impl ConfigResult {
    pub fn new(config: ArchitectureConfig, source: ConfigSource, warnings: Vec<String>) -> Self {
        Self {
            config,
            source,
            warnings,
        }
    }
}
```

---

## File: crates/shared/src/file-system/contract_system_port.rs

```rust
// PURPOSE: IFileSystemPort — port trait for filesystem operations (read, write, exists, glob, walk)

use async_trait::async_trait;

use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_source_vo::ContentString;
use crate::file_system::taxonomy_filesystem_error::FileSystemError;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

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

## File: crates/shared/src/file-system/mod.rs

```rust
// file-system — taxonomy and contract types
pub mod contract_system_port;
pub mod taxonomy_filesystem_error;
```

---

## File: crates/shared/src/file-system/taxonomy_filesystem_error.rs

```rust
// PURPOSE: FileSystemError — structured error type for filesystem operation failures
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::mcp_server::taxonomy_action_vo::ActionName;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
        let code = {
            let c: &str = &self.error_code;
            if c.is_empty() {
                String::new()
            } else {
                format!(" [{}]", c)
            }
        };
        write!(
            f,
            "FS Error during {} on {}{}: {}",
            self.operation, self.path, code, self.message
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct PathNotFoundError {
    #[serde(flatten)]
    pub base: FileSystemError,
}

impl PathNotFoundError {
    pub fn new(path: FilePath, message: ErrorMessage) -> Self {
        Self {
            base: FileSystemError::new(path, message, ActionName::new("read")),
        }
    }
}

impl std::fmt::Display for PathNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Path not found: {} ({})",
            self.base.path, self.base.message
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct AccessDeniedError {
    #[serde(flatten)]
    pub base: FileSystemError,
}

impl AccessDeniedError {
    pub fn new(path: FilePath, message: ErrorMessage) -> Self {
        Self {
            base: FileSystemError::new(path, message, ActionName::new("access")),
        }
    }
}

impl std::fmt::Display for AccessDeniedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Access denied: {} ({})",
            self.base.path, self.base.message
        )
    }
}
```

---

## File: crates/shared/src/file-watch/mod.rs

```rust
// file-watch — taxonomy and contract types
pub mod contract_provider_port;
pub mod contract_watch_aggregate;
pub mod taxonomy_diff_result_vo;
pub mod taxonomy_result_vo;
pub mod taxonomy_service_error;
pub mod taxonomy_watch_config_vo;
pub mod taxonomy_watch_event_vo;
pub mod taxonomy_watch_vo;
```

---

## File: crates/shared/src/file-watch/taxonomy_diff_result_vo.rs

```rust
// PURPOSE: GitDiffResultVO — value object representing git diff results
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::Count;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use crate::source_parsing::taxonomy_paths_vo::RenamedFileList;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitDiffResultVO {
    pub added: FilePathList,
    pub modified: FilePathList,
    pub deleted: FilePathList,
    pub renamed: RenamedFileList,
    pub lintable_files: FilePathList,
    pub all_files: FilePathList,
    pub total_changed: Count,
}

impl GitDiffResultVO {
    pub fn new(
        added: FilePathList,
        modified: FilePathList,
        deleted: FilePathList,
        renamed: RenamedFileList,
        lintable_files: FilePathList,
        all_files: FilePathList,
        total_changed: Count,
    ) -> Self {
        Self {
            added,
            modified,
            deleted,
            renamed,
            lintable_files,
            all_files,
            total_changed,
        }
    }
}
```

---

## File: crates/shared/src/file-watch/taxonomy_result_vo.rs

```rust
// PURPOSE: WatchResult — result type for watch operations
use serde::{Deserialize, Serialize};

use crate::file_watch::taxonomy_service_error::WatchServiceError;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub enum WatchResult {
    #[default]
    Started,
    Stopped,
    Changed(Vec<String>),
    Error(WatchServiceError),
}
```

---

## File: crates/shared/src/file-watch/taxonomy_service_error.rs

```rust
// PURPOSE: WatchServiceError — structured error type for file watch service failures
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::taxonomy_common_error::ErrorMessage;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WatchServiceError {
    pub path: FilePath,
    pub message: String,
}

impl WatchServiceError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            path: FilePath::default(),
            message: message.value,
        }
    }
}

impl std::fmt::Display for WatchServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Watch Error on {}: {}", self.path.value, self.message)
    }
}

impl std::error::Error for WatchServiceError {}

#[derive(Debug, Clone)]
pub struct WatchSubscriptionError(pub WatchServiceError);

#[derive(Debug, Clone)]
pub struct WatchEventError(pub WatchServiceError);
```

---

## File: crates/shared/src/file-watch/taxonomy_watch_event_vo.rs

```rust
// PURPOSE: WatchEvent — value object representing a filesystem change event
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WatchEventKind {
    Created,
    Modified,
    Removed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WatchEvent {
    pub path: String,
    pub kind: WatchEventKind,
    pub timestamp_ms: u64,
}

impl WatchEvent {
    pub fn new(path: String, kind: WatchEventKind) -> Self {
        let timestamp_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        Self {
            path,
            kind,
            timestamp_ms,
        }
    }
}
```

---

## File: crates/shared/src/git-hooks/taxonomy_hook_error.rs

```rust
// PURPOSE: GitHookError — structured error type for git hook operation failures
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone)]
pub struct GitHookError {
    pub path: FilePath,
    pub message: String,
}

impl GitHookError {
    pub fn new(message: crate::taxonomy_common_error::ErrorMessage) -> Self {
        Self {
            path: FilePath::default(),
            message: message.value,
        }
    }
}

impl std::fmt::Display for GitHookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Git Hook Error on {}: {}", self.path.value, self.message)
    }
}

impl std::error::Error for GitHookError {}
```

---

## File: crates/shared/src/git-hooks/taxonomy_installed_event.rs

```rust
// PURPOSE: HookInstalled — domain event published when a git hook is installed
use crate::common::taxonomy_common_vo::Timestamp;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HookInstalled {
    pub path: FilePath,
    pub executable: FilePath,
    #[serde(default)]
    pub timestamp: Timestamp,
}

impl HookInstalled {
    pub fn new(path: FilePath, executable: FilePath) -> Self {
        Self {
            path,
            executable,
            timestamp: Timestamp::default(),
        }
    }
}
```

---

## File: crates/shared/src/git-hooks/taxonomy_ref_vo.rs

```rust
// PURPOSE: GitRefVO — value object for git reference (branch, tag)
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct GitRef {
    pub value: String,
}

impl GitRef {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for GitRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for GitRef {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for GitRef {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for GitRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct GitRefVisitor {}
        impl<'de> serde::de::Visitor<'de> for GitRefVisitor {
            type Value = GitRef;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(GitRef {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(GitRef { value: v })
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
                Ok(GitRef {
                    value: value.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_any(GitRefVisitor {})
    }
}
```

---

## File: crates/shared/src/git-hooks/taxonomy_removed_event.rs

```rust
// PURPOSE: HookRemoved — domain event published when a git hook is removed
use crate::common::taxonomy_common_vo::Timestamp;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HookRemoved {
    pub path: FilePath,
    #[serde(default)]
    pub timestamp: Timestamp,
}

impl HookRemoved {
    pub fn new(path: FilePath) -> Self {
        Self {
            path,
            timestamp: Timestamp::default(),
        }
    }
}
```

---

## File: crates/shared/src/import-rules/contract_import_parser_port.rs

```rust
// PURPOSE: IImportParserPort — contract port trait for import parsing utilities
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use crate::import_rules::taxonomy_language_vo::LanguageVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::taxonomy_common_vo::LineNumber;
use crate::taxonomy_layer_vo::FileContentVO;
use crate::taxonomy_layer_vo::Identity;
use crate::taxonomy_layer_vo::LayerNameVO;
use crate::taxonomy_layer_vo::LineContentVO;
use crate::taxonomy_name_vo::SymbolName;

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
    fn read_file_to_string(&self, file: &FilePath) -> Result<String, std::io::Error>;
    fn extract_import_modules(&self, content: &str) -> Vec<String>;
    fn get_language_from_path(&self, path: &str) -> LanguageVO;
    fn get_dummy_function_ranges(&self, lines: &[&str], lang: LanguageVO) -> Vec<(usize, usize)>;
    fn get_imported_symbols(&self, lines: &[&str], lang: LanguageVO) -> Vec<(String, usize)>;
    fn get_dummy_impl_traits_with_lines(&self, lines: &[&str]) -> Vec<(String, usize)>;
    fn is_symbol_used_real(
        &self,
        lines: &[&str],
        symbol: &str,
        dummy_ranges: &[(usize, usize)],
        dummy_impl_traits: &[String],
    ) -> bool;
    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName>;

    // Fine-grained parsing utilities for unused import steps
    fn extract_imported_aliases(&self, content: &str) -> std::collections::HashMap<String, String>;
    fn extract_exported_symbols(&self, content: &str) -> std::collections::HashSet<String>;
    fn extract_used_symbols(
        &self,
        content: &str,
        imported_aliases: &std::collections::HashMap<String, String>,
    ) -> std::collections::HashSet<String>;
    fn find_import_line_number(&self, content: &str, alias: &str) -> usize;
    fn extract_rust_js_imports(&self, content: &str) -> Vec<(String, usize)>;
    fn is_name_used(&self, name: &str, content: &str, exclude_line: usize) -> bool;
}
```

---

## File: crates/shared/src/import-rules/contract_import_runner_aggregate.rs

```rust
// PURPOSE: IImportRunnerAggregate — contract for import-rules feature orchestrator
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait IImportRunnerAggregate: Send + Sync {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
    fn name(&self) -> &str;
}
```

---

## File: crates/shared/src/import-rules/contract_rule_protocol.rs

```rust
// PURPOSE: IAnalyzer trait — core analyzer interface for import checks
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::file_system::contract_system_port::IFileSystemPort;
use crate::source_parsing::contract_parser_port::ISourceParserPort;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use crate::taxonomy_common_error::ErrorMessage;
use crate::taxonomy_common_vo::Count;
use crate::taxonomy_common_vo::PatternList;
use crate::taxonomy_definition_vo::LayerMapVO;
use crate::taxonomy_layer_vo::Identity;
use crate::taxonomy_layer_vo::LayerNameVO;

pub trait IAnalyzer: Send + Sync {
    fn config(&self) -> &ArchitectureConfig;
    fn layer_map(&self) -> &LayerMapVO;
    fn fs(&self) -> &dyn IFileSystemPort;
    fn parser(&self) -> &dyn ISourceParserPort;
    fn detect_layer(&self, f: &FilePath, root_dir: &FilePath) -> Option<LayerNameVO>;
    fn detect_module_layer(&self, module_path: &FilePath) -> Option<LayerNameVO>;
}

pub trait IArchRuleProtocol {
    fn rule_name(&self) -> Identity;
}

#[async_trait::async_trait]
pub trait INamingCheckerProtocol: Send + Sync {
    async fn check_file_naming(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    async fn check_domain_suffixes(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

pub trait IInternalCheckerProtocol: Send + Sync {
    fn check_layer_internal_rules(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

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

pub struct CheckFileNamingParams<'a> {
    pub files: &'a FilePathList,
    pub root_dir: &'a FilePath,
    pub layer_map: &'a LayerMapVO,
    pub global_expected: Count,
    pub global_exceptions: &'a PatternList,
    pub results: &'a mut LintResultList,
    pub detect_layer_fn: &'a dyn Fn(&FilePath, &FilePath) -> Option<LayerNameVO>,
}

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

#[async_trait::async_trait]
pub trait IArchImportProtocol: IArchRuleProtocol + Send + Sync {
    async fn check_mandatory_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
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
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IUnusedImportProtocol: Send + Sync {
    /// Find unused imports in a file by path (reads file internally)
    fn find_unused_imports(&self, path: &FilePath) -> Vec<String>;

    /// Check unused imports given file content directly (for inline checking)
    /// Useful when content is already available (avoids re-reading file)
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
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use crate::taxonomy_name_vo::SymbolName;
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
    let base = name.rsplit('/').next().unwrap_or(name);
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
use crate::import_rules::taxonomy_language_vo::LanguageVO;

pub fn dummy_function_ranges(lines: &[&str], lang: LanguageVO) -> Vec<(usize, usize)> {
    match lang {
        LanguageVO::Rust => rust_dummy_function_ranges(lines),
        LanguageVO::Python => python_dummy_function_ranges(lines),
        LanguageVO::JavaScript => js_dummy_function_ranges(lines),
        LanguageVO::Unknown => Vec::new(),
    }
}

pub fn imported_symbols(lines: &[&str], lang: LanguageVO) -> Vec<(String, usize)> {
    match lang {
        LanguageVO::Rust => rust_imported_symbols(lines),
        LanguageVO::Python => python_imported_symbols(lines),
        LanguageVO::JavaScript => js_imported_symbols(lines),
        LanguageVO::Unknown => Vec::new(),
    }
}

pub fn dummy_impl_traits_with_lines(lines: &[&str]) -> Vec<(String, usize)> {
    let mut traits = Vec::new();
    let mut i = 0usize;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("impl ") && trimmed.contains(" for ") {
            if let Some(trait_name) = impl_trait_name(trimmed) {
                let (end, body_lines) = impl_block(lines, i);
                if trait_impl_is_dummy(&body_lines) {
                    traits.push((trait_name, i + 1));
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
        && symbol.chars().nth(1).unwrap_or(' ').is_uppercase())
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

fn rust_dummy_function_ranges(lines: &[&str]) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("fn _use_") || trimmed.starts_with("fn dummy_") {
            let start = i + 1;
            let mut depth = 0usize;
            let mut end = i + 1;

            for (idx, line) in lines.iter().enumerate().skip(i) {
                let t = line.trim();
                depth = depth.saturating_add(t.matches('{').count());
                depth = depth.saturating_sub(t.matches('}').count());
                end = idx + 1;
                if depth == 0 && t.contains('}') {
                    break;
                }
            }

            ranges.push((start, end));
            i = end;
        }
        i += 1;
    }

    ranges
}

fn python_dummy_function_ranges(lines: &[&str]) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("def _use_") || trimmed.starts_with("def dummy_") {
            let start = i + 1;
            let mut end = i + 1;
            let indent = lines[i].len() - lines[i].trim_start().len();

            for (idx, line) in lines.iter().enumerate().skip(i + 1) {
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

            ranges.push((start, end));
            i = end;
        }
        i += 1;
    }

    ranges
}

fn js_dummy_function_ranges(lines: &[&str]) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("function _use")
            || trimmed.starts_with("function dummy")
            || trimmed.starts_with("const _use")
            || trimmed.starts_with("const dummy")
        {
            let start = i + 1;
            let mut depth = 0usize;
            let mut end = i + 1;

            for (idx, line) in lines.iter().enumerate().skip(i) {
                let t = line.trim();
                depth = depth.saturating_add(t.matches('{').count());
                depth = depth.saturating_sub(t.matches('}').count());
                end = idx + 1;
                if depth == 0 && t.contains('}') {
                    break;
                }
            }

            ranges.push((start, end));
            i = end;
        }
        i += 1;
    }

    ranges
}

fn rust_imported_symbols(lines: &[&str]) -> Vec<(String, usize)> {
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
                            symbols.push((symbol, idx + 1));
                        }
                    }
                }
            }
            continue;
        }

        if let Some(symbol) = rust_imported_symbol_from_part(body) {
            symbols.push((symbol, idx + 1));
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

    let name = part.split("::").last().unwrap_or(part).trim();
    if name.is_empty() || name.contains('{') || name.contains('}') {
        return None;
    }

    Some(name.to_string())
}

fn python_imported_symbols(lines: &[&str]) -> Vec<(String, usize)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("from ") && trimmed.contains(" import ") {
            if let Some(import_part) = trimmed.split_once(" import ").map(|(_, p)| p) {
                for name in import_part.split(',') {
                    let name = name.split_whitespace().next().unwrap_or("");
                    if !name.is_empty() && name != "*" {
                        symbols.push((name.to_string(), idx + 1));
                    }
                }
            }
            continue;
        }

        if trimmed.starts_with("import ") {
            let module = trimmed
                .trim_start_matches("import ")
                .split_whitespace()
                .next()
                .unwrap_or("");
            if !module.is_empty() {
                let name = module.rsplit('.').next().unwrap_or(module);
                symbols.push((name.to_string(), idx + 1));
            }
        }
    }

    symbols
}

fn js_imported_symbols(lines: &[&str]) -> Vec<(String, usize)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("import ") && trimmed.contains('{') && trimmed.contains("from") {
            if let Some(open) = trimmed.find('{') {
                if let Some(close) = trimmed.find('}') {
                    let inside = &trimmed[open + 1..close];
                    for part in inside.split(',') {
                        let name = part.split_whitespace().next().unwrap_or("");
                        if !name.is_empty() && name != "type" {
                            symbols.push((name.to_string(), idx + 1));
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
                    .unwrap_or("");
                let name = name.trim();
                if !name.is_empty() && name != "default" {
                    symbols.push((name.to_string(), idx + 1));
                }
            }
            continue;
        }

        if trimmed.starts_with("const ") && trimmed.contains("require(") && trimmed.contains('{') {
            if let Some(open) = trimmed.find('{') {
                if let Some(close) = trimmed.find('}') {
                    let inside = &trimmed[open + 1..close];
                    for part in inside.split(',') {
                        let name = part.trim().split(':').next().unwrap_or("").trim();
                        if !name.is_empty() {
                            symbols.push((name.to_string(), idx + 1));
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
    let trait_name = trait_part.split("::").last().unwrap_or(trait_part).trim();
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
    let body: String = lines
        .iter()
        .skip(1)
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .collect::<Vec<_>>()
        .join(" ");

    let trimmed = body.trim();
    if trimmed == "{}" || trimmed == "{ }" {
        return true;
    }

    let inner = trimmed.trim_start_matches('{').trim_end_matches('}').trim();
    let short_markers = ["todo!(", "unimplemented!(", "panic!(", "unreachable!("];
    if inner.is_empty() || short_markers.iter().any(|m| inner.starts_with(m)) {
        return true;
    }

    false
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
            .unwrap_or("");
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

pub fn extract_import_modules(content: &str) -> Vec<String> {
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
        .unwrap_or("");

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
    let normalized_file = Path::new(file_path)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| file_path.replace('\\', "/"));
    let normalized_root = Path::new(root_dir)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| root_dir.trim_end_matches('/').replace('\\', "/"));
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
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};

static ALL_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r#"__all__\s*=\s*\[([^\]]*)\]"#).ok());

pub fn extract_imported_aliases(content: &str) -> HashMap<String, String> {
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
                            alias.trim().to_string(),
                            format!("{}.{}", module, sym.trim()),
                        );
                    } else {
                        aliases.insert(name.to_string(), format!("{}.{}", module, name));
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
                    aliases.insert(alias.trim().to_string(), sym.trim().to_string());
                } else {
                    let alias = name.rsplit('.').next().unwrap_or(name);
                    aliases.insert(alias.to_string(), name.to_string());
                }
            }
        }
    }
    aliases
}

pub fn extract_exported_symbols(content: &str) -> HashSet<String> {
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
                        exported.insert(item.to_string());
                    }
                }
            }
        }
    }
    exported
}

pub fn extract_used_symbols(
    content: &str,
    imported_aliases: &HashMap<String, String>,
) -> HashSet<String> {
    let mut used = HashSet::new();
    let code_lines = content
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.starts_with("import ") && !t.starts_with("from ") && !t.starts_with('#')
        })
        .collect::<Vec<_>>()
        .join("\n");

    for alias in imported_aliases.keys() {
        let pattern = format!(r"\b{}\b", regex::escape(alias));
        if let Ok(re) = Regex::new(&pattern) {
            if re.is_match(&code_lines) {
                used.insert(alias.clone());
            }
        }
    }

    used
}

pub fn extract_rust_js_imports(content: &str) -> Vec<(String, usize)> {
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

        let names: Vec<String> = if t.starts_with("use ") {
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
                vec![name]
            }
        } else if t.starts_with("import ") {
            if let Some(from_idx) = t.find(" from ") {
                let import_part = t[7..from_idx].trim();
                let names: Vec<String> = if import_part.starts_with('{') {
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
                        .collect()
                } else {
                    vec![import_part.trim().to_string()]
                };
                names
            } else {
                continue;
            }
        } else {
            continue;
        };

        for name in names {
            if (name.starts_with('I')
                && name.len() > 1
                && name.chars().nth(1).unwrap_or(' ').is_uppercase())
                || name.ends_with("Protocol")
                || name.ends_with("Port")
                || name.ends_with("Trait")
                || name.ends_with("Aggregate")
                || name == "Parser"
            {
                continue;
            }
            imports.push((name, i));
        }
    }
    imports
}

pub fn is_name_used(name: &str, content: &str, exclude_line: usize) -> bool {
    if is_rust_trait_import(name) {
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
            | "Parser"
    )
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
                let allowed_str = if allowed.is_empty() {
                    "none".to_string()
                } else {
                    allowed
                        .iter()
                        .map(|v| v.value().to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
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
                        FIX: Remove the import or refactor to use one of the allowed layers: [{}].",
                    source_layer, forbidden_layer, dynamic_why, allowed_str
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
                        "Capabilities implement business rules — they must import contract ports to honor interface contracts and enable dependency injection.".to_string()
                    } else if src == "infrastructure" {
                        "Infrastructure adapters bridge technology and domain — they must import contract ports to implement the required protocols.".to_string()
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
                let supplement = reason
                    .as_ref()
                    .map(|r| format!("\n  Context: {}", r))
                    .unwrap_or_default();
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
                intent,
                reason,
            } => {
                let default_why = format!(
                    "Import '{}' in layer '{}' is not used according to its intended purpose.",
                    import_type, source_layer
                );
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
                write!(
                    f,
                    "AES204 IMPORT_INTENT: '{}' import in layer '{}' violates its intended purpose.\n\
                        WHY? {}\n\
                        FIX: {}",
                    import_type, source_layer, why, intent
                )
            }
            AesImportViolation::CircularImport { reason } => {
                let default_why = "Circular dependencies couple components together and break unidirectional data/import flow.".to_string();
                let why = reason
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or(default_why);
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
                let supplement = reason
                    .as_ref()
                    .map(|r| format!("\n  Context: {}", r))
                    .unwrap_or_default();
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

## File: crates/shared/src/lib.rs

```rust
// PURPOSE: shared — all taxonomy types, contract traits, and shared definitions
// No dependencies on other feature crates — this is the foundation layer.

#[path = "common/mod.rs"]
pub mod common;

// Re-export all taxonomy_* and contract_* types from common
pub use common::*;

// Error macros — must precede feature modules that use define_error!/define_wrapper!
#[macro_use]


// Feature-specific types (in feature folders)
#[path = "auto-fix/mod.rs"]
pub mod auto_fix;
#[path = "cli-commands/mod.rs"]
pub mod cli_commands;
#[path = "code-analysis/mod.rs"]
pub mod code_analysis;
#[path = "config-system/mod.rs"]
pub mod config_system;
#[path = "external-lint/mod.rs"]
pub mod external_lint;
#[path = "file-system/mod.rs"]
pub mod file_system;
#[path = "file-watch/mod.rs"]
pub mod file_watch;
#[path = "git-hooks/mod.rs"]
pub mod git_hooks;
#[path = "import-rules/mod.rs"]
pub mod import_rules;
#[path = "mcp-server/mod.rs"]
pub mod mcp_server;
#[path = "multi-project/mod.rs"]
pub mod multi_project;
#[path = "naming-rules/mod.rs"]
pub mod naming_rules;
#[path = "orphan-detector/mod.rs"]
pub mod orphan_detector;
#[path = "project-setup/mod.rs"]
pub mod project_setup;
#[path = "role-rules/mod.rs"]
pub mod role_rules;
#[path = "source-parsing/mod.rs"]
pub mod source_parsing;
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

## File: crates/shared/src/multi-project/taxonomy_summary_vo.rs

```rust
// PURPOSE: ProjectSummaryVO — value object for multi-project governance summary data
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_common_vo::Score;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AggregatedResults {
    pub projects: Vec<ProjectResult>,
    pub total_projects: Count,
    pub passing_projects: Count,
    pub failing_projects: Count,
    pub average_score: Score,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectResult {
    pub path: FilePath,
    pub score: Score,
    pub is_passing: ComplianceStatus,
    pub issues: Vec<std::collections::HashMap<String, serde_json::Value>>,
    pub adapters: PatternList,
    pub error: ErrorMessage,
}

impl AggregatedResults {
    pub fn new(
        projects: Vec<ProjectResult>,
        total_projects: Count,
        passing_projects: Count,
        failing_projects: Count,
        average_score: Score,
    ) -> Self {
        Self {
            projects,
            total_projects,
            passing_projects,
            failing_projects,
            average_score,
        }
    }
}

impl ProjectResult {
    pub fn new(
        path: FilePath,
        score: Score,
        is_passing: ComplianceStatus,
        issues: Vec<std::collections::HashMap<String, serde_json::Value>>,
        adapters: PatternList,
        error: ErrorMessage,
    ) -> Self {
        Self {
            path,
            score,
            is_passing,
            issues,
            adapters,
            error,
        }
    }
}
```

---

## File: crates/shared/src/multi-project/taxonomy_workspace_info_vo.rs

```rust
// PURPOSE: WorkspaceInfo — value object for discovered workspace with its config
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceInfo {
    pub path: FilePath,
    pub workspace_type: String,
    pub config: ArchitectureConfig,
}

impl WorkspaceInfo {
    pub fn new(path: FilePath, workspace_type: String, config: ArchitectureConfig) -> Self {
        Self {
            path,
            workspace_type,
            config,
        }
    }
}
```

---

## File: crates/shared/src/naming-rules/taxonomy_suffix_vo.rs

```rust
// PURPOSE: SuffixPolicyVO, SuffixVO — value objects for suffix naming rules
use crate::common::taxonomy_common_vo::PatternList;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(transparent)]
pub struct SuffixPolicyVO {
    pub value: String,
}

impl SuffixPolicyVO {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SuffixVO {
    pub values: PatternList,
}
```

---

## File: crates/shared/src/project-setup/mod.rs

```rust
pub mod contract_maintenance_aggregate;
pub mod contract_setup_aggregate;
pub mod contract_setup_protocol;
pub mod taxonomy_doctor_vo;
pub mod taxonomy_language_vo;
pub mod taxonomy_stats_vo;
```

---

## File: crates/shared/src/project-setup/taxonomy_doctor_vo.rs

```rust
// PURPOSE: DoctorResultVO, DoctorCheck — VOs for project health diagnostics results
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DoctorResultVO {
    pub python_version: DescriptionVO,
    pub is_installed: ComplianceStatus,
    pub config_found: FilePathList,
    pub adapter_statuses: HashMap<AdapterName, String>,
    pub issues: Vec<ErrorMessage>,
    pub healthy: ComplianceStatus,
}

impl DoctorResultVO {
    pub fn new(
        python_version: DescriptionVO,
        is_installed: ComplianceStatus,
        config_found: FilePathList,
        adapter_statuses: HashMap<AdapterName, String>,
        issues: Vec<ErrorMessage>,
        healthy: ComplianceStatus,
    ) -> Self {
        Self {
            python_version,
            is_installed,
            config_found,
            adapter_statuses,
            issues,
            healthy,
        }
    }
}

impl std::fmt::Display for DoctorResultVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DoctorResult(healthy={}, python={})",
            self.healthy.value, self.python_version.value
        )
    }
}
```

---

## File: crates/shared/src/project-setup/taxonomy_language_vo.rs

```rust
// PURPOSE: LanguageConfigVO — value object for programming language configuration
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectLanguage {
    pub value: String,
}

impl ProjectLanguage {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LanguageSource {
    pub language: String,
    pub confidence: u8,
    pub source: String,
}

impl LanguageSource {
    pub fn new(language: impl Into<String>, confidence: u8, source: impl Into<String>) -> Self {
        Self {
            language: language.into(),
            confidence,
            source: source.into(),
        }
    }
}
```

---

## File: crates/shared/src/project-setup/taxonomy_stats_vo.rs

```rust
// PURPOSE: ProjectStatsVO, MaintenanceStatsVO — VOs for project statistics and maintenance data
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::Score;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MaintenanceStatsVO {
    pub project_path: FilePath,
    pub total_files: Count,
    pub test_files: Count,
    pub test_ratio: Score,
    pub python_files: Count,
}

impl MaintenanceStatsVO {
    pub fn new(
        project_path: FilePath,
        total_files: Count,
        test_files: Count,
        test_ratio: Score,
        python_files: Count,
    ) -> Self {
        Self {
            project_path,
            total_files,
            test_files,
            test_ratio,
            python_files,
        }
    }
}

impl std::fmt::Display for MaintenanceStatsVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MaintenanceStats({}: {} files, {} test, {:.1}%)",
            self.project_path,
            self.total_files.value,
            self.test_files.value,
            self.test_ratio.value * 100.0
        )
    }
}
```

---

## File: crates/shared/src/source-parsing/contract_parser_port.rs

```rust
// PURPOSE: ISourceParserPort — port trait for language-specific source code parsing (imports, definitions)
use crate::code_analysis::taxonomy_import_source_vo::ImportInfoList;
use crate::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList;
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::common::taxonomy_suggestion_vo::MetadataVO;
use crate::mcp_server::taxonomy_job_vo::ResponseData;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;
use crate::source_parsing::taxonomy_naming_list_vo::PrimitiveTypeList;
use crate::source_parsing::taxonomy_parser_error::SourceParserError;
use crate::source_parsing::taxonomy_path_vo::FilePath;

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

## File: crates/shared/src/source-parsing/mod.rs

```rust
// source-parsing — taxonomy and contract types
pub mod contract_language_detector_port;
pub mod contract_parser_port;
pub mod contract_path_normalization_port;
pub mod contract_scanner_provider_port;
pub mod taxonomy_adapter_error;
pub mod taxonomy_barrel_provider_vo;
pub mod taxonomy_file_collector;
pub mod taxonomy_language_detector_helper;
pub mod taxonomy_naming_error;
pub mod taxonomy_naming_list_vo;
pub mod taxonomy_parser_error;
pub mod taxonomy_path_vo;
pub mod taxonomy_paths_vo;
pub mod taxonomy_semantic_error;
```

---

## File: crates/shared/src/source-parsing/taxonomy_adapter_error.rs

```rust
// PURPOSE: AdapterError, ScanError, ValidationError — structured error types for adapter operations
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::Constraint;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_error::ExitCode;
use crate::common::taxonomy_common_error::FieldName;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_source_vo::ContentString;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
        let code = self
            .error_code
            .as_ref()
            .map(|c| format!(" [{}]", c))
            .unwrap_or_default();
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
        let adapter = self
            .adapter_name
            .as_ref()
            .map(|a| format!(" ({})", a))
            .unwrap_or_default();
        let code = self
            .error_code
            .as_ref()
            .map(|c| format!(" [{}]", c))
            .unwrap_or_default();
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

## File: crates/shared/src/source-parsing/taxonomy_language_detector_helper.rs

```rust
// PURPOSE: LanguageDetector — Helper for detecting programming languages from file paths
use crate::source_parsing::contract_language_detector_port::Language;
use crate::source_parsing::taxonomy_path_vo::FilePath;

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

## File: crates/shared/src/source-parsing/taxonomy_naming_error.rs

```rust
// PURPOSE: NamingError — structured error type for naming convention violations
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_error_vo::ErrorCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct NamingError {
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: ErrorCode,
    #[serde(default)]
    pub cause: Cause,
}

impl NamingError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            message,
            error_code: ErrorCode::default(),
            cause: Cause::default(),
        }
    }
}

impl std::fmt::Display for NamingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code: &str = &self.error_code;
        if code.is_empty() {
            write!(f, "Naming Error: {}", self.message)
        } else {
            write!(f, "Naming Error [{}]: {}", code, self.message)
        }
    }
}
```

---

## File: crates/shared/src/source-parsing/taxonomy_naming_list_vo.rs

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

## File: crates/shared/src/source-parsing/taxonomy_parser_error.rs

```rust
// PURPOSE: ParserError — structured error type for source code parsing failures
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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

## File: crates/shared/src/source-parsing/taxonomy_path_vo.rs

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
        // Remove all trailing slashes
        while value.ends_with('/') && value.len() > 1 {
            value.pop();
        }
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
        if special_files.contains(&self.value.as_ref()) || self.value.starts_with('.') {
            return "".to_string();
        }
        match self.value.rsplit('.').next() {
            Some(ext) => ext.to_string(),
            None => "".to_string(),
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

    /// Check if the path is a barrel file.
    pub fn is_barrel_file(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py" | "mod.rs" | "index.ts" | "index.js"
        )
    }

    /// Check if the path is a module/layer entry point file.
    pub fn is_entry_point(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py" | "main.py" | "py.typed" | "app.py" | "lib.rs"
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
        // Remove trailing slash unless it's just "/"
        if value.ends_with('/') && value.len() > 1 {
            value.pop();
        }
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

#[cfg(test)]
mod tests {
    use super::{DirectoryPath, FilePath};

    #[test]
    fn test_file_path_new() {
        let fp = FilePath::new("test.txt").unwrap_or_default();
        assert_eq!(fp.value, "test.txt");
        assert_eq!(fp.extension(), "txt");
        assert!(fp.has_extension("txt"));
        assert!(!fp.has_extension("md"));

        // Test normalization
        let fp = FilePath::new("path\\to\\file.txt").unwrap_or_default();
        assert_eq!(fp.value, "path/to/file.txt");

        let fp = FilePath::new("path/to/file/").unwrap_or_default();
        assert_eq!(fp.value, "path/to/file");

        let fp = FilePath::new("/").unwrap_or_default();
        assert_eq!(fp.value, "/");

        let fp = FilePath::new("///").unwrap_or_default();
        assert_eq!(fp.value, "/");
    }

    #[test]
    fn test_file_path_invalid() {
        assert!(FilePath::new("").is_err());
        assert!(FilePath::new("   ").is_err());
    }

    #[test]
    fn test_directory_path_new() {
        let dp = DirectoryPath::new("test/dir").unwrap_or_default();
        assert_eq!(dp.value, "test/dir");

        let dp = DirectoryPath::new("test/dir/").unwrap_or_default();
        assert_eq!(dp.value, "test/dir");

        let dp = DirectoryPath::new("/").unwrap_or_default();
        assert_eq!(dp.value, "/");
    }

    #[test]
    fn test_directory_path_invalid() {
        assert!(DirectoryPath::new("").is_err());
        assert!(DirectoryPath::new("   ").is_err());
    }
}
```

---

## File: crates/shared/src/source-parsing/taxonomy_paths_vo.rs

```rust
// PURPOSE: FilePathList, DirectoryPath, SourceDir — VOs for file/directory path collections
use serde::{Deserialize, Serialize};

use crate::source_parsing::taxonomy_path_vo::FilePath;

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

## File: crates/shared/src/source-parsing/taxonomy_semantic_error.rs

```rust
// PURPOSE: SemanticError — structured error type for semantic analysis failures
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct SemanticError {
    #[serde(default)]
    pub path: FilePath,
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: ErrorCode,
    #[serde(default)]
    pub cause: Cause,
}

impl SemanticError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            path: FilePath::default(),
            message,
            error_code: ErrorCode::default(),
            cause: Cause::default(),
        }
    }
}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let target = {
            let p: &str = &self.path;
            if p.is_empty() {
                String::new()
            } else {
                format!(" on {}", p)
            }
        };
        let code = {
            let c: &str = &self.error_code;
            if c.is_empty() {
                String::new()
            } else {
                format!(" [{}]", c)
            }
        };
        write!(f, "Semantic Error{}{}: {}", target, code, self.message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct ScopeResolutionError {
    #[serde(flatten)]
    pub base: SemanticError,
}

impl ScopeResolutionError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            base: SemanticError::new(message),
        }
    }
}

impl std::fmt::Display for ScopeResolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct CallChainError {
    #[serde(flatten)]
    pub base: SemanticError,
}

impl CallChainError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            base: SemanticError::new(message),
        }
    }
}

impl std::fmt::Display for CallChainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)
    }
}
```

---

