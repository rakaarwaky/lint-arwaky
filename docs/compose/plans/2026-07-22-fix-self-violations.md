# Fix Self-Violations Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use compose:subagent (recommended) or compose:execute to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix all 116 AES architecture violations in the lint-arwaky codebase to achieve 0 violations on self-lint check.

**Architecture:** Systematic approach to fix violations by category: compilation errors first, then architecture violations (import rules, dead inheritance, bypass comments, file size, naming, code duplication), and finally clippy/formatting issues.

**Tech Stack:** Rust, AES architecture rules, cargo clippy, cargo fmt

## Global Constraints

- All changes must pass `cargo test --workspace`
- All changes must pass `cargo clippy --all-targets -- -D warnings`
- All changes must pass `cargo fmt --all`
- Self-lint check must report 0 violations: `cargo run --bin lint-arwaky-cli -- check .`
- Never use bypass comments (`#[allow(...)]`, `unwrap()`, `expect()`) to suppress warnings
- Fix root causes, not symptoms

---

## Violation Summary

| Category | Count | Priority |
|----------|-------|----------|
| AES201 FORBIDDEN_IMPORT | 17 | HIGH |
| AES302 FILE_TOO_SHORT | 22 | MEDIUM |
| AES305 DEAD_INHERITANCE | 22 | MEDIUM |
| AES204 IMPORT_INTENT | 10 | MEDIUM |
| AES304 BYPASS_COMMENT | 7 | HIGH |
| AES402 CONTRACT_PRIMITIVE | 3 | HIGH |
| AES202 MANDATORY_IMPORT | 3 | HIGH |
| AES304 UNIMPLEMENTED | 3 | HIGH |
| AES305 CODE_DUPLICATION | 2 | MEDIUM |
| AES203 UNUSED_IMPORT | 2 | LOW |
| AES102 SUFFIX_MISMATCH | 1 | HIGH |
| AES304 UNWRAP_EXPECT | 1 | HIGH |
| AES304 TODO | 1 | HIGH |
| AES304 PANIC | 1 | HIGH |
| AES405 AGENT_ROLE | 1 | MEDIUM |
| AES403 CAPABILITY_ROLE | 1 | MEDIUM |
| dead_code | 1 | LOW |
| clippy::clone_on_copy | 1 | LOW |
| clippy::too_many_arguments | 1 | LOW |
| formatting | 1 | LOW |

---

## Task 1: Fix Compilation Errors and Basic Clippy Issues

**Covers:** Compilation fixes, dead_code, clone_on_copy

**Files:**
- Modify: `crates/cli-commands/src/agent_analysis_pipeline_orchestrator.rs`

**Interfaces:**
- Consumes: None
- Produces: Compiling codebase with no errors

- [ ] **Step 1: Fix dead_code warning for `member` field**

The `member` field in `AnalysisPipelineOrchestrator` is never read. Either use it or remove it.

```rust
// In crates/cli-commands/src/agent_analysis_pipeline_orchestrator.rs
// Remove the unused field from the struct definition (line 37)
pub struct AnalysisPipelineOrchestrator {
    deps: shared::cli_commands::taxonomy_lint_dependencies_vo::LintDependencies,
    format: Format,
    filter: Option<String>,
    // member: Option<String>,  // REMOVED - unused field
}
```

Also update the `new` constructor to remove the `member` parameter if it exists, or just remove the field initialization.

- [ ] **Step 2: Fix clone_on_copy warning**

`Format` implements `Copy`, so use copy instead of clone:

```rust
// In crates/cli-commands/src/agent_analysis_pipeline_orchestrator.rs
// Line 269: Change from
format: self.format.clone(),
// To
format: self.format,
```

- [ ] **Step 3: Run tests to verify**

Run: `cargo test --workspace`
Expected: All tests pass

- [ ] **Step 4: Run clippy to verify**

Run: `cargo clippy --all-targets -- -D warnings`
Expected: No warnings

- [ ] **Step 5: Commit**

```bash
git add crates/cli-commands/src/agent_analysis_pipeline_orchestrator.rs
git commit -m "fix: remove dead_code and clone_on_copy warnings in pipeline orchestrator"
```

---

## Task 2: Fix AES201 FORBIDDEN_IMPORT Violations (17 violations)

**Covers:** AES201 FORBIDDEN_IMPORT

**Files:**
- Modify: `crates/shared/src/cli-commands/taxonomy_lint_dependencies_vo.rs` (14 violations)
- Modify: `crates/tui/src/capabilities_lint_executor.rs` (3 violations)

**Interfaces:**
- Consumes: None
- Produces: Taxonomy layer files with no forbidden imports

- [ ] **Step 1: Fix taxonomy_lint_dependencies_vo.rs**

This file is in the taxonomy layer but imports from contract layer. Taxonomy must be pure.

Read the file to understand what it imports from contract layer, then refactor to remove those imports. The taxonomy layer should only contain VOs, entities, errors, events, and constants.

Options:
1. Move the contract-dependent types to a different layer
2. Use trait objects or generics to avoid direct contract imports
3. Create intermediate types in taxonomy that don't depend on contracts

- [ ] **Step 2: Fix capabilities_lint_executor.rs**

This file is in capabilities layer but imports from other capabilities. Capabilities must not depend on other capabilities.

Read the file to understand what it imports, then refactor to:
1. Extract shared logic to utility layer
2. Use contract traits instead of concrete capability implementations
3. Pass dependencies via constructor injection

- [ ] **Step 3: Run self-lint check**

Run: `cargo run --bin lint-arwaky-cli -- check . 2>&1 | grep AES201`
Expected: No AES201 violations

- [ ] **Step 4: Run tests**

Run: `cargo test --workspace`
Expected: All tests pass

- [ ] **Step 5: Commit**

```bash
git add crates/shared/src/cli-commands/taxonomy_lint_dependencies_vo.rs crates/tui/src/capabilities_lint_executor.rs
git commit -m "fix: remove forbidden imports from taxonomy and capabilities layers"
```

---

## Task 3: Fix AES304 Bypass Comment Violations (7 violations)

**Covers:** AES304 BYPASS_COMMENT

**Files:**
- Modify: `crates/cli-commands/src/surface_check_command.rs` (1 violation)
- Modify: `crates/code-analysis/src/capabilities_check_bypass_checker.rs` (5 violations)
- Modify: `crates/naming-rules/src/capabilities_naming_convention_checker.rs` (1 violation)
- Modify: `crates/orphan-detector/src/agent_orphan_orchestrator.rs` (1 violation)
- Modify: `crates/shared/src/code-analysis/utility_bypass.rs` (1 violation)

**Interfaces:**
- Consumes: None
- Produces: Code with no bypass comments

- [ ] **Step 1: Find and remove all bypass comments**

Search for bypass patterns:
```bash
grep -rn "#\[allow\|unwrap()\|expect(" crates/ --include="*.rs" | grep -v "test"
```

For each bypass comment found:
1. Understand why it was added
2. Fix the root cause properly
3. Remove the bypass comment

- [ ] **Step 2: Run self-lint check**

Run: `cargo run --bin lint-arwaky-cli -- check . 2>&1 | grep AES304`
Expected: No AES304 BYPASS_COMMENT violations

- [ ] **Step 3: Run tests**

Run: `cargo test --workspace`
Expected: All tests pass

- [ ] **Step 4: Commit**

```bash
git add crates/
git commit -m "fix: remove bypass comments and fix root causes"
```

---

## Task 4: Fix AES304 UNWRAP_EXPECT, TODO, UNIMPLEMENTED, PANIC Violations (6 violations)

**Covers:** AES304 UNWRAP_EXPECT, AES304 TODO, AES304 UNIMPLEMENTED, AES304 PANIC

**Files:**
- Modify: `crates/config-system/src/agent_config_orchestrator.rs` (1 UNWRAP_EXPECT)
- Modify: `crates/shared/src/import-rules/utility_dummy_detector.rs` (1 TODO, 2 UNIMPLEMENTED, 1 PANIC)
- Modify: `crates/root_cli_main_entry.rs` (1 UNIMPLEMENTED)

**Interfaces:**
- Consumes: None
- Produces: Code with proper error handling

- [ ] **Step 1: Fix agent_config_orchestrator.rs**

Replace `unwrap()`/`expect()` with proper error handling:

```rust
// Instead of:
let value = some_option.unwrap();

// Use:
let value = match some_option {
    Some(v) => v,
    None => return Err(PipelineError::MissingConfig("description".to_string())),
};
```

- [ ] **Step 2: Fix utility_dummy_detector.rs**

Replace `todo!()`, `unimplemented!()`, and `panic!()` with proper implementations:

```rust
// Instead of:
todo!("implement this")

// Use:
return Err(PipelineError::NotImplemented("feature name".to_string()));
```

- [ ] **Step 3: Fix root_cli_main_entry.rs**

Replace `unimplemented!()` with proper implementation or error return.

- [ ] **Step 4: Run self-lint check**

Run: `cargo run --bin lint-arwaky-cli -- check . 2>&1 | grep AES304`
Expected: No AES304 violations

- [ ] **Step 5: Run tests**

Run: `cargo test --workspace`
Expected: All tests pass

- [ ] **Step 6: Commit**

```bash
git add crates/
git commit -m "fix: replace unwrap/expect/todo/unimplemented/panic with proper error handling"
```

---

## Task 5: Fix AES402 CONTRACT_PRIMITIVE Violations (3 violations)

**Covers:** AES402 CONTRACT_PRIMITIVE

**Files:**
- Modify: `crates/shared/src/cli-commands/contract_report_formatter_protocol.rs`
- Modify: `crates/shared/src/code-analysis/contract_code_metric_analyzer_protocol.rs`
- Modify: `crates/shared/src/external-lint/contract_external_lint_selector_protocol.rs`

**Interfaces:**
- Consumes: Taxonomy VOs
- Produces: Contract traits using VOs instead of primitives

- [ ] **Step 1: Read each contract file**

Understand what primitive types are being used and what VOs should replace them.

- [ ] **Step 2: Create or use existing VOs**

For each primitive type in contract signatures:
1. Check if a VO already exists in taxonomy
2. If not, create one
3. Update the contract to use the VO

Example:
```rust
// Instead of:
fn format(&self, results: Vec<LintResult>, path: &str) -> String;

// Use:
fn format(&self, results: Vec<LintResult>, path: FilePath) -> String;
```

- [ ] **Step 3: Update implementations**

Update all implementations of these contracts to use the new VO types.

- [ ] **Step 4: Run self-lint check**

Run: `cargo run --bin lint-arwaky-cli -- check . 2>&1 | grep AES402`
Expected: No AES402 violations

- [ ] **Step 5: Run tests**

Run: `cargo test --workspace`
Expected: All tests pass

- [ ] **Step 6: Commit**

```bash
git add crates/shared/
git commit -m "fix: replace primitive types with VOs in contract signatures"
```

---

## Task 6: Fix AES202 MANDATORY_IMPORT Violations (3 violations)

**Covers:** AES202 MANDATORY_IMPORT

**Files:**
- Modify: `crates/shared/src/external-lint/contract_external_lint_selector_protocol.rs`

**Interfaces:**
- Consumes: Taxonomy layer
- Produces: Contract with required taxonomy import

- [ ] **Step 1: Add required taxonomy import**

The contract layer must import taxonomy to satisfy interface composition requirements.

```rust
// Add at the top of the file:
use crate::taxonomy::*;  // or specific taxonomy imports
```

- [ ] **Step 2: Run self-lint check**

Run: `cargo run --bin lint-arwaky-cli -- check . 2>&1 | grep AES202`
Expected: No AES202 violations

- [ ] **Step 3: Run tests**

Run: `cargo test --workspace`
Expected: All tests pass

- [ ] **Step 4: Commit**

```bash
git add crates/shared/src/external-lint/contract_external_lint_selector_protocol.rs
git commit -m "fix: add mandatory taxonomy import to contract"
```

---

## Task 7: Fix AES102 SUFFIX_MISMATCH Violation (1 violation)

**Covers:** AES102 SUFFIX_MISMATCH

**Files:**
- Rename: `crates/shared/src/external-lint/taxonomy_external_lint_helper.rs`

**Interfaces:**
- Consumes: None
- Produces: File with correct suffix

- [ ] **Step 1: Analyze the file**

Read the file to determine its actual purpose:
- If it contains domain types (structs, enums) → rename to `taxonomy_*.vo.rs`, `taxonomy_*.entity.rs`, etc.
- If it contains business logic → move to `capabilities_*.rs`
- If it contains stateless technical mechanics → move to `utility_*.rs`

- [ ] **Step 2: Rename or move the file**

Based on analysis, either:
1. Rename with correct suffix
2. Move to correct layer

- [ ] **Step 3: Update all imports**

Update any files that import from this file to use the new path.

- [ ] **Step 4: Run self-lint check**

Run: `cargo run --bin lint-arwaky-cli -- check . 2>&1 | grep AES102`
Expected: No AES102 violations

- [ ] **Step 5: Run tests**

Run: `cargo test --workspace`
Expected: All tests pass

- [ ] **Step 6: Commit**

```bash
git add crates/shared/src/external-lint/
git commit -m "fix: rename file with correct suffix for taxonomy layer"
```

---

## Task 8: Fix AES203 UNUSED_IMPORT and AES204 IMPORT_INTENT Violations (12 violations)

**Covers:** AES203 UNUSED_IMPORT, AES204 IMPORT_INTENT

**Files:**
- Modify: `crates/code-analysis/src/capabilities_code_duplication_analyzer.rs` (2 violations)
- Modify: `crates/code-analysis/src/agent_code_analysis_orchestrator.rs` (4 violations)
- Modify: `crates/import-rules/src/capabilities_dummy_import_checker.rs` (4 violations)
- Modify: `crates/shared/src/cli-commands/utility_score_calculator.rs` (1 violation)
- Modify: `crates/shared/src/code-analysis/utility_duplication.rs` (2 violations)
- Modify: `crates/shared/src/common/utility_file.rs` (1 violation)
- Modify: `crates/cli-commands/src/surface_check_action.rs` (1 violation)

**Interfaces:**
- Consumes: None
- Produces: Code with proper imports

- [ ] **Step 1: Remove unused imports**

For each AES203 violation, remove the unused import statement.

- [ ] **Step 2: Fix import intent violations**

For each AES204 violation, the import is only used in dummy functions. Either:
1. Use the import in real logic
2. Remove the import if not needed
3. Move the logic to the correct layer

- [ ] **Step 3: Run self-lint check**

Run: `cargo run --bin lint-arwaky-cli -- check . 2>&1 | grep -E "AES203|AES204"`
Expected: No AES203 or AES204 violations

- [ ] **Step 4: Run tests**

Run: `cargo test --workspace`
Expected: All tests pass

- [ ] **Step 5: Commit**

```bash
git add crates/
git commit -m "fix: remove unused imports and fix import intent violations"
```

---

## Task 9: Fix AES305 DEAD_INHERITANCE Violations (22 violations)

**Covers:** AES305 DEAD_INHERITANCE

**Files:**
- Multiple files across crates (see violation list)

**Interfaces:**
- Consumes: None
- Produces: Code with no empty implementation blocks

- [ ] **Step 1: Identify all empty implementation blocks**

Search for empty impl blocks:
```bash
grep -rn "impl.*{" crates/ --include="*.rs" -A 1 | grep -B 1 "^}$"
```

- [ ] **Step 2: Fix each empty implementation**

For each empty impl block:
1. If it's a trait implementation, add the required methods
2. If it's an inherent impl with no methods, remove it
3. If it's a stub, implement the logic

- [ ] **Step 3: Run self-lint check**

Run: `cargo run --bin lint-arwaky-cli -- check . 2>&1 | grep AES305`
Expected: No AES305 DEAD_INHERITANCE violations

- [ ] **Step 4: Run tests**

Run: `cargo test --workspace`
Expected: All tests pass

- [ ] **Step 5: Commit**

```bash
git add crates/
git commit -m "fix: implement empty trait implementations and remove dead code"
```

---

## Task 10: Fix AES302 FILE_TOO_SHORT Violations (22 violations)

**Covers:** AES302 FILE_TOO_SHORT

**Files:**
- Multiple files in `crates/shared/src/` (see violation list)

**Interfaces:**
- Consumes: None
- Produces: Files with minimum 10 lines

- [ ] **Step 1: Analyze each short file**

For each file with fewer than 10 lines:
1. Determine if it should be expanded with more functionality
2. Determine if it should be merged into a related module
3. Determine if the minimum line requirement should be adjusted in config

- [ ] **Step 2: Fix or merge files**

Options:
1. Add more functionality to the file
2. Merge with a related file
3. Add documentation/comments if the file is intentionally small
4. Update the lint config to exempt certain file types

- [ ] **Step 3: Run self-lint check**

Run: `cargo run --bin lint-arwaky-cli -- check . 2>&1 | grep AES302`
Expected: No AES302 violations

- [ ] **Step 4: Run tests**

Run: `cargo test --workspace`
Expected: All tests pass

- [ ] **Step 5: Commit**

```bash
git add crates/shared/
git commit -m "fix: expand or merge short files to meet minimum line requirement"
```

---

## Task 11: Fix AES305 CODE_DUPLICATION Violations (2 violations)

**Covers:** AES305 CODE_DUPLICATION

**Files:**
- Modify: `crates/report-formatter/src/capabilities_sarif_formatter.rs`
- Modify: `crates/report-formatter/src/capabilities_junit_formatter.rs`

**Interfaces:**
- Consumes: None
- Produces: Code with no duplication

- [ ] **Step 1: Identify duplicated code**

The violations indicate these files duplicate code from:
- `crates/cli-commands/src/agent_analysis_pipeline_orchestrator.rs`
- `crates/report-formatter/src/capabilities_json_formatter.rs`
- `crates/report-formatter/src/capabilities_text_formatter.rs`

- [ ] **Step 2: Extract shared logic**

Create a shared utility function or module for the duplicated logic.

- [ ] **Step 3: Update files to use shared logic**

Refactor the duplicated files to use the shared module.

- [ ] **Step 4: Run self-lint check**

Run: `cargo run --bin lint-arwaky-cli -- check . 2>&1 | grep "CODE_DUPLICATION"`
Expected: No CODE_DUPLICATION violations

- [ ] **Step 5: Run tests**

Run: `cargo test --workspace`
Expected: All tests pass

- [ ] **Step 6: Commit**

```bash
git add crates/report-formatter/
git commit -m "fix: extract duplicated code into shared module"
```

---

## Task 12: Fix AES405 AGENT_ROLE and AES403 CAPABILITY_ROLE Violations (2 violations)

**Covers:** AES405 AGENT_ROLE, AES403 CAPABILITY_ROLE

**Files:**
- Modify: `crates/cli-commands/src/agent_analysis_pipeline_orchestrator.rs` (AES405)
- Modify: `crates/orphan-detector/src/capabilities_orphan_contract_analyzer.rs` (AES403)

**Interfaces:**
- Consumes: None
- Produces: Compliant agent and capability files

- [ ] **Step 1: Fix AES405 - Agent file exceeds 500 lines**

The agent file is too large. Split it into smaller focused modules:
1. Extract pipeline logic into separate modules
2. Keep the orchestrator focused on coordination
3. Move helper functions to utility layer

- [ ] **Step 2: Fix AES403 - Capability struct has no trait implementation**

The `SearchFilesCache` struct needs to implement its corresponding trait.

```rust
impl ISearchFilesCache for SearchFilesCache {
    // Implement required methods
}
```

- [ ] **Step 3: Run self-lint check**

Run: `cargo run --bin lint-arwaky-cli -- check . 2>&1 | grep -E "AES405|AES403"`
Expected: No AES405 or AES403 violations

- [ ] **Step 4: Run tests**

Run: `cargo test --workspace`
Expected: All tests pass

- [ ] **Step 5: Commit**

```bash
git add crates/cli-commands/src/agent_analysis_pipeline_orchestrator.rs crates/orphan-detector/src/capabilities_orphan_contract_analyzer.rs
git commit -m "fix: split large agent file and add trait implementation"
```

---

## Task 13: Fix Formatting and Final Verification

**Covers:** formatting, final verification

**Files:**
- All modified files

**Interfaces:**
- Consumes: All previous tasks
- Produces: Clean codebase with 0 violations

- [ ] **Step 1: Run cargo fmt**

Run: `cargo fmt --all`
Expected: All files formatted

- [ ] **Step 2: Run cargo clippy**

Run: `cargo clippy --all-targets -- -D warnings`
Expected: No warnings

- [ ] **Step 3: Run all tests**

Run: `cargo test --workspace`
Expected: All tests pass

- [ ] **Step 4: Run self-lint check**

Run: `cargo run --bin lint-arwaky-cli -- check .`
Expected: Total violations: 0

- [ ] **Step 5: Final commit**

```bash
git add .
git commit -m "fix: format code and final verification - 0 violations achieved"
```

---

## Execution Order

1. Task 1: Fix compilation errors (MUST be first)
2. Task 2: Fix AES201 FORBIDDEN_IMPORT (17 violations - highest count)
3. Task 3: Fix AES304 BYPASS_COMMENT (7 violations)
4. Task 4: Fix AES304 UNWRAP_EXPECT/TODO/UNIMPLEMENTED/PANIC (6 violations)
5. Task 5: Fix AES402 CONTRACT_PRIMITIVE (3 violations)
6. Task 6: Fix AES202 MANDATORY_IMPORT (3 violations)
7. Task 7: Fix AES102 SUFFIX_MISMATCH (1 violation)
8. Task 8: Fix AES203/AES204 IMPORT violations (12 violations)
9. Task 9: Fix AES305 DEAD_INHERITANCE (22 violations)
10. Task 10: Fix AES302 FILE_TOO_SHORT (22 violations)
11. Task 11: Fix AES305 CODE_DUPLICATION (2 violations)
12. Task 12: Fix AES405/AES403 role violations (2 violations)
13. Task 13: Final formatting and verification

---

## Success Criteria

- [ ] `cargo test --workspace` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo fmt --all` is clean
- [ ] `cargo run --bin lint-arwaky-cli -- check .` reports **Total violations: 0**
