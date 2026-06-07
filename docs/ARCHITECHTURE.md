# AES Architecture: Agentic Engineering System

The **Agentic Engineering System (AES)** is a strictly layered, highly decoupled, and AI-native architectural pattern. It is designed to achieve maximum modularity, absolute testability, and extreme maintainability by enforcing rigid structural boundaries.

Under the AES paradigm, technical details are isolated, domain models are protected, and dependencies are strictly inverted via abstract contracts. Furthermore, AES is specifically optimized for **Agentic workflows**, ensuring that AI agents and LLMs can easily navigate, understand, and modify the codebase without hallucinating architectural violations.

> The current reference implementation of AES is **lint-arwaky** (a Rust project). The architecture itself is language-agnostic; the suffix conventions and examples below are written with the Rust implementation in mind but apply equally to Python, TypeScript.

---

## Core Pillars and Philosophy

### 1. Strict Layered Boundary Enforcement

The codebase is divided into six distinct horizontal and vertical boundaries. Layers can only communicate using downward-only dependency paths to prevent coupling and circular dependencies. Any violation of these import boundaries is caught at compile or lint time by static analysis checkers.

### 2. Sibling Equivalence and Peer Layers

Unlike traditional three-tier architectures, **Capabilities** and **Infrastructure** are horizontal peer layers.

- Neither layer is above or below the other.
- Neither layer can ever import from or know about the other.
- Both layers depend downward on the **Contract** layer via Ports and Protocols.

#### Layer Hierarchy (Dependency Direction)

```mermaid
%%{init: {'theme': 'default', 'themeVariables': { 'primaryColor': '#ffffff', 'primaryTextColor': '#000000', 'primaryBorderColor': '#333333', 'lineColor': '#333333', 'secondaryColor': '#f4f4f4', 'tertiaryColor': '#ffffff' }}}%%
graph TD
    S["Surfaces<br/>(CLI, MCP Server, API)"]
    A["Agent<br/>(DI Container, Orchestrators)"]
    C["Capabilities<br/>(Checkers, Analyzers)"]
    I["Infrastructure<br/>(Adapters, Scanners)"]
    CT["Contract<br/>(Ports, Protocols)"]
    T["Taxonomy<br/>(VOs, Entities, Errors)"]

    S -->|"imports"| CT
    A -->|"imports"| C
    A -->|"imports"| I
    C -->|"imports"| CT
    I -->|"imports"| CT
    CT -->|"imports"| T
```

#### Data Flow (Request Lifecycle)

```mermaid
%%{init: {'theme': 'default'}}%%
sequenceDiagram
    participant User
    participant Surface as Surfaces
    participant Agent as Agent
    participant Cap as Capabilities
    participant Infra as Infrastructure
    participant Contract as Contract
    participant Tax as Taxonomy

    User->>Surface: CLI command / MCP request
    Surface->>Agent: Delegate via ServiceContainerAggregate
    Agent->>Agent: Wire DI, create orchestrator
    Agent->>Cap: Call protocol method
    Cap->>Contract: Use port interfaces
    Contract->>Infra: Dispatch to adapter
    Infra-->>Tax: Return VOs/Results
    Tax-->>Contract: Domain objects
    Contract-->>Cap: Port responses
    Cap-->>Agent: LintResultList
    Agent-->>Surface: GovernanceReport
    Surface-->>User: Formatted output
```

#### Import Rules (What Each Layer Can Import)

```mermaid
%%{init: {'theme': 'default', 'themeVariables': { 'primaryColor': '#ffffff', 'primaryTextColor': '#000000', 'primaryBorderColor': '#333333', 'lineColor': '#333333' }}}%%
graph TD
    S["Surfaces"]
    A["Agent"]
    C["Capabilities"]
    I["Infrastructure"]
    CT["Contract"]
    T["Taxonomy"]

    S -->|"✅ Taxonomy"| T
    S -->|"✅ Contract"| CT
    S -.->|"❌ Agent"| A
    S -.->|"❌ Capabilities"| C
    S -.->|"❌ Infrastructure"| I

    A -->|"✅ Taxonomy"| T
    A -->|"✅ Contract"| CT
    A -->|"✅ Capabilities"| C
    A -->|"✅ Infrastructure"| I

    C -->|"✅ Taxonomy"| T
    C -->|"✅ Contract"| CT
    C -.->|"❌ Infrastructure"| I

    I -->|"✅ Taxonomy"| T
    I -->|"✅ Contract"| CT
    I -.->|"❌ Capabilities"| C

    CT -->|"✅ Taxonomy"| T
    CT -.->|"❌ Capabilities"| C
    CT -.->|"❌ Infrastructure"| I
```

#### Check Command Flow (End-to-End)

```mermaid
%%{init: {'theme': 'default'}}%%
sequenceDiagram
    participant U as User
    participant CLI as cli_main_entry
    participant DI as DependencyInjectionContainer
    participant CS as CheckCommandsSurface
    participant AL as IArchLintProtocol
    participant AH as ArchLintHandler
    participant AC as ArchComplianceAnalyzer
    participant NC as ArchNamingChecker
    participant IC as ArchInternalChecker
    participant MC as ArchMetricChecker

    Note over U,MC: User runs: lint-arwaky check .

    U->>CLI: CLI command
    CLI->>DI: DependencyInjectionContainer::new()
    Note over DI: Creates OSFileSystemAdapter
    Note over DI: Creates SourceParserAdapter
    CLI->>CS: register_check_commands(container)
    CLI->>CS: check(path, git_diff)

    CS->>AL: container.get_architecture_linter()
    AL-->>CS: ArchLintHandler

    CS->>AL: linter.run_self_lint(path)
    AL->>AH: run_lint_with_deps(src_dir, fs, parser)
    AH->>AC: ArchComplianceAnalyzer::new(config, fs, parser)

    loop For each .rs file
        AC->>NC: check_file_naming()
        AC->>IC: check_internal_rules()
        AC->>MC: check_line_counts()
    end

    AC-->>AH: Vec of LintResult
    AH-->>AL: LintResultList
    AL-->>CS: LintResultList
    CS->>CS: format_report()
    CS-->>U: Formatted output
```

#### DI Container Wiring Flow

```mermaid
%%{init: {'theme': 'default', 'themeVariables': { 'primaryColor': '#ffffff', 'primaryTextColor': '#000000', 'primaryBorderColor': '#333333', 'lineColor': '#333333' }}}%%
graph TB
    BE["Binary Entry Point"] --> DI

    DI["DependencyInjectionContainer::new()"] --> FS
    DI --> SC
    DI --> EX
    DI --> JR

    FS["OSFileSystemAdapter"] --> TFS
    SC["SourceParserAdapter"] --> TSC
    EX["StdioClient"] --> TEX
    JR["MemoryJobRegistryAdapter"] --> TJD

    SC --> PP["ASTPythonParserAdapter"]
    SC --> RP["ASTRustParserAdapter"]
    SC --> JP["ASTJSParserAdapter"]

    TFS["Arc of dyn IFileSystemPort"] --> SC_GET
    TSC["Arc of dyn ISourceParserPort"] --> SC_GET
    TEX["Arc of dyn ICommandExecutorPort"] --> SC_JOB
    TJD["Arc of dyn JobRegistryAggregate"] --> SC_JOB

    SC_GET["get_architecture_linter()"] --> AH
    AH["ArchLintHandler::new(fs, parser)"]

    SC_JOB["get_job_registry()"] --> JR2["JobRegistry"]
```

#### Pipeline Orchestration Flow

```mermaid
%%{init: {'theme': 'default', 'themeVariables': { 'primaryColor': '#ffffff', 'primaryTextColor': '#000000', 'primaryBorderColor': '#333333', 'lineColor': '#333333' }}}%%
graph LR
    subgraph "Stage 1: Scan"
        S1["collect_rs_files()"]
        S2["load_config()"]
    end

    subgraph "Stage 2: Analyze"
        A1["ArchComplianceAnalyzer"]
        A2["run_analysis()"]
        A3["run_project_wide_checks()"]
    end

    subgraph "Stage 3: Checkers"
        C1["ArchNamingChecker"]
        C2["ArchInternalChecker"]
        C3["ArchMetricChecker"]
        C4["ArchImportRuleChecker"]
        C5["ArchQualityChecker"]
    end

    subgraph "Stage 4: Project-Wide"
        P1["DependencyCycleAnalyzer"]
        P2["ArchOrphanAnalyzer"]
        P3["ArchInheritanceChecker"]
        P4["ArchRoleChecker"]
    end

    subgraph "Stage 5: Output"
        O1["LintResultList"]
        O2["format_report()"]
    end

    S1 --> A1
    S2 --> A1
    A1 --> A2
    A2 --> C1
    A2 --> C2
    A2 --> C3
    A2 --> C4
    A2 --> C5
    A1 --> A3
    A3 --> P1
    A3 --> P2
    A3 --> P3
    A3 --> P4
    C1 --> O1
    C2 --> O1
    C3 --> O1
    C4 --> O1
    C5 --> O1
    P1 --> O1
    P2 --> O1
    P3 --> O1
    P4 --> O1
    O1 --> O2
```

### 3. Dependency Inversion

Higher-level orchestrating layers (Agent, Surfaces) never import concrete implementations. Instead, they interact with implementations exclusively through interfaces declared in the Contract layer using Dependency Injection.

### 4. The 3-Word Naming Philosophy (Virtual Namespacing)

To solve the "Scattered Feature Problem" inherent in layered architectures, AES abandons deep physical folder nesting in favor of a **3-Word File Naming Convention**. Every file acts as a 3D coordinate system: `[prefix]_[middle]_[suffix]`.

1. **Prefix (Virtual Folder / Bounded Context):** Groups files by domain or module (e.g., `auth`, `payment`, `lint`). This allows developers and AI to find all files related to a feature across all 6 layers instantly.
2. **Middle (Unique Responsibility):** A single word defining the core concept or single responsibility of the file (e.g., `token`, `invoice`, `session`).
3. **Suffix (Role Category):** Defines the architectural layer, behavioral contract, and import rules (e.g., `_vo`, `_port`, `_orchestrator`).

_Example:_ `auth_session_port.rs` instantly tells us the domain (auth), the concept (session), and the architectural role (port).

---

## Detailed Layer Specifications

Listed from the innermost (core) to the outermost (edge) layer.

### 1. Taxonomy: The Domain Foundation

- **Path**: `src/taxonomy/`
- **Allowed Suffixes**: `_vo`, `_entity`, `_event`, `_error`, `_constant`
- **Allowed Imports**: Restricted strictly to `src/taxonomy/`. Outer imports trigger an **AES001** violation.
- **Description**: Contains pure, framework-agnostic domain models, value objects, and business entities. It has zero external dependencies and represents the fundamental vocabulary of the system.
- **Components**:
  - **Value Object (`_vo`)**: Immutable data containers encapsulating domain constraints. Constructed at runtime, identified by value, may carry behavior (methods, validation). Primitive types (raw `str`, `int`) are forbidden in core entities and must be wrapped in VOs (**AES006**). _Ex: `auth_token_vo.rs`_
  - **Entity (`_entity`)**: Stateful domain concepts with unique IDs and lifecycle transitions. _Ex: `user_profile_entity.rs`_
  - **Event (`_event`)**: Immutable snapshots of domain facts. _Ex: `lint_scan_event.rs`_
  - **Error (`_error`)**: Specialized domain-level exceptions. _Ex: `file_system_error.rs`_
  - **Constant (`_constant`)**: Compile-time literals (`pub const` / `pub static` in Rust, module-level `Final` in Python) representing fixed domain values: protocol versions, validation bounds, enumerated literals, and other system-wide invariants. Identified by name (not value), zero runtime construction, zero behavior. The only Taxonomy role permitted to expose raw primitives (**AES006** exception) since constants are primitives by definition. Must contain _only_ constant declarations — `struct`/`enum`/`fn`/`impl` blocks are forbidden and trigger **AES033**. Use this role for cross-cutting values shared across multiple VOs or layers; values that constrain a single VO should be expressed as associated constants on that VO instead. _Ex: `mcp_protocol_constant.rs`, `source_extension_constant.rs`_

### 2. Contract: The Abstraction Boundaries

- **Path**: `src/contract/`
- **Allowed Suffixes**: `_port`, `_protocol`, `_aggregate`
- **Allowed Imports**: `src/taxonomy/` and `src/contract/`. Importing implementation layers is strictly forbidden.
- **Description**: The system's formal promises. Defines _what_ can be done without defining _how_.
- **Components**:
  - **Port (`_port`)**: Outbound interfaces for technical operations (I/O, DB, Network). Implemented by Infrastructure. _Ex: `file_system_port.rs`_
  - **Protocol (`_protocol`)**: Inbound interfaces for use cases or domain calculations. Implemented by Capabilities. _Ex: `arch_rule_protocol.rs`_
  - **Aggregate (`_aggregate`)**: Composition-based facades grouping related ports/protocols. _`service_container_aggregate.rs`_

### 3. Capabilities: Domain Logic and Core Use Cases

- **Path**: `src/capabilities/`
- **Allowed Suffixes**: `_checker`, `_analyzer`, `_processor`, `_evaluator`, `_resolver`, `_validator`, `_formatter`, `_handler`
- **Allowed Imports**: `src/taxonomy/` and `src/contract/`.
- **Description**: Implements core business logic, policies, and algorithms. Entirely agnostic of concrete infrastructure.
- **Components**:
  - **Checker/Analyzer (`_checker`, `_analyzer`)**: Evaluates specific audit rules. _Ex: `arch_import_checker.rs`_
  - **Processor/Resolver (`_processor`, `_resolver`)**: Orchestrates transformations or graph operations. _Ex: `orphan_graph_resolver.rs`_
  - **Evaluator (`_evaluator`)**: Coordinates multiple checkers to score complex rules. _Ex: `architecture_rule_evaluator.rs`_

### 4. Infrastructure: Technical and Adapter Layer

- **Path**: `src/infrastructure/`
- **Allowed Suffixes**: `_adapter`, `_provider`, `_scanner`, `_client`, `_constants`, `_schemas`, `_lifespan`, `_validator`, `_wrapper`
- **Allowed Imports**: `src/taxonomy/` and `src/contract/`. Sibling infrastructure imports are forbidden to enforce isolation.
- **Description**: Houses technical implementations, external library wrappers, and system drivers.
- **Components**:
  - **Adapter (`_adapter`)**: Implements concrete ports for external tools. _Ex: `python_ruff_adapter.rs`_
  - **Scanner (`_scanner`)**: Interfaces with raw hardware/platform APIs. _Ex: `os_fs_scanner.rs`_
  - **Provider (`_provider`)**: Delivers technical configuration or utilities. _Ex: `config_yaml_provider.rs`_

### 5. Agent: System Governance and Dependency Injection

- **Path**: `src/agent/`
- **Allowed Suffixes**: `_container`, `_orchestrator`, `_manager`, `_registry`, `_coordinator`
- **Allowed Imports**: `src/taxonomy/`, `src/contract/`, `src/capabilities/`, `src/infrastructure/`, and sibling agent components.
- **Description**: The orchestrator of the system. Governs execution flow, sets up DI, and wires capabilities/infrastructure.
- **Components**:
  - **Container (`_container`)**: Purely structural DI wiring. Zero business logic . _Ex: `dependency_injection_container.rs`_
  - **Orchestrator (`_orchestrator`)**: Conducts sequential flow for a single domain goal. Must be completely stateless between calls (**AES021**). _Ex: `arch_compliance_orchestrator.rs`_
  - **Coordinator (`_coordinator`)**: Orchestrates high-level policies across multiple orchestrators. _Ex: `arch_compliance_coordinator.rs`_
  - **Registry (`_registry`)**: Thread-safe, passive inventory store for CRUD/state. _Ex: `pipeline_job_registry.rs`_
  - **Manager (`_manager`)**: Supervises lifecycles and background runners. _Ex: `lifecycle_state_manager.rs`_

#### DI Wiring Flow (How Agent Connects Everything)

```mermaid
%%{init: {'theme': 'default', 'themeVariables': { 'primaryColor': '#ffffff', 'primaryTextColor': '#000000', 'primaryBorderColor': '#333333', 'lineColor': '#333333' }}}%%
graph TB
    subgraph "Binary Entry Point (Composition Root)"
        BE["cli_main_entry.rs / mcp_main_entry.rs"]
    end

    subgraph "Agent Layer"
        DI["DependencyInjectionContainer<br/><i>Creates concrete adapters</i>"]
        LO["ArchitectureLintOrchestrator<br/><i>Coordinates pipeline</i>"]
        PL["PipelineLintOrchestrator<br/><i>Chains stages</i>"]
    end

    subgraph "Contract (Interfaces)"
        SC["ServiceContainerAggregate<br/><i>get_architecture_linter()</i>"]
        FP["IFileSystemPort<br/><i>read/write operations</i>"]
        SP["ISourceParserPort<br/><i>parse source code</i>"]
        AL["IArchLintProtocol<br/><i>run_self_lint()</i>"]
    end

    subgraph "Infrastructure (Implementations)"
        FS["OSFileSystemAdapter<br/><i>implements IFileSystemPort</i>"]
        SA["SourceParserAdapter<br/><i>implements ISourceParserPort</i>"]
        PP["ASTPythonParserAdapter"]
        RP["ASTRustParserAdapter"]
        JP["ASTJSParserAdapter"]
    end

    subgraph "Capabilities (Business Logic)"
        AH["ArchLintHandler<br/><i>implements IArchLintProtocol</i>"]
        AC["ArchComplianceAnalyzer<br/><i>runs all checkers</i>"]
    end

    BE -->|"creates"| DI
    DI -->|"implements"| SC
    DI -->|"creates & injects"| FS
    DI -->|"creates & injects"| SA
    SA -->|"delegates to"| PP
    SA -->|"delegates to"| RP
    SA -->|"delegates to"| JP

    SC -->|"returns"| AL
    AL -->|"implemented by"| AH
    AH -->|"uses"| AC

    LO -->|"uses"| SC
    PL -->|"uses"| LO
```

#### Surface → Agent Communication (Trait-Only)

```mermaid
%%{init: {'theme': 'default'}}%%
sequenceDiagram
    participant S as Surfaces
    participant SC as ServiceContainerAggregate
    participant DI as DependencyInjectionContainer
    participant AL as IArchLintProtocol
    participant AH as ArchLintHandler

    Note over S,AH: Step 1: Surface requests linter from container
    S->>SC: container.get_architecture_linter()
    SC->>DI: Dispatched to Agent implementation
    DI-->>SC: Returns trait object
    SC-->>S: Option of Arc of dyn IArchLintProtocol

    Note over S,AH: Step 2: Surface calls linter method
    S->>AL: linter.run_self_lint(path)
    AL->>AH: Dispatched to Capabilities implementation
    AH-->>AL: LintResultList
    AL-->>S: Returns results

    Note over S: Surface only knows traits, never sees Agent or Capabilities
```

**Key Principle**: Surface interacts with `contract` **only**

### 6. Surfaces: External Interfaces and Entrypoints

- **Path**: `src/surfaces/`
- **Allowed Suffixes**: `_command`, `_handler`, `_controller`, `_page`, `_view`, `_component`
- **Allowed Imports**: `src/taxonomy/`, `src/contract/`. Direct imports to capabilities/infrastructure/agent are forbidden.
- **Description**: The outermost layer interfacing with users, terminals, or client applications.
- **Components**:
  - **Smart Surfaces**: Parse input, delegate to Agent orchestrators, return structured output. _Ex: `cli_check_command.rs`, `mcp_server_handler.rs`_
  - **Passive Surfaces**: Dumb, presentation-only components. Receive read-only VOs, never import agents/contracts (**AES019**). _Ex: `dashboard_view.rs`_
