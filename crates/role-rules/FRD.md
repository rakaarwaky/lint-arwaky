# FRD — role-rules

## System Overview

The role-rules crate enforces architectural boundaries and responsibility rules for each layer (Taxonomy, Contract, Capabilities, Agent, Surface, Utility, Root) as defined by the 7-layer AES architecture. The `RoleOrchestrator` classifies files by their filename prefix and dispatches to 6 layer-specific role checkers (AES401–AES406). Root layer files are skipped (pure DI wiring only).

```
Target Path
    │
    ▼
┌──────────────────┐
│ RoleOrchestrator │  ← IRoleRunnerAggregate trait
│ (file dispatch)  │
└──┬──┬──┬──┬──┬───┘
   │  │  │  │  │
   ▼  ▼  ▼  ▼  ▼
 Taxonomy Contract Capabilities Utility Agent Surface
 Checker  Checker   Checker     Checker  Checker Checker
(AES401) (AES402)  (AES403)    (AES404) (AES405)(AES406)
```

**Supported languages:** Rust (.rs), Python (.py), TypeScript (.ts/.tsx), JavaScript (.js/.jsx)

## Functional Requirements

### FR-001: File Collection and Classification
- **Description**: Walk the target directory, collect source files, and classify each by its filename prefix to determine its AES layer.
- **Input**: Target `FilePath` (directory or single file) and `ArchitectureConfig`.
- **Output**: `Vec<LintResult>` of all violations found across all files.
- **Business Rules**:
  - Collect files recursively, filtering by extensions: `.rs`, `.py`, `.js`, `.ts`, `.jsx`, `.tsx`.
  - Extract filename prefix as the first `_`-separated segment of the stem.
  - Match prefix to layer: `agent`, `surfaces`/`surface`, `contract`, `capabilities`/`capability`, `utility`, `taxonomy`, `root`.
  - Apply ignore paths from `ArchitectureConfig` — matching files/directories are excluded.
  - Skip `root` layer files (pure DI wiring, no role checks).
- **Edge Cases**: Files with no underscore in the name have no prefix match — silently skipped. Files matching multiple ignore patterns are still excluded (any match suffices).
- **Error Handling**: Unreadable files produce a default empty content string and are checked with empty content (no crash).

### FR-002: Taxonomy Purity and Primitive Restriction (AES401)
- **Description**: Audit taxonomy layer files (`taxonomy_*`) for raw primitive types in type annotations and ensure constant files contain only pure constant declarations.
- **Input**: `SourceContentVO` (file path + content + language).
- **Output**: `Vec<LintResult>` violations.
- **Business Rules**:
  - `check_entity`: Scan `_entity` files for raw primitives in type annotations. Primitives: `String`, `i32`, `bool`, `Vec<`, `HashMap<` (Rust); `str`, `int`, `float`, `bool`, `list`, `dict` (Python); `string`, `number`, `boolean`, `any` (JS/TS).
  - `check_error`: Same primitive scan on `_error` files.
  - `check_event`: Same primitive scan on `_event` files.
  - `check_constant`: Ensure `_constant` files contain only `pub const` / `pub static` (Rust). Flag any struct, enum, fn, impl, mod, trait, class, or type alias.
  - **Skip rules**: Lines starting with `class `, `pub struct `, `struct ` are excluded (type definitions). Lines containing `pub(crate) value:` or `pub value:` are excluded (internal VO wrappers). Lines starting with `fn from(` or `fn visit_` are excluded.
- **Edge Cases**: Taxonomy file with mixed valid and invalid annotations — only the violating lines are reported. Constant file with a helper function buried in comments — noise stripping removes comments first.
- **Error Handling**: Empty files produce no violations. Files with unsupported language produce no violations.

### FR-003: Contract Primitive Restriction (AES402)
- **Description**: Audit contract layer files (`contract_*`) for raw primitive types in method signatures.
- **Input**: `SourceContentVO` (file path + content + language).
- **Output**: `Vec<LintResult>` violations.
- **Business Rules**:
  - `check_protocol`: Detect raw primitives in method signatures of `_protocol` files.
  - `check_aggregate`: Same check on `_aggregate` files.
  - Detection uses shared utility functions: `extract_trait_method_signatures` (Rust), `extract_python_method_signatures` (Python), `extract_typescript_method_signatures` (TypeScript).
  - Each extracted signature is scanned for forbidden primitive types.
- **Edge Cases**: Protocol file with zero methods — no violations (nothing to check). Aggregate file with only type aliases — no method signatures to extract.
- **Error Handling**: Unparseable signatures are skipped (fail-safe, no false positives).

### FR-004: Capability Protocol Implementation (AES403)
- **Description**: Audit capability files (`capabilities_*` / `capability_*`) for correct protocol implementation and composition constraints.
- **Input**: `SourceContentVO` (file path + content + language).
- **Output**: `Vec<LintResult>` violations.
- **Business Rules**:
  - **Rule 1**: File must `import` from a `_protocol` module. Violation: `CapabilityNoProtocol`.
  - **Rule 2**: At least 1 struct/class must implement the imported protocol (`impl Trait for Struct` in Rust, `class Name(Protocol)` in Python, `class Name implements IProtocol` in TS). Violation: `CapabilityNoImplementor`.
  - **Rule 3**: Max 3 type declarations (struct/enum/class/interface) per file. Violation: `CapabilityTooManyTypes`.
  - Internal helper types (structs without protocol impl) are allowed and not flagged.
- **Edge Cases**: Capability file with protocol import but no implementation (abstract capability) — flagged by Rule 2. File with exactly 3 types — passes Rule 3.
- **Error Handling**: Files that cannot be parsed for imports/types produce no violations (fail-safe).

### FR-005: Utility Purity (AES404)
- **Description**: Audit utility files (`utility_*`) to ensure they contain only stateless standalone functions with no type definitions.
- **Input**: `SourceContentVO` (file path + content + language).
- **Output**: `Vec<LintResult>` violations.
- **Business Rules**:
  - **Rust**: Forbid `pub struct`, `pub enum`.
  - **TypeScript**: Forbid `export class`, `export interface`, `export enum`, `export type`.
  - **Python**: Forbid `class `, `def ` (any function definition).
  - **Noise stripping before detection**:
    - Rust: line comments (`//`), block comments (`/* */`), `macro_rules!` bodies.
    - TypeScript: line comments (`//`), block comments (`/* */`), template literals (`` ` ``).
    - Python: line comments (`#`), docstrings (`"""` / `'''`).
- **Edge Cases**: Utility file with a `pub struct` inside a comment — noise stripping removes the comment, no violation. Utility file with only helper functions in Python — flagged (Python utility forbids `def`).
- **Error Handling**: Empty files produce no violations.

### FR-006: Agent Orchestrator Composition (AES405)
- **Description**: Audit agent files (`agent_*`) for correct aggregate implementation and composition constraints.
- **Input**: `SourceContentVO` (file path + content + language).
- **Output**: `Vec<LintResult>` violations.
- **Business Rules**:
  - **Rule 1**: File must `import` from an `_aggregate` module. Violation: `AgentNoAggregate`.
  - **Rule 2**: At least 1 struct/class must implement the imported aggregate. Violation: `AgentNoImplementor`.
  - **Rule 3**: Max 3 type declarations (struct/enum/class/interface) per file. Violation: `AgentTooManyTypes`.
  - Internal helper types (structs without aggregate impl) are allowed and not flagged.
- **Edge Cases**: Agent file with aggregate import but no implementation — flagged by Rule 2. File with helper struct + orchestrator struct = 2 types, passes Rule 3.
- **Error Handling**: Files that cannot be parsed produce no violations (fail-safe).

### FR-007: Surface Passive Role (AES406)
- **Description**: Audit surface files (`surface_*` / `surfaces_*`) for role-appropriate constraints based on Smart/Utility/Passive classification.
- **Input**: `SourceContentVO` (file path + content + language).
- **Output**: `Vec<LintResult>` violations.
- **Business Rules**:
  - **Surface classification by filename suffix**:
    - Smart: `_command`, `_controller`, `_page`, `_entry` — may contain orchestration logic.
    - Utility: `_hook`, `_store`, `_action`, `_screen`, `_router` — support smart surfaces.
    - Passive: All other surface files — presentation-only.
  - **Global check (all surfaces)**: `check_fn_count_limit` — max 15 `fn`/`def`/`function` occurrences per file.
  - **Passive + Utility checks**:
    - `check_surface_hierarchy`: Max 10 public methods per class/impl block.
    - Method body length: Max 80 lines per method.
    - If-nesting depth: Max 3 levels.
  - **Domain logic check (passive + utility, LayerMap-dependent)**: Max 3 control-flow statements (`if`, `else`, `for`, `while`, `match`, `switch`, `try`, `except`, `catch`). Exceeding flagged as `NoDomainLogic` violation.
  - Smart surfaces (`_command`, `_controller`, `_page`, `_entry`) are exempted from passive checks but still subject to the 15-function global limit.
- **Edge Cases**: Surface file with 16 functions — flagged by global limit even if it is a Smart surface. Passive surface with 10 public methods in one class and 5 in another — first class passes, second passes (limit is per class/impl, not per file).
- **Error Handling**: Files with unclassifiable suffixes default to Passive group.

### FR-008: Configuration-Driven Ignore and Toggle
- **Description**: Respect per-layer configuration for ignore paths and enable/disable toggles.
- **Input**: `ArchitectureConfig`.
- **Output**: Filtered scan results.
- **Business Rules**:
  - `config.enabled: false` — `run_all_role_checks` returns immediately, no violations produced.
  - `ArchitectureConfig::ignored_paths` — Files/directories matching any pattern are excluded from scanning.
  - Path matching uses substring containment on the full path and the directory name.
- **Edge Cases**: Empty ignored_paths list — no files excluded. Ignored path pattern with leading `/` — matched against both full path and trimmed directory name.
- **Error Handling**: N/A — simple string matching.

## Data Model / Entity Relationship

```
SourceContentVO
├── file: FilePath
├── content: ContentString
└── language: String

RoleCheckerDeps
├── taxonomy: Arc<dyn ITaxonomyRoleChecker>
├── contract: Arc<dyn IContractRoleChecker>
├── capabilities: Arc<dyn ICapabilitiesRoleChecker>
├── surface: Arc<dyn ISurfaceRoleChecker>
├── agent: Arc<dyn IAgentRoleChecker>
└── utility: Arc<dyn IUtilityRoleChecker>

RoleOrchestrator
├── deps: RoleCheckerDeps
├── config: ArchitectureConfig
└── ignored_paths: Vec<String>

LintResult (output)
├── file: FilePath
├── line: LineNumber
├── column: ColumnNumber
├── code: ErrorCode
├── message: LintMessage
├── source: AdapterName
├── severity: Severity
├── enclosing_scope: Option<ScopeRef>
└── related_locations: LocationList
```

## API Contract

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| `IRoleRunnerAggregate::run_audit` | `&FilePath` target | `Vec<LintResult>` | Run all role checks on target path |
| `IRoleRunnerAggregate::name` | — | `&str` | Returns "role-rules" |
| `RoleOrchestrator::run_all_role_checks` | `&[String]` files, `&mut Vec<LintResult>` | — | Dispatch all files to layer checkers |
| `ITaxonomyRoleChecker::check_entity` | `&SourceContentVO`, `&mut Vec<LintResult>` | — | AES401 entity primitive check |
| `ITaxonomyRoleChecker::check_error` | `&SourceContentVO`, `&mut Vec<LintResult>` | — | AES401 error primitive check |
| `ITaxonomyRoleChecker::check_event` | `&SourceContentVO`, `&mut Vec<LintResult>` | — | AES401 event primitive check |
| `ITaxonomyRoleChecker::check_constant` | `&SourceContentVO`, `&mut Vec<LintResult>` | — | AES401 constant purity check |
| `IContractRoleChecker::check_protocol` | `&SourceContentVO` | `Vec<LintResult>` | AES402 protocol primitive check |
| `IContractRoleChecker::check_aggregate` | `&SourceContentVO` | `Vec<LintResult>` | AES402 aggregate primitive check |
| `ICapabilitiesRoleChecker::check_capability_routing` | `&SourceContentVO`, `&str`, `&mut Vec<LintResult>` | — | AES403 capability composition check |
| `IUtilityRoleChecker::check_utility_convention` | `&SourceContentVO`, `&mut Vec<LintResult>` | — | AES404 utility purity check |
| `IAgentRoleChecker::check_agent_routing` | `&SourceContentVO`, `&str`, `&mut Vec<LintResult>` | — | AES405 agent composition check |
| `ISurfaceRoleChecker::check_fn_count_limit` | `&SourceContentVO`, `&mut Vec<LintResult>` | — | AES406 global function count |
| `ISurfaceRoleChecker::check_smart_surface` | `&SourceContentVO`, `&mut Vec<LintResult>` | — | AES406 smart surface checks |
| `ISurfaceRoleChecker::check_utility_surface` | `&SourceContentVO`, `&mut Vec<LintResult>` | — | AES406 utility surface checks |
| `ISurfaceRoleChecker::check_passive_surface` | `&SourceContentVO`, `&mut Vec<LintResult>` | — | AES406 passive surface checks |
| `RoleContainer::new_with_config` | `ArchitectureConfig` | `RoleContainer` | DI container with config |
| `RoleContainer::from_orchestrator` | `&Arc<dyn IConfigOrchestratorAggregate>`, `&str` | `RoleContainer` | Canonical DI from config orchestrator |
| `RoleContainer::orchestrator` | — | `Arc<dyn IRoleRunnerAggregate>` | Expose orchestrator as trait object |

## Integration Points

- **Internal**:
  - `shared::role_rules::contract_role_runner_aggregate` — `IRoleRunnerAggregate` trait (aggregate contract).
  - `shared::role_rules::contract_*_role_protocol` — 6 layer-specific role checker protocols.
  - `shared::taxonomy_source_vo` — `SourceContentVO` (file path + content + language).
  - `shared::common::utility_language_detector` — Language detection from file extension.
  - `shared::config_system::taxonomy_config_vo` — `ArchitectureConfig` for ignore paths and toggles.
  - `shared::cli_commands::taxonomy_result_vo` — `LintResult` output type.
  - `shared::config_system::contract_config_orchestrator_aggregate` — Config loading from orchestrator.
- **External**: None — pure static analysis, no network or filesystem writes.

## Non-functional Requirements (Detailed)

- **Performance**: Role checks operate on in-memory file content. No I/O during check execution. File collection walks directories once.
- **Memory**: One file loaded at a time into `SourceContentVO`. For 10,000 files, peak memory < 10MB (content strings are dropped after each check).
- **Accuracy**: Zero false positives on correctly structured code. Each AES rule has precisely defined skip rules and thresholds.
- **Language coverage**: Rust, Python, TypeScript, JavaScript all produce accurate violations via language-specific parsers and noise stripping.
- **Configurability**: All behavior overridable via `ArchitectureConfig`. Ignore paths, enable/disable toggles, and layer-specific exceptions are all config-driven.

## Test Scenarios / QA Checklist

- [ ] AES401: Taxonomy entity file with `String` field — violation reported at exact line.
- [ ] AES401: Taxonomy entity file with `FilePath` field — no violation (custom VO, not primitive).
- [ ] AES401: Taxonomy constant file with `pub const X: i32 = 5` — no violation.
- [ ] AES401: Taxonomy constant file with `pub fn helper()` — violation (function in constant file).
- [ ] AES402: Contract protocol with `String` in method signature — violation.
- [ ] AES402: Contract protocol with `FilePath` in method signature — no violation.
- [ ] AES402: Contract aggregate with zero methods — no violations.
- [ ] AES403: Capability file with no protocol import — `CapabilityNoProtocol` violation.
- [ ] AES403: Capability file with protocol import but no implementor — `CapabilityNoImplementor` violation.
- [ ] AES403: Capability file with 4 type declarations — `CapabilityTooManyTypes` violation.
- [ ] AES403: Capability file with 3 types including helper struct — passes (helper not counted if no protocol impl).
- [ ] AES404: Utility file with `pub struct Config` — violation.
- [ ] AES404: Utility file with only `pub fn helper()` — no violation.
- [ ] AES404: Utility file with `pub struct` inside `/* */` comment — noise stripped, no violation.
- [ ] AES405: Agent file with no aggregate import — `AgentNoAggregate` violation.
- [ ] AES405: Agent file with aggregate import but no implementor — `AgentNoImplementor` violation.
- [ ] AES406: Smart surface with 16 functions — violation (global limit applies).
- [ ] AES406: Passive surface with 11 public methods in one class — violation.
- [ ] AES406: Utility surface with 4 control-flow statements — `NoDomainLogic` violation.
- [ ] AES406: Smart surface with control-flow statements — no `NoDomainLogic` violation (exempt).
- [ ] Root layer file — completely skipped, zero violations.
- [ ] Config `enabled: false` — zero violations for entire scan.
- [ ] Config `ignored_paths: ["test/"]` — test directory files produce no violations.
- [ ] Multi-language workspace: same rule applied correctly across Rust, Python, TS files.

## Assumptions & Constraints

- Files are classified by filename prefix (first `_`-separated segment), not by content analysis.
- Naming convention is assumed correct (enforced by `naming-rules` crate).
- Root layer files are pure DI wiring and never checked.
- Language detection is based on file extension, not content analysis.
- Noise stripping operates on raw text lines, not AST parsing.
- The `check_surface_roles` rule (domain logic count) requires a `LayerMap` and is conditionally available.

## Glossary

| Term | Definition |
|------|------------|
| **AES** | Architecture Enforcement Standard — the 7-layer coding convention |
| **Smart surface** | Surface with `_command`, `_controller`, `_page`, `_entry` suffix — may contain orchestration |
| **Utility surface** | Surface with `_hook`, `_store`, `_action`, `_screen`, `_router` suffix — supports smart surfaces |
| **Passive surface** | Any surface file not classified as Smart or Utility — presentation-only |
| **Primitive type** | Raw language types (`String`, `int`, `bool`, etc.) that violate VO-based signatures |
| **Noise stripping** | Removal of comments, docstrings, macros, and template literals before rule checking |
| **VO** | Value Object — a typed wrapper around a primitive that replaces raw types in signatures |

## Reference

- PRD: [PRD.md](../../PRD.md)
