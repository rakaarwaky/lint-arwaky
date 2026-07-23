# FRD — role-rules

## System Overview

The role-rules crate enforces architectural boundaries and responsibility rules for each layer (Taxonomy, Contract, Capabilities, Agent, Surface, Utility, Root) as defined by the 7-layer AES architecture. The file dispatcher classifies files by their filename prefix and dispatches to 6 layer-specific role checkers (AES401–AES406). Root layer files are skipped (pure DI wiring only).

```
Target Path
    │
    ▼
┌──────────────────┐
│ Role Dispatcher  │  ← role enforcement aggregate trait
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
- **Input**: Target file path (directory or single file) and architecture configuration.
- **Output**: All violations found across all files.
- **Business Rules**:
  - Collect files recursively, filtering by extensions: `.rs`, `.py`, `.js`, `.ts`, `.jsx`, `.tsx`.
  - Extract filename prefix as the first `_`-separated segment of the stem.
  - Match prefix to layer: `agent`, `surfaces`/`surface`, `contract`, `capabilities`/`capability`, `utility`, `taxonomy`, `root`.
  - Apply ignore paths from architecture configuration — matching files/directories are excluded.
  - Skip `root` layer files (pure DI wiring, no role checks).
- **Edge Cases**: Files with no underscore in the name have no prefix match — silently skipped. Files matching multiple ignore patterns are still excluded (any match suffices).
- **Error Handling**: Unreadable files produce a default empty content string and are checked with empty content (no crash).

### FR-002: Taxonomy Purity and Primitive Restriction (AES401)
- **Description**: Audit taxonomy layer files (`taxonomy_*`) for raw primitive types in type annotations and ensure constant files contain only pure constant declarations.
- **Input**: Source content value object (file path + content + language).
- **Output**: Violations.
- **Business Rules**:
  - Entity check: Scan `_entity` files for raw primitives in type annotations. Primitives: `String`, `i32`, `bool`, `Vec<`, `HashMap<` (Rust); `str`, `int`, `float`, `bool`, `list`, `dict` (Python); `string`, `number`, `boolean`, `any` (JS/TS).
  - Error check: Same primitive scan on `_error` files.
  - Event check: Same primitive scan on `_event` files.
  - Constant check: Ensure `_constant` files contain only `pub const` / `pub static` (Rust). Flag any struct, enum, fn, impl, mod, trait, class, or type alias.
  - **Skip rules**: Lines starting with `class `, `pub struct `, `struct ` are excluded (type definitions). Lines containing `pub(crate) value:` or `pub value:` are excluded (internal VO wrappers). Lines starting with `fn from(` or `fn visit_` are excluded.
- **Edge Cases**: Taxonomy file with mixed valid and invalid annotations — only the violating lines are reported. Constant file with a helper function buried in comments — noise stripping removes comments first.
- **Error Handling**: Empty files produce no violations. Files with unsupported language produce no violations.

### FR-003: Contract Primitive Restriction (AES402)
- **Description**: Audit contract layer files (`contract_*`) for raw primitive types in method signatures.
- **Input**: Source content value object (file path + content + language).
- **Output**: Violations.
- **Business Rules**:
  - Protocol check: Detect raw primitives in method signatures of `_protocol` files.
  - Aggregate check: Same check on `_aggregate` files.
  - Detection uses shared utility functions for extracting method signatures from Rust, Python, and TypeScript source code.
  - Each extracted signature is scanned for forbidden primitive types.
- **Edge Cases**: Protocol file with zero methods — no violations (nothing to check). Aggregate file with only type aliases — no method signatures to extract.
- **Error Handling**: Unparseable signatures are skipped (fail-safe, no false positives).

### FR-004: Capability Protocol Implementation (AES403)
- **Description**: Audit capability files (`capabilities_*` / `capability_*`) for correct protocol implementation and composition constraints.
- **Input**: Source content value object (file path + content + language).
- **Output**: Violations.
- **Business Rules**:
  - **Rule 1**: File must import from a protocol module. Violation: "missing protocol import".
  - **Rule 2**: At least 1 struct/class must implement the imported protocol (`impl Trait for Struct` in Rust, `class Name(Protocol)` in Python, `class Name implements IProtocol` in TS). Violation: "missing protocol implementor".
  - **Rule 3**: Max 3 type declarations (struct/enum/class/interface) per file. Violation: "too many types".
  - Internal helper types (structs without protocol impl) are allowed and not flagged.
- **Edge Cases**: Capability file with protocol import but no implementation (abstract capability) — flagged by Rule 2. File with exactly 3 types — passes Rule 3.
- **Error Handling**: Files that cannot be parsed for imports/types produce no violations (fail-safe).

### FR-005: Utility Purity (AES404)
- **Description**: Audit utility files (`utility_*`) to ensure they contain only stateless standalone functions with no type definitions.
- **Input**: Source content value object (file path + content + language).
- **Output**: Violations.
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
- **Input**: Source content value object (file path + content + language).
- **Output**: Violations.
- **Business Rules**:
  - **Rule 1**: File must import from an aggregate module. Violation: "missing aggregate import".
  - **Rule 2**: At least 1 struct/class must implement the imported aggregate. Violation: "missing aggregate implementor".
  - **Rule 3**: Max 3 type declarations (struct/enum/class/interface) per file. Violation: "too many types".
  - Internal helper types (structs without aggregate impl) are allowed and not flagged.
- **Edge Cases**: Agent file with aggregate import but no implementation — flagged by Rule 2. File with helper struct + orchestrator struct = 2 types, passes Rule 3.
- **Error Handling**: Files that cannot be parsed produce no violations (fail-safe).

### FR-007: Surface Passive Role (AES406)
- **Description**: Audit surface files (`surface_*` / `surfaces_*`) for role-appropriate constraints based on Smart/Utility/Passive classification.
- **Input**: Source content value object (file path + content + language).
- **Output**: Violations.
- **Business Rules**:
  - **Surface classification by filename suffix**:
    - Smart: `_command`, `_controller`, `_page`, `_entry` — may contain orchestration logic.
    - Utility: `_hook`, `_store`, `_action`, `_screen`, `_router` — support smart surfaces.
    - Passive: All other surface files — presentation-only.
  - **Global check (all surfaces)**: Function count limit — max 15 `fn`/`def`/`function` occurrences per file.
  - **Passive + Utility checks**:
    - Hierarchy check: Max 10 public methods per class/impl block.
    - Method body length: Max 80 lines per method.
    - If-nesting depth: Max 3 levels.
  - **Domain logic check (passive + utility, layer-map-dependent)**: Max 3 control-flow statements (`if`, `else`, `for`, `while`, `match`, `switch`, `try`, `except`, `catch`). Exceeding flagged as domain logic violation.
  - Smart surfaces (`_command`, `_controller`, `_page`, `_entry`) are exempted from passive checks but still subject to the 15-function global limit.
- **Edge Cases**: Surface file with 16 functions — flagged by global limit even if it is a Smart surface. Passive surface with 10 public methods in one class and 5 in another — first class passes, second passes (limit is per class/impl, not per file).
- **Error Handling**: Files with unclassifiable suffixes default to Passive group.

### FR-008: Configuration-Driven Ignore and Toggle
- **Description**: Respect per-layer configuration for ignore paths and enable/disable toggles.
- **Input**: Architecture configuration.
- **Output**: Filtered scan results.
- **Business Rules**:
  - Configuration disabled — full role check returns immediately, no violations produced.
  - Architecture configuration ignore paths — Files/directories matching any pattern are excluded from scanning.
  - Path matching uses substring containment on the full path and the directory name.
- **Edge Cases**: Empty ignored_paths list — no files excluded. Ignored path pattern with leading `/` — matched against both full path and trimmed directory name.
- **Error Handling**: N/A — simple string matching.

## Data Model / Entity Relationship

```
Source Content Value Object
├── file: file path
├── content: file content
└── language: language identifier

Role Checker Dependencies
├── taxonomy: taxonomy role checker protocol
├── contract: contract role checker protocol
├── capabilities: capabilities role checker protocol
├── surface: surface role checker protocol
├── agent: agent role checker protocol
└── utility: utility role checker protocol

Role Orchestrator
├── dependencies: role checker dependencies
├── config: architecture configuration
└── ignored paths: list of path patterns to skip

Lint Result (output)
├── file: file path
├── line: line number
├── column: column number
├── code: error code
├── message: lint message
├── source: adapter name
├── severity: severity level
├── enclosing scope: optional scope reference
└── related locations: list of related locations
```

## API Contract

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| Run role enforcement audit | Target file path | Lint results | Run all role checks on target path |
| Get auditor name | — | String | Returns "role-rules" |
| Dispatch all files to layer checkers | File list, lint result collector | — | Dispatch all files to layer checkers |
| Taxonomy entity primitive check | Source content, lint result collector | — | AES401 entity primitive check |
| Taxonomy error primitive check | Source content, lint result collector | — | AES401 error primitive check |
| Taxonomy event primitive check | Source content, lint result collector | — | AES401 event primitive check |
| Taxonomy constant purity check | Source content, lint result collector | — | AES401 constant purity check |
| Contract protocol primitive check | Source content | Lint results | AES402 protocol primitive check |
| Contract aggregate primitive check | Source content | Lint results | AES402 aggregate primitive check |
| Capability composition check | Source content, root dir, lint result collector | — | AES403 capability composition check |
| Utility purity check | Source content, lint result collector | — | AES404 utility purity check |
| Agent composition check | Source content, root dir, lint result collector | — | AES405 agent composition check |
| Surface global function count | Source content, lint result collector | — | AES406 global function count |
| Smart surface checks | Source content, lint result collector | — | AES406 smart surface checks |
| Utility surface checks | Source content, lint result collector | — | AES406 utility surface checks |
| Passive surface checks | Source content, lint result collector | — | AES406 passive surface checks |
| Create DI container with config | Architecture configuration | Role enforcement container | DI container with config |
| Create DI from config orchestrator | Config orchestrator reference, root dir | Role enforcement container | Canonical DI from config orchestrator |
| Expose orchestrator | — | Role runner aggregate | Expose orchestrator as trait object |

## Integration Points

- **Internal**:
  - The role rules aggregate contract — role enforcement aggregate trait (aggregate contract).
  - The role rules protocol contracts — 6 layer-specific role checker protocols.
  - The shared source content value object — file path + content + language.
  - The common language detection utility — language detection from file extension.
  - The config system configuration value objects — architecture config for ignore paths and toggles.
  - The CLI result value objects — lint result output type.
  - The config system orchestrator aggregate — config loading from orchestrator.
- **External**: None — pure static analysis, no network or filesystem writes.

## Non-functional Requirements (Detailed)

- **Performance**: Role checks operate on in-memory file content. No I/O during check execution. File collection walks directories once.
- **Memory**: One file loaded at a time into the source content value object. For 10,000 files, peak memory < 10MB (content strings are dropped after each check).
- **Accuracy**: Zero false positives on correctly structured code. Each AES rule has precisely defined skip rules and thresholds.
- **Language coverage**: Rust, Python, TypeScript, JavaScript all produce accurate violations via language-specific parsers and noise stripping.
- **Configurability**: All behavior overridable via the architecture configuration. Ignore paths, enable/disable toggles, and layer-specific exceptions are all config-driven.

## Test Scenarios / QA Checklist

- [ ] AES401: Taxonomy entity file with `String` field — violation reported at exact line.
- [ ] AES401: Taxonomy entity file with `FilePath` field — no violation (custom VO, not primitive).
- [ ] AES401: Taxonomy constant file with `pub const X: i32 = 5` — no violation.
- [ ] AES401: Taxonomy constant file with `pub fn helper()` — violation (function in constant file).
- [ ] AES402: Contract protocol with `String` in method signature — violation.
- [ ] AES402: Contract protocol with `FilePath` in method signature — no violation.
- [ ] AES402: Contract aggregate with zero methods — no violations.
- [ ] AES403: Capability file with no protocol import — missing protocol import violation.
- [ ] AES403: Capability file with protocol import but no implementor — missing protocol implementor violation.
- [ ] AES403: Capability file with 4 type declarations — too many types violation.
- [ ] AES403: Capability file with 3 types including helper struct — passes (helper not counted if no protocol impl).
- [ ] AES404: Utility file with `pub struct Config` — violation.
- [ ] AES404: Utility file with only `pub fn helper()` — no violation.
- [ ] AES404: Utility file with `pub struct` inside `/* */` comment — noise stripped, no violation.
- [ ] AES405: Agent file with no aggregate import — missing aggregate import violation.
- [ ] AES405: Agent file with aggregate import but no implementor — missing aggregate implementor violation.
- [ ] AES406: Smart surface with 16 functions — violation (global limit applies).
- [ ] AES406: Passive surface with 11 public methods in one class — violation.
- [ ] AES406: Utility surface with 4 control-flow statements — domain logic violation.
- [ ] AES406: Smart surface with control-flow statements — no domain logic violation (exempt).
- [ ] Root layer file — completely skipped, zero violations.
- [ ] Config disabled — zero violations for entire scan.
- [ ] Config with `ignored_paths: ["test/"]` — test directory files produce no violations.
- [ ] Multi-language workspace: same rule applied correctly across Rust, Python, TS files.

## Assumptions & Constraints

- Files are classified by filename prefix (first `_`-separated segment), not by content analysis.
- Naming convention is assumed correct (enforced by the naming rules crate).
- Root layer files are pure DI wiring and never checked.
- Language detection is based on file extension, not content analysis.
- Noise stripping operates on raw text lines, not AST parsing.
- The domain logic check rule requires a layer map and is conditionally available.

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
