# FRD — role-rules

## Feature Goal

The role-rules crate enforces architectural boundaries and responsibility rules for each layer (Taxonomy, Contract, Utility, Capabilities, Agent, Surface, Root) as defined by the 7-layer AES architecture. It audits files based on their filename prefix and applies layer-specific checks, ensuring components behave exactly according to their architectural roles.

## Scope

The crate covers **6 active AES rules** (AES401–AES406) dispatched via `RoleOrchestrator`, which classifies files by their filename prefix and routes them to the appropriate role checker. Root layer files are skipped (pure DI wiring only).

**Supported languages:** Rust (.rs), Python (.py), TypeScript (.ts/.tsx), JavaScript (.js/.jsx)

**Supported file extensions:** `.rs`, `.py`, `.js`, `.ts`, `.jsx`, `.tsx`

## Requirements

### AES401 — Taxonomy Purity and Primitive Restriction

**Scope:** Files with `taxonomy_` prefix.

| Check | What It Audits |
|-------|----------------|
| `check_entity` | Scans `_entity` files for raw primitive types in type annotations (fields, return types). Primitives detected: `String`, `i32`, `bool`, `Vec<`, `HashMap<`, etc. (Rust); `str`, `int`, `float`, `bool`, `list`, `dict`, etc. (Python); `string`, `number`, `boolean`, `any`, etc. (JS/TS) |
| `check_error` | Same primitive scan on `_error` files |
| `check_event` | Same primitive scan on `_event` files |
| `check_constant` | Ensures `_constant.rs` / `_constant.py` files contain **only** pure constant declarations (`pub const`, `pub static` in Rust). Flags any struct, enum, fn, impl, mod, trait, class, or type alias. |

**Skip rules:** Lines starting with `class `, `pub struct `, `struct ` are excluded from primitive scan (they are type definitions, not field annotations). Lines containing `pub(crate) value:` or `pub value:` are excluded (internal VO wrappers). Lines starting with `fn from(` or `fn visit_` are also excluded.

### AES402 — Contract Primitive Restriction

**Scope:** Files with `contract_` prefix.

| Check | What It Audits |
|-------|----------------|
| `check_protocol` | Detects raw primitive types in method signatures of `_protocol` files |
| `check_aggregate` | Same check on `_aggregate` files |

**Detection method:** Extracts method signatures via shared utility functions (`extract_trait_method_signatures` for Rust, `extract_python_method_signatures` for Python, `extract_typescript_method_signatures` for TypeScript), then checks each signature for forbidden primitive types.

### AES403 — Capability Protocol Implementation

**Scope:** Files with `capabilities_` or `capability_` prefix.

**Three rules enforced:**

| Rule | Condition | Violation |
|------|-----------|-----------|
| Rule 1 | File must `import` from a `_protocol` module | `CapabilityNoProtocol` |
| Rule 2 | At least 1 struct/class must implement the imported protocol (`impl Trait for Struct` in Rust, `class Name(Protocol)` in Python, `class Name implements IProtocol` in TS) | `CapabilityNoImplementor` |
| Rule 3 | Max 3 type declarations (struct/enum/class/interface) per file | `CapabilityTooManyTypes` |

**Internal helper types** (structs without protocol impl) are allowed and not flagged.

### AES404 — Utility Purity

**Scope:** Files with `utility_` prefix.

**What is forbidden per language:**

| Language | Forbidden constructs |
|----------|---------------------|
| Rust | `pub struct`, `pub enum` |
| TypeScript | `export class`, `export interface`, `export enum`, `export type` |
| Python | `class `, `def ` (any function definition) |

**Noise stripping before detection:**
- Rust: line comments (`//`), block comments (`/* */`), `macro_rules!` bodies
- TypeScript: line comments (`//`), block comments (`/* */`), template literals (`` ` ``)
- Python: line comments (`#`), docstrings (`"""` / `'''`)

### AES405 — Agent Orchestrator Composition

**Scope:** Files with `agent_` prefix.

**Three rules enforced:**

| Rule | Condition | Violation |
|------|-----------|-----------|
| Rule 1 | File must `import` from an `_aggregate` module | `AgentNoAggregate` |
| Rule 2 | At least 1 struct/class must implement the imported aggregate (`impl Trait for Struct` in Rust, `class Name(AggregateABC)` in Python, `class Name implements IAggregate` in TS) | `AgentNoImplementor` |
| Rule 3 | Max 3 type declarations (struct/enum/class/interface) per file | `AgentTooManyTypes` |

**Internal helper types** (structs without aggregate impl) are allowed and not flagged.

### AES406 — Surface Passive Role

**Scope:** Files with `surfaces_` or `surface_` prefix, or files inside `surfaces/` / `surface/` / `cli_commands/` directories.

**Surface classification by filename suffix:**

| Group | Filename suffixes | Expected behavior |
|-------|-------------------|-------------------|
| Smart | `_command`, `_controller`, `_page`, `_entry` | May contain orchestration logic (exempted from passive checks) |
| Utility | `_hook`, `_store`, `_action`, `_screen`, `_router` | Support smart surfaces, checked for passive constraints |
| Passive | All other surface files | Presentation-only, strict passive checks |

**Checks applied:**

| Check | What It Audits | Threshold |
|-------|----------------|-----------|
| `check_fn_count_limit` | Counts `fn` / `def` / `function` occurrences per file | Max 15 functions |
| `check_surface_hierarchy` (passive) | Scans class/impl blocks for excessive public methods | Max 10 public methods per class/impl |
| | Checks method body length | Max 80 lines per method |
| | Checks if-nesting depth | Max 3 levels of `if` nesting |
| `check_surface_roles` (async, LayerMap-dependent) | Counts control-flow statements (`if`, `else`, `for`, `while`, `match`, `switch`, `try`, `except`, `catch`) in non-smart surfaces | Max 3 control-flow statements → flagged as `NoDomainLogic` violation |

### RoleOrchestrator — File Dispatch

The `RoleOrchestrator` is the central entry point (`IRoleRunnerAggregate`). It:

1. Collects files from target path (walks directories, filters by supported extensions)
2. Applies ignore paths from `ArchitectureConfig`
3. Extracts filename prefix (first `_`-separated segment) to determine layer
4. Dispatches to the corresponding role checker:

| Prefix | Dispatcher |
|--------|------------|
| `agent` | `AgentRoleChecker.check_agent_routing` |
| `surfaces` / `surface` | `SurfaceRoleChecker.check_fn_count_limit` + smart/utility/passive classification |
| `contract` | `ContractRoleChecker.check_protocol` or `check_aggregate` (based on `_protocol`/`_aggregate` in filename) |
| `capabilities` / `capability` | `CapabilitiesRoleChecker.check_capability_routing` |
| `utility` | `UtilityRoleChecker.check_utility_convention` |
| `taxonomy` | `TaxonomyRoleChecker` (all 4 checks: entity, error, event, constant) |
| `root` | Skipped (pure DI wiring) |

## Success Indicators

- [ ] All 6 AES rules (AES401–AES406) are audited with precise line-level violation reporting
- [ ] Multi-language support: Rust, Python, TypeScript, JavaScript all produce accurate violations
- [ ] Smart surfaces (`_command`, `_controller`, `_page`, `_entry`) are correctly exempted from passive checks
- [ ] Capability and Agent layers enforce the 3-rule composition pattern (protocol/aggregate import, implementor, max 3 types)
- [ ] Utility files are confirmed free of type definitions (struct/class/export) after noise stripping
- [ ] Config-driven ignore paths and enable/disable toggle work correctly
