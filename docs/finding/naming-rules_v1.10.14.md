# Crate: naming-rules (v1.10.14)

This document contains the source code for feature crate `naming-rules` along with its corresponding and imported definitions from the `shared` crate.

## Problem Statement

The following issues were detected by `lint-arwaky-cli scan`:

```
============================================================
  AES Architecture Compliance Report
============================================================
  Project: /home/raka/mcp-arwaky/lint-arwaky/crates/naming-rules
  Violations: 0
```

---

## File List

- [crates/naming-rules/Cargo.toml](file:///home/raka/mcp-arwaky/lint-arwaky/crates/naming-rules/Cargo.toml)
- [crates/naming-rules/FRD.md](file:///home/raka/mcp-arwaky/lint-arwaky/crates/naming-rules/FRD.md)
- [crates/naming-rules/src/agent_naming_orchestrator.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/naming-rules/src/agent_naming_orchestrator.rs)
- [crates/naming-rules/src/capabilities_naming_convention_checker.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/naming-rules/src/capabilities_naming_convention_checker.rs)
- [crates/naming-rules/src/capabilities_suffix_prefix_checker.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/naming-rules/src/capabilities_suffix_prefix_checker.rs)
- [crates/naming-rules/src/infrastructure_filesystem_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/naming-rules/src/infrastructure_filesystem_adapter.rs)
- [crates/naming-rules/src/lib.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/naming-rules/src/lib.rs)
- [crates/naming-rules/src/root_naming_rules_container.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/naming-rules/src/root_naming_rules_container.rs)
- [crates/shared/src/cli-commands/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/mod.rs)
- [crates/shared/src/cli-commands/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_severity_vo.rs)
- [crates/shared/src/common/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/mod.rs)
- [crates/shared/src/common/taxonomy_adapter_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_name_vo.rs)
- [crates/shared/src/common/taxonomy_common_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_vo.rs)
- [crates/shared/src/common/taxonomy_definition_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_definition_vo.rs)
- [crates/shared/src/common/taxonomy_error_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_error_vo.rs)
- [crates/shared/src/common/taxonomy_lint_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_lint_vo.rs)
- [crates/shared/src/common/taxonomy_path_utils_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_path_utils_vo.rs)
- [crates/shared/src/common/taxonomy_path_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_path_vo.rs)
- [crates/shared/src/common/taxonomy_paths_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_paths_vo.rs)
- [crates/shared/src/common/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_severity_vo.rs)
- [crates/shared/src/config-system/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/mod.rs)
- [crates/shared/src/config-system/taxonomy_config_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_config_vo.rs)
- [crates/shared/src/naming-rules/contract_naming_analyzer_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/contract_naming_analyzer_protocol.rs)
- [crates/shared/src/naming-rules/contract_naming_checker_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/contract_naming_checker_protocol.rs)
- [crates/shared/src/naming-rules/contract_naming_filesystem_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/contract_naming_filesystem_port.rs)
- [crates/shared/src/naming-rules/contract_naming_runner_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/contract_naming_runner_aggregate.rs)
- [crates/shared/src/naming-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/mod.rs)
- [crates/shared/src/naming-rules/taxonomy_naming_rule_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/taxonomy_naming_rule_vo.rs)
- [crates/shared/src/naming-rules/taxonomy_naming_violation_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/taxonomy_naming_violation_vo.rs)
- [crates/shared/src/naming-rules/taxonomy_suffix_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/taxonomy_suffix_vo.rs)

---

## File: crates/naming-rules/Cargo.toml

```toml
[package]
name = "naming_rules-lint-arwaky"
version = "1.10.14"
edition = "2021"
description = "Naming-convention enforcer for AES101 (lowercase + underscore snake_case stems) and AES102 (suffix taxonomy `_checker`/`_analyzer`/...)."
license = "MIT"
repository = "https://github.com/rakaarwaky/lint-arwaky"
publish = false

[lints]
workspace = true

[dependencies]  # (unchanged)
async-trait.workspace = true
regex.workspace = true
shared.workspace = true
```

---

## File: crates/naming-rules/FRD.md

```rust
# Feature Requirement Document (FRD) - Naming Rules

## 1. Feature Goal

The primary objective of the `naming-rules` feature is to enforce strict naming conventions across the codebase to ensure consistency, readability, and adherence to the 7-layer architecture system. By validating that files and identifiers conform to structural and semantic naming patterns, it prevents naming chaos and helps developers instantly recognize the architectural role of any component based solely on its name.

## 2. Requirements & Scope

The `naming-rules` module is responsible for scanning workspace source files and auditing naming compliance based on the following rules:

### Rules Specifications

- **AES101: Name Convention Consistency**

  - **Requirement**: All file stems (basenames without extensions) must be in `snake_case` (lowercase letters and underscores only).
  - **Scope**: Applies to all scanned source files in the project (Rust, Python, JavaScript, TypeScript).
  - **Length Constraint**: Each name segment must be at least 2 words or meet project-defined length patterns to avoid overly short/cryptic names (e.g., `db.rs` is flagged; `db_connector.rs` is accepted).

- **AES102: Suffix and Prefix Layer Alignment**

  - **Requirement**: Files must match the specific naming convention of the architectural layer they belong to. The architectural layer of a file is determined by its prefix, and its suffix must align with its definition.
  - **Scope**:
    - `taxonomy_` files must end with allowed suffixes: `_vo`, `_entity`, `_event`, `_error`, `_constant`, `_utility`, `_helper`.
    - `contract_` files must end with: `_port`, `_protocol`, `_aggregate`.
    - `capabilities_` files must end with: `_checker`, `_analyzer`, `_processor`, etc.
    - `infrastructure_` files must end with: `_adapter`, `_provider`, `_scanner`, etc.
    - `agent_` files must end with: `_orchestrator` only.
    - `surface_` files must end with: `_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_hook`, `_store`, `_action`, `_screen`.
    - `root_` files must end with: `_container`, `_entry`.

### Inputs

- A list of source file paths (`&[FilePath]`) and the current project configuration (`ArchitectureConfig`).

### Outputs

- A list of naming violations containing the file path, violating name, the exact rule violated (AES101/AES102), and detailed error descriptions.

---

## 3. Success Indicators

The success of the `naming-rules` module is measured by the following metrics:

- **Accuracy**: Zero false positives. Valid `snake_case` stems and correct layer suffixes must never be flagged, while invalid ones must be caught 100% of the time.
- **Coverage**: Successfully checks Rust, Python, JavaScript, and TypeScript files as configured in the project settings.
- **Integration**: Correctly reports issues to the central CLI/MCP runner with precise location mappings.
- **Self-Audit**: The `naming-rules` codebase itself must adhere to naming rules (e.g., prefix and suffix alignment) and pass the naming checks.
```

---

## File: crates/naming-rules/src/agent_naming_orchestrator.rs

```rust
// PURPOSE: NamingOrchestrator — agent that orchestrates naming rule checks via contract ports
//
// Orchestrates 2 naming-related AES rules:
//   1. AES101: file suffix convention (e.g., `*_vo.rs` for value objects)
//   2. AES102: filename pattern matching (e.g., snake_case for Rust files)
//
// The orchestrator walks the target directory, filters to source files,
// then delegates to two checkers. Both checkers implement the same
// INamingCheckerProtocol trait but are configured with different rules.
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol;
use shared::naming_rules::contract_naming_checker_protocol::INamingCheckerProtocol;
use shared::naming_rules::contract_naming_filesystem_port::INamingFileSystemPort;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::taxonomy_common_vo::PatternList;
use std::sync::Arc;

/// Naming orchestrator — the agent layer for naming convention enforcement.
///
/// Dependencies:
///   - naming_convention_checker: AES102 — filename pattern matching
///   - suffix_prefix_checker: AES101 — file suffix convention checks
///   - analyzer: provides layer configuration and detection
///   - fs: filesystem access for walking directories
///   - ignored_patterns: paths to skip during file collection
pub struct NamingOrchestrator {
    naming_convention_checker: Arc<dyn INamingCheckerProtocol>,
    suffix_prefix_checker: Arc<dyn INamingCheckerProtocol>,
    analyzer: Arc<dyn INamingAnalyzerProtocol>,
    fs: Arc<dyn INamingFileSystemPort>,
    ignored_patterns: PatternList,
}

impl NamingOrchestrator {
    /// Constructor: builds the orchestrator with injected dependencies.
    /// Pre-processes ignored patterns from config (normalize paths).
    pub fn new(
        naming_convention_checker: Arc<dyn INamingCheckerProtocol>,
        suffix_prefix_checker: Arc<dyn INamingCheckerProtocol>,
        analyzer: Arc<dyn INamingAnalyzerProtocol>,
        fs: Arc<dyn INamingFileSystemPort>,
    ) -> Self {
        let config = analyzer.config();
        let ignored_patterns = PatternList {
            values: config
                .ignored_paths
                .values
                .iter()
                .map(|fp| {
                    fp.value
                        .trim_start_matches("./")
                        .trim_start_matches('/')
                        .trim_end_matches('/')
                        .to_string()
                })
                .collect(),
        };
        Self {
            naming_convention_checker,
            suffix_prefix_checker,
            analyzer,
            fs,
            ignored_patterns,
        }
    }

    /// Filter to source code files only (.rs, .py, .js, .ts, .jsx, .tsx).
    pub fn filter_source_files(files: &FilePathList) -> FilePathList {
        let source_exts = ["rs", "py", "js", "ts", "jsx", "tsx"];
        let filtered: Vec<FilePath> = files
            .values
            .iter()
            .filter(|f| match f.value.rsplit('.').next() {
                Some(ext) => source_exts.contains(&ext),
                None => false,
            })
            .cloned()
            .collect();
        FilePathList::new(filtered)
    }
}

#[async_trait]
impl INamingRunnerAggregate for NamingOrchestrator {
    /// Run both naming convention checks (AES101 + AES102) on the target.
    ///
    /// Orchestration flow:
    ///   1. Walk target directory (via fs port, skipping ignored paths)
    ///   2. Filter to source files only
    ///   3. Run naming_convention_checker.check_file_naming (AES102)
    ///   4. Run suffix_prefix_checker.check_domain_suffixes (AES101)
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        let mut results = LintResultList::new(Vec::new());
        let all_files = self.fs.walk(target, Some(&self.ignored_patterns)).await;
        let files = Self::filter_source_files(&all_files);
        let root_dir = target.clone();

        self.naming_convention_checker
            .check_file_naming(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;
        self.suffix_prefix_checker
            .check_domain_suffixes(self.analyzer.as_ref(), &files, &root_dir, &mut results)
            .await;

        results.values
    }

    fn name(&self) -> &str {
        "naming-rules"
    }
}
```

---

## File: crates/naming-rules/src/capabilities_naming_convention_checker.rs

```rust
// PURPOSE: NamingConventionChecker — Handles AES101 naming convention checks (lowercase, underscore, min 2 words)
use async_trait::async_trait;
use regex::Regex;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol;
use shared::naming_rules::contract_naming_checker_protocol::INamingCheckerProtocol;
use shared::naming_rules::taxonomy_naming_violation_vo::NamingViolation;
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

#[derive(Clone)]
pub struct NamingConventionChecker {}

impl Default for NamingConventionChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl NamingConventionChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn make_result(
        file: &str,
        code: &str,
        msg: impl Into<String>,
        sev: Severity,
    ) -> LintResult {
        let file_path = FilePath::new(file.to_string()).unwrap_or_default();
        LintResult {
            file: file_path,
            line: LineNumber::new(1),
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

    pub fn is_barrel_file(filename: &str) -> bool {
        matches!(
            filename,
            "__init__.py" | "mod.rs" | "index.ts" | "index.js" | "index.tsx" | "index.jsx"
        )
    }

    pub fn is_entry_point(filename: &str) -> bool {
        matches!(
            filename,
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

    /// Check file naming conventions (AES101: pattern validation — lowercase, underscore, min 2 words).
    pub fn check_file_naming(
        &self,
        file: &str,
        filename: &str,
        layer_name: &Option<String>,
        definition: Option<&LayerDefinition>,
        _config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        // AES layer prefixes (must match extract_layer_from_prefix in LayerDetectionAnalyzer)
        const LAYER_PREFIXES: &[&str] = &[
            "taxonomy_",
            "contract_",
            "capabilities_",
            "infrastructure_",
            "agent_",
            "surface_",
            "root_",
        ];

        // Step 1: Skip naming verification for barrel files (e.g. __init__.py, mod.rs, index.ts) and entry point files (e.g. main.rs, app.py).
        if Self::is_barrel_file(filename) || Self::is_entry_point(filename) {
            return;
        }

        // Step 2: Handle cases where the layer could not be determined.
        if layer_name.is_none() {
            let stem = filename.split('.').next().unwrap_or(filename);
            let actual_prefix = stem.split('_').next().unwrap_or_default().to_string();

            // Check if the file starts with an unrecognized/invalid prefix (not corresponding to a standard AES layer).
            if !actual_prefix.is_empty() && !LAYER_PREFIXES.iter().any(|p| stem.starts_with(p)) {
                let allowed: Vec<String> = LAYER_PREFIXES
                    .iter()
                    .map(|p| p.trim_end_matches('_').to_string())
                    .collect();
                violations.push(Self::make_result(
                    file,
                    "AES102",
                    NamingViolation::UnknownPrefix {
                        prefix: actual_prefix.clone(),
                        allowed,
                        reason: Some(LintMessage::new(format!(
                            "The prefix '{}' is not one of the {} recognised AES layer prefixes. \
                             Every source file must start with a valid layer prefix so it can be assigned to the correct architectural layer. \
                             Likely causes: typo in the prefix name, or the file is in the wrong directory.",
                            actual_prefix, LAYER_PREFIXES.len()
                        ))),
                    }
                    .to_string(),
                    Severity::HIGH,
                ));
                return;
            }

            // If the prefix is recognized or is empty, but there is no underscore or does not meet basic naming requirements.
            let stem = filename.split('.').next().unwrap_or(filename);
            violations.push(Self::make_result(
                file,
                "AES101",
                NamingViolation::NamingConvention {
                    min_words: 2,
                    separator: "_".to_string(),
                    reason: Some(LintMessage::new(format!(
                        "No architectural layer could be determined for '{}', and the stem '{}' does not follow \
                         the 'prefix_concept_suffix' naming pattern. Files must contain at least 2 underscore-separated \
                         lowercase words (e.g., 'capabilities_user_checker'). A valid layer prefix is the first word.",
                        file, stem
                    ))),
                }
                .to_string(),
                Severity::HIGH,
            ));
            return;
        }

        // Step 3: Retrieve the layer definition. If it does not exist, abort.
        let def = match definition {
            Some(d) => d,
            None => return,
        };

        // Step 4: Skip validation if the file name is explicitly listed in the layer's exception config.
        if def.exceptions.values.contains(&filename.to_string()) {
            return;
        }

        // Step 5: Validate the file stem pattern using a regular expression.
        // It must consist of lowercase letters and digits separated by underscores (e.g., prefix_concept_suffix).
        let stem = filename.split('.').next().unwrap_or_default();
        let naming_regex = r"^[a-z0-9]+(_[a-z0-9]+)+$";

        if let Ok(re) = Regex::new(naming_regex) {
            if !re.is_match(stem) {
                violations.push(Self::make_result(
                    file,
                    "AES101",
                    NamingViolation::NamingConvention {
                        min_words: 2,
                        separator: "_".to_string(),
                        reason: Some(LintMessage::new(format!(
                            "The stem '{}' does not match the required pattern 'prefix_concept_suffix'. \
                             Expected: lowercase alphanumeric words separated by underscores, minimum 2 words. \
                             Example valid names: 'capabilities_user_checker', 'infrastructure_db_adapter'. \
                             Issue: '{}' may have uppercase characters, wrong separator, or only 1 word.",
                            stem, stem
                        ))),
                    }
                    .to_string(),
                    Severity::HIGH,
                ));
            }
        }
    }
}

#[async_trait]
impl INamingCheckerProtocol for NamingConventionChecker {
    // Implement check_file_naming from INamingCheckerProtocol trait to perform checks on multiple files.
    async fn check_file_naming(
        &self,
        analyzer: &dyn INamingAnalyzerProtocol,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        // Step 1: Iterate over each file path in the checklist.
        for f in &files.values {
            let f_str = f.to_string();
            // Step 2: Extract the raw filename from the path.
            let filename = match f.rsplit('/').next() {
                Some(name) => name,
                None => &f_str,
            };
            // Step 3: Determine the architectural layer for the file.
            let layer = analyzer
                .detect_layer(f, root_dir)
                .map(|l| l.value().to_string());
            // Step 4: Fetch layer-specific definition properties.
            let def = layer.as_ref().and_then(|l| {
                analyzer
                    .layer_map()
                    .values
                    .get(&LayerNameVO::new(l.as_str()))
            });
            // Step 5: Execute the naming checker function.
            self.check_file_naming(
                &f_str,
                filename,
                &layer,
                def,
                analyzer.config(),
                &mut results.values,
            );
        }
    }

    async fn check_domain_suffixes(
        &self,
        _analyzer: &dyn INamingAnalyzerProtocol,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
        // No-op for naming convention checker
    }
}
```

---

## File: crates/naming-rules/src/capabilities_suffix_prefix_checker.rs

```rust
// PURPOSE: SuffixPrefixChecker — Handles AES102 suffix/prefix rules (allowed, forbidden, mandatory strict)
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol;
use shared::naming_rules::contract_naming_checker_protocol::INamingCheckerProtocol;
use shared::naming_rules::taxonomy_naming_violation_vo::NamingViolation;
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

#[derive(Clone)]
pub struct SuffixPrefixChecker {}

impl Default for SuffixPrefixChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl SuffixPrefixChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn make_result(
        file: &str,
        code: &str,
        msg: impl Into<String>,
        sev: Severity,
    ) -> LintResult {
        let file_path = FilePath::new(file.to_string()).unwrap_or_default();
        LintResult {
            file: file_path,
            line: LineNumber::new(1),
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

    pub fn is_barrel_file(filename: &str) -> bool {
        matches!(
            filename,
            "__init__.py" | "mod.rs" | "index.ts" | "index.js" | "index.tsx" | "index.jsx"
        )
    }

    pub fn is_entry_point(filename: &str) -> bool {
        matches!(
            filename,
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

    pub fn get_stem(filename: &str) -> Option<String> {
        if let Some(pos) = filename.rfind('.') {
            Some(filename[..pos].to_string())
        } else {
            Some(filename.to_string())
        }
    }

    pub fn get_suffix(stem: &str) -> Option<String> {
        stem.rfind('_').map(|pos| stem[pos + 1..].to_string())
    }

    /// Check domain suffix rules per layer (AES102: suffix/prefix rules).
    pub fn check_domain_suffixes(
        &self,
        file: &str,
        filename: &str,
        definition: Option<&LayerDefinition>,
        _layer_name: &Option<String>,
        violations: &mut Vec<LintResult>,
    ) {
        // Step 1: Skip checking for barrel files and system entry point files.
        if Self::is_barrel_file(filename) || Self::is_entry_point(filename) {
            return;
        }

        // Step 2: Retrieve the layer definition config.
        let def = match definition {
            Some(d) => d,
            None => return,
        };

        // Step 3: Skip validation if the filename matches an exception rule.
        if def.exceptions.values.contains(&filename.to_string()) {
            return;
        }

        // Step 4: Extract the file stem (name without extension) and get the suffix (word after the last underscore).
        let stem = match Self::get_stem(filename) {
            Some(s) => s,
            None => return,
        };

        let suffix = Self::get_suffix(&stem);

        // Step 5: Check if the suffix is explicitly forbidden for the current layer.
        if let Some(ref suf) = suffix {
            if def.naming.forbidden_suffix.values.contains(suf) {
                let layer_display = _layer_name.as_deref().unwrap_or("unknown").to_string();
                violations.push(Self::make_result(
                    file,
                    "AES102",
                    NamingViolation::SuffixForbidden {
                        layer_name: layer_display.clone(),
                        forbidden_suffix: suf.clone(),
                        reason: Some(LintMessage::new(format!(
                            "Suffix '{}' is not permitted in the '{}' layer. Each architectural layer allows only \
                             specific suffixes that match its role. The suffix '{}' belongs to a different layer's domain. \
                             Rename the file with an allowed suffix for '{}', or move it to the appropriate layer.",
                            suf, layer_display, suf, layer_display
                        ))),
                    }
                    .to_string(),
                    Severity::HIGH,
                ));
                return;
            }
        }

        // Step 6: If the layer configuration enforces a strict suffix policy, ensure the suffix matches the allowed list.
        if def.naming.suffix_policy.value == "strict" {
            let valid = match &suffix {
                Some(s) => def.naming.allowed_suffix.values.contains(s),
                None => false,
            };
            if !valid {
                let allowed_list = def.naming.allowed_suffix.values.clone();
                let layer_display = _layer_name.as_deref().unwrap_or("unknown").to_string();
                let suffix_display = suffix.as_deref().unwrap_or("(none)");
                violations.push(Self::make_result(
                    file,
                    "AES102",
                    NamingViolation::SuffixMismatch {
                        layer_name: layer_display.clone(),
                        allowed: allowed_list.clone(),
                        reason: Some(LintMessage::new(format!(
                            "Layer '{}' enforces a strict suffix policy, but the file uses suffix '{}'. \
                             Expected one of: {}. \
                             The suffix determines the file's architectural role — a missing or incorrect suffix \
                             breaks the layer-to-role mapping that automated checks depend on.",
                            layer_display,
                            suffix_display,
                            allowed_list.join(", ")
                        ))),
                    }
                    .to_string(),
                    Severity::HIGH,
                ));
            }
        }
    }
}

#[async_trait]
impl INamingCheckerProtocol for SuffixPrefixChecker {
    async fn check_file_naming(
        &self,
        _analyzer: &dyn INamingAnalyzerProtocol,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
        // No-op for suffix/prefix checker
    }

    // Implement check_domain_suffixes from INamingCheckerProtocol trait to perform checks on multiple files.
    async fn check_domain_suffixes(
        &self,
        analyzer: &dyn INamingAnalyzerProtocol,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        // Step 1: Iterate over each file path in the checklist.
        for f in &files.values {
            let f_str = f.to_string();
            // Step 2: Extract the raw filename from the path.
            let filename = f.rsplit('/').next().unwrap_or(&f_str);
            // Step 3: Determine the architectural layer for the file.
            let layer = analyzer
                .detect_layer(f, root_dir)
                .map(|l| l.value().to_string());
            // Step 4: Fetch layer-specific definition properties.
            let def = layer.as_ref().and_then(|l| {
                analyzer
                    .layer_map()
                    .values
                    .get(&LayerNameVO::new(l.as_str()))
            });
            // Step 5: Execute the suffix checker function.
            self.check_domain_suffixes(&f_str, filename, def, &layer, &mut results.values);
        }
    }
}
```

---

## File: crates/naming-rules/src/infrastructure_filesystem_adapter.rs

```rust
// PURPOSE: FileSystemAdapter — INamingFileSystemPort implementation custom-tailored for naming-rules (crawling/walking only)
use async_trait::async_trait;

use shared::common::taxonomy_path_utils_vo::PathUtils;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::naming_rules::contract_naming_filesystem_port::INamingFileSystemPort;
use shared::taxonomy_common_vo::PatternList;

pub struct OSFileSystemAdapter {}

impl OSFileSystemAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for OSFileSystemAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl INamingFileSystemPort for OSFileSystemAdapter {
    async fn walk(&self, path: &FilePath, ignored_patterns: Option<&PatternList>) -> FilePathList {
        let root = std::path::Path::new(&path.value);
        let ignored: Vec<String> = match ignored_patterns {
            Some(p) => p.values.clone(),
            None => Vec::new(),
        };
        let ignored_refs: Vec<&str> = ignored.iter().map(|s| s.as_str()).collect();
        let results = PathUtils::walk_recursive(root, &ignored_refs);
        FilePathList {
            values: results
                .into_iter()
                .filter_map(|p| FilePath::new(p.to_string_lossy().to_string()).ok())
                .collect(),
        }
    }
}
```

---

## File: crates/naming-rules/src/lib.rs

```rust
// PURPOSE: Module declarations for naming-rules (checkers, orchestrator, container)
pub mod agent_naming_orchestrator;
pub mod capabilities_naming_convention_checker;
pub mod capabilities_suffix_prefix_checker;
pub mod infrastructure_filesystem_adapter;
pub mod root_naming_rules_container;
```

---

## File: crates/naming-rules/src/root_naming_rules_container.rs

```rust
// PURPOSE: NamingContainer — wiring for naming-rules feature (root layer, wiring only)
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_analyzer_protocol::INamingAnalyzerProtocol;
use shared::naming_rules::contract_naming_checker_protocol::INamingCheckerProtocol;
use shared::naming_rules::contract_naming_filesystem_port::INamingFileSystemPort;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::taxonomy_definition_vo::LayerMapVO;
use shared::taxonomy_layer_vo::LayerNameVO;
use std::sync::Arc;

pub struct NamingContainer {
    naming_convention_checker: Arc<dyn INamingCheckerProtocol>,
    suffix_prefix_checker: Arc<dyn INamingCheckerProtocol>,
    analyzer: Arc<dyn INamingAnalyzerProtocol>,
    fs: Arc<dyn INamingFileSystemPort>,
}

impl NamingContainer {
    pub fn new(analyzer: Arc<dyn INamingAnalyzerProtocol>) -> Self {
        let naming_convention_checker: Arc<dyn INamingCheckerProtocol> =
            Arc::new(crate::capabilities_naming_convention_checker::NamingConventionChecker::new());
        let suffix_prefix_checker: Arc<dyn INamingCheckerProtocol> =
            Arc::new(crate::capabilities_suffix_prefix_checker::SuffixPrefixChecker::new());
        let fs: Arc<dyn INamingFileSystemPort> =
            Arc::new(crate::infrastructure_filesystem_adapter::OSFileSystemAdapter::new());
        Self {
            naming_convention_checker,
            suffix_prefix_checker,
            analyzer,
            fs,
        }
    }

    pub fn new_default() -> Self {
        Self::new(Arc::new(DefaultNamingAnalyzer))
    }

    pub fn naming_convention_checker(&self) -> &Arc<dyn INamingCheckerProtocol> {
        &self.naming_convention_checker
    }

    pub fn suffix_prefix_checker(&self) -> &Arc<dyn INamingCheckerProtocol> {
        &self.suffix_prefix_checker
    }

    pub fn analyzer(&self) -> Arc<dyn INamingAnalyzerProtocol> {
        self.analyzer.clone()
    }

    pub fn orchestrator(&self) -> Arc<dyn INamingRunnerAggregate> {
        Arc::new(crate::agent_naming_orchestrator::NamingOrchestrator::new(
            self.naming_convention_checker.clone(),
            self.suffix_prefix_checker.clone(),
            self.analyzer.clone(),
            self.fs.clone(),
        ))
    }
}

struct DefaultNamingAnalyzer;
impl INamingAnalyzerProtocol for DefaultNamingAnalyzer {
    fn config(&self) -> &ArchitectureConfig {
        static CONFIG: std::sync::OnceLock<ArchitectureConfig> = std::sync::OnceLock::new();
        CONFIG.get_or_init(ArchitectureConfig::default)
    }
    fn layer_map(&self) -> &LayerMapVO {
        static MAP: std::sync::OnceLock<LayerMapVO> = std::sync::OnceLock::new();
        MAP.get_or_init(|| LayerMapVO::new(std::collections::HashMap::new()))
    }
    fn detect_layer(&self, _f: &FilePath, _root_dir: &FilePath) -> Option<LayerNameVO> {
        None
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

## File: crates/shared/src/common/taxonomy_path_utils_vo.rs

```rust
pub struct PathUtils;

impl PathUtils {
    /// Walk a directory recursively, collecting files while skipping ignored patterns.
    pub fn walk_recursive(dir: &std::path::Path, ignored: &[&str]) -> Vec<std::path::PathBuf> {
        use std::fs;

        let mut results = Vec::new();

        if !dir.is_dir() {
            if dir.is_file() {
                if let Some(name_str) = dir.file_name().and_then(|s| s.to_str()) {
                    if !ignored.contains(&name_str) {
                        results.push(dir.to_path_buf());
                    }
                }
            }
            return results;
        }

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if let Some(name_str) = entry.file_name().to_str() {
                    if ignored.contains(&name_str) {
                        continue;
                    }

                    if path.is_dir() {
                        results.extend(Self::walk_recursive(&path, ignored));
                    } else {
                        results.push(path);
                    }
                }
            }
        }

        results
    }

    /// Convenience wrapper used by OSFileSystemAdapter and workspace helpers.
    pub fn collect_paths(start: &str, ignored: &[&str]) -> Vec<std::path::PathBuf> {
        let root = std::path::Path::new(start);
        Self::walk_recursive(root, ignored)
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

## File: crates/shared/src/naming-rules/contract_naming_checker_protocol.rs

```rust
// PURPOSE: INamingCheckerProtocol — protocol trait for naming check capabilities
use super::contract_naming_analyzer_protocol::INamingAnalyzerProtocol;
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use async_trait::async_trait;

#[async_trait]
pub trait INamingCheckerProtocol: Send + Sync {
    async fn check_file_naming(
        &self,
        analyzer: &dyn INamingAnalyzerProtocol,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    async fn check_domain_suffixes(
        &self,
        analyzer: &dyn INamingAnalyzerProtocol,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
```

---

## File: crates/shared/src/naming-rules/contract_naming_filesystem_port.rs

```rust
// PURPOSE: INamingFileSystemPort — Local contract trait for naming-rules filesystem operations
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use async_trait::async_trait;

#[async_trait]
pub trait INamingFileSystemPort: Send + Sync {
    async fn walk(&self, path: &FilePath, ignored_patterns: Option<&PatternList>) -> FilePathList;
}
```

---

## File: crates/shared/src/naming-rules/contract_naming_runner_aggregate.rs

```rust
// PURPOSE: INamingRunnerAggregate — contract for naming-rules feature orchestrator
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait INamingRunnerAggregate: Send + Sync {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
    fn name(&self) -> &str;
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

## File: crates/shared/src/naming-rules/taxonomy_naming_rule_vo.rs

```rust
// PURPOSE: NamingRuleVO — value object containing naming convention and suffix policy rule definitions
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::PatternList;
use crate::naming_rules::taxonomy_suffix_vo::SuffixPolicyVO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct NamingRuleVO {
    #[serde(default)]
    pub naming_convention: BooleanVO,
    #[serde(default)]
    pub suffix_policy: SuffixPolicyVO,
    #[serde(default, alias = "allowed_suffix")]
    pub allowed_suffix: PatternList,
    #[serde(default, alias = "forbidden_suffix")]
    pub forbidden_suffix: PatternList,
}
```

---

## File: crates/shared/src/naming-rules/taxonomy_naming_violation_vo.rs

```rust
// PURPOSE: NamingViolation — AES101/AES102 violation messages for naming rules domain
use crate::common::taxonomy_message_vo::LintMessage;
use std::fmt;

#[derive(Debug, Clone)]
pub enum NamingViolation {
    /// AES101 — filename doesn't follow prefix_concept_suffix pattern
    /// Min 2 words separated by underscore (e.g., prefix_suffix).
    NamingConvention {
        min_words: usize,
        separator: String,
        reason: Option<LintMessage>,
    },
    /// AES102 — filename prefix is not one of the recognised layer prefixes
    UnknownPrefix {
        prefix: String,
        allowed: Vec<String>,
        reason: Option<LintMessage>,
    },
    /// AES102 — suffix is explicitly forbidden for this layer
    /// Carries the layer name and the actual suffix used.
    SuffixForbidden {
        layer_name: String,
        forbidden_suffix: String,
        reason: Option<LintMessage>,
    },
    /// AES102 — strict suffix policy violated (missing required suffix)
    /// Carries the layer name and allowed suffixes from config for dynamic messages.
    SuffixMismatch {
        layer_name: String,
        allowed: Vec<String>,
        reason: Option<LintMessage>,
    },
}

impl fmt::Display for NamingViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NamingConvention {
                min_words,
                separator,
                reason,
            } => {
                let why = Option::unwrap_or_else(reason.as_ref().map(|r| r.to_string()), || {
                    format!("The AES layer naming convention requires filenames to contain at least {} words separated by '{}' (e.g., prefix{}suffix). Each word must be lowercase alphanumeric. This pattern ensures every file's architectural layer (prefix) and role (suffix) is immediately identifiable — both for human readers and automated tooling.", min_words, separator, separator)
                });
                write!(
                    f,
                    "AES101 NAMING_CONVENTION: Filename must contain at least {} words separated by '{}'.\n\
                    WHY? {}\n\
                    FIX: Rename to follow prefix{}suffix pattern (e.g., capabilities{}user_checker.rs).",
                    min_words, separator, why, separator, separator
                )
            }
            Self::UnknownPrefix {
                prefix,
                allowed,
                reason,
            } => {
                let allowed_str = allowed.join(", ");
                let default_why = format!(
                    "Every source file must begin with one of the recognised layer prefixes ({}) so that its architectural layer can be determined automatically. The prefix '{}' does not correspond to any known layer.",
                    allowed_str, prefix
                );
                let why = Option::unwrap_or(reason.as_ref().map(|r| r.to_string()), default_why);
                write!(
                    f,
                    "AES102 UNKNOWN_PREFIX: File uses prefix '{}' which is not a recognised layer.\n\
                    WHY? {}\n\
                    FIX: Rename to start with one of the allowed prefixes: {}.",
                    prefix, why, allowed_str
                )
            }
            Self::SuffixForbidden {
                layer_name,
                forbidden_suffix,
                reason,
            } => {
                let default_why = format!(
                    "The suffix '{}' belongs to a different architectural role and is not allowed in the '{}' layer. Mixing role suffixes across layers breaks the strict layer-to-suffix mapping that tooling depends on for automatic validation.",
                    forbidden_suffix, layer_name
                );
                let why = Option::unwrap_or(reason.as_ref().map(|r| r.to_string()), default_why);
                write!(
                    f,
                    "AES102 SUFFIX_FORBIDDEN: File in layer '{}' uses suffix '{}' which is forbidden.\n\
                    WHY? {}\n\
                    FIX: Rename the file to use an allowed suffix or move it to the correct layer.",
                    layer_name, forbidden_suffix, why
                )
            }
            Self::SuffixMismatch {
                layer_name,
                allowed,
                reason,
            } => {
                let allowed_str = allowed.join(", ");
                let default_why = format!(
                    "Files in the '{}' layer must end with a recognised role suffix ({}) so that their architectural intent is clear and automated boundary checks can verify that each file belongs exactly where it is. A missing or unrecognised suffix bypasses this safeguard.",
                    layer_name, allowed_str
                );
                let why = Option::unwrap_or(reason.as_ref().map(|r| r.to_string()), default_why);
                write!(
                    f,
                    "AES102 SUFFIX_MISMATCH: File in layer '{}' is missing a required strict suffix.\n\
                    WHY? {}\n\
                    FIX: Rename the file to include one of the allowed suffixes: {}.",
                    layer_name, why, allowed_str
                )
            }
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
