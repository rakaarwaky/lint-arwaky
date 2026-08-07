# Plan: shared — Business-Analyst

## Summary

The shared crate provides the domain foundation for lint-arwaky: 180 source files across taxonomy VOs (89 files), contract traits (58 files), and utility functions (14 files). The crate is well-structured overall — VOs use macros consistently, all contracts require `Send + Sync`, and utility files in `common/` have zero non-taxonomy imports. However, there are 6 findings across 3 severity levels: a taxonomy VO misplaced inside a contract file, a stateful utility function, missing FRD coverage, and 26+ contract methods using primitive types (`bool`, `&str`, `String`) instead of domain VOs. None are CRITICAL blockers, but the misplaced `WorkspaceType` and primitive contract signatures should be addressed for long-term AES compliance.

---

## Findings

### Requirements Clarity

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 1 | 🟡 WARNING | FRD missing FR-005 for `WorkspaceType` enum — a domain taxonomy type (`WorkspaceType`) exists in the codebase (`contract_workspace_detector_protocol.rs`) but is not documented in any FR | `config_system/contract_workspace_detector_protocol.rs:6-10` | Either add FR-005 covering domain enums used by contracts, or move `WorkspaceType` to `taxonomy_config_language_vo.rs` and document it under FR-001 |
| 2 | 🟢 INFO | FRD utility count claims "13 files in common" but there are 13 utility files in `common/` plus 1 in `config_system/` (14 total) | FRD FR-003 module table | Update FRD to clarify: 13 utility files in common, 1 additional in config_system |

### Business Flow

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 3 | 🟡 WARNING | `WorkspaceType` enum with `impl` blocks (Display, From<WorkspaceType> for ConfigLanguage) placed in contract file — violates AES contract role (pure trait definitions only) and creates a contract→taxonomy coupling via `impl From` | `config_system/contract_workspace_detector_protocol.rs:6-40` | Move `WorkspaceType` to `config_system/taxonomy_workspace_type_vo.rs`. Contract file should contain only `IWorkspaceDetectorProtocol` trait |

### Logic Implementation

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 4 | 🟡 WARNING | `utility_config_parser.rs` uses `OnceLock<ArchitectureConfig>` static cache — violates AES404 (utility must be stateless standalone functions only) | `config_system/utility_config_parser.rs:8,16-18` | Remove static cache; make `default_aes_config()` accept a path parameter. The caching belongs in the capabilities layer (config parser implementation), not in utility |
| 5 | 🟡 WARNING | 26+ contract methods use primitive types (`bool`, `&str`, `String`) instead of domain VOs — violates AES402 (contract traits must use taxonomy VO/constant types) | See sub-table below | Replace `bool` returns with typed VOs, `&str` params with VOs like `ToolName`/`FilePath`, `String` returns with domain error VOs |

**AES402 Primitive-in-contract sub-findings (subset — highest-impact):**

| File | Method | Primitive | Should Be |
|------|--------|-----------|-----------|
| `maintenance/contract_tool_executor_protocol.rs:13` | `fn tool_exists(&self, name: &str) -> bool` | `&str` + `bool` | `ToolName` VO + `BooleanVO` |
| `auto_fix/contract_file_adapter_protocol.rs:15-16` | `fn write_file() -> bool`, `fn path_exists() -> bool` | `bool` | `BooleanVO` |
| `auto_fix/contract_fix_aggregate.rs:20` | `fn execute(..., dry_run: bool)` | `bool` | `BooleanVO` |
| `auto_fix/contract_fix_protocol.rs:38` | `fn execute(..., dry_run: bool)` | `bool` | `BooleanVO` |
| `config_system/contract_workspace_detector_protocol.rs:46` | `fn is_workspace(...) -> bool` | `bool` | `BooleanVO` |
| `external_lint/contract_external_lint_selector_protocol.rs:10` | `fn select_adapters(has_rs: bool, has_py: bool, has_js: bool)` | 3x `bool` | `BooleanVO` or typed presence flags |
| `file_watch/contract_change_analyzer_protocol.rs:10` | `fn is_lintable(&self, path: &str) -> bool` | `&str` + `bool` | `FilePath` + `BooleanVO` |
| `file_watch/contract_watch_aggregate.rs:20` | `fn is_lintable(&self, path: &str) -> bool` | `&str` + `bool` | `FilePath` + `BooleanVO` |
| `maintenance/contract_maintenance_aggregate.rs:18` | `fn run_dependency_report(...) -> Result<DependencyReport, String>` | `String` | Domain error VO |
| `maintenance/contract_maintenance_protocol.rs:12` | `fn run_dependency_report(...) -> Result<DependencyReport, String>` | `String` | Domain error VO |
| `config_system/contract_validator_protocol.rs:9` | `fn is_adapter_enabled(...) -> bool` | `bool` | `BooleanVO` |
| `filesystem/contract_tool_resolution_protocol.rs:13-48` | 5 methods with `bool` returns | `bool` | `BooleanVO` |
| `filesystem/contract_workspace_protocol.rs:20-32` | `fn is_member_path() -> bool`, `fn is_leaf_member_path() -> bool`, `fn check_wired_in_container() -> bool` | `bool` | `BooleanVO` |

### Testability & Acceptance

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 6 | 🟢 INFO | 14 test files exist (unit, contract, integration, acceptance, e2e, smoke) — good coverage but no tests for `utility_signature_parser`, `utility_scope_matcher` edge cases, `utility_config_parser` YAML transform logic, or `taxonomy_orphan_parse_result_vo` (307 lines of complex type) | `tests/` directory | Add unit tests for signature extraction edge cases, config YAML edge cases, and orphan parse result serialization |

### Traceability (FRD→Code)

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 7 | 🟡 WARNING | FRD FR-002 lists `IDeadInheritanceProtocol` and `ILineCheckerProtocol` for quality-rules — these exist in code (`contract_dead_inheritance_protocol.rs`, `contract_line_protocol.rs`) but the FRD description ("detect empty struct/impl blocks") is vague about what the protocol actually checks | `quality_rules/contract_dead_inheritance_protocol.rs`, `contract_line_protocol.rs` | FRD should clarify that `IDeadInheritanceProtocol` detects empty struct/impl blocks (AES303 sub-check 2) and `ILineCheckerProtocol` checks line-count limits (AES301/AES302) |

---

## Violations

**AES404 — Utility Role (MEDIUM):**
- `config_system/utility_config_parser.rs` — `OnceLock` static cache violates stateless function requirement

**AES402 — Contract Role (HIGH):**
- 26+ contract methods across 13 files use primitive types (`bool`, `&str`, `String`) instead of domain VOs

**Contract Role Boundary (structural):**
- `config_system/contract_workspace_detector_protocol.rs` — contains `WorkspaceType` enum + impl blocks (taxonomy types in a contract file)

---

## Action Items

- [ ] **HIGH** Move `WorkspaceType` enum from `contract_workspace_detector_protocol.rs` to a new `taxonomy_workspace_type_vo.rs` file, update contract to use re-exported type
- [ ] **HIGH** Remove `OnceLock` static cache from `utility_config_parser.rs` — move caching to capabilities layer
- [ ] **MEDIUM** Replace `bool` returns in contract methods with `BooleanVO` (highest-impact: tool resolution, workspace detection, file adapter, file watch)
- [ ] **MEDIUM** Replace `&str` params in contract methods with appropriate VOs (`ToolName`, `FilePath`)
- [ ] **MEDIUM** Replace `Result<*, String>` returns in maintenance contracts with domain error VOs
- [ ] **LOW** Update FRD FR-003 utility count (13 → clarify 13+1)
- [ ] **LOW** Add FR-005 or extend FR-001 to cover `WorkspaceType` and other domain enums used by contracts
- [ ] **LOW** Add unit tests for `utility_signature_parser`, `utility_config_parser` YAML transforms, `taxonomy_orphan_parse_result_vo`

---

## Fixed Code

### Fix 1: Move WorkspaceType to taxonomy (config_system/taxonomy_workspace_type_vo.rs)

```rust
// PURPOSE: WorkspaceType — enum for workspace language detection
use crate::config_system::taxonomy_config_language_vo::ConfigLanguage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkspaceType {
    Rust,
    TypeScript,
    Python,
    Unknown,
}

impl WorkspaceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkspaceType::Rust => "rust",
            WorkspaceType::TypeScript => "typescript",
            WorkspaceType::Python => "python",
            WorkspaceType::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for WorkspaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<WorkspaceType> for ConfigLanguage {
    fn from(ws: WorkspaceType) -> Self {
        match ws {
            WorkspaceType::Rust => ConfigLanguage::Rust,
            WorkspaceType::Python => ConfigLanguage::Python,
            WorkspaceType::TypeScript => ConfigLanguage::TypeScript,
            WorkspaceType::Unknown => ConfigLanguage::Rust,
        }
    }
}
```

**Updated `config_system/contract_workspace_detector_protocol.rs`:**
```rust
// PURPOSE: IWorkspaceDetectorProtocol — protocol trait for detecting workspace type from directory structure
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_workspace_type_vo::WorkspaceType;

pub trait IWorkspaceDetectorProtocol: Send + Sync {
    /// Detect workspace type by checking folder structure and config files.
    fn detect(&self, path: &FilePath) -> WorkspaceType;

    /// Check if a path is a workspace root (contains crates/, packages/, or modules/).
    fn is_workspace(&self, path: &FilePath) -> BooleanVO;

    /// Discover workspace member directories under the given root.
    fn discover_workspace_members(&self, root: &FilePath) -> Vec<FilePath>;
}
```

**Updated `config_system/mod.rs` — add module declaration + re-export:**
```rust
pub mod taxonomy_workspace_type_vo;
pub use taxonomy_workspace_type_vo::WorkspaceType;
```

### Fix 2: Remove state from utility_config_parser.rs

The `OnceLock<ArchitectureConfig>` static should be removed. Caching belongs in the capabilities layer (`config-system` crate's implementation), not in the shared utility. The utility should remain pure — `parse_config_yaml(yaml_str)` stays as-is (it's already pure). Remove `default_aes_config()` and `default_config_for_language()` from shared, move them to config-system capabilities.

```rust
// Remove these from shared:
// static DEFAULT_CONFIG: OnceLock<ArchitectureConfig> = OnceLock::new();
// pub fn default_aes_config() -> ArchitectureConfig { ... }
// pub fn default_config_for_language(language: &str) -> ArchitectureConfig { ... }

// Keep only the pure parsing functions:
pub fn parse_config_yaml(yaml_str: &str) -> ArchitectureConfig {
    parse_config_yaml_with_warnings(yaml_str).0
}

pub fn parse_config_yaml_with_warnings(yaml_str: &str) -> (ArchitectureConfig, Vec<String>) {
    // ... existing pure logic unchanged
}
```
