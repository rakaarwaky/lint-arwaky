# AES Architecture: Agentic Engineering System

See [AGENTS.md](../AGENTS.md) for workspace conventions and [RULES_AES.md](../.agents/rules/RULES_AES.md) for the full rule catalog.

The **Agentic Engineering System (AES)** is a strictly layered, decoupled, and AI-native architectural pattern. It isolates domain models, protects business logic from low-level detail, and makes the codebase easy for AI agents and LLMs to navigate, understand, and modify without hallucinating architectural violations.

Under this AES revision there are **no Ports and no Infrastructure layer**. Technical implementation lives inside Capabilities as **Utility** functions — standalone, free functions with no struct, no trait, no contract. Business-logic files stay clean: they only call utility functions by clear names and never deal with low-level detail.

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

The codebase is divided into distinct horizontal and vertical boundaries. Layers communicate using downward-only dependency paths to prevent coupling and circular dependencies.

### 2. Single Implementation Layer (Capabilities)

The **Capabilities** layer is the single implementation layer. It owns all non-trivial logic: business logic (group Utama) AND technical implementation (group Utility). There is no separate Infrastructure layer and no Port abstraction. Technical detail is expressed as standalone utility functions, not as adapters behind contracts.

### 3. Business Logic Stays Clean

File Utama (business logic) must NOT contain low-level detail — algorithms, regex, array manipulation, third-party library wiring. It calls Utility functions by clear, intention-revealing names. The complexity lives in Utility; the readability lives in Utama.

### 4. The 3-Structure Naming Philosophy (Layer Prefix + Vertical Slicing + Role Suffix)

AES enforces a **File Naming Convention**: `[layer]_[concept]_[suffix]` or `[layer]_[concept1]_[concept2]_[suffix]` if needed

1. **Layer (prefix)**: The architectural layer (e.g., `contract_`, `capabilities_`, `taxonomy_`).
2. **Concept (middle)**: A single/multiple word defining the core concept (e.g., `compliance`, `import_rule`).
3. **Role (suffix)**: Defines the architectural role (e.g., `_protocol`, `_aggregate`, `_checker`).

Files are organized into **feature crates** (vertical slicing). All layers coexist in each feature crate, distinguished by their file prefix.

Exceptions: `main.rs/py/ts`, `lib.rs/py/ts`, `mod.rs`, `__init__.py`, `index.ts`.

---

## Layer Hierarchy (Dependency Direction)

The dependency hierarchy flows as follows:

- **Root** layer wraps all other layers and contains wiring/bootstrap.
- **Surface** layer (CLI, MCP Server, API) imports from Contract-aggregate and Taxonomy only.
- **Agent** layer (Orchestrators) imports from Contract and Taxonomy only.
- **Capabilities** layer (Utama + Utility) imports from Contract and Taxonomy only. Utility functions are standalone (no struct/trait/port).
- **Contract** layer (Protocol, Aggregate) imports from Taxonomy only.
- **Taxonomy** layer (VOs, Entities, Errors, Events, Constants) is the foundation layer with no imports.

## The Six Layers

1. **Taxonomy** — pure domain models (VO/entity/error/constant). Foundation, no imports.
2. **Contract** — abstract inbound interfaces (protocol) and facades (aggregate). No ports. Imports Taxonomy only.
3. **Capabilities** — the single implementation layer, split into **Utama** (business logic) and **Utility** (standalone technical functions). Imports Taxonomy + Contract only.
4. **Agent** — orchestration / pipeline flow control. Imports Taxonomy + Contract only.
5. **Surface** — user-facing entry points (CLI, TUI, MCP, API). Imports Taxonomy + Contract-aggregate only.
6. **Root** — wiring / bootstrap. No business logic.

## Layer Prefix Specifications

Files use the layer as a **file prefix** (not a directory): `[layer]_[concept]_[suffix]`.

| Layer Prefix    | Allowed Suffixes                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Allowed Imports                                                                                  | Semantic Role / Description                                                                       |
| :-------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------ |
| `taxonomy_`     | `_vo`, `_entity`, `_event`, `_error`, `_constant` (strict)                                                                                                                                                                                                                                                                                                                                                                                                            | `taxonomy_` files only.                                                                          | Pure domain models, value objects, domain events, errors, and compile-time constants.             |
| `contract_`     | `_protocol`, `_aggregate`                                                                                                                                                                                                                                                                                                                                                                                                                                             | `taxonomy_`, `contract_`                                                                         | Abstract inbound interfaces (protocol) and facade aggregates. NO ports.                           |
| `capabilities_` | `_analyzer`, `_checker`, `_processor`, `_evaluator`, `_resolver`, `_validator`, `_formatter`, `_executor`, `_transformer`, `_calculator`, `_builder`, `_compiler`, `_classifier`, `_extractor`, `_reporter`, `_mapper`, `_filter`, `_collector`, `_comparator`, `_scorer`, `_inspector`, `_reviewer`, `_assessor`, `_auditor`, `_utility`, `_helper`                                                                                                                  | `taxonomy_`, `contract_`                                                                         | **Utama**: business logic, use-cases, computations. **Utility**: standalone technical functions.  |
| `agent_`        | `_orchestrator`                                                                                                                                                                                                                                                                                                                                                                                                                                                       | `taxonomy_`, `contract_`                                                                         | Coordinates multiple capabilities flows to execute pipelines/workflows.                           |
| `surface_`      | `_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_hook`, `_store`, `_action`, `_screen`                                                                                                                                                                                                                                                                                                                                              | Varies by surface role (see Surface layer details below).                                        | Application entry points, UI components, CLI commands, controllers, and pages.                    |
| `root_`         | `_container`, `_entry`                                                                                                                                                                                                                                                                                                                                                                                                                                                | All layers (`taxonomy_`, `contract_`, `capabilities_`, `agent_`, `surface_`).                   | App bootstrap, inline composition, and wiring. Absolutely no business logic.                     |

---

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

Interface definitions — _what_ can be done without _how_. **Ports are removed.** Only two components remain:

##### Components

- **Protocol (`_protocol`)**: Inbound interface implemented by Capabilities Utama. Defines the behavior a caller (Agent/Surface) can rely on, without exposing implementation.
- **Aggregate (`_aggregate`)**: Composition facade. Exposes a typed entry point so Surfaces do not import Capabilities directly (AES201).

### 3. Capabilities (`capabilities_` prefix)

The single implementation layer. Split into two groups — **Utama** (business logic) and **Utility** (standalone technical functions).

#### (A) Utama — Business Logic

Use-cases, business rules, computations. Clean and readable: it calls Utility functions by clear names and never contains low-level detail.

| Category                      | Description                                    | Allowed Suffix                        |
| ----------------------------- | ---------------------------------------------- | ------------------------------------- |
| **Computation / Calculation** | Score calculation from violation counts        | `_calculator`, `_scorer`              |
| **Validation / Checking**     | Input validation with error returns            | `_checker`, `_validator`              |
| **Data Transformation**       | Map, filter, reduce on collections             | `_transformer`, `_mapper`, `_filter`  |
| **Business Rules**            | Conditional blocking based on severity         | `_evaluator`, `_resolver`             |
| **Information Extraction**    | Parse imports from source code                 | `_extractor`, `_classifier`           |
| **Assessment / Scoring**      | Grade conversion from score values             | `_assessor`, `_auditor`, `_inspector` |

#### (B) Utility — Standalone Technical Implementation

Contains the hard, low-level detail: specific algorithms, third-party library setup, regex/array manipulation, file-system walking, process spawning. **Rules for Utility:**

- **Standalone free functions.** No `struct`, no `trait`, no `impl` of a contract/port.
- **No Contract dependency.** Utility does not reference `_protocol` / `_aggregate`.
- **Called by name.** Utama files invoke them directly: `collect_source_files(dir)`, `run_ruff(path)`, `normalize_path(p)`.
- **Reusable.** Cross-feature reusable helpers belong in `shared` as `taxonomy_*_utility` / `capabilities_*_utility`.

| Category                     | Description                                          | Allowed Suffix              |
| ---------------------------- | ---------------------------------------------------- | --------------------------- |
| **File I/O detail**          | Walking dirs, ignore-matching, source-file detection | `_utility`, `_helper`       |
| **External tool execution**  | Spawn ruff/clippy/eslint/tsc, parse their output     | `_utility`, `_helper`       |
| **Algorithm / regex**        | Low-level string/array/regex manipulation            | `_utility`, `_helper`       |
| **System operations**        | Process/env handling                                 | `_utility`, `_helper`       |

##### Utama vs Utility — the key separation

| Concern                       | Belongs in Utama            | Belongs in Utility                        |
| ----------------------------- | --------------------------- | ----------------------------------------- |
| "collect all source files"    | call `collect_source_files` | the recursive walk + ignore logic         |
| "run the linter"              | call `run_ruff(path)`       | spawn process, capture stdout, parse JSON |
| "is this an import violation" | apply the rule              | regex/alias resolution helper             |

#### Forbidden Logic (both Utama and Utility):

- ❌ Flow control (loops, branching for orchestration) → **Agent**
- ❌ Domain model definition → **Taxonomy**
- ❌ Low-level detail inside Utama → extract to **Utility**
- ❌ `struct`/`trait`/`impl` of a contract inside Utility → keep Utility standalone

### 4. Agent (`agent_` prefix)

Orchestration and pipeline execution. Coordinates multiple capabilities flows.

#### Allowed Logic (Flow Control):

| Category                   | Description                                   | Rust Syntax                            | Python Syntax                     | TypeScript Syntax                |
| -------------------------- | --------------------------------------------- | -------------------------------------- | --------------------------------- | -------------------------------- |
| **Looping**                | Process events in event loop                  | `while`, `loop`, `for`                 | `while`, `for`                    | `while`, `for`, `forEach`        |
| **Sequential**             | Step A → Step B → Step C                      | Sequential statements                  | Sequential statements             | Sequential statements            |
| **Branching**              | Conditional execution                         | `if/else`, `match`                     | `if/elif/else`                    | `if/else`, `switch`              |
| **Error Handling**         | Handle Result/Option from other layers        | `match`, `if let`, `?`                 | `try/except`                      | `try/catch`                      |
| **Timeout / Cancellation** | Cancellable async operations                  | `tokio::select!`, `tokio::time::sleep` | `asyncio.wait_for()`              | `AbortSignal`, `Promise.race()`  |

#### Forbidden Logic:

- ❌ Computation / business rules → **Capabilities (Utama)**
- ❌ Low-level technical detail → **Capabilities (Utility)**
- ❌ Domain model definition → **Taxonomy**

#### File Limits:

- **AES301** (max 1000 lines) applies to all files including agent files
- **AES405** checks for `any` type annotations (no file size limit)

#### Agent Orchestrator Pattern:

Orchestrator struct holds references to protocols via Contract layer. Constructor receives dependencies through wiring. Execute method sequences calls: fetch data, check emptiness with branching, loop over items with error handling, return result.

### 5. Surfaces (`surface_` prefix)

User-facing entry points — CLI, TUI, MCP server, API.

##### Components

- **Smart Surfaces (`command`/`controller`/`page`/`entry`)**: `taxonomy_` + `contract_aggregate_` only (AES201). Must NOT import capabilities/agent directly — use the aggregate facade.
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

- ❌ Computation / business rules → **Capabilities (Utama)**
- ❌ Flow control for orchestration → **Agent**
- ❌ Low-level technical detail → **Capabilities (Utility)**
- ❌ Direct import of capabilities → **AES201 violation**

#### Surface vs Capabilities — Key Difference

| Aspect            | Surface                              | Capabilities (Utility)                     |
| ----------------- | ------------------------------------ | ------------------------------------------ |
| **Purpose**       | User interaction (UI/presentation)   | System interaction (technical detail)      |
| **Input source**  | Human (keyboard, mouse, API request) | System (file system, network, processes)   |
| **Output target** | Human (console, UI, HTTP response)   | System (files, network, external processes)|
| **Logic**         | Event mapping, routing, rendering    | Algorithms, regex, library wiring, I/O     |

### 6. Root (`root_` prefix)

Wiring layer. Responsible for composition and bootstrap. No business logic is allowed here — only instantiation and wiring.

##### Components

- **Container (`_container`)**: Per-feature wiring container. Instantiates Capabilities and wires them behind Contract protocols/aggregates, exposing typed factory methods. Each feature crate owns exactly one `root_*_container`.
- **Entry (`_entry`)**: Binary entry point. Bootstraps the application by creating the composition root (top-level container composing all feature containers) and starting the main loop.

---

## Quick Reference: Logic Ownership

| Logic Type                    | Layer             | Suffix                    |
| ----------------------------- | ----------------- | ------------------------- |
| **User Input Handling**       | `surface_`        | `_command`, `_controller` |
| **UI Rendering**              | `surface_`        | `_view`, `_component`     |
| **Event Mapping**             | `surface_`        | `_command`, `_action`     |
| **Routing**                   | `surface_`        | `_router`, `_page`        |
| **Computation / Calculation** | `capabilities_`   | `_calculator`, `_scorer`  |
| **Validation / Checking**     | `capabilities_`   | `_checker`, `_validator`  |
| **Data Transformation**       | `capabilities_`   | `_transformer`, `_mapper` |
| **Business Rules**            | `capabilities_`   | `_evaluator`, `_resolver` |
| **Low-level technical detail**| `capabilities_`   | `_utility`, `_helper`     |
| **Looping**                   | `agent_`          | `_orchestrator`           |
| **Sequential Steps**          | `agent_`          | `_orchestrator`           |
| **Branching (if/match)**      | `agent_`          | `_orchestrator`           |
| **Error Handling**            | `agent_`          | `_orchestrator`           |
| **Domain Models**             | `taxonomy_`       | `_vo`, `_entity`          |
| **Inbound Interfaces**        | `contract_`       | `_protocol`               |
| **Facades**                   | `contract_`       | `_aggregate`              |
| **Wiring / Bootstrap**        | `root_`           | `_container`, `_entry`    |

---

## Architectural Patterns

### Wiring (no Port / no DI-through-trait)

Root container wires Capabilities implementations behind Contract protocols/aggregates. There is no Port abstraction — Utama files call Utility functions directly by name; Surfaces reach Capabilities only through the aggregate facade.

### Utama → Utility Pattern

Business-logic file calls a clearly named utility function. The utility owns the messy detail (regex, third-party library, recursion, process spawn). Example:

- Utama: `let files = collect_source_files(dir);` — one readable line.
- Utility: recursive walk, ignore-matching, extension check, symlink/inode handling.

### Data Flow (Surface → Agent → Capability → Contract)

User presses "c" (check) in TUI. Surface maps key to an internal event via the aggregate. Agent receives the event and delegates to the lint executor. Capabilities Utama runs check logic; when it needs low-level work (file scanning, running a linter) it calls a Utility function. Contract protocol defines the inbound interface the Agent relies on.

### Migration Note

Earlier AES revisions defined a separate **Infrastructure** layer (peer of Capabilities) reached through **Ports**. Both were retired: Infrastructure's technical responsibilities became **Capabilities Utility** (standalone free functions, no struct/trait/port), and Ports were removed — Contract now carries only Protocol and Aggregate. The architecture is a six-layer model where Capabilities is the single implementation layer, split into clean business logic (Utama) and standalone technical utility.
