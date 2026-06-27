# Crate: role-rules (v1.10.14)

This document contains the source code for feature crate `role-rules` along with its corresponding and imported definitions from the `shared` crate.

## Problem Statement

The following issues were detected by `lint-arwaky-cli scan`:

```
============================================================
  AES Architecture Compliance Report
============================================================
  Project: /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules
  Violations: 28
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/agent_role_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/agent_role_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/agent_role_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/agent_role_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/agent_role_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/agent_role_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/agent_role_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/agent_role_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_capabilities_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_capabilities_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_capabilities_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_capabilities_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_capabilities_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_capabilities_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_capabilities_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_contract_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_contract_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_surface_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_surface_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_surface_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_surface_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_surface_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_surface_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_surface_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_surface_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_surface_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_taxonomy_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_taxonomy_role_auditor.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
```

---

## File List

- [crates/role-rules/Cargo.toml](file:///home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/Cargo.toml)
- [crates/role-rules/src/agent_role_orchestrator.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/agent_role_orchestrator.rs)
- [crates/role-rules/src/capabilities_agent_role_auditor.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_agent_role_auditor.rs)
- [crates/role-rules/src/capabilities_capabilities_role_auditor.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_capabilities_role_auditor.rs)
- [crates/role-rules/src/capabilities_contract_role_auditor.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_contract_role_auditor.rs)
- [crates/role-rules/src/capabilities_infrastructure_role_auditor.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_infrastructure_role_auditor.rs)
- [crates/role-rules/src/capabilities_surface_role_auditor.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_surface_role_auditor.rs)
- [crates/role-rules/src/capabilities_taxonomy_role_auditor.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/capabilities_taxonomy_role_auditor.rs)
- [crates/role-rules/src/lib.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/lib.rs)
- [crates/role-rules/src/root_role_rules_container.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/role-rules/src/root_role_rules_container.rs)
- [crates/shared/src/cli-commands/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/mod.rs)
- [crates/shared/src/cli-commands/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_severity_vo.rs)
- [crates/shared/src/code-analysis/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/mod.rs)
- [crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs)
- [crates/shared/src/common/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/mod.rs)
- [crates/shared/src/common/taxonomy_adapter_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_name_vo.rs)
- [crates/shared/src/common/taxonomy_common_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_vo.rs)
- [crates/shared/src/common/taxonomy_definition_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_definition_vo.rs)
- [crates/shared/src/common/taxonomy_error_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_error_vo.rs)
- [crates/shared/src/common/taxonomy_lint_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_lint_vo.rs)
- [crates/shared/src/common/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_severity_vo.rs)
- [crates/shared/src/common/taxonomy_source_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_source_vo.rs)
- [crates/shared/src/config-system/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/mod.rs)
- [crates/shared/src/config-system/taxonomy_config_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_config_vo.rs)
- [crates/shared/src/import-rules/contract_rule_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_rule_protocol.rs)
- [crates/shared/src/import-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/mod.rs)
- [crates/shared/src/role-rules/contract_agent_role_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_agent_role_protocol.rs)
- [crates/shared/src/role-rules/contract_capabilities_role_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_capabilities_role_protocol.rs)
- [crates/shared/src/role-rules/contract_infrastructure_role_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_infrastructure_role_protocol.rs)
- [crates/shared/src/role-rules/contract_role_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_role_aggregate.rs)
- [crates/shared/src/role-rules/contract_role_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_role_protocol.rs)
- [crates/shared/src/role-rules/contract_role_runner_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_role_runner_aggregate.rs)
- [crates/shared/src/role-rules/contract_surface_role_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_surface_role_protocol.rs)
- [crates/shared/src/role-rules/contract_taxonomy_role_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_taxonomy_role_protocol.rs)
- [crates/shared/src/role-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/mod.rs)
- [crates/shared/src/role-rules/taxonomy_layer_names_constant.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/taxonomy_layer_names_constant.rs)
- [crates/shared/src/role-rules/taxonomy_layer_names_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/taxonomy_layer_names_vo.rs)
- [crates/shared/src/role-rules/taxonomy_role_rule_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/taxonomy_role_rule_vo.rs)
- [crates/shared/src/role-rules/taxonomy_violation_role_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/taxonomy_violation_role_vo.rs)
- [crates/shared/src/source-parsing/contract_language_detector_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/contract_language_detector_port.rs)
- [crates/shared/src/source-parsing/infrastructure_file_collector_provider.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/infrastructure_file_collector_provider.rs)
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

## File: crates/role-rules/Cargo.toml

```toml
[package]
name = "role_rules-lint-arwaky"
version = "1.10.14"
edition = "2021"
description = "Architectural role-layer violation checks covering AES401–AES406 (taxonomy, contract, capability, infrastructure, surface, root wiring)."
license = "MIT"
repository = "https://github.com/rakaarwaky/lint-arwaky"
publish = false

[lints]
workspace = true

[dependencies]  # (unchanged)
serde.workspace = true
serde_json.workspace = true
async-trait.workspace = true
once_cell.workspace = true
regex.workspace = true
shared.workspace = true
```

---

## File: crates/role-rules/src/agent_role_orchestrator.rs

```rust
// PURPOSE: RoleOrchestrator — dispatches files to correct role checker based on filename prefix
//
// ALGORITHM:
//   1. run_all_role_checks iterates files, extracts filename prefix (first underscore-segment).
//   2. Matches prefix to layer (taxonomy, contract, capabilities, infrastructure, agent,
//      surfaces, root/lib/mod) and dispatches to the corresponding role checker.
//   3. Each checker receives the SourceContentVO (file path + content + language) and
//      returns violations via the violations Vec.
//   4. Unknown prefixes emit an INFO-level structured violation instead of eprintln!.
//
// NOTE: check_aggregate (forbidden inheritance) is NOT called here because the orchestrator
//      lacks layer definitions; that check runs via the IContractRoleChecker trait path
//      where callers supply the proper LayerDefinition.

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::role_rules::contract_role_aggregate::IRoleAggregate;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use shared::taxonomy_source_vo::{ContentString, SourceContentVO};
use std::path::Path;
use std::sync::Arc;

use shared::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
use shared::role_rules::contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
use shared::role_rules::contract_infrastructure_role_protocol::IInfrastructureRoleChecker;
use shared::role_rules::contract_role_protocol::IContractRoleChecker;
use shared::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;
use shared::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;

pub struct RoleOrchestrator {
    aggregate: Arc<dyn IRoleAggregate>,
    ignored_paths: Vec<String>,
}

impl RoleOrchestrator {
    pub fn new(
        aggregate: Arc<dyn IRoleAggregate>,
        config: &shared::config_system::taxonomy_config_vo::ArchitectureConfig,
    ) -> Self {
        let ignored_paths: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();
        Self {
            aggregate,
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

    pub fn run_all_role_checks(
        &self,
        files: &[String],
        max_lines: usize,
        violations: &mut Vec<LintResult>,
    ) {
        for file in files {
            let content = std::fs::read_to_string(file).unwrap_or_default();
            let filename = Path::new(file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            let stem = Path::new(filename)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            let prefix = stem.split('_').next().unwrap_or("");

            let fp = FilePath::new(file.to_string()).unwrap_or_default();
            let content_vo = ContentString::new(content);
            let detector =
                shared::source_parsing::taxonomy_language_detector_helper::LanguageDetector::new();
            let language = detector.detect(&fp).as_str().to_string();
            let source_vo = SourceContentVO::new(fp, content_vo, &language);

            match prefix {
                "agent" => {
                    let checker = self.aggregate.agent();
                    checker.check_file_size_limit(&source_vo, max_lines, violations);
                    checker.check_any_type_annotation(&source_vo, violations);
                    if filename.contains("_container") {
                        checker.check_container(&source_vo, violations);
                    } else if filename.contains("_orchestrator") {
                        checker.check_orchestrator(&source_vo, violations);
                    } else if filename.contains("_lifecycle") {
                        checker.check_lifecycle(&source_vo, violations);
                    }
                }
                "root" => {}
                "surfaces" | "surface" => {
                    let checker = self.aggregate.surface();
                    checker.check_fn_count_limit(&source_vo, violations);
                    let is_smart = filename.contains("_command")
                        || filename.contains("_controller")
                        || filename.contains("_page")
                        || filename.contains("_entry");
                    let is_utility = filename.contains("_hook")
                        || filename.contains("_store")
                        || filename.contains("_action")
                        || filename.contains("_screen")
                        || filename.contains("_router");
                    if is_smart {
                        checker.check_smart_surface(&source_vo, violations);
                    } else if is_utility {
                        checker.check_utility_surface(&source_vo, violations);
                    } else {
                        checker.check_passive_surface(&source_vo, violations);
                    }
                }
                "infrastructure" | "infra" => {
                    let checker = self.aggregate.infrastructure();
                    checker.check_port_implementation(&source_vo, violations);
                }
                "contract" => {
                    let checker = self.aggregate.contract();
                    if filename.contains("_port") {
                        violations.extend(checker.check_port(&source_vo));
                    } else if filename.contains("_protocol") {
                        violations.extend(checker.check_protocol(&source_vo));
                    }
                }
                "capabilities" | "capability" => {
                    let checker = self.aggregate.capabilities();
                    checker.check_capability_routing(&source_vo, "capabilities", violations);
                }
                "taxonomy" => {
                    let checker = self.aggregate.taxonomy();
                    checker.check_entity(&source_vo, violations);
                    checker.check_error(&source_vo, violations);
                    checker.check_event(&source_vo, violations);
                    checker.check_constant(&source_vo, violations);
                }
                _ => {}
            }
        }
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
impl shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate for RoleOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        let mut results = Vec::new();
        let files = self.collect_files(target);
        let file_strings: Vec<String> = files.values.iter().map(|f| f.to_string()).collect();
        self.run_all_role_checks(&file_strings, 500, &mut results);
        results
    }

    fn name(&self) -> &str {
        "role-rules"
    }
}

pub struct RoleAggregateImpl {
    taxonomy: Arc<dyn ITaxonomyRoleChecker>,
    contract: Arc<dyn IContractRoleChecker>,
    infrastructure: Arc<dyn IInfrastructureRoleChecker>,
    capabilities: Arc<dyn ICapabilitiesRoleChecker>,
    surface: Arc<dyn ISurfaceRoleChecker>,
    agent: Arc<dyn IAgentRoleChecker>,
}

impl RoleAggregateImpl {
    pub fn new(
        taxonomy: Arc<dyn ITaxonomyRoleChecker>,
        contract: Arc<dyn IContractRoleChecker>,
        infrastructure: Arc<dyn IInfrastructureRoleChecker>,
        capabilities: Arc<dyn ICapabilitiesRoleChecker>,
        surface: Arc<dyn ISurfaceRoleChecker>,
        agent: Arc<dyn IAgentRoleChecker>,
    ) -> Self {
        Self {
            taxonomy,
            contract,
            infrastructure,
            capabilities,
            surface,
            agent,
        }
    }
}

impl IRoleAggregate for RoleAggregateImpl {
    fn taxonomy(&self) -> &dyn ITaxonomyRoleChecker {
        self.taxonomy.as_ref()
    }
    fn contract(&self) -> &dyn IContractRoleChecker {
        self.contract.as_ref()
    }
    fn infrastructure(&self) -> &dyn IInfrastructureRoleChecker {
        self.infrastructure.as_ref()
    }
    fn capabilities(&self) -> &dyn ICapabilitiesRoleChecker {
        self.capabilities.as_ref()
    }
    fn surface(&self) -> &dyn ISurfaceRoleChecker {
        self.surface.as_ref()
    }
    fn agent(&self) -> &dyn IAgentRoleChecker {
        self.agent.as_ref()
    }
}
```

---

## File: crates/role-rules/src/capabilities_agent_role_auditor.rs

```rust
// PURPOSE: AgentRoleChecker — IAgentRoleChecker for AES405: agent file size limits and any-type checks
//
// ALGORITHM:
//   1. check_file_size_limit — Counts lines in the source file. If the count exceeds
//      max_lines, emits AES405 AgentFileSizeLimit.
//   2. check_any_type_annotation — Line-by-line scan for `: any`, `: Any`, `-> any`,
//      `-> Any`, `Any<`, `Any[`, or `any[` patterns. Flags each match as AES405 AnyType.
//
// NOTE: check_container / check_orchestrator / check_lifecycle are no-ops because
//      container/orchestrator/lifecycle role checks are done via the IAnalyzer-based
//      entry points (check_surface_hierarchy, check_surface_roles) rather than inline.
//      These trait methods are required by IAgentRoleChecker but are intentionally
//      empty for this checker implementation.
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_source_vo::SourceContentVO;

pub struct AgentRoleChecker {}

impl Default for AgentRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentRoleChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check_file_size_limit(
        &self,
        source: &SourceContentVO,
        max_lines: usize,
        violations: &mut Vec<LintResult>,
    ) {
        let content = source.content.value();
        let file = source.file_path.value();
        if content.lines().count() > max_lines {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES405",
                Severity::HIGH,
                AesRoleViolation::AgentFileSizeLimit { max_lines }.to_string(),
            ));
        }
    }

    pub fn check_any_type_annotation(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<LintResult>,
    ) {
        let content = source.content.value();
        let file = source.file_path.value();
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if t.contains(": any")
                || t.contains(": Any")
                || t.contains("-> any")
                || t.contains("-> Any")
                || t.contains("Any<")
                || t.contains("Any[")
                || t.contains("any[")
            {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES405",
                    Severity::HIGH,
                    AesRoleViolation::AnyType { reason: None }.to_string(),
                ));
            }
        }
    }
}

impl IAgentRoleChecker for AgentRoleChecker {
    fn check_container(
        &self,
        _source: &SourceContentVO,
        _violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_orchestrator(
        &self,
        _source: &SourceContentVO,
        _violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_lifecycle(
        &self,
        _source: &SourceContentVO,
        _violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_file_size_limit(
        &self,
        source: &SourceContentVO,
        max_lines: usize,
        violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
        self.check_file_size_limit(source, max_lines, violations);
    }
    fn check_any_type_annotation(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
        self.check_any_type_annotation(source, violations);
    }
}
```

---

## File: crates/role-rules/src/capabilities_capabilities_role_auditor.rs

```rust
// PURPOSE: CapabilitiesRoleChecker — AES403: detect capability routing (missing interface implementation)
//
// ALGORITHM:
//   1. check_capability_routing — Scans capabilities-layer files for struct definitions.
//      For each struct, checks if the file contains `impl I{StructName}`, `impl ... for {StructName}`,
//      or `impl {StructName}`. If not and the file has <= 3 structs, flags CapabilityRouting.
//      Skips `#[cfg(test)]` blocks.
//
// NOTE: The layer guard is redundant with the caller but kept for defensive programming.
//      This checker assumes Rust syntax; Python/JS support would need additional parsing.
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::role_rules::contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::source_parsing::contract_language_detector_port::Language as DetLang;
use shared::taxonomy_name_vo::SymbolName;
use shared::taxonomy_source_vo::SourceContentVO;

pub struct CapabilitiesRoleChecker {}

impl Default for CapabilitiesRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilitiesRoleChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check_capability_routing(
        &self,
        source: &SourceContentVO,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if layer != "capabilities" && !layer.starts_with("capabilities(") {
            return;
        }
        let file = source.file_path.value();
        let content = source.content.value();
        let detector =
            shared::source_parsing::taxonomy_language_detector_helper::LanguageDetector::new();
        let lang = detector.detect(&source.file_path);
        let is_rs = lang == DetLang::Rust;
        let is_py = lang == DetLang::Python;
        let is_js = lang == DetLang::JavaScript || lang == DetLang::TypeScript;

        if is_rs {
            self._check_rust_routing(file, content, violations);
        } else if is_py {
            self._check_python_routing(file, content, violations);
        } else if is_js {
            self._check_js_routing(file, content, violations);
        }
    }

    fn _check_rust_routing(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let mut in_cfg_test = false;
        let structs: Vec<&str> = content
            .lines()
            .filter_map(|l| {
                let t = l.trim();
                if t.starts_with("#[cfg(test)]") {
                    in_cfg_test = true;
                    return None;
                }
                if in_cfg_test {
                    if t == "}" || t.starts_with("}") {
                        in_cfg_test = false;
                    }
                    return None;
                }
                let words: Vec<&str> = t.split_whitespace().collect();
                if (t.starts_with("pub struct ") || t.starts_with("struct ")) && words.len() >= 2 {
                    let struct_idx = words.iter().position(|w| *w == "struct").unwrap_or(0);
                    Some(
                        words
                            .get(struct_idx + 1)
                            .unwrap_or(&"")
                            .trim_end_matches(';'),
                    )
                } else {
                    None
                }
            })
            .filter(|n| !n.is_empty() && !n.starts_with('_'))
            .collect();
        for s in &structs {
            let hi = content.contains(&format!("impl I{}", s))
                || content.contains(&format!("for {} ", s))
                || content.contains(&format!("for {}{{", s))
                || content.contains(&format!("for {} {{", s))
                || content.contains(&format!("impl {} ", s))
                || content.contains(&format!("impl {}{{", s));
            if !hi && structs.len() <= 3 {
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES403",
                    Severity::MEDIUM,
                    AesRoleViolation::CapabilityRouting {
                        struct_name: SymbolName::new(*s),
                        reason: None,
                    },
                ));
            }
        }
    }

    fn _check_js_routing(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let mut classes: Vec<(&str, usize)> = Vec::new();
        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            if t.starts_with("class ") {
                let name = t
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("")
                    .split('{')
                    .next()
                    .unwrap_or("")
                    .split(':')
                    .next()
                    .unwrap_or("")
                    .split_whitespace()
                    .next()
                    .unwrap_or("");
                if !name.is_empty() && !name.starts_with('_') {
                    classes.push((name, i));
                }
            }
        }
        if classes.len() > 3 {
            return;
        }
        for (name, start_line) in &classes {
            let mut has_method = false;
            for line in lines.iter().skip(start_line + 1).map(|l| l.trim()) {
                if line.starts_with('}') || line.starts_with(';') {
                    break;
                }
                if line.starts_with("function ")
                    || line.starts_with("public ")
                    || line.starts_with("private ")
                    || line.starts_with("protected ")
                    || line.starts_with("static ")
                    || line.starts_with("get ")
                    || line.starts_with("set ")
                    || line.starts_with("async ")
                {
                    has_method = true;
                    break;
                }
            }
            if !has_method {
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES403",
                    Severity::MEDIUM,
                    AesRoleViolation::CapabilityRouting {
                        struct_name: SymbolName::new(*name),
                        reason: None,
                    },
                ));
            }
        }
    }

    fn _check_python_routing(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let mut classes: Vec<(&str, usize)> = Vec::new();
        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            if t.starts_with("class ") {
                let name = t
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("")
                    .trim_end_matches(':');
                if !name.is_empty() && !name.starts_with('_') {
                    classes.push((name, i));
                }
            }
        }
        if classes.len() > 3 {
            return;
        }
        for (name, start_line) in &classes {
            let mut body_lines = 0;
            let mut has_method = false;
            let mut indent: Option<usize> = None;
            for line in lines.iter().skip(start_line + 1) {
                if line.trim().is_empty() {
                    continue;
                }
                let leading = line.len() - line.trim_start().len();
                if indent.is_none() {
                    if leading == 0 {
                        break;
                    }
                    indent = Some(leading);
                }
                if line.trim_start().starts_with("def ") {
                    has_method = true;
                    break;
                }
                body_lines += 1;
                if body_lines > 20 {
                    break;
                }
            }
            if !has_method {
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES403",
                    Severity::MEDIUM,
                    AesRoleViolation::CapabilityRouting {
                        struct_name: SymbolName::new(*name),
                        reason: None,
                    },
                ));
            }
        }
    }
}

impl ICapabilitiesRoleChecker for CapabilitiesRoleChecker {
    fn check_capability_routing(
        &self,
        source: &SourceContentVO,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        self.check_capability_routing(source, layer, violations);
    }
}
```

---

## File: crates/role-rules/src/capabilities_contract_role_auditor.rs

```rust
// PURPOSE: ContractRoleChecker — IContractRoleChecker for AES402: contract primitive type audits
//
// ALGORITHM:
//   1. check_aggregate — Scans import lines for blocked trait patterns (layer + suffix)
//      defined in LayerDefinition.role.forbidden_inheritance. Flags any `impl Trait for X`
//      or equivalent that uses a disallowed trait by name.
//   2. scan_contract_primitive (port/protocol dispatch) — Detects primitive type employment
//      in contract interfaces (port/protocol files). Uses LanguageDetector to determine
//      language, then checks for known primitive keywords per language.
//
// NOTE: check_contract_primitive is called for all files (not just test projects)
//      since AES402 applies universally — removed test-project guard per DX audit.
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::Language;
use shared::role_rules::contract_role_protocol::IContractRoleChecker;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::source_parsing::contract_language_detector_port::Language as DetLang;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_source_vo::SourceContentVO;

fn aes013_forbidden_inheritance(trait_name: &str) -> String {
    format!(
        "AES013 FORBIDDEN_INHERITANCE: '{}' implemented from forbidden source.\n\
         WHY? Contracts must not inherit from forbidden source layers.\n\
         FIX: Remove the inheritance or use a valid contract port/protocol instead.",
        trait_name
    )
}

pub struct ContractRoleChecker {}

impl Default for ContractRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

// ─── AES402 helper functions ──────────────────────────────────────────────
//
// These pure functions implement the trait-method-signature parser used by
// `check_contract_primitive` above. They live at module level so they can be
// unit-tested directly without needing to construct a `ContractRoleChecker`
// or feed it a full `SourceContentVO`.

/// Extract `(line_no, raw_signature_line)` for every `fn name(...) -> ... ;`
/// declaration that lives inside a `pub trait Name { ... }` block.
///
/// Only Rust trait declarations are tracked. Free-standing `fn` definitions
/// (impl blocks, inherent impls, free functions) are intentionally ignored
/// because the AES402 rule applies to the contract layer (port / protocol
/// traits) — implementation details are an adapter concern.
fn extract_trait_method_signatures(content: &str) -> Vec<(usize, String)> {
    let mut results = Vec::new();
    let mut in_trait_depth: i32 = 0;
    let mut brace_depth: i32 = 0;

    for (idx, raw) in content.lines().enumerate() {
        let line_no = idx + 1;
        let line = raw.trim();

        if in_trait_depth == 0 {
            // Detect a trait header line. We accept both `pub trait Foo` and
            // `pub trait Foo: Bar` (trait inheritance). We require a `{` on the
            // same line so we don't mistake `trait Foo;` (item declaration)
            // for a real trait body.
            let is_trait_header = (line.starts_with("pub trait ") || line.starts_with("trait "))
                && line.contains('{')
                && line.contains(')').ge(&line.contains('(')); // rough sanity
            if is_trait_header {
                in_trait_depth = 1;
                brace_depth = line.matches('{').count() as i32 - line.matches('}').count() as i32;
                continue;
            }
            continue;
        }

        // We're inside a trait body. Check for method declaration.
        // Heuristic: line starts with `fn ` (allowing leading whitespace) and
        // contains a `;` somewhere — that's the canonical Rust trait method
        // declaration form.
        if line.starts_with("fn ") && line.contains(';') {
            results.push((line_no, raw.to_string()));
        }

        brace_depth += line.matches('{').count() as i32 - line.matches('}').count() as i32;
        if brace_depth <= 0 {
            in_trait_depth = 0;
            brace_depth = 0;
        }
    }

    results
}

/// Decide whether a single Rust method signature uses a forbidden primitive
/// type. Returns the list of forbidden type tokens found (used for the
/// violation message). Empty list means the signature is clean.
///
/// Policy (per AES402 + project conventions):
///   * `&str` is ALLOWED — borrowed string slice, idiomatic Rust for file
///     paths, error messages, and other borrowed string data passed into
///     trait methods. Borrow lifetimes preclude replacing it with a taxonomy
///     VO without major API churn.
///   * `bool` is ALLOWED — represents a semantic toggle that is not
///     meaningfully expressible as a VO without ceremony.
///   * `String` (owned) is FORBIDDEN — must be replaced with a taxonomy VO
///     such as `LintMessage`, `ErrorMessage`, `SymbolName`, or `JobId`.
///   * `Result<String, _>` and `Result<&str, _>` are FORBIDDEN — error
///     variants must use a defined `taxonomy_*_error` type.
///   * Numeric primitives (`i32`, `i64`, `u32`, `u64`, `f32`, `f64`,
///     `usize`, `isize`) are FORBIDDEN — must be wrapped in a domain VO
///     (`Count`, `LineNumber`, `ColumnNumber`, `Duration`).
///   * `char` is FORBIDDEN — must use a domain VO if a single character is
///     ever needed (rare).
fn signature_uses_forbidden_primitive(sig: &str) -> Vec<&'static str> {
    let mut forbidden: Vec<&'static str> = Vec::new();

    let line = sig.trim();

    // Return type — anything after `->` up to `{` or `;` or EOL.
    let ret_type: String = if let Some(arrow_idx) = line.find("->") {
        let after = &line[arrow_idx + 2..];
        // Trim at the first `;` (single-line sig) or `{` (rare, multiline).
        let end = after
            .find(';')
            .or_else(|| after.find('{'))
            .unwrap_or(after.len());
        after[..end].trim().to_string()
    } else {
        String::new()
    };

    // Parameter list — inside the FIRST top-level `(` ... `)` on the line.
    let params_str: String = if let Some(open) = line.find('(') {
        // Match the closing paren at the same nesting level.
        let bytes = line.as_bytes();
        let mut depth = 0i32;
        let mut close_idx = None;
        for (i, &b) in bytes.iter().enumerate().skip(open) {
            match b {
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        close_idx = Some(i);
                        break;
                    }
                }
                _ => {}
            }
        }
        if let Some(close) = close_idx {
            line[open + 1..close].to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    // Concatenate param + return into one searchable string.
    let combined = format!("{} {}", params_str, ret_type);

    // Owned `String` (NOT preceded by `&`).
    // Negative lookbehind on `&` to avoid matching `&String` (which is rare but
    // we still want to flag — borrow lifetimes of `String` are themselves a
    // code smell because they usually mean a borrowed temporary).
    if regex_lite_match_whole_token(&combined, "String") {
        forbidden.push("String");
    }

    // Result<String, _> / Result<&str, _> / Result<String, ErrorCode> etc.
    if combined.contains("Result<String,") || combined.contains("Result<String >") {
        forbidden.push("Result<String, _>");
    }
    if combined.contains("Result<&str,") || combined.contains("Result<&str >") {
        forbidden.push("Result<&str, _>");
    }

    // Numeric primitives (and `usize`/`isize`).
    for kw in &["i32", "i64", "u32", "u64", "f32", "f64", "usize", "isize"] {
        if regex_lite_match_whole_token(&combined, kw) {
            forbidden.push(kw);
        }
    }

    // `char` — only single-token usage, not inside identifiers.
    if regex_lite_match_whole_token(&combined, "char") {
        forbidden.push("char");
    }

    forbidden
}

/// Lightweight whole-token match: returns true if `needle` appears in
/// `haystack` as a standalone identifier (not preceded or followed by
/// alphanumeric or `_`). Avoids pulling in the `regex` crate for a check this
/// small. Uses ASCII byte-level comparisons.
fn regex_lite_match_whole_token(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return false;
    }
    let h = haystack.as_bytes();
    let n = needle.as_bytes();
    let nlen = n.len();
    if h.len() < nlen {
        return false;
    }
    let is_ident_cont = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    let mut i = 0;
    while i + nlen <= h.len() {
        if &h[i..i + nlen] == n {
            let before_ok = i == 0 || !is_ident_cont(h[i - 1]);
            let after_ok = i + nlen == h.len() || !is_ident_cont(h[i + nlen]);
            if before_ok && after_ok {
                return true;
            }
        }
        i += 1;
    }
    false
}

impl ContractRoleChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check_port(&self, source: &SourceContentVO) -> Vec<LintResult> {
        let mut violations = Vec::new();
        self.check_contract_primitive(source, &mut violations);
        violations
    }

    pub fn check_protocol(&self, source: &SourceContentVO) -> Vec<LintResult> {
        let mut violations = Vec::new();
        self.check_contract_primitive(source, &mut violations);
        violations
    }

    pub fn check_aggregate(
        &self,
        source: &SourceContentVO,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        if def.role.forbidden_inheritance.values.is_empty() {
            return;
        }
        let content = source.content.value();
        let file = source.file_path.value();
        let mut forbidden_traits: Vec<String> = Vec::new();
        for line in content.lines() {
            let t = line.trim();
            let is_import = t.starts_with("use ")
                || (t.starts_with("from ") && t.contains(" import "))
                || (t.starts_with("import ") && t.contains(" from "));
            if !is_import {
                continue;
            }
            for pattern in &def.role.forbidden_inheritance.values {
                let (layer, suffixes) = Self::resolve_scope(pattern);
                let lower = t.to_lowercase();
                let layer_match = lower.contains(&format!("{}::", layer))
                    || lower.contains(&format!("::{}::", layer))
                    || lower.contains(&format!("{}.", layer))
                    || lower.contains(&format!(".{}.", layer))
                    || lower.contains(&format!("{}/", layer))
                    || lower.contains(&format!("/{}/", layer));
                if !layer_match {
                    continue;
                }
                if !suffixes.is_empty()
                    && !suffixes.iter().any(|s| {
                        lower.contains(&format!("_{}", s)) || lower.contains(&format!("::{}", s))
                    })
                {
                    continue;
                }
                if let Some(name) = t.split("::").last() {
                    let tn = name
                        .trim_end_matches(';')
                        .trim()
                        .trim_start_matches('{')
                        .trim_end_matches('}')
                        .split(',')
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_string();
                    if !tn.is_empty() {
                        forbidden_traits.push(tn);
                    }
                }
            }
        }
        for trait_name in &forbidden_traits {
            let rust_pattern = format!("impl {} for ", trait_name);
            let py_pattern = format!("({}", trait_name);
            let js_extends = format!("extends {}", trait_name);
            let js_implements = format!("implements {}", trait_name);
            if content.contains(&rust_pattern)
                || content.contains(&py_pattern)
                || content.contains(&js_extends)
                || content.contains(&js_implements)
            {
                let msg = aes013_forbidden_inheritance(trait_name);
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES013",
                    Severity::HIGH,
                    &msg,
                ));
            }
        }
    }

    /// Detect primitive type usage in contract method signatures (AES402).
    ///
    /// Scans ONLY method signatures inside `pub trait Name { ... }` blocks, NOT
    /// the whole file. This prevents false positives from:
    ///   * doc-comments mentioning "String" or "str" in prose
    ///   * identifier names that contain primitive type names
    ///     (e.g. `StringBuilder`, `MyFloat`)
    ///   * language words in English comments ("Float values are rounded")
    ///
    /// Rules:
    ///   * `&str` (borrowed string slice) is allowed — borrow lifetimes preclude
    ///     replacement with a taxonomy VO without major API changes. It is the
    ///     idiomatic Rust type for file paths, error messages, and other borrowed
    ///     string data passed into trait methods.
    ///   * `bool` is allowed — represents a semantic toggle that is not meaningfully
    ///     expressible as a VO without ceremony.
    ///   * `String` (owned) is FORBIDDEN — must be replaced with a taxonomy VO
    ///     (`LintMessage`, `ErrorMessage`, `SymbolName`, `JobId`, etc.).
    ///   * `Result<String, _>` / `Result<&str, _>` are FORBIDDEN — error variants
    ///     must use a defined `taxonomy_*_error` type, not a raw `String`.
    ///   * Numeric primitives `i32/i64/u32/u64/f32/f64` and `char` are FORBIDDEN —
    ///     must be wrapped in domain VOs (`Count`, `LineNumber`, `ColumnNumber`,
    ///     `Duration`, etc.) or new domain-specific VOs.
    ///
    /// Only the parameter types and return type of each trait method signature
    /// are inspected — implementation bodies are out of scope (the contract
    /// layer is the public interface; internal representations are an adapter
    /// concern).
    fn check_contract_primitive(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let file = source.file_path.value();
        let content = source.content.value();
        let detector =
            shared::source_parsing::taxonomy_language_detector_helper::LanguageDetector::new();
        let det_lang = detector.detect(&source.file_path);
        let is_rs = det_lang == DetLang::Rust;
        let is_py = det_lang == DetLang::Python;
        let is_js = det_lang == DetLang::JavaScript || det_lang == DetLang::TypeScript;
        if !is_rs && !is_py && !is_js {
            return;
        }

        // For each language, parse trait/method signatures and flag forbidden
        // primitive types in params and return types.
        //
        // Note: the AES402 helper functions are Rust-specific — they parse
        // `pub trait Name { fn ... ; }` syntax. Python and JS contract files
        // are routed through this same code path because the same Rust-style
        // primitive types can appear in cross-language adapter contracts.
        let _ = is_py;
        let _ = is_js;
        for (line_no, sig) in extract_trait_method_signatures(content) {
            let forbidden = signature_uses_forbidden_primitive(&sig);
            if forbidden.is_empty() {
                continue;
            }
            let lang = if is_rs {
                Language::Rust
            } else if is_py {
                Language::Python
            } else {
                Language::JavaScript
            };
            let msg = AesRoleViolation::ContractPrimitive { reason: None }
                .with_language(lang)
                .to_string();
            violations.push(LintResult::new_arch(
                file,
                line_no,
                "AES402",
                Severity::HIGH,
                msg,
            ));
        }
    }

    fn resolve_scope(scope: &str) -> (&str, Vec<&str>) {
        if let Some(paren) = scope.find('(') {
            let layer = scope[..paren].trim();
            let inner = scope[paren + 1..].trim_end_matches(')').trim();
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
            (layer, suffixes)
        } else {
            (scope.trim(), vec![])
        }
    }
}

impl IContractRoleChecker for ContractRoleChecker {
    fn check_port(
        &self,
        source: &SourceContentVO,
    ) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
        self.check_port(source)
    }
    fn check_protocol(
        &self,
        source: &SourceContentVO,
    ) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
        self.check_protocol(source)
    }
    fn check_aggregate(
        &self,
        source: &SourceContentVO,
        def: &shared::taxonomy_definition_vo::LayerDefinition,
        violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
        self.check_aggregate(source, def, violations);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_single_line_method_signatures() {
        let src = "\
pub trait IFoo {
    fn a(&self) -> bool;
    fn b(&self, x: &str) -> usize;
    fn c(&self) -> Result<String, ErrorCode>;
}
";
        let sigs = extract_trait_method_signatures(src);
        assert_eq!(sigs.len(), 3);
        assert!(sigs[0].1.contains("fn a"));
        assert!(sigs[1].1.contains("fn b"));
        assert!(sigs[2].1.contains("fn c"));
    }

    #[test]
    fn ignores_free_functions_and_impls() {
        let src = "\
fn helper() -> String { ... }
impl Foo {
    pub fn method(&self) -> String { ... }
}
pub trait IFoo {
    fn only(&self) -> usize;
}
";
        let sigs = extract_trait_method_signatures(src);
        assert_eq!(sigs.len(), 1);
        assert!(sigs[0].1.contains("fn only"));
    }

    #[test]
    fn detects_string_param() {
        assert_eq!(
            signature_uses_forbidden_primitive("fn f(&self, msg: String);"),
            vec!["String"],
        );
    }

    #[test]
    fn detects_result_string() {
        let v = signature_uses_forbidden_primitive(
            "fn f(&self, p: &Path) -> Result<String, ErrorCode>;",
        );
        assert!(v.contains(&"String"));
        assert!(v.contains(&"Result<String, _>"));
    }

    #[test]
    fn detects_result_borrowed_str() {
        let v =
            signature_uses_forbidden_primitive("fn f(&self, p: &Path) -> Result<&str, ErrorCode>;");
        assert!(v.contains(&"Result<&str, _>"));
    }

    #[test]
    fn detects_numeric_primitives() {
        assert!(signature_uses_forbidden_primitive("fn f(&self, n: i32) -> i64;").contains(&"i32"));
        assert!(
            signature_uses_forbidden_primitive("fn f(&self, n: usize) -> bool;").contains(&"usize")
        );
        assert!(signature_uses_forbidden_primitive("fn f(&self) -> f64;").contains(&"f64"));
    }

    #[test]
    fn allows_borrowed_str() {
        assert!(signature_uses_forbidden_primitive(
            "fn f(&self, file: &str, content: &str) -> bool;"
        )
        .is_empty());
    }

    #[test]
    fn allows_bool() {
        assert!(signature_uses_forbidden_primitive("fn f(&self) -> bool;").is_empty());
        assert!(signature_uses_forbidden_primitive("fn f(&self, flag: bool) -> bool;").is_empty());
    }

    #[test]
    fn does_not_match_substring_of_identifier() {
        // `StringBuilder` (an identifier) must NOT trigger String.
        assert!(signature_uses_forbidden_primitive("fn f(&self, s: StringBuilder);").is_empty());
        // `MyFloat` must NOT trigger float.
        assert!(signature_uses_forbidden_primitive("fn f(&self, x: MyFloat);").is_empty());
    }

    #[test]
    fn empty_signature_is_clean() {
        assert!(signature_uses_forbidden_primitive("").is_empty());
        assert!(signature_uses_forbidden_primitive("   ").is_empty());
    }
}
```

---

## File: crates/role-rules/src/capabilities_infrastructure_role_auditor.rs

```rust
// PURPOSE: InfrastructureRoleChecker — IInfrastructureRoleChecker for AES404: infrastructure has no port implementation
//
// ALGORITHM:
//   1. check_port_implementation checks if the file imports a port/protocol
//      (contains `_port::` or `_protocol::` after `use `) but has no `impl ... for ...`
//      block. If an import exists without a corresponding impl, emits
//      InfrastructureNoPort violation.
//
// NOTE: This is a simple keyword-based heuristic. It may miss cases where the
//      implementation is in a different file or uses a different pattern.
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::role_rules::contract_infrastructure_role_protocol::IInfrastructureRoleChecker;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_source_vo::SourceContentVO;

pub struct InfrastructureRoleChecker {}

impl Default for InfrastructureRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl InfrastructureRoleChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl IInfrastructureRoleChecker for InfrastructureRoleChecker {
    fn check_port_implementation(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<LintResult>,
    ) {
        let file = source.file_path.value();
        let content = source.content.value();
        let detector =
            shared::source_parsing::taxonomy_language_detector_helper::LanguageDetector::new();
        let lang = detector.detect(&source.file_path);
        let is_rs = lang == shared::source_parsing::contract_language_detector_port::Language::Rust;
        let is_py =
            lang == shared::source_parsing::contract_language_detector_port::Language::Python;
        let is_js = lang
            == shared::source_parsing::contract_language_detector_port::Language::JavaScript
            || lang
                == shared::source_parsing::contract_language_detector_port::Language::TypeScript;

        if is_rs {
            self._check_rust(file, content, violations);
        } else if is_py {
            self._check_python(file, content, violations);
        } else if is_js {
            self._check_js(file, content, violations);
        }
    }
}

impl InfrastructureRoleChecker {
    fn _check_rust(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let has_import = content.contains("use ")
            && (content.contains("_port::") || content.contains("_protocol::"));
        if !has_import {
            return;
        }
        let has_impl = content.contains("impl ")
            && (content.contains(" for ")
                || content.contains(" for")
                || content.contains("impl<T")
                || content.contains("impl<"));
        if !has_impl {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES404",
                Severity::HIGH,
                AesRoleViolation::InfrastructureNoPort { reason: None }.to_string(),
            ));
        }
    }

    fn _check_python(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let has_import = (content.contains("import ") || content.contains("from "))
            && (content.contains("_port") || content.contains("_protocol"));
        if !has_import {
            return;
        }
        let has_impl = content.contains("class ")
            && (content.contains("(_port") || content.contains("(_protocol"));
        if !has_impl {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES404",
                Severity::HIGH,
                AesRoleViolation::InfrastructureNoPort { reason: None }.to_string(),
            ));
        }
    }

    fn _check_js(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let has_import = content.contains("import ")
            && (content.contains("_port") || content.contains("_protocol"));
        if !has_import {
            return;
        }
        let has_impl = content.contains("implements");
        if !has_impl {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES404",
                Severity::HIGH,
                AesRoleViolation::InfrastructureNoPort { reason: None }.to_string(),
            ));
        }
    }
}
```

---

## File: crates/role-rules/src/capabilities_surface_role_auditor.rs

```rust
// PURPOSE: SurfaceRoleChecker — ISurfaceRoleChecker for AES406: smart/utility/passive surface role checks
//
// ALGORITHM:
//   1. check_fn_count_limit — Counts `fn ` occurrences. If > 15, flags SurfaceRoleViolation.
//   2. check_surface_hierarchy — Iterates files, filters to surface-prefixed or surface-dir files,
//      skips smart surfaces (_command, _controller, _page, _entry) and init files, then runs
//      _check_passive on remaining (passive) surfaces.
//   3. _check_passive — Reads file content, detects language (Rust/Python/JS), dispatches to
//      language-specific passive checks:
//      - Rust: Scans impl blocks for too many public methods (>10) or methods exceeding 80 lines.
//      - Python: Scans class definitions for too many public methods, method length, if-nesting depth.
//      - JS/TS: Same as Python but uses JS-specific class/method regex.
//   4. check_surface_roles (async, IAnalyzer-dependent) — Uses analyzer.detect_layer + layer_map
//      to check no_domain_logic on non-smart surfaces (control_flow_count > 3).
//
// NOTE: check_smart_surface / check_utility_surface / check_passive_surface are no-ops because
//      the actual surface role checks run via check_surface_hierarchy (passive checks) and
//      check_surface_roles (no-domain-logic checks) which are the primary entry points.
//      These trait methods are required by ISurfaceRoleChecker but are intentionally empty.
use once_cell::sync::Lazy;
use regex::Regex;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;
use shared::role_rules::taxonomy_layer_names_vo::layer_surfaces;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::source_parsing::contract_language_detector_port::Language as DetLang;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::{ColumnNumber, LineNumber};
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::LintMessage;
use shared::taxonomy_source_vo::SourceContentVO;

const MAX_PUBLIC_METHODS: usize = 10;
const MAX_FUNCTION_BODY_LINES: i64 = 80;
const MAX_IF_DEPTH: usize = 3;

// Regex: detect Python function/method definitions inside a class
static PY_METHOD_RE: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^(?:async\s+)?def\s+(\w+)\s*\(").ok());

// Regex: detect class definitions
static PY_CLASS_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r"^class\s+(\w+)").ok());

// Regex: detect JavaScript/TypeScript class definitions
static JS_CLASS_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r"^export\s+class\s+(\w+)").ok());

// Regex: detect JavaScript/TypeScript method definitions
static JS_METHOD_RE: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^\s*(?:public|private|protected)?\s*(?:async\s+)?(\w+)\s*\(").ok());

// Regex: detect if statements for nesting depth
static IF_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r"^\s*if\s+").ok());

// Regex: detect Rust impl blocks
static RUST_IMPL_RE: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^\s*(?:pub\s+)?(?:unsafe\s+)?impl\s+").ok());

// Regex: detect Rust fn definitions
static RUST_FN_RE: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)\s*\(").ok());

fn aes406_passive_violation_details(file: &str, details: &str) -> String {
    format!("AES406 SURFACE_ROLE: Surface file '{}' contains active domain logic:\n{}\nWHY? Surfaces must be passive I/O boundaries.\nFIX: Move logic to capabilities/agent layers.", file, details)
}

pub struct SurfaceRoleChecker {}
fn make_adapter(name: &str) -> Option<AdapterName> {
    AdapterName::new(name).ok()
}
impl Default for SurfaceRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl SurfaceRoleChecker {
    pub fn new() -> Self {
        Self {}
    }
    pub fn check_smart(&self) -> Vec<LintResult> {
        vec![]
    }
    pub fn check_utility(&self) -> Vec<LintResult> {
        vec![]
    }
    pub fn check_passive(&self) -> Vec<LintResult> {
        vec![]
    }

    pub fn check_fn_count_limit(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let content = source.content.value();
        let file = source.file_path.value();
        let detector =
            shared::source_parsing::taxonomy_language_detector_helper::LanguageDetector::new();
        let lang = detector.detect(&source.file_path);
        let fn_keyword = if lang == DetLang::Python {
            "def "
        } else if lang == DetLang::JavaScript || lang == DetLang::TypeScript {
            "function "
        } else {
            "fn "
        };
        if content.matches(fn_keyword).count() > 15 {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES406",
                Severity::HIGH,
                AesRoleViolation::SurfaceRoleViolation { reason: None },
            ));
        }
    }

    // ---- moved from capabilities_role_checker.rs ----

    pub async fn check_surface_roles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &shared::source_parsing::taxonomy_paths_vo::FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in &files.values {
            let layer_vo = match analyzer.detect_layer(f, root_dir) {
                Some(l) => l,
                None => continue,
            };

            let is_surface = layer_vo == layer_surfaces()
                || layer_vo
                    .value
                    .starts_with(&format!("{}(", layer_surfaces().value));
            if !is_surface {
                continue;
            }

            let definition = match analyzer.layer_map().values.get(&layer_vo) {
                Some(d) => d.clone(),
                None => continue,
            };

            if definition.role.no_domain_logic.value {
                let basename = std::path::Path::new(&f.value)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                let is_smart = basename.ends_with("_command")
                    || basename.ends_with("_controller")
                    || basename.ends_with("_page")
                    || basename.ends_with("_entry");
                if !is_smart {
                    self._check_no_domain_logic(f, &definition, analyzer, results, "AES406");
                }
            }
        }
    }

    fn _check_no_domain_logic(
        &self,
        f: &FilePath,
        _definition: &LayerDefinition,
        analyzer: &dyn IAnalyzer,
        results: &mut LintResultList,
        code: &str,
    ) {
        let control_flow_count = analyzer.parser().get_control_flow_count(f);
        if control_flow_count.value > 3 {
            results.push(LintResult {
                file: f.clone(),
                line: LineNumber::new(0),
                column: ColumnNumber::new(0),
                code: ErrorCode::raw(code),
                message: LintMessage::new(AesRoleViolation::NoDomainLogic { reason: None }),
                source: make_adapter("architecture"),
                severity: Severity::HIGH,
                enclosing_scope: None,
                related_locations: LocationList::new(),
            });
        }
    }

    // ---- migrated from capabilities_hierarchy_checker.rs ----

    /// Main entry point — run AES406 passive surface check.
    pub fn check_surface_hierarchy(
        &self,
        files: &[FilePath],
        _root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in files {
            if !is_in_surfaces(f) {
                continue;
            }
            if is_init(f) {
                continue;
            }

            // AES406: check if file is passive
            self._check_passive(f, results);
        }
    }

    /// Check if a surface file is passive (thin I/O boundary).
    /// Smart surfaces (_command, _controller, _page, _entry) are exempted
    /// — they are expected to contain orchestration logic.
    fn _check_passive(&self, f: &FilePath, results: &mut LintResultList) {
        let f_str = f.to_string();
        let basename = std::path::Path::new(&f_str)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        if basename.ends_with("_command")
            || basename.ends_with("_controller")
            || basename.ends_with("_page")
            || basename.ends_with("_entry")
        {
            return;
        }

        let content = match std::fs::read_to_string(f.to_string()) {
            Ok(c) => c,
            Err(_) => return,
        };

        let lines: Vec<&str> = content.lines().collect();
        let mut violations: Vec<String> = Vec::new();
        let detector =
            shared::source_parsing::taxonomy_language_detector_helper::LanguageDetector::new();
        let lang = detector.detect(f);

        match lang {
            DetLang::Rust => self._check_rust_passive(f, &lines, &mut violations),
            DetLang::JavaScript | DetLang::TypeScript => {
                self._check_javascript_passive(f, &lines, &mut violations)
            }
            _ => self._check_python_passive(f, &lines, &mut violations),
        }

        if !violations.is_empty() {
            self._report_aes0306(f, violations, results);
        }
    }

    /// Rust-specific passive check: detect impl blocks and fn methods.
    fn _check_rust_passive(&self, _f: &FilePath, lines: &[&str], violations: &mut Vec<String>) {
        let impl_re = match &*RUST_IMPL_RE {
            Some(r) => r,
            None => return,
        };
        let fn_re = match &*RUST_FN_RE {
            Some(r) => r,
            None => return,
        };

        let mut current_impl: Option<(String, usize)> = None;
        let mut methods: Vec<(String, usize, Option<usize>)> = Vec::new();
        let mut impl_indent: usize = 0;

        for (i, raw_line) in lines.iter().enumerate() {
            let trimmed = raw_line.trim();
            if trimmed.starts_with("use ") || trimmed.starts_with("//") || trimmed.starts_with("/*")
            {
                continue;
            }
            if trimmed.starts_with("pub mod ") || trimmed.starts_with("mod ") {
                continue;
            }

            if impl_re.captures(trimmed).is_some() {
                if let Some((_name, start)) = current_impl.take() {
                    self._add_impl_violations(&methods, "impl", start, violations);
                }
                let trait_name = if let Some(pos) = trimmed.find(" for ") {
                    trimmed[pos + 5..].trim().to_string()
                } else {
                    String::new()
                };
                current_impl = Some((trait_name, i));
                impl_indent = raw_line.len() - raw_line.trim_start().len();
                methods.clear();
                continue;
            }

            if let (Some((name, _start)), Some(cap)) = (&current_impl, fn_re.captures(trimmed)) {
                let method_name = cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
                if !method_name.starts_with('_')
                    && !name.contains("Drop")
                    && !name.contains("Clone")
                {
                    let _m_indent = raw_line.len() - raw_line.trim_start().len();
                    let mut end_line = lines.len();
                    for (k, line) in lines.iter().enumerate().skip(i + 1) {
                        let next = line.trim();
                        if next.starts_with("fn ") || next.starts_with("impl ") {
                            end_line = k;
                            break;
                        }
                    }
                    methods.push((method_name, i + 1, Some(end_line)));
                }
            }

            // If we exit an impl block, finalize
            if current_impl.is_some() {
                let line_indent = raw_line.len() - raw_line.trim_start().len();
                if !trimmed.is_empty() && trimmed != "}" && line_indent <= impl_indent {
                    if let Some((_name, start)) = current_impl.take() {
                        self._add_impl_violations(&methods, "impl", start, violations);
                    }
                }
            }
        }
        // Finalize any remaining impl block
        if let Some((_name, start)) = current_impl.take() {
            self._add_impl_violations(&methods, "impl", start, violations);
        }
    }

    fn _add_impl_violations(
        &self,
        methods: &[(String, usize, Option<usize>)],
        impl_name: &str,
        _start: usize,
        violations: &mut Vec<String>,
    ) {
        if methods.len() > MAX_PUBLIC_METHODS {
            violations.push(format!(
                "Impl block '{}' has {} public methods (max {})",
                impl_name,
                methods.len(),
                MAX_PUBLIC_METHODS
            ));
        }
        for (method_name, s, e) in methods {
            if let Some(end_line) = e {
                let body_len = (*end_line as i64) - (*s as i64);
                if body_len > MAX_FUNCTION_BODY_LINES {
                    violations.push(format!(
                        "Method '{}' is {} lines (max {})",
                        method_name, body_len, MAX_FUNCTION_BODY_LINES
                    ));
                }
            }
        }
    }

    /// Python-specific passive check: detect classes and methods.
    fn _check_python_passive(&self, _f: &FilePath, lines: &[&str], violations: &mut Vec<String>) {
        for (i, raw_line) in lines.iter().enumerate() {
            let stripped = raw_line.trim();
            let class_re = match &*PY_CLASS_RE {
                Some(r) => r,
                None => continue,
            };
            if let Some(cap) = class_re.captures(stripped) {
                let class_name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                let indent = raw_line.len() - raw_line.trim_start().len();

                let mut pub_methods: Vec<(String, usize, Option<usize>)> = Vec::new();

                for j in (i + 1)..lines.len() {
                    let method_line = lines[j];
                    if method_line.trim().is_empty() {
                        continue;
                    }
                    let m_indent = method_line.len() - method_line.trim_start().len();

                    if m_indent <= indent && !method_line.trim().is_empty() {
                        break;
                    }

                    let method_re = match &*PY_METHOD_RE {
                        Some(r) => r,
                        None => break,
                    };
                    if let Some(mcap) = method_re.captures(method_line.trim()) {
                        let method_name = mcap.get(1).map(|m| m.as_str()).unwrap_or("");
                        if !method_name.starts_with('_') {
                            let mut end_line = lines.len();
                            for (k, next) in lines.iter().enumerate().skip(j + 1) {
                                if !next.trim().is_empty() {
                                    let n_indent = next.len() - next.trim_start().len();
                                    if n_indent <= m_indent {
                                        end_line = k;
                                        break;
                                    }
                                }
                            }
                            pub_methods.push((method_name.to_string(), j + 1, Some(end_line)));
                        }
                    }
                }

                self._check_methods_too_public(class_name, &pub_methods, violations);
                self._check_method_lengths(class_name, lines, &pub_methods, violations);
                self._check_method_nesting(class_name, lines, &pub_methods, violations);
            }
        }
    }

    /// JavaScript/TypeScript-specific passive check: detect classes and methods.
    fn _check_javascript_passive(
        &self,
        _f: &FilePath,
        lines: &[&str],
        violations: &mut Vec<String>,
    ) {
        let class_re = match &*JS_CLASS_RE {
            Some(r) => r,
            None => return,
        };
        let method_re = match &*JS_METHOD_RE {
            Some(r) => r,
            None => return,
        };

        for (i, raw_line) in lines.iter().enumerate() {
            let stripped = raw_line.trim();
            if let Some(cap) = class_re.captures(stripped) {
                let class_name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                let indent = raw_line.len() - raw_line.trim_start().len();

                let mut pub_methods: Vec<(String, usize, Option<usize>)> = Vec::new();

                for j in (i + 1)..lines.len() {
                    let method_line = lines[j];
                    if method_line.trim().is_empty() {
                        continue;
                    }
                    let m_indent = method_line.len() - method_line.trim_start().len();

                    if m_indent <= indent && !method_line.trim().is_empty() {
                        break;
                    }

                    if let Some(mcap) = method_re.captures(method_line.trim()) {
                        let method_name = mcap.get(1).map(|m| m.as_str()).unwrap_or("");
                        if !method_name.starts_with('_') {
                            let mut end_line = lines.len();
                            for (k, next) in lines.iter().enumerate().skip(j + 1) {
                                if !next.trim().is_empty() {
                                    let n_indent = next.len() - next.trim_start().len();
                                    if n_indent <= m_indent {
                                        end_line = k;
                                        break;
                                    }
                                }
                            }
                            pub_methods.push((method_name.to_string(), j + 1, Some(end_line)));
                        }
                    }
                }

                self._check_methods_too_public(class_name, &pub_methods, violations);
                self._check_method_lengths(class_name, lines, &pub_methods, violations);
                self._check_method_nesting(class_name, lines, &pub_methods, violations);
            }
        }
    }

    // -- AES406 sub-checks ---------------------------------------------------

    /// AES406: too many public methods in a surface class.
    fn _check_methods_too_public(
        &self,
        class_name: &str,
        pub_methods: &[(String, usize, Option<usize>)],
        violations: &mut Vec<String>,
    ) {
        if pub_methods.len() > MAX_PUBLIC_METHODS {
            violations.push(format!(
                "Class '{}' has {} public methods (max {})",
                class_name,
                pub_methods.len(),
                MAX_PUBLIC_METHODS
            ));
        }
    }

    /// AES406: method body exceeds line limit.
    fn _check_method_lengths(
        &self,
        class_name: &str,
        _lines: &[&str],
        pub_methods: &[(String, usize, Option<usize>)],
        violations: &mut Vec<String>,
    ) {
        for (method_name, start, end) in pub_methods {
            if let Some(end_line) = end {
                let body_len = (*end_line as i64) - (*start as i64);
                if body_len > MAX_FUNCTION_BODY_LINES {
                    violations.push(format!(
                        "Method '{}.{}' is {} lines (max {})",
                        class_name, method_name, body_len, MAX_FUNCTION_BODY_LINES
                    ));
                }
            }
        }
    }

    /// AES406: method control-flow nesting exceeds limit.
    fn _check_method_nesting(
        &self,
        class_name: &str,
        lines: &[&str],
        pub_methods: &[(String, usize, Option<usize>)],
        violations: &mut Vec<String>,
    ) {
        for (method_name, start, end) in pub_methods {
            let end_line = end.unwrap_or(lines.len());
            let mut max_depth: usize = 0;

            for i in *start..end_line {
                if i >= lines.len() {
                    break;
                }
                let line = lines[i];
                let trimmed = line.trim();

                if IF_RE.as_ref().is_some_and(|re| re.is_match(trimmed)) {
                    let indent = line.len() - line.trim_start().len();
                    let depth = indent / 4;
                    if depth > max_depth {
                        max_depth = depth;
                    }
                }
            }

            if max_depth > MAX_IF_DEPTH {
                violations.push(format!(
                    "Method '{}.{}' has deep control flow (if-nesting > {})",
                    class_name, method_name, MAX_IF_DEPTH
                ));
            }
        }
    }

    /// Append a single AES406 result to the results list.
    fn _report_aes0306(&self, f: &FilePath, violations: Vec<String>, results: &mut LintResultList) {
        let detail: String = violations
            .iter()
            .map(|v| format!("  - {}", v))
            .collect::<Vec<_>>()
            .join("\n");

        results.push(LintResult {
            file: f.clone(),
            line: LineNumber::new(1),
            column: ColumnNumber::new(1),
            code: ErrorCode::raw("AES406"),
            message: LintMessage::new(aes406_passive_violation_details(&f.to_string(), &detail)),
            source: Some(AdapterName::raw("surface_hierarchy")),
            severity: Severity::HIGH,
            enclosing_scope: None,
            related_locations: LocationList::new(),
        });
    }
}

// --- helpers -----------------------------------------------------------------

/// Check if the file is a surface file by filename prefix `surface_` or `surfaces_` or directory `surfaces/`.
fn is_in_surfaces(f: &FilePath) -> bool {
    let path_str = f.to_string();
    let basename = path_str.rsplit('/').next().unwrap_or(&path_str);
    let stem = basename.split('.').next().unwrap_or(basename);
    if stem.starts_with("surface_") || stem.starts_with("surfaces_") {
        return true;
    }
    if let Some(parent) = path_str.rsplit('/').nth(1) {
        if parent == "surfaces" || parent == "surface" || parent == "cli_commands" {
            return true;
        }
    }
    false
}

/// Check if the file is a barrel/init file.
fn is_init(f: &FilePath) -> bool {
    let path_str = f.to_string();
    path_str.ends_with("__init__.py")
        || path_str.ends_with("mod.rs")
        || path_str.ends_with("index.ts")
        || path_str.ends_with("index.js")
}

impl ISurfaceRoleChecker for SurfaceRoleChecker {
    fn check_smart_surface(
        &self,
        _source: &SourceContentVO,
        _violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_utility_surface(
        &self,
        _source: &SourceContentVO,
        _violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_passive_surface(
        &self,
        _source: &SourceContentVO,
        _violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_fn_count_limit(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
        self.check_fn_count_limit(source, violations);
    }
}

#[cfg(test)]
mod tests {
    use super::{is_in_surfaces, is_init, FilePath};

    #[test]
    fn test_is_in_surfaces() {
        let f = FilePath::new("src/surfaces/surface_handler.py")
            .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
        assert!(is_in_surfaces(&f));

        let f = FilePath::new("src/capabilities/capabilities_not_checker.py")
            .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
        assert!(!is_in_surfaces(&f));

        let f = FilePath::new("src/cli-commands/surface_check_command.rs")
            .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
        assert!(is_in_surfaces(&f));
    }

    #[test]
    fn test_is_init() {
        let f = FilePath::new("src/surfaces/__init__.py")
            .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
        assert!(is_init(&f));

        let f = FilePath::new("src/surfaces/handler.py")
            .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
        assert!(!is_init(&f));
    }
}
```

---

## File: crates/role-rules/src/capabilities_taxonomy_role_auditor.rs

```rust
// PURPOSE: TaxonomyRoleChecker — ITaxonomyRoleChecker for AES401: taxonomy primitive usage + constant purity
//
// ALGORITHM:
//   1. scan_primitives (entity/error/event) — Detects primitive type annotations
//      in taxonomy files. For each line with a `:`, extracts the type after the colon
//      and checks against language-specific primitive lists (RUST_PRIMITIVES,
//      PY_PRIMITIVES, JS_PRIMITIVES). Handles generic wrappers (Option<X>, Vec<X>)
//      by checking the inner type. Skips: pub(crate) value: Primitive (newtype pattern),
//      From<Primitive>/visit_* from() methods (trait-mandated boundaries).
//   2. check_constant — Scans _constant files for non-constant declarations.
//      Allows only: pub const, pub static, use/pub use/pub(crate) use.
//      Flags struct, enum, fn, impl, mod, trait, class, type declarations.
//
// NOTE: scan_primitives uses language-specific primitive sets. Only Rust, Python,
//      and JavaScript/TypeScript are currently supported.
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::Language;
use shared::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::source_parsing::contract_language_detector_port::Language as DetLang;
use shared::taxonomy_name_vo::SymbolName;
use shared::taxonomy_source_vo::SourceContentVO;
use std::path::Path;

fn has_suffix(file: &str, suffix: &str) -> bool {
    let path = Path::new(file);
    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        stem.ends_with(suffix)
    } else {
        false
    }
}

pub struct TaxonomyRoleChecker {}

impl Default for TaxonomyRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl TaxonomyRoleChecker {
    pub fn new() -> Self {
        Self {}
    }

    const RUST_PRIMITIVES: &'static [&'static str] = &[
        "String", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128",
        "usize", "f32", "f64", "bool", "char", "Vec<", "HashMap<", "Option<", "Result<", "Box<",
        "Cell<", "RefCell<", "Arc<", "Mutex<", "Rc<",
    ];
    const PY_PRIMITIVES: &'static [&'static str] = &[
        "str",
        "int",
        "float",
        "bool",
        "list",
        "dict",
        "tuple",
        "set",
        "bytes",
        "None",
        "Any",
        "Optional",
        "Union",
        "List",
        "Dict",
        "Tuple",
        "Set",
        "FrozenSet",
    ];
    const JS_PRIMITIVES: &'static [&'static str] = &[
        "string",
        "number",
        "boolean",
        "any",
        "object",
        "Array",
        "Record",
        "Map",
        "Set",
        "Promise",
        "unknown",
        "never",
        "void",
        "null",
        "undefined",
        "bigint",
        "symbol",
    ];

    fn scan_primitives(source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let file = source.file_path.value();
        let content = source.content.value();
        let detector =
            shared::source_parsing::taxonomy_language_detector_helper::LanguageDetector::new();
        let lang = detector.detect(&source.file_path);
        let primitives: &[&str] = match lang {
            DetLang::Rust => Self::RUST_PRIMITIVES,
            DetLang::Python => Self::PY_PRIMITIVES,
            DetLang::JavaScript | DetLang::TypeScript => Self::JS_PRIMITIVES,
            _ => return,
        };
        let is_rs = lang == DetLang::Rust;
        let is_py = lang == DetLang::Python;

        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if !t.contains(':') {
                continue;
            }
            // Skip class/struct definitions and value object newtype wrappers
            if t.starts_with("class ") || t.starts_with("pub struct ") || t.starts_with("struct ") {
                continue;
            }
            if t.contains("pub(crate) value:") || t.trim_start().starts_with("pub value:") {
                continue;
            }
            // Skip trait-mandated conversion boundaries: From<Primitive>::from()
            // and Visitor::visit_*() method parameters. The primitive type is
            // mandated by the trait definition and cannot be replaced with a VO.
            if t.starts_with("fn from(") || t.starts_with("fn visit_") {
                continue;
            }
            if !(t.ends_with(',')
                || t.ends_with(';')
                || t.ends_with('}')
                || t.ends_with(')')
                || t.ends_with(':')
                || t.contains("-> "))
            {
                continue;
            }
            let after_colon = match t.split_once(':') {
                Some((_, r)) => r.trim(),
                None => continue,
            };
            let type_candidate = after_colon
                .trim_end_matches(',')
                .trim_end_matches(';')
                .trim_end_matches(')')
                .trim_end_matches('}')
                .trim();
            for p in primitives {
                // For generic wrappers like Option<X>, Vec<X>, check if X is a primitive
                if p.ends_with('<') {
                    if type_candidate.starts_with(p) {
                        let inner = type_candidate
                            .strip_prefix(p)
                            .unwrap_or(type_candidate)
                            .trim_end_matches('>');
                        let inner_trimmed = inner.trim();
                        if primitives.iter().any(|prim| {
                            let prim_clean = prim.trim_end_matches('<');
                            inner_trimmed == prim_clean || inner_trimmed.starts_with(prim_clean)
                        }) {
                            let primitive_clean = p.trim_end_matches('<');
                            let lang = if is_rs {
                                Language::Rust
                            } else if is_py {
                                Language::Python
                            } else {
                                Language::JavaScript
                            };
                            let msg = AesRoleViolation::PrimitiveUsage {
                                primitive: SymbolName::new(primitive_clean),
                                reason: None,
                            }
                            .with_language(lang)
                            .to_string();

                            violations.push(LintResult::new_arch(
                                file,
                                i + 1,
                                "AES401",
                                Severity::HIGH,
                                msg,
                            ));
                            break;
                        }
                    }
                    continue; // Skip starts_with for generic wrappers
                }
                // Direct primitive types (String, i64, etc.)
                if type_candidate.starts_with(p) || type_candidate == *p {
                    let primitive_clean = p.trim_end_matches('<');
                    let lang = if is_rs {
                        Language::Rust
                    } else if is_py {
                        Language::Python
                    } else {
                        Language::JavaScript
                    };
                    let msg = AesRoleViolation::PrimitiveUsage {
                        primitive: SymbolName::new(primitive_clean),
                        reason: None,
                    }
                    .with_language(lang)
                    .to_string();

                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES401",
                        Severity::HIGH,
                        msg,
                    ));
                    break;
                }
            }
        }
    }

    pub fn check_vo(&self) -> Vec<LintResult> {
        vec![]
    }

    pub fn check_entity(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        if !has_suffix(source.file_path.value(), "_entity") {
            return;
        }
        Self::scan_primitives(source, violations);
    }

    pub fn check_error(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        if !has_suffix(source.file_path.value(), "_error") {
            return;
        }
        Self::scan_primitives(source, violations);
    }

    pub fn check_event(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        if !has_suffix(source.file_path.value(), "_event") {
            return;
        }
        Self::scan_primitives(source, violations);
    }

    pub fn check_constant(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let file = source.file_path.value();
        let basename = Path::new(file)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("");
        if !basename.ends_with("_constant.rs") && !basename.ends_with("_constant.py") {
            return;
        }
        let content = source.content.value();
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if t.is_empty() || t.starts_with("//") || t.starts_with('#') || t.starts_with("#[") {
                continue;
            }
            if t.starts_with("pub const ") || t.starts_with("pub static ") {
                continue;
            }
            if t.starts_with("use ")
                || t.starts_with("pub use ")
                || t.starts_with("pub(crate) use ")
            {
                continue;
            }
            if t.starts_with("pub struct ")
                || t.starts_with("struct ")
                || t.starts_with("pub enum ")
                || t.starts_with("enum ")
                || t.starts_with("pub fn ")
                || t.starts_with("fn ")
                || t.starts_with("impl ")
                || t.starts_with("pub mod ")
                || t.starts_with("mod ")
                || t.starts_with("pub trait ")
                || t.starts_with("trait ")
                || t.starts_with("class ")
                || t.starts_with("pub type ")
                || t.starts_with("type ")
            {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES401",
                    Severity::HIGH,
                    AesRoleViolation::ConstantPurity { reason: None }.to_string(),
                ));
            }
        }
    }
}

impl ITaxonomyRoleChecker for TaxonomyRoleChecker {
    fn check_vo(&self) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
        self.check_vo()
    }
    fn check_entity(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
        self.check_entity(source, violations);
    }
    fn check_error(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
        self.check_error(source, violations);
    }
    fn check_event(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
        self.check_event(source, violations);
    }
    fn check_constant(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
        self.check_constant(source, violations);
    }
}
```

---

## File: crates/role-rules/src/lib.rs

```rust
// PURPOSE: Module declarations for role-rules (role auditors, orchestrator, container)
pub use shared::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
pub use shared::role_rules::contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
pub use shared::role_rules::contract_role_aggregate::IRoleAggregate;
pub use shared::role_rules::contract_role_protocol::IContractRoleChecker;
pub use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
pub use shared::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;
pub use shared::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;
pub use shared::role_rules::taxonomy_layer_names_vo::{
    layer_agent, layer_capabilities, layer_contract, layer_global, layer_infrastructure,
    layer_root, layer_surfaces, layer_taxonomy, LayerNames,
};
pub mod agent_role_orchestrator;
pub use agent_role_orchestrator::RoleOrchestrator;
pub mod capabilities_agent_role_auditor;
pub use capabilities_agent_role_auditor::AgentRoleChecker;

pub mod capabilities_contract_role_auditor;
pub use capabilities_contract_role_auditor::ContractRoleChecker;
pub mod capabilities_infrastructure_role_auditor;
pub use capabilities_infrastructure_role_auditor::InfrastructureRoleChecker;
pub mod capabilities_capabilities_role_auditor;
pub use capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
pub mod capabilities_surface_role_auditor;
pub use capabilities_surface_role_auditor::SurfaceRoleChecker;
pub mod capabilities_taxonomy_role_auditor;
pub use agent_role_orchestrator::RoleAggregateImpl;
pub use capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
pub mod root_role_rules_container;
```

---

## File: crates/role-rules/src/root_role_rules_container.rs

```rust
// PURPOSE: RoleContainer — wiring for role-rules feature (root layer, wiring only)
use crate::agent_role_orchestrator::RoleOrchestrator;
use shared::role_rules::contract_role_aggregate::IRoleAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use std::sync::Arc;

use crate::capabilities_agent_role_auditor::AgentRoleChecker;
use crate::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
use crate::capabilities_contract_role_auditor::ContractRoleChecker;
use crate::capabilities_infrastructure_role_auditor::InfrastructureRoleChecker;
use crate::capabilities_surface_role_auditor::SurfaceRoleChecker;
use crate::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;

use crate::agent_role_orchestrator::RoleAggregateImpl;

pub struct RoleContainer {
    aggregate: Arc<dyn IRoleAggregate>,
    config: shared::config_system::taxonomy_config_vo::ArchitectureConfig,
}

impl RoleContainer {
    pub fn new() -> Self {
        Self::new_with_config(shared::config_system::taxonomy_config_vo::default_aes_config())
    }

    pub fn new_with_config(
        config: shared::config_system::taxonomy_config_vo::ArchitectureConfig,
    ) -> Self {
        let aggregate: Arc<dyn IRoleAggregate> = Arc::new(RoleAggregateImpl::new(
            Arc::new(TaxonomyRoleChecker::new()),
            Arc::new(ContractRoleChecker::new()),
            Arc::new(InfrastructureRoleChecker::new()),
            Arc::new(CapabilitiesRoleChecker::new()),
            Arc::new(SurfaceRoleChecker::new()),
            Arc::new(AgentRoleChecker::new()),
        ));
        Self { aggregate, config }
    }

    pub fn aggregate(&self) -> Arc<dyn IRoleAggregate> {
        self.aggregate.clone()
    }

    pub fn orchestrator(&self) -> Arc<dyn IRoleRunnerAggregate> {
        Arc::new(RoleOrchestrator::new(self.aggregate.clone(), &self.config))
    }
}

impl Default for RoleContainer {
    fn default() -> Self {
        Self::new()
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
pub mod taxonomy_governance_entity;
pub mod taxonomy_import_source_vo;
pub mod taxonomy_operation_error;
pub mod taxonomy_violation_code_analysis_vo;
pub use taxonomy_violation_code_analysis_vo::{AesCodeAnalysisViolation, Language};
```

---

## File: crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs

```rust
// PURPOSE: AesCodeAnalysisViolation — violation messages for code quality rules (AES301-305)
use std::fmt;

use crate::common::taxonomy_message_vo::LintMessage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    JavaScript,
    Python,
    TypeScript,
}

impl Language {
    pub fn from_adapter_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "clippy" | "rust" => Self::Rust,
            "eslint" | "prettier" | "tsc" | "javascript" => Self::JavaScript,
            "ruff" | "mypy" | "bandit" | "python" => Self::Python,
            "typescript" => Self::TypeScript,
            _ => Self::Rust,
        }
    }

    pub fn struct_keyword(&self) -> &'static str {
        match self {
            Self::Rust => "struct",
            Self::JavaScript | Self::TypeScript => "class/interface",
            Self::Python => "class/Protocol",
        }
    }

    pub fn type_kw(&self) -> &'static str {
        match self {
            Self::Rust => "type",
            Self::JavaScript | Self::TypeScript => "interface/type",
            Self::Python => "Protocol/type",
        }
    }

    pub fn interface_kw(&self) -> &'static str {
        match self {
            Self::Rust => "trait",
            Self::JavaScript | Self::TypeScript => "interface",
            Self::Python => "Protocol",
        }
    }

    pub fn inherits_kw(&self) -> &'static str {
        match self {
            Self::Rust => "implements",
            Self::JavaScript | Self::TypeScript => "implements/extends",
            Self::Python => "implements/inherits",
        }
    }
}

#[derive(Debug, Clone)]
pub enum AesCodeAnalysisViolation {
    // AES301 — File size
    FileTooLarge { reason: Option<LintMessage> },
    FileTooShort { reason: Option<LintMessage> },
    // AES303 — Mandatory class/struct definition
    MandatoryClassDefinition { reason: Option<LintMessage> },
    // AES304 — Bypass comments (Rust only)
    BypassComment { reason: Option<LintMessage> },
    UnwrapExpect { reason: Option<LintMessage> },
    Panic { reason: Option<LintMessage> },
    Todo { reason: Option<LintMessage> },
    Unimplemented { reason: Option<LintMessage> },
    // AES305 — Duplicate/dead code (empty impl blocks)
    DeadInheritance { reason: Option<LintMessage> },
    CodeDuplication { reason: Option<LintMessage> },
}

impl fmt::Display for AesCodeAnalysisViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AesCodeAnalysisViolation::FileTooLarge { reason } => {
                let default_why =
                    "Large files violate the Single Responsibility Principle.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES301 FILE_TOO_LARGE: File exceeds the maximum allowed line count.\n\
                        WHY? {}\n\
                        FIX: Split the module into smaller, more focused files.",
                    why
                )
            }
            AesCodeAnalysisViolation::FileTooShort { reason } => {
                let default_why =
                    "Excessively small files clutter the project structure.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES302 FILE_TOO_SHORT: File contains fewer than the required minimum lines.\n\
                        WHY? {}\n\
                        FIX: Expand the component or merge this logic into a related module.",
                    why
                )
            }
            AesCodeAnalysisViolation::BypassComment { reason } => {
                let default_why =
                    "Bypassing code checks hides issues and risks architectural regressions."
                        .to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(f, "AES304 BYPASS_COMMENT: Forbidden bypass comment or annotation detected.\n\
                        WHY? {}\n\
                        FIX: Remove the bypass comment (e.g. noqa, eslint-disable, ts-ignore) and resolve the issue properly.", why)
            }
            AesCodeAnalysisViolation::UnwrapExpect { reason } => {
                let default_why = "Using unwrap or expect results in runtime panics and bypasses proper error propagation.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(f, "AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.\n\
                        WHY? {}\n\
                        FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').", why)
            }
            AesCodeAnalysisViolation::Panic { reason } => {
                let default_why = "Manual panic calls crash the program unexpectedly instead of using structured error recovery.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 PANIC: Forbidden panic call detected.\n\
                        WHY? {}\n\
                        FIX: Return a Result or handle the failure case gracefully without panicking.",
                    why
                )
            }
            AesCodeAnalysisViolation::Todo { reason } => {
                let default_why = "todo!() placeholders represent incomplete code paths that can crash at runtime if reached unexpectedly.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 TODO: Forbidden todo!() call detected.\n\
                        WHY? {}\n\
                        FIX: Implement the function body with real logic, or return a meaningful default/error instead of leaving a todo!() placeholder.",
                    why
                )
            }
            AesCodeAnalysisViolation::Unimplemented { reason } => {
                let default_why = "unimplemented!() claims a code path is unreachable, but when reached it crashes — violating the principle of fail-fast with clear error messages.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 UNIMPLEMENTED: Forbidden unimplemented!() call detected.\n\
                        WHY? {}\n\
                        FIX: Either implement the missing logic or return a Result::Err with a descriptive error message.",
                    why
                )
            }
            AesCodeAnalysisViolation::MandatoryClassDefinition { reason } => {
                let lang = Language::Rust;
                let default_why = format!(
                    "Encapsulation in {} is required for proper modularization and contract adherence.",
                    lang.struct_keyword()
                );
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(f, "AES303 MANDATORY_DEFINITION: File is missing a {}, {}, or {} definition.\n\
                        WHY? {}\n\
                        FIX: Group functions into a {} or implement a {} that defines the module interface.", lang.struct_keyword(), lang.interface_kw(), lang.type_kw(), why, lang.struct_keyword(), lang.interface_kw())
            }
            AesCodeAnalysisViolation::DeadInheritance { reason } => {
                let lang = Language::Rust;
                let default_why = format!("Empty {} implementation blocks do not add behavior and indicate dead or incomplete code.", lang.inherits_kw());
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(f, "AES305 DEAD_INHERITANCE: Empty {}, class, or {} implementation block detected.\n\
                        WHY? {}\n\
                        FIX: Implement the necessary methods/fields or remove the empty definition block.", lang.struct_keyword(), lang.interface_kw(), why)
            }
            AesCodeAnalysisViolation::CodeDuplication { reason } => {
                let default_why = "Duplicate code blocks increase maintenance burden and indicate missing abstraction.".to_string();
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES305 CODE_DUPLICATION: Duplicate code block detected.\n\
                        WHY? {}\n\
                        FIX: Extract the duplicated logic into a shared function or module.",
                    why
                )
            }
        }
    }
}

impl From<AesCodeAnalysisViolation> for String {
    fn from(v: AesCodeAnalysisViolation) -> String {
        v.to_string()
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
pub mod taxonomy_value_object_utility;
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

use crate::source_parsing::taxonomy_path_vo::FilePath;

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

## File: crates/shared/src/config-system/mod.rs

```rust
// config-system — taxonomy and contract types
pub mod contract_multi_project_orchestrator_aggregate;
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

## File: crates/shared/src/import-rules/contract_rule_protocol.rs

```rust
// PURPOSE: IAnalyzer trait — core analyzer interface for import checks
use crate::cli_commands::taxonomy_result_vo::LintResultList;
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

pub trait IAnalyzer:
    crate::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol + Send + Sync
{
    fn fs(&self) -> &dyn IFileSystemPort;
    fn parser(&self) -> &dyn ISourceParserPort;
    fn detect_module_layer(&self, module_path: &FilePath) -> Option<LayerNameVO>;
}

pub trait IArchRuleProtocol {
    fn rule_name(&self) -> Identity;
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

## File: crates/shared/src/role-rules/contract_agent_role_protocol.rs

```rust
// PURPOSE: IAgentRoleChecker — port trait for AES405: agent role audits (container, orchestrator, lifecycle, file size, any type)
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_source_vo::SourceContentVO;

pub trait IAgentRoleChecker: Send + Sync {
    fn check_container(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_orchestrator(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_lifecycle(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_file_size_limit(
        &self,
        source: &SourceContentVO,
        max_lines: usize,
        violations: &mut Vec<LintResult>,
    );
    fn check_any_type_annotation(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
}
```

---

## File: crates/shared/src/role-rules/contract_capabilities_role_protocol.rs

```rust
// PURPOSE: ICapabilitiesRoleChecker — port trait for AES403: capability routing bottlenecks and role audits
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_source_vo::SourceContentVO;

pub trait ICapabilitiesRoleChecker: Send + Sync {
    fn check_capability_routing(
        &self,
        source: &SourceContentVO,
        layer: &str,
        violations: &mut Vec<LintResult>,
    );
}
```

---

## File: crates/shared/src/role-rules/contract_infrastructure_role_protocol.rs

```rust
// PURPOSE: IInfrastructureRoleChecker — port trait for AES404: infrastructure has no port implementation
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_source_vo::SourceContentVO;

pub trait IInfrastructureRoleChecker: Send + Sync {
    fn check_port_implementation(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
}
```

---

## File: crates/shared/src/role-rules/contract_role_aggregate.rs

```rust
// PURPOSE: IRoleAggregate — aggregate trait bundling taxonomy, contract, infrastructure, capabilities, surface, and agent role checkers
use crate::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
use crate::role_rules::contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
use crate::role_rules::contract_infrastructure_role_protocol::IInfrastructureRoleChecker;
use crate::role_rules::contract_role_protocol::IContractRoleChecker;
use crate::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;
use crate::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;

pub trait IRoleAggregate: Send + Sync {
    fn taxonomy(&self) -> &dyn ITaxonomyRoleChecker;
    fn contract(&self) -> &dyn IContractRoleChecker;
    fn infrastructure(&self) -> &dyn IInfrastructureRoleChecker;
    fn capabilities(&self) -> &dyn ICapabilitiesRoleChecker;
    fn surface(&self) -> &dyn ISurfaceRoleChecker;
    fn agent(&self) -> &dyn IAgentRoleChecker;
}
```

---

## File: crates/shared/src/role-rules/contract_role_protocol.rs

```rust
// PURPOSE: IContractRoleChecker — port trait for AES402: contract primitive type audits
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_source_vo::SourceContentVO;

pub trait IContractRoleChecker: Send + Sync {
    fn check_port(&self, source: &SourceContentVO) -> Vec<LintResult>;
    fn check_protocol(&self, source: &SourceContentVO) -> Vec<LintResult>;
    fn check_aggregate(
        &self,
        source: &SourceContentVO,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    );
}
```

---

## File: crates/shared/src/role-rules/contract_role_runner_aggregate.rs

```rust
// PURPOSE: IRoleRunnerAggregate — contract for role-rules feature orchestrator
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait IRoleRunnerAggregate: Send + Sync {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
    fn name(&self) -> &str;
}
```

---

## File: crates/shared/src/role-rules/contract_surface_role_protocol.rs

```rust
// PURPOSE: ISurfaceRoleChecker — port trait for AES406: smart, utility, and passive surface role checks
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_source_vo::SourceContentVO;

pub trait ISurfaceRoleChecker: Send + Sync {
    fn check_smart_surface(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_utility_surface(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_passive_surface(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_fn_count_limit(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
}
```

---

## File: crates/shared/src/role-rules/contract_taxonomy_role_protocol.rs

```rust
// PURPOSE: ITaxonomyRoleChecker — port trait for AES401: taxonomy role audits (VO, entity, error, event, constant)
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_source_vo::SourceContentVO;

pub trait ITaxonomyRoleChecker: Send + Sync {
    fn check_vo(&self) -> Vec<LintResult>;
    fn check_entity(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_error(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_event(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_constant(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
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

## File: crates/shared/src/role-rules/taxonomy_layer_names_vo.rs

```rust
// PURPOSE: LayerNames — value object for layer name collection and lookup
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_AGENT;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_CAPABILITIES;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_CONTRACT;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_GLOBAL;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_INFRASTRUCTURE;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_ROOT;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_SURFACES;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_TAXONOMY;

/// Value object holding the set of core layer names.
pub struct LayerNames {}

pub fn layer_agent() -> LayerNameVO {
    LayerNameVO::new(LAYER_AGENT)
}
pub fn layer_capabilities() -> LayerNameVO {
    LayerNameVO::new(LAYER_CAPABILITIES)
}
pub fn layer_taxonomy() -> LayerNameVO {
    LayerNameVO::new(LAYER_TAXONOMY)
}
pub fn layer_contract() -> LayerNameVO {
    LayerNameVO::new(LAYER_CONTRACT)
}
pub fn layer_infrastructure() -> LayerNameVO {
    LayerNameVO::new(LAYER_INFRASTRUCTURE)
}
pub fn layer_surfaces() -> LayerNameVO {
    LayerNameVO::new(LAYER_SURFACES)
}
pub fn layer_root() -> LayerNameVO {
    LayerNameVO::new(LAYER_ROOT)
}
pub fn layer_global() -> LayerNameVO {
    LayerNameVO::new(LAYER_GLOBAL)
}

pub fn all_core_layers() -> Vec<LayerNameVO> {
    vec![
        layer_agent(),
        layer_capabilities(),
        layer_taxonomy(),
        layer_contract(),
        layer_infrastructure(),
        layer_surfaces(),
        layer_root(),
    ]
}

pub fn core_layer_names() -> std::collections::HashSet<String> {
    all_core_layers().iter().map(|l| l.value.clone()).collect()
}
```

---

## File: crates/shared/src/role-rules/taxonomy_role_rule_vo.rs

```rust
// PURPOSE: RoleRuleVO — value object containing role compliance rule definitions
use crate::common::taxonomy_common_vo::{BooleanVO, PatternList};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RoleRuleVO {
    #[serde(default)]
    pub no_domain_logic: BooleanVO,
    #[serde(default)]
    pub must_implement_service_container_aggregate: BooleanVO,
    #[serde(default)]
    pub lazy_eager_initialization_only: BooleanVO,
    #[serde(default)]
    pub stateless_execution: BooleanVO,
    #[serde(default)]
    pub single_execution_goal: BooleanVO,
    #[serde(default)]
    pub high_level_policy_only: BooleanVO,
    #[serde(default)]
    pub coordinates_multiple_orchestrators: BooleanVO,
    #[serde(default)]
    pub crud_only: BooleanVO,
    #[serde(default)]
    pub no_decision_logic: BooleanVO,
    #[serde(default)]
    pub thread_async_safe: BooleanVO,
    #[serde(default)]
    pub no_domain_data_storage: BooleanVO,
    #[serde(default)]
    pub owns_system_health_transitions: BooleanVO,
    #[serde(default)]
    pub lifecycle_tracking_only: BooleanVO,
    #[serde(default)]
    pub no_primitives: BooleanVO,
    #[serde(default)]
    pub forbidden_inheritance: PatternList,
}
```

---

## File: crates/shared/src/role-rules/taxonomy_violation_role_vo.rs

```rust
// PURPOSE: AesRoleViolation — violation messages for role rules (AES401-406)
use crate::code_analysis::taxonomy_violation_code_analysis_vo::Language;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_name_vo::SymbolName;
use std::fmt;

pub struct LabeledRoleViolation {
    violation: AesRoleViolation,
    lang: Language,
}

/// Resolve `reason` to the user-facing "why" string. Falls back to a
/// language-aware default message when no reason was supplied by the auditor.
fn resolve_why<S: Into<String>>(reason: &Option<LintMessage>, default: S) -> String {
    reason
        .as_ref()
        .map(|r| r.to_string())
        .unwrap_or_else(|| default.into())
}

/// Write the violation body for `v` using `lang` for language-aware wording.
/// Both `Display` impls (`AesRoleViolation` and `LabeledRoleViolation`) route
/// through here so the message templates live in exactly one place per variant.
fn write_violation(
    f: &mut fmt::Formatter<'_>,
    v: &AesRoleViolation,
    lang: Language,
) -> fmt::Result {
    match v {
        AesRoleViolation::ConstantPurity { reason } => {
            let why = resolve_why(
                reason,
                "Constant taxonomy modules must only contain pure constant or static values \
                 to maintain value-level immutability.",
            );
            write!(
                f,
                "AES401 TAXONOMY_ROLE: Constant file contains non-constant declaration.\n\
                        WHY? {why}\n\
                        FIX: Move the non-constant code to the appropriate layer, or convert it \
                        to a constant/static declaration."
            )
        }
        AesRoleViolation::PrimitiveUsage { primitive, reason } => {
            let why = resolve_why(
                reason,
                format!(
                    "Direct primitive types (like '{primitive}') are forbidden in taxonomy \
                     entities, errors, and events to maintain strict value object boundaries \
                     and avoid primitive obsession."
                ),
            );
            write!(
                f,
                "AES401 TAXONOMY_ROLE: Direct primitive '{primitive}' in taxonomy entity, \
                        error, or event.\n\
                        WHY? {why}\n\
                        FIX: Replace the primitive type with a domain Value Object (VO) or \
                        constant from the taxonomy layer."
            )
        }
        AesRoleViolation::ContractPrimitive { reason } => {
            let default = format!(
                "Contracts must enforce value object boundaries to prevent primitive obsession. \
                 Use {} instead of primitives.",
                lang.type_kw()
            );
            let why = resolve_why(reason, default);
            write!(
                f,
                "AES402 CONTRACT_PRIMITIVE: Contract {} or method signature uses primitive \
                        types instead of taxonomy VO or constant.\n\
                        WHY? {why}\n\
                        FIX: Replace primitive types with appropriate Value Objects (VO) or \
                        constants from the taxonomy layer.",
                lang.interface_kw()
            )
        }
        AesRoleViolation::CapabilityRouting {
            struct_name,
            reason,
        } => {
            let default = format!(
                "Capability {}s must implement their corresponding {} traits/interfaces to \
                 ensure clean interface boundaries.",
                lang.struct_keyword(),
                lang.interface_kw()
            );
            let why = resolve_why(reason, default);
            write!(
                f,
                "AES403 CAPABILITY_ROLE: {} '{struct_name}' has no {} implementation.\n\
                        WHY? {why}\n\
                        FIX: Implement the capability protocol {} for '{struct_name}'.",
                lang.struct_keyword(),
                lang.interface_kw(),
                lang.interface_kw()
            )
        }
        AesRoleViolation::SingleBottleneck { reason } => {
            let why = resolve_why(
                reason,
                "Routing all commands to a single capability violates high-level decomposition \
                 and creates a single bottleneck.",
            );
            write!(
                f,
                "AES403 CAPABILITY_ROLE: All orchestrator dispatch routes route to a single \
                        capability.\n\
                        WHY? {why}\n\
                        FIX: Distribute logic or route commands to multiple distinct capabilities."
            )
        }
        AesRoleViolation::InfrastructureNoPort { reason } => {
            let why = resolve_why(
                reason,
                "Infrastructure adapters must implement their corresponding port interfaces.",
            );
            write!(
                f,
                "AES404 INFRASTRUCTURE_ROLE: Infrastructure file has no port trait/protocol \
                        implementation.\n\
                        WHY? {why}\n\
                        FIX: Implement the corresponding port or protocol interface in this \
                        infrastructure adapter."
            )
        }
        AesRoleViolation::StatelessExecution { reason } => {
            let why = resolve_why(
                reason,
                "Agent execution components must be stateless to guarantee reentrancy and \
                 prevent side effects.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Non-stateless behavior detected.\n\
                        WHY? {why}\n\
                        FIX: Remove mutable class state assignments or move initialization \
                        logic to the constructor."
            )
        }
        AesRoleViolation::HighLevelPolicy { reason } => {
            let why = resolve_why(
                reason,
                "Agents must focus on high-level orchestration policies and not import \
                 infrastructure adapters directly.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Low-level implementation details imported.\n\
                        WHY? {why}\n\
                        FIX: Reference components using their contract interfaces instead of \
                        concrete infrastructure types."
            )
        }
        AesRoleViolation::CoordinatesMultiple { reason } => {
            let why = resolve_why(
                reason,
                "Orchestrator agents exist to coordinate multiple subsystems; simple \
                 single-component logic belongs elsewhere.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Orchestrator coordinates too few subsystems.\n\
                        WHY? {why}\n\
                        FIX: Merge this simple flow into its caller or delegate at least two \
                        subsystems to this orchestrator."
            )
        }
        AesRoleViolation::NoDomainLogic { reason } => {
            let why = resolve_why(
                reason,
                "Complex domain logic detected in a passive agent role or surface wrapper.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Complex domain logic detected in a passive role.\n\
                        WHY? {why}\n\
                        FIX: Move the complex domain/control logic into capabilities or \
                        orchestrator components."
            )
        }
        AesRoleViolation::LazyEagerInit { reason } => {
            let why = resolve_why(
                reason,
                "Agent containers must only declare and wire dependencies, avoiding complex \
                 logic in constructors.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Complex initialization logic found in container module.\n\
                        WHY? {why}\n\
                        FIX: Move the initialization/conditional logic out of the constructor \
                        or container setup."
            )
        }
        AesRoleViolation::MustImplementContract { reason } => {
            let default = format!(
                "Agent containers must implement the 'ServiceContainerAggregate' {} to satisfy \
                 dependency injection protocols.",
                lang.interface_kw()
            );
            let why = resolve_why(reason, default);
            write!(
                f,
                "AES405 AGENT_ROLE: Class is missing required contract implementation.\n\
                        WHY? {why}\n\
                        FIX: Add the 'ServiceContainerAggregate' implementation for the \
                        container class."
            )
        }
        AesRoleViolation::AnyType { reason } => {
            let why = resolve_why(
                reason,
                "Using 'any' or 'Any' type annotations bypasses type safety and violates \
                 agent-level domain-driven design.",
            );
            write!(
                f,
                "AES405 AGENT_ROLE: Forbidden 'any' type annotation found.\n\
                        WHY? {why}\n\
                        FIX: Replace 'any' annotations with strongly-typed objects, \
                        structures, or domain Value Objects (VO)."
            )
        }
        AesRoleViolation::AgentFileSizeLimit { max_lines } => write!(
            f,
            "AES405 AGENT_ROLE: Agent file exceeds {max_lines} lines.\n\
                    WHY? Agent files must remain compact to preserve role clarity.\n\
                    FIX: Split the orchestrator/container into smaller focused modules."
        ),
        AesRoleViolation::PassiveViolation { reason } => {
            let why = resolve_why(
                reason,
                "Passive surfaces must not contain logic that should be in capabilities or \
                 agents.",
            );
            write!(
                f,
                "AES406 SURFACE_ROLE: Passive surface contains business logic.\n\
                        WHY? {why}\n\
                        FIX: Move logic to appropriate capability or agent."
            )
        }
        AesRoleViolation::SurfaceRoleViolation { reason } => {
            let why = resolve_why(
                reason,
                "Surface role violation - surfaces must adhere to their designated role \
                 (command, controller, component, hook, etc.).",
            );
            write!(
                f,
                "AES406 SURFACE_ROLE: Surface role boundary violation.\n\
                        WHY? {why}\n\
                        FIX: Ensure surface only performs its designated responsibilities."
            )
        }
    }
}

impl AesRoleViolation {
    pub fn with_language(self, lang: Language) -> LabeledRoleViolation {
        LabeledRoleViolation {
            violation: self,
            lang,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AesRoleViolation {
    // AES401 — Taxonomy role
    ConstantPurity {
        reason: Option<LintMessage>,
    },
    PrimitiveUsage {
        primitive: SymbolName,
        reason: Option<LintMessage>,
    },
    // AES402 — Contract primitive
    ContractPrimitive {
        reason: Option<LintMessage>,
    },
    // AES403 — Capability role
    CapabilityRouting {
        struct_name: SymbolName,
        reason: Option<LintMessage>,
    },
    SingleBottleneck {
        reason: Option<LintMessage>,
    },
    // AES404 — Infrastructure role
    InfrastructureNoPort {
        reason: Option<LintMessage>,
    },
    // AES405 — Agent role
    StatelessExecution {
        reason: Option<LintMessage>,
    },
    HighLevelPolicy {
        reason: Option<LintMessage>,
    },
    CoordinatesMultiple {
        reason: Option<LintMessage>,
    },
    NoDomainLogic {
        reason: Option<LintMessage>,
    },
    LazyEagerInit {
        reason: Option<LintMessage>,
    },
    MustImplementContract {
        reason: Option<LintMessage>,
    },
    AnyType {
        reason: Option<LintMessage>,
    },
    AgentFileSizeLimit {
        max_lines: usize,
    },
    // AES406 — Surface role
    PassiveViolation {
        reason: Option<LintMessage>,
    },
    SurfaceRoleViolation {
        reason: Option<LintMessage>,
    },
}

impl fmt::Display for AesRoleViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_violation(f, self, Language::Rust)
    }
}

impl fmt::Display for LabeledRoleViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_violation(f, &self.violation, self.lang)
    }
}

impl From<AesRoleViolation> for String {
    fn from(v: AesRoleViolation) -> String {
        v.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labeled(v: AesRoleViolation, lang: Language) -> LabeledRoleViolation {
        v.with_language(lang)
    }

    /// `LabeledRoleViolation::Display` must produce the same output as the
    /// underlying `AesRoleViolation::Display` when the language is Rust,
    /// because Rust is the implicit default for the bare `AesRoleViolation`
    /// Display impl. Guards against accidental drift between the two impls.
    #[test]
    fn labeled_rust_matches_bare_display() {
        let v = AesRoleViolation::CoordinatesMultiple {
            reason: Some(LintMessage::new("custom".to_string())),
        };
        assert_eq!(
            v.to_string(),
            labeled(v.clone(), Language::Rust).to_string()
        );
    }

    /// `LabeledRoleViolation` with a non-Rust language must swap language-
    /// sensitive tokens (e.g. Python uses "Protocol", JS uses "interface").
    /// Guards against hard-coded Rust tokens leaking into non-Rust labels.
    #[test]
    fn labeled_python_uses_protocol_token() {
        let v = AesRoleViolation::ContractPrimitive {
            reason: Some(LintMessage::new("custom".to_string())),
        };
        let out = labeled(v, Language::Python).to_string();
        assert!(
            out.contains("Protocol"),
            "expected Protocol token in: {out}"
        );
        assert!(
            !out.contains("trait"),
            "must not contain Rust trait in: {out}"
        );
    }

    /// When `reason` is `None`, the per-variant default WHY message must be
    /// used. Confirms `resolve_why` does not emit `None` / `Some(...)`.
    #[test]
    fn missing_reason_uses_default_why() {
        let v = AesRoleViolation::SingleBottleneck { reason: None };
        let out = v.to_string();
        assert!(
            out.contains("single bottleneck"),
            "default WHY missing in: {out}"
        );
        assert!(!out.contains("None"), "leaked None in: {out}");
    }

    /// When `reason` is `Some`, the auditor-supplied WHY must override the
    /// per-variant default.
    #[test]
    fn present_reason_overrides_default_why() {
        let v = AesRoleViolation::SingleBottleneck {
            reason: Some(LintMessage::new("auditor-custom".to_string())),
        };
        let out = v.to_string();
        assert!(
            out.contains("auditor-custom"),
            "custom WHY missing in: {out}"
        );
    }

    /// `AgentFileSizeLimit` carries no `reason`; its display must embed the
    /// numeric `max_lines`.
    #[test]
    fn agent_file_size_limit_includes_max_lines() {
        let v = AesRoleViolation::AgentFileSizeLimit { max_lines: 250 };
        let out = v.to_string();
        assert!(out.contains("250"), "max_lines not in output: {out}");
    }
}
```

---

## File: crates/shared/src/source-parsing/contract_language_detector_port.rs

```rust
// PURPOSE: ILanguageDetectorPort — contract for detecting programming language from file path
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Python,
    JavaScript,
    TypeScript,
    Rust,
    Unknown,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Python => "python",
            Language::JavaScript => "javascript",
            Language::TypeScript => "typescript",
            Language::Rust => "rust",
            Language::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub trait ILanguageDetectorPort: Send + Sync {
    /// Detect language from a file path based on extension.
    fn detect(&self, path: &FilePath) -> Language;

    /// Check if a file path matches a specific language.
    fn is_language(&self, path: &FilePath, lang: Language) -> bool {
        self.detect(path) == lang
    }

    /// Check if a file path is a lintable language (Python, JS, TS, Rust).
    fn is_lintable(&self, path: &FilePath) -> bool {
        matches!(
            self.detect(path),
            Language::Python | Language::JavaScript | Language::TypeScript | Language::Rust
        )
    }
}
```

---

## File: crates/shared/src/source-parsing/infrastructure_file_collector_provider.rs

```rust
use std::fs;
use std::path::{Path, PathBuf};

use crate::config_system::taxonomy_config_vo::default_aes_config;
use crate::file_system::taxonomy_filesystem_error::FileSystemError;
use crate::source_parsing::contract_scanner_provider_port::IScannerProviderPort;
use crate::source_parsing::taxonomy_file_collector_helper::is_path_ignored;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

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

pub fn count_loc(path: &str) -> usize {
    let src = Path::new(path);
    let ignored = default_ignored_paths();
    let mut count = 0usize;
    walk_rs_files(
        src,
        &mut |p| {
            if let Ok(c) = fs::read_to_string(&p) {
                count += c.lines().count();
            }
        },
        &ignored,
    );
    count.max(1)
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
pub mod infrastructure_file_collector_provider;
pub mod taxonomy_adapter_error;
pub mod taxonomy_barrel_provider_vo;
pub mod taxonomy_file_collector_helper;
pub mod taxonomy_language_detector_helper;
pub mod taxonomy_naming_error;
pub mod taxonomy_naming_list_vo;
pub mod taxonomy_parser_error;
pub mod taxonomy_path_vo;
pub mod taxonomy_paths_vo;
pub mod taxonomy_semantic_error;
pub use infrastructure_file_collector_provider::{
    collect_all_source_files, count_loc, walk_rs_files, FileCollectorProvider,
};
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

    /// Regression: `./foo.rs` must report `rs` as its extension, not empty string.
    /// The old implementation treated any path starting with `.` as having no
    /// extension, which caused `LanguageDetector::is_lintable` to skip relative
    /// paths emitted by `std::fs::read_dir` in `collect_source_files`. Result: zero
    /// files collected when the user runs `lint-arwaky check .` on a directory
    /// tree with non-`.git`-anchored paths.
    #[test]
    fn test_extension_with_dot_slash_prefix() {
        let fp = FilePath::new("./foo.rs").unwrap_or_default();
        assert_eq!(fp.extension(), "rs");
        let fp = FilePath::new("./nested/foo.py").unwrap_or_default();
        assert_eq!(fp.extension(), "py");
        let fp = FilePath::new(".//foo.ts").unwrap_or_default();
        assert_eq!(fp.extension(), "ts");
    }

    /// Regression: a hidden-file basename (e.g. `.bashrc`) must still report no
    /// extension, since the basename itself starts with a dot.
    #[test]
    fn test_extension_hidden_basename() {
        let fp = FilePath::new(".bashrc").unwrap_or_default();
        assert_eq!(fp.extension(), "");
        let fp = FilePath::new("/home/user/.gitignore").unwrap_or_default();
        assert_eq!(fp.extension(), "");
    }

    /// Regression: full paths must still resolve the extension on the basename.
    #[test]
    fn test_extension_full_path() {
        let fp =
            FilePath::new("/tmp/bypass_test/capabilities_unwrap_checker.rs").unwrap_or_default();
        assert_eq!(fp.extension(), "rs");
        let fp = FilePath::new("crates/code-analysis/src/foo.rs").unwrap_or_default();
        assert_eq!(fp.extension(), "rs");
    }

    /// Makefile / Dockerfile — special filenames, no extension.
    #[test]
    fn test_extension_special_filenames() {
        let fp = FilePath::new("Makefile").unwrap_or_default();
        assert_eq!(fp.extension(), "");
        let fp = FilePath::new("Dockerfile").unwrap_or_default();
        assert_eq!(fp.extension(), "");
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
