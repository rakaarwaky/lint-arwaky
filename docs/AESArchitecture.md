# AES Architecture: Agentic Engineering System

The Agentic Engineering System Architecture, referred to as AES Architecture, is a strictly layered and highly decoupled architectural pattern. It is designed to achieve maximum modularity, absolute testability, and extreme maintainability by enforcing rigid structural boundaries. Under the AES paradigm, technical details are isolated, domain models are protected, and dependencies are strictly inverted via abstract contracts.

---

## Core Pillars and Philosophy

### 1. Strict Layered Boundary Enforcement
The codebase is divided into six distinct horizontal and vertical boundaries. Layers can only communicate using downward-only dependency paths to prevent coupling and circular dependencies. Any violation of these import boundaries is caught at compile or lint time by static analysis checkers.

### 2. Sibling Equivalence and Peer Layers
Unlike traditional three-tier architectures, Capabilities and Infrastructure are horizontal peer layers.
* Neither layer is above or below the other.
* Neither layer can ever import from or know about the other.
* Both layers depend downward on the Contract layer via Ports and Protocols.

```
                      +-------------------+
                      |      Surfaces     |
                      +---------+---------+
                                |
                                v
                      +-------------------+
                      |       Agent       |
                      +---------+---------+
                                |
                                v
                      +-------------------+
                      |      Contract     |
                      +----+---------+----+
                           |         |
                 +---------+         +---------+
                 |                             |
                 v                             v
       +-------------------+         +-------------------+
       |    Capabilities   |         |   Infrastructure  |
       +---------+---------+         +---------+---------+
                 |                             |
                 +---------+         +---------+
                           |         |
                           v         v
                      +-------------------+
                      |      Taxonomy     |
                      +-------------------+
```

### 3. Dependency Inversion
Higher-level orchestrating layers such as Agent and Surfaces never import concrete implementations. Instead, they interact with implementations exclusively through interfaces declared in the Contract layer using Dependency Injection.

### 4. Rule-Based Role Specialization
Every source file must strictly declare its architectural role via filename suffixes such as _vo, _port, _adapter, or _orchestrator. This strict naming convention defines exactly which rules apply to that file and limits its permitted imports.

---

## Detailed Layer Specifications

Here is the comprehensive specification of each layer in the AES architecture, listed from the innermost to the outermost layer, mapped directly to real examples from the src/ directory.

---

### 1. Taxonomy: The Domain Foundation
* **Path**: src/taxonomy/
* **Allowed Suffixes**: _vo, _entity, _event, _error
* **Allowed Imports**: restricted to the src/taxonomy/ directory. Outer imports are strictly forbidden, which leads to an AES001 violation.
* **Description**: Contains pure, framework-agnostic domain models, value objects, and business entities. It has zero external dependencies and represents the fundamental vocabulary of the entire system.

#### Roles and Suffixed Components:
* **Value Object: suffix _vo**: Immutable data containers that encapsulate domain constraints. They contain no logic except validation. Primitive types such as raw str, int, or float are forbidden in core entities and must be wrapped in VOs to enforce rule AES006.
  * *Example*: src/taxonomy/error_code_vo.py which encapsulates the AESXXX formatting rule, or src/taxonomy/lint_result_vo.py.
* **Entity: suffix _entity**: Stateful domain concepts containing unique identifiers, lifecycle transitions, and business rules composed of VOs.
  * *Example*: src/taxonomy/governance_report_entity.py.
* **Event: suffix _event**: Immutable snapshots of facts that have occurred in the domain.
  * *Example*: src/taxonomy/lint_scan_event.py.
* **Error: suffix _error**: Specialized domain-level exceptions to ensure error structures do not leak technical implementations.
  * *Example*: src/taxonomy/file_system_error.py.

---

### 2. Contract: The Abstraction Boundaries
* **Path**: src/contract/
* **Allowed Suffixes**: _port, _protocol, _aggregate
* **Allowed Imports**: src/taxonomy/ and other files in src/contract/. It is absolutely forbidden to import any implementation layers such as infrastructure, capabilities, agent, or surfaces.
* **Description**: The system's formal promises. It defines what can be done by the system without defining how.

#### Roles and Suffixed Components:
* **Port: suffix _port**: Abstract interfaces defining outbound requirements for technical operations such as I/O, network, or scanning. Must be implemented by Infrastructure.
  * *Example*: src/contract/file_system_port.py which defines OS file system operations, or src/contract/linter_adapter_port.py.
* **Protocol: suffix _protocol**: Abstract interfaces defining inbound operations for use cases or domain calculations. Must be implemented by Capabilities.
  * *Example*: src/contract/arch_rule_protocol.py which defines how architectural linter rules are evaluated, or src/contract/dispatch_routing_protocol.py.
* **Aggregate: suffix _aggregate**: Composition-based aggregates or facades that group related ports and protocols to define clean subsystem boundaries. It is forbidden to inherit from Port or Protocol directly to enforce rule AES026.
  * *Example*: src/contract/service_container_aggregate.py.

---

### 3. Capabilities: Domain Logic and Core Use Cases
* **Path**: src/capabilities/
* **Allowed Suffixes**: _checker, _analyzer, _processor, _evaluator, _resolver, _validator, _formatter, _handler
* **Allowed Imports**: src/taxonomy/ and src/contract/. It is strictly forbidden to import infrastructure, agent, or surfaces to enforce rule AES001.
* **Description**: Implements the core business logic, policies, and algorithms of the system. Capabilities are entirely agnostic of concrete infrastructure details such as databases, CLI, or HTTP.

#### Roles and Suffixed Components:
* **Checker: suffix _checker** or **Analyzer: suffix _analyzer**: Components implementing specific evaluation or audit rules.
  * *Example*: src/capabilities/mcp_tool_schema_checker.py which validates JSON schemas for FastMCP, or src/capabilities/arch_import_checker.py which validates layered imports.
* **Processor: suffix _processor** or **Resolver: suffix _resolver**: Orchestrates transformations or graph operations.
  * *Example*: src/capabilities/arch_import_processor.py, or src/capabilities/orphan_graph_resolver.py.
* **Evaluator: suffix _evaluator**: Coordinates multiple checkers to score and evaluate complex rules.
  * *Example*: src/capabilities/architecture_rule_evaluator.py.

---

### 4. Infrastructure: Technical and Adapter Layer
* **Path**: src/infrastructure/
* **Allowed Suffixes**: _adapter, _provider, _scanner, _client, _constants, _schemas, _lifespan, _validator, _wrapper
* **Allowed Imports**: src/taxonomy/ and src/contract/. Absolutely forbidden to import capabilities, agent, surfaces, or sibling infrastructure components to enforce isolation and loose coupling.
* **Description**: Houses technical implementations, external library wrappers, and adapters for system drivers.

#### Roles and Suffixed Components:
* **Adapter: suffix _adapter**: Implements concrete ports defined in Contract to interface with external tools or system packages.
  * *Example*: src/infrastructure/python_ruff_adapter.py for Ruff linter integration, or src/infrastructure/python_mypy_adapter.py.
* **Scanner: suffix _scanner**: Interfaces with raw hardware or platform APIs to traverse state.
  * *Example*: src/infrastructure/os_fs_scanner.py which scans active workspaces, or src/infrastructure/git_diff_scanner.py.
* **Provider: suffix _provider**: Delivers technical configuration or structural utilities.
  * *Example*: src/infrastructure/config_yaml_provider.py which parses YAML configs for rule sets.

---

### 5. Agent: System Governance and Dependency Injection
* **Path**: src/agent/
* **Allowed Suffixes**: _container, _orchestrator, _manager, _registry, _coordinator
* **Allowed Imports**: src/taxonomy/, src/contract/, src/capabilities/, src/infrastructure/, and sibling agent components.
* **Description**: The orchestrator of the system. It governs how the application runs, sets up dependency injection, and wires up capabilities and infrastructure using contract interfaces.

#### Roles and Suffixed Components:
* **Container: suffix _container**: Orchestrates structural wiring and Dependency Injection. Must be purely structural and contain zero business or domain logic to enforce rule AES021.
  * *Example*: src/agent/dependency_injection_container.py.
* **Orchestrator: suffix _orchestrator**: Conducts the sequential flow of a single, highly specialized domain execution goal. Orchestrators must remain completely stateless between calls to enforce rule AES021.
  * *Example*: src/agent/arch_compliance_orchestrator.py which coordinates workspace audits, or src/agent/lint_fix_orchestrator.py.
* **Coordinator: suffix _coordinator**: Orchestrates strategic high-level policies across multiple orchestrators.
  * *Example*: src/agent/arch_compliance_coordinator.py.
* **Registry: suffix _registry**: Acts as a safe, concurrent inventory store for CRUD actions. It must be a passive dumb data store and remain thread-safe to enforce rule AES021.
  * *Example*: src/agent/agent_job_registry.py.
* **Manager: suffix _manager**: Supervises and drives application lifecycles, background runners, and system health states.
  * *Example*: src/agent/lifecycle_state_manager.py.

---

### 6. Surfaces: External Interfaces and Entrypoints
* **Path**: src/surfaces/
* **Allowed Suffixes**: _command, _handler, _controller, _page, _view, _component
* **Allowed Imports**: src/taxonomy/, src/contract/, and src/agent/. Under no circumstances can surfaces import capabilities or infrastructure directly to enforce clean boundary inversion.
* **Description**: The outermost layer interfacing with users, terminal interfaces, or client applications.

#### Roles and Suffixed Components:
* **Smart Surfaces**: Take user or client input, convert it to domain types, delegate execution to the Agent orchestrators, and return structured output.
  * *Example for CLI Commands*: src/surfaces/cli_check_command.py, or src/surfaces/cli_fix_command.py.
  * *Example for MCP Handlers*: src/surfaces/mcp_server_handler.py, or src/surfaces/mcp_execute_command.py.
* **Passive Surfaces**: Dumb, presentation-only components for views or web layouts. They can only receive read-only taxonomy VOs and must never import agents or contracts to enforce rule AES019.

---

## Core Architectural Rules: Enforced by Linter

The linter implements strict code analysis matching these documented definitions:

* **AES001: Import Layer Violation**: Enforces that files do not import from forbidden layers, such as taxonomy importing infrastructure, or capabilities importing surfaces.
* **AES002: Mandatory Import Missing**: Enforces that domain models are properly composed of Value Objects by requiring taxonomy imports.
* **AES003: Naming Convention**: Enforces strict three-word snake_case filenames such as error_code_vo.py for architectural consistency.
* **AES005: File Too Short**: Catches empty or cluttered files to ensure logic is cohesive.
* **AES006: Primitive Usage**: Flags raw primitive types such as raw str instead of a Value Object in domain entities.
* **AES012: Barrel Completeness**: Ensures all public layer APIs are explicitly exported via __init__.py or index.ts barrels.
* **AES014: Bypass Comment Violation**: Rejects bypass code comments such as # noqa or type ignore.
* **AES016: Dead Inheritance Bypass**: Rejects hollow inheritance bypasses where a class inherits from a contract but implements nothing.
* **AES021: Agent Role Violation**: Ensures agents strictly comply with their behavioral constraints such as forbidding state in orchestrators and forbidding logic in containers.
* **AES026: Forbidden Contract Inheritance**: Prevents aggregate contracts such as aggregates or facades from inheriting from ports or protocols, forcing composition instead.
* **AES027: Mandatory Contract Inheritance**: Ensures that files in implementation layers such as capabilities or infrastructure that import a contract interface actually implement or inherit from it.
