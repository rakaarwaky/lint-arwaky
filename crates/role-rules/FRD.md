# FRD — role-rules (v2.0.0)

---

## Feature Overview

The role-rules crate enforces architectural boundaries and responsibility rules for each layer (Taxonomy, Contract, Capabilities, Agent, Surface, Utility) as defined by the 7-layer AES architecture. It receives pre-parsed file data from the external filesystem crate, classifies files by their filename prefix, and dispatches to 6 layer-specific role checkers (AES401–AES406). Root layer files are skipped (pure DI wiring only).

File discovery, raw content reads, and AST parsing are handled by the external `filesystem` crate. The Surface calls `filesystem.build_file_index(root)` to populate caches, then passes pre-fetched `&[FileEntry]` (with parse_metadata) to the role-rules orchestrator via `run_audit_with_entries`. The role-rules crate does zero I/O — it only performs business logic analysis on pre-fetched data.

Import checking is NOT performed by role-rules. All import validation (forbidden imports, mandatory imports, unused imports) is the responsibility of the import-rules crate (AES201–AES205). Role-rules only validates structural and responsibility constraints within each file.

- **Architecture & Data Flow**

```mermaid
flowchart TD
    A["Surface"] -->|"build_file_index(root)"| D["filesystem_aggregate\n(external crate)"]
    A -->|"file_list()"| D

    subgraph FS ["filesystem crate (external)"]
        D --> E1["file_walker"]
        D --> E2["AST parser\n(parse_metadata)"]
        E1 --> G1["FileEntry[]\n+ content_map"]
        E2 --> G1
    end

    G1 -->|"return"| D
    D -->|"FileEntry[]\n(pre-fetched)"| A

    A -->|"run_audit_with_entries(&[FileEntry])"| B["role_aggregate"]
    B --> C["role_orchestrator\n(zero I/O)"]

    C -->|"classify by prefix"| H1["taxonomy_check"]
    C -->|"classify by prefix"| H2["contract_check"]
    C -->|"classify by prefix"| H3["capabilities_check"]
    C -->|"classify by prefix"| H4["utility_check"]
    C -->|"classify by prefix"| H5["agent_check"]
    C -->|"classify by prefix"| H6["surface_check"]

    H1 --> I["Violations"]
    H2 --> I
    H3 --> I
    H4 --> I
    H5 --> I
    H6 --> I
    I --> J["LintResult"]
    J --> C
    C --> B
    B -->|output| A

```

## Functional Requirements

### FR-RoleRules-001: File Classification and Dispatch

- **Description**: Classify each file received from the filesystem crate by its filename prefix to determine its AES layer, then dispatch to the appropriate layer-specific role checker.
- **Input**: File data (path + content + language + parse metadata, from filesystem crate), architecture configuration.
- **Output**: All violations found across all dispatched files.
- **Business Rules**:

  - Extract filename prefix as the first `_`-separated segment of the stem.
  - Match prefix to layer:

    | Prefix                       | Layer        | Checker                                   |
    | ---------------------------- | ------------ | ----------------------------------------- |
    | `taxonomy`                   | taxonomy     | taxonomy_checker (AES401)                 |
    | `contract`                   | contract     | contract_checker (AES402)                 |
    | `capabilities`, `capability` | capabilities | capabilities_checker (AES403)             |
    | `utility`                    | utility      | utility_checker (AES404)                  |
    | `agent`                      | agent        | agent_checker (AES405)                    |
    | `surface`, `surfaces`        | surface      | surface_checker (AES406)                  |
    | `root`                       | root         | **SKIP** (pure DI wiring, no role checks) |

  - Apply ignore paths from architecture configuration using **segment matching** (split path by `/`, match per segment — pattern `test` matches segment `test` only, not `latest` or `contest`).
  - Files with no underscore in the name have no prefix match → silently skipped.
  - Files with unrecognized prefix → silently skipped.
  - Barrel files (module barrels, library roots, binary entries, package markers, and index barrels) are skipped.
  - Files in the rule's `exceptions` list are skipped.
  - Binary-entry files are always skipped.
- **Edge Cases**:

  - Files matching multiple ignore patterns → excluded (any segment match suffices).
  - `capability_*` and `capabilities_*` both map to capabilities layer.
  - `surface_*` and `surfaces_*` both map to surface layer.
- **Error Handling**: Files that could not be read or parsed by the filesystem crate are excluded from the file list and never reach role-rules. No empty-content fallback.

---

### FR-RoleRules-002: Taxonomy Purity and Primitive Restriction (AES401)

- **Description**: Audit taxonomy layer files (`taxonomy_*`) for raw primitive types in type annotations and ensure constant files contain only pure constant declarations.
- **Input**: `FileEntry` (path + content + language + parse metadata).
- **Output**: AES401 violations.
- **Business Rules**:

  - **Entity/Error/Event primitive check** (`_entity`, `_error`, `_event` files):

    - Scan type annotations in struct fields, function parameters, and return types for raw primitives.
    - Forbidden primitives per language:

      | Language              | Forbidden Primitives                                                                                                                                    |
      | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
      | Rust                  | `String`, `i8`–`i128`, `u8`–`u128`, `f32`, `f64`, `bool`, `char`, `Vec<`, `HashMap<`, `BTreeMap<`, `Option<`, `Result<`, `Box<`, `Cell<`, `RefCell<`, `Arc<`, `Mutex<`, `Rc<` |
      | Python                | `str`, `int`, `float`, `bool`, `list`, `dict`, `tuple`, `set`                                                                                           |
      | TypeScript/JavaScript | `string`, `number`, `boolean`, `any`, `Array<`, `Record<`                                                                                               |

    - Type annotations using custom VO wrappers (e.g., `FilePath`, `LineNumber`, `SymbolName`) are NOT flagged.
    - Detection uses line-based scanning: extract type annotations after `:` in each line and match against the forbidden primitive list.
  - **Constant purity check** (`_constant` files):

    - Only constant declarations are allowed: `pub const` / `pub static` (Rust), module-level assignments (Python), `export const` (TS).
    - Forbidden: `struct`, `enum`, `fn`, `impl`, `mod`, `trait` (Rust); `class`, `def` (Python); `class`, `interface`, `function`, `type` (TS).
    - Uses AST parse metadata when available; falls back to line-based scanning when parse metadata is absent.
  - **Skip rules**: Type definition lines (`struct Foo { ... }`, `class Foo`) are excluded from primitive scanning (the definition itself is not a violation; only field/parameter types are checked).
- **Edge Cases**:

  - Taxonomy file with mixed valid and invalid annotations → only the violating lines are reported.
  - Constant file with a helper function → AES401 violation (function in constant file).
  - Empty files → no violations.
  - Files with unsupported language → no violations.
- **Error Handling**: Emit AES401 with the file path, line number, primitive type found, and expected VO wrapper.

---

### FR-RoleRules-003: Contract Primitive Restriction (AES402)

- **Description**: Audit contract layer files (`contract_*`) for raw primitive types in method signatures. Uses the shared signature parser utility, not parse metadata.
- **Input**: `FileEntry` (path + content + language + parse metadata).
- **Output**: AES402 violations.
- **Business Rules**:

  - **Protocol check** (`_protocol` files): Detect raw primitives in method signatures (parameters and return types) of trait/interface definitions.
  - **Aggregate check** (`_aggregate` files): Same check on aggregate trait/interface definitions.
  - I/O protocol files (`io_protocol`, `filesystem_io`) are exempted from checking.
  - Forbidden primitives: same list as FR-RoleRules-002 per language.
  - Detection uses the shared `utility_signature_parser` functions which extract method signatures and scan parameter/return types for forbidden primitives.
  - Each violating signature is reported individually with line number.
- **Edge Cases**:

  - Protocol file with zero methods → no violations (nothing to check).
  - Aggregate file with only type aliases → no method signatures to extract, no violations.
  - Methods with custom VO types in signatures → no violation.
- **Error Handling**: Emit AES402 with the file path, line number, method name, primitive type found, and expected VO wrapper.

---

### FR-RoleRules-004: Capability Protocol Implementation (AES403)

- **Description**: Audit capability files (`capabilities_*` / `capability_*`) for protocol implementation and composition constraints. This rule checks **implementation only**, not imports. Import validation is handled by the import-rules crate (AES201–AES205).
- **Input**: `FileEntry` (path + content + language + parse metadata).
- **Output**: AES403 violations.
- **Business Rules**:

  - **Rule 1 — Max type declarations**: Maximum 3 type declarations (struct/enum/class/interface) per file. Violation: "too many types". Checked first — if exceeded, skip Rule 2.
  - **Rule 2 — Protocol implementor**: At least 1 struct/class must implement a protocol trait/interface.
    - Rust: `impl Trait for Struct` where Trait is a protocol (detected via AST `ItemImpl` with `trait_` path).
    - Python: `class Name(Parent)` where Parent is a protocol class.
    - TypeScript: `class Name implements IProtocol`.
    - Violation: "missing protocol implementor".
  - Internal helper types (structs/classes without protocol impl) are allowed and not flagged individually.
  - Detection uses AST parse metadata when available; falls back to line-based scanning when absent.
  - **Rule 3 — Single protocol**: exactly 1 protocol trait per capability file; 2 or more must be split, with shared helpers moved to a `utility_*` file.
  - **Rule 4 — Block order**: Block 2 (protocol impl) must precede Block 3 (inherent impl). HIGH.
  - **Rule 5 — Block markers**: the `Block 1:` / `Block 2:` / `Block 3:` banner comments must all be present, in ascending order, and none above 3. MEDIUM. A banner is `Block <digits>:` standing as a whole word inside a comment, so prose such as `Block 1 (types) -> Block 2` and `Sub-Block 4:` are not markers. This is distinct from Rule 4: Rule 4 compares two `impl` lines and cannot tell a file that documents its three blocks from one whose `impl` blocks merely happen to land in a valid order.
  - **Rule 6 — Constant placement**: a file-level `const` is a violation; it belongs in the feature's taxonomy constant module. An associated `const` nested in an `impl` is exempt.
  - **Rule 7 — Test placement**: an inline `#[cfg(test)]` or `mod tests` is a violation; test code belongs in the feature's test target.
  - **Rule 8 — Helper visibility**: a Block 3 helper that is `pub` with no production caller should be `fn`, or `pub(crate)` when a same-crate test module calls it. A helper called only from a `tests/` target stays `pub`, because that target compiles as a separate crate.
- **Edge Cases**:

  - Capability file with no implementor → AES403 violation (Rule 2).
  - File with exactly 3 types → passes Rule 1.
  - File with > 3 types → AES403 violation (Rule 1 only, Rule 2 skipped).
  - File with helper struct + implementor struct = 2 types → passes Rule 1, passes Rule 2.
  - File with all three banners in order → passes Rule 5.
  - File with no `Block N:` banner at all → AES403 violation (Rule 5).
  - File with `Block 1:` and `Block 3:` but no `Block 2:` → AES403 violation (Rule 5).
  - File whose banners read 1 → 3 → 2 → AES403 violation (Rule 5).
  - File carrying a `Block 4:` banner → AES403 violation (Rule 5).
  - A comment containing the prose `Block 1 (types) -> Block 2` → not a marker, so it neither satisfies nor violates Rule 5.
- **Error Handling**: Emit AES403 with the violation kind (`TooManyTypes` or `MissingProtocolImplementor`), file path, and relevant counts.

---

### FR-RoleRules-005: Utility Purity (AES404)

- **Description**: Audit utility files (`utility_*`) to ensure they contain only stateless standalone functions with no type definitions.
- **Input**: `FileEntry` (path + content + language + parse metadata).
- **Output**: AES404 violations.
- **Business Rules**:

  - **Rust**: Forbid `struct`, `enum`, `trait` definitions. Only `fn` (functions) and `const`/`static` (constants) are allowed.
  - **Python**: Forbid `class` definitions. Allow `def` (stateless functions) and module-level assignments (constants).
  - **TypeScript/JavaScript**: Forbid `class`, `interface`, `type alias` definitions. Allow `export function`, `export const`.
  - Detection uses AST parse metadata when available; falls back to line-based scanning (with comment stripping) when absent.
- **Edge Cases**:

  - Utility file with a `struct` inside a comment → not flagged (AST does not parse comments as code).
  - Utility file with only helper functions → no violation.
  - Utility file with `class` (Python) → AES404 violation.
  - Utility file with `struct` (Rust) → AES404 violation.
  - Empty files → no violations.
- **Error Handling**: Emit AES404 with the file path, line number, forbidden item kind, and expected content (stateless functions only).

---

### FR-RoleRules-006: Agent Orchestrator Composition (AES405)

- **Description**: Audit agent files (`agent_*`) for correct aggregate implementation and composition constraints. This rule checks **implementation only**, not imports. Import validation is handled by the import-rules crate (AES201–AES205).
- **Input**: `FileEntry` (path + content + language + parse metadata).
- **Output**: AES405 violations.
- **Business Rules**:

  - **Rule 1 — Max type declarations**: Maximum 3 type declarations (struct/enum/class/interface) per file. Violation: "too many types". Checked first — if exceeded, skip Rule 2.
  - **Rule 2 — Aggregate implementor**: At least 1 struct/class must implement an aggregate trait/interface (trait name must contain "aggregate" or "Aggregate").
    - Rust: `impl Trait for Struct` where Trait name contains "aggregate".
    - Python: `class Name(Parent)` where Parent name contains "aggregate".
    - TypeScript: `class Name implements IAggregate` where interface name contains "aggregate".
    - Violation: "missing aggregate implementor".
  - **Any-type annotation check**: Scans all non-comment lines for `: Any`, `Any<`, `Any[`, `-> Any` patterns. Any use of the `Any` type is flagged.
  - Internal helper types (structs/classes without aggregate impl) are allowed and not flagged individually.
  - Detection uses AST parse metadata when available; falls back to line-based scanning when absent.
- **Edge Cases**:

  - Agent file with no implementor → AES405 violation (Rule 2).
  - File with helper struct + orchestrator struct = 2 types → passes Rule 1.
  - File with > 3 types → AES405 violation (Rule 1, Rule 2 skipped).
  - Agent file with `Any` type annotations → AES405 violation (Any check runs independently).
- **Error Handling**: Emit AES405 with the violation kind (`TooManyTypes`, `MissingAggregateImplementor`, or `AnyTypeAnnotation`), file path, and relevant counts.

---

### FR-RoleRules-007: Surface Passive Role (AES406)

- **Description**: Audit surface files (`surface_*` / `surfaces_*`) for role-appropriate constraints based on Smart/Utility/Passive classification.
- **Input**: `FileEntry` (path + content + language + parse metadata).
- **Output**: AES406 violations.
- **Business Rules**:

  - **Surface classification by filename suffix**:

    - **Smart**: `_command`, `_controller`, `_page`, `_entry`, `_router` — may contain orchestration logic.
    - **Utility**: `_hook`, `_store`, `_action`, `_screen` — support smart surfaces.
    - **Passive**: All other surface suffixes — presentation-only.
  - **Function count limit**: Removed — no limit on function count per file. Smart surfaces are subject to no checks. Utility and Passive surfaces are subject to hierarchy and domain logic checks.
  - **Utility + Passive checks — hierarchy**:

    - Max 50 function definitions per file (counts all functions across all classes/impl blocks).
    - Uses AST parse metadata when available; falls back to line-based scanning when absent.
  - **Utility + Passive checks — domain logic**:

    - Max 50 control-flow statements (`if`, `else`, `for`, `while`, `match`, `switch`, `try:`, `except`, `catch`) per file.
    - Exceeding flagged as domain logic violation — surface files should delegate logic to lower layers.
  - **Surface logic bypass check**:

    - Surface files must not call business logic functions directly (e.g., `lint_path(`, `walk_rs_files(`). These must be delegated to the aggregate layer.
  - **Smart surface exemption**: Smart surfaces are exempted from all checks (hierarchy, domain logic) — no violations produced for Smart surfaces.
  - Detection uses AST parse metadata for function definition counts; control-flow check uses line-based scanning on all files.
- **Edge Cases**:

  - Passive surface with > 50 functions → AES406 violation.
  - Utility surface with 40 control-flow statements → no violation (below threshold).
  - Surface file with unclassifiable suffix → defaults to Passive group.
  - Smart surface with control-flow statements → no violation (exempt).
- **Error Handling**: Emit AES406 with the violation kind (`TooManyMethods` or `DomainLogic`), file path, and actual vs configured threshold.

---

## Public Interface

### Protocol API

| Method | Input | Output | Error | Event | Description |
|---|---|---|---|---|---|
| `check_agent_routing` | &FileEntry, &str, &mut Vec<LintResult> | `` | — | — | Check agent routing. |
| `check_capability_routing` | &FileEntry, &str, &mut Vec<LintResult> | `` | — | — | Check capability routing. |
| `check_protocol` | &FileEntry | `Vec<LintResult>` | — | — | Check protocol. |
| `check_aggregate` | &FileEntry | `Vec<LintResult>` | — | — | Check aggregate. |
| `check_smart_surface` | &FileEntry, &mut Vec<LintResult> | `` | — | — | Check smart surface. |
| `check_utility_surface` | &FileEntry, &mut Vec<LintResult> | `` | — | — | Check utility surface. |
| `check_passive_surface` | &FileEntry, &mut Vec<LintResult> | `` | — | — | Check passive surface. |
| `check_fn_count_limit` | &FileEntry, &mut Vec<LintResult> | `` | — | — | Check fn count limit. |
| `check_entity` | &FileEntry, &mut Vec<LintResult> | `` | — | — | Check entity. |
| `check_error` | &FileEntry, &mut Vec<LintResult> | `` | — | — | Check error. |
| `check_event` | &FileEntry, &mut Vec<LintResult> | `` | — | — | Check event. |
| `check_constant` | &FileEntry, &mut Vec<LintResult> | `` | — | — | Check constant. |
| `check_utility_convention` | &FileEntry, &mut Vec<LintResult> | `` | — | — | Check utility convention. |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
|---|---|---|---|---|---|
| `execute` | RoleRequest | `RoleResponse` | — | — | Single composite entry point over the feature. |

## Reference

- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature; this file is specification only.
- PRD: [PRD.md](../../PRD.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
- Filesystem crate
- Shared crate

| Roadmap | {Link to ROADMAP.md} | [{link}](ROADMAP.md) |
