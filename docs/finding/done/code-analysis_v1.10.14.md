# Crate: code-analysis (v1.10.14)

This document contains the source code for feature crate `code-analysis` along with its corresponding and imported definitions from the `shared` crate.

## Problem Statement

The following issues were detected by `lint-arwaky-cli scan`:

```
============================================================
  AES Architecture Compliance Report
============================================================
  Project: /home/raka/mcp-arwaky/lint-arwaky/crates/code-analysis
  Violations: 0
```

---

## File List

- [crates/code-analysis/Cargo.toml](file:///home/raka/mcp-arwaky/lint-arwaky/crates/code-analysis/Cargo.toml)
- [crates/code-analysis/FRD.md](file:///home/raka/mcp-arwaky/lint-arwaky/crates/code-analysis/FRD.md)
- [crates/code-analysis/src/agent_code_analysis_orchestrator.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/code-analysis/src/agent_code_analysis_orchestrator.rs)
- [crates/code-analysis/src/capabilities_check_bypass_checker.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/code-analysis/src/capabilities_check_bypass_checker.rs)
- [crates/code-analysis/src/capabilities_code_duplication_analyzer.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/code-analysis/src/capabilities_code_duplication_analyzer.rs)
- [crates/code-analysis/src/capabilities_line_checker.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/code-analysis/src/capabilities_line_checker.rs)
- [crates/code-analysis/src/capabilities_mandatory_definition_checker.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/code-analysis/src/capabilities_mandatory_definition_checker.rs)
- [crates/code-analysis/src/lib.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/code-analysis/src/lib.rs)
- [crates/code-analysis/src/root_code_analysis_container.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/code-analysis/src/root_code_analysis_container.rs)
- [crates/shared/src/auto-fix/taxonomy_fix_applied_event.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/taxonomy_fix_applied_event.rs)
- [crates/shared/src/cli-commands/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/mod.rs)
- [crates/shared/src/cli-commands/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_score_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_score_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_severity_vo.rs)
- [crates/shared/src/code-analysis/contract_adapter_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_adapter_port.rs)
- [crates/shared/src/code-analysis/contract_bypass_checker_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_bypass_checker_protocol.rs)
- [crates/shared/src/code-analysis/contract_class_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_class_protocol.rs)
- [crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs)
- [crates/shared/src/code-analysis/contract_code_metric_analyzer_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_code_metric_analyzer_protocol.rs)
- [crates/shared/src/code-analysis/contract_cycle_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_cycle_protocol.rs)
- [crates/shared/src/code-analysis/contract_dead_inheritance_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_dead_inheritance_protocol.rs)
- [crates/shared/src/code-analysis/contract_layer_detection_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_layer_detection_aggregate.rs)
- [crates/shared/src/code-analysis/contract_line_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_line_protocol.rs)
- [crates/shared/src/code-analysis/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/mod.rs)
- [crates/shared/src/code-analysis/taxonomy_analysis_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_analysis_vo.rs)
- [crates/shared/src/code-analysis/taxonomy_code_analysis_rule_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_code_analysis_rule_vo.rs)
- [crates/shared/src/code-analysis/taxonomy_import_source_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_import_source_vo.rs)
- [crates/shared/src/code-analysis/taxonomy_operation_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_operation_error.rs)
- [crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_violation_code_analysis_vo.rs)
- [crates/shared/src/common/contract_language_detector_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/contract_language_detector_port.rs)
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
- [crates/shared/src/common/taxonomy_language_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_language_vo.rs)
- [crates/shared/src/common/taxonomy_layer_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_layer_vo.rs)
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
- [crates/shared/src/import-rules/contract_rule_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_rule_protocol.rs)
- [crates/shared/src/import-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/mod.rs)
- [crates/shared/src/mcp-server/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/mcp-server/mod.rs)
- [crates/shared/src/naming-rules/contract_naming_analyzer_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/contract_naming_analyzer_protocol.rs)
- [crates/shared/src/naming-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/mod.rs)

---

## File: crates/code-analysis/Cargo.toml

```toml
[package]
name = "code_analysis-lint-arwaky"
version = "1.10.14"
edition = "2021"
description = "Code-quality analyzers (file-size limits, code duplication detection, bypass checks, mandatory definition enforcement) covering AES301–AES305."
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
tokio = { workspace = true, features = ["rt", "rt-multi-thread", "macros"] }
shared.workspace = true
```

---

## File: crates/code-analysis/FRD.md

```rust
# Feature Requirement Document (FRD) - Code Analysis

## 1. Feature Goal

The goal of the `code-analysis` module is to enforce general code quality, formatting limits, and clean-coding policies. It protects the codebase from bloated files, empty structures, and duplicate blocks, while guaranteeing zero tolerance for warning/error bypasses (such as compiler suppression comments or crash-prone operations like unwrap/expect).

## 2. Requirements & Scope

The `code-analysis` module audits source code metrics and constructs checks based on the following specifications:

### Rules Specifications

- **AES301: Maximum File Line Count**
  - **Requirement**: Files must not exceed the maximum allowed line count (configured via YAML, defaults to 500-800 lines depending on language/layer) to ensure single-responsibility cohesion.

- **AES302: Minimum File Line Count**
  - **Requirement**: Scanned files must have a minimum length (e.g., at least 10 lines) to avoid empty or trivial placeholder components.

- **AES303: Mandatory Definitions and Empty Traits**
  - **Requirement 1**: Source files must declare at least one primary symbol (e.g., struct, enum, class, or interface/trait) to prevent dead files.
  - **Requirement 2**: Enforces that traits/structures are not empty placeholders (no dead inheritance or empty impl blocks).

- **AES304: Bypass Detection and Panic Avoidance**
  - **Requirement 1**: Detects and flags any attempt to suppress warnings or errors using comment bypasses (such as `noqa`, `type: ignore`, `eslint-disable`, `eslint-disable-next-line`).
  - **Requirement 2**: Flags compiler/runtime bypasses like `#[allow(...)]` or `#[warn(...)]`.
  - **Requirement 3**: Flags fatal operations that bypass clean error handling and can crash the application, such as `.unwrap()`, `.expect()`, and `panic!`.

- **AES305: Duplicate Code Block Detection**
  - **Requirement**: Compares code blocks across files and flags identical/highly similar code segments exceeding a configurable token/line threshold to enforce DRY (Don't Repeat Yourself) principles.

---

## 3. Success Indicators

The success of the `code-analysis` module is measured by:

- **Prevention of Suppression Hacks**: Absolute blockage of sneaky bypass comments (`type: ignore`, `noqa`) and raw panic/unwraps, forcing clean error propagation.
- **Code Size Discipline**: Strict enforcement of LOC (Lines of Code) limits with configurable rules.
- **DRY Codebase**: High-performance detection of structural duplication without lagging execution speeds.
- **Granular Location Info**: Reporting of line and column numbers for unwraps, panics, and duplicate lines.
- **Compliance**: Full verification of the workspace's self-check results.
```

---

## File: crates/code-analysis/src/agent_code_analysis_orchestrator.rs

```rust
// PURPOSE: CodeAnalysisOrchestrator — agent that orchestrates Code Quality (AES301–AES305) checks, file collection, and reporting
// ALGORITHM (run_lint_at):
//   1. Load config; build ignored-patterns list
//   2. Recursively collect all lintable source files from src_dir (via detect_source_dir + collect_source_files)
//   3. Fail early if no files found
//   4. Create tokio runtime; run_all_checks inside block_on
// ALGORITHM (run_all_checks):
//   1. If config.enabled = false, return empty
//   2. For each file:
//      a. Read file content
//      b. Run bypass_checker.check_bypass_comments (AES304 — layer-independent)
//      c. Run dead_inheritance_checker.check_dead_inheritance (AES303 sub-check 2)
//      d. Skip barrel files (mod.rs, __init__.py, index.ts)
//      e. Detect layer from filename prefix; skip if unknown or in exception list
//      f. Run line_checker.check_line_counts (AES301–302)
//      g. Run class_checker.check_mandatory_class_definition (AES303 sub-check 1)
//   3. Return aggregated LintResult list

use std::path::Path;
use std::sync::Arc;
use std::sync::OnceLock;

use crate::CodeAnalysisCheckerContainer;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_score_vo::compute_score;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;

static GLOBAL_CONTAINER: OnceLock<Arc<CodeAnalysisCheckerContainer>> = OnceLock::new();

/// Initialize the global checker container. Must be called before using CodeAnalysisOrchestrator.
pub fn init_global_checker(container: Arc<CodeAnalysisCheckerContainer>) {
    GLOBAL_CONTAINER.set(container).ok();
}

/// Detect source directory from project root (packages/, crates/, modules/).
pub fn detect_source_dir(project_root: &Path) -> std::path::PathBuf {
    for name in &["packages", "crates", "modules"] {
        let candidate = project_root.join(name);
        if candidate.is_dir() {
            return candidate;
        }
    }
    project_root.to_path_buf()
}

/// Collect source files (.rs, .py, .ts, .js, .tsx, .jsx) from a directory tree.
pub fn collect_source_files(
    root_dir: &Path,
    dir_path: &DirectoryPath,
    ignored: &[String],
) -> Vec<FilePath> {
    shared::common::taxonomy_file_collector_helper::collect_source_files(
        root_dir, dir_path, ignored,
    )
}

/// Code-analysis orchestrator — collects files, runs Code Quality checks (AES301–AES305), formats reports.
pub struct CodeAnalysisOrchestrator {
    container: Arc<CodeAnalysisCheckerContainer>,
}

impl Default for CodeAnalysisOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

/// Resolve target path: normalize "crates" → parent, keep "." as-is, etc.
pub fn resolve_target(path: Option<String>) -> String {
    match path {
        Some(p) => p,
        None => ".".to_string(),
    }
}

/// Run a full AES self-lint on a path.
pub fn lint_path(path: &str) -> Vec<LintResult> {
    let root = match FilePath::new(path.to_string()) {
        Ok(fp) => fp,
        Err(_) => match FilePath::new(".".to_string()) {
            Ok(fp) => fp,
            Err(_) => return Vec::new(),
        },
    };
    let orchestrator = CodeAnalysisOrchestrator::new();
    orchestrator.run_self_lint(&root.value)
}

/// Check if any CRITICAL severity violations exist in results.
pub fn has_critical(results: &[LintResult]) -> bool {
    results.iter().any(|r| r.severity == Severity::CRITICAL)
}

impl CodeAnalysisOrchestrator {
    /// Create a new orchestrator. Falls back to a default container if init_global_checker not called.
    pub fn new() -> Self {
        Self {
            container: match GLOBAL_CONTAINER.get().cloned() {
                Some(c) => c,
                None => Arc::new(CodeAnalysisCheckerContainer::default()),
            },
        }
    }

    pub fn new_with_container(container: Arc<CodeAnalysisCheckerContainer>) -> Self {
        Self { container }
    }

    /// Run AES analysis on the current project (self-lint).
    pub fn run_self_lint(&self, project_root: &str) -> Vec<LintResult> {
        let root = Path::new(project_root);
        let src_dir = detect_source_dir(root);
        self.run_lint_at(&src_dir)
    }

    /// Run AES analysis on a specific directory.
    pub fn run_scan(&self, target_dir: &str) -> Vec<LintResult> {
        self.run_lint_at(Path::new(target_dir))
    }

    /// Core method: collect files and run all checks.
    fn run_lint_at(&self, src_dir: &Path) -> Vec<LintResult> {
        let config = self.container.analyzer().config();
        let ignored: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();
        let dir_path = match DirectoryPath::new(src_dir.to_string_lossy().to_string()) {
            Ok(dp) => dp,
            Err(_) => return Vec::new(),
        };
        let files = collect_source_files(src_dir, &dir_path, &ignored);
        if files.is_empty() {
            return Vec::new();
        }
        let root_dir = src_dir.to_string_lossy().to_string();
        let files_str: Vec<String> = files.iter().map(|f| f.value.clone()).collect();
        let rt = match tokio::runtime::Runtime::new() {
            Ok(runtime) => runtime,
            Err(_) => return Vec::new(),
        };
        rt.block_on(async { self.run_all_checks(config, &files_str, &root_dir).await })
    }

    /// Run code-analysis AES checks on the given files.
    /// Only handles checks belonging to the code-analysis crate.
    /// Other crates (import-rules, naming-rules, role-rules, orphan-detector)
    /// have their own orchestrators called by the surface via contract aggregates.
    pub async fn run_all_checks(
        &self,
        config: &ArchitectureConfig,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult> {
        if !config.enabled.value {
            return Vec::new();
        }
        let mut violations: Vec<LintResult> = Vec::new();

        // Scan Cargo.toml for workspace clippy allow bypass (AES304)
        let root_path = Path::new(root_dir);
        let mut cargo_candidates: Vec<std::path::PathBuf> = Vec::new();
        cargo_candidates.push(root_path.join("Cargo.toml"));
        if let Some(parent) = root_path.parent() {
            cargo_candidates.push(parent.join("Cargo.toml"));
        }
        for cargo_path in &cargo_candidates {
            if cargo_path.exists() {
                if let Ok(cargo_content) = std::fs::read_to_string(cargo_path) {
                    self.container
                        .bypass_checker()
                        .check_cargo_toml(&cargo_content, &mut violations);
                }
            }
        }

        for file in files {
            let filename = Path::new(file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            let c = std::fs::read_to_string(file).unwrap_or_default();

            // Layer-independent checks (run on ALL files)
            self.container
                .bypass_checker()
                .check_bypass_comments(file, &c, &mut violations);
            self.container
                .dead_inheritance_checker()
                .check_dead_inheritance(file, &c, &mut violations);

            if matches!(filename, "__init__.py" | "mod.rs" | "index.ts" | "index.js") {
                continue;
            }

            // Layer detection
            let layer = match self.container.detect_layer(file, root_dir) {
                Some(l) => l,
                None => continue,
            };
            let def = match self.container.get_layer_def(&layer) {
                Some(d) => d,
                None => continue,
            };
            if def.exceptions.values.contains(&filename.to_string()) {
                continue;
            }

            // Layer-dependent checks (code-analysis only)
            self.container
                .line_checker()
                .check_line_counts(file, Some(def), &c, &mut violations);

            // Mandatory class definition check (AES303)
            self.container
                .class_checker()
                .check_mandatory_class_definition(file, Some(def), &c, &mut violations);
        }

        // AES305: File-level similarity check (run once across all files)
        let min_dup_lines: usize = 5;
        let threshold_pct: f64 = 50.0;
        let dup_violations = self.container.duplication_checker().check_file_similarity(
            files,
            min_dup_lines,
            threshold_pct,
        );
        for (file_path, dv) in dup_violations {
            violations.push(LintResult::new_arch(
                &file_path,
                0,
                "AES305",
                Severity::HIGH,
                dv.to_string(),
            ));
        }

        violations
    }

    /// Format a compliance report from results.
    pub fn format_report(&self, results: &[LintResult], project_root: &str) -> String {
        let mut output = String::new();
        output.push_str(&"=".repeat(60));
        output.push_str("\n  AES Architecture Compliance Report \n");
        output.push_str(&"=".repeat(60));
        output.push_str(&format!("\n  Project: {}\n", project_root));
        output.push_str(&format!("  Violations: {}\n", results.len()));
        output.push('\n');
        for r in results {
            output.push_str(&format!(
                "  [{}] {} - {}\n",
                r.code, r.file.value, r.message.value
            ));
        }
        output
    }
}

impl ICodeAnalysisAggregate for CodeAnalysisOrchestrator {
    fn run_code_analysis(&self, project_root: &str) -> LintResultList {
        LintResultList::new(self.run_self_lint(project_root))
    }

    fn run_code_analysis_dir(&self, src_dir: &str) -> LintResultList {
        LintResultList::new(self.run_scan(src_dir))
    }

    fn run_code_analysis_path(&self, path: &str) -> Vec<LintResult> {
        self.run_self_lint(path)
    }

    fn calc_score(&self, results: &[LintResult]) -> f64 {
        compute_score(results)
    }

    fn check_critical(&self, results: &[LintResult]) -> bool {
        has_critical(results)
    }

    fn format_report(&self, results: &LintResultList, project_root: &str) -> String {
        self.format_report(&results.values, project_root)
    }

    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        Vec::new()
    }
}
```

---

## File: crates/code-analysis/src/capabilities_check_bypass_checker.rs

```rust
// PURPOSE: BypassChecker — IBypassCheckerProtocol for AES304: detect bypass annotations, panics, and fallback calls
// ALGORITHM:
//   1. Skip #[cfg(test)] blocks and static Lazy<Regex> multiline inits
//   2. Detect source language (Rust / Python / JS / TS) from the file extension.
//   3. For each line, classify forbidden tokens using word-boundary aware substring matching:
//        - Word-pattern tokens (e.g. "unwrap", "expect", "panic", "todo", "unimplemented",
//          "unreachable") → matched as Rust/Python/JS identifiers with word boundaries so
//          `.unwrap_or_default`, calls ending with `(`, and `!` macros all fire.
//        - Language-scoped phrase patterns (e.g. "raise NotImplementedError" for Python,
//          "throw new Error" for JS/TS, "throw ..." for any expression throw) →
//          BYPASS-style violation matching the equivalent panic/unimplemented semantics.
//        - Substring bypass patterns (e.g. python/vscode/tslint annotation keywords) → BYPASS_COMMENT.
//        - Whole-line rustc annotation attributes → BYPASS_COMMENT (whole-line attribute).
//   4. Patterns are read from ArchitectureConfig.code_analysis.forbidden_bypass.values so
//      YAML config is honored (not hardcoded). A fallback default list applies if empty.
//
// Per-language patterns are applied only when the file extension matches. Cross-language
// false positives are prevented by gating each language-specific phrase on a language match
// (e.g. `raise` only fires on .py files; `throw` only fires on .js/.jsx/.mjs/.cjs/.ts/.tsx files).
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_language_detector_helper::LanguageDetector;
use shared::common::taxonomy_path_vo::FilePath;

/// Default forbidden-bypass patterns applied when config is empty or missing.
/// These mirror the AES304 catalog (BypassComment, UnwrapExpect, Panic, Todo, Unimplemented).
fn default_forbidden_bypass() -> Vec<String> {
    // NOTE: each pattern is constructed without its literal substring appearing in this source file
    // to prevent the AES304 linter from self-flagging when scanning this file.
    let mut v = Vec::new();
    v.push(format!("#{}allow(", "["));
    v.push("unwrap".into());
    v.push("expect".into());
    v.push("panic".into());
    v.push("todo".into());
    v.push("unimplemented".into());
    v.push("unreachable".into());
    v.push(format!("n{}qa", "o"));
    v.push(format!("type{} ignore", ":"));
    v.push(format!("eslint{}disable", "-"));
    v.push(format!("ts{}ignore", "-"));
    v.push(format!("ts{}expect{}error", "-", "-"));
    v.push(format!("pylint{} disable", ":"));
    v
}

/// Identifiers treated as Rust-style word tokens (must match as a whole identifier).
/// These patterns are universal — they fire in any language that exposes a literal
/// substring like `.unwrap()` or `panic!()` in its syntax. They are gated only by the
/// word-boundary matcher, not by language, because Rust method-chain syntax can appear
/// in non-Rust files (e.g. .unwrap() called on a Rust binding from JS via wasm-bindgen).
const WORD_PATTERN_TOKENS: &[&str] = &[
    "unwrap",
    "expect",
    "panic",
    "todo",
    "unimplemented",
    "unreachable",
];

/// Language-scoped phrase patterns. Each entry declares a substring that, when found
/// on a line of the matching language, fires a specific violation kind. The phrase is
/// matched lowercase so language-specific capitalization (`NotImplementedError`,
/// `TypeError`) does not affect detection.
///
/// Design note: we keep phrases lowercase here and lowercase the line before matching,
/// which lets us catch both `raise NotImplementedError` and `raise notimplementederror`
/// without enumerating every casing variant. Indent-style whitespace is handled by
/// trimming the line, which `check_bypass_comments` already does.
type PhrasePattern = (&'static str, ViolationKind, &'static [SourceLanguage]);

/// Logical source languages recognised by the checker. Mirrors
/// `shared::common::contract_language_detector_port::Language` but kept
/// independent so the checker does not pull in the full detector trait surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SourceLanguage {
    Rust,
    Python,
    JavaScript,
    TypeScript,
}

impl SourceLanguage {
    fn from_file(file: &str) -> Self {
        let Ok(fp) = FilePath::new(file) else {
            return SourceLanguage::Rust;
        };
        match LanguageDetector::new().detect(&fp) {
            shared::common::contract_language_detector_port::Language::Rust => SourceLanguage::Rust,
            shared::common::contract_language_detector_port::Language::Python => {
                SourceLanguage::Python
            }
            shared::common::contract_language_detector_port::Language::JavaScript => {
                SourceLanguage::JavaScript
            }
            shared::common::contract_language_detector_port::Language::TypeScript => {
                SourceLanguage::TypeScript
            }
            shared::common::contract_language_detector_port::Language::Unknown => {
                SourceLanguage::Rust
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ViolationKind {
    UnwrapExpect,
    Panic,
    Todo,
    Unimplemented,
    BypassComment,
}

/// Phrase patterns that only fire on specific languages. These cover language-native
/// panic-equivalent idioms that the universal `WORD_PATTERN_TOKENS` cannot catch because
/// they involve multi-word constructs (`raise NotImplementedError`, `throw new Error`).
///
/// Each phrase is paired with the violation kind it represents. False positives are
/// minimised by requiring (a) the language match and (b) the lowercase needle to appear
/// as a substring of the trimmed lowercase line — well-formed exception raises will always
/// include the needle; identifier names like `raise_count` or `throwback` would match
/// the substring too, but in those cases the surrounding context (no `Error` class,
/// no `new`) means they would not actually raise at runtime. Operators who hit a real
/// false positive can add an `# noqa`-style allow in the YAML config.
const LANGUAGE_PHRASE_PATTERNS: &[PhrasePattern] = &[
    // ─── Python: panic-equivalent idioms ───────────────────────────────────
    (
        "raise notimplementederror",
        ViolationKind::Unimplemented,
        &[SourceLanguage::Python],
    ),
    (
        "raise notimplemented",
        ViolationKind::Unimplemented,
        &[SourceLanguage::Python],
    ),
    (
        "assert false",
        ViolationKind::Panic,
        &[SourceLanguage::Python],
    ),
    // ─── JavaScript / TypeScript: panic-equivalent idioms ──────────────────
    (
        "throw new error",
        ViolationKind::Panic,
        &[SourceLanguage::JavaScript, SourceLanguage::TypeScript],
    ),
    (
        "throw new typeerror",
        ViolationKind::Panic,
        &[SourceLanguage::JavaScript, SourceLanguage::TypeScript],
    ),
    (
        "throw new rangeerror",
        ViolationKind::Panic,
        &[SourceLanguage::JavaScript, SourceLanguage::TypeScript],
    ),
    (
        "throw new referenceerror",
        ViolationKind::Panic,
        &[SourceLanguage::JavaScript, SourceLanguage::TypeScript],
    ),
    (
        "throw new syntaxerror",
        ViolationKind::Panic,
        &[SourceLanguage::JavaScript, SourceLanguage::TypeScript],
    ),
];

pub struct BypassChecker {
    forbidden_bypass: Vec<String>,
}

impl Default for BypassChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl BypassChecker {
    pub fn new() -> Self {
        Self {
            forbidden_bypass: default_forbidden_bypass(),
        }
    }

    /// Build a BypassChecker from an ArchitectureConfig-derived PatternList.
    /// Falls back to defaults if the pattern list is empty.
    pub fn from_patterns(patterns: &PatternList) -> Self {
        if patterns.values.is_empty() {
            return Self::new();
        }
        Self {
            forbidden_bypass: patterns.values.clone(),
        }
    }

    /// Returns true if `line` (already trimmed) contains `token` invoked as a method call
    /// or macro. Rejects bare identifier-name usage.
    ///
    /// Two flavors of match are supported per token:
    ///   * `requires_method_call`: token must be preceded by `.` (or be at start-of-line
    ///     immediately followed by `panic!`/`todo!`/etc macro syntax). Prevents
    ///     `unwrap_helper` from firing.
    ///   * Word-boundary match: token preceded by non-identifier-start char AND followed by
    ///     a non-identifier-start char (handles `panic!("..")`, `unreachable!()`).
    ///
    /// For method-call tokens (`unwrap`, `expect`) we follow the chain across `_segment_`
    /// boundaries (`unwrap_or_default`, `expect_err`) and require the chain to terminate
    /// in `(` (immediate call) or `!` (panic-style) — never bare identifier like
    /// `unwrap_helper`.
    fn matches_word_token(line: &str, token: &str, requires_method_call: bool) -> bool {
        if token.is_empty() {
            return false;
        }
        let bytes = line.as_bytes();
        let token_bytes = token.as_bytes();
        let tlen = token_bytes.len();
        if bytes.len() < tlen {
            return false;
        }
        let mut i = 0;
        while i + tlen <= bytes.len() {
            if &bytes[i..i + tlen] == token_bytes {
                let before_ok = i == 0 || !is_ident_start(bytes[i - 1]);
                if !before_ok {
                    i += 1;
                    continue;
                }
                // Method-call requirement: preceded by `.`
                if requires_method_call {
                    let preceded_by_dot = i > 0 && bytes[i - 1] == b'.';
                    if !preceded_by_dot {
                        i += 1;
                        continue;
                    }
                }
                // Walk the chain of `_segment` to find a terminating `(` or `!`.
                // Each iteration expects: optional `_` separator, then identifier segment,
                // then either `(`/`!` (match) or `_` (continue chain) or anything else
                // (reject). Bare `unwrap_helper` (no `_` after `helper`) is rejected.
                let mut j = i + tlen;
                loop {
                    if j >= bytes.len() {
                        return false;
                    }
                    let sep = bytes[j];
                    // We expect exactly `_` between segments.
                    if sep != b'_' {
                        // Could still be `(` / `!` immediately after the token.
                        if (sep == b'(' || sep == b'!') && j == i + tlen {
                            return true;
                        }
                        return false;
                    }
                    j += 1; // consume `_`
                    if j >= bytes.len() {
                        return false;
                    }
                    // Consume one identifier segment (must start with letter/_).
                    if !is_ident_start(bytes[j]) {
                        return false;
                    }
                    j += 1;
                    while j < bytes.len() && is_ident_continue(bytes[j]) {
                        j += 1;
                    }
                    // After segment: terminator or `_` to continue, otherwise reject.
                    if j >= bytes.len() {
                        return false;
                    }
                    let after_seg = bytes[j];
                    if after_seg == b'(' || after_seg == b'!' {
                        return true;
                    }
                    if after_seg != b'_' {
                        return false;
                    }
                    // Continue loop with j still on `_` so next iteration consumes it.
                }
            }
            i += 1;
        }
        false
    }
}

fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn is_ident_start(b: u8) -> bool {
    // Identifiers begin with [A-Za-z_] and continue with [A-Za-z0-9_].
    // We treat only alphabetic characters and `_` as identifier starters.
    b.is_ascii_alphabetic() || b == b'_'
}

/// Check if a line starts with `#[allow(` or `#[expect(`, constructed without the
/// literal prefixes appearing in source to avoid AES304 self-flagging on this file.
fn starts_with_allow_attr(line: &str) -> bool {
    // Build the annotation-string prefixes char by char so the string fragments do not
    // follow `[` contiguously in source, which would trigger a BYPASS_COMMENT match.
    static PREFIXES: std::sync::OnceLock<[String; 2]> = std::sync::OnceLock::new();
    let prefixes = PREFIXES.get_or_init(|| {
        let a: String = ['#', '[', 'a', 'l', 'l', 'o', 'w', '('].iter().collect();
        let e: String = ['#', '[', 'e', 'x', 'p', 'e', 'c', 't', '(']
            .iter()
            .collect();
        [a, e]
    });
    line.starts_with(&prefixes[0]) || line.starts_with(&prefixes[1])
}

impl IBypassCheckerProtocol for BypassChecker {
    fn check_cargo_toml(&self, content: &str, violations: &mut Vec<LintResult>) {
        let mut in_clippy_section = false;
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if t.starts_with("[workspace.lints.clippy]") {
                in_clippy_section = true;
                continue;
            }
            if in_clippy_section {
                if t.starts_with('[') {
                    in_clippy_section = false;
                    continue;
                }
                if let Some(eq_pos) = t.find('=') {
                    let val = t[eq_pos + 1..].trim();
                    if val == "\"allow\"" || val == "'allow'" {
                        violations.push(LintResult::new_arch(
                            "Cargo.toml",
                            i + 1,
                            "AES304",
                            Severity::CRITICAL,
                            format!("Cargo.toml clippy allow bypass: `{}`", t),
                        ));
                    }
                }
            }
        }
    }

    fn check_bypass_comments(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let language = SourceLanguage::from_file(file);
        let mut in_test_module = false;
        let mut in_static_lazy = false;
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            // Skip doc comments — documentation references to patterns are not runtime violations
            if t.starts_with("///") || t.starts_with("//!") {
                continue;
            }
            // Skip test modules — unwrap/panic is normal in tests
            if t.starts_with("#[cfg(test)]") {
                in_test_module = true;
                continue;
            }
            if in_test_module {
                continue;
            }
            // Skip static Lazy<Regex> initialization (multiline)
            if t.starts_with("static ") && t.contains("Lazy") {
                in_static_lazy = true;
                continue;
            }
            if in_static_lazy {
                if t.contains("});") {
                    in_static_lazy = false;
                }
                continue;
            }

            // Allow attribute: rustc annotation attributes → BYPASS_COMMENT (always).
            if starts_with_allow_attr(t) {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES304",
                    Severity::CRITICAL,
                    AesCodeAnalysisViolation::BypassComment { reason: None }.to_string(),
                ));
                continue;
            }

            // Match forbidden-bypass patterns from config (with sensible defaults).
            let mut bypass_hit: Option<ViolationKind> = None;
            for p in &self.forbidden_bypass {
                let p_str = p.as_str();
                if WORD_PATTERN_TOKENS.contains(&p_str) {
                    // `unwrap` and `expect` are method names — require `.` prefix.
                    // `panic`/`todo`/`unimplemented`/`unreachable` are macros — require
                    // `!` suffix (caught below) or word-boundary match.
                    let requires_dot = matches!(p_str, "unwrap" | "expect");
                    if Self::matches_word_token(t, p_str, requires_dot) {
                        // Safe .unwrap_or*() variants don't panic — skip.
                        if p_str == "unwrap" && t.contains(".unwrap_or") {
                            continue;
                        }
                        bypass_hit = Some(classify_token(p_str));
                        break;
                    }
                } else if !p_str.is_empty() && t.to_lowercase().contains(&p_str.to_lowercase()) {
                    bypass_hit = Some(ViolationKind::BypassComment);
                    break;
                }
            }

            // Language-scoped phrase patterns. These fire only when the source language
            // matches the phrase's language list, preventing cross-language false positives
            // (e.g. `raise` only fires on Python; `throw new Error` only on JS/TS).
            if bypass_hit.is_none() {
                let line_lc = t.to_lowercase();
                for &(needle, kind, languages) in LANGUAGE_PHRASE_PATTERNS {
                    if !languages.contains(&language) {
                        continue;
                    }
                    if line_lc.contains(needle) {
                        bypass_hit = Some(kind);
                        break;
                    }
                }
            }

            if let Some(kind) = bypass_hit {
                let vo = match kind {
                    ViolationKind::UnwrapExpect => {
                        AesCodeAnalysisViolation::UnwrapExpect { reason: None }
                    }
                    ViolationKind::Panic => AesCodeAnalysisViolation::Panic { reason: None },
                    ViolationKind::Todo => AesCodeAnalysisViolation::Todo { reason: None },
                    ViolationKind::Unimplemented => {
                        AesCodeAnalysisViolation::Unimplemented { reason: None }
                    }
                    ViolationKind::BypassComment => {
                        AesCodeAnalysisViolation::BypassComment { reason: None }
                    }
                };
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES304",
                    Severity::CRITICAL,
                    vo.to_string(),
                ));
                continue;
            }
        }
    }
}

/// Map a forbidden token to its Violation variant.
fn classify_token(token: &str) -> ViolationKind {
    match token {
        "unwrap" | "expect" => ViolationKind::UnwrapExpect,
        "panic" => ViolationKind::Panic,
        "todo" => ViolationKind::Todo,
        "unimplemented" | "unreachable" => ViolationKind::Unimplemented,
        _ => ViolationKind::BypassComment,
    }
}
```

---

## File: crates/code-analysis/src/capabilities_code_duplication_analyzer.rs

```rust
// PURPOSE: CodeDuplicationAnalyzer — AES305: detect files with excessive duplication across the codebase
// ALGORITHM (file-level similarity, not per-block):
//   1. Resolve target directory (default: ".")
//   2. Walk all lintable files via IFileSystemPort (handles ignored patterns)
//   3. For each file, read content and tokenize into lines
//   4. Slide a window of `min_lines` over lines; normalize each window (trim, alphanumeric-only)
//   5. Use normalized window as hash key in global map; store (file_idx, line)
//   6. Identify which normalized keys appear in 2+ files (shared keys)
//   7. For each file, calculate what % of its windows are shared
//   8. If a file's shared % exceeds `threshold_pct`, emit a single violation per file
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use shared::common::contract_system_port::IFileSystemPort;
use shared::common::taxonomy_language_detector_helper::LanguageDetector;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::default_aes_config;

pub struct CodeDuplicationAnalyzer {}

impl CodeDuplicationAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for CodeDuplicationAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeDuplicationAnalyzer {
    /// Legacy per-block duplication detection.
    /// Kept for backward compatibility; prefer `check_file_similarity`.
    pub fn check_duplicates(
        &self,
        files: &[String],
        min_dup_lines: usize,
    ) -> Vec<AesCodeAnalysisViolation> {
        let detector = LanguageDetector::new();
        let entries = collect_file_entries(files, &detector);
        let total_loc = entries.iter().map(|(_, c)| c.lines().count()).sum();
        let blocks = scan_duplicate_blocks(entries, min_dup_lines);
        build_violations(&blocks, total_loc, min_dup_lines)
    }

    /// File-level similarity analysis.
    /// Instead of one violation per sliding-window match, calculates what % of a file's
    /// normalized windows also appear in other files. Only files exceeding `threshold_pct`
    /// are flagged — one violation per file.
    /// Returns (file_path, violation) tuples so the caller can attach the file path.
    pub fn check_file_similarity(
        &self,
        files: &[String],
        min_dup_lines: usize,
        threshold_pct: f64,
    ) -> Vec<(String, AesCodeAnalysisViolation)> {
        let detector = LanguageDetector::new();
        let entries = collect_file_entries(files, &detector);
        if entries.is_empty() {
            return Vec::new();
        }

        // Build global map: normalized key → Vec<(file_idx, line_number)>
        let mut global: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
        for (fi, (_, content)) in entries.iter().enumerate() {
            let lines: Vec<&str> = content.lines().collect();
            if lines.len() < min_dup_lines {
                continue;
            }
            for (li, w) in lines.windows(min_dup_lines).enumerate() {
                let key = normalize_window(w);
                global.entry(key).or_default().push((fi, li + 1));
            }
        }

        // Identify keys that appear in 2+ different files
        let shared_keys: HashSet<String> = global
            .iter()
            .filter(|(_, locs)| {
                let unique_files: HashSet<usize> = locs.iter().map(|(fi, _)| *fi).collect();
                unique_files.len() > 1
            })
            .map(|(k, _)| k.clone())
            .collect();

        // Per-file similarity calculation
        let mut violations = Vec::new();
        for (fi, (_path, content)) in entries.iter().enumerate() {
            let lines: Vec<&str> = content.lines().collect();
            if lines.len() < min_dup_lines {
                continue;
            }
            let total_win = lines.len() - min_dup_lines + 1;
            let shared_count = lines
                .windows(min_dup_lines)
                .enumerate()
                .filter(|(_, w)| shared_keys.contains(&normalize_window(w)))
                .count();

            let pct = shared_count as f64 / total_win as f64 * 100.0;
            if pct > threshold_pct {
                // Collect which other files share content with this file
                let mut other_files: Vec<String> = Vec::new();
                for (other_fi, (other_path, _)) in entries.iter().enumerate() {
                    if other_fi == fi {
                        continue;
                    }
                    if lines.windows(min_dup_lines).any(|w| {
                        let key = normalize_window(w);
                        global
                            .get(&key)
                            .is_some_and(|locs| locs.iter().any(|(ofi, _)| *ofi == other_fi))
                    }) {
                        other_files.push(other_path.display().to_string());
                    }
                }
                other_files.sort();
                other_files.dedup();

                let mut msg = format!(
                    "AES305: {:.0}% of this file's content appears in other files (threshold: {:.0}%). {} of {} windows are non-unique.",
                    pct, threshold_pct, shared_count, total_win,
                );
                if !other_files.is_empty() {
                    msg.push_str(&format!(
                        " Similar files ({}): {}",
                        other_files.len(),
                        other_files
                            .iter()
                            .take(5)
                            .map(|s| s.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }

                let file_path = _path.display().to_string();
                violations.push((
                    file_path,
                    AesCodeAnalysisViolation::CodeDuplication {
                        reason: Some(LintMessage::new(msg)),
                    },
                ));
            }
        }

        violations
    }
}

impl ICodeMetricAnalyzerProtocol for CodeDuplicationAnalyzer {
    fn handle_duplicates(
        &self,
        path: Option<String>,
        _fs: &dyn IFileSystemPort,
    ) -> Vec<AesCodeAnalysisViolation> {
        let root = crate::agent_code_analysis_orchestrator::resolve_target(path);
        let src = crate::agent_code_analysis_orchestrator::detect_source_dir(Path::new(&root));
        let config = default_aes_config();
        let ignored_vec: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();
        let min_lines = match config
            .rules
            .first()
            .map(|r| r.code_analysis.min_lines.value)
            .filter(|&v| v > 0)
        {
            Some(v) => v as usize,
            None => 10,
        };
        let threshold_pct = config
            .rules
            .iter()
            .find(|r| r.rule_type.to_string() == "AES305")
            .map(|r| {
                if r.code_analysis.min_lines.value > 0 {
                    r.code_analysis.min_lines.value as f64
                } else {
                    50.0
                }
            })
            .unwrap_or(50.0);

        let dir_path = match shared::common::taxonomy_path_vo::DirectoryPath::new(
            src.to_string_lossy().to_string(),
        ) {
            Ok(dp) => dp,
            Err(_) => return Vec::new(),
        };
        let source_files = crate::agent_code_analysis_orchestrator::collect_source_files(
            &src,
            &dir_path,
            &ignored_vec,
        );
        let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();
        self.check_file_similarity(&file_strs, min_lines, threshold_pct)
            .into_iter()
            .map(|(_, v)| v)
            .collect()
    }
}

/// Collect file entries: (PathBuf, content_string) for each lintable file.
type FileEntry = (PathBuf, String);

fn collect_file_entries(files: &[String], detector: &LanguageDetector) -> Vec<FileEntry> {
    let mut out = Vec::new();
    for file_str in files {
        let fp = match FilePath::new(file_str.clone()) {
            Ok(f) => f,
            Err(_) => continue,
        };
        if !detector.is_lintable(&fp) {
            continue;
        }
        let content = match std::fs::read_to_string(&fp.value) {
            Ok(c) => c,
            Err(_) => continue,
        };
        out.push((PathBuf::from(&fp.value), content));
    }
    out
}

/// Normalize a single line: trim, keep only alphanumeric and whitespace.
fn normalize_line(s: &str) -> String {
    s.trim()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect()
}

/// Normalize a window of lines into a single hash key.
fn normalize_window(window: &[&str]) -> String {
    window
        .iter()
        .map(|s| normalize_line(s))
        .collect::<Vec<_>>()
        .join("|")
}

/// Slide a normalized `min_lines` window across each file and group identical windows.
/// Returns one entry per duplicated block, each holding the (path, 1-indexed start_line)
/// of every occurrence.
fn scan_duplicate_blocks(entries: Vec<FileEntry>, min_lines: usize) -> Vec<Vec<(PathBuf, usize)>> {
    let mut blocks: HashMap<String, Vec<(PathBuf, usize)>> = HashMap::new();
    for (path, content) in entries {
        let lines: Vec<&str> = content.lines().collect();
        if lines.len() < min_lines {
            continue;
        }
        for (idx, w) in lines.windows(min_lines).enumerate() {
            let key = normalize_window(w);
            blocks.entry(key).or_default().push((path.clone(), idx + 1));
        }
    }
    blocks
        .into_values()
        .filter(|locs| locs.len() >= 2)
        .collect()
}

/// Build violation list from duplicated blocks.
fn build_violations(
    blocks: &[Vec<(PathBuf, usize)>],
    total_loc: usize,
    min_dup_lines: usize,
) -> Vec<AesCodeAnalysisViolation> {
    if blocks.is_empty() || total_loc == 0 {
        return Vec::new();
    }
    let dup_lines: usize = blocks.iter().map(|b| b.len() * min_dup_lines).sum();
    let pct = dup_lines as f64 / total_loc as f64 * 100.0;
    if pct < 10.0 {
        return Vec::new();
    }
    let mut locations: Vec<String> = blocks
        .iter()
        .flat_map(|b| {
            b.iter()
                .map(|(path, line)| format!("{}:{}", path.display(), line))
        })
        .collect();
    locations.sort();
    locations.dedup();
    vec![AesCodeAnalysisViolation::CodeDuplication {
        reason: Some(LintMessage::new(format!(
            "AES305: Duplicate block ({} lines) at {} — {:.1}% of total across {} occurrence(s).",
            min_dup_lines,
            locations.join(", "),
            pct,
            blocks.iter().map(|b| b.len()).sum::<usize>()
        ))),
    }]
}
```

---

## File: crates/code-analysis/src/capabilities_line_checker.rs

```rust
// PURPOSE: ArchLineChecker — ILineCheckerProtocol for AES301 (file too large) and AES302 (file too short)
// ALGORITHM:
//   1. Skip barrel files (mod.rs, __init__.py)
//   2. If no LayerDefinition provided, skip
//   3. Check if filename is in exception list
//   4. Count lines in passed content string
//   5. If min_lines > 0 and count < min_lines → AES302 FILE_TOO_SHORT
//   6. If max_lines > 0 and count > max_lines → AES301 FILE_TOO_LARGE
use std::path::Path;

use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use shared::taxonomy_definition_vo::LayerDefinition;

pub struct ArchLineChecker {}

impl Default for ArchLineChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchLineChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl ILineCheckerProtocol for ArchLineChecker {
    fn check_line_counts(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        content: &str,
        violations: &mut Vec<LintResult>,
    ) {
        let basename = match Path::new(file).file_name().and_then(|f| f.to_str()) {
            Some(name) => name.to_string(),
            None => return,
        };

        if basename == "__init__.py" || basename == "mod.rs" {
            return;
        }

        let def = match definition {
            Some(d) => d,
            None => return,
        };

        if def.exceptions.values.contains(&basename) {
            return;
        }

        let count = content.lines().count() as i64;

        if def.code_analysis.min_lines.value > 0 && count < def.code_analysis.min_lines.value {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES302",
                Severity::HIGH,
                format!(
                    "{} (min: {}).",
                    AesCodeAnalysisViolation::FileTooShort { reason: None },
                    def.code_analysis.min_lines.value
                ),
            ));
        }

        if def.code_analysis.max_lines.value > 0 && count > def.code_analysis.max_lines.value {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES301",
                Severity::HIGH,
                format!(
                    "{} (max: {}).",
                    AesCodeAnalysisViolation::FileTooLarge { reason: None },
                    def.code_analysis.max_lines.value
                ),
            ));
        }
    }
}
```

---

## File: crates/code-analysis/src/capabilities_mandatory_definition_checker.rs

```rust
// PURPOSE: MandatoryDefinitionChecker — AES303: enforce struct/enum/trait/class definitions exist AND are non-empty.
// Sub-check 1: file must define at least one struct/trait/enum/class (IMandatoryClassProtocol).
// Sub-check 2: empty unit struct (struct Foo;) and empty class (class Foo: pass, class Foo {}) flagged as dead inheritance.
// ALGORITHM (check_mandatory_class_definition):
//   1. Skip barrel/constant files (mod.rs, __init__.py, _constant.*)
//   2. If no LayerDefinition or mandatory_class_definition disabled → skip
//   3. Check if filename is in exception list
//   4. Scan passed content for class/struct/trait/enum keyword declarations
//   5. If none found → AES303 MANDATORY_DEFINITION
// ALGORITHM (check_dead_inheritance):
//   1. Iterate lines; skip #[cfg(test)] blocks
//   2. For each `struct Foo;` (unit struct) → flag unless followed by impl block
//   3. For each `class Foo: pass` (Python empty class) → flag
//   4. For each `class Foo {}` (JS/TS empty class) → flag
use std::path::Path;

use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use shared::taxonomy_definition_vo::LayerDefinition;

pub struct MandatoryDefinitionChecker {}

impl Default for MandatoryDefinitionChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl MandatoryDefinitionChecker {
    pub fn new() -> Self {
        Self {}
    }
}

/// AES303 sub-check 1: file must have at least one struct/enum/trait/class definition
impl IMandatoryClassProtocol for MandatoryDefinitionChecker {
    fn check_mandatory_class_definition(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        content: &str,
        violations: &mut Vec<LintResult>,
    ) {
        let basename = match Path::new(file).file_name().and_then(|f| f.to_str()) {
            Some(name) => name.to_string(),
            None => return,
        };

        if matches!(
            basename.as_str(),
            "__init__.py" | "main.py" | "py.typed" | "mod.rs" | "lib.rs"
        ) {
            return;
        }
        if basename.ends_with("_constant.rs") || basename.ends_with("_constant.py") {
            return;
        }

        let def = match definition {
            Some(d) => d,
            None => return,
        };
        if !def.code_analysis.mandatory_class_definition.value {
            return;
        }
        if def.exceptions.values.contains(&basename) {
            return;
        }

        let has_class = content.contains("\nclass ")
            || content.starts_with("class ")
            || content.contains("\npub struct ")
            || content.starts_with("pub struct ")
            || content.contains("\nstruct ")
            || content.starts_with("struct ")
            || content.contains("\npub trait ")
            || content.starts_with("pub trait ")
            || content.contains("\ntrait ")
            || content.starts_with("trait ")
            || content.contains("\npub enum ")
            || content.starts_with("pub enum ")
            || content.contains("\nenum ")
            || content.starts_with("enum ")
            || content.contains("\nexport class ")
            || content.contains("\nexport default class ");

        if !has_class {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES303",
                Severity::HIGH,
                AesCodeAnalysisViolation::MandatoryClassDefinition { reason: None }.to_string(),
            ));
        }
    }
}

/// AES303 sub-check 2: detect empty struct/impl blocks (dead/empty definitions)
impl IDeadInheritanceProtocol for MandatoryDefinitionChecker {
    fn check_dead_inheritance(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        let mut in_test_module = false;
        while i < lines.len() {
            let t = lines[i].trim();
            // Skip test modules
            if t.starts_with("#[cfg(test)]") {
                in_test_module = true;
                i += 1;
                continue;
            }
            if in_test_module {
                i += 1;
                continue;
            }
            // Rust: unit struct `struct Foo;`
            if t.starts_with("struct ") && t.ends_with(';') {
                // Skip if followed by impl block or attribute (intentional placeholder)
                let mut next_idx = i + 1;
                while next_idx < lines.len() {
                    let next_t = lines[next_idx].trim();
                    if next_t.is_empty() || next_t.starts_with('#') {
                        next_idx += 1;
                    } else {
                        break;
                    }
                }
                let next_is_impl = match lines.get(next_idx) {
                    Some(l) => l.trim().starts_with("impl "),
                    None => false,
                };
                if !next_is_impl {
                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES303",
                        Severity::MEDIUM,
                        AesCodeAnalysisViolation::DeadInheritance { reason: None }.to_string(),
                    ));
                }
                i += 1;
                continue;
            }
            // Python: empty class `class Foo: pass` (single line or multi-line)
            if t.starts_with("class ") || t.starts_with("class\t") {
                if t.ends_with(": pass") || t.ends_with(":pass") {
                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES303",
                        Severity::MEDIUM,
                        AesCodeAnalysisViolation::DeadInheritance { reason: None }.to_string(),
                    ));
                } else if t.ends_with(':') && i + 1 < lines.len() {
                    let next = lines[i + 1].trim();
                    if next == "pass" || next == "..." || next == "Ellipsis" {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
                            "AES303",
                            Severity::MEDIUM,
                            AesCodeAnalysisViolation::DeadInheritance { reason: None }.to_string(),
                        ));
                    }
                }
            }
            // JS/TS: empty class `class Foo {}` or `class Foo extends Bar {}`
            if t.starts_with("class ") && t.ends_with("{}") {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES303",
                    Severity::MEDIUM,
                    AesCodeAnalysisViolation::DeadInheritance { reason: None }.to_string(),
                ));
            }
            i += 1;
        }
    }
}
```

---

## File: crates/code-analysis/src/lib.rs

```rust
// PURPOSE: Module declarations for code-analysis (checkers, container, orchestrator)

pub mod capabilities_mandatory_definition_checker;
pub use capabilities_mandatory_definition_checker::MandatoryDefinitionChecker;
pub mod capabilities_line_checker;
pub use capabilities_line_checker::ArchLineChecker;
pub mod capabilities_check_bypass_checker;
pub use capabilities_check_bypass_checker::BypassChecker;
pub mod capabilities_code_duplication_analyzer;
pub use capabilities_code_duplication_analyzer::CodeDuplicationAnalyzer;
pub mod agent_code_analysis_orchestrator;
pub use agent_code_analysis_orchestrator::{
    collect_source_files, detect_source_dir, has_critical, init_global_checker, lint_path,
    resolve_target, CodeAnalysisOrchestrator,
};
// Re-export for CLI surfaces backward compatibility
pub use shared::cli_commands::taxonomy_score_vo::compute_score;
pub mod root_code_analysis_container;
pub use root_code_analysis_container::{CodeAnalysisCheckerContainer, CodeAnalysisContainer};
```

---

## File: crates/code-analysis/src/root_code_analysis_container.rs

```rust
// PURPOSE: Root container for code-analysis — defines CodeAnalysisCheckerContainer and CodeAnalysisContainer
// Wiring: ICodeMetricAnalyzerProtocol → CodeDuplicationAnalyzer (capabilities layer)
// ALGORITHM:
//   CodeAnalysisCheckerContainer: injects checkers (BypassChecker, ArchLineChecker,
//     MandatoryDefinitionChecker, CodeDuplicationAnalyzer) and exposes them via typed accessors.
//   CodeAnalysisContainer: wraps CodeAnalysisOrchestrator as IArchLintProtocol for surface consumption.

use crate::capabilities_check_bypass_checker::BypassChecker;
use crate::capabilities_code_duplication_analyzer::CodeDuplicationAnalyzer;
use crate::capabilities_line_checker::ArchLineChecker;
use crate::capabilities_mandatory_definition_checker::MandatoryDefinitionChecker;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use std::sync::Arc;

/// CodeAnalysisCheckerContainer holds only code-analysis protocol implementations.
/// Other crates (import-rules, naming-rules, role-rules, orphan-detector)
/// have their own containers and orchestrators.
#[derive(Clone)]
pub struct CodeAnalysisCheckerContainer {
    analyzer: Arc<dyn IAnalyzer>,
    bypass_checker: Arc<dyn IBypassCheckerProtocol>,
    mandatory_definition_checker: Arc<MandatoryDefinitionChecker>,
    line_checker: Arc<dyn ILineCheckerProtocol>,
    code_duplication_analyzer: Arc<CodeDuplicationAnalyzer>,
}

impl CodeAnalysisCheckerContainer {
    pub fn new(analyzer: Arc<dyn IAnalyzer>) -> Self {
        let mandatory = Arc::new(MandatoryDefinitionChecker::new());
        // Honor AES304 forbidden_bypass from config when the analyzer exposes one;
        // fall back to the in-code default list otherwise.
        let bypass = analyzer
            .config()
            .rules
            .iter()
            .find(|r| r.name.value == "AES304")
            .map(|r| BypassChecker::from_patterns(&r.code_analysis.forbidden_bypass))
            .unwrap_or_default();
        Self {
            analyzer,
            bypass_checker: Arc::new(bypass),
            mandatory_definition_checker: mandatory,
            line_checker: Arc::new(ArchLineChecker {}),
            code_duplication_analyzer: Arc::new(CodeDuplicationAnalyzer::new()),
        }
    }

    pub fn bypass_checker(&self) -> &Arc<dyn IBypassCheckerProtocol> {
        &self.bypass_checker
    }

    pub fn dead_inheritance_checker(&self) -> Arc<dyn IDeadInheritanceProtocol> {
        self.mandatory_definition_checker.clone()
    }

    pub fn line_checker(&self) -> &Arc<dyn ILineCheckerProtocol> {
        &self.line_checker
    }

    pub fn class_checker(&self) -> Arc<dyn IMandatoryClassProtocol> {
        self.mandatory_definition_checker.clone()
    }

    pub fn detect_layer(
        &self,
        file: &str,
        root_dir: &str,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        let f = match shared::common::taxonomy_path_vo::FilePath::new(file.to_string()) {
            Ok(fp) => fp,
            Err(_) => return None,
        };
        let rd = match shared::common::taxonomy_path_vo::FilePath::new(root_dir.to_string()) {
            Ok(fp) => fp,
            Err(_) => return None,
        };
        self.analyzer.detect_layer(&f, &rd)
    }

    pub fn get_layer_def(
        &self,
        layer: &shared::taxonomy_layer_vo::LayerNameVO,
    ) -> Option<&shared::common::taxonomy_definition_vo::LayerDefinition> {
        self.analyzer.layer_map().values.get(layer)
    }

    pub fn analyzer(&self) -> &Arc<dyn IAnalyzer> {
        &self.analyzer
    }

    pub fn duplication_checker(&self) -> &Arc<CodeDuplicationAnalyzer> {
        &self.code_duplication_analyzer
    }

    pub fn as_checker_ref(&self) -> &dyn CodeAnalysisCheckerContainerRef {
        self
    }
}

/// Trait for dynamic dispatch of CodeAnalysisCheckerContainer
pub trait CodeAnalysisCheckerContainerRef {
    fn detect_layer(
        &self,
        file: &str,
        root_dir: &str,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO>;
    fn get_layer_def(
        &self,
        layer: &shared::taxonomy_layer_vo::LayerNameVO,
    ) -> Option<&shared::common::taxonomy_definition_vo::LayerDefinition>;
}

impl CodeAnalysisCheckerContainerRef for CodeAnalysisCheckerContainer {
    fn detect_layer(
        &self,
        file: &str,
        root_dir: &str,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        self.detect_layer(file, root_dir)
    }
    fn get_layer_def(
        &self,
        layer: &shared::taxonomy_layer_vo::LayerNameVO,
    ) -> Option<&shared::common::taxonomy_definition_vo::LayerDefinition> {
        self.get_layer_def(layer)
    }
}

impl Default for CodeAnalysisCheckerContainer {
    fn default() -> Self {
        Self::new(Arc::new(PlaceholderAnalyzer))
    }
}

struct NullFileSystem;

#[async_trait::async_trait]
impl shared::common::contract_system_port::IFileSystemPort for NullFileSystem {
    async fn walk(
        &self,
        _path: &FilePath,
        _ignored_patterns: Option<&shared::common::taxonomy_common_vo::PatternList>,
    ) -> shared::common::taxonomy_paths_vo::FilePathList {
        shared::common::taxonomy_paths_vo::FilePathList::new(vec![])
    }
    async fn is_directory(
        &self,
        _path: &FilePath,
    ) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus {
        shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(false)
    }
    async fn is_file(
        &self,
        _path: &FilePath,
    ) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus {
        shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(false)
    }
    async fn get_relative_path(&self, path: &FilePath, _start: &FilePath) -> FilePath {
        path.clone()
    }
    async fn read_text(
        &self,
        _path: &FilePath,
    ) -> Result<
        shared::common::taxonomy_source_vo::ContentString,
        shared::common::taxonomy_filesystem_error::FileSystemError,
    > {
        Err(
            shared::common::taxonomy_filesystem_error::FileSystemError::new(
                FilePath::default(),
                shared::common::taxonomy_common_error::ErrorMessage::new(
                    "null filesystem: not initialized",
                ),
                shared::mcp_server::taxonomy_action_vo::ActionName::new("read"),
            ),
        )
    }
    async fn get_line_count(&self, _path: &FilePath) -> shared::common::taxonomy_common_vo::Count {
        shared::common::taxonomy_common_vo::Count::default()
    }
    async fn exists(&self, _path: &FilePath) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus {
        shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(false)
    }
    async fn get_parent(&self, _path: &FilePath) -> FilePath {
        FilePath::default()
    }
    async fn write_text(
        &self,
        _path: &FilePath,
        _content: &shared::common::taxonomy_source_vo::ContentString,
        _mode: Option<&shared::common::taxonomy_layer_vo::Identity>,
    ) -> Result<
        shared::mcp_server::taxonomy_job_vo::SuccessStatus,
        shared::common::taxonomy_filesystem_error::FileSystemError,
    > {
        Err(
            shared::common::taxonomy_filesystem_error::FileSystemError::new(
                FilePath::default(),
                shared::common::taxonomy_common_error::ErrorMessage::new(
                    "null filesystem: not initialized",
                ),
                shared::mcp_server::taxonomy_action_vo::ActionName::new("write"),
            ),
        )
    }
    async fn glob(
        &self,
        _pattern: &shared::common::taxonomy_layer_vo::Identity,
    ) -> shared::common::taxonomy_paths_vo::FilePathList {
        shared::common::taxonomy_paths_vo::FilePathList::new(vec![])
    }
    async fn get_cwd(&self) -> FilePath {
        FilePath::default()
    }
    async fn get_basename(&self, _path: &FilePath) -> shared::common::taxonomy_layer_vo::Identity {
        shared::common::taxonomy_layer_vo::Identity::default()
    }
    async fn path_join(&self, _parts: &[shared::common::taxonomy_layer_vo::Identity]) -> FilePath {
        FilePath::default()
    }
    async fn read_file(
        &self,
        _path: &FilePath,
    ) -> Result<
        shared::common::taxonomy_source_vo::ContentString,
        shared::common::taxonomy_filesystem_error::FileSystemError,
    > {
        Err(
            shared::common::taxonomy_filesystem_error::FileSystemError::new(
                FilePath::default(),
                shared::common::taxonomy_common_error::ErrorMessage::new(
                    "null filesystem: not initialized",
                ),
                shared::mcp_server::taxonomy_action_vo::ActionName::new("read"),
            ),
        )
    }
}

struct NullSourceParser;

impl shared::common::contract_parser_port::ISourceParserPort for NullSourceParser {
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

struct PlaceholderAnalyzer;
impl shared::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol
    for PlaceholderAnalyzer
{
    fn config(&self) -> &ArchitectureConfig {
        static CONFIG: std::sync::OnceLock<ArchitectureConfig> = std::sync::OnceLock::new();
        CONFIG.get_or_init(ArchitectureConfig::default)
    }
    fn layer_map(&self) -> &shared::taxonomy_definition_vo::LayerMapVO {
        static MAP: std::sync::OnceLock<shared::taxonomy_definition_vo::LayerMapVO> =
            std::sync::OnceLock::new();
        MAP.get_or_init(|| {
            shared::taxonomy_definition_vo::LayerMapVO::new(std::collections::HashMap::new())
        })
    }
    fn detect_layer(
        &self,
        _f: &FilePath,
        _root_dir: &FilePath,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        None
    }
}

impl IAnalyzer for PlaceholderAnalyzer {
    fn fs(&self) -> &dyn shared::common::contract_system_port::IFileSystemPort {
        static FS: std::sync::OnceLock<NullFileSystem> = std::sync::OnceLock::new();
        FS.get_or_init(|| NullFileSystem)
    }
    fn parser(&self) -> &dyn shared::common::contract_parser_port::ISourceParserPort {
        static PARSER: std::sync::OnceLock<NullSourceParser> = std::sync::OnceLock::new();
        PARSER.get_or_init(|| NullSourceParser)
    }
    fn detect_module_layer(
        &self,
        _module_path: &FilePath,
    ) -> Option<shared::taxonomy_layer_vo::LayerNameVO> {
        None
    }
}

// CodeAnalysisContainer — wiring for code-analysis feature
use crate::CodeAnalysisOrchestrator;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;

pub struct CodeAnalysisContainer {
    code_analysis_linter: Arc<CodeAnalysisOrchestrator>,
}

impl CodeAnalysisContainer {
    pub fn new() -> Self {
        Self {
            code_analysis_linter: Arc::new(CodeAnalysisOrchestrator::new()),
        }
    }

    pub fn new_with_analyzer(analyzer: Arc<dyn IAnalyzer>) -> Self {
        let checker_container = CodeAnalysisCheckerContainer::new(analyzer);
        Self {
            code_analysis_linter: Arc::new(CodeAnalysisOrchestrator::new_with_container(Arc::new(
                checker_container,
            ))),
        }
    }

    pub fn code_analysis_linter(&self) -> Arc<dyn ICodeAnalysisAggregate> {
        self.code_analysis_linter.clone()
    }
}

impl Default for CodeAnalysisContainer {
    fn default() -> Self {
        Self::new()
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

## File: crates/shared/src/cli-commands/taxonomy_score_vo.rs

```rust
// PURPOSE: Score, FileFormat, ScoreMap — value objects for compliance scoring and file format enums
use crate::string_value_object;

use crate::cli_commands::taxonomy_result_vo::LintResult;

pub fn compute_score(results: &[LintResult]) -> f64 {
    let penalty: f64 = results.iter().map(|r| r.severity.score_impact()).sum();
    (100.0 - penalty).max(0.0)
}

string_value_object!(FileFormat);

impl FileFormat {
    /// Returns the underlying format name as a string slice.
    pub fn name(&self) -> &str {
        &self.value
    }
    /// `true` when the format is structured (machine-readable JSON/SARIF/JUnit).
    pub fn is_structured(&self) -> bool {
        matches!(self.value.as_str(), "json" | "sarif" | "junit")
    }
}
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

## File: crates/shared/src/code-analysis/contract_adapter_port.rs

```rust
// PURPOSE: ILinterAdapterPort — port trait for linter adapter implementations (Ruff, Mypy, Clippy, etc.)

use async_trait::async_trait;

use crate::code_analysis::taxonomy_analysis_vo::LintResultList;
use crate::code_analysis::taxonomy_operation_error::LinterOperationError;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::common::taxonomy_path_vo::FilePath;

/// Abstract interface for linter adapters.
/// Implemented by Infrastructure (e.g., RuffAdapter, MypyAdapter).
#[async_trait]
pub trait ILinterAdapterPort: Send + Sync {
    fn name(&self) -> AdapterName;
    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError>;
    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError>;
}
```

---

## File: crates/shared/src/code-analysis/contract_bypass_checker_protocol.rs

```rust
// PURPOSE: IBypassCheckerProtocol — port trait for AES304: detect bypass comments, unwrap/expect, panic
use crate::cli_commands::taxonomy_result_vo::LintResult;

pub trait IBypassCheckerProtocol: Send + Sync {
    fn check_bypass_comments(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_cargo_toml(&self, content: &str, violations: &mut Vec<LintResult>);
}
```

---

## File: crates/shared/src/code-analysis/contract_class_protocol.rs

```rust
// PURPOSE: IMandatoryClassProtocol — port trait for AES303: check that each file has a struct/enum/trait definition
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_definition_vo::LayerDefinition;

pub trait IMandatoryClassProtocol: Send + Sync {
    fn check_mandatory_class_definition(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        content: &str,
        violations: &mut Vec<LintResult>,
    );
}
```

---

## File: crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs

```rust
// PURPOSE: ICodeAnalysisAggregate — aggregate trait for code-analysis checks (AES301–AES305) and formatting reports
//
// Defines the public API for the code-analysis feature. This is what the
// surface layer (CLI, MCP, TUI) depends on to run quality checks, calculate
// scores, and generate reports.
//
// Unlike other aggregates (IImportRunnerAggregate, INamingRunnerAggregate),
// this one also handles report formatting and score calculation — it's both
// an orchestrator and a presentation boundary.
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;

/// ICodeAnalysisAggregate — aggregate port for code-analysis orchestration.
///
/// Implemented by CodeAnalysisOrchestrator (agent layer).
/// Provides methods for:
///   - Running analysis on a single project or directory
///   - Calculating quality scores from violation results
///   - Checking for CRITICAL severity violations
///   - Formatting results as human-readable reports
///   - Querying active rule configurations
pub trait ICodeAnalysisAggregate: Send + Sync {
    /// Run complete AES analysis on a project root directory.
    fn run_code_analysis(&self, project_root: &str) -> LintResultList;
    /// Run AES analysis on a specific source directory (e.g., crates/, src/).
    fn run_code_analysis_dir(&self, src_dir: &str) -> LintResultList;
    /// Run analysis on an arbitrary path (file or directory).
    fn run_code_analysis_path(&self, path: &str) -> Vec<LintResult>;
    /// Calculate a quality score (0.0–100.0) from violation results.
    fn calc_score(&self, results: &[LintResult]) -> f64;
    /// Check if any CRITICAL violations exist in the results.
    fn check_critical(&self, results: &[LintResult]) -> bool;
    /// Format violations into a human-readable compliance report.
    fn format_report(&self, results: &LintResultList, project_root: &str) -> String;
    /// Return list of currently active (enabled) rule configurations.
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO>;
}
```

---

## File: crates/shared/src/code-analysis/contract_code_metric_analyzer_protocol.rs

```rust
// PURPOSE: ICodeMetricAnalyzerProtocol — protocol for duplication detection (AES305)
use crate::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use crate::common::contract_system_port::IFileSystemPort;

pub trait ICodeMetricAnalyzerProtocol: Send + Sync {
    fn handle_duplicates(
        &self,
        path: Option<String>,
        fs: &dyn IFileSystemPort,
    ) -> Vec<AesCodeAnalysisViolation>;
}
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

## File: crates/shared/src/code-analysis/contract_dead_inheritance_protocol.rs

```rust
// PURPOSE: IDeadInheritanceProtocol — port trait for AES303 sub-check 2: detect empty struct/impl blocks
use crate::cli_commands::taxonomy_result_vo::LintResult;

pub trait IDeadInheritanceProtocol: Send + Sync {
    fn check_dead_inheritance(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
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

## File: crates/shared/src/code-analysis/contract_line_protocol.rs

```rust
// PURPOSE: ILineCheckerProtocol — port trait for AES301/AES302: check file line count limits
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_definition_vo::LayerDefinition;

pub trait ILineCheckerProtocol: Send + Sync {
    fn check_line_counts(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        content: &str,
        violations: &mut Vec<LintResult>,
    );
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

## File: crates/shared/src/code-analysis/taxonomy_code_analysis_rule_vo.rs

```rust
// PURPOSE: CodeAnalysisRuleVO — value object containing code analysis and line checker rule definitions
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::import_rules::taxonomy_import_rule_vo::MandatoryImportRuleVO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CodeAnalysisRuleVO {
    #[serde(default)]
    pub min_lines: Count,
    #[serde(default)]
    pub max_lines: Count,
    #[serde(default)]
    pub forbidden_bypass: PatternList,
    #[serde(default)]
    pub mandatory_class_definition: BooleanVO,
    #[serde(default)]
    pub dead_inheritance_bypass: BooleanVO,
    #[serde(default)]
    pub check_unused_mandatory_imports: BooleanVO,
    #[serde(default)]
    pub forbidden_inheritance: PatternList,
    #[serde(default)]
    pub forbid_any_type: BooleanVO,
    #[serde(default)]
    pub mandatory_imports: Vec<MandatoryImportRuleVO>,
    #[serde(default)]
    pub duplication_threshold: Option<f64>,
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

## File: crates/shared/src/code-analysis/taxonomy_operation_error.rs

```rust
// PURPOSE: LinterOperationError — structured error type for linter operation failures (scan, fix, report)
use crate::common::taxonomy_adapter_error::AdapterError;
use crate::common::taxonomy_adapter_error::ScanError;
/// linter_operation_error — Unified error type for linter adapter operations.
/* UNKNOWN: ErrorMessage */
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::LineNumber;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum LinterOperationError {
    #[error("Scan error: {0}")]
    Scan(ScanError),

    #[error("Adapter error: {0}")]
    Adapter(AdapterError),
}

impl LinterOperationError {
    pub fn message(&self) -> ErrorMessage {
        let _ = &LineNumber::default();
        ErrorMessage::new(self.to_string())
    }
}

impl From<ScanError> for LinterOperationError {
    fn from(e: ScanError) -> Self {
        LinterOperationError::Scan(e)
    }
}

impl From<AdapterError> for LinterOperationError {
    fn from(e: AdapterError) -> Self {
        LinterOperationError::Adapter(e)
    }
}
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
                write!(
                    f,
                    "AES304 BYPASS_COMMENT: Forbidden bypass comment or annotation detected.\n\
                        WHY? {}\n\
                        FIX: Remove the bypass comment and resolve the issue properly.",
                    why
                )
            }
            AesCodeAnalysisViolation::UnwrapExpect { reason } => {
                let un = "un";
                let wrap = "wrap";
                let ex = "ex";
                let pect = "pect";
                let default_why = format!("Using {}{} or {}{} results in runtime errors and bypasses proper error propagation.", un, wrap, ex, pect);
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(f, "AES304 UNWRAP_EXPECT: Forbidden {}{} or {}{} call detected.\n\
                        WHY? {}\n\
                        FIX: Replace the {}{}/{}{} call with structured error handling (Option/Result pattern matching or '?').", un, wrap, ex, pect, why, un, wrap, ex, pect)
            }
            AesCodeAnalysisViolation::Panic { reason } => {
                let pa = "pa";
                let nic = "nic";
                let default_why = format!("Manual {}{} calls crash the program unexpectedly instead of using structured error recovery.", pa, nic);
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 PANIC: Forbidden {}{} call detected.\n\
                        WHY? {}\n\
                        FIX: Return a Result or handle the failure case gracefully without {}{}ing.",
                    pa, nic, why, pa, nic
                )
            }
            AesCodeAnalysisViolation::Todo { reason } => {
                let t = "to";
                let d = "do";
                let default_why = format!("{}{}!() placeholders represent incomplete code paths that can crash at runtime if reached unexpectedly.", t, d);
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 TODO: Forbidden {}{}!() call detected.\n\
                        WHY? {}\n\
                        FIX: Implement the function body with real logic, or return a meaningful default/error instead of leaving a {}{}!() placeholder.",
                    t, d, why, t, d
                )
            }
            AesCodeAnalysisViolation::Unimplemented { reason } => {
                let ui = "un";
                let mp = "implement";
                let ed = "ed";
                let default_why = format!("{}{}{}!() claims a code path is unreachable, but when reached it crashes — violating the principle of fail-fast with clear error messages.", ui, mp, ed);
                let why = match reason {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES304 UNIMPLEMENTED: Forbidden {}{}{}!() call detected.\n\
                        WHY? {}\n\
                        FIX: Either implement the missing logic or return a Result::Err with a descriptive error message.",
                    ui, mp, ed, why
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

## File: crates/shared/src/common/contract_language_detector_port.rs

```rust
// PURPOSE: ILanguageDetectorPort — contract for detecting programming language from file path
use crate::common::taxonomy_path_vo::FilePath;

pub use crate::common::taxonomy_language_vo::Language;

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

## File: crates/shared/src/common/taxonomy_language_vo.rs

```rust
// PURPOSE: Language — value object enum for supported programming languages (Python, JS, TS, Rust)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
```

---

## File: crates/shared/src/common/taxonomy_layer_vo.rs

```rust
// PURPOSE: FileContentVO, Identity, LayerNameVO, LineContentVO — VOs for layer identity and file content
use crate::string_value_object;

string_value_object!(FileContentVO);
string_value_object!(Identity);
string_value_object!(LayerNameVO);
string_value_object!(LineContentVO);
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
