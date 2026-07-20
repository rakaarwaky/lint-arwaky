# AES Architecture: Agentic Engineering System

See [AGENTS.md](../AGENTS.md) for workspace conventions and [RULES_AES.md](../.agents/rules/RULES_AES.md) for the rule catalog (AESxxx codes referenced below).

The **Agentic Engineering System (AES)** is a strictly layered, decoupled, AI-native architectural pattern. It isolates domain models, keeps business logic free of low-level detail, and makes the codebase easy for humans and LLMs to navigate without architectural violations.

The seven layers, from foundation to user-facing, are: **Taxonomy → Contract → Utility → Capabilities → Agent → Surface → Root**.

## Terminology

| Term         | Language        | Definition                                                                                  |
| ------------ | --------------- | ------------------------------------------------------------------------------------------- |
| Workspace    | All             | Project root (e.g. `lint-arwaky/`) containing all configs and language sub-projects.        |
| `crates/`    | Rust            | Rust workspace members (Cargo).                                                             |
| `packages/`  | TypeScript / JS | TypeScript/JavaScript packages (npm/pnpm workspace).                                        |
| `modules/`   | Python          | Python sub-projects (independent modules).                                                  |
| Member       | All             | One self-contained sub-project (crate, package, or module) inside the workspace.            |

## Core Pillars

### 1. Strict Layered Boundary Enforcement

Layers communicate only along downward dependency paths to prevent coupling and circular dependencies. A layer may import layers below it, never above it.

### 2. The 3-Structure Naming Convention

Every source file is named `[layer]_[concept]_[suffix]` (or `[layer]_[concept1]_[concept2]_[suffix]`). The layer prefix is the architectural boundary; all seven layers coexist inside one feature crate, distinguished by prefix — not by directory.

1. **Layer (prefix)**: `taxonomy_`, `contract_`, `utility_`, `capabilities_`, `agent_`, `surface_`, `root_`.
2. **Concept (middle)**: the core concept, e.g. `import_rule`.
3. **Role (suffix)**: the architectural role, e.g. `_checker`, `_protocol`.

## Layer Catalog

Numbered foundation → user-facing. Each layer imports only the layers listed.

| # | Layer       | Role                                                            | Imports                     |
| - | ----------- | --------------------------------------------------------------- | --------------------------- |
| 1 | Taxonomy    | Pure domain models (VO/entity/error/constant).                  | — (foundation)              |
| 2 | Contract    | Inbound interfaces (`_protocol`) and facades (`_aggregate`).    | Taxonomy                    |
| 3 | Utility     | Standalone technical functions (no struct/trait/contract).      | Taxonomy                    |
| 4 | Capabilities| Business logic, use-cases, computations.                         | Contract, Utility, Taxonomy |
| 5 | Agent       | Orchestration / pipeline flow control.                           | Contract, Taxonomy          |
| 6 | Surface     | User-facing entry points (CLI, TUI, MCP, API).                  | Contract-aggregate, Taxonomy|
| 7 | Root        | Wiring / bootstrap. No business logic.                          | All layers                  |

**Dependency direction (user-facing → foundation):** Surface → Agent → Capabilities → Utility → Contract → Taxonomy. Root wraps all.

## Layer Prefix Specifications

| Prefix         | Allowed Suffixes                                                                                                                                                                                                                                                                                                                                                         | Allowed Imports                              | Semantic Role                                                              |
| -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------- | -------------------------------------------------------------------------- |
| `taxonomy_`    | `_vo`, `_entity`, `_event`, `_error`, `_constant` (strict)                                                                                                                                                                                                                                                                                                               | `taxonomy_` only                             | Pure domain models, value objects, events, errors, constants.             |
| `contract_`    | `_protocol`, `_aggregate`                                                                                                                                                                                                                                                                                                                                               | `taxonomy_`, `contract_`                     | Inbound interfaces and composition facades.                               |
| `utility_`     | `_utility`, `_helper`, `_walker`, `_scanner`, `_runner`, `_parser`, `_normalizer`, `_matcher`                                                                                                                                                                                                                                                                            | `taxonomy_` only                             | Standalone technical functions. No struct/trait/contract.                 |
| `capabilities_`| `_analyzer`, `_checker`, `_processor`, `_evaluator`, `_resolver`, `_validator`, `_formatter`, `_executor`, `_transformer`, `_calculator`, `_builder`, `_compiler`, `_classifier`, `_extractor`, `_reporter`, `_mapper`, `_filter`, `_collector`, `_comparator`, `_scorer`, `_inspector`, `_reviewer`, `_assessor`, `_auditor` | `taxonomy_`, `contract_`, `utility_`         | Business logic. Calls Utility by name.                                    |
| `agent_`       | `_orchestrator`                                                                                                                                                                                                                                                                                                                                                         | `taxonomy_`, `contract_`                     | Coordinates capabilities flows into pipelines.                             |
| `surface_`     | `_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_hook`, `_store`, `_action`, `_screen`                                                                                                                                                                                                                                                  | varies by surface role (see Surface)         | CLI/TUI/MCP/API entry points.                                             |
| `root_`        | `_container`, `_entry`                                                                                                                                                                                                                                                                                                                                                  | all layers                                   | Composition root and binary entry. No business logic.                     |

## Layer Specifications

### 1. Taxonomy (`taxonomy_`)

Pure domain models and value objects.

- **Value Object (`_vo`)**: immutable data container (primitives allowed internally, AES401).
- **Entity (`_entity`)**: stateful domain concept with a unique ID.
- **Event (`_event`)**: immutable domain fact snapshot.
- **Error (`_error`)**: domain-level exception.
- **Constant (`_constant`)**: compile-time literal (AES401).

### 2. Contract (`contract_`)

Abstract interfaces — _what_ without _how_. Two components:

- **Protocol (`_protocol`)**: inbound interface implemented by Capabilities. Callers (Agent/Surface) depend on it, not on the implementation.
- **Aggregate (`_aggregate`)**: composition facade. Surfaces reach Capabilities only through it (AES201), never by direct import.

### 3. Utility (`utility_`)

Standalone technical implementation. Owns all low-level detail as free functions.

**Rules:**
- Free functions (`pub fn`). No `struct`, no `trait`, no `impl` of any contract.
- Imports Taxonomy only — never references `_protocol` / `_aggregate` or Capabilities.
- Called by name from Capabilities: `collect_source_files(dir)`, `run_ruff(path)`, `normalize_path(p)`.
- Performs mechanical work and returns data; decides no policy.

| Concern                 | Example Suffix      | Examples                                  |
| ----------------------- | ------------------- | ----------------------------------------- |
| File I/O detail         | `_walker`, `_utility` | dir walking, ignore-matching, detection   |
| External tool execution | `_runner`, `_helper`   | spawn ruff/clippy/eslint/tsc, parse output|
| Algorithm / regex       | `_matcher`, `_parser` | string/array/regex manipulation           |
| Path / normalization    | `_normalizer`, `_utility`| cross-platform path handling            |
| System operations       | `_helper`, `_utility`  | process / env handling                    |

### 4. Capabilities (`capabilities_`)

Business logic, use-cases, computations. Calls Utility by name; contains no low-level detail.

| Concern                 | Example Suffix            |
| ----------------------- | ------------------------- |
| Computation / scoring   | `_calculator`, `_scorer`  |
| Validation / checking   | `_checker`, `_validator`  |
| Data transformation     | `_transformer`, `_mapper` |
| Business rules          | `_evaluator`, `_resolver` |
| Information extraction  | `_extractor`, `_classifier`|
| Assessment / scoring    | `_assessor`, `_auditor`, `_inspector` |

**Forbidden:** flow control / orchestration → Agent; domain models → Taxonomy; low-level detail (algorithms, regex, I/O, library wiring) → Utility.

### 5. Agent (`agent_`)

Orchestration and pipeline execution. Holds protocol references via Contract; sequences fetch → branch → loop → handle errors → return.

Allowed flow-control syntax: `while`/`loop`/`for`, sequential statements, `if/else`/`match`, `match`/`if let`/`?` (or `try/except`), `tokio::select!`/`asyncio.wait_for()` for cancellation.

**Forbidden:** computation / business rules → Capabilities; low-level detail → Utility; domain models → Taxonomy.

### 6. Surface (`surface_`)

User-facing entry points — CLI, TUI, MCP server, API.

- **Smart** (`command`/`controller`/`page`/`entry`): `taxonomy_` + `contract_aggregate_` only (AES201). Never import Capabilities/Utility/Agent directly.
- **Utility** (`hook`/`store`/`action`/`screen`): `taxonomy_` only (AES406).
- **Passive** (`component`/`view`/`layout`): `taxonomy_` only (AES406).

Allowed: input handling, UI rendering, event mapping, state, routing. **Forbidden:** business rules → Capabilities; orchestration → Agent; low-level detail → Utility; direct import of Capabilities/Utility → AES201.

### 7. Root (`root_`)

Wiring and bootstrap. Instantiates Capabilities and wires them behind Contract protocols/aggregates; each feature crate owns one `root_*_container`. The `_entry` binary builds the composition root and starts the main loop. No business logic.

## Concrete Example

A feature crate, showing the seven prefixes side by side:

```
crates/import-rules/src/
  taxonomy_import_rule_vo.rs          # 1. domain model
  contract_import_rule_protocol.rs     # 2. inbound interface
  utility_import_rule_walker.rs        # 3. low-level scan (free fn)
  capabilities_import_rule_checker.rs  # 4. business logic (calls the walker)
  agent_import_rule_orchestrator.rs    # 5. pipeline
  surface_import_rule_command.rs       # 6. CLI command
  root_import_rule_container.rs         # 7. wiring
```

## Quick Reference: Where Logic Lives

| Logic Type               | Layer          | Suffix                          |
| ------------------------ | -------------- | ------------------------------- |
| User input / UI / routing| `surface_`     | `_command`, `_view`, `_router`  |
| Computation / scoring    | `capabilities_`| `_calculator`, `_scorer`        |
| Validation / rules       | `capabilities_`| `_checker`, `_validator`        |
| Low-level detail         | `utility_`     | `_utility`, `_walker`, `_runner`|
| Regex / parsing          | `utility_`     | `_matcher`, `_parser`           |
| Looping / branching      | `agent_`       | `_orchestrator`                 |
| Domain models            | `taxonomy_`    | `_vo`, `_entity`                |
| Inbound interfaces       | `contract_`    | `_protocol`                     |
| Facades                  | `contract_`    | `_aggregate`                    |
| Wiring / bootstrap       | `root_`        | `_container`, `_entry`          |

## Patterns

**Capabilities → Utility.** Business-logic file calls one clearly named utility function; the utility owns the messy detail.

```
// capabilities_import_rule_checker.rs
let files = collect_source_files(dir);   // one readable line

// utility_import_rule_walker.rs
// recursive walk, ignore-matching, extension check, symlink/inode handling
```

**Data flow:** Surface → Agent → Capabilities → Utility → Taxonomy. Surface maps input via the aggregate; Agent delegates to the executor; Capabilities runs logic and calls Utility for low-level work; Utility operates on Taxonomy value objects.

**Wiring:** Root wires Capabilities behind Contract protocols/aggregates. Surfaces reach Capabilities only through the aggregate (AES201). No port/DI-through-trait abstraction.
