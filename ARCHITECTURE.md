# AES Architecture: Agentic Engineering System

See [AGENTS.md](../AGENTS.md) for workspace conventions and [RULES_AES.md](../.agents/rules/RULES_AES.md) for the full rule catalog.

The **Agentic Engineering System (AES)** is a strictly layered, highly decoupled, and AI-native architectural pattern. It is designed to achieve maximum modularity, absolute testability, and extreme maintainability by enforcing rigid structural boundaries. Under the AES paradigm, technical details are isolated, domain models are protected, and dependencies are strictly inverted via abstract contracts. Furthermore, AES is specifically optimized for **Agentic workflows**, ensuring that AI agents and LLMs can easily navigate, understand, and modify the codebase without hallucinating architectural violations.

---

## Terminology

AES supports multiple languages (Rust, TypeScript, Python) to maintain a single unified vocabulary:

| Term          | Language        | Definition                                                                                                         |
| ------------- | --------------- | ------------------------------------------------------------------------------------------------------------------ |
| **Workspace** | All             | The entire project root directory (e.g.,`lint-arwaky/`) containing all configs and language-specific sub-projects. |
| `crates/`     | Rust            | The directory containing all Rust crates (workspace members), conforming to Cargo workspace specifications.        |
| `packages/`   | TypeScript / JS | The directory containing all TypeScript/JavaScript packages, following npm/pnpm workspace conventions.             |
| `modules/`    | Python          | The directory containing all Python sub-projects, organized as independent python modules.                         |
| Member        | All             | A single, self-contained sub-project (crate, package, or module) inside the workspace.                             |

## Core Pillars and Philosophy

### 1. Strict Layered Boundary Enforcement

The codebase is divided into distinct horizontal and vertical boundaries. Layers can only communicate using downward-only dependency paths to prevent coupling and circular dependencies. Any violation of these import boundaries is caught at compile or lint time by static analysis checkers.

### 2. Single Implementation Layer (Capabilities)

Under AES the **Capabilities** layer is the single implementation layer under **Contract**. Capabilities owns all non-trivial logic: domain use-cases/business logic AND technical I/O (adapters, scanners, external-tool wrappers). There is no separate Infrastructure layer — technical I/O is a sub-concern of Capabilities (group B), not a peer layer. Capabilities depends downward on Contract and Taxonomy only.

### 3. Dependency Inversion

Higher-level orchestrating layers never import concrete implementations. Instead, they interact with implementations exclusively through interfaces declared in the Contract layer using Dependency Injection (e.g., Surfaces call `ServiceContainerAggregate`, not concrete Orchestrators).

### 4. The 3-Structure Naming Philosophy (Layer Prefix + Vertical Slicing + Role Suffix)

AES enforces a **File Naming Convention**: `[layer]_[concept]_[suffix]` or `[layer]_[concept1]_[concept2]_[suffix]` if needed

1. **Layer (prefix)**: The architectural layer (e.g., `contract_`, `capabilities_`, `taxonomy_`).
2. **Concept (middle)**: A single/multiple word defining the core concept (e.g., `compliance`, `import_rule`).
3. **Role (suffix)**: Defines the architectural role (e.g., `_port`, `_protocol`, `_checker`).

Files are organized into **feature crates** (vertical slicing) rather than layer directories. All layers coexist in each feature crate, distinguished by their file prefix.

Exceptions: `main.rs/py/ts`, `lib.rs/py/ts`, `mod.rs`, `__init__.py`, `index.ts`.

---

## Layer Hierarchy (Dependency Direction)

The dependency hierarchy flows as follows:

- **Root** layer wraps all other layers and contains DI wiring.
- **Surface** layer (CLI, MCP Server, API) imports from Contract and Taxonomy only.
- **Agent** layer (Orchestrators) imports from Contract and Taxonomy only.
- **Capabilities** layer (Checkers, Analyzers, Adapters, Scanners, Utilities) imports from Contract and Taxonomy only. Capabilities is the single implementation layer — it owns both domain logic and technical I/O, and delegates pure file/system operations to `taxonomy_*_utility` free functions.
- **Contract** layer (Ports, Protocols, Aggregates) imports from Taxonomy only.
- **Taxonomy** layer (VOs, Entities, Errors, Events, Constants) is the foundation layer with no imports.

The Root container handles DI wiring that instantiates and injects all layers. The Root entry point bootstraps the application by creating the composition root and starting the main loop.

## The Six Layers

1. **Taxonomy** — pure domain models (VO/entity/error/constant). Foundation, no imports.
2. **Contract** — abstract interfaces (port / protocol / aggregate). Imports Taxonomy only.
3. **Capabilities** — the single implementation layer. Domain logic (A) + technical I/O (B) + utility helpers. Imports Taxonomy + Contract only.
4. **Agent** — orchestration / pipeline flow control. Imports Taxonomy + Contract only.
5. **Surface** — user-facing entry points (CLI, TUI, MCP, API). Imports Taxonomy + Contract-aggregate only.
6. **Root** — DI wiring / composition. No business logic.

## Layer Prefix Specifications

Files use the layer as a **file prefix** (not a directory): `[layer]_[concept]_[suffix]`.

| Layer Prefix    | Allowed Suffixes                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Allowed Imports                                                                                  | Semantic Role / Description                                                                       |
| :-------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------ |
| `taxonomy_`     | `_vo`, `_entity`, `_event`, `_error`, `_constant` (strict)                                                                                                                                                                                                                                                                                                                                                                                                            | `taxonomy_` files only (outer imports trigger **AES201**).                                       | Pure domain models, value objects, domain events, errors, and compile-time constants.             |
| `contract_`     | `_port`, `_protocol`, `_aggregate`                                                                                                                                                                                                                                                                                                                                                                                                                                    | `taxonomy_`, `contract_`                                                                         | Abstract interfaces: Outbound interface ports, inbound protocols, and facade aggregates.          |
| `capabilities_` | `_analyzer`, `_checker`, `_processor`, `_evaluator`, `_resolver`, `_validator`, `_formatter`, `_executor`, `_transformer`, `_calculator`, `_builder`, `_compiler`, `_classifier`, `_extractor`, `_reporter`, `_mapper`, `_filter`, `_collector`, `_comparator`, `_scorer`, `_inspector`, `_reviewer`, `_assessor`, `_auditor`, `_adapter`, `_provider`, `_scanner`, `_client`, `_wrapper`, `_system`, `_utility`, `_helper`                                                                                                                  | `taxonomy_`, `contract_`                                                                         | **(A) Domain use-cases, business logic, computations** AND **(B) technical I/O implementations** (adapters, scanners, external-tool wrappers). Delegates pure reusable logic to `taxonomy_*_utility`. |
| `agent_`        | `_orchestrator`                                                                                                                                                                                                                                                                                                                                                                                                                                                       | `taxonomy_`, `contract_`                                                                         | Coordinates multiple capabilities flows to execute pipelines/workflows.                           |
| `surface_`      | `_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_hook`, `_store`, `_action`, `_screen`                                                                                                                                                                                                                                                                                                                                              | Varies by surface role (see Surface layer details below).                                        | Application entry points, UI components, CLI commands, controllers, and pages.                    |
| `root_`         | `_container`, `_entry`                                                                                                                                                                                                                                                                                                                                                                                                                                                | All layers (`taxonomy_`, `contract_`, `capabilities_`, `agent_`, `surface_`).                   | App bootstrap, inline composition, and Dependency Injection wiring. Absolutely no business logic. |

## Layer Specifications

### 1. Taxonomy (`taxonomy_` prefix)

Pure domain models, value objects, and business entities.

##### Components

- **Value Object (`_vo`)**: Immutable data containers. May use primitive types internally (**AES401** allows primitives in VO).
- **Entity (`_entity`)**: Stateful domain concepts with unique IDs.
- **Event (`_event`)**: Immutable domain fact snapshots.
- **Error (`_error`)**: Domain-level exceptions.
- **Constant (`_constant`)**: Compile-time literals only (**AES401**).

### 2. Contract (`contract_` prefix)

Interface definitions — _what_ can be done without _how_.

##### Components

- **Port (`_port`)**: Outbound interfaces implemented by Capabilities group (B) (adapters/scanners).
- **Protocol (`_protocol`)**: Inbound interfaces implemented by Capabilities.
- **Aggregate (`_aggregate`)**: Composition facades.

### 3. Capabilities (`capabilities_` prefix)

The single implementation layer. Owns ALL non-trivial logic under the Contract layer — both domain use-cases and technical I/O. Capabilities is divided into three clearly separated concerns:

#### (A) Domain Logic — use-cases, business logic, computations

Pure, testable domain behavior. Agnostic of concrete I/O where possible; depends on Contract ports/protocols for anything external.

| Category                      | Description                                    | Allowed Suffix                        |
| ----------------------------- | ---------------------------------------------- | ------------------------------------- |
| **Computation / Calculation** | Score calculation from violation counts        | `_calculator`, `_scorer`              |
| **Validation / Checking**     | Input validation with error returns            | `_checker`, `_validator`              |
| **Data Transformation**       | Map, filter, reduce on collections             | `_transformer`, `_mapper`, `_filter`  |
| **Business Rules**            | Conditional blocking based on severity         | `_evaluator`, `_resolver`             |
| **Information Extraction**    | Parse imports from source code                 | `_extractor`, `_classifier`           |
| **Assessment / Scoring**      | Grade conversion from score values             | `_assessor`, `_auditor`, `_inspector` |

#### (B) Technical Implementations — adapters, scanners, external-tool wrappers

Concrete I/O lives here. These are Capabilities with I/O suffixes (`_adapter`, `_provider`, `_scanner`, `_client`, `_wrapper`, `_system`). They perform file I/O, spawn external linters, read config — but they MUST delegate pure, reusable operations (path walking, ignore-matching, source-file detection) to `taxonomy_*_utility` free functions. Do NOT re-implement walking/normalization logic inside a capability — call the shared utility.

| Category              | Description                                 | Allowed Suffix                           |
| --------------------- | ------------------------------------------- | ---------------------------------------- |
| **File I/O**          | Read/write files from filesystem            | `_adapter`, `_reader`, `_writer`         |
| **Network Calls**     | HTTP requests and API calls                 | `_client`, `_connector`, `_gateway`      |
| **CLI Execution**     | Run external commands / linters             | `_adapter`, `_provider`                  |
| **Library Wrappers**  | Wrap third-party crates/packages            | `_provider`, `_wrapper`                  |
| **System Operations** | Process management, environment variables   | `_system`                                |

#### Utility Functions — `capabilities_*_utility` / `capabilities_*_helper`

Stateless, reusable helpers used by both (A) and (B). Pure functions only — no module-level mutable state. Examples: file collection helpers, normalization helpers, detector helpers. These are the boundary between "logic that belongs in a capability" and "pure logic that belongs in `taxonomy_*_utility`": if a helper needs no capability context and is cross-feature reusable, prefer `taxonomy_*_utility`; if it is feature-specific glue, keep it `capabilities_*_utility`.

#### Forbidden Logic (A, B, and Utility):

- ❌ Flow control (loops, branching for orchestration) → **Agent**
- ❌ Domain model definition → **Taxonomy**
- ❌ Re-implementing pure reusable logic that already exists in `taxonomy_*_utility` (path walking, ignore matching, source-file detection) → call the utility instead

### 4. Agent (`agent_` prefix)

Orchestration and pipeline execution. Coordinates multiple capabilities flows to execute pipelines/workflows.

#### Allowed Logic (Flow Control):

| Category                   | Description                                   | Rust Syntax                            | Python Syntax                     | TypeScript Syntax                |
| -------------------------- | --------------------------------------------- | -------------------------------------- | --------------------------------- | -------------------------------- |
| **Looping**                | Process events in event loop                  | `while`, `loop`, `for`                 | `while`, `for`                    | `while`, `for`, `forEach`        |
| **Sequential**             | Step A → Step B → Step C                      | Sequential statements                  | Sequential statements             | Sequential statements            |
| **Branching**              | Conditional execution                         | `if/else`, `match`                     | `if/elif/else`                    | `if/else`, `switch`              |
| **Error Handling**         | Handle Result/Option from other layers        | `match`, `if let`, `?`                 | `try/except`                      | `try/catch`                      |
| **Timeout / Cancellation** | Cancellable async operations                  | `tokio::select!`, `tokio::time::sleep` | `asyncio.wait_for()`              | `AbortSignal`, `Promise.race()`  |

#### Forbidden Logic:

- ❌ Computation / business rules → **Capabilities**
- ❌ File I/O, network calls → **Capabilities group (B)**
- ❌ Domain model definition → **Taxonomy**

#### File Limits:

- **AES301** (max 1000 lines) applies to all files including agent files
- **AES405** checks for `any` type annotations (no file size limit)

#### Agent Orchestrator Pattern:

Orchestrator struct holds references to protocols via Contract layer. Constructor receives dependencies through DI. Execute method sequences calls: fetch data, check emptiness with branching, loop over items with error handling, return result.

### 5. Surfaces (`surface_` prefix)

User-facing entry points — CLI, TUI, MCP server, API.

##### Components

- **Smart Surfaces (`command`/`controller`/`page`/`entry`)**: `taxonomy_` + `contract_aggregate_` only (AES201). Must NOT import capabilities/agent directly — use `ServiceContainerAggregate`.
- **Utility Surfaces (`hook`/`store`/`action`/`screen`)**: `taxonomy_` only + passive surfaces. Must NOT import smart surfaces (AES406).
- **Passive Surfaces (`component`/`view`/`layout`)**: `taxonomy_` only (AES406). No logic or orchestration.

##### Allowed Logic:

| Category                | Description                                | Allowed Suffix            |
| ----------------------- | ------------------------------------------ | ------------------------- |
| **User Input Handling** | Match keys to actions                      | `_command`, `_controller` |
| **UI Rendering**        | Print status messages to console           | `_view`, `_component`     |
| **Event Mapping**       | Map key press to internal event types      | `_command`, `_action`     |
| **State Management**    | Update selected file in state              | `_store`, `_state`        |
| **Routing**             | Navigate between screens                   | `_router`, `_page`        |

##### Forbidden Logic:

- ❌ Computation / business rules → **Capabilities**
- ❌ Flow control for orchestration → **Agent**
- ❌ File I/O, network calls → **Capabilities group (B)**
- ❌ Direct import of capabilities → **AES201 violation**

#### Surface vs Capabilities — Key Difference

| Aspect            | Surface                              | Capabilities (group B)                          |
| ----------------- | ------------------------------------ | ----------------------------------------------- |
| **Purpose**       | User interaction (UI/presentation)   | System interaction (technical I/O)              |
| **Input source**  | Human (keyboard, mouse, API request) | System (file system, network, processes)        |
| **Output target** | Human (console, UI, HTTP response)   | System (files, network, external processes)     |
| **Logic**         | Event mapping, routing, rendering    | File I/O, network calls, CLI execution          |

### 6. Root (`root_` prefix)

Wiring layer. Responsible for Dependency Injection (DI) composition. No business logic is allowed here — only instantiation and wiring.

##### Components

- **Container (`_container`)**: Per-feature DI container. Instantiates `capabilities_*` implementations, wires them behind `contract_*` traits/interfaces, and exposes typed factory methods. Each feature crate owns exactly one `root_*_container`.
- **Entry (`_entry`)**: Binary entry point. Bootstraps the application by creating the `CompositionRoot` (the top-level root container that composes all feature containers) and starts the main loop.

---

## Quick Reference: Logic Ownership

| Logic Type                    | Layer           | Suffix                    |
| ----------------------------- | --------------- | ------------------------- |
| **User Input Handling**       | `surface_`      | `_command`, `_controller` |
| **UI Rendering**              | `surface_`      | `_view`, `_component`     |
| **Event Mapping**             | `surface_`      | `_command`, `_action`     |
| **Routing**                   | `surface_`      | `_router`, `_page`        |
| **Computation / Calculation** | `capabilities_` | `_calculator`, `_scorer`  |
| **Validation / Checking**     | `capabilities_` | `_checker`, `_validator`  |
| **Data Transformation**       | `capabilities_` | `_transformer`, `_mapper` |
| **Business Rules**            | `capabilities_` | `_evaluator`, `_resolver` |
| **File I/O**                  | `capabilities_` | `_adapter`, `_reader`     |
| **Network Calls**             | `capabilities_` | `_client`, `_gateway`     |
| **CLI Execution**             | `capabilities_` | `_adapter`, `_provider`   |
| **Library Wrappers**          | `capabilities_` | `_provider`, `_wrapper`   |
| **Looping**                   | `agent_`        | `_orchestrator`           |
| **Sequential Steps**          | `agent_`        | `_orchestrator`           |
| **Branching (if/match)**      | `agent_`        | `_orchestrator`           |
| **Error Handling**            | `agent_`        | `_orchestrator`           |
| **Domain Models**             | `taxonomy_`     | `_vo`, `_entity`          |
| **Interfaces**                | `contract_`     | `_port`, `_protocol`      |
| **DI Wiring**                 | `root_`         | `_container`, `_entry`    |

---

## Architectural Patterns

### Container Wiring (DI)

Container struct holds orchestrator reference. Constructor creates capability adapters (group B), passes them to capability checkers (group A), then wires everything into the orchestrator. Factory method returns orchestrator reference.

### Port → Adapter Pattern

Contract defines trait/interface with read, write, exists methods. Capabilities group (B) implements the trait, delegating pure path logic to `taxonomy_*_utility` and concrete I/O to the language standard library (std::fs, open(), fs.promises).

### Data Flow (Surface → Agent → Capability → Contract)

User presses "c" (check) in TUI. Surface layer maps key to internal event type. Agent layer receives event and delegates to lint executor. Capabilities layer (group A) runs check logic and calls code analysis. Contract layer provides interface for import parsing. Capabilities group (B) performs actual file scanning, delegating pure walking to `taxonomy_*_utility`.

### Migration Note

Earlier AES revisions defined a separate **Infrastructure** layer as a peer of Capabilities. That layer was retired: its legitimate technical responsibilities (file I/O, external-tool wrappers, system adapters) were absorbed into **Capabilities group (B)**, and its pure reusable helpers were merged into `taxonomy_*_utility`. The architecture is now a six-layer model with Capabilities as the single implementation layer.
