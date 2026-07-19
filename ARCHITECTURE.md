# AES Architecture: Agentic Engineering System [L1-502]

See [AGENTS.md](../AGENTS.md) for workspace conventions and [RULES_AES.md](../.agents/rules/RULES_AES.md) for the full rule catalog.

The **Agentic Engineering System (AES)** is a strictly layered, highly decoupled, and AI-native architectural pattern. It is designed to achieve maximum modularity, absolute testability, and extreme maintainability by enforcing rigid structural boundaries. Under the AES paradigm, technical details are isolated, domain models are protected, and dependencies are strictly inverted via abstract contracts. Furthermore, AES is specifically optimized for **Agentic workflows**, ensuring that AI agents and LLMs can easily navigate, understand, and modify the codebase without hallucinating architectural violations.

---

## Terminology [L9-21]

AES supports multiple languages (Rust, TypeScript, Python) to maintain a single unified vocabulary:

| Term          | Language        | Definition                                                                                                         |
| ------------- | --------------- | ------------------------------------------------------------------------------------------------------------------ |
| **Workspace** | All             | The entire project root directory (e.g.,`lint-arwaky/`) containing all configs and language-specific sub-projects. |
| `crates/`     | Rust            | The directory containing all Rust crates (workspace members), conforming to Cargo workspace specifications.        |
| `packages/`   | TypeScript / JS | The directory containing all TypeScript/JavaScript packages, following npm/pnpm workspace conventions.             |
| `modules/`    | Python          | The directory containing all Python sub-projects, organized as independent python modules.                         |
| Member        | All             | A single, self-contained sub-project (crate, package, or module) inside the workspace.                             |

## Core Pillars and Philosophy [L21-69]

### 1. Strict Layered Boundary Enforcement [L23-27]

The codebase is divided into distinct horizontal and vertical boundaries. Layers can only communicate using downward-only dependency paths to prevent coupling and circular dependencies. Any violation of these import boundaries is caught at compile or lint time by static analysis checkers.

### 2. Sibling Equivalence and Peer Layers [L27-35]

Unlike traditional three-tier architectures, **Capabilities** and **Infrastructure** are horizontal peer layers.

- Neither layer is above or below the other.
- Neither layer can ever import from or know about the other.
- Both layers depend downward on the **Contract** layer exclusively via Ports and Protocols.

### 3. Dependency Inversion [L35-39]

Higher-level orchestrating layers never import concrete implementations. Instead, they interact with implementations exclusively through interfaces declared in the Contract layer using Dependency Injection (e.g., Surfaces call `ServiceContainerAggregate`, not concrete Orchestrators).

### 4. The 3-Structure Naming Philosophy (Layer Prefix + Vertical Slicing + Role Suffix) [L39-69]

AES enforces a **File Naming Convention**: `[layer]_[concept]_[suffix]` or `[layer]_[concept1]_[concept2]_[suffix]` if needed

1. **Layer (prefix)**: The architectural layer (e.g., `contract_`, `capabilities_`, `taxonomy_`).
2. **Concept (middle)**: A single/multiple word defining the core concept (e.g., `compliance`, `import_rule`).
3. **Role (suffix)**: Defines the architectural role (e.g., `_port`, `_protocol`, `_checker`).

Files are organized into **feature crates** (vertical slicing) rather than layer directories. All seven layers coexist in each feature crate, distinguished by their file prefix.

Exceptions: `main.rs/py/ts`, `lib.rs/py/ts`, `mod.rs`, `__init__.py`, `index.ts`.

---

## Layer Hierarchy (Dependency Direction) [L69-107]

The dependency hierarchy flows as follows:

- **Root** layer wraps all other layers and contains DI wiring.
- **Surface** layer (CLI, MCP Server, API) imports from Contract and Taxonomy only.
- **Agent** layer (Orchestrators) imports from Contract and Taxonomy only.
- **Capabilities** layer (Checkers, Analyzers) and **Infrastructure** layer (Adapters, Scanners) are peer layers with no direct sibling import between them. Both import from Contract and Taxonomy only.
- **Contract** layer (Ports, Protocols, Aggregates) imports from Taxonomy only.
- **Taxonomy** layer (VOs, Entities, Errors, Events, Constants) is the foundation layer with no imports.

The Root container handles DI wiring that instantiates and injects all layers. The Root entry point bootstraps the application by creating the composition root and starting the main loop.

## Layer Prefix Specifications [L107-121]

Files use the layer as a **file prefix** (not a directory): `[layer]_[concept]_[suffix].`or `[layer]_[concept1]_[concept2]_[suffix]` if needed All seven layers coexist in each feature crate, distinguished by their prefix.

| Layer Prefix      | Allowed Suffixes                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Allowed Imports                                                                                  | Semantic Role / Description                                                                       |
| :---------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------ |
| `taxonomy_`       | `_vo`, `_entity`, `_event`, `_error`, `_constant` (strict)                                                                                                                                                                                                                                                                                                                                                                                                            | `taxonomy_` files only (outer imports trigger **AES201**).                                       | Pure domain models, value objects, domain events, errors, and compile-time constants.             |
| `contract_`       | `_port`, `_protocol`, `_aggregate`                                                                                                                                                                                                                                                                                                                                                                                                                                    | `taxonomy_`, `contract_`                                                                         | Abstract interfaces: Outbound interface ports, inbound protocols, and facade aggregates.          |
| `capabilities_`   | `_analyzer`, `_checker`, `_processor`, `_evaluator`, `_resolver`, `_validator`, `_formatter`, `_executor`, `_transformer`, `_calculator`, `_builder`, `_compiler`, `_classifier`, `_extractor`, `_reporter`, `_mapper`, `_filter`, `_collector`, `_comparator`, `_scorer`, `_inspector`, `_reviewer`, `_assessor`, `_auditor`, `_utility`, `_helper`                                                                                                                  | `taxonomy_`, `contract_`                                                                         | Domain use-cases, business logic, and computations. Pure and agnostic of infrastructure.          |
| `infrastructure_` | `_adapter`, `_provider`, `_scanner`, `_client`, `_schemas`, `_lifespan`, `_wrapper`, `_tracer`, `_tracker`, `_variants`, `_detector`, `_patterns`, `_system`, `_repository`, `_cache`, `_loader`, `_writer`, `_reader`, `_driver`, `_connector`, `_gateway`, `_serializer`, `_encoder`, `_decoder`, `_fetcher`, `_watcher`, `_indexer`, `_dispatcher`, `_recorder`, `_proxy`, `_publisher`, `_subscriber`, `_listener`, `_poller`, `_streamer`, `_utility`, `_helper` | `taxonomy_`, `contract_`                                                                         | Technical implementations, system adapters, library wraps, databases, CLI/network calls.          |
| `agent_`          | `_orchestrator`                                                                                                                                                                                                                                                                                                                                                                                                                                                       | `taxonomy_`, `contract_`                                                                         | Coordinates multiple capabilities and infrastructure flows to execute pipelines/workflows.        |
| `surface_`        | `_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_hook`, `_store`, `_action`, `_screen`                                                                                                                                                                                                                                                                                                                                              | Varies by surface role (see Surface layer details below).                                        | Application entry points, UI components, CLI commands, controllers, and pages.                    |
| `root_`           | `_container`, `_entry`                                                                                                                                                                                                                                                                                                                                                                                                                                                | All layers (`taxonomy_`, `contract_`, `capabilities_`, `infrastructure_`, `agent_`, `surface_`). | App bootstrap, inline composition, and Dependency Injection wiring. Absolutely no business logic. |

## Layer Specifications [L121-343]

#### 1. Taxonomy (`taxonomy_` prefix) [L123-135]

Pure domain models, value objects, and business entities.

##### Components [L127-135]

- **Value Object (`_vo`)**: Immutable data containers. May use primitive types internally (**AES401** allows primitives in VO).
- **Entity (`_entity`)**: Stateful domain concepts with unique IDs.
- **Event (`_event`)**: Immutable domain fact snapshots.
- **Error (`_error`)**: Domain-level exceptions.
- **Constant (`_constant`)**: Compile-time literals only (**AES401**).

#### 2. Contract (`contract_` prefix) [L135-145]

Interface definitions — _what_ can be done without _how_.

##### Components [L139-145]

- **Port (`_port`)**: Outbound interfaces implemented by Infrastructure.
- **Protocol (`_protocol`)**: Inbound interfaces implemented by Capabilities.
- **Aggregate (`_aggregate`)**: Composition facades.

### 3. Capabilities (`capabilities_` prefix) [L145-182]

Use-case logic. Entirely agnostic of infrastructure.

#### Allowed Logic: [L149-160]

| Category                      | Description                                    | Allowed Suffix                        |
| ----------------------------- | ---------------------------------------------- | ------------------------------------- |
| **Computation / Calculation** | Score calculation from violation counts        | `_calculator`, `_scorer`              |
| **Validation / Checking**     | Input validation with error returns            | `_checker`, `_validator`              |
| **Data Transformation**       | Map, filter, reduce on collections             | `_transformer`, `_mapper`, `_filter`  |
| **Business Rules**            | Conditional blocking based on severity         | `_evaluator`, `_resolver`             |
| **Information Extraction**    | Parse imports from source code                 | `_extractor`, `_classifier`           |
| **Assessment / Scoring**      | Grade conversion from score values             | `_assessor`, `_auditor`, `_inspector` |

#### Forbidden Logic: [L160-166]

- ❌ Flow control (loops, branching for orchestration) → **Agent**
- ❌ File I/O, network calls → **Infrastructure**
- ❌ Direct struct mutation without contract → **Taxonomy**

### 4. Infrastructure (`infrastructure_` prefix) [L182-343]

Technical implementations and external tool wrappers.

#### Allowed Logic: [L186-198]

| Category                | Description                                 | Allowed Suffix                           |
| ----------------------- | ------------------------------------------- | ---------------------------------------- |
| **File I/O**            | Read/write files from filesystem            | `_adapter`, `_reader`, `_writer`         |
| **Network Calls**       | HTTP requests and API calls                 | `_client`, `_connector`, `_gateway`      |
| **Database Operations** | SQL queries and data persistence            | `_repository`, `_driver`                 |
| **CLI Execution**       | Run external commands                       | `_adapter`, `_provider`                  |
| **Library Wrappers**    | Wrap third-party crates/packages            | `_provider`, `_wrapper`                  |
| **System Operations**   | Process management, environment variables   | `_system`, `_lifespan`                   |
| **Streaming / Pub-Sub** | Broadcast channels and event streams        | `_publisher`, `_subscriber`, `_streamer` |

#### Forbidden Logic: [L198-204]

- ❌ Business rules / computation → **Capabilities**
- ❌ Flow control for orchestration → **Agent**
- ❌ Domain logic without I/O → **Taxonomy**

#### 5. Agent (`agent_` prefix) [L217-221]

Orchestration and pipeline execution.

#### Allowed Logic (Flow Control): [L221-231]

| Category                   | Description                                   | Rust Syntax                            | Python Syntax                     | TypeScript Syntax                |
| -------------------------- | --------------------------------------------- | -------------------------------------- | --------------------------------- | -------------------------------- |
| **Looping**                | Process events in event loop                  | `while`, `loop`, `for`                 | `while`, `for`                    | `while`, `for`, `forEach`        |
| **Sequential**             | Step A → Step B → Step C                      | Sequential statements                  | Sequential statements             | Sequential statements            |
| **Branching**              | Conditional execution                         | `if/else`, `match`                     | `if/elif/else`                    | `if/else`, `switch`              |
| **Error Handling**         | Handle Result/Option from other layers        | `match`, `if let`, `?`                 | `try/except`                      | `try/catch`                      |
| **Timeout / Cancellation** | Cancellable async operations                  | `tokio::select!`, `tokio::time::sleep` | `asyncio.wait_for()`              | `AbortSignal`, `Promise.race()`  |

#### Forbidden Logic: [L231-237]

- ❌ Computation / business rules → **Capabilities**
- ❌ File I/O, network calls → **Infrastructure**
- ❌ Domain model definition → **Taxonomy**

#### File Limits: [L237-242]

- **AES301** (max 1000 lines) applies to all files including agent files
- **AES405** checks for `any` type annotations (no file size limit)

#### Agent Orchestrator Pattern: [L242-279]

Orchestrator struct holds references to protocols via Contract layer. Constructor receives dependencies through DI. Execute method sequences calls: fetch data, check emptiness with branching, loop over items with error handling, return result.

#### 6. Surfaces (`surface_` prefix) [L279-306]

User-facing entry points — CLI, TUI, MCP server, API.

##### Components [L127-135]

- **Smart Surfaces (`command`/`controller`/`page`/`entry`)**: `taxonomy_` + `contract_aggregate_` only (AES201). Must NOT import capabilities/infrastructure/agent directly — use `ServiceContainerAggregate`.
- **Utility Surfaces (`hook`/`store`/`action`/`screen`)**: `taxonomy_` only + passive surfaces. Must NOT import smart surfaces (AES406).
- **Passive Surfaces (`component`/`view`/`layout`)**: `taxonomy_` only (AES406). No logic or orchestration.

##### Allowed Logic: [L289-299]

| Category                | Description                                | Allowed Suffix            |
| ----------------------- | ------------------------------------------ | ------------------------- |
| **User Input Handling** | Match keys to actions                      | `_command`, `_controller` |
| **UI Rendering**        | Print status messages to console           | `_view`, `_component`     |
| **Event Mapping**       | Map key press to internal event types      | `_command`, `_action`     |
| **State Management**    | Update selected file in state              | `_store`, `_state`        |
| **Routing**             | Navigate between screens                   | `_router`, `_page`        |

##### Forbidden Logic: [L299-306]

- ❌ Computation / business rules → **Capabilities**
- ❌ Flow control for orchestration → **Agent**
- ❌ File I/O, network calls → **Infrastructure**
- ❌ Direct import of capabilities/infrastructure → **AES201 violation**

#### Surface vs Infrastructure — Key Difference [L306-332]

| Aspect            | Surface                                             | Infrastructure                                        |
| ----------------- | --------------------------------------------------- | ----------------------------------------------------- |
| **Purpose**       | User interaction (UI/presentation)                  | System interaction (technical I/O)                    |
| **Input source**  | Human (keyboard, mouse, API request)                | System (file system, network, database)               |
| **Output target** | Human (console, UI, HTTP response)                  | System (files, network, processes)                    |
| **Logic**         | Event mapping, routing, rendering                   | File I/O, network calls, CLI execution                |

#### 7. Root (`root_` prefix) [L332-343]

Wiring layer. Responsible for Dependency Injection (DI) composition. No business logic is allowed here — only instantiation and wiring.

##### Components [L336-343]

- **Container (`_container`)**: Per-feature DI container. Instantiates `infrastructure_*` and `capabilities_*` implementations, wires them behind `contract_*` traits/interfaces, and exposes typed factory methods. Each feature crate owns exactly one `root_*_container`.
- **Entry (`_entry`)**: Binary entry point. Bootstraps the application by creating the `CompositionRoot` (the top-level root container that composes all feature containers) and starts the main loop.

---

## Quick Reference: Logic Ownership [L343-370]

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
| **File I/O**                  | `infrastructure_` | `_adapter`, `_reader`     |
| **Network Calls**             | `infrastructure_` | `_client`, `_gateway`     |
| **Database**                  | `infrastructure_` | `_repository`, `_driver`  |
| **CLI Execution**             | `infrastructure_` | `_adapter`, `_provider`   |
| **Library Wrappers**          | `infrastructure_` | `_provider`, `_wrapper`   |
| **Looping**                   | `agent_`          | `_orchestrator`           |
| **Sequential Steps**          | `agent_`           | `_orchestrator`           |
| **Branching (if/match)**      | `agent_`          | `_orchestrator`           |
| **Error Handling**            | `agent_`          | `_orchestrator`           |
| **Domain Models**             | `taxonomy_`       | `_vo`, `_entity`          |
| **Interfaces**                | `contract_`       | `_port`, `_protocol`      |
| **DI Wiring**                 | `root_`           | `_container`, `_entry`    |

---

## Architectural Patterns [L370-502]

### Container Wiring (DI) [L372-408]

Container struct holds orchestrator reference. Constructor creates infrastructure adapter, passes it to capability checkers, then wires everything into orchestrator. Factory method returns orchestrator reference.

### Port → Adapter Pattern [L408-434]

Contract defines trait/interface with read, write, exists methods. Infrastructure implements the trait, delegating to language standard library (std::fs, open(), fs.promises).

### Data Flow (Surface → Agent → Capability → Contract → Infrastructure) [L434-450]

User presses "c" (check) in TUI. Surface layer maps key to internal event type. Agent layer receives event and delegates to lint executor. Capabilities layer runs check logic and calls code analysis. Contract layer provides interface for import parsing. Infrastructure layer performs actual file scanning.

### Before/After Migration [L450-502]

**BEFORE (flat, no layers):**

Files in the src directory contained mixed responsibilities — main entry files combined struct definitions, business logic, database calls, and I/O operations in a single file without separation of concerns. User configuration files mixed data structures with persistence logic. API handlers combined HTTP processing with business rules.

**AFTER (AES 7-layer — feature-based vertical slicing):**

The workspace root is organized by language-specific conventions: Rust uses crates/, TypeScript/JavaScript uses packages/, Python uses modules/. Each feature is a self-contained crate/module with all seven layers represented by file prefixes within the same directory. Shared types are isolated in a shared subdirectory with common types accessible across all features.

Each feature crate contains files for each layer: taxonomy files hold data structures, contract files define interfaces, capabilities files contain validation logic, infrastructure files implement adapters, agent files coordinate operations, surface files handle CLI commands, and root files manage DI wiring. The lib.rs/py/ts file serves as the module root.

Top-level entry points include separate files for CLI binary, MCP server, and TUI launcher, each bootstrapping their respective containers.
