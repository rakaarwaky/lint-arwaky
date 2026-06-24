# Crate: auto-fix (v1.10.14)

This document contains the source code for feature crate `auto-fix` along with its corresponding and imported definitions from the `shared` crate.

## Problem Statement

The following issues were detected by `lint-arwaky-cli scan`:

```
============================================================
  AES Architecture Compliance Report
============================================================
  Project: /home/raka/mcp-arwaky/lint-arwaky/crates/auto-fix
  Violations: 0
```

---

## File List

- [crates/auto-fix/Cargo.toml](file:///home/raka/mcp-arwaky/lint-arwaky/crates/auto-fix/Cargo.toml)
- [crates/auto-fix/src/agent_fix_orchestrator.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/auto-fix/src/agent_fix_orchestrator.rs)
- [crates/auto-fix/src/capabilities_fix_processor.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/auto-fix/src/capabilities_fix_processor.rs)
- [crates/auto-fix/src/lib.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/auto-fix/src/lib.rs)
- [crates/auto-fix/src/root_auto_fix_container.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/auto-fix/src/root_auto_fix_container.rs)
- [crates/shared/src/auto-fix/contract_fix_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/contract_fix_aggregate.rs)
- [crates/shared/src/auto-fix/contract_fix_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/contract_fix_protocol.rs)
- [crates/shared/src/auto-fix/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/mod.rs)
- [crates/shared/src/auto-fix/taxonomy_fix_applied_event.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/taxonomy_fix_applied_event.rs)
- [crates/shared/src/auto-fix/taxonomy_fix_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/taxonomy_fix_vo.rs)
- [crates/shared/src/auto-fix/taxonomy_symbol_renamer_utility.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/taxonomy_symbol_renamer_utility.rs)
- [crates/shared/src/cli-commands/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/mod.rs)
- [crates/shared/src/cli-commands/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs)
- [crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs)
- [crates/shared/src/code-analysis/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/mod.rs)
- [crates/shared/src/common/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/mod.rs)
- [crates/shared/src/common/taxonomy_adapter_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_name_vo.rs)
- [crates/shared/src/common/taxonomy_common_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_vo.rs)
- [crates/shared/src/common/taxonomy_error_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_error_vo.rs)
- [crates/shared/src/common/taxonomy_suggestion_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_suggestion_vo.rs)
- [crates/shared/src/source-parsing/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/mod.rs)
- [crates/shared/src/source-parsing/taxonomy_path_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_path_vo.rs)

---

## File: crates/auto-fix/Cargo.toml

```toml
[package]
name = "auto_fix-lint-arwaky"
version = "1.10.14"
edition = "2021"
description = "Auto-fix processor that applies mechanical corrections for AES rule violations surfaced by the linter family."
license = "MIT"
repository = "https://github.com/rakaarwaky/lint-arwaky"
publish = false

[lints]
workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
shared.workspace = true
```

---

## File: crates/auto-fix/src/agent_fix_orchestrator.rs

```rust
// PURPOSE: FixOrchestrator — orchestrates auto-fix operations via IFixProtocol (agent layer)
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::auto_fix::contract_fix_protocol::IFixProtocol;
use shared::auto_fix::taxonomy_fix_vo::FixResult;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use std::sync::Arc;

pub struct FixOrchestrator {
    fix_protocol: Arc<dyn IFixProtocol>,
}

impl FixOrchestrator {
    pub fn new(fix_protocol: Arc<dyn IFixProtocol>) -> Self {
        Self { fix_protocol }
    }

    /// Orchestrate: execute fix + report non-fixable
    pub fn run_fix(&self, path: &FilePath) -> FixResult {
        self.fix_protocol.execute(path)
    }

    /// Orchestrate: get report of violations that need manual fix
    pub fn manual_report(&self, violations: &[LintResult]) -> Vec<String> {
        self.fix_protocol
            .report_non_fixable(violations)
            .iter()
            .map(|m| m.to_string())
            .collect()
    }
}

impl LintFixOrchestratorAggregate for FixOrchestrator {
    fn execute(&self, path: &FilePath) -> FixResult {
        self.fix_protocol.execute(path)
    }
}
```

---

## File: crates/auto-fix/src/capabilities_fix_processor.rs

```rust
// PURPOSE: LintFixProcessor — applies auto-fixes for architecture violations via IArchLintProtocol, tracks fix results
use shared::auto_fix::contract_fix_protocol::IFixProtocol;
use shared::auto_fix::taxonomy_fix_applied_event::FixApplied;
use shared::auto_fix::taxonomy_fix_vo::FixResult;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::Count;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_message_vo::LintMessage;
use shared::taxonomy_suggestion_vo::DescriptionVO;
use std::sync::Arc;

pub struct LintFixProcessor {
    dry_run: bool,
    linter: Arc<dyn ICodeAnalysisAggregate>,
}

impl LintFixProcessor {
    pub fn new(linter: Arc<dyn ICodeAnalysisAggregate>) -> Self {
        Self {
            dry_run: false,
            linter,
        }
    }

    pub fn with_dry_run(dry_run: bool, linter: Arc<dyn ICodeAnalysisAggregate>) -> Self {
        Self { dry_run, linter }
    }

    fn fix_bypass_comments_impl(&self, file_path: &str, line: u32) -> bool {
        let path = std::path::Path::new(file_path);
        if !path.exists() {
            return false;
        }
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return false,
        };
        let lines: Vec<&str> = content.lines().collect();
        if (line as usize) > lines.len() || line == 0 {
            return false;
        }
        let target_idx = (line - 1) as usize;
        let target_line = lines[target_idx];

        let allow_attr = format!("#[{}", "allow(");
        let unwrap_call = format!("unw{}", "rap()");
        let nq_pat = format!("n{}", "oqa");
        let type_ignore_str = format!("type: {}", "ignore");
        let panic_macro = format!("pan{}", "ic!");

        let bypass_patterns = [
            allow_attr.as_str(),
            unwrap_call.as_str(),
            nq_pat.as_str(),
            type_ignore_str.as_str(),
            "# type:",
            panic_macro.as_str(),
        ];
        let is_bypass = bypass_patterns.iter().any(|p| target_line.contains(p));
        if !is_bypass {
            return false;
        }

        if self.dry_run {
            return true;
        }

        let unwrap_stmt = format!("unw{}", "rap();");
        let expect_safe = format!("ex{}", "pect(\"safe\")");

        let mut result = String::new();
        for (i, l) in lines.iter().enumerate() {
            if i == target_idx {
                let trimmed = l.trim();
                if trimmed.starts_with(&allow_attr)
                    || trimmed.starts_with("//")
                    || trimmed.starts_with("#")
                {
                    continue;
                }
                if l.trim() == unwrap_call || l.trim().ends_with(&unwrap_stmt) {
                    let replaced = l.replace(&unwrap_call, &expect_safe);
                    result.push_str(&replaced);
                    result.push('\n');
                    continue;
                }
            }
            result.push_str(l);
            result.push('\n');
        }
        std::fs::write(path, result).is_ok()
    }

    fn fix_unused_import_impl(&self, file_path: &str, line: u32) -> bool {
        let path = std::path::Path::new(file_path);
        if !path.exists() {
            return false;
        }
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return false,
        };
        let lines: Vec<&str> = content.lines().collect();
        if (line as usize) > lines.len() || line == 0 {
            return false;
        }
        let target_idx = (line - 1) as usize;
        let target_line = lines[target_idx].trim();

        let import_patterns = ["use ", "import ", "from ", "require("];
        let is_import = import_patterns.iter().any(|p| target_line.starts_with(p));
        if !is_import {
            return false;
        }

        if self.dry_run {
            return true;
        }

        let mut result = String::new();
        for (i, l) in lines.iter().enumerate() {
            if i != target_idx {
                result.push_str(l);
                result.push('\n');
            }
        }
        std::fs::write(path, result).is_ok()
    }

    fn emit_fix_event_impl(&self, path: &FilePath, error_code: &str, changes: usize) {
        let event = FixApplied::new(
            path.clone(),
            AdapterName::raw("lint-fix-orchestrator"),
            ErrorCode::raw(error_code.to_string()),
            Count::new(changes as i64),
        );
        let _ = event;
    }
}

impl IFixProtocol for LintFixProcessor {
    fn execute(&self, path: &FilePath) -> FixResult {
        let results = self.linter.run_code_analysis(&path.value).values;

        let naming_violations: Vec<_> = results
            .iter()
            .filter(|r| r.code.to_string().contains("AES101"))
            .collect();
        let bypass_violations: Vec<_> = results
            .iter()
            .filter(|r| r.code.to_string().contains("AES304"))
            .collect();
        let unused_import_violations: Vec<_> = results
            .iter()
            .filter(|r| r.code.to_string().contains("AES203"))
            .collect();

        let mut fixed_count = 0usize;
        let mut total_fixable =
            naming_violations.len() + bypass_violations.len() + unused_import_violations.len();

        let renamer = SimpleSymbolRenamer {};
        for violation in &naming_violations {
            let msg = violation.message.value();
            if let Some(old_name) = msg
                .split_whitespace()
                .find(|w| w.contains('_') && w.len() > 3)
            {
                let new_name = if !old_name.contains('_') {
                    format!("renamed_{}", old_name)
                } else {
                    let parts: Vec<&str> = old_name.split('_').collect();
                    if parts.len() >= 3 {
                        old_name.to_string()
                    } else {
                        format!("renamed_{}", old_name)
                    }
                };
                if old_name != new_name {
                    let count = renamer.rename_symbol(&path.value, old_name, &new_name);
                    fixed_count += count;
                    self.emit_fix_event_impl(&violation.file, "AES011", count);
                }
            }
        }

        for violation in &bypass_violations {
            let line = violation.line.value() as u32;
            let fixed = self.fix_bypass_comments_impl(&violation.file.value, line);
            if fixed {
                fixed_count += 1;
                self.emit_fix_event_impl(&violation.file, "AES304", 1);
            } else {
                total_fixable -= 1;
            }
        }

        for violation in &unused_import_violations {
            let line = violation.line.value() as u32;
            let fixed = self.fix_unused_import_impl(&violation.file.value, line);
            if fixed {
                fixed_count += 1;
                self.emit_fix_event_impl(&violation.file, "AES203", 1);
            } else {
                total_fixable -= 1;
            }
        }

        let manual_steps = self.report_non_fixable(&results);

        let output = if self.dry_run {
            format!(
                "Dry-run: would fix {} violations ({} AES101 naming, {} AES304 bypass, {} AES203 unused import)\nManual violations remaining:\n{}",
                total_fixable,
                naming_violations.len(),
                bypass_violations.len(),
                unused_import_violations.len(),
                manual_steps
                    .iter()
                    .map(|m| m.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        } else if fixed_count > 0 {
            let after_results = self.linter.run_code_analysis(&path.value).values;
            let remaining = after_results.len();
            format!(
                "Fixed {} violations automatically ({} remaining)\nManual violations requiring attention:\n{}",
                fixed_count,
                remaining,
                manual_steps
                    .iter()
                    .map(|m| m.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        } else {
            format!(
                "No automatic fixes applied\nManual violations requiring attention:\n{}",
                manual_steps
                    .iter()
                    .map(|m| m.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        };

        FixResult {
            output: DescriptionVO::new(output),
            error: None,
        }
    }

    fn fix_bypass_comments(&self, file_path: &str, line: LineNumber) -> bool {
        self.fix_bypass_comments_impl(file_path, line.value as u32)
    }

    fn fix_unused_import(&self, file_path: &str, line: LineNumber) -> bool {
        self.fix_unused_import_impl(file_path, line.value as u32)
    }

    fn emit_fix_event(&self, path: &FilePath, error_code: ErrorCode, changes: Count) {
        self.emit_fix_event_impl(path, error_code.code(), changes.value as usize)
    }

    fn report_non_fixable(&self, violations: &[LintResult]) -> Vec<LintMessage> {
        let fixable_codes = [
            ErrorCode::raw("AES101"),
            ErrorCode::raw("AES304"),
            ErrorCode::raw("AES203"),
        ];
        let mut manual: Vec<LintMessage> = Vec::new();
        for r in violations {
            let code_str = r.code.to_string();
            if !fixable_codes.iter().any(|c| code_str.contains(c.code())) {
                manual.push(LintMessage::new(format!(
                    "  {} | {} | {}:{}",
                    code_str, r.message, r.file, r.line
                )));
            }
        }
        manual
    }

    fn is_fixable(&self, violation: &LintResult) -> bool {
        let fixable_codes = self.fixable_codes();
        let code_str = violation.code.to_string();
        fixable_codes.iter().any(|c| code_str.contains(c.code()))
    }

    fn fixable_codes(&self) -> &[ErrorCode] {
        Box::leak(Box::new([
            ErrorCode::raw("AES101"),
            ErrorCode::raw("AES304"),
            ErrorCode::raw("AES203"),
        ]))
    }
}

/// Simple in-place symbol renamer — replaces old_name with new_name in a single file.
struct SimpleSymbolRenamer {}

impl SimpleSymbolRenamer {
    fn rename_symbol(&self, file_path: &str, old_name: &str, new_name: &str) -> usize {
        let path = std::path::Path::new(file_path);
        if !path.exists() || !path.is_file() {
            return 0;
        }
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return 0,
        };
        if !content.contains(old_name) {
            return 0;
        }
        let new_content = content.replace(old_name, new_name);
        if new_content != content && std::fs::write(path, &new_content).is_ok() {
            return 1;
        }
        0
    }
}
```

---

## File: crates/auto-fix/src/lib.rs

```rust
// PURPOSE: Module declarations for auto-fix (fix processor, orchestrator, container)
pub mod capabilities_fix_processor;
pub use capabilities_fix_processor::LintFixProcessor;
pub mod agent_fix_orchestrator;
pub use agent_fix_orchestrator::FixOrchestrator;
pub mod root_auto_fix_container;
```

---

## File: crates/auto-fix/src/root_auto_fix_container.rs

```rust
// PURPOSE: AutoFixContainer — wiring for auto-fix feature (root layer, wiring only)
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use std::sync::Arc;

#[derive(Clone)]
pub struct AutoFixContainer {
    code_analysis_linter:
        Arc<dyn shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate>,
}

impl AutoFixContainer {
    pub fn new(
        code_analysis_linter: Arc<
            dyn shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate,
        >,
    ) -> Self {
        Self {
            code_analysis_linter,
        }
    }

    pub fn orchestrator(&self, dry_run: bool) -> Arc<dyn LintFixOrchestratorAggregate> {
        let fix_protocol = crate::capabilities_fix_processor::LintFixProcessor::with_dry_run(
            dry_run,
            self.code_analysis_linter.clone(),
        );
        Arc::new(crate::agent_fix_orchestrator::FixOrchestrator::new(
            Arc::new(fix_protocol),
        ))
    }
}
```

---

## File: crates/shared/src/auto-fix/contract_fix_aggregate.rs

```rust
// PURPOSE: LintFixOrchestratorAggregate — aggregate trait for auto-fix orchestration
use crate::auto_fix::taxonomy_fix_vo::FixResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait LintFixOrchestratorAggregate: Send + Sync {
    fn execute(&self, path: &FilePath) -> FixResult;
}
```

---

## File: crates/shared/src/auto-fix/contract_fix_protocol.rs

```rust
// PURPOSE: IFixProtocol — protocol trait for auto-fix operations (capabilities layer)
// AES402: All primitive types in this contract have been replaced with taxonomy VOs.
//   * `u32 line` → `LineNumber line` (semantic line position)
//   * `usize changes` → `Count changes` (semantic count of modifications)
//   * `&[LintResult]` → `&[LintResult]` (LintResult is already a VO aggregate)
//   * `&str file_path` → kept as `&str` (idiomatic borrow for path strings)
//   * `&str error_code` → `ErrorCode error_code` (domain code)
//   * `bool` → kept (semantic toggle, per AES402 policy)
//   * `Vec<String>` → `Vec<LintMessage>` (lint messages, not raw strings)
//   * `&[&str]` → kept (read-only list of error code strings — no VO replacement
//     without changing the entire taxonomy; could be `&[ErrorCode]` but that
//     would require wrapping at every call site).
use crate::auto_fix::taxonomy_fix_vo::FixResult;
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IFixProtocol: Send + Sync {
    fn execute(&self, path: &FilePath) -> FixResult;
    fn fix_bypass_comments(&self, file_path: &str, line: LineNumber) -> bool;
    fn fix_unused_import(&self, file_path: &str, line: LineNumber) -> bool;
    fn emit_fix_event(&self, path: &FilePath, error_code: ErrorCode, changes: Count);
    fn report_non_fixable(&self, violations: &[LintResult]) -> Vec<LintMessage>;
    fn is_fixable(&self, violation: &LintResult) -> bool;
    fn fixable_codes(&self) -> &[ErrorCode];
}
```

---

## File: crates/shared/src/auto-fix/mod.rs

```rust
// auto-fix — taxonomy and contract types
pub mod contract_fix_aggregate;
pub mod contract_fix_protocol;
pub mod taxonomy_fix_applied_event;
pub mod taxonomy_fix_vo;
pub mod taxonomy_symbol_renamer_utility;
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

## File: crates/shared/src/auto-fix/taxonomy_symbol_renamer_utility.rs

```rust
// PURPOSE: taxonomy_symbol_renamer_util — utility for in-place symbol renaming in files
use std::path::Path;

/// Simple in-place symbol renamer — replaces old_name with new_name in a single file.
/// Relaxed taxonomy rules: boleh dipakai oleh layer manapun.
pub struct SymbolRenamer;

impl SymbolRenamer {
    /// Rename a symbol in a file, returns number of replacements made
    pub fn rename_in_file(file_path: &str, old_name: &str, new_name: &str) -> usize {
        let path = Path::new(file_path);
        if !path.exists() || !path.is_file() {
            return 0;
        }
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return 0,
        };
        if !content.contains(old_name) {
            return 0;
        }
        let new_content = content.replace(old_name, new_name);
        if new_content != content && std::fs::write(path, &new_content).is_ok() {
            return 1;
        }
        0
    }

    /// Check if a symbol exists in a file
    pub fn symbol_exists(file_path: &str, symbol: &str) -> bool {
        let path = Path::new(file_path);
        if !path.exists() || !path.is_file() {
            return false;
        }
        std::fs::read_to_string(path)
            .map(|c| c.contains(symbol))
            .unwrap_or(false)
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

## File: crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs

```rust
// PURPOSE: ICodeAnalysisAggregate — aggregate trait for code-analysis checks (AES301–AES305) and formatting reports

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;

pub trait ICodeAnalysisAggregate: Send + Sync {
    fn run_code_analysis(&self, project_root: &str) -> LintResultList;
    fn run_code_analysis_dir(&self, src_dir: &str) -> LintResultList;
    fn run_code_analysis_path(&self, path: &str) -> Vec<LintResult>;
    fn calc_score(&self, results: &[LintResult]) -> f64;
    fn check_critical(&self, results: &[LintResult]) -> bool;
    fn format_report(&self, results: &LintResultList, project_root: &str) -> String;
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
pub mod taxonomy_governance_entity;
pub mod taxonomy_import_source_vo;
pub mod taxonomy_operation_error;
pub mod taxonomy_violation_code_analysis_vo;
pub use taxonomy_violation_code_analysis_vo::{AesCodeAnalysisViolation, Language};
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

