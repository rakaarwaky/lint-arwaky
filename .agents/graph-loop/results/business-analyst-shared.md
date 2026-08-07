# Plan: shared — Business-Analyst

## Summary

The shared crate provides the domain foundation for lint-arwaky: 180 source files across taxonomy VOs (89 files), contract traits (58 files), and utility functions (14 files). The crate is well-structured overall — VOs use macros consistently, all contracts require `Send + Sync`, no AES304 bypass comments, no AES301/302 size violations, and no AES101/AES102 naming violations. However, there are 15 findings across 3 severity levels including 2 CRITICAL: 5 utility files in `common/` import from non-common modules violating AES201/AES404, and a taxonomy file imports from `quality_rules` creating a circular dependency risk. Additionally, 4 dead Cargo dependencies, `tokio` contradicting project policy, a stateful utility function, 6 contract files with implementation code (not pure traits), 40+ contract methods using primitive types instead of VOs (AES402), 5 aggregates with inconsistent `I` prefix naming, and FRD gaps.

---

## Findings

### Requirements Clarity

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 1 | 🟡 WARNING | FRD missing FR-005 for `WorkspaceType` enum — a domain taxonomy type exists in the codebase but is not documented in any FR | `config_system/contract_workspace_detector_protocol.rs:6-10` | Either add FR-005 covering domain enums used by contracts, or move `WorkspaceType` to `taxonomy_config_language_vo.rs` and document under FR-001 |
| 2 | 🟢 INFO | FRD utility count claims "13 files in common" — accurate for common/ but there's 1 additional utility in config_system/ | FRD FR-003 module table | No change needed (FRD correctly states "13 files in common") |

### Business Flow

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 3 | 🟡 WARNING | `WorkspaceType` enum with `impl` blocks (Display, From<WorkspaceType> for ConfigLanguage) placed in contract file — violates AES contract role (pure trait definitions only) and creates contract→taxonomy coupling | `config_system/contract_workspace_detector_protocol.rs:6-40` | Move `WorkspaceType` to `config_system/taxonomy_workspace_type_vo.rs`. Contract file should contain only `IWorkspaceDetectorProtocol` trait |

### Logic Implementation

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 4 | 🟡 WARNING | `utility_config_parser.rs` uses `OnceLock<ArchitectureConfig>` static cache — violates AES404 (utility must be stateless standalone functions only) | `config_system/utility_config_parser.rs:8,16-18` | Remove static cache; move `default_aes_config()` and `default_config_for_language()` to config-system capabilities layer. Keep only pure `parse_config_yaml()` in shared |
| 5 | 🟡 WARNING | 40+ contract methods use primitive types (`bool`, `&str`, `String`) instead of domain VOs — violates AES402 across 21 contract files | See sub-table below | Replace with typed VOs (`BooleanVO`, `ToolName`, `FilePath`, `ContentString`, `LayerNameVO`, domain error VOs) |
| 5a | 🟡 WARNING | 6 contract files contain implementation code (enums, structs, default methods with delegation logic) — should be pure trait definitions only | See sub-table 2 below | Move domain types to taxonomy files; extract delegation logic from default methods to capabilities layer |

**AES402 Primitive-in-contract sub-findings (by module):**

| Module | File | Primitives Found | Fix |
|--------|------|-----------------|-----|
| quality_rules | `contract_bypass_checker_protocol.rs` | `file: &str`, `content: &str` | `FilePath`, `ContentString` |
| quality_rules | `contract_class_protocol.rs` | `file: &str`, `content: &str` | `FilePath`, `ContentString` |
| quality_rules | `contract_dead_inheritance_protocol.rs` | `file: &str`, `content: &str` | `FilePath`, `ContentString` |
| quality_rules | `contract_line_protocol.rs` | `file: &str`, `content: &str` | `FilePath`, `ContentString` |
| orphan_rules | `contract_orphan_parser_protocol.rs` | `path: &str`, `content: &str` | `FilePath`, `ContentString` |
| role_rules | `contract_agent_role_protocol.rs` | `layer: &str` | `LayerNameVO` |
| auto_fix | `contract_fix_protocol.rs` | `dry_run: bool`, `file_path: &str` | `BooleanVO`, `FilePath` |
| auto_fix | `contract_fix_aggregate.rs` | `dry_run: bool` | `BooleanVO` |
| external_lint | `contract_external_lint_selector_protocol.rs` | `has_rs: bool`, `has_py: bool`, `has_js: bool` | `BooleanVO` or typed flags |
| file_watch | `contract_change_analyzer_protocol.rs` | `path: &str` | `FilePath` |
| file_watch | `contract_watch_aggregate.rs` | `path: &str` | `FilePath` |
| filesystem | `contract_filesystem_io_protocol.rs` | `content: &str`, `dir: &str` | `ContentString`, `FilePath` |
| filesystem | `contract_filesystem_aggregate.rs` | `ignored: &[String]` | `&PatternList` |
| filesystem | `contract_parser_protocol.rs` | `content: &str` | `ContentString` |
| filesystem | `contract_workspace_protocol.rs` | `path: &str` | `FilePath` |
| filesystem | `contract_tool_resolution_protocol.rs` | 5x `bool` returns | `BooleanVO` |
| config_system | `contract_workspace_detector_protocol.rs` | `bool` return | `BooleanVO` |
| config_system | `contract_validator_protocol.rs` | `bool` return | `BooleanVO` |
| maintenance | `contract_tool_executor_protocol.rs` | `name: &str`, `bool` return | `ToolName`, `BooleanVO` |
| maintenance | `contract_maintenance_protocol.rs` | `Result<_, String>` | Domain error VO |
| maintenance | `contract_maintenance_aggregate.rs` | `Result<_, String>` | Domain error VO |
| git_hooks | `contract_hook_protocol.rs` | `path: &str` (3 params) | `FilePath` |
| import_rules | `contract_cycle_import_protocol.rs` | `name: &str` | Appropriate VO |
| project_setup | `contract_setup_protocol.rs` | `sudo: bool`, `language: &str`, `filename: &str`, `content: &str`, `path: &str` | `BooleanVO`, VOs |

**Contract files with implementation code (should be pure traits):**

| File | Implementation Found | Issue |
|------|---------------------|-------|
| `config_system/contract_workspace_detector_protocol.rs` | `WorkspaceType` enum + 3 impl blocks | Taxonomy type in contract file |
| `git_hooks/contract_git_hooks_aggregate.rs` | 6 default method implementations with delegation logic | Orchestration in contract |
| `import_rules/contract_dummy_import_protocol.rs` | `check_all_dummy` default method with orchestration logic | Orchestration in contract |
| `maintenance/contract_tool_executor_protocol.rs` | `ToolOutput` struct with primitive fields | Struct definition in contract |
| `project_setup/contract_setup_protocol.rs` | `PackageManagerStatus` struct + `PreFlightResult` type alias | Struct definition in contract |
| `project_setup/contract_setup_aggregate.rs` | `SetupMgmtProtocol` type alias | Type alias in contract |

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 6 | 🔴 CRITICAL | 5 utility files in `common/` import from non-common modules — violates AES201 (utility may depend on taxonomy only) and AES404 | See sub-table below | Move imported VO types from `orphan_rules`/`cli_commands` into `common/taxonomy_*` so imports stay within taxonomy layer |

**AES201 Utility cross-layer import violations:**

| Utility File | Imports From | Violating Types |
|---|---|---|
| `utility_compliance_score.rs` | `crate::cli_commands::taxonomy_result_vo` | `LintResult` |
| `utility_parser_dispatcher.rs` | `crate::orphan_rules::taxonomy_orphan_parse_result_vo` | `FileParseResultVO` |
| `utility_python_parser.rs` | `crate::orphan_rules::taxonomy_orphan_parse_result_vo` | `AstFnDefVO`, `AstImportVO`, `PythonParseResultVO` |
| `utility_rust_parser.rs` | `crate::orphan_rules::taxonomy_orphan_parse_result_vo` | `AstFnDefVO`, `AstImportVO`, `AstModDeclVO`, `AstStructDefVO`, `AstTraitDefVO`, `AstTraitImplVO`, `IdentifierVisitor`, `RustParseResultVO` |
| `utility_ts_parser.rs` | `crate::orphan_rules::taxonomy_orphan_parse_result_vo` | `AstFnDefVO`, `AstImportVO`, `TsParseResultVO` |

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 7 | 🔴 CRITICAL | `taxonomy_definition_vo.rs` (taxonomy layer) imports `CodeAnalysisRuleVO` from `crate::quality_rules::taxonomy_code_analysis_vo` — taxonomy cannot depend on capabilities crate (AES201 circular risk: quality_rules also depends on shared) | `common/taxonomy_definition_vo.rs` imports `quality_rules::taxonomy_code_analysis_vo` | Move `CodeAnalysisRuleVO` and `MandatoryImportRuleVO` to `common/taxonomy_code_analysis_vo.rs` (per project convention: cross-dep VOs belong in common) |

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 8 | 🟡 WARNING | 4 dead dependencies in Cargo.toml — `rayon`, `once_cell`, `regex`, `dashmap` are declared but have zero imports across all 180 files | `Cargo.toml` | Remove dead dependencies to reduce compile time and dependency tree |
| 9 | 🟡 WARNING | `tokio` in shared contradicts project policy — AGENTS.md states "No async runtime — use std::thread/rayon". Used in only 2 files: `contract_provider_protocol.rs` and `utility_command_runner.rs` | `Cargo.toml`, `file_watch/contract_provider_protocol.rs`, `common/utility_command_runner.rs` | Move tokio usage to the feature crates that need it (file-watch). Shared should not introduce async runtime |
| 10 | 🟡 WARNING | 5 aggregate traits lack `I` prefix — inconsistent with the other 10 aggregates that use `I*Aggregate` naming | See sub-table below | Rename to `IGitHooksAggregate`, `ILintFixOrchestratorAggregate`, `IMaintenanceCommandsAggregate`, `IHookManagementOrchestratorAggregate`, `ISetupManagementAggregate` |
| 11 | 🟢 INFO | Surface-specific dependencies (`rmcp` in 1 file, `clap` in 2 files, `syn`/`proc-macro2` in 2 files) in foundation crate couple all downstream crates to surface concerns | `Cargo.toml` | Consider whether MCP/CLI VO types belong in shared or should be defined in their respective surface crates |

**Aggregate naming inconsistency (I-prefix):**

| Current Name | Should Be | File |
|---|---|---|
| `GitHooksAggregate` | `IGitHooksAggregate` | `git_hooks/contract_git_hooks_aggregate.rs` |
| `LintFixOrchestratorAggregate` | `ILintFixOrchestratorAggregate` | `auto_fix/contract_fix_aggregate.rs` |
| `MaintenanceCommandsAggregate` | `IMaintenanceCommandsAggregate` | `maintenance/contract_maintenance_aggregate.rs` |
| `HookManagementOrchestratorAggregate` | `IHookManagementOrchestratorAggregate` | `git_hooks/contract_orchestrator_aggregate.rs` |
| `SetupManagementAggregate` | `ISetupManagementAggregate` | `project_setup/contract_setup_aggregate.rs` |

### Testability & Acceptance

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 11 | 🟢 INFO | 14 test files exist (unit, contract, integration, acceptance, e2e, smoke) — good coverage but no tests for `utility_signature_parser`, `utility_scope_matcher` edge cases, `utility_config_parser` YAML transform logic, or `taxonomy_orphan_parse_result_vo` (307 lines of complex type) | `tests/` directory | Add unit tests for signature extraction edge cases, config YAML edge cases, and orphan parse result serialization |

### Traceability (FRD→Code)

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 12 | 🟡 WARNING | FRD FR-002 lists `IDeadInheritanceProtocol` and `ILineCheckerProtocol` for quality-rules — these exist in code but the FRD description ("detect empty struct/impl blocks") is vague about what the protocol actually checks | `quality_rules/contract_dead_inheritance_protocol.rs`, `contract_line_protocol.rs` | FRD should clarify: `IDeadInheritanceProtocol` detects empty struct/impl blocks (AES303 sub-check 2), `ILineCheckerProtocol` checks line-count limits (AES301/AES302) |

---

## Violations

**AES201 — Forbidden Import (CRITICAL):**
- `common/utility_compliance_score.rs` imports `crate::cli_commands::*` (surface layer type)
- `common/utility_parser_dispatcher.rs` imports `crate::orphan_rules::*` (capabilities layer type)
- `common/utility_python_parser.rs` imports `crate::orphan_rules::*` (capabilities layer type)
- `common/utility_rust_parser.rs` imports `crate::orphan_rules::*` (capabilities layer type)
- `common/utility_ts_parser.rs` imports `crate::orphan_rules::*` (capabilities layer type)
- `common/taxonomy_definition_vo.rs` imports `crate::quality_rules::*` (capabilities layer type — circular risk)

**AES402 — Contract Role (HIGH):**
- 40+ contract methods across 24 files use primitive types (`bool`, `&str`, `String`) instead of domain VOs

**AES404 — Utility Role (MEDIUM):**
- `config_system/utility_config_parser.rs` — `OnceLock` static cache violates stateless function requirement

**Contract Role Boundary (structural):**
- 6 contract files contain implementation code (enums, structs, default methods) — should be pure trait definitions only

**Naming Inconsistency:**
- 5 aggregate traits lack `I` prefix (inconsistent with other 10 aggregates)

**Dependency Hygiene:**
- 4 dead dependencies: `rayon`, `once_cell`, `regex`, `dashmap`
- `tokio` in foundation crate contradicts project async policy

---

## Action Items

- [ ] **CRITICAL** Move orphan parse result VOs from `orphan_rules/taxonomy_orphan_parse_result_vo.rs` into `common/taxonomy_orphan_parse_result_vo.rs` — fix 4 utility import violations
- [ ] **CRITICAL** Move `LintResult` VO (used by compliance_score) into `common/taxonomy_lint_result_vo.rs` (already exists) — remove cli_commands import
- [ ] **CRITICAL** Move `CodeAnalysisRuleVO` and `MandatoryImportRuleVO` to `common/taxonomy_code_analysis_vo.rs` (already exists) — remove quality_rules circular import
- [ ] **HIGH** Move `WorkspaceType` enum from `contract_workspace_detector_protocol.rs` to `taxonomy_workspace_type_vo.rs`
- [ ] **HIGH** Move `ToolOutput` struct from `contract_tool_executor_protocol.rs` to taxonomy; extract default method delegation from `contract_git_hooks_aggregate.rs` and `contract_dummy_import_protocol.rs` to capabilities
- [ ] **HIGH** Remove `OnceLock` static cache from `utility_config_parser.rs` — move caching to capabilities layer
- [ ] **HIGH** Remove dead dependencies from Cargo.toml: `rayon`, `once_cell`, `regex`, `dashmap`
- [ ] **MEDIUM** Replace `&str` params with VOs in 24 contract files (quality_rules, orphan_rules, role_rules, auto_fix, file_watch, filesystem, git_hooks, import_rules, maintenance, project_setup)
- [ ] **MEDIUM** Replace `bool` returns with `BooleanVO` in contract methods (filesystem, config_system, auto_fix, external_lint, maintenance)
- [ ] **MEDIUM** Replace `Result<*, String>` returns in maintenance contracts with domain error VOs
- [ ] **MEDIUM** Rename 5 aggregate traits to add `I` prefix for consistency
- [ ] **MEDIUM** Move `PackageManagerStatus` struct and `PreFlightResult` type alias from contract files to taxonomy
- [ ] **MEDIUM** Move `tokio` usage from shared to feature crates (file-watch)
- [ ] **LOW** Update FRD to clarify `IDeadInheritanceProtocol`/`ILineCheckerProtocol` descriptions
- [ ] **LOW** Add unit tests for `utility_signature_parser`, config YAML transforms, orphan parse result serialization

---

## Fixed Code

### Fix 1: Move orphan parse VOs to common (AES201 fix — 4 utility files)

Move `FileParseResultVO`, `AstFnDefVO`, `AstImportVO`, `AstModDeclVO`, `AstStructDefVO`, `AstTraitDefVO`, `AstTraitImplVO`, `IdentifierVisitor`, `RustParseResultVO`, `PythonParseResultVO`, `TsParseResultVO` from `orphan_rules/taxonomy_orphan_parse_result_vo.rs` to `common/taxonomy_orphan_parse_result_vo.rs`. Add `pub mod taxonomy_orphan_parse_result_vo;` to `common/mod.rs`. Update all 4 utility files to import from `crate::common::taxonomy_orphan_parse_result_vo` instead of `crate::orphan_rules::taxonomy_orphan_parse_result_vo`.

### Fix 2: Move WorkspaceType to taxonomy (AES contract role fix)

Create `config_system/taxonomy_workspace_type_vo.rs` with the `WorkspaceType` enum and all its impl blocks. Update `contract_workspace_detector_protocol.rs` to import from `crate::config_system::taxonomy_workspace_type_vo::WorkspaceType`. Add module declaration to `config_system/mod.rs`.

### Fix 3: Remove state from utility_config_parser.rs (AES404 fix)

Remove `OnceLock`, `DEFAULT_CONFIG` static, `default_aes_config()`, and `default_config_for_language()` from shared. These belong in the config-system capabilities layer. Keep only the pure parsing functions: `parse_config_yaml()` and `parse_config_yaml_with_warnings()`.

### Fix 4: Remove dead dependencies (Cargo.toml cleanup)

Remove `rayon`, `once_cell`, `regex`, `dashmap` from `[dependencies]`.
