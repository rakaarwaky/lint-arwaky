# AES Architecture: Agentic Engineering System

The **Agentic Engineering System (AES)** is a strictly layered, highly decoupled, and AI-native architectural pattern. It is designed to achieve maximum modularity, absolute testability, and extreme maintainability by enforcing rigid structural boundaries. Under the AES paradigm, technical details are isolated, domain models are protected, and dependencies are strictly inverted via abstract contracts. Furthermore, AES is specifically optimized for **Agentic workflows**, ensuring that AI agents and LLMs can easily navigate, understand, and modify the codebase without hallucinating architectural violations.

## Core Pillars and Philosophy

### 1. Strict Layered Boundary Enforcement

The codebase is divided into distinct horizontal and vertical boundaries. Layers can only communicate using downward-only dependency paths to prevent coupling and circular dependencies. Any violation of these import boundaries is caught at compile or lint time by static analysis checkers.

### 2. Sibling Equivalence and Peer Layers

Unlike traditional three-tier architectures, **Capabilities** and **Infrastructure** are horizontal peer layers.

- Neither layer is above or below the other.
- Neither layer can ever import from or know about the other.
- Both layers depend downward on the **Contract** layer exclusively via Ports and Protocols.

### 3. Dependency Inversion

Higher-level orchestrating layers never import concrete implementations. Instead, they interact with implementations exclusively through interfaces declared in the Contract layer using Dependency Injection (e.g., Surfaces call `ServiceContainerAggregate`, not concrete Orchestrators).

### 4. The 3-Structure Naming Philosophy (Layer Prefix + Vertical Slicing + Role Suffix)

AES enforces a **Word File Naming Convention**: `[layer]_[concept]_[suffix]` or `[layer]_[concept1]_[concept2]_[suffix]`

1. **Layer (prefix)**: The architectural layer (e.g., `contract_`, `capabilities_`, `taxonomy_`).
2. **Concept (middle)**: A single/multiple word defining the core concept (e.g., `compliance`, `import_rule`).
3. **Role (suffix)**: Defines the architectural role (e.g., `_port`, `_protocol`, `_checker`).

_Example:_ `contract_compliance_port.rs` = layer=contract, concept=compliance, suffix=port.

Files are organized into **feature crates** (vertical slicing) rather than layer directories. All six layers coexist in each feature crate, distinguished by their file prefix.

_Example feature crate `import-rules/` — all 6 layers in one crate:_

```
contract_import_parser_port.rs           ← contract layer
contract_import_runner_aggregate.rs       ← contract layer
capabilities_import_mandatory_checker.rs  ← capabilities layer
capabilities_import_forbidden_checker.rs  ← capabilities layer
capabilities_import_intent_checker.rs     ← capabilities layer
capabilities_layer_detection_analyzer.rs  ← capabilities layer
infrastructure_import_parser_adapter.rs   ← infrastructure layer
agent_import_orchestrator.rs              ← agent layer
taxonomy_import_rule_vo.rs                ← taxonomy layer
```

Exceptions: `main.rs`, `lib.rs`, `mod.rs`, `__init__.py`, `index.ts`, `index.js`.

---

### Layer Hierarchy (Dependency Direction)

```mermaid
%%{init: {'theme': 'default'}}%%
graph TD
    subgraph ROOT["root_  ── Wiring Layer (wraps all layers)"]
        direction TB

        S["surface_<br/>(CLI, MCP Server, API)"]
        A["agent_<br/>(Orchestrators)"]

        subgraph PEER["Peer Layers (no direct sibling import)"]
            direction LR
            C["capabilities_<br/>(Checkers, Analyzers)"]
            I["infrastructure_<br/>(Adapters, Scanners)"]
        end

        CT["contract_<br/>(Ports, Protocols, Aggregates)"]
        T["taxonomy_<br/>(VOs, Entities, Errors, Events, Constants)"]

        S -->|"imports"| CT
        S -->|"imports"| T
        A -->|"imports"| CT
        A -->|"imports"| T
        C -->|"imports"| CT
        C -->|"imports"| T
        I -->|"imports"| CT
        I -->|"imports"| T
        CT -->|"imports"| T
    end

    ROOT_CONT["root_container<br/>(DI Wiring — instantiates & injects all)"]
    ROOT_ENTRY["root_entry<br/>(Binary Bootstrap)"]

    ROOT_CONT -->|"wires"| ROOT
    ROOT_ENTRY -->|"starts"| ROOT_CONT
```

#### Layer Prefix Specifications

Files use the layer as a **file prefix** (not a directory): `[layer]_[concept]_[suffix].rs`. All six layers coexist in each feature crate, distinguished by their prefix.

| Layer (prefix)      | Allowed suffixes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `taxonomy_`       | `_vo`, `_entity`, `_event`, `_error`, `_constant`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `contract_`       | `_port`, `_protocol`, `_aggregate`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `capabilities_`   | `_analyzer`, `_checker`, `_processor`, `_evaluator`, `_resolver`, `_validator`, `_formatter`, `_executor`, `_transformer`, `_calculator`, `_builder`, `_compiler`, `_aggregator`, `_classifier`, `_extractor`, `_reporter`, `_mapper`, `_filter`, `_collector`, `_comparator`, `_scorer`, `_inspector`, `_reviewer`, `_assessor`, `_actions`                                                                                                                                                  |
| `infrastructure_` | `_adapter`, `_provider`, `_scanner`, `_client`, `_constants`, `_schemas`, `_lifespan`, `_wrapper`, `_tracer`, `_tracker`, `_variants`, `_detector`, `_patterns`, `_util`, `_system`, `_repository`, `_cache`, `_loader`, `_writer`, `_reader`, `_driver`, `_connector`, `_gateway`, `_serializer`, `_encoder`, `_decoder`, `_fetcher`, `_watcher`, `_indexer`, `_dispatcher`, `_recorder`, `_proxy`, `_publisher`, `_subscriber`, `_listener`, `_poller`, `_streamer` |
| `agent_`          | `_orchestrator`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `surface_`        | `_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_entry`, `_hook`, `_store`, `_action`, `_screen`                                                                                                                                                                                                                                                                                                                                                                                      |
| `root_`           | `_container`, `_entry`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |

### Feature Crates (Workspace Crates & Members)

```
crates/
  shared/               — Foundation: ALL taxonomy_* + contract_* (NO deps on feature crates)
  import-rules/         — Import compliance (AES001, AES002)
  naming-rules/         — Naming convention & variants (AES010, AES011)
  role-rules/           — Role violations (AES0305, AES0306)
  orphan-detector/      — Orphan code detection (AES030)
  code-analysis/        — Quality: unused (AES023), class/line, auto-fix
  auto-fix/             — Auto-fix processor (AES0303)
  config-system/        — Config loading & parsing
  pipeline-jobs/        — Jobs, dispatcher, execution
  source-parsing/       — Source code parsing (scanners, parsers)
  language-adapters/    — Python, JS, Rust linter adapters
  file-system/          — File system abstraction
  file-watch/           — File watching
  git-hooks/            — Git hooks management
  multi-project/        — Multi-project governance
  project-setup/        — Project init, doctor, mcp-config
  plugin-system/        — Plugin discovery & management
  output-report/        — Output formatting & report generation
  lifecycle-state/      — Agent lifecycle management
  metrics-service/      — Metrics provider
  cli-commands/         — CLI surfaces (_command) + transport
  mcp-server/           — MCP server surfaces
  root_composition_container.rs — Root composition (root layer)
  root_cli_main_entry.rs       — CLI binary entry (root_entry)
  root_mcp_main_entry.rs       — MCP binary entry (root_entry)
  root_tui_main_entry.rs       — TUI binary entry (root_entry)
```

### Layer Specifications

#### 1. Taxonomy (`taxonomy_` prefix)

- **Prefix**: `taxonomy_`
- **Allowed Suffixes**: `_vo`, `_entity`, `_event`, `_error`, `_constant`
- **Allowed Imports**: Other `taxonomy_` files only. Outer imports trigger **AES001**.
- **Description**: Pure domain models, value objects, and business entities.
- **Components**:
  - **Value Object (`_vo`)**: Immutable data containers. Primitive types forbidden (**AES016**). _Ex: `taxonomy_import_rule_vo.rs`_
  - **Entity (`_entity`)**: Stateful domain concepts with unique IDs. _Ex: `taxonomy_governance_entity.rs`_
  - **Event (`_event`)**: Immutable domain fact snapshots. _Ex: `taxonomy_fix_applied_event.rs`_
  - **Error (`_error`)**: Domain-level exceptions. _Ex: `taxonomy_system_error.rs`_
  - **Constant (`_constant`)**: Compile-time literals only (**AES015**). _Ex: `taxonomy_layer_names_constant.rs`_

#### 2. Contract (`contract_` prefix)

- **Prefix**: `contract_`
- **Allowed Suffixes**: `_port`, `_protocol`, `_aggregate`
- **Allowed Imports**: `taxonomy_` files and other `contract_` files. Implementation layers forbidden.
- **Description**: Interface definitions — _what_ can be done without _how_.
- **Components**:
  - **Port (`_port`)**: Outbound interfaces implemented by Infrastructure. _Ex: `contract_system_port.rs`_
  - **Protocol (`_protocol`)**: Inbound interfaces implemented by Capabilities. _Ex: `contract_rule_protocol.rs`_
  - **Aggregate (`_aggregate`)**: Composition facades. _Ex: `contract_service_aggregate.rs`_

#### 3. Capabilities (`capabilities_` prefix)

- **Prefix**: `capabilities_`
- **Allowed Suffixes**: `_checker`, `_analyzer`, `_processor`, etc.
- **Allowed Imports**: `taxonomy_` and `contract_` files only.
- **Description**: Use-case logic. Entirely agnostic of infrastructure.

#### 4. Infrastructure (`infrastructure_` prefix)

- **Prefix**: `infrastructure_`
- **Allowed Suffixes**: `_adapter`, `_provider`, `_scanner`, etc.
- **Allowed Imports**: `taxonomy_` and `contract_` files only.
- **Description**: Technical implementations, external tool wrappers.

#### 5. Agent (`agent_` prefix)

- **Prefix**: `agent_`
- **Allowed Suffixes**: `_orchestrator`
- **Allowed Imports**: `taxonomy_` + `contract_` only (AES0305). Must NOT import capabilities/infrastructure directly.
- **Description**: Orchestration and pipeline execution.

#### 6. Surfaces (`surface_` prefix)

- **Prefix**: `surface_`
- **Allowed Suffixes**: `_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_entry`, `_hook`, `_store`, `_action`, `_screen`
- **Allowed Imports**: Depends on role:
  - Smart surfaces (`command`/`controller`/`page`/`entry`): `taxonomy_` + `contract_aggregate_` only (AES001). Must NOT import capabilities/infrastructure/agent directly — use `ServiceContainerAggregate`.
  - Utility surfaces (`hook`/`store`/`action`/`screen`): `taxonomy_` only + passive surfaces. Must NOT import smart surfaces (AES0306).
  - Passive surfaces (`component`/`view`/`layout`): `taxonomy_` only (AES0306). No logic or orchestration.
- **Description**: CLI and MCP server entry points.

#### 7. Root (`root_` prefix)

- **Prefix**: `root_`
- **Allowed Suffixes**: `_container`, `_entry`
- **Allowed Imports**: All layers — `taxonomy_`, `contract_`, `capabilities_`, `infrastructure_`, `agent_`, `surface_`. This is the **only** layer permitted to import across all other layers.
- **Description**: Wiring layer. Responsible for Dependency Injection (DI) composition. No business logic is allowed here — only instantiation and wiring.
- **Components**:
  - **Container (`_container`)**: Per-feature DI container. Instantiates `infrastructure_*` and `capabilities_*` implementations, wires them behind `contract_*` traits, and exposes typed factory methods (e.g., `orchestrator()`, `source_parser()`). Each feature crate owns exactly one `root_*_container.rs`. _Ex: `root_source_parsing_container.rs`_
  - **Entry (`_entry`)**: Binary entry point. Bootstraps the application by creating the `CompositionRoot` (the top-level root container that composes all feature containers) and starts the main loop. _Ex: `root_cli_main_entry.rs`, `root_mcp_main_entry.rs`_
- **Rules**:
  - A `root_container` **must NOT** contain business logic — only `new()` calls and trait-object wiring.
  - Infrastructure and capabilities files **must NOT** import each other directly — all cross-infra/cross-capabilities wiring happens exclusively in the container.
  - `root_entry` files must only call into the `CompositionRoot` and are forbidden from importing feature crate internals directly.

---

## Multi-Crate Workspace Enforcement

The AES layer rules are enforced at two levels:

**Dependency rules per crate type:**

| Crate Type                             | Example                                                                       | Allowed Deps             |
| -------------------------------------- | ----------------------------------------------------------------------------- | ------------------------ |
| **Foundation**                   | `shared`                                                                    | NONE (only std/external) |
| **Feature (capabilities/infra)** | `import-rules`, `naming-rules`, `language-adapters`, `source-parsing` | `shared`               |
| **Feature (agent)**              | `code-analysis`, `auto-fix`, `pipeline-jobs`                            | `shared`              |
| **Surface**                      | `cli-commands`, `mcp-server`                                              | `shared`              |
| **Composition**                  | `root_composition_container.rs`                                             | All feature crates       |
