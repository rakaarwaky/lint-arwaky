# Crate: import-rules (v1.10.115)

This document contains the source code for feature crate `import-rules` along with its corresponding and imported definitions from the `shared` crate.

## Problem Statement

The following issues were detected by `lint-arwaky-cli scan`:

```
Lint Arwaky v1.10.115 — Scan Report
Target: /home/raka/mcp-arwaky/lint-arwaky/crates/import-rules
Total: 0 violations
```

---

## File List

- [ARCHITECTURE.md](file:///home/raka/mcp-arwaky/lint-arwaky/ARCHITECTURE.md)
- [crates/import-rules/Cargo.toml](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/Cargo.toml)
- [crates/import-rules/FRD.md](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/FRD.md)
- [crates/import-rules/src/agent_import_orchestrator.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/src/agent_import_orchestrator.rs)
- [crates/import-rules/src/capabilities_cycle_import_analyzer.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/src/capabilities_cycle_import_analyzer.rs)
- [crates/import-rules/src/capabilities_dummy_import_checker.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/src/capabilities_dummy_import_checker.rs)
- [crates/import-rules/src/capabilities_import_forbidden_checker.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/src/capabilities_import_forbidden_checker.rs)
- [crates/import-rules/src/capabilities_import_mandatory_checker.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/src/capabilities_import_mandatory_checker.rs)
- [crates/import-rules/src/capabilities_import_unused_checker.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/src/capabilities_import_unused_checker.rs)
- [crates/import-rules/src/lib.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/src/lib.rs)
- [crates/import-rules/src/root_import_rules_container.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/import-rules/src/root_import_rules_container.rs)
- [crates/shared/src/cli-commands/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/mod.rs)
- [crates/shared/src/cli-commands/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs)
- [crates/shared/src/common/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/mod.rs)
- [crates/shared/src/common/taxonomy_adapter_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_error.rs)
- [crates/shared/src/common/taxonomy_common_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_error.rs)
- [crates/shared/src/common/taxonomy_common_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_vo.rs)
- [crates/shared/src/common/taxonomy_message_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_message_vo.rs)
- [crates/shared/src/common/taxonomy_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_name_vo.rs)
- [crates/shared/src/common/taxonomy_path_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_path_vo.rs)
- [crates/shared/src/common/taxonomy_paths_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_paths_vo.rs)
- [crates/shared/src/common/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_severity_vo.rs)
- [crates/shared/src/common/taxonomy_source_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_source_vo.rs)
- [crates/shared/src/common/utility_file_handler.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/utility_file_handler.rs)
- [crates/shared/src/common/utility_layer_detector.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/utility_layer_detector.rs)
- [crates/shared/src/common/utility_scope_matcher.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/utility_scope_matcher.rs)
- [crates/shared/src/config-system/contract_config_orchestrator_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_config_orchestrator_aggregate.rs)
- [crates/shared/src/config-system/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/mod.rs)
- [crates/shared/src/config-system/taxonomy_config_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_config_vo.rs)
- [crates/shared/src/import-rules/contract_cycle_import_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_cycle_import_protocol.rs)
- [crates/shared/src/import-rules/contract_dummy_import_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_dummy_import_protocol.rs)
- [crates/shared/src/import-rules/contract_import_forbidden_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_import_forbidden_protocol.rs)
- [crates/shared/src/import-rules/contract_import_mandatory_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_import_mandatory_protocol.rs)
- [crates/shared/src/import-rules/contract_import_runner_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_import_runner_aggregate.rs)
- [crates/shared/src/import-rules/contract_unused_import_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_unused_import_protocol.rs)
- [crates/shared/src/import-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/mod.rs)
- [crates/shared/src/import-rules/taxonomy_dependency_edge_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_dependency_edge_vo.rs)
- [crates/shared/src/import-rules/taxonomy_forbidden_rule_config_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_forbidden_rule_config_vo.rs)
- [crates/shared/src/import-rules/taxonomy_graph_color_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_graph_color_vo.rs)
- [crates/shared/src/import-rules/taxonomy_import_constant.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_import_constant.rs)
- [crates/shared/src/import-rules/taxonomy_violation_import_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_violation_import_vo.rs)
- [crates/shared/src/import-rules/utility_cycle_detector.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/utility_cycle_detector.rs)
- [crates/shared/src/import-rules/utility_dummy_detector.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/utility_dummy_detector.rs)
- [crates/shared/src/import-rules/utility_import_module_parser.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/utility_import_module_parser.rs)
- [crates/shared/src/import-rules/utility_import_resolver.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/utility_import_resolver.rs)
- [crates/shared/src/import-rules/utility_import_symbol_extractor.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/utility_import_symbol_extractor.rs)
- [crates/shared/src/import-rules/utility_path_normalizer.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/utility_path_normalizer.rs)

---

## File: ARCHITECTURE.md

```rust
# Agentic Engineering System Architecture

## 1. Purpose

The Agentic Engineering System is a layered, AI-native architecture pattern. It keeps domain models stable, business logic readable, technical detail isolated, and layer boundaries explicit enough for both humans and AI agents to modify the system safely.

---

## 2. Workspace Organization

The architecture supports multi-language workspaces.

| Term               | Meaning                                                           |
| ------------------ | ----------------------------------------------------------------- |
| Project Workspaces | Project root containing all configuration and language members    |
| Workspace Member   | One self-contained crate, package, or module inside the workspace |
| Crates directory   | Rust workspace members                                            |
| Packages directory | TypeScript or JavaScript packages                                 |
| Modules directory  | Python modules or sub-projects                                    |

---

## 3. Naming Convention

File names must communicate three parts:

1. Layer as prefix
2. Concern as middle name
3. Role as suffix

The parts are joined by underscores, followed by the normal file extension for the language.

`layer_concern_role.rs/py/ts`

---

## 4. Vertical Slicing Folder Structure

The recommended folder structure follows this order:

#### Features member

_Example feature crate `crates|packages|modules/<name-features>/`_

``` `text
surface_<concern>_<role>.rs/py/ts                ← surface layer
capabilities_<concern>_<role>.rs/py/ts           ← capabilities layer
agent_<concern>_orchestrator.rs/py/ts            ← agent layer
``` `

Exceptions: `main.rs`, `lib.rs`, `mod.rs`, `__init__.py`, `index.ts`, `index.js`.

#### Shared member

`crates|packages|modules/shared/<common>or<domain-folder>`

``` `text
contract_<concern>_protocol.rs/py/ts             ← contract layer
contract_<concern>_aggregate.rs/py/ts            ← contract layer
taxonomy_<concern>_vo.rs/py/ts                   ← taxonomy layer
taxonomy_<concern>_event.rs/py/ts                ← taxonomy layer
taxonomy_<concern>_entity.rs/py/ts               ← taxonomy layer
taxonomy_<concern>_constant.rs/py/ts             ← taxonomy layer
utility_<concern>_<role>.rs/py/ts                ← utility layer
``` `

`shared` folder groups by domain. Use `shared/common/` for generic files.

---

## 5. Taxonomy Layer

### Purpose

Taxonomy is the domain foundation layer. It defines the stable language of the domain and must remain free from technical or behavioral concerns.

### Components

| Role         | Meaning                               |
| ------------ | ------------------------------------- |
| Value object | Immutable data concept                |
| Entity       | Stateful domain concept with identity |
| Event        | Immutable domain fact                 |
| Error        | Domain-level error                    |
| Constant     | Compile-time literal value            |

### Dependencies

Taxonomy depends on nothing.

### Special Rules

- Value objects and Constants may use all primitive types.
- Entities, Events, and Errors must use Value objects/Constants instead of primitive types (bool/str is an exception).
- Constants must be compile-time values.
- Taxonomy must not contain business rules, infrastructure, or imports from other layers.

---

## 6. Contract Layer

### Purpose

Contract defines the public behavior of the system without exposing implementation. It allows callers to depend on stable interfaces instead of concrete logic.

### Components

| Role      | Meaning                                                                                           |
| --------- | ------------------------------------------------------------------------------------------------- |
| Protocol  | Interface defining inbound behavior. It is implemented by Capabilities and consumed by the Agent. |
| Aggregate | Facade definition implemented by Agent, used by Surface to access feature behavior.               |

### Dependencies

Contract may depend on Taxonomy only.

### Special Rules

- Protocol defines behavior only without implementation.
- Aggregate hides Capabilities from Surface.

---

## 7. Utility Layer

### Purpose

Utility contains low-level technical mechanics. It exists so that Capabilities can remain clean and expressive.

### Role Naming

Utility role suffixes are unlimited. The role name is chosen based on demand and must describe the technical responsibility and concern of the file.

parser
splitter
trimmer
slugifier
sanitizer
normalizer
extractor
replacer
converter
counter
resolver
detector
builder
joiner
serializer
deserializer
encoder
decoder
hasher
generator
formatter
comparator
differ
matcher
checker
calculator
mapper
merger
grouper
sorter
deduplicator
printer

### Dependencies

Utility may depend only on Taxonomy.

### Technical Concern Examples

| Concern                 | Responsibility                                      |
| ----------------------- | --------------------------------------------------- |
| File discovery          | Walk directories, detect files, apply ignore        |
| External tool execution | Run linters, compilers, formatters, analyzers       |
| Parsing and matching    | Parse text, match patterns, extract structured data |
| Path normalization      | Normalize paths across platforms                    |
| System operations       | Handle process or environment mechanics             |

### Special Rules

- Utility must use stateless standalone functions only.
- Utility must not contain stateful objects, behavior definitions, or contract implementations.
- Utility must not make business decisions.
- Utility may perform technical operations if needed.
- Utility must not implement any contract.
- Utility role names may expand freely, but the layer must remain technical and standalone.
- Utility must use stateless standalone functions only.

---

## 8. Capabilities Layer

### Purpose

Capabilities contain the concrete implementation of the system's behavior. This layer encapsulates both **pure business logic** (computations, validations) and **external adaptations** (database access, third-party API calls, infrastructure mechanics). By hiding these implementations behind Contracts, the system keeps its behavior modular, swappable, and fully isolated from orchestration.

### Role Naming

#### Internal Examples

validator
assessor
calculator
resolver
classifier
selector
mapper
transformer
policy
enricher
evaluator
analyzer
scorer
grader
ranker
filter
checker
reviewer
approver
rejector

#### External Examples

repository
gateway
client
provider
fetcher
reader
writer
scanner
executor
publisher
subscriber
adapter
connector
uploader
downloader
sender
receiver
dispatcher
watcher
monitor

### Dependencies

- Capabilities may depend on Taxonomy, Contract, and Utility.
- Capabilities must not depend on or import other Capabilities.

### Concern Examples

Capabilities generally handle two types of concerns:

| Category                | Concern        | Responsibility                                 |
| ----------------------- | -------------- | ---------------------------------------------- |
| **Business Logic**      | Validation     | Check domain conditions or input correctness   |
|                         | Computation    | Calculate scores, totals, or derived values    |
|                         | Transformation | Map, filter, reduce, or reshape data           |
|                         | Resolution     | Apply rules and decide outcomes                |
|                         | Assessment     | Judge severity, compliance, grade, or quality  |
| **External Adaptation** | Repository     | Fetch or persist domain entities to a database |
|                         | Integration    | Communicate with third-party services or APIs  |
|                         | Provider       | Generate data from external systems            |

### Special Rules

- **No Inter-Capability Dependency:** Capabilities must never import or call other Capabilities directly. They are standalone execution units.
- **Pipeline Aggregation:** Multiple Capabilities (e.g., Capability A for data fetching, Capability B for business calculation) are designed to be composed into a sequential pipeline by the **Agent Layer**, not by themselves.
- **Shared Logic Extraction (DRY):** If multiple Capabilities require the same technical mechanics or functions, that logic must be extracted into a reusable standalone function in the **Utility Layer**. Capabilities must not duplicate technical code (Don't Repeat Yourself).
- **Contract Implementation:** Capabilities must implement the `protocol_` defined in the Contract Layer.
- **State Ownership:** Capabilities are the owners of business and technical state within their execution scope.
- **Utility Delegation:** Capabilities must call Utility standalone functions when low-level technical operations are required, passing their state/data as arguments.
- **No Orchestration:** Capabilities must not contain flow control (looping across capabilities, branching between capabilities, or error escalation policy). They execute their single responsibility and return a result.
- **No Domain Definition:** Capabilities must not define domain models (Entities, Value Objects); they only consume and produce Taxonomy.

---

## 9. Agent Layer

### Purpose

Agent coordinates multiple capabilities into executable flows. It controls sequence and movement, not business calculation.

### Allowed Role

The only Agent role is orchestrator.

### Dependencies

Agent may depend only on Taxonomy, Contract, and Utility.

### Allowed Flow Control

| Flow Type               | Purpose                                |
| ----------------------- | -------------------------------------- |
| Sequential execution    | Run steps in order                     |
| Looping                 | Process multiple items or events       |
| Branching               | Choose path based on result            |
| Error handling          | Recover, abort, continue, or escalate  |
| Timeout or cancellation | Stop long-running or asynchronous work |

### Special Rules

- Agent must depend on Contract, not concrete implementations.
- Agent must not use and must be completely ignorant of Capabilities implementations.
- Agent must not calculate business results.
- Agent must not define domain models.

---

## 10. Surface Layer

### Purpose

Surface is the outer boundary of the system. It handles user-facing or external-facing interaction and translates it into architectural actions.

### Allowed Roles

Surface roles include:

- command
- controller
- page
- view
- component
- router
- layout
- hook
- store
- action
- screen

### Surface Groups

| Group            | Roles                             | Dependencies                          | Rule                                            |
| ---------------- | --------------------------------- | ------------------------------------- | ----------------------------------------------- |
| Smart surfaces   | command, controller, page, router | Taxonomy, Contract Aggregate, Utility | May initiate feature behavior through aggregate |
| Utility surfaces | hook, store, action, screen       | Taxonomy, Contract Aggregate, Utility | Support smart surfaces but must not import smart surfaces |
| Passive surfaces | component, view, layout           | Taxonomy only                         | Presentation-only, no logic or orchestration    |

### Special Rules

- Smart surfaces must consume Contract Aggregates.
- Surfaces must not import Capabilities, Utility, or Agent directly.
- Surfaces must not contain business calculation or orchestration.

---

## 11. Root Layer

### Purpose

Root is the composition layer. It assembles the system by connecting concrete implementations to contracts and starting the application.

### Components

| Role      | Meaning                                                                           |
| --------- | --------------------------------------------------------------------------------- |
| Container | Wires one feature by connecting Capabilities to Contract protocols and aggregates |
| Entry     | Bootstraps the application and composes feature containers                        |

### Dependencies

Root may depend on all layers.

### Special Rules

- Root may instantiate and wire components.
- Root must not contain business logic.
- Root must not contain orchestration policy.
- Root must not contain technical parsing or user interface behavior.
```

---

## File: crates/import-rules/Cargo.toml

```toml
[package]
name = "import-rules-lint-arwaky"
version = "1.10.115"
edition = "2021"
description = "Import-compliance checks covering AES201–AES205: dummy/unused/forbidden/mandatory imports, layer detection, and cross-layer cycle detection."
license = "MIT"
repository = "https://github.com/rakaarwaky/lint-arwaky"
publish = true

[lints]
workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
async-trait.workspace = true
once_cell.workspace = true
regex.workspace = true
shared.workspace = true
tokio.workspace = true
rayon.workspace = true

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
tempfile = "3"

[[bench]]
name = "bench_import_rules_throughput"
path = "benches/bench_import_rules_throughput.rs"
harness = false
```

---

## File: crates/import-rules/FRD.md

```rust
# FRD — import-rules

## System Overview

The import-rules crate enforces correct structural boundaries and unidirectional dependency flows. It prevents spaghetti architecture, circular dependencies, and dead/unused imports by validating every import statement against a predefined layer-hierarchy matrix.

## Functional Requirements

### FR-001: Layer Dependency Violation (AES201)

- **Description**: Restricts imports based on the layer hierarchy. Lower layers must never import higher layers.
- **Input**: Rust, Python, JS/TS source files
- **Output**: List of layer violation diagnostics
- **Business Rules**:
  - taxonomy_ must not import contract_, utility_, capabilities_, agent_, surface_, root_
  - contract_ must not import utility_, capabilities_, agent_, surface_, root_
  - utility_ and capabilities_ must not import each other directly
- **Edge Cases**: Circular imports across layers
- **Error Handling**: Emit AES201 diagnostic with file path and line number

### FR-002: Mandatory Layer Imports (AES202)

- **Description**: Verifies that specific layers contain required imports.
- **Input**: Rust, Python, JS/TS source files
- **Output**: List of missing import diagnostics
- **Business Rules**:
  - Capability files must import their corresponding contract trait
  - Surface entries must import their container
- **Edge Cases**: Files with multiple roles
- **Error Handling**: Emit AES202 diagnostic with expected import

### FR-003: Unused Import Detection (AES203)

- **Description**: Detects and flags imported symbols that are never referenced within the file body.
- **Input**: Rust, Python, JS/TS source files
- **Output**: List of unused import diagnostics
- **Business Rules**:
  - Symbol must be referenced at least once after import
  - Wildcard imports (`use foo::*`) are flagged
- **Edge Cases**: Re-exports, pub use statements
- **Error Handling**: Emit AES203 diagnostic with unused symbol name

### FR-004: Dummy Import Detection (AES204)

- **Description**: Detects imports, functions, and trait implementations that are dummy/stub code existing only to suppress unused-import warnings.
- **Input**: Rust, Python, JS/TS source files
- **Output**: List of dummy import diagnostics
- **Business Rules**:
  - Imported symbols placed inside `_use_*` dummy functions are flagged
  - Dummy functions (`fn _use_*`, `def _use_*`, `function _use*`) are flagged
  - Trait implementations with empty bodies are flagged
  - Taxonomy VOs used only in dummy functions (not in real logic) are flagged
- **Edge Cases**: Re-exports, pub use statements
- **Error Handling**: Emit AES204 diagnostic with dummy symbol name and line number

### FR-005: Forbidden Import Config Detection (AES201)

- **Description**: Detects imports that violate layer boundary rules defined in YAML configuration.
- **Input**: Rust, Python, JS/TS source files
- **Output**: List of forbidden import diagnostics
- **Business Rules**:
  - Import path must not match forbidden patterns from config
  - Layer-specific forbidden rules are enforced per scope pattern
- **Edge Cases**: Conditional imports, feature flags
- **Error Handling**: Emit AES201 diagnostic with forbidden import path

### FR-006: Circular Dependency Detection (AES205)

- **Description**: Builds a dependency graph of imports across all workspace files and detects cycles.
- **Input**: All workspace source files
- **Output**: List of circular dependency diagnostics
- **Business Rules**:
  - Direct cycles (A → B → A) are flagged
  - Indirect cycles (A → B → C → A) are flagged
- **Edge Cases**: Self-imports, conditional cycles
- **Error Handling**: Emit AES205 diagnostic with cycle path

## API Contract

| Operation                       | Input                      | Output          | Description  |
| ------------------------------- | -------------------------- | --------------- | ------------ |
| Check for forbidden imports     | File path, imports         | Diagnostic list | Check AES201 |
| Check for mandatory imports     | File path, imports         | Diagnostic list | Check AES202 |
| Check for unused imports        | File path, imports, usages | Diagnostic list | Check AES203 |
| Check for dummy imports         | File path, imports         | Diagnostic list | Check AES204 |
| Check for circular dependencies | All files, imports         | Diagnostic list | Check AES205 |

## Integration Points

- **Internal**: config-system (YAML rules), shared/common (utility functions for file reading, layer detection, import parsing)
- **External**: None

## Non-functional Requirements (Detailed)

- Performance: Check 1000 files in < 2 seconds (validated via criterion benchmark in the benchmark suite)
- Memory: O(n) where n = number of imports
- Accuracy: Zero false positives for valid imports

## Test Scenarios / QA Checklist

### AES201 — Forbidden Import

| Test Case                   | Input                                                      | Expected Output           |
| --------------------------- | ---------------------------------------------------------- | ------------------------- |
| taxonomy imports contract   | `taxonomy_vo.rs` with `use contract_protocol::*`           | AES201 CRITICAL violation |
| capabilities imports agent  | `capabilities_checker.rs` with `use agent_orchestrator::*` | AES201 CRITICAL violation |
| valid unidirectional import | `capabilities_checker.rs` with `use taxonomy_vo::*`        | No violation              |

### AES202 — Mandatory Import

| Test Case               | Input                                             | Expected Output       |
| ----------------------- | ------------------------------------------------- | --------------------- |
| missing contract import | `capabilities_checker.rs` without protocol import | AES202 HIGH violation |
| present contract import | `capabilities_checker.rs` with protocol import    | No violation          |

### AES203 — Unused Import

| Test Case     | Input                                          | Expected Output         |
| ------------- | ---------------------------------------------- | ----------------------- |
| unused symbol | File with `use foo::Bar;` but `Bar` never used | AES203 MEDIUM violation |
| used symbol   | File with `use foo::Bar;` and `Bar` referenced | No violation            |

### AES204 — Dummy Import

| Test Case        | Input                                | Expected Output       |
| ---------------- | ------------------------------------ | --------------------- |
| dummy function   | File with `fn _use_imports() {}`     | AES204 HIGH violation |
| empty trait impl | File with `impl Trait for Struct {}` | AES204 HIGH violation |

### AES205 — Circular Dependency

| Test Case    | Input                    | Expected Output           |
| ------------ | ------------------------ | ------------------------- |
| direct cycle | A imports B, B imports A | AES205 CRITICAL violation |
| no cycle     | A imports B, B imports C | No violation              |

## Assumptions & Constraints

- Layer hierarchy is defined in config YAML
- Import statements are parsed via utility functions in shared/common (regex-based extraction, not full AST parsing)
- Workspace structure follows AES conventions

## Glossary

- **AES**: Agentic Engineering System
- **Layer**: Architectural boundary (taxonomy, contract, utility, capabilities, agent, surface, root)
- **Diagnostic**: Violation report with file path, line, column, rule code
- **Dummy Import**: Import that exists only to suppress unused-import warnings, placed inside `_use_*` functions
- **Forbidden Import**: Import that violates layer boundary rules defined in YAML configuration

## Reference

- PRD: [PRD.md](../../PRD.md)

---

## Appendix A: YAML Configuration Schema

The import-rules crate reads its configuration from `lint_arwaky.config.<language>.yaml` files. Below is the schema for the `architecture` section relevant to import rules.

### Top-Level Structure

``` `yaml
architecture:
  enabled: true # Master switch for all architecture rules
  rules: # Map of rule codes to their configurations
    AES201: { ... }
    AES202: { ... }
    AES203: { ... }
    AES204: { ... }
    AES205: { ... }
``` `

### Rule Configuration Schema

Each rule (AES201–AES205) follows this schema:

``` `yaml
AES201:
  enabled: true # Enable/disable this specific rule
  scope: # List of layer prefixes this rule applies to
    - "taxonomy"
    - "contract"
    - "utility"
    - "capabilities"
    - "agent"
    - "surface"
    - "root"
  exceptions: # Filenames to skip (basename match)
    - "main.rs"
    - "lib.rs"
    - "mod.rs"
  conditions: # (AES201/AES202 only) Layer-specific rules
    - scope: "taxonomy(vo)" # File scope pattern
      allowed: ["taxonomy"] # Layers this scope can import from
      mandatory: null # Required imports (null = none)
      forbidden: # Layers this scope cannot import from
        - "agent"
        - "surface"
        - "contract"
        - "utility"
        - "capabilities"
        - "root"
``` `

### Scope Pattern Syntax

Scope patterns use parentheses to specify sub-layers:

| Pattern                                        | Meaning                               |
| ---------------------------------------------- | ------------------------------------- |
| `taxonomy`                                     | All taxonomy files                    |
| `taxonomy(vo)`                                 | Only taxonomy value objects           |
| `taxonomy(entity,error,event)`                 | Taxonomy entities, errors, and events |
| `contract(protocol)`                           | Only contract protocols               |
| `contract(aggregate)`                          | Only contract aggregates              |
| `capabilities`                                 | All capability files                  |
| `agent(orchestrator)`                          | Only agent orchestrators              |
| `surface(command\|controller\|page)`           | Smart surfaces                        |
| `surface(hook\|store\|action\|screen\|router)` | Utility surfaces                      |
| `surface(component\|view\|layout)`             | Passive surfaces                      |

### Layer Hierarchy (Default)

The default layer hierarchy for import rules:

``` `
taxonomy (lowest)
  └── contract
       └── utility
       └── capabilities
            └── agent
                 └── surface
                      └── root (highest)
``` `

### Example: Minimal Config

``` `yaml
architecture:
  enabled: true
  rules:
    AES201:
      enabled: true
      scope: ["taxonomy", "contract", "capabilities"]
      exceptions: ["main.rs", "lib.rs"]
    AES202:
      enabled: true
      scope: ["capabilities"]
      exceptions: ["main.rs"]
    AES203:
      enabled: true
      scope: ["taxonomy", "contract", "capabilities"]
    AES204:
      enabled: true
      scope: ["taxonomy", "contract", "capabilities"]
    AES205:
      enabled: true
      scope: ["taxonomy", "contract", "capabilities"]
``` `

---

## Appendix B: File Discovery Algorithm

The file collection method of the import analysis orchestrator discovers source files for analysis. Here is the algorithm:

### Algorithm

``` `
collect_files(target_path):
  if target_path is a file:
    return [target_path]
  if target_path is a directory:
    return walk_dir(target_path)

walk_dir(dir, is_subdir=false):
  if is_subdir and is_ignored(dir):
    return []
  results = []
  for entry in read_dir(dir):
    if entry is a directory:
      results.extend(walk_dir(entry, is_subdir=true))
    else if entry is a file:
      if is_ignored(entry):
        continue
      if entry.extension in ["rs", "py", "js", "ts", "jsx", "tsx"]:
        results.append(entry)
  return results
``` `

### Ignore Rules

Files and directories are skipped if they match any of these criteria:

1. **Config-level ignores**: Paths listed in `ignored_paths` in the YAML config
2. **Default skip directories**: `.git`, `node_modules`, `target`, `dist`, `build`, `.venv`, `__pycache__`
3. **Hidden directories**: Any directory starting with `.` (e.g., `.github`, `.vscode`)
4. **File extension**: Only files with extensions `rs`, `py`, `js`, `ts`, `jsx`, `tsx` are collected

### Language Detection

Language is determined by file extension:

| Extension     | Language   |
| ------------- | ---------- |
| `.rs`         | Rust       |
| `.py`         | Python     |
| `.js`, `.jsx` | JavaScript |
| `.ts`, `.tsx` | TypeScript |

### Layer Detection

After file collection, each file's architectural layer is detected from its filename prefix:

| Filename Pattern    | Detected Layer |
| ------------------- | -------------- |
| `taxonomy_*.rs`     | taxonomy       |
| `contract_*.rs`     | contract       |
| `capabilities_*.rs` | capabilities   |
| `utility_*.rs`      | utility        |
| `agent_*.rs`        | agent          |
| `surface_*.rs`      | surface        |
| `root_*.rs`         | root           |

Files without a recognized prefix are skipped by layer-dependent rules (AES201, AES202) but still checked by layer-agnostic rules (AES203, AES204).
```

---

## File: crates/import-rules/src/agent_import_orchestrator.rs

```rust
// PURPOSE: ImportOrchestrator — agent that orchestrates import rule checks
// Uses new protocol interfaces — no IAnalyzer, no IArchImportProtocol.

use async_trait::async_trait;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::path::Path;
use std::sync::Arc;

use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_adapter_error::ScanError;
use shared::common::taxonomy_common_error::ErrorMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_source_vo::ContentString;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::import_rules::taxonomy_import_constant::DEFAULT_SKIP_DIRS;
use shared::taxonomy_definition_vo::LayerMapVO;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ImportOrchestratorDeps {
    pub mandatory: Arc<dyn IImportMandatoryProtocol>,
    pub forbidden: Arc<dyn IImportForbiddenProtocol>,
    pub unused: Arc<dyn IUnusedImportProtocol>,
    pub cycle: Arc<dyn ICycleImportProtocol>,
    pub dummy: Arc<dyn IDummyImportCheckerProtocol>,
}

pub struct ImportOrchestrator {
    deps: ImportOrchestratorDeps,
    layer_map: LayerMapVO,
    config: ArchitectureConfig,
    ignored_paths: Vec<String>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

#[async_trait]
impl IImportRunnerAggregate for ImportOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Result<Vec<LintResult>, ScanError> {
        if !self.config.enabled.value {
            return Ok(Vec::new());
        }
        let path = Path::new(target.value());
        if !path.exists() {
            return Err(ScanError::new(
                FilePath::new(target.value().to_string()).unwrap_or_default(),
                ErrorMessage::new(format!("Target path does not exist: {}", target.value())),
            ));
        }

        let files = self.collect_files(target);

        let root_dir = shared::common::utility_file_handler::find_workspace_root(target.value())
            .and_then(|p| FilePath::new(p.to_string_lossy().to_string()).ok())
            .unwrap_or_else(|| FilePath::new(".").unwrap_or_default());

        let (mandatory_results, forbidden_results) = tokio::join!(
            async {
                let mut r = LintResultList::new(Vec::new());
                self.deps
                    .mandatory
                    .run_mandatory_imports(&self.config, &self.layer_map, &files, &root_dir, &mut r)
                    .await;
                r
            },
            async {
                let mut r = LintResultList::new(Vec::new());
                self.deps
                    .forbidden
                    .check_forbidden_imports(
                        &self.config,
                        &self.layer_map,
                        &files,
                        &root_dir,
                        &mut r,
                    )
                    .await;
                r
            }
        );

        let root_dir_clone = root_dir.clone();
        let deps = &self.deps;
        let layer_map = &self.layer_map;

        let file_violations: Vec<LintResult> = files
            .values
            .par_iter()
            .flat_map(|file| {
                let mut local_results = Vec::new();
                if let Ok(content) = std::fs::read_to_string(file.value()) {
                    deps.unused
                        .check_unused_imports(file.value(), &content, &mut local_results);

                    let content_str = ContentString::new(content);
                    deps.dummy.check_all_dummy(
                        file,
                        &content_str,
                        &mut local_results,
                        &root_dir_clone,
                        layer_map,
                    );
                }
                local_results
            })
            .collect();

        let mut results = LintResultList::new(Vec::new());
        results.values.extend(mandatory_results.values);
        results.values.extend(forbidden_results.values);
        results.values.extend(file_violations);

        self.deps
            .cycle
            .check_cycles(
                &self.config,
                &self.layer_map,
                &files,
                &root_dir,
                &mut results,
            )
            .await;
        Ok(results.values)
    }

    fn name(&self) -> &str {
        "import-rules"
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl ImportOrchestrator {
    pub fn new(
        deps: ImportOrchestratorDeps,
        config: ArchitectureConfig,
        ignored_paths: Vec<String>,
    ) -> Self {
        let layer_map = LayerMapVO::new(config.layers.clone());

        Self {
            deps,
            config,
            layer_map,
            ignored_paths,
        }
    }

    fn is_ignored(&self, p: &Path) -> bool {
        let dir_name = p
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        if DEFAULT_SKIP_DIRS.contains(&dir_name.as_str()) || dir_name.starts_with('.') {
            return true;
        }
        let path_str = p.to_string_lossy();
        self.ignored_paths.iter().any(|ignored| {
            path_str.contains(ignored.as_str())
                || dir_name.contains(ignored.trim_start_matches('/'))
        })
    }

    fn collect_files(&self, target: &FilePath) -> FilePathList {
        let path = Path::new(target.value());
        let mut files = Vec::new();
        if path.is_dir() {
            self.walk_dir(path, &mut files, false);
        } else if path.is_file() {
            if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                files.push(fp);
            }
        }
        FilePathList::new(files)
    }

    fn walk_dir(&self, dir: &Path, files: &mut Vec<FilePath>, is_subdir: bool) {
        if is_subdir && self.is_ignored(dir) {
            return;
        }
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    self.walk_dir(&path, files, true);
                } else if path.is_file() {
                    if self.is_ignored(&path) {
                        continue;
                    }
                    if let Some(ext) = path.extension() {
                        if matches!(
                            ext.to_str(),
                            Some("rs" | "py" | "js" | "ts" | "jsx" | "tsx")
                        ) {
                            if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                                files.push(fp);
                            }
                        }
                    }
                }
            }
        }
    }
}
```

---

## File: crates/import-rules/src/capabilities_cycle_import_analyzer.rs

```rust
// PURPOSE: DependencyCycleAnalyzer — AES205: circular dependency detection
use async_trait::async_trait;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_layer_detector;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::{utility_cycle_detector, utility_import_module_parser};
use shared::taxonomy_definition_vo::LayerMapVO;
use shared::taxonomy_layer_vo::LayerNameVO;
use shared::taxonomy_message_vo::LintMessage;
use shared::taxonomy_name_vo::SymbolName;
use std::collections::HashMap;

// ─── Block 1: Struct Definition ───────────────────────────

#[derive(Default)]
pub struct DependencyCycleAnalyzer {}

type ScannedFileEdges = (Vec<DependencyEdge>, Option<(String, String)>);

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl ICycleImportProtocol for DependencyCycleAnalyzer {
    fn scan(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &[FilePath],
        root_dir: &FilePath,
    ) -> Vec<LintResult> {
        let file_strs: Vec<String> = files.iter().map(|f| f.to_string()).collect();
        let root_str = root_dir.to_string();
        self._scan(config, layer_map, &file_strs, &root_str)
    }

    async fn check_cycles(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let file_strs: Vec<String> = files.values.iter().map(|f| f.to_string()).collect();
        let cycle_violations = self._scan(config, layer_map, &file_strs, &root_dir.to_string());
        results.values.extend(cycle_violations);
    }

    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName> {
        utility_cycle_detector::detect_cycle_edges(edges)
    }

    fn normalize_to_layer(&self, name: &str) -> LayerNameVO {
        LayerNameVO::new(name.split('_').next().unwrap_or(name))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl DependencyCycleAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    fn _scan(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &[String],
        _root_dir: &str,
    ) -> Vec<LintResult> {
        if !config.enabled.value {
            return vec![];
        }
        let aes205_rule = config.rules.iter().find(|r| r.name.value == "AES205");
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();
        let layer_prefixes: Vec<String> = layer_keys.iter().map(|k| format!("{}_", k)).collect();

        let file_results: Vec<ScannedFileEdges> = files
            .par_iter()
            .filter_map(|file| {
                let file_fp = FilePath::new(file.clone()).ok()?;
                let basename = file_fp.basename();
                if let Some(rule) = aes205_rule {
                    if rule.exceptions.values.contains(&basename.to_string()) {
                        return None;
                    }
                }
                let content = shared::common::utility_file_handler::read_file_generic(file).ok()?;

                let filename = utility_layer_detector::extract_filename(file);
                let file_layer = match utility_layer_detector::detect_layer_from_prefix(filename) {
                    Some(l) => {
                        let specialized = utility_layer_detector::resolve_specialized_layer(
                            &l,
                            file,
                            &layer_keys,
                        );
                        let base_part = specialized
                            .find('(')
                            .map(|i| &specialized[..i])
                            .unwrap_or(&specialized);
                        base_part.to_string()
                    }
                    None => return None,
                };

                let modules = utility_import_module_parser::extract_import_modules(&content);
                let mut local_edges = Vec::new();
                let mut has_cross_layer = false;
                for module in modules {
                    let module_value = module.value();
                    let is_crate_import = module_value.starts_with("crate::")
                        || module_value.starts_with("lint_arwaky::");
                    let is_cross_layer_crate = if is_crate_import {
                        let stripped = module_value
                            .strip_prefix("crate::")
                            .or_else(|| module_value.strip_prefix("lint_arwaky::"))
                            .unwrap_or("");
                        let first_segment = stripped.split("::").next().unwrap_or("");
                        layer_prefixes
                            .iter()
                            .any(|prefix| stripped.starts_with(prefix))
                            || layer_keys.iter().any(|k| k == first_segment)
                    } else {
                        false
                    };
                    if is_crate_import && !is_cross_layer_crate {
                        continue;
                    }
                    let module_path = if is_crate_import {
                        module_value
                            .strip_prefix("crate::")
                            .or_else(|| module_value.strip_prefix("lint_arwaky::"))
                            .unwrap_or(module_value)
                    } else {
                        module_value
                    };
                    if let Some(target_layer) =
                        utility_layer_detector::detect_module_layer(module_path, &layer_keys)
                    {
                        let target_layer_str = match target_layer.split('(').next() {
                            Some(p) => p.to_string(),
                            None => target_layer,
                        };
                        if target_layer_str != file_layer {
                            local_edges
                                .push(DependencyEdge::new(file_layer.clone(), target_layer_str));
                            has_cross_layer = true;
                        }
                    }
                }
                let layer_mapping = if has_cross_layer {
                    Some((file_layer, file.clone()))
                } else {
                    None
                };
                Some((local_edges, layer_mapping))
            })
            .collect();

        let mut edges = Vec::new();
        let mut file_by_layer: HashMap<String, String> = HashMap::new();
        for (local_edges, layer_mapping) in file_results {
            edges.extend(local_edges);
            if let Some((fl, f)) = layer_mapping {
                file_by_layer.entry(fl).or_insert(f);
            }
        }

        let cycle_edge_results = utility_cycle_detector::detect_cycle_edges(&edges);
        cycle_edge_results.into_iter().map(|sn| {
            let edge_key = sn.value;
            let parts: Vec<&str> = edge_key.split("->").collect();
            let source = parts[0];
            let target = parts[1];
            let file = file_by_layer.get(source).cloned().unwrap_or_else(|| source.to_string());
            LintResult::new_arch(&file, 1, "AES205", Severity::CRITICAL,
                AesImportViolation::CircularImport {
                    reason: Some(LintMessage::new(format!(
                        "Circular dependency between layers '{}' and '{}' creates implicit bidirectional coupling.",
                        source, target
                    ))),
                }.to_string(),
            )
        }).collect()
    }
}
```

---

## File: crates/import-rules/src/capabilities_dummy_import_checker.rs

```rust
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::common::taxonomy_common_vo::LanguageVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_source_vo::ContentString;
use shared::common::utility_layer_detector;
use shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::utility_dummy_detector;
use shared::taxonomy_definition_vo::LayerMapVO;
use shared::taxonomy_layer_vo::{Identity, LayerNameVO};
use shared::taxonomy_message_vo::LintMessage;
use shared::taxonomy_name_vo::SymbolName;

// PURPOSE: DummyImportChecker — AES204: detect dummy imports, dummy functions, dummy trait impls
// Uses utility functions directly — no IImportParserProtocol, no IAnalyzer.

// ─── Block 1: Struct Definition ───────────────────────────

pub struct DummyImportChecker;

impl DummyImportChecker {}

// Pre-computed per-file data shared across all dummy checks.
struct DummyFileContext {
    lines: Vec<String>,
    lang: LanguageVO,
    layer_name: String,
    dummy_ranges: Vec<(
        shared::taxonomy_common_vo::LineNumber,
        shared::taxonomy_common_vo::LineNumber,
    )>,
    dummy_impl_traits: Vec<String>,
}

impl DummyFileContext {
    fn compute(file: &str, content: &str, layer_map: &LayerMapVO) -> Option<Self> {
        let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
        let str_refs: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
        let lang = LanguageVO::from_path(file);
        let layer_name = Self::detect_layer(file, layer_map);
        let dummy_ranges = utility_dummy_detector::dummy_function_ranges(&str_refs, lang);
        let dummy_impl_traits: Vec<String> =
            utility_dummy_detector::dummy_impl_traits_with_lines(&str_refs)
                .into_iter()
                .map(|(t, _)| t.value().to_string())
                .collect();
        Some(Self {
            lines,
            lang,
            layer_name,
            dummy_ranges,
            dummy_impl_traits,
        })
    }

    fn detect_layer(file: &str, layer_map: &LayerMapVO) -> String {
        let filename: &str = utility_layer_detector::extract_filename(file);
        match utility_layer_detector::detect_layer_from_prefix(filename) {
            Some(base) => {
                let layer_keys: Vec<String> =
                    layer_map.values.keys().map(|k| k.to_string()).collect();
                utility_layer_detector::resolve_specialized_layer(&base, file, &layer_keys)
            }
            None => "any".to_string(),
        }
    }

    fn str_refs(&self) -> Vec<&str> {
        self.lines.iter().map(|s| s.as_str()).collect()
    }
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IDummyImportCheckerProtocol for DummyImportChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES204")
    }

    fn check_dummy_imports(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        _root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) {
        if let Some(ctx) = DummyFileContext::compute(file.value(), content.value(), layer_map) {
            Self::_check_dummy_imports(file.value(), &ctx, violations, layer_map);
        }
    }

    fn check_dummy_functions(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        _root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) {
        if let Some(ctx) = DummyFileContext::compute(file.value(), content.value(), layer_map) {
            Self::_check_dummy_functions(file.value(), &ctx, violations);
        }
    }

    fn check_dummy_impls(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        _root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) {
        if let Some(ctx) = DummyFileContext::compute(file.value(), content.value(), layer_map) {
            Self::_check_dummy_impls(file.value(), &ctx, violations);
        }
    }

    fn check_taxonomy_intent(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        _root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) {
        if let Some(ctx) = DummyFileContext::compute(file.value(), content.value(), layer_map) {
            Self::_check_taxonomy_intent(file.value(), &ctx, violations);
        }
    }

    fn check_layer_contract_intent(
        &self,
        _file: &FilePath,
        _content: &ContentString,
        _violations: &mut Vec<LintResult>,
        _root_dir: &FilePath,
        _layer_map: &LayerMapVO,
    ) {
    }

    fn check_surface_logic(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        _root_dir: &FilePath,
        _layer_map: &LayerMapVO,
    ) {
        Self::_check_surface_logic(file.value(), content.value(), violations);
    }

    fn check_all_dummy(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        _root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) {
        if let Some(ctx) = DummyFileContext::compute(file.value(), content.value(), layer_map) {
            Self::_check_dummy_imports(file.value(), &ctx, violations, layer_map);
            Self::_check_dummy_functions(file.value(), &ctx, violations);
            Self::_check_dummy_impls(file.value(), &ctx, violations);
            Self::_check_taxonomy_intent(file.value(), &ctx, violations);
            Self::_check_surface_logic(file.value(), content.value(), violations);
        }
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Default for DummyImportChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl DummyImportChecker {
    pub fn new() -> Self {
        Self
    }

    fn _check_dummy_imports(
        file: &str,
        ctx: &DummyFileContext,
        violations: &mut Vec<LintResult>,
        _layer_map: &LayerMapVO,
    ) {
        let lines = ctx.str_refs();
        let imported = utility_dummy_detector::imported_symbols(&lines, ctx.lang);

        for (symbol, line_no) in imported {
            let symbol_str = symbol.value().to_string();
            if utility_dummy_detector::symbol_used_real(
                &lines,
                &symbol_str,
                &ctx.dummy_ranges,
                &ctx.dummy_impl_traits,
            ) {
                continue;
            }
            violations.push(LintResult::new_arch(file, line_no.value() as usize, "AES204", Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(ctx.layer_name.clone()),
                    import_type: SymbolName::new(symbol_str),
                    intent: SymbolName::new("Use imported symbols in real logic, not only in dummy functions or stubs"),
                    reason: Some(LintMessage::new(
                        "Imported symbols placed inside _use_ dummy functions are dead code — they exist only to suppress unused-import warnings."
                    )),
                }.to_string(),
            ));
        }
    }

    fn _check_dummy_functions(
        file: &str,
        ctx: &DummyFileContext,
        violations: &mut Vec<LintResult>,
    ) {
        for (start, end) in &ctx.dummy_ranges {
            violations.push(LintResult::new_arch(
                file,
                start.value() as usize,
                "AES204",
                Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(ctx.layer_name.clone()),
                    import_type: SymbolName::new("_use_mandatory_imports"),
                    intent: SymbolName::new(
                        "Remove dummy functions that exist only to silence unused import checks",
                    ),
                    reason: Some(LintMessage::new(format!(
                        "Dummy function range ends at line {}",
                        end
                    ))),
                }
                .to_string(),
            ));
        }
    }

    fn _check_dummy_impls(file: &str, ctx: &DummyFileContext, violations: &mut Vec<LintResult>) {
        let lines = ctx.str_refs();
        for (trait_name, start) in utility_dummy_detector::dummy_impl_traits_with_lines(&lines) {
            violations.push(LintResult::new_arch(
                file,
                start.value() as usize,
                "AES204",
                Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(ctx.layer_name.clone()),
                    import_type: SymbolName::new(trait_name.value().to_string()),
                    intent: SymbolName::new(
                        "Implement contract methods with real behavior instead of empty/todo stubs",
                    ),
                    reason: Some(LintMessage::new(
                        "Trait implementations with empty bodies violate the contract abstraction.",
                    )),
                }
                .to_string(),
            ));
        }
    }

    fn _check_taxonomy_intent(
        file: &str,
        ctx: &DummyFileContext,
        violations: &mut Vec<LintResult>,
    ) {
        let lines = ctx.str_refs();
        let imported = utility_dummy_detector::imported_symbols(&lines, ctx.lang);

        let mut has_dummy_function = false;
        let mut dummy_function_line = 0;
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            let is_dummy = match ctx.lang {
                LanguageVO::Rust => trimmed.starts_with("fn _use_") && trimmed.contains("()"),
                LanguageVO::Python => trimmed.starts_with("def _use_") && trimmed.contains("()"),
                LanguageVO::JavaScript => {
                    trimmed.starts_with("function _use") && trimmed.contains("()")
                }
                LanguageVO::Unknown => false,
            };
            if is_dummy {
                has_dummy_function = true;
                dummy_function_line = i + 1;
                break;
            }
        }
        if !has_dummy_function {
            return;
        }

        let has_real_usage = imported.iter().any(|(symbol, line_no)| {
            let is_taxonomy = lines
                .get(line_no.value().saturating_sub(1) as usize)
                .is_some_and(|line| {
                    let t = line.trim();
                    match ctx.lang {
                        LanguageVO::Rust => {
                            t.contains("use shared::taxonomy_")
                                || t.contains("use shared::common::taxonomy_")
                                || t.contains("use crate::common::taxonomy_")
                                || t.contains("use crate::taxonomy_")
                        }
                        LanguageVO::Python => {
                            t.contains("from taxonomy_") || t.contains("from shared.taxonomy_")
                        }
                        LanguageVO::JavaScript => {
                            t.contains("from 'taxonomy_") || t.contains("from \"taxonomy_")
                        }
                        LanguageVO::Unknown => false,
                    }
                });
            if !is_taxonomy {
                return false;
            }
            utility_dummy_detector::symbol_used_real(
                &lines,
                symbol.value(),
                &ctx.dummy_ranges,
                &ctx.dummy_impl_traits,
            )
        });

        if !has_real_usage {
            let has_taxonomy_import = lines.iter().any(|l| {
                let t = l.trim();
                match ctx.lang {
                    LanguageVO::Rust => {
                        t.contains("use shared::taxonomy_")
                            || t.contains("use shared::common::taxonomy_")
                            || t.contains("use crate::common::taxonomy_")
                            || t.contains("use crate::taxonomy_")
                    }
                    LanguageVO::Python => {
                        t.contains("import taxonomy_") || t.contains("from taxonomy_")
                    }
                    LanguageVO::JavaScript => {
                        t.contains("from 'taxonomy_") || t.contains("from \"taxonomy_")
                    }
                    LanguageVO::Unknown => false,
                }
            });
            if has_taxonomy_import {
                violations.push(LintResult::new_arch(file, dummy_function_line, "AES204", Severity::HIGH,
                    AesImportViolation::ImportIntentViolation {
                        source_layer: LayerNameVO::new("surfaces"),
                        import_type: SymbolName::new("taxonomy"),
                        intent: SymbolName::new("Use taxonomy Value Objects in function signatures instead of primitives"),
                        reason: Some(LintMessage::new(
                            "Taxonomy VOs encode domain concepts — using raw primitives defeats the purpose."
                        )),
                    }.to_string(),
                ));
            }
        }
    }

    fn _check_surface_logic(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = LanguageVO::from_path(file);
        let mk = |c: &[char]| c.iter().collect::<String>();
        let logic_patterns = [
            mk(&['l', 'i', 'n', 't', '_', 'p', 'a', 't', 'h', '(']),
            mk(&[
                'c', 'o', 'm', 'p', 'u', 't', 'e', '_', 's', 'c', 'o', 'r', 'e', '(',
            ]),
            mk(&[
                'h', 'a', 's', '_', 'c', 'r', 'i', 't', 'i', 'c', 'a', 'l', '(',
            ]),
            mk(&[
                'w', 'a', 'l', 'k', '_', 'r', 's', '_', 'f', 'i', 'l', 'e', 's', '(',
            ]),
        ];

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            let is_skip = match lang {
                LanguageVO::Rust => trimmed.starts_with("//") || trimmed.starts_with("fn _use_"),
                LanguageVO::Python => trimmed.starts_with("#") || trimmed.starts_with("def _use_"),
                LanguageVO::JavaScript => {
                    trimmed.starts_with("//") || trimmed.starts_with("function _use")
                }
                LanguageVO::Unknown => false,
            };
            if is_skip {
                continue;
            }
            for pattern in &logic_patterns {
                if trimmed.contains(pattern) {
                    violations.push(LintResult::new_arch(file, i + 1, "AES204", Severity::MEDIUM,
                        AesImportViolation::ImportIntentViolation {
                            source_layer: LayerNameVO::new("surfaces"),
                            import_type: SymbolName::new(pattern.to_string()),
                            intent: SymbolName::new(format!("Delegate to aggregate instead of calling '{}' directly", pattern)),
                            reason: Some(LintMessage::new(
                                "Surface-layer code must delegate business logic to the aggregate layer."
                            )),
                        }.to_string(),
                    ));
                }
            }
        }
    }
}
```

---

## File: crates/import-rules/src/capabilities_import_forbidden_checker.rs

```rust
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_layer_detector;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::utility_import_resolver;
use shared::import_rules::utility_path_normalizer;
use shared::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::taxonomy_layer_vo::{Identity, LayerNameVO};
use std::collections::HashSet;

// PURPOSE: ArchImportForbiddenChecker — AES201: enforce forbidden import rules
// Uses utility functions directly — no IImportParserProtocol, no IAnalyzer.

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ArchImportForbiddenChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl IImportForbiddenProtocol for ArchImportForbiddenChecker {
    fn rule_name(&self) -> Identity {
        let _ = utility_path_normalizer::extract_layer_from_prefix("");
        Identity::new("AES201")
    }

    async fn check_forbidden_imports(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        _root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();

        let aes201_exceptions: HashSet<String> = config
            .rules
            .iter()
            .filter(|r| r.name.value == "AES201")
            .flat_map(|r| r.exceptions.values.iter().cloned())
            .collect();

        let file_violations: Vec<LintResult> = files
            .values
            .iter()
            .flat_map(|f| {
                let f_str = f.to_string();
                let basename = f.basename();
                if aes201_exceptions.contains(&basename) {
                    return Vec::new();
                }

                let content =
                    match shared::common::utility_file_handler::read_file_generic(&f_str).ok() {
                        Some(c) => c,
                        None => return Vec::new(),
                    };
                let import_lines = utility_import_resolver::parse_import_lines_helper(&content);
                if import_lines.is_empty() {
                    return Vec::new();
                }

                let mut local_violations = Vec::new();
                let filename = utility_layer_detector::extract_filename(&f_str);
                if let Some(base_layer) = utility_layer_detector::detect_layer_from_prefix(filename)
                {
                    let specialized = utility_layer_detector::resolve_specialized_layer(
                        &base_layer,
                        &f_str,
                        &layer_keys,
                    );
                    let layer_name = LayerNameVO::new(specialized.as_str());
                    if let Some(def) = layer_map.values.get(&layer_name) {
                        self._check_forbidden_imports_with_lines(
                            &f_str,
                            &specialized,
                            def,
                            &import_lines,
                            &mut local_violations,
                        );
                    }
                }
                self._check_scope_forbidden_imports_with_lines(
                    &f_str,
                    &basename,
                    config,
                    &import_lines,
                    &mut local_violations,
                );
                local_violations
            })
            .collect();

        results.values.extend(file_violations);
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for ArchImportForbiddenChecker {
    fn default() -> Self {
        Self
    }
}

impl ArchImportForbiddenChecker {
    pub fn new() -> Self {
        Self
    }

    fn _check_forbidden_imports_with_lines(
        &self,
        file: &str,
        layer_name: &str,
        definition: &LayerDefinition,
        import_lines: &[(
            shared::taxonomy_common_vo::LineNumber,
            shared::taxonomy_layer_vo::LineContentVO,
        )],
        violations: &mut Vec<LintResult>,
    ) {
        let file_path = match FilePath::new(file.to_string()) {
            Ok(p) => p,
            Err(_) => return,
        };
        let basename = file_path.basename();
        if definition.exceptions.values.contains(&basename.to_string()) {
            return;
        }

        let is_surfaces = layer_name == "surfaces" || layer_name.starts_with("surfaces(");
        if definition.forbidden.values.is_empty() && !is_surfaces {
            return;
        }
        let forbidden_list: Vec<String> = if !definition.forbidden.values.is_empty() {
            definition.forbidden.values.clone()
        } else {
            vec!["agent".into(), "capabilities".into()]
        };

        let layer_name_vo = LayerNameVO::new(layer_name);

        for (line_num, line) in import_lines {
            if let Some(module) = utility_import_resolver::extract_module_from_line(line) {
                let module_val = module.value();
                for forbidden in &forbidden_list {
                    let forbidden_identity = Identity::new(forbidden);
                    let (layer, suffixes) =
                        utility_import_resolver::resolve_scope(&forbidden_identity);
                    let is_forbidden = if suffixes.is_empty() {
                        module_val
                            .split([':', '.', '/', '\\'])
                            .filter(|s| !s.is_empty())
                            .any(|seg| {
                                let cleaned = Identity::new(seg.trim_end_matches(';').trim());
                                match utility_import_resolver::extract_layer_from_import(&cleaned) {
                                    Some(l) => l == layer,
                                    None => false,
                                }
                            })
                    } else {
                        utility_import_resolver::import_matches_scope(line, &layer, &suffixes)
                    };
                    if is_forbidden {
                        let allowed: Vec<LayerNameVO> = definition
                            .allowed
                            .values
                            .iter()
                            .map(|s| {
                                LayerNameVO::new(
                                    utility_import_resolver::resolve_scope(&Identity::new(s))
                                        .0
                                        .value()
                                        .to_string(),
                                )
                            })
                            .collect();
                        violations.push(LintResult::new_arch(
                            file,
                            line_num.value() as usize,
                            "AES201",
                            Severity::CRITICAL,
                            AesImportViolation::ForbiddenImport {
                                source_layer: layer_name_vo.clone(),
                                forbidden_layer: LayerNameVO::new(forbidden.clone()),
                                allowed,
                                reason: None,
                            }
                            .to_string(),
                        ));
                    }
                }
            }
        }
    }

    fn _check_scope_forbidden_imports_with_lines(
        &self,
        file: &str,
        basename: &str,
        config: &ArchitectureConfig,
        import_lines: &[(
            shared::taxonomy_common_vo::LineNumber,
            shared::taxonomy_layer_vo::LineContentVO,
        )],
        violations: &mut Vec<LintResult>,
    ) {
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }

        for rule in &config.rules {
            if rule.exceptions.values.contains(&basename.to_string()) {
                continue;
            }
            let Some((rule_layer_str, _rule_suffixes)) =
                shared::common::utility_scope_matcher::file_belongs_to_scope(
                    basename,
                    &Identity::new(&rule.scope.value),
                )
            else {
                continue;
            };

            for (line_num, line) in import_lines {
                if let Some(module) = utility_import_resolver::extract_module_from_line(line) {
                    let module_val = module.value();
                    for forbidden in &rule.forbidden.values {
                        let forbidden_identity = Identity::new(forbidden);
                        let (forbidden_layer, forbidden_suffixes) =
                            utility_import_resolver::resolve_scope(&forbidden_identity);
                        let is_forbidden = if forbidden_suffixes.is_empty() {
                            module_val
                                .split([':', '.', '/', '\\'])
                                .filter(|s| !s.is_empty())
                                .any(|seg| {
                                    let cleaned = Identity::new(seg.trim_end_matches(';').trim());
                                    match utility_import_resolver::extract_layer_from_import(
                                        &cleaned,
                                    ) {
                                        Some(l) => l == forbidden_layer,
                                        None => false,
                                    }
                                })
                        } else {
                            utility_import_resolver::import_matches_scope(
                                line,
                                &forbidden_layer,
                                &forbidden_suffixes,
                            )
                        };
                        if is_forbidden {
                            let allowed: Vec<LayerNameVO> = rule
                                .allowed
                                .values
                                .iter()
                                .map(|s| {
                                    LayerNameVO::new(
                                        utility_import_resolver::resolve_scope(&Identity::new(s))
                                            .0
                                            .value()
                                            .to_string(),
                                    )
                                })
                                .collect();
                            violations.push(LintResult::new_arch(
                                file,
                                line_num.value() as usize,
                                "AES201",
                                Severity::CRITICAL,
                                AesImportViolation::ForbiddenImport {
                                    source_layer: LayerNameVO::new(rule_layer_str.clone()),
                                    forbidden_layer: LayerNameVO::new(forbidden.clone()),
                                    allowed,
                                    reason: None,
                                }
                                .to_string(),
                            ));
                        }
                    }
                }
            }
        }
    }
}
```

---

## File: crates/import-rules/src/capabilities_import_mandatory_checker.rs

```rust
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_layer_detector;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::utility_import_resolver;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO, LineContentVO};
use shared::taxonomy_name_vo::SymbolName;
use std::collections::HashSet;

// PURPOSE: ArchImportMandatoryChecker — AES202: enforce mandatory import rules
// Uses utility functions directly — no IImportParserProtocol, no IAnalyzer.

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ArchImportMandatoryChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl IImportMandatoryProtocol for ArchImportMandatoryChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES202")
    }

    async fn run_mandatory_imports(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        _root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();

        let aes202_exceptions: HashSet<String> = config
            .rules
            .iter()
            .filter(|r| r.name.value == "AES202")
            .flat_map(|r| r.exceptions.values.iter().cloned())
            .collect();

        let file_violations: Vec<LintResult> = files
            .values
            .iter()
            .flat_map(|f| {
                let f_str = f.to_string();
                let basename = f.basename();
                if aes202_exceptions.contains(&basename) {
                    return Vec::new();
                }

                let content =
                    match shared::common::utility_file_handler::read_file_generic(&f_str).ok() {
                        Some(c) => c,
                        None => return Vec::new(),
                    };
                let file_content = FileContentVO::new(content);
                let import_lines: Vec<(LineNumber, LineContentVO)> =
                    utility_import_resolver::parse_import_lines_helper(file_content.value());

                let mut local_violations = Vec::new();
                let filename = utility_layer_detector::extract_filename(&f_str);
                if let Some(base_layer) = utility_layer_detector::detect_layer_from_prefix(filename)
                {
                    let specialized = utility_layer_detector::resolve_specialized_layer(
                        &base_layer,
                        &f_str,
                        &layer_keys,
                    );
                    let layer_name = LayerNameVO::new(specialized.as_str());
                    if let Some(def) = layer_map.values.get(&layer_name) {
                        self._check_mandatory_imports_with_lines(
                            &f_str,
                            &basename,
                            def,
                            &import_lines,
                            &mut local_violations,
                        );
                    }
                }
                self._check_scope_mandatory_imports_with_lines(
                    &f_str,
                    &basename,
                    config,
                    &import_lines,
                    &mut local_violations,
                );
                local_violations
            })
            .collect();

        results.values.extend(file_violations);
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for ArchImportMandatoryChecker {
    fn default() -> Self {
        Self
    }
}

impl ArchImportMandatoryChecker {
    pub fn new() -> Self {
        Self
    }

    fn _check_mandatory_imports_with_lines(
        &self,
        file: &str,
        basename: &str,
        definition: &LayerDefinition,
        import_lines: &[(LineNumber, LineContentVO)],
        violations: &mut Vec<LintResult>,
    ) {
        if definition.mandatory.values.is_empty() || basename == "__init__.py" {
            return;
        }
        if definition.exceptions.values.contains(&basename.to_string()) {
            return;
        }

        let stem: &str = basename.rsplit('.').next_back().map_or(basename, |s| s);
        let source_layer: &str = stem.split('_').next().map_or("unknown", |s| s);

        for required in &definition.mandatory.values {
            let required_identity = Identity::new(required);
            let (layer, suffixes) = utility_import_resolver::resolve_scope(&required_identity);
            let layer_str: &str = layer.value();
            let is_present: bool = if suffixes.is_empty() {
                import_lines
                    .iter()
                    .any(|(_, l)| l.value().contains(layer_str))
            } else {
                import_lines.iter().any(|(_, l)| {
                    utility_import_resolver::import_matches_scope(l, &layer, &suffixes)
                })
            };
            if !is_present {
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES202",
                    Severity::HIGH,
                    AesImportViolation::MissingImport {
                        source_layer: LayerNameVO::new(source_layer.to_string()),
                        required: SymbolName::new(required.clone()),
                        reason: None,
                    }
                    .to_string(),
                ));
            }
        }
    }

    fn _check_scope_mandatory_imports_with_lines(
        &self,
        file: &str,
        basename: &str,
        config: &ArchitectureConfig,
        import_lines: &[(LineNumber, LineContentVO)],
        violations: &mut Vec<LintResult>,
    ) {
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }

        for rule in &config.rules {
            if rule.mandatory.values.is_empty() {
                continue;
            }
            let scope_identity = Identity::new(&rule.scope.value);
            let Some((rule_layer_str, _rule_suffixes)) =
                shared::common::utility_scope_matcher::file_belongs_to_scope(
                    basename,
                    &scope_identity,
                )
            else {
                continue;
            };

            for required in &rule.mandatory.values {
                let required_identity = Identity::new(required);
                let (req_layer, req_suffixes) =
                    utility_import_resolver::resolve_scope(&required_identity);
                let req_layer_str = req_layer.value();
                let is_present = if req_suffixes.is_empty() {
                    import_lines
                        .iter()
                        .any(|(_, l)| l.value().contains(req_layer_str))
                } else {
                    import_lines.iter().any(|(_, l)| {
                        utility_import_resolver::import_matches_scope(l, &req_layer, &req_suffixes)
                    })
                };
                if !is_present {
                    violations.push(LintResult::new_arch(
                        file,
                        0,
                        "AES202",
                        Severity::HIGH,
                        AesImportViolation::MissingImport {
                            source_layer: LayerNameVO::new(rule_layer_str.clone()),
                            required: SymbolName::new(required.clone()),
                            reason: None,
                        }
                        .to_string(),
                    ));
                }
            }
        }
    }
}
```

---

## File: crates/import-rules/src/capabilities_import_unused_checker.rs

```rust
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::{utility_import_resolver, utility_import_symbol_extractor};

// PURPOSE: UnusedImportRuleChecker — AES203: detect unused imports (Rust/Python/JS)
// Uses utility functions directly — no IImportParserProtocol.
use shared::cli_commands::taxonomy_result_vo::LintResult;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct UnusedImportRuleChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IUnusedImportProtocol for UnusedImportRuleChecker {
    fn find_unused_imports(&self, path: &FilePath) -> Vec<LintMessage> {
        let Some(content) =
            shared::common::utility_file_handler::read_file_generic(path.value()).ok()
        else {
            return vec![];
        };
        let imported_aliases = utility_import_symbol_extractor::extract_imported_aliases(&content);
        let exported_symbols = utility_import_symbol_extractor::extract_exported_symbols(&content);
        let used_symbols =
            utility_import_symbol_extractor::extract_used_symbols(&content, &imported_aliases);

        let mut unused: Vec<String> = Vec::new();
        for alias in imported_aliases.keys() {
            if !used_symbols.contains(alias) && !exported_symbols.contains(alias) {
                unused.push(alias.value().to_string());
            }
        }
        let rust_js_imports = utility_import_symbol_extractor::extract_rust_js_imports(&content);
        for (name, line_idx) in rust_js_imports {
            let name_str = name.value();
            if !utility_import_symbol_extractor::is_name_used(
                name_str,
                &content,
                line_idx.value() as usize,
            ) {
                unused.push(name_str.to_string());
            }
        }
        unused.into_iter().map(LintMessage::new).collect()
    }

    fn check_unused_imports(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let imported_aliases = utility_import_symbol_extractor::extract_imported_aliases(content);
        let exported_symbols = utility_import_symbol_extractor::extract_exported_symbols(content);
        let used_symbols =
            utility_import_symbol_extractor::extract_used_symbols(content, &imported_aliases);

        for alias in imported_aliases.keys() {
            if !used_symbols.contains(alias) && !exported_symbols.contains(alias) {
                let line_num = utility_import_resolver::find_import_line_number(
                    content,
                    alias.value(),
                )
                .value() as usize;
                violations.push(LintResult::new_arch(
                    file,
                    line_num,
                    "AES203",
                    Severity::MEDIUM,
                    AesImportViolation::FixUnusedImport {
                        reason: Some(LintMessage::new(format!(
                            "Import '{}' is declared but never used in this file.",
                            alias
                        ))),
                    }
                    .to_string(),
                ));
            }
        }
        let rust_js_imports = utility_import_symbol_extractor::extract_rust_js_imports(content);
        for (name, line_idx) in rust_js_imports {
            let name_str = name.value().to_string();
            if !utility_import_symbol_extractor::is_name_used(
                &name_str,
                content,
                line_idx.value() as usize,
            ) {
                violations.push(LintResult::new_arch(
                    file,
                    line_idx.value() as usize,
                    "AES203",
                    Severity::MEDIUM,
                    AesImportViolation::FixUnusedImport {
                        reason: Some(LintMessage::new(format!(
                            "Import '{}' is declared but never used in this file.",
                            name_str
                        ))),
                    }
                    .to_string(),
                ));
            }
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for UnusedImportRuleChecker {
    fn default() -> Self {
        Self
    }
}

impl UnusedImportRuleChecker {
    pub fn new() -> Self {
        Self
    }
}
```

---

## File: crates/import-rules/src/lib.rs

```rust
// PURPOSE: Module declarations for import-rules (5 capabilities + 5 protocols)
pub mod agent_import_orchestrator;
pub mod capabilities_cycle_import_analyzer;
pub mod capabilities_dummy_import_checker;
pub mod capabilities_import_forbidden_checker;
pub mod capabilities_import_mandatory_checker;
pub mod capabilities_import_unused_checker;
pub mod root_import_rules_container;
```

---

## File: crates/import-rules/src/root_import_rules_container.rs

```rust
// PURPOSE: ImportContainer — wiring for import-rules feature (root layer, wiring only)
use crate::agent_import_orchestrator::{ImportOrchestrator, ImportOrchestratorDeps};
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use std::sync::Arc;

pub struct ImportContainer {
    config: ArchitectureConfig,
}

impl ImportContainer {
    pub fn new_with_config(config: ArchitectureConfig) -> Self {
        Self { config }
    }

    /// Create from config orchestrator — the canonical way per AES architecture.
    pub fn from_orchestrator(
        orchestrator: &Arc<dyn IConfigOrchestratorAggregate>,
        project_root: &str,
    ) -> Self {
        let fp = FilePath::new(project_root.to_string()).unwrap_or_default();
        let config = orchestrator.load_config_sync(&fp);
        Self::new_with_config(config)
    }

    pub fn orchestrator(&self) -> Arc<dyn IImportRunnerAggregate> {
        let ignored_paths: Vec<String> = self
            .config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.clone())
            .collect();
        Arc::new(ImportOrchestrator::new(
            ImportOrchestratorDeps {
                mandatory: Arc::new(
                    crate::capabilities_import_mandatory_checker::ArchImportMandatoryChecker::new(),
                ),
                forbidden: Arc::new(
                    crate::capabilities_import_forbidden_checker::ArchImportForbiddenChecker::new(),
                ),
                unused: Arc::new(
                    crate::capabilities_import_unused_checker::UnusedImportRuleChecker::new(),
                ),
                cycle: Arc::new(
                    crate::capabilities_cycle_import_analyzer::DependencyCycleAnalyzer::new(),
                ),
                dummy: Arc::new(
                    crate::capabilities_dummy_import_checker::DummyImportChecker::new(),
                ),
            },
            self.config.clone(),
            ignored_paths,
        ))
    }
}
```

---

## File: crates/shared/src/cli-commands/mod.rs

```rust
// cli-commands — taxonomy and contract types
pub mod taxonomy_cli_vo;
pub mod taxonomy_command_catalog_vo;
pub mod taxonomy_format_vo;
pub mod taxonomy_position_vo;
pub mod taxonomy_protocol_vo;
pub mod taxonomy_result_vo;
pub mod taxonomy_scan_report_vo;
pub mod taxonomy_scan_request_vo;
pub mod utility_path_resolver;
```

---

## File: crates/shared/src/cli-commands/taxonomy_result_vo.rs

```rust
// PURPOSE: LintResult, LintResultList, FilePathSet — value objects for lint violation results
use serde::{Deserialize, Serialize};

use crate::cli_commands::taxonomy_position_vo::Position;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_lint_vo::LocationList;
use crate::common::taxonomy_lint_vo::ScopeRef;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LintResult {
    pub file: FilePath,
    pub line: LineNumber,
    pub column: ColumnNumber,
    pub code: ErrorCode,
    pub message: LintMessage,
    pub source: Option<AdapterName>,
    pub severity: Severity,
    pub enclosing_scope: Option<ScopeRef>,
    pub related_locations: LocationList,
}

impl LintResult {
    /// Convenience constructor used by architecture checkers (make_result / mk pattern).
    pub fn new_arch(
        file: &str,
        line: usize,
        code: &str,
        sev: Severity,
        msg: impl Into<String>,
    ) -> Self {
        Self::new_arch_with_column(file, line, 0, code, sev, msg)
    }

    /// Column-aware constructor for architecture checkers.
    pub fn new_arch_with_column(
        file: &str,
        line: usize,
        column: usize,
        code: &str,
        sev: Severity,
        msg: impl Into<String>,
    ) -> Self {
        Self {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(line as i64),
            column: ColumnNumber::new(column as i64),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
            enclosing_scope: Some(ScopeRef {
                name: DescriptionVO::new(String::new()),
                kind: DescriptionVO::new(String::new()),
                file: None,
                start_line: None,
                end_line: None,
            }),
            related_locations: LocationList::new(),
        }
    }

    /// Specialized constructor for orphan detection results (no enclosing scope).
    pub fn new_orphan(file: &str, msg: impl Into<String>, sev: Severity, code: &str) -> Self {
        Self {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(0),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
            enclosing_scope: None,
            related_locations: LocationList::new(),
        }
    }

    pub fn position(&self) -> Position {
        Position {
            line: self.line.clone(),
            column: self.column.clone(),
        }
    }
    pub fn identity(&self) -> Identity {
        Identity::new(format!(
            "{}:{}:{}:{:?}",
            self.file, self.line, self.code, self.source
        ))
    }
}

/// Generate a `Vec<T>`-backed newtype with `Default`, `new`, `iter`,
/// `len`, `is_empty`, `push`, and `append`. Used for the `LintResultList`
/// wrapper below; siblings `ImportInfoList`/`PrimitiveViolationList` in
/// `taxonomy_import_source_vo.rs` carry the same surface.
macro_rules! lint_result_list_wrapper {
    ($name:ident, $item:ty) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
        pub struct $name {
            pub values: Vec<$item>,
        }

        impl $name {
            pub fn new(value: Vec<$item>) -> Self {
                Self { values: value }
            }
            pub fn iter(&self) -> std::slice::Iter<'_, $item> {
                self.values.iter()
            }
            pub fn len(&self) -> usize {
                self.values.len()
            }
            pub fn is_empty(&self) -> bool {
                self.values.is_empty()
            }
            pub fn push(&mut self, item: $item) {
                self.values.push(item);
            }
            pub fn append(&mut self, item: $item) {
                self.values.push(item);
            }
        }
    };
}

lint_result_list_wrapper!(LintResultList, LintResult);
```

---

## File: crates/shared/src/common/mod.rs

```rust
// common — truly shared types used by multiple features
pub mod contract_executor_protocol;
pub mod taxonomy_action_vo;
pub mod taxonomy_adapter_error;
pub mod taxonomy_adapter_list_vo;
pub mod taxonomy_adapter_name_vo;
pub mod taxonomy_common_error;
pub mod taxonomy_common_vo;
pub mod taxonomy_definition_vo;
pub mod taxonomy_display_content_vo;
pub mod taxonomy_duration_vo;
pub mod taxonomy_error_vo;
pub mod taxonomy_filesystem_error;
pub mod taxonomy_git_vo;
pub mod taxonomy_job_id_vo;
pub mod taxonomy_job_vo;
pub mod taxonomy_language_info_vo;
pub mod taxonomy_language_vo;
pub mod taxonomy_layer_vo;
pub mod taxonomy_line_count_vo;
pub mod taxonomy_lint_vo;
pub mod taxonomy_message_vo;
pub mod taxonomy_name_vo;
pub mod taxonomy_path_utils_vo;
pub mod taxonomy_path_vo;
pub mod taxonomy_paths_vo;
pub mod taxonomy_response_data_vo;
pub mod taxonomy_severity_vo;
pub mod taxonomy_source_vo;
pub mod taxonomy_suffix_vo;
pub mod taxonomy_suggestion_vo;
pub mod taxonomy_threshold_vo;
pub mod utility_command_runner;
pub mod utility_compliance_score;
pub mod utility_file_handler;
pub mod utility_language_detector;
pub mod utility_layer_detector;
pub mod utility_path_normalization;
pub mod utility_scope_matcher;
pub mod utility_signature_parser;
pub mod utility_value_object_generator;
```

---

## File: crates/shared/src/common/taxonomy_adapter_error.rs

```rust
// PURPOSE: AdapterError, ScanError, ValidationError — structured error types for adapter operations
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::Constraint;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_error::ExitCode;
use crate::common::taxonomy_common_error::FieldName;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, thiserror::Error)]
pub struct AdapterError {
    pub adapter_name: AdapterName,
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: Option<ErrorCode>,
    #[serde(default)]
    pub command: Option<ContentString>,
    #[serde(default)]
    pub stderr: Option<ErrorMessage>,
    #[serde(default)]
    pub exit_code: Option<ExitCode>,
}

impl AdapterError {
    pub fn new(adapter_name: AdapterName, message: ErrorMessage) -> Self {
        Self {
            adapter_name,
            message,
            error_code: None,
            command: None,
            stderr: None,
            exit_code: None,
        }
    }
}

impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self.error_code.as_ref() {
            Some(c) => format!(" [{}]", c),
            None => String::new(),
        };
        write!(f, "[{}]{} {}", self.adapter_name, code, self.message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct ScanError {
    pub path: FilePath,
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: Option<ErrorCode>,
    #[serde(default)]
    pub adapter_name: Option<AdapterName>,
    #[serde(default)]
    pub cause: Option<Cause>,
}

impl ScanError {
    pub fn new(path: FilePath, message: ErrorMessage) -> Self {
        Self {
            path,
            message,
            error_code: None,
            adapter_name: None,
            cause: None,
        }
    }
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let adapter = match self.adapter_name.as_ref() {
            Some(a) => format!(" ({})", a),
            None => String::new(),
        };
        let code = match self.error_code.as_ref() {
            Some(c) => format!(" [{}]", c),
            None => String::new(),
        };
        write!(
            f,
            "Scan failed{}{}: {} — {}",
            adapter, code, self.path, self.message
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct ValidationError {
    pub field_name: FieldName,
    pub message: ErrorMessage,
    #[serde(default)]
    pub constraint: Option<Constraint>,
    #[serde(default)]
    pub value: Option<String>,
}

impl ValidationError {
    pub fn new(field_name: FieldName, message: ErrorMessage) -> Self {
        Self {
            field_name,
            message,
            constraint: None,
            value: None,
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Validation failed on '{}': {}",
            self.field_name, self.message
        )
    }
}
```

---

## File: crates/shared/src/common/taxonomy_common_error.rs

```rust
// PURPOSE: Cause, Constraint, ExitCode, FieldName, ModuleName, PrimitiveTypeName — common error value objects
pub use crate::common::taxonomy_common_vo::ErrorMessage;
use crate::string_value_object;
use serde::Serialize;

string_value_object!(Cause);
string_value_object!(Constraint);
string_value_object!(FieldName);
string_value_object!(ModuleName);
string_value_object!(PrimitiveTypeName);

/// Strongly-typed exit code value object. Written manually because the
/// `string_value_object!` macro only supports `String` (not `i64`).
///
/// Workspace Exit Code Contract (root PRD):
///   0 = Ok / clean / diagnostic completed
///   1 = Policy fail (violations, CI fail, vulns found, remaining after fix)
///   2 = Runtime error (bad path, pipeline crash, invalid state)
///   3 = Prerequisite missing (required external tool not installed)
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ExitCode {
    pub value: crate::common::taxonomy_common_vo::LineNumber,
}

impl ExitCode {
    pub fn new(value: impl Into<crate::common::taxonomy_common_vo::LineNumber>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> i64 {
        self.value.value()
    }

    // ── Named constants (workspace exit-code contract) ──────────────
    /// Exit 0 — Ok / clean / diagnostic completed.
    pub const OK: Self = Self {
        value: crate::common::taxonomy_common_vo::LineNumber { value: 0 },
    };
    /// Exit 1 — Policy fail (violations, CI fail, vulns found, remaining after fix).
    pub const POLICY_FAIL: Self = Self {
        value: crate::common::taxonomy_common_vo::LineNumber { value: 1 },
    };
    /// Exit 2 — Runtime error (bad path, pipeline crash, invalid state).
    pub const RUNTIME_ERROR: Self = Self {
        value: crate::common::taxonomy_common_vo::LineNumber { value: 2 },
    };
    /// Exit 3 — Prerequisite missing (required external tool not installed).
    pub const PREREQUISITE_MISSING: Self = Self {
        value: crate::common::taxonomy_common_vo::LineNumber { value: 3 },
    };

    /// Convert to `std::process::ExitCode` for CLI surface return values.
    pub fn to_process_exit_code(&self) -> std::process::ExitCode {
        std::process::ExitCode::from(self.value.value() as u8)
    }

    /// Compare against a `std::process::ExitCode` (e.g. from CLI handlers).
    /// Uses the underlying numeric value for comparison.
    pub fn matches_std(&self, other: &std::process::ExitCode) -> bool {
        self.to_process_exit_code() == *other
    }
}

impl std::fmt::Display for ExitCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for ExitCode {
    fn from(v: i64) -> Self {
        Self {
            value: crate::common::taxonomy_common_vo::LineNumber::new(v),
        }
    }
}

impl<'de> serde::Deserialize<'de> for ExitCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct W {
            value: crate::common::taxonomy_common_vo::LineNumber,
        }
        let w = W::deserialize(deserializer)?;
        Ok(Self { value: w.value })
    }
}
```

---

## File: crates/shared/src/common/taxonomy_common_vo.rs

```rust
// PURPOSE: BooleanVO, ColumnNumber, Count, DataFlowList, LineContentList, LineNumber, PatternList, Score, Timestamp — common VOs
use crate::common::taxonomy_job_id_vo::JobId;
use crate::common::taxonomy_layer_vo::LineContentVO;
use crate::common::taxonomy_response_data_vo::ResponseData;
use crate::common::taxonomy_severity_vo::Severity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LanguageVO {
    Rust,
    Python,
    JavaScript,
    Unknown,
}

impl LanguageVO {
    pub fn from_path(path: &str) -> Self {
        let ext = path.rsplit('.').next().unwrap_or("");
        match ext {
            "rs" => LanguageVO::Rust,
            "py" => LanguageVO::Python,
            "js" | "ts" | "jsx" | "tsx" => LanguageVO::JavaScript,
            _ => LanguageVO::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct BooleanVO {
    pub value: bool,
}

impl BooleanVO {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}

impl std::fmt::Display for BooleanVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<bool> for BooleanVO {
    fn from(v: bool) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for BooleanVO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BooleanVOVisitor {}
        impl<'de> serde::de::Visitor<'de> for BooleanVOVisitor {
            type Value = BooleanVO;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BooleanVO { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<bool>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(BooleanVO { value: val })
            }
        }
        deserializer.deserialize_any(BooleanVOVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ColumnNumber {
    pub value: i64,
}

impl ColumnNumber {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for ColumnNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for ColumnNumber {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for ColumnNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ColumnNumberVisitor {}
        impl<'de> serde::de::Visitor<'de> for ColumnNumberVisitor {
            type Value = ColumnNumber;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ColumnNumber { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ColumnNumber { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(ColumnNumber { value: val })
            }
        }
        deserializer.deserialize_any(ColumnNumberVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct Count {
    pub value: i64,
}

impl Count {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for Count {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for Count {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for Count {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CountVisitor {}
        impl<'de> serde::de::Visitor<'de> for CountVisitor {
            type Value = Count;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Count { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Count { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Count { value: val })
            }
        }
        deserializer.deserialize_any(CountVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataFlowList {
    pub values: Vec<ErrorMessage>,
}

impl DataFlowList {
    pub fn new(value: Vec<ErrorMessage>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[ErrorMessage] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, ErrorMessage> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: ErrorMessage) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobIdList {
    pub values: Vec<JobId>,
}

impl JobIdList {
    pub fn new(value: Vec<JobId>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[JobId] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, JobId> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: JobId) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LineContentList {
    pub values: Vec<LineContentVO>,
}

impl LineContentList {
    pub fn new(value: Vec<LineContentVO>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[LineContentVO] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, LineContentVO> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: LineContentVO) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
#[derive(Default)]
pub struct LineNumber {
    pub value: i64,
}

impl LineNumber {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for LineNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for LineNumber {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for LineNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct LineNumberVisitor {}
        impl<'de> serde::de::Visitor<'de> for LineNumberVisitor {
            type Value = LineNumber;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LineNumber { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LineNumber { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(LineNumber { value: val })
            }
        }
        deserializer.deserialize_any(LineNumberVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct PatternList {
    pub values: Vec<String>,
}

impl PatternList {
    pub fn new(value: impl IntoPatternListValues) -> Self {
        Self {
            values: value.into_pattern_list_values(),
        }
    }
    pub fn values(&self) -> &[String] {
        &self.values
    }
}

impl PatternList {
    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: String) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseDataList {
    pub values: Vec<ResponseData>,
}

impl ResponseDataList {
    pub fn new(value: Vec<ResponseData>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[ResponseData] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, ResponseData> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: ResponseData) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(transparent)]
pub struct Score {
    pub value: f64,
}

impl Score {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> f64 {
        self.value
    }
    pub fn is_perfect(&self) -> bool {
        self.value >= 100.0
    }
    pub fn is_passing(&self, threshold: &Score) -> bool {
        self.value >= threshold.value
    }
    pub fn deduct(&self, severity: &Severity) -> Score {
        Score {
            value: self.value - severity.score_impact(),
        }
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}", self.value)
    }
}

impl From<f64> for Score {
    fn from(v: f64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for Score {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ScoreVisitor {}
        impl<'de> serde::de::Visitor<'de> for ScoreVisitor {
            type Value = Score;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Score { value: v })
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Score { value: v as f64 })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Score { value: v as f64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<f64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Score { value: val })
            }
        }
        deserializer.deserialize_any(ScoreVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct Timestamp {
    pub value: String,
}

impl Timestamp {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn now() -> Self {
        Self {
            value: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for Timestamp {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for Timestamp {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TimestampVisitor {}
        impl<'de> serde::de::Visitor<'de> for TimestampVisitor {
            type Value = Timestamp;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Timestamp {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Timestamp { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<String>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Timestamp { value: val })
            }
        }
        deserializer.deserialize_any(TimestampVisitor {})
    }
}

// Custom Coercion Traits for PatternList

pub trait IntoPatternListValues {
    fn into_pattern_list_values(self) -> Vec<String>;
}

impl IntoPatternListValues for &str {
    fn into_pattern_list_values(self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl IntoPatternListValues for String {
    fn into_pattern_list_values(self) -> Vec<String> {
        vec![self]
    }
}

impl IntoPatternListValues for Vec<String> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self
    }
}

impl IntoPatternListValues for Vec<&str> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self.into_iter().map(|s| s.to_string()).collect()
    }
}

impl IntoPatternListValues for &Vec<String> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ErrorMessage {
    pub value: String,
}

impl ErrorMessage {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for ErrorMessage {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for ErrorMessage {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(transparent)]
pub struct SuffixPolicyVO {
    pub value: String,
}

impl SuffixPolicyVO {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SuffixVO {
    pub values: PatternList,
}
```

---

## File: crates/shared/src/common/taxonomy_message_vo.rs

```rust
// PURPOSE: ComplianceStatus, LintMessage — VOs for compliance status and violation messages
use crate::string_value_object;

string_value_object!(LintMessage);

/// Boolean compliance flag. Written manually because `bool` is not supported
/// by the `string_value_object!` macro (`i64 as bool` is not a valid Rust cast).
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ComplianceStatus {
    pub value: bool,
}

impl ComplianceStatus {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}

impl std::fmt::Display for ComplianceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<bool> for ComplianceStatus {
    fn from(v: bool) -> Self {
        Self { value: v }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_name_vo.rs

```rust
// PURPOSE: NameVariants, SymbolName — value objects for symbol naming and naming convention variants
use crate::string_value_object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NameVariants {
    pub values: Vec<SymbolName>,
}

impl NameVariants {
    pub fn new(value: Vec<SymbolName>) -> Self {
        Self { values: value }
    }
    pub fn iter(&self) -> std::slice::Iter<'_, SymbolName> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: SymbolName) {
        self.values.push(item);
    }
}

string_value_object!(SymbolName);
```

---

## File: crates/shared/src/common/taxonomy_path_vo.rs

```rust
// PURPOSE: FilePath, DirectoryPath — value objects for validated file and directory paths
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// file_path_vo — File and directory path value objects.
///
/// File path identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FilePath {
    pub value: String,
}

impl FilePath {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new FilePath from a string.
    ///
    /// # Errors
    /// Returns an error if the path is invalid (empty or only whitespace).
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let mut value = value.into();
        if value.trim().is_empty() {
            return Err("File path cannot be empty".to_string());
        }
        // Normalize: replace backslashes with forward slashes, collapse repeated slashes.
        let mut normalized = String::with_capacity(value.len());
        let mut prev_slash = false;
        for c in value.chars() {
            if c == '/' || c == '\\' {
                if !prev_slash {
                    normalized.push('/');
                    prev_slash = true;
                }
            } else {
                normalized.push(c);
                prev_slash = false;
            }
        }
        value = normalized;
        // Remove trailing slashes
        let trimmed = value.trim_end_matches('/');
        value = if trimmed.is_empty() {
            "/".to_string()
        } else {
            trimmed.to_string()
        };
        // If after normalization it's empty, then it was all slashes -> treat as root
        if value.is_empty() {
            return Ok(FilePath {
                value: "/".to_string(),
            });
        }
        Ok(FilePath { value })
    }

    /// File extension without dot.
    pub fn extension(&self) -> String {
        let special_files = [
            "Makefile",
            "Dockerfile",
            "Dockerfile.dev",
            "Dockerfile.prod",
            ".bashrc",
            ".profile",
            ".zshrc",
            ".gitignore",
            ".dockerignore",
        ];
        // Operate on the basename, not the full path — `./foo.rs` must still yield
        // `rs` as its extension, and `.bashrc` (which is fully a basename) must NOT
        // be confused with a hidden file mid-path.
        let basename = match self.value.rsplit('/').next() {
            Some(b) => b,
            None => return String::new(),
        };
        if special_files.contains(&basename) || basename.starts_with('.') {
            return String::new();
        }
        match basename.rsplit_once('.') {
            Some((_, ext)) => ext.to_string(),
            None => String::new(),
        }
    }

    /// Check if path has given extension (without dot).
    pub fn has_extension(&self, ext: &str) -> bool {
        self.extension().eq_ignore_ascii_case(ext)
    }

    /// Extract filename/basename of the path.
    pub fn basename(&self) -> String {
        match self.value.rsplit('/').next() {
            Some(f) => f.to_string(),
            None => self.value.clone(),
        }
    }

    /// Check if the path is a barrel file (module re-export aggregator).
    pub fn is_barrel_file(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py" | "mod.rs" | "index.ts" | "index.js" | "index.tsx" | "index.jsx"
        )
    }

    /// Check if the path is a module/layer entry point file.
    pub fn is_entry_point(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py"
                | "main.py"
                | "py.typed"
                | "app.py"
                | "lib.rs"
                | "main.rs"
                | "index.ts"
                | "index.js"
                | "index.tsx"
                | "index.jsx"
                | "main.ts"
                | "main.js"
                | "app.ts"
                | "app.js"
        )
    }
}

impl std::ops::Deref for FilePath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Hash for FilePath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

/// Directory path identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct DirectoryPath {
    pub value: String,
}

impl DirectoryPath {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new DirectoryPath from a string.
    ///
    /// # Errors
    /// Returns an error if the path is invalid (empty or only whitespace).
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let mut value = value.into();
        if value.trim().is_empty() {
            return Err("Directory path cannot be empty".to_string());
        }
        // Normalize: replace backslashes with forward slashes, and remove trailing slash.
        value = value.replace('\\', "/");
        // Remove trailing slashes
        let trimmed = value.trim_end_matches('/');
        value = if trimmed.is_empty() {
            "/".to_string()
        } else {
            trimmed.to_string()
        };
        Ok(DirectoryPath { value })
    }
}

impl std::ops::Deref for DirectoryPath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for DirectoryPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'de> serde::Deserialize<'de> for DirectoryPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DirectoryPath::new(s).map_err(serde::de::Error::custom)
    }
}

impl Hash for DirectoryPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
```

---

## File: crates/shared/src/common/taxonomy_paths_vo.rs

```rust
// PURPOSE: FilePathList, DirectoryPath, SourceDir — VOs for file/directory path collections
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RenamedFile {
    pub old_path: FilePath,
    pub new_path: FilePath,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RenamedFileList {
    pub values: Vec<RenamedFile>,
}

impl RenamedFileList {
    pub fn new(value: Vec<RenamedFile>) -> Self {
        Self { values: value }
    }
    pub fn iter(&self) -> std::slice::Iter<'_, RenamedFile> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: RenamedFile) {
        self.values.push(item);
    }
}

impl RenamedFile {
    pub fn new(old_path: FilePath, new_path: FilePath) -> Self {
        Self { old_path, new_path }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FilePathList {
    pub values: Vec<FilePath>,
}

impl FilePathList {
    pub fn new(value: Vec<FilePath>) -> Self {
        Self { values: value }
    }
}

impl FilePathList {
    pub fn iter(&self) -> std::slice::Iter<'_, FilePath> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: FilePath) {
        self.values.push(item);
    }
}

impl std::ops::Deref for FilePathList {
    type Target = Vec<FilePath>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}
```

---

## File: crates/shared/src/common/taxonomy_severity_vo.rs

```rust
// PURPOSE: Severity — value object for violation severity levels (critical, high, medium, low)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum Severity {
    #[serde(rename = "info")]
    #[default]
    INFO,
    #[serde(rename = "low")]
    LOW,
    #[serde(rename = "medium")]
    MEDIUM,
    #[serde(rename = "high")]
    HIGH,
    #[serde(rename = "critical")]
    CRITICAL,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::INFO => write!(f, "info"),
            Severity::LOW => write!(f, "low"),
            Severity::MEDIUM => write!(f, "medium"),
            Severity::HIGH => write!(f, "high"),
            Severity::CRITICAL => write!(f, "critical"),
        }
    }
}

impl Severity {
    pub fn score_impact(&self) -> f64 {
        match self {
            Severity::INFO => 0.0,
            Severity::LOW => 1.0,
            Severity::MEDIUM => 2.0,
            Severity::HIGH => 3.0,
            Severity::CRITICAL => 5.0,
        }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_source_vo.rs

```rust
// PURPOSE: ContentString, SourceContentVO — VOs for source code content representation
use crate::string_value_object;
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_path_vo::FilePath;

string_value_object!(ContentString);

/// Source content value object: combines a file path, a `ContentString`
/// payload, and a language marker. Carries three fields rather than one,
/// so it does not fit the single-field `string_value_object!` macro;
/// defined manually.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceContentVO {
    pub file_path: FilePath,
    pub content: ContentString,
    pub language: String,
}

impl SourceContentVO {
    pub fn new(file_path: FilePath, content: ContentString, language: impl Into<String>) -> Self {
        Self {
            file_path,
            content,
            language: language.into(),
        }
    }
}
```

---

## File: crates/shared/src/common/utility_file_handler.rs

```rust
// PURPOSE: File & workspace utility — pure logic + I/O, free functions only
// Single source of truth for file walking, ignored path matching, source file detection,
// and workspace root detection.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::common::taxonomy_filesystem_error::FileSystemError;
use crate::common::taxonomy_path_vo::DirectoryPath;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;

/// Check if a file extension is a known source file.
pub fn is_source_file(ext: &str) -> bool {
    matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx")
}

/// Check if a directory is in the ignored list.
pub fn is_ignored_dir(dir: &Path, ignored: &[String]) -> bool {
    let s = dir.to_string_lossy();
    is_path_ignored(&s, ignored)
}

/// Collect a single source file path into the output vector.
pub fn collect_source_file(path: &Path, files: &mut Vec<FilePath>) {
    if let Some(path_str) = path.to_str() {
        if let Ok(fp) = FilePath::new(path_str.to_string()) {
            files.push(fp);
        }
    }
}

/// Return true if `rel_path` should be skipped based on `ignored` patterns.
pub fn is_path_ignored(rel_path: &str, ignored: &[String]) -> bool {
    if rel_path.is_empty() {
        return false;
    }
    let segments: Vec<&str> = rel_path
        .split(['/', '\\'])
        .filter(|s| !s.is_empty())
        .collect();
    for pat in ignored {
        if pat.is_empty() {
            continue;
        }
        if let Some(stripped) = pat.strip_prefix('/') {
            if stripped.is_empty() {
                continue;
            }
            let pat_segments: Vec<&str> = stripped
                .split(['/', '\\'])
                .filter(|s| !s.is_empty())
                .collect();
            if pat_segments.is_empty() {
                continue;
            }
            let n_pat = pat_segments.len();
            let n_seg = segments.len();
            if n_seg < n_pat {
                continue;
            }
            for start in 0..=(n_seg - n_pat) {
                if segments[start..start + n_pat] == pat_segments[..] {
                    return true;
                }
            }
            continue;
        }

        // Handle **/*.rs patterns (recursive glob)
        if pat.starts_with("**/") {
            let suffix = pat.strip_prefix("**/").unwrap_or(pat);
            if let Some(ext_pattern) = suffix.strip_prefix("*.") {
                let ext = ext_pattern.trim_start_matches('.');
                if !ext.is_empty() {
                    let basename = segments.last().copied().unwrap_or_default();
                    if basename.ends_with(&format!(".{ext}")) {
                        return true;
                    }
                }
            }
            continue;
        }

        // Handle target/* patterns (prefix with wildcard)
        if let Some(prefix) = pat.strip_suffix("/*") {
            if !prefix.is_empty() && segments.first() == Some(&prefix) {
                return true;
            }
            continue;
        }

        if let Some(suffix) = pat.strip_prefix("*.") {
            let suffix = suffix.trim_start_matches('.');
            if suffix.is_empty() {
                continue;
            }
            let basename = segments.last().copied().unwrap_or_default();
            if basename.ends_with(&format!(".{suffix}")) {
                return true;
            }
            continue;
        }

        if pat.starts_with('.') {
            if segments.iter().any(|seg| *seg == pat) {
                return true;
            }
            continue;
        }
        let pat_segments: Vec<&str> = pat.split(['/', '\\']).filter(|s| !s.is_empty()).collect();
        if pat_segments.len() == 1 {
            if segments.contains(&pat_segments[0]) {
                return true;
            }
        } else if pat_segments.len() > 1 {
            let n_pat = pat_segments.len();
            let n_seg = segments.len();
            if n_seg >= n_pat {
                for start in 0..=(n_seg - n_pat) {
                    if segments[start..start + n_pat] == pat_segments[..] {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Collect all lintable source files from a directory tree.
pub fn collect_all_source_files(dir: &Path, ignored_paths: &[String]) -> Vec<FilePath> {
    let mut files = Vec::new();
    if dir.exists() && dir.is_dir() {
        walk_source_files(dir, &mut files, ignored_paths);
    }
    files
}

/// Collect all lintable source files without applying default ignores.
pub fn collect_all_source_files_raw(dir: &Path) -> Vec<FilePath> {
    let mut files = Vec::new();
    if dir.exists() && dir.is_dir() {
        let ignored: Vec<String> = Vec::new();
        walk_source_files(dir, &mut files, &ignored);
    }
    files
}

/// Scan a directory and return files as FilePathList (replaces IScannerProviderProtocol).
pub fn scan_directory(
    path: &DirectoryPath,
    ignored_paths: &[String],
) -> Result<FilePathList, FileSystemError> {
    let dir = std::path::Path::new(&path.value);
    if !dir.exists() || !dir.is_dir() {
        return Ok(FilePathList { values: vec![] });
    }
    let files = collect_all_source_files(dir, ignored_paths);
    Ok(FilePathList { values: files })
}

/// Walk a directory tree collecting all source files, skipping ignored directories.
/// Symlink targets outside the root directory are pruned to prevent path traversal.
/// Uses canonical-path-based visited set (works on all platforms).
pub fn walk_source_files(dir: &Path, files: &mut Vec<FilePath>, ignored: &[String]) {
    let root = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    let mut visited = HashSet::<PathBuf>::new();
    walk_source_files_inner(&root, files, ignored, &mut visited, &root)
}

fn walk_source_files_inner(
    dir: &Path,
    files: &mut Vec<FilePath>,
    ignored: &[String],
    visited: &mut HashSet<PathBuf>,
    root: &Path,
) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if is_ignored_dir(&path, ignored) {
                continue;
            }
            if let Ok(sym_meta) = std::fs::symlink_metadata(&path) {
                if sym_meta.file_type().is_symlink() {
                    if let Ok(target) = std::fs::canonicalize(&path) {
                        // P4.1 fix: prevent symlink escape — skip targets outside root
                        if !target.starts_with(root) {
                            continue;
                        }
                        if !visited.insert(target.clone()) {
                            continue;
                        }
                        if let Ok(target_meta) = target.metadata() {
                            if target_meta.is_dir() {
                                walk_source_files_inner(&target, files, ignored, visited, root);
                            } else if target_meta.is_file() {
                                collect_source_file(&target, files);
                            }
                        }
                    }
                    continue;
                }
            }
            if path.is_dir() {
                let dir_name = path
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_default();
                if dir_name == "tests" {
                    continue;
                }
                let canonical = std::fs::canonicalize(&path).unwrap_or_else(|_| path.to_path_buf());
                if !visited.insert(canonical) {
                    continue;
                }
                walk_source_files_inner(&path, files, ignored, visited, root);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if is_source_file(ext) {
                    collect_source_file(&path, files);
                }
            }
        }
    }
}

/// Walk a directory tree collecting all .rs files.
/// Contained to `dir` (symlink targets outside the root are pruned).
/// Uses canonical-path-based visited set (works on all platforms).
#[rustfmt::skip]
pub fn walk_rs_files
    (dir: &Path, cb: &mut dyn FnMut(PathBuf), ignored: &[String]) {
    let root = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    let mut visited = HashSet::<PathBuf>::new();
    walk_rs_files_inner(&root, cb, ignored, &mut visited, &root)
}

fn walk_rs_files_inner(
    dir: &Path,
    cb: &mut dyn FnMut(PathBuf),
    ignored: &[String],
    visited: &mut HashSet<PathBuf>,
    root: &Path,
) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if is_ignored_dir(&p, ignored) {
                continue;
            }
            if let Ok(sym_meta) = std::fs::symlink_metadata(&p) {
                if sym_meta.file_type().is_symlink() {
                    if let Ok(target) = std::fs::canonicalize(&p) {
                        if !target.starts_with(root) {
                            continue;
                        }
                        // Use canonical path instead of inode (P2.1)
                        if !visited.insert(target.clone()) {
                            continue;
                        }
                        if let Ok(target_meta) = target.metadata() {
                            if target_meta.is_dir() {
                                walk_rs_files_inner(&target, cb, ignored, visited, root);
                            } else if target_meta.is_file()
                                && target.starts_with(root)
                                && matches!(target.extension().and_then(|e| e.to_str()), Some("rs"))
                            {
                                cb(target);
                            }
                        }
                    }
                    continue;
                }
            }
            if p.is_dir() {
                // Use canonical path instead of inode (P2.1)
                let canonical = std::fs::canonicalize(&p).unwrap_or_else(|_| p.to_path_buf());
                if !visited.insert(canonical) {
                    continue;
                }
                walk_rs_files_inner(&p, cb, ignored, visited, root);
            } else if matches!(p.extension().and_then(|e| e.to_str()), Some("rs")) {
                cb(p);
            }
        }
    }
}

/// Read file content synchronously. Returns Ok(content) or Err(io::Error).
pub fn read_file_sync(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

/// Get file basename (filename without directory path).
pub fn get_basename(path: &str) -> &str {
    std::path::Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
}

/// Get file stem (filename without extension and directory).
pub fn get_file_stem(path: &str) -> &str {
    std::path::Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
}

/// Check if path is a directory.
pub fn is_directory(path: &str) -> bool {
    std::path::Path::new(path).is_dir()
}

/// Check if path is a file.
pub fn is_file(path: &str) -> bool {
    std::path::Path::new(path).is_file()
}

/// Get parent directory path.
pub fn get_parent(path: &str) -> &str {
    std::path::Path::new(path)
        .parent()
        .and_then(|p| p.to_str())
        .unwrap_or("")
}

/// Read file content synchronously. Returns Ok(content) or Err(io::Error).
pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

/// Read file content, returning empty string on error.
pub fn read_file_safe(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

/// Maximum allowed file size for full memory reads (10MB).
pub const MAX_FILE_READ_SIZE_BYTES: u64 = 10 * 1024 * 1024;

/// Read file content with generic path, verifying file size does not exceed MAX_FILE_READ_SIZE_BYTES.
pub fn read_file_generic<P: AsRef<std::path::Path>>(path: P) -> Result<String, std::io::Error> {
    let p = path.as_ref();
    let metadata = fs::metadata(p)?;
    if metadata.len() > MAX_FILE_READ_SIZE_BYTES {
        return Err(std::io::Error::other(format!(
            "File size {} bytes exceeds maximum limit of {} bytes",
            metadata.len(),
            MAX_FILE_READ_SIZE_BYTES
        )));
    }
    fs::read_to_string(p)
}

/// Check if path exists.
pub fn path_exists<P: AsRef<std::path::Path>>(path: P) -> bool {
    path.as_ref().exists()
}

/// Write content to file.
pub fn write_file<P: AsRef<std::path::Path>, C: AsRef<[u8]>>(
    path: P,
    contents: C,
) -> std::io::Result<()> {
    fs::write(path, contents)
}

/// Check if path is a directory (generic).
pub fn is_dir<P: AsRef<std::path::Path>>(path: P) -> bool {
    path.as_ref().is_dir()
}

/// Check if path is a file (generic).
pub fn is_file_generic<P: AsRef<std::path::Path>>(path: P) -> bool {
    path.as_ref().is_file()
}

/// Walk up from `start` looking for workspace root markers.
/// Returns the first directory containing Cargo.toml, crates/, packages/, or modules/.
pub fn find_workspace_root(start: &str) -> Option<std::path::PathBuf> {
    let mut dir = std::path::Path::new(start).to_path_buf();
    if !dir.is_absolute() {
        dir = std::env::current_dir().ok()?.join(&dir);
    }
    loop {
        // Priority 1: workspace root markers (crates/, packages/, modules/)
        if dir.join("crates").is_dir()
            || dir.join("packages").is_dir()
            || dir.join("modules").is_dir()
        {
            return Some(dir);
        }
        // Priority 2: Cargo.toml (only if not inside a workspace member)
        if dir.join("Cargo.toml").exists() {
            // Check if parent or grandparent has workspace markers — if so, keep walking up
            if let Some(parent) = dir.parent() {
                let parent_name = parent
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                if parent.join("crates").is_dir()
                    || parent.join("packages").is_dir()
                    || parent.join("modules").is_dir()
                    || matches!(parent_name, "crates" | "packages" | "modules")
                {
                    // Don't return yet — parent/grandparent is the real workspace root
                } else {
                    return Some(dir);
                }
            } else {
                return Some(dir);
            }
        }
        if !dir.pop() {
            return None;
        }
    }
}
```

---

## File: crates/shared/src/common/utility_layer_detector.rs

```rust
// PURPOSE: Layer detection utility — pure function, simple prefix check
use std::collections::HashMap;
use std::path::Path;

use crate::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use crate::taxonomy_layer_vo::LayerNameVO;

/// Detect architectural layer from filename prefix.
///
/// Returns the layer name if the filename starts with a valid prefix, otherwise None.
///
/// # Examples
/// - "taxonomy_foo.rs" → Some("taxonomy")
/// - "contract_bar.rs" → Some("contract")
/// - "foo.rs" → None
pub fn detect_layer_from_prefix(filename: &str) -> Option<String> {
    let stem = Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    const PREFIX_MAP: &[(&str, &str)] = &[
        ("taxonomy_", "taxonomy"),
        ("contract_", "contract"),
        ("capabilities_", "capabilities"),
        ("utility_", "utility"),
        ("agent_", "agent"),
        ("surface_", "surface"),
        ("root_", "root"),
    ];

    for &(prefix, layer) in PREFIX_MAP {
        if stem.starts_with(prefix) {
            return Some(layer.to_string());
        }
    }

    None
}

/// Resolve specialised sub-layer from file suffix.
///
/// E.g., "capabilities_command" with base_layer="capabilities":
///   → suffix = "command"
///   → checks if "capabilities(command)" exists in config
///   → returns "capabilities(command)" if found, else "capabilities"
pub fn resolve_specialized_layer(
    base_layer: &str,
    file_path: &str,
    layer_keys: &[String],
) -> String {
    let basename = Path::new(file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    if let Some(underscore_pos) = basename.rfind('_') {
        let suffix = &basename[underscore_pos + 1..];
        if !suffix.is_empty() {
            let specialized = format!("{}({})", base_layer, suffix);
            if layer_keys.contains(&specialized) {
                return specialized;
            }
        }
    }

    base_layer.to_string()
}

/// Detect layer from module path (from import statement).
///
/// Tries 3 strategies:
/// 1. Direct segment match (e.g., "shared::taxonomy::..." → "taxonomy")
/// 2. Prefix-based match (e.g., "taxonomy_definition_vo" → "taxonomy")
pub fn detect_module_layer(module: &str, layer_names: &[String]) -> Option<String> {
    let meaningful_parts: Vec<&str> = module
        .split([':', '.', '/', '\\'])
        .filter(|p| !p.is_empty())
        .collect();

    if meaningful_parts.is_empty() {
        return None;
    }

    // Strategy 1: Direct match with layer names
    for name in layer_names {
        let base_name = match name.split('(').next() {
            Some(s) => s,
            None => name,
        };
        if meaningful_parts.contains(&base_name) {
            return Some(base_name.to_string());
        }
    }

    // Strategy 2: Prefix-based match
    for part in &meaningful_parts {
        if let Some(layer) = detect_layer_from_prefix(part) {
            return Some(layer);
        }
    }

    None
}

/// Extract filename from file path.
///
/// Returns the filename (last component) as a string slice, or empty string if extraction fails.
pub fn extract_filename(file_path: &str) -> &str {
    Path::new(file_path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
}

/// Collect layer keys as strings from a LayerMapVO.
pub fn collect_layer_keys(layer_map: &LayerMapVO) -> Vec<String> {
    layer_map.values.keys().map(|k| k.to_string()).collect()
}

/// Look up a LayerDefinition by layer name string.
///
/// Tries direct lookup first, then falls back to base name (before parenthesis).
pub fn get_layer_def<'a>(
    layer: &str,
    layers: &'a HashMap<LayerNameVO, LayerDefinition>,
) -> Option<&'a LayerDefinition> {
    layers.get(&LayerNameVO::new(layer)).or_else(|| {
        let base = match layer.split('(').next() {
            Some(s) => s,
            None => layer,
        };
        layers.get(&LayerNameVO::new(base))
    })
}
```

---

## File: crates/shared/src/common/utility_scope_matcher.rs

```rust
// PURPOSE: Utility functions for scope-based file matching in import rules
// Extracted from forbidden/mandatory checkers to eliminate duplicated
// stem/suffix extraction and scope-membership logic.

use crate::common::taxonomy_layer_vo::Identity;
use crate::import_rules::utility_import_resolver;

/// Check if a file belongs to a given scope rule based on its filename.
///
/// Returns `Some((layer_prefix, suffixes))` when the file's stem matches
/// the scope's layer prefix and suffix constraints, or `None` otherwise.
///
/// This function handles:
/// - Extracting the stem (filename without extension)
/// - Resolving the scope to get expected layer and suffixes
/// - Checking if stem starts with `{layer}_` prefix
/// - Verifying suffix constraints if the scope has them
///
/// # Examples
/// ``` `rust
/// use shared_lint_arwaky::common::utility_scope_matcher::file_belongs_to_scope;
/// use shared_lint_arwaky::common::taxonomy_layer_vo::Identity;
///
/// // "surfaces_auth.rs" belongs to layer "surfaces" (no suffix constraint)
/// let result = file_belongs_to_scope("surfaces_auth.rs", &Identity::new("surfaces"));
/// assert!(result.is_some());
/// ``` `
pub fn file_belongs_to_scope(
    basename: &str,
    scope_identity: &Identity,
) -> Option<(String, Vec<Identity>)> {
    let stem = extract_file_stem(basename);

    // Resolve scope to get expected layer and suffixes
    let (expected_layer, suffixes) = utility_import_resolver::resolve_scope(scope_identity);
    let expected_prefix = expected_layer.value();

    // Check if stem starts with `{layer}_` prefix
    let expected_pattern = format!("{}_{}", expected_prefix, "");
    if !stem.starts_with(&expected_pattern) {
        return None;
    }

    // Check suffix constraint if any
    if !suffixes.is_empty() {
        let file_suffix = extract_suffix(stem);
        let suffix_match = suffixes.iter().any(|s| s.value() == file_suffix);
        if !suffix_match {
            return None;
        }
    }

    Some((expected_prefix.to_string(), suffixes))
}

/// Extract the file stem (without extension) from a basename.
///
/// # Examples
/// ``` `rust
/// use shared_lint_arwaky::common::utility_scope_matcher::extract_file_stem;
///
/// assert_eq!(extract_file_stem("surfaces_auth.rs"), "surfaces_auth");
/// assert_eq!(extract_file_stem("mod.rs"), "mod");
/// assert_eq!(extract_file_stem("lib.rs"), "lib");
/// ``` `
pub fn extract_file_stem(basename: &str) -> &str {
    basename.rsplit('.').next_back().map_or(basename, |s| s)
}

/// Extract the layer prefix from a file stem (first part before `_`).
///
/// # Examples
/// ``` `rust
/// use shared_lint_arwaky::common::utility_scope_matcher::extract_layer_prefix;
///
/// assert_eq!(extract_layer_prefix("surfaces_auth"), "surfaces");
/// assert_eq!(extract_layer_prefix("utility_parser"), "utility");
/// assert_eq!(extract_layer_prefix("unknown_file"), "unknown");
/// ``` `
pub fn extract_layer_prefix(stem: &str) -> &str {
    stem.split('_').next().map_or("unknown", |s| s)
}

/// Extract the suffix from a file stem (last part after `_`).
///
/// # Examples
/// ``` `rust
/// use shared_lint_arwaky::common::utility_scope_matcher::extract_suffix;
///
/// assert_eq!(extract_suffix("surfaces_auth"), "auth");
/// assert_eq!(extract_suffix("utility_parser"), "parser");
/// assert_eq!(extract_suffix("no_suffix"), "suffix");
/// ``` `
pub fn extract_suffix(stem: &str) -> &str {
    stem.rsplit('_').next().map_or("", |s| s)
}

#[cfg(test)]
mod tests {
    use crate::common::utility_scope_matcher::{
        extract_file_stem, extract_layer_prefix, extract_suffix, file_belongs_to_scope, Identity,
    };

    #[test]
    fn test_file_belongs_to_scope_matches() {
        let scope = Identity::new("surfaces");
        let result = file_belongs_to_scope("surfaces_auth.rs", &scope);
        assert!(result.is_some());
        let (layer, suffixes) = result.unwrap();
        assert_eq!(layer, "surfaces");
        assert!(suffixes.is_empty());
    }

    #[test]
    fn test_file_belongs_to_scope_no_match() {
        let scope = Identity::new("surfaces");
        let result = file_belongs_to_scope("utility_auth.rs", &scope);
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_file_stem() {
        assert_eq!(extract_file_stem("surfaces_auth.rs"), "surfaces_auth");
        assert_eq!(extract_file_stem("mod.rs"), "mod");
        assert_eq!(extract_file_stem("lib.rs"), "lib");
        assert_eq!(extract_file_stem("no_extension"), "no_extension");
    }

    #[test]
    fn test_extract_layer_prefix() {
        assert_eq!(extract_layer_prefix("surfaces_auth"), "surfaces");
        assert_eq!(extract_layer_prefix("utility_parser"), "utility");
        assert_eq!(extract_layer_prefix("unknown_file"), "unknown");
        assert_eq!(extract_layer_prefix("single"), "single");
    }

    #[test]
    fn test_extract_suffix() {
        assert_eq!(extract_suffix("surfaces_auth"), "auth");
        assert_eq!(extract_suffix("utility_parser"), "parser");
        assert_eq!(extract_suffix("no_suffix"), "suffix");
        assert_eq!(extract_suffix("single"), "single");
    }
}
```

---

## File: crates/shared/src/config-system/contract_config_orchestrator_aggregate.rs

```rust
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_error::ConfigError;
use crate::config_system::taxonomy_config_language_vo::ConfigLanguage;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use crate::config_system::taxonomy_source_vo::{ConfigResult, ConfigSource};
use async_trait::async_trait;

#[async_trait]
pub trait IConfigOrchestratorAggregate: Send + Sync {
    async fn load_project_config(&self, project_root: &FilePath) -> ConfigResult;

    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> ConfigResult;

    async fn discover_workspaces(&self, root: &FilePath) -> Vec<WorkspaceInfo>;

    /// Synchronous config loading for container initialization.
    /// Searches workspace root for config YAML, falls back to embedded defaults.
    fn load_config_sync(&self, project_root: &FilePath) -> ArchitectureConfig;

    /// Get ignored paths from config (hardcoded defaults + config values).
    fn ignored_paths(&self, project_root: &FilePath) -> PatternList;

    /// Get ignored paths for a specific language (hardcoded defaults + config values for that language).
    fn ignored_paths_for_language(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> PatternList;

    /// List config files found in the project.
    async fn list_config_files(
        &self,
        project_root: &FilePath,
    ) -> Result<Vec<(ConfigLanguage, FilePath)>, ConfigError>;

    /// Read config content for a specific language.
    async fn read_config(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<Option<ConfigSource>, ConfigError>;
}
```

---

## File: crates/shared/src/config-system/mod.rs

```rust
// config-system — taxonomy and contract types
pub mod contract_config_orchestrator_aggregate;
pub mod contract_parser_protocol;
pub mod contract_reader_protocol;
pub mod contract_validator_protocol;
pub mod contract_workspace_detector_protocol;
pub mod taxonomy_config_error;
pub mod taxonomy_config_language_vo;
pub mod taxonomy_config_vo;
pub mod taxonomy_identifier_vo;
pub mod taxonomy_multi_project_summary_vo;
pub mod taxonomy_multi_project_workspace_info_vo;
pub mod taxonomy_setting_vo;
pub mod taxonomy_source_vo;
pub mod taxonomy_validation_vo;
pub mod utility_config_defaults;
pub mod utility_config_io;
pub mod utility_config_merger;
pub mod utility_config_parser;
```

---

## File: crates/shared/src/config-system/taxonomy_config_vo.rs

```rust
// PURPOSE: ArchitectureConfig, LayerDefinition, ConfigRule — configuration value objects for AES rules definition
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_definition_vo::NamingConfig;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct NamingRuleVO {
    #[serde(default)]
    pub naming_convention: BooleanVO,
    #[serde(default)]
    pub suffix_policy: crate::common::taxonomy_common_vo::SuffixPolicyVO,
    #[serde(default, alias = "allowed_suffix")]
    pub allowed_suffix: PatternList,
    #[serde(default, alias = "forbidden_suffix")]
    pub forbidden_suffix: PatternList,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RoleRuleVO {
    #[serde(default)]
    pub no_domain_logic: BooleanVO,
    #[serde(default)]
    pub must_implement_service_container_aggregate: BooleanVO,
    #[serde(default)]
    pub lazy_eager_initialization_only: BooleanVO,
    #[serde(default)]
    pub stateless_execution: BooleanVO,
    #[serde(default)]
    pub single_execution_goal: BooleanVO,
    #[serde(default)]
    pub high_level_policy_only: BooleanVO,
    #[serde(default)]
    pub coordinates_multiple_orchestrators: BooleanVO,
    #[serde(default)]
    pub crud_only: BooleanVO,
    #[serde(default)]
    pub no_decision_logic: BooleanVO,
    #[serde(default)]
    pub thread_async_safe: BooleanVO,
    #[serde(default)]
    pub no_domain_data_storage: BooleanVO,
    #[serde(default)]
    pub owns_system_health_transitions: BooleanVO,
    #[serde(default)]
    pub lifecycle_tracking_only: BooleanVO,
    #[serde(default)]
    pub no_primitives: BooleanVO,
    #[serde(default)]
    pub forbidden_inheritance: PatternList,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OrphanRuleVO {
    #[serde(default)]
    pub check_orphan: BooleanVO,
    #[serde(default, alias = "entry_points")]
    pub orphan_entry_points: PatternList,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(default)]
pub struct ArchitectureRule {
    pub name: DescriptionVO,
    pub description: DescriptionVO,
    pub rule_type: ErrorCode,
    pub enabled: BooleanVO,
    pub scope: LayerNameVO,
    pub exceptions: PatternList,
    #[serde(default)]
    pub allowed: PatternList,
    #[serde(default)]
    pub forbidden: PatternList,
    #[serde(default)]
    pub mandatory: PatternList,

    #[serde(flatten)]
    pub naming: NamingRuleVO,
    #[serde(flatten)]
    pub code_analysis: crate::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO,
    #[serde(flatten)]
    pub role: RoleRuleVO,
    #[serde(flatten)]
    pub orphan: OrphanRuleVO,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ArchitectureConfig {
    pub enabled: BooleanVO,
    pub layers: std::collections::HashMap<LayerNameVO, LayerDefinition>,
    pub rules: Vec<ArchitectureRule>,
    pub naming: NamingConfig,
    pub ignored_paths: FilePathList,
    pub mandatory_class_definition: BooleanVO,
}

impl ArchitectureConfig {
    pub fn new(
        enabled: BooleanVO,
        layers: std::collections::HashMap<LayerNameVO, LayerDefinition>,
        rules: Vec<ArchitectureRule>,
        naming: NamingConfig,
        ignored_paths: FilePathList,
        mandatory_class_definition: BooleanVO,
    ) -> Self {
        Self {
            enabled,
            layers,
            rules,
            naming,
            ignored_paths,
            mandatory_class_definition,
        }
    }
}

impl Default for ArchitectureConfig {
    fn default() -> Self {
        Self {
            enabled: BooleanVO::new(true),
            layers: HashMap::new(),
            rules: Vec::new(),
            naming: NamingConfig::new(Count::new(3)),
            ignored_paths: FilePathList { values: vec![] },
            mandatory_class_definition: BooleanVO::new(false),
        }
    }
}
```

---

## File: crates/shared/src/import-rules/contract_cycle_import_protocol.rs

```rust
// PURPOSE: ICycleImportProtocol — unified contract for cycle import detection (AES205)
use crate::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use crate::taxonomy_definition_vo::LayerMapVO;
use crate::taxonomy_layer_vo::LayerNameVO;
use crate::taxonomy_name_vo::SymbolName;
use async_trait::async_trait;

#[async_trait]
pub trait ICycleImportProtocol: Send + Sync {
    fn scan(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &[FilePath],
        root_dir: &FilePath,
    ) -> Vec<LintResult>;

    async fn check_cycles(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &crate::common::taxonomy_paths_vo::FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );

    fn detect_cycle_edges(&self, edges: &[DependencyEdge]) -> Vec<SymbolName>;
    fn normalize_to_layer(&self, name: &str) -> LayerNameVO;
}
```

---

## File: crates/shared/src/import-rules/contract_dummy_import_protocol.rs

```rust
// PURPOSE: IDummyImportCheckerProtocol — unified contract for AES204 dummy import checking
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use crate::taxonomy_definition_vo::LayerMapVO;
use crate::taxonomy_layer_vo::Identity;

pub trait IDummyImportCheckerProtocol: Send + Sync {
    fn rule_name(&self) -> Identity;

    fn check_dummy_imports(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    );
    fn check_dummy_functions(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    );
    fn check_dummy_impls(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    );
    fn check_taxonomy_intent(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    );
    fn check_layer_contract_intent(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    );
    fn check_surface_logic(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    );

    /// Run all dummy checks in one call, pre-computing shared data once.
    fn check_all_dummy(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) {
        self.check_dummy_imports(file, content, violations, root_dir, layer_map);
        self.check_dummy_functions(file, content, violations, root_dir, layer_map);
        self.check_dummy_impls(file, content, violations, root_dir, layer_map);
        self.check_taxonomy_intent(file, content, violations, root_dir, layer_map);
        self.check_surface_logic(file, content, violations, root_dir, layer_map);
    }
}
```

---

## File: crates/shared/src/import-rules/contract_import_forbidden_protocol.rs

```rust
// PURPOSE: IImportForbiddenProtocol — exclusive contract for forbidden import checks (AES201)
// Checks that files do NOT import from layers they are forbidden to depend on.
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::taxonomy_definition_vo::LayerMapVO;
use crate::taxonomy_layer_vo::Identity;
use async_trait::async_trait;

/// For each file, verify its imports do not reach any layer listed in the `forbidden` set.
/// Used by the import orchestrator as part of the AES201 gate.
#[async_trait]
pub trait IImportForbiddenProtocol: Send + Sync {
    fn rule_name(&self) -> Identity;
    async fn check_forbidden_imports(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
```

---

## File: crates/shared/src/import-rules/contract_import_mandatory_protocol.rs

```rust
// PURPOSE: IImportMandatoryProtocol — exclusive contract for mandatory import checks (AES202)
// Verifies that every file imports from the layers it MUST depend on per its layer scope.
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::taxonomy_definition_vo::LayerMapVO;
use crate::taxonomy_layer_vo::Identity;
use async_trait::async_trait;

/// For each file, check that at least one import targets each layer in the `mandatory` set.
/// Used by the import orchestrator as part of the AES202 gate.
#[async_trait]
pub trait IImportMandatoryProtocol: Send + Sync {
    fn rule_name(&self) -> Identity;
    async fn run_mandatory_imports(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
```

---

## File: crates/shared/src/import-rules/contract_import_runner_aggregate.rs

```rust
// PURPOSE: IImportRunnerAggregate — contract for import-rules feature orchestrator
//
// This is the primary contract that decouples the import-rules agent layer
// from its callers (CLI, MCP, TUI). Callers depend on this trait, not on
// ImportOrchestrator directly.
//
// run_audit is async because it may perform file I/O and spawn blocking
// tasks internally. The caller provides a FilePath target (file or dir).
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_adapter_error::ScanError;
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

/// IImportRunnerAggregate — aggregate protocol for import-rules orchestration.
///
/// Implemented by ImportOrchestrator (agent layer).
/// Called by surface layer (CLI, MCP, TUI) via Arc<dyn IImportRunnerAggregate>.
#[async_trait]
pub trait IImportRunnerAggregate: Send + Sync {
    /// Run all 5 import-related AES checks (AES201–AES205) on the target.
    /// Returns aggregated violations from mandatory, forbidden, unused, dummy, and cycle checks.
    async fn run_audit(&self, target: &FilePath) -> Result<Vec<LintResult>, ScanError>;
    /// Human-readable name for this orchestrator ("import-rules").
    fn name(&self) -> &str;
}
```

---

## File: crates/shared/src/import-rules/contract_unused_import_protocol.rs

```rust
// PURPOSE: IUnusedImportProtocol — unified protocol trait for AES203: detect unused imports across Rust, Python, JavaScript
// AES402: All primitive types in this contract have been replaced with taxonomy VOs.
//   * `Vec<String>` returns → `Vec<LintMessage>` (semantic messages, not raw strings)
//   * `&str file_path` params → kept as `&str` (idiomatic borrow, AES402 allows)
//   * `&mut Vec<LintResult>` → kept (`LintResult` is itself a VO)
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;

pub trait IUnusedImportProtocol: Send + Sync {
    /// Find unused imports in a file by path (reads file internally).
    /// Returns a list of human-readable lint messages describing each unused
    /// import. Replaces the previous `Vec<String>` so callers can introspect,
    /// translate, or log messages without parsing free-form strings.
    fn find_unused_imports(&self, path: &FilePath) -> Vec<LintMessage>;

    /// Check unused imports given file content directly (for inline checking).
    /// Useful when content is already available (avoids re-reading file).
    fn check_unused_imports(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
}
```

---

## File: crates/shared/src/import-rules/mod.rs

```rust
// import-rules — taxonomy and contract types
pub mod contract_cycle_import_protocol;
pub mod contract_dummy_import_protocol;
pub mod contract_import_forbidden_protocol;
pub mod contract_import_mandatory_protocol;
pub mod contract_import_runner_aggregate;
pub mod contract_unused_import_protocol;
pub mod taxonomy_dependency_edge_vo;
pub mod taxonomy_forbidden_rule_config_vo;
pub mod taxonomy_graph_color_vo;
pub mod taxonomy_import_constant;
pub mod taxonomy_violation_import_vo;
pub mod utility_cycle_detector;
pub mod utility_dummy_detector;
pub mod utility_import_module_parser;
pub mod utility_import_resolver;
pub mod utility_import_symbol_extractor;
pub mod utility_path_normalizer;
```

---

## File: crates/shared/src/import-rules/taxonomy_dependency_edge_vo.rs

```rust
// PURPOSE: DependencyEdge — Value Object representing a directed dependency edge between layers
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DependencyEdge {
    pub source: String,
    pub target: String,
}

impl DependencyEdge {
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
        }
    }
}
```

---

## File: crates/shared/src/import-rules/taxonomy_forbidden_rule_config_vo.rs

```rust
// PURPOSE: ForbiddenRuleConfigVO — Value object for forbidden import rule configuration
use crate::taxonomy_layer_vo::LayerNameVO;

pub struct ForbiddenRuleConfigVO<'a> {
    pub forbidden_list: &'a [String],
    pub source_layer: &'a LayerNameVO,
    pub allowed_values: &'a [String],
}
```

---

## File: crates/shared/src/import-rules/taxonomy_graph_color_vo.rs

```rust
// PURPOSE: GraphColorVO — 3-color DFS graph traversal state VO for cycle detection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GraphColorVO {
    #[default]
    White,
    Gray,
    Black,
}
```

---

## File: crates/shared/src/import-rules/taxonomy_import_constant.rs

```rust
// PURPOSE: taxonomy_import_constant — compile-time constants for import-rules layer
// All domain values MUST be named constants. No hardcoded literals in layer files.

/// Known derive-macro imports that Rust compiler consumes implicitly.
/// These are never "used" as ordinary symbols — they're consumed by #[derive(...)]
/// attributes, so they must never be flagged as unused.
pub const DERIVE_MACROS: &[&str] = &[
    "async_trait",
    "Serialize",
    "Deserialize",
    "Clone",
    "Debug",
    "Default",
    "PartialEq",
    "Eq",
    "Hash",
    "Ord",
    "PartialOrd",
    "Copy",
    "EnumIter",
    "Display",
    "EnumString",
    "AsRefStr",
];

/// Layer prefixes used for filename-based layer detection.
pub const LAYER_PREFIXES: &[(&str, &str)] = &[
    ("taxonomy_", "taxonomy"),
    ("contract_", "contract"),
    ("utility_", "utility"),
    ("capabilities_", "capabilities"),
    ("agent_", "agent"),
    ("surface_", "surfaces"),
    ("root_", "root"),
];

/// Rust entry file names that should be skipped during scope-level checks.
pub const RUST_ENTRY_FILES: &[&str] = &["mod.rs", "lib.rs", "main.rs"];

/// Python entry file names that should be skipped during mandatory checks.
pub const PYTHON_ENTRY_FILES: &[&str] = &["__init__.py"];

/// Source code file extensions for file collection.
pub const SOURCE_EXTENSIONS: &[&str] = &["rs", "py", "js", "ts", "jsx", "tsx"];

/// Directories to skip during file collection (build artifacts, dependencies, caches).
pub const DEFAULT_SKIP_DIRS: &[&str] = &[
    ".git",
    "node_modules",
    "target",
    "dist",
    "build",
    ".venv",
    "__pycache__",
];
```

---

## File: crates/shared/src/import-rules/taxonomy_violation_import_vo.rs

```rust
// PURPOSE: AesImportViolation — violation messages for import rules (AES201-205)
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_name_vo::SymbolName;
use std::fmt;

#[derive(Debug, Clone)]
pub enum AesImportViolation {
    // AES201 — Forbidden Import
    ForbiddenImport {
        source_layer: LayerNameVO,
        forbidden_layer: LayerNameVO,
        allowed: Vec<LayerNameVO>,
        reason: Option<LintMessage>,
    },
    // AES202 — Mandatory import
    MissingImport {
        source_layer: LayerNameVO,
        required: SymbolName,
        reason: Option<LintMessage>,
    },
    // AES203 — Unused imports
    FixUnusedImport {
        reason: Option<LintMessage>,
    },
    // AES204 — Dummy import / Intent violation
    ImportIntentViolation {
        source_layer: LayerNameVO,
        import_type: SymbolName,
        intent: SymbolName,
        reason: Option<LintMessage>,
    },
    // AES205 — Circular import
    CircularImport {
        reason: Option<LintMessage>,
    },
}

impl fmt::Display for AesImportViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AesImportViolation::ForbiddenImport {
                source_layer,
                forbidden_layer,
                allowed,
                reason,
            } => {
                let (allowed_str, fix_extra) = if allowed.is_empty() {
                    ("none".to_string(), " This layer is fully isolated — move the imported code into this layer or remove the dependency entirely.".to_string())
                } else {
                    (
                        allowed
                            .iter()
                            .map(|v| v.value().to_string())
                            .collect::<Vec<String>>()
                            .join(", "),
                        String::new(),
                    )
                };
                let dynamic_why = match reason {
                    Some(r) => r.to_string(),
                    None => {
                        let src = source_layer.value();
                        if src == "taxonomy(vo)" {
                            "Taxonomy Value Objects (VO) must remain completely pure and cannot import agents, surfaces, contracts, utility, capabilities, or root components.".to_string()
                        } else if src == "taxonomy(entity)"
                            || src == "taxonomy(error)"
                            || src == "taxonomy(event)"
                        {
                            "Taxonomy Entities, Errors, and Events can only import taxonomy VOs/constants and are forbidden from importing agents, surfaces, contracts, utility, or capabilities.".to_string()
                        } else if src == "taxonomy(constant)" {
                            "Taxonomy Constants must remain pure static value declarations and cannot import agents, surfaces, contracts, utility, capabilities, or root components.".to_string()
                        } else if src == "contract(protocol)" {
                            "Contract Protocols represent pure interface definitions and are forbidden from importing agents, surfaces, capabilities, utility, aggregates, or root components.".to_string()
                        } else if src == "contract(aggregate)" {
                            "Contract Aggregates represent high-level composition/DI contracts and must not import agents, surfaces, capabilities, utility, or root components.".to_string()
                        } else if src == "utility" {
                            "Utility files contain stateless standalone functions and must only import taxonomy. They cannot import agents, surfaces, contracts, capabilities, or root components.".to_string()
                        } else if src == "capabilities" {
                            "Capabilities implement domain business logic and must never depend on agents, UI/surfaces, or other capabilities.".to_string()
                        } else if src == "agent(container)" {
                            "Agent Containers handle dependency injection and are forbidden from importing UI/surfaces or root components.".to_string()
                        } else if src == "agent(orchestrator)" {
                            "Agent Orchestrators coordinate flows and are forbidden from importing UI/surfaces, capabilities, or root components.".to_string()
                        } else if src == "agent(lifecycle)" {
                            "Agent Lifecycles manage agent states and are forbidden from importing orchestrators/containers, capabilities, surfaces, or root components.".to_string()
                        } else if src == "surfaces(command)"
                            || src == "surfaces(controller)"
                            || src == "surfaces(page)"
                            || src == "surfaces(entry)"
                        {
                            "Smart Surfaces act as user/CLI entry points and must never import agents, capabilities, or ports/protocols directly (must use ServiceContainerAggregate).".to_string()
                        } else if src == "surfaces(hook)"
                            || src == "surfaces(store)"
                            || src == "surfaces(action)"
                            || src == "surfaces(screen)"
                            || src == "surfaces(router)"
                        {
                            "Surface utility components (hooks, stores, routers) manage local state and must never import agents, capabilities, or ports/protocols.".to_string()
                        } else if src == "surfaces(component)"
                            || src == "surfaces(view)"
                            || src == "surfaces(layout)"
                        {
                            "Passive Surface components (views, layouts) render UI and are forbidden from importing agents, contracts, capabilities, or smart surfaces.".to_string()
                        } else if src.starts_with("taxonomy") {
                            "Taxonomy must remain pure and free from framework/layer dependencies to ensure domain model integrity.".to_string()
                        } else if src.starts_with("contract") {
                            "Contract interfaces represent pure specifications and must not depend on capabilities, utility, or agent implementations.".to_string()
                        } else if src.starts_with("agent") {
                            "Agent orchestrators and containers must never depend on the UI/surface layer to maintain clean separation of concerns.".to_string()
                        } else if src.starts_with("surfaces") {
                            "Surfaces are external I/O boundaries and must never bypass contract aggregates to depend on capabilities, agent internals, or utility.".to_string()
                        } else {
                            format!("Layer '{}' must not depend on '{}' to maintain architectural boundaries.", source_layer, forbidden_layer)
                        }
                    }
                };
                write!(
                    f,
                    "AES201 FORBIDDEN_IMPORT: Layer '{}' is importing from forbidden layer '{}'.\n\
                        WHY? {}\n\
                        FIX: Remove the import or refactor to use one of the allowed layers: [{}]{}",
                    source_layer, forbidden_layer, dynamic_why, allowed_str, fix_extra
                )
            }
            AesImportViolation::MissingImport {
                source_layer,
                required,
                reason,
            } => {
                let default_why = {
                    let src = source_layer.value();
                    if src == "taxonomy(vo)" {
                        "Taxonomy Value Objects define domain primitives — they must import contracts to declare their structural contract.".to_string()
                    } else if src == "taxonomy(entity)" {
                        "Taxonomy Entities model domain state — they must import aggregator contracts to participate in business operations.".to_string()
                    } else if src == "contract(protocol)" {
                        "Contract protocols define service boundaries — they must import contract aggregate types to compose cross-cutting workflows.".to_string()
                    } else if src == "contract(aggregate)" {
                        "Contract aggregates orchestrate cross-layer collaboration — they must import all relevant protocol contracts.".to_string()
                    } else if src == "utility" {
                        "Utility files contain stateless standalone functions — they must import taxonomy to access domain types.".to_string()
                    } else if src == "capabilities" {
                        "Capabilities implement business rules — they MUST import contract protocols to know what interface to honor. Missing contract protocol means broken/useless capability or missing requirement.".to_string()
                    } else if src == "agent(container)" {
                        "Agent containers wire dependencies at startup — they must import service contracts to register all concrete implementations.".to_string()
                    } else if src == "agent(orchestrator)" {
                        "Agent orchestrators coordinate use-case flows — they must import capability contracts to dispatch work correctly.".to_string()
                    } else if src == "surfaces(command)" || src == "surfaces(controller)" {
                        "Command/controller surfaces are user entry points — they must import aggregate contracts to delegate without bypassing business logic.".to_string()
                    } else if src == "surfaces(component)" || src == "surfaces(view)" {
                        "Passive surface components render UI — they must import taxonomy VOs to display type-safe domain data.".to_string()
                    } else if src.starts_with("taxonomy") {
                        format!(
                            "Layer '{}' must import '{}' to maintain domain model integrity.",
                            src, required
                        )
                    } else if src.starts_with("contract") {
                        format!("Layer '{}' must import '{}' to satisfy interface composition requirements.", src, required)
                    } else if src.starts_with("agent") {
                        format!(
                            "Layer '{}' must import '{}' to wire all required dependencies.",
                            src, required
                        )
                    } else if src.starts_with("surfaces") {
                        format!("Layer '{}' must import '{}' to properly delegate to the aggregate layer.", src, required)
                    } else {
                        format!("Layer '{}' must import '{}' to satisfy architectural contract requirements.", src, required)
                    }
                };
                let supplement = match reason.as_ref() {
                    Some(r) => format!("\n  Context: {}", r),
                    None => String::new(),
                };
                write!(
                    f,
                    "AES202 MANDATORY_IMPORT: Layer '{}' is missing required import '{}'.\n\
                        WHY? {}{}\n\
                        FIX: Add the required import statement for '{}' in this file.",
                    source_layer, required, default_why, supplement, required
                )
            }
            AesImportViolation::ImportIntentViolation {
                source_layer,
                import_type,
                intent: _,
                reason,
            } => {
                let default_why = format!(
                    "Import '{}' in layer '{}' is not used according to its intended purpose.",
                    import_type, source_layer
                );
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES204 IMPORT_INTENT: '{}' import in layer '{}' violates its intended purpose.\n\
                        WHY? {why}\n\
                        FIX: Use imported symbols in real logic, not only in dummy functions or stubs",
                    import_type, source_layer
                )
            }
            AesImportViolation::CircularImport { reason } => {
                let default_why = "Circular dependencies couple components together and break unidirectional data/import flow.".to_string();
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => default_why,
                };
                write!(
                    f,
                    "AES205 CIRCULAR_IMPORT: Circular dependency detected.\n\
                        WHY? {}\n\
                        FIX: Refactor imports or extract the shared logic into a lower, common layer.",
                    why
                )
            }
            AesImportViolation::FixUnusedImport { reason } => {
                let default_why =
                    "Unused imports clutter the codebase and increase compilation/dependency overhead."
                        .to_string();
                let supplement = match reason.as_ref() {
                    Some(r) => format!("\n  Context: {}", r),
                    None => String::new(),
                };
                write!(f, "AES203 UNUSED_IMPORT: Unused import detected.\n\
                        WHY? {}{}\n\
                        FIX: Remove the unused import statement or use the imported symbol in this file.", default_why, supplement)
            }
        }
    }
}

impl From<AesImportViolation> for String {
    fn from(v: AesImportViolation) -> String {
        v.to_string()
    }
}
```

---

## File: crates/shared/src/import-rules/utility_cycle_detector.rs

```rust
// PURPOSE: Utility functions for cycle detection — pure functions only, no contract dependencies
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use crate::import_rules::taxonomy_graph_color_vo::GraphColorVO;
use crate::taxonomy_name_vo::SymbolName;
use std::collections::{HashMap, HashSet};

pub fn normalize_to_layer(name: &str) -> String {
    let layer_prefixes = [
        "taxonomy_",
        "contract_",
        "capabilities_",
        "utility_",
        "agent_",
        "surface_",
    ];
    let base = match name.rsplit('/').next() {
        Some(b) => b,
        None => name,
    };
    for prefix in &layer_prefixes {
        if base.starts_with(prefix) {
            return prefix.trim_end_matches('_').to_string();
        }
    }
    name.to_string()
}

pub fn detect_cycle_edges(edges: &[DependencyEdge]) -> Vec<SymbolName> {
    let normalized_edges: Vec<DependencyEdge> = edges
        .iter()
        .map(|e| DependencyEdge::new(normalize_to_layer(&e.source), normalize_to_layer(&e.target)))
        .collect();

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for e in &normalized_edges {
        graph
            .entry(e.source.clone())
            .or_default()
            .push(e.target.clone());
        graph.entry(e.target.clone()).or_default();
    }

    let mut color: HashMap<String, GraphColorVO> = HashMap::new();
    let mut parent: HashMap<String, String> = HashMap::new();
    let mut cycle_edges_set: HashSet<(String, String)> = HashSet::new();

    for node in graph.keys() {
        color.entry(node.clone()).or_insert(GraphColorVO::White);
    }

    for node in graph.keys().cloned().collect::<Vec<_>>() {
        if color[&node] == GraphColorVO::White {
            dfs_3color(&node, &graph, &mut color, &mut parent, &mut cycle_edges_set);
        }
    }

    let mut unique_cycles: Vec<String> = Vec::new();
    let mut reported: HashSet<String> = HashSet::new();

    for (src, tgt) in &cycle_edges_set {
        let cycle_nodes = extract_cycle_nodes(src, tgt, &parent);
        if let Some(cycle) = cycle_nodes {
            let mut sorted_cycle = cycle.clone();
            sorted_cycle.sort();
            let dedup_key = sorted_cycle.join("->");
            if reported.insert(dedup_key) {
                for i in 0..cycle.len() {
                    let next = cycle[(i + 1) % cycle.len()].clone();
                    unique_cycles.push(format!("{}->{}", cycle[i], next));
                }
            }
        }
    }

    unique_cycles.into_iter().map(SymbolName::new).collect()
}

fn dfs_3color(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    color: &mut HashMap<String, GraphColorVO>,
    parent: &mut HashMap<String, String>,
    cycle_edges: &mut HashSet<(String, String)>,
) {
    color.insert(node.to_string(), GraphColorVO::Gray);

    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            if *color.get(neighbor).unwrap_or(&GraphColorVO::White) == GraphColorVO::Gray {
                cycle_edges.insert((node.to_string(), neighbor.clone()));
            } else if *color.get(neighbor).unwrap_or(&GraphColorVO::White) == GraphColorVO::White {
                parent.insert(neighbor.clone(), node.to_string());
                dfs_3color(neighbor, graph, color, parent, cycle_edges);
            }
        }
    }

    color.insert(node.to_string(), GraphColorVO::Black);
}

fn extract_cycle_nodes(
    src: &str,
    tgt: &str,
    parent: &HashMap<String, String>,
) -> Option<Vec<String>> {
    let mut path = Vec::new();
    let mut cur = src;
    path.push(cur.to_string());

    while cur != tgt {
        let p = parent.get(cur)?;
        cur = p;
        path.push(cur.to_string());
    }

    path.reverse();
    Some(path)
}
```

---

## File: crates/shared/src/import-rules/utility_dummy_detector.rs

```rust
// PURPOSE: utility_dummy_helper — pure utility functions for dummy function, block, and trait detection
use crate::common::taxonomy_common_vo::LanguageVO;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_name_vo::SymbolName;

pub fn dummy_function_ranges(lines: &[&str], lang: LanguageVO) -> Vec<(LineNumber, LineNumber)> {
    match lang {
        LanguageVO::Rust => rust_dummy_function_ranges(lines),
        LanguageVO::Python => python_dummy_function_ranges(lines),
        LanguageVO::JavaScript => js_dummy_function_ranges(lines),
        LanguageVO::Unknown => Vec::new(),
    }
}

pub fn imported_symbols(lines: &[&str], lang: LanguageVO) -> Vec<(SymbolName, LineNumber)> {
    match lang {
        LanguageVO::Rust => rust_imported_symbols(lines),
        LanguageVO::Python => python_imported_symbols(lines),
        LanguageVO::JavaScript => js_imported_symbols(lines),
        LanguageVO::Unknown => Vec::new(),
    }
}

pub fn dummy_impl_traits_with_lines(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut traits = Vec::new();
    let mut i = 0usize;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("impl ") && trimmed.contains(" for ") {
            if let Some(trait_name) = impl_trait_name(trimmed) {
                let (end, body_lines) = impl_block(lines, i);
                if trait_impl_is_dummy(&body_lines) {
                    traits.push((SymbolName::new(trait_name), LineNumber::new(i as i64 + 1)));
                }
                i = end;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    traits
}

pub fn symbol_used_real(
    lines: &[&str],
    symbol: &str,
    dummy_ranges: &[(LineNumber, LineNumber)],
    dummy_impl_traits: &[String],
) -> bool {
    let dummy_ranges_usize: Vec<(usize, usize)> = dummy_ranges
        .iter()
        .map(|(a, b)| (a.value() as usize, b.value() as usize))
        .collect();
    if (symbol.starts_with('I')
        && symbol.len() > 1
        && matches!(symbol.chars().nth(1), Some(c) if c.is_uppercase()))
        || symbol.ends_with("Protocol")
        || symbol.ends_with("Trait")
        || symbol.ends_with("Aggregate")
        || symbol.ends_with("Ext")
        || symbol == "Default"
        || symbol == "Debug"
        || symbol == "Display"
        || symbol == "Clone"
        || symbol == "Copy"
        || symbol == "From"
        || symbol == "Into"
        || symbol == "TryFrom"
        || symbol == "TryInto"
        || symbol == "AsRef"
        || symbol == "AsMut"
        || symbol == "Deref"
        || symbol == "DerefMut"
        || symbol == "Iterator"
        || symbol == "IntoIterator"
        || symbol == "ParallelIterator"
        || symbol == "IntoParallelRefIterator"
        || symbol == "IntoParallelIterator"
        || symbol == "Future"
        || symbol == "Stream"
        || symbol == "Read"
        || symbol == "Write"
        || symbol == "BufRead"
        || symbol == "Seek"
        || symbol == "Hash"
        || symbol == "PartialEq"
        || symbol == "Eq"
        || symbol == "PartialOrd"
        || symbol == "Ord"
        || symbol == "Send"
        || symbol == "Sync"
        || symbol == "Unpin"
        || symbol == "Sized"
        || symbol == "Drop"
        || symbol == "Fn"
        || symbol == "FnMut"
        || symbol == "FnOnce"
        || symbol == "async_trait"
        || symbol == "Parser"
        || symbol == "Digest"
        || symbol == "Manager"
        || symbol == "Emitter"
        || symbol == "Serialize"
        || symbol == "Deserialize"
    {
        return true;
    }

    for (idx, line) in lines.iter().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim();

        if in_dummy_range(line_no, &dummy_ranges_usize)
            || trimmed.starts_with("use ")
            || trimmed.starts_with("import ")
            || trimmed.starts_with("from ")
            || trimmed.starts_with("//")
            || trimmed.starts_with("/*")
            || trimmed.starts_with("*")
            || trimmed.starts_with("*/")
            || (trimmed.starts_with("#") && !trimmed.starts_with("#["))
            || trimmed.contains("PhantomData")
        {
            continue;
        }

        if !contains_ident(trimmed, symbol) {
            continue;
        }

        // If the symbol only appears inside string literals, it's not real usage
        if is_symbol_only_in_strings(trimmed, symbol) {
            continue;
        }

        if trimmed.starts_with("impl ") && trimmed.contains(" for ") {
            if let Some(trait_name) = impl_trait_name(trimmed) {
                if dummy_impl_traits.contains(&trait_name) {
                    continue;
                }
            }
        }

        return true;
    }

    false
}

// ─── Private Helpers ───

/// Check if `haystack` contains `needle` as a whole identifier (not a substring).
pub fn contains_ident(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return false;
    }

    let mut start = 0usize;

    while let Some(pos) = haystack[start..].find(needle) {
        let abs = start + pos;
        let end = abs + needle.len();

        let before_ok = abs == 0 || {
            let before_char = haystack[..abs].chars().next_back().unwrap_or(' ');
            !before_char.is_alphanumeric() && before_char != '_'
        };

        let after_ok = end == haystack.len() || {
            let after_char = haystack[end..].chars().next().unwrap_or(' ');
            !after_char.is_alphanumeric() && after_char != '_'
        };

        if before_ok && after_ok {
            return true;
        }

        start = abs + needle.len();
    }

    false
}

/// Check if all occurrences of `needle` in `haystack` appear strictly inside
/// double-quoted string literals. Returns true when the symbol is never used
/// as a code identifier (only inside strings, comments, or doc lines).
pub fn is_symbol_only_in_strings(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() || !haystack.contains(needle) {
        return false;
    }

    let mut start = 0usize;
    let mut found_anywhere = false;

    while let Some(pos) = haystack[start..].find(needle) {
        let abs = start + pos;

        // Determine which string literal (if any) this occurrence falls inside.
        // We scan backwards from `abs` to find the opening quote that is not
        // escaped and not inside a comment.
        let opening_quote = match find_enclosing_string_start(haystack, abs) {
            Some(q) => q,
            None => return false, // not inside a string — real usage
        };

        // Verify the quote is an actual string delimiter (not escaped)
        let before_quote = &haystack[..opening_quote];
        let backslash_count = before_quote
            .chars()
            .rev()
            .take_while(|c| *c == '\\')
            .count();
        if backslash_count % 2 != 0 {
            // Quote is escaped — treat as real usage
            return false;
        }

        // Check if this is a comment line (starts with //, #, or /*)
        let line_start = find_line_start(haystack, abs);
        let line_prefix = &haystack[line_start..abs];
        if line_prefix.trim().starts_with("//")
            || line_prefix.trim().starts_with("///")
            || line_prefix.trim().starts_with("#")
            || line_prefix.trim().ends_with("/*")
        {
            // Inside a comment — skip, not real usage
            start = abs + needle.len();
            found_anywhere = true;
            continue;
        }

        // It's inside a string literal — skip, not real usage
        start = abs + needle.len();
        found_anywhere = true;
    }

    // If we never found the symbol outside strings/comments, return true
    found_anywhere
}

/// Find the position of the opening double-quote that encloses the character
/// at `pos`. Returns None if `pos` is not inside a string literal.
fn find_enclosing_string_start(haystack: &str, pos: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut in_string = false;
    let mut string_start = 0usize;
    let mut i = 0usize;

    while i <= pos {
        if i >= haystack.len() {
            break;
        }
        let ch = haystack[i..].chars().next()?;
        let ch_len = ch.len_utf8();

        if in_string {
            if ch == '"' && i > string_start {
                // Check not escaped
                let before = &haystack[..i];
                let bs = before.chars().rev().take_while(|c| *c == '\\').count();
                if bs % 2 == 0 {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        in_string = false;
                    }
                }
            }
        } else if ch == '"' {
            // Check not escaped
            let before = &haystack[..i];
            let bs = before.chars().rev().take_while(|c| *c == '\\').count();
            if bs % 2 == 0 {
                in_string = true;
                string_start = i;
                depth += 1;
            }
        }

        if in_string && i == pos {
            return Some(string_start);
        }

        i += ch_len;
    }

    None
}

/// Find the start of the line containing position `pos`.
fn find_line_start(haystack: &str, pos: usize) -> usize {
    haystack[..pos].rfind('\n').map(|n| n + 1).unwrap_or(0)
}

/// Iterate `lines`, invoking `is_header(trimmed_line)` to identify function
/// definitions and `body_extent(start, lines)` to compute the body end line
/// for that definition. Returns `[(start_line, end_line), ...]` of all ranges.
///
/// The two language-specific differences (Rust/JS brace-counting vs. Python
/// indent-based termination) live in the closures passed in.
fn collect_ranges<F, G>(
    lines: &[&str],
    is_header: F,
    body_extent: G,
) -> Vec<(LineNumber, LineNumber)>
where
    F: Fn(&str) -> bool,
    G: Fn(usize, &[&str]) -> usize,
{
    let mut ranges = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if is_header(lines[i].trim()) {
            let start = i + 1;
            let end = body_extent(i, lines);
            ranges.push((LineNumber::new(start as i64), LineNumber::new(end as i64)));
            i = end;
        }
        i += 1;
    }
    ranges
}

/// Brace-counting body extenter for Rust/JS-like brace-delimited languages.
fn brace_extent(start: usize, lines: &[&str]) -> usize {
    let mut depth = 0usize;
    let mut end = start + 1;
    for (idx, line) in lines.iter().enumerate().skip(start) {
        let t = line.trim();
        depth = depth.saturating_add(t.matches('{').count());
        depth = depth.saturating_sub(t.matches('}').count());
        end = idx + 1;
        if depth == 0 && t.contains('}') {
            break;
        }
    }
    end
}

/// Indent-based body extenter for Python. Returns the line *after* the
/// `def` block ends (the next non-empty, non-comment line at the same or
/// shallower indent).
fn indent_extent(start: usize, lines: &[&str]) -> usize {
    let mut end = start + 1;
    let indent = lines[start].len() - lines[start].trim_start().len();
    for (idx, line) in lines.iter().enumerate().skip(start + 1) {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            end = idx + 1;
            continue;
        }
        let line_indent = line.len() - line.trim_start().len();
        if line_indent <= indent && !t.is_empty() {
            break;
        }
        end = idx + 1;
    }
    end
}

fn rust_dummy_function_ranges(lines: &[&str]) -> Vec<(LineNumber, LineNumber)> {
    collect_ranges(
        lines,
        |t| t.starts_with("fn _use_") || t.starts_with("fn dummy_"),
        brace_extent,
    )
}

fn python_dummy_function_ranges(lines: &[&str]) -> Vec<(LineNumber, LineNumber)> {
    collect_ranges(
        lines,
        |t| t.starts_with("def _use_") || t.starts_with("def dummy_"),
        indent_extent,
    )
}

fn js_dummy_function_ranges(lines: &[&str]) -> Vec<(LineNumber, LineNumber)> {
    collect_ranges(
        lines,
        |t| {
            t.starts_with("function _use")
                || t.starts_with("function dummy")
                || t.starts_with("const _use")
                || t.starts_with("const dummy")
        },
        brace_extent,
    )
}

fn rust_imported_symbols(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if !trimmed.starts_with("use ") || !trimmed.ends_with(';') {
            continue;
        }

        if trimmed == "use super::*;" {
            continue;
        }

        let body = trimmed
            .trim_start_matches("use ")
            .trim_end_matches(';')
            .trim();

        if body.contains('{') {
            if let Some(open) = body.find('{') {
                if let Some(close) = body.rfind('}') {
                    let inside = &body[open + 1..close];
                    for part in inside.split(',') {
                        if let Some(symbol) = rust_imported_symbol_from_part(part.trim()) {
                            symbols
                                .push((SymbolName::new(symbol), LineNumber::new(idx as i64 + 1)));
                        }
                    }
                }
            }
            continue;
        }

        if let Some(symbol) = rust_imported_symbol_from_part(body) {
            symbols.push((SymbolName::new(symbol), LineNumber::new(idx as i64 + 1)));
        }
    }

    symbols
}

fn rust_imported_symbol_from_part(part: &str) -> Option<String> {
    let part = part.trim();
    if part.is_empty() || part == "self" || part.starts_with('*') {
        return None;
    }

    if let Some((_, alias)) = part.split_once(" as ") {
        return Some(alias.trim().to_string());
    }

    let name = match part.split("::").last() {
        Some(n) => n.trim(),
        None => part.trim(),
    };
    if name.is_empty() || name.contains('{') || name.contains('}') {
        return None;
    }

    Some(name.to_string())
}

pub fn python_imported_symbols(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if let Some(import_part) = trimmed
            .strip_prefix("from ")
            .and_then(|s| s.split_once(" import ").map(|(_, p)| p))
        {
            for name in import_part.split(',') {
                let name = name.trim();
                if name.is_empty() || name == "*" {
                    continue;
                }

                let used_name = match name.split_once(" as ") {
                    Some((_, alias)) => alias.trim(),
                    None => name.split_whitespace().next().unwrap_or_default(),
                };

                if !used_name.is_empty() && used_name != "*" {
                    symbols.push((SymbolName::new(used_name), LineNumber::new(idx as i64 + 1)));
                }
            }
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("import ") {
            for module in rest.split(',') {
                let module = module.trim();
                if module.is_empty() {
                    continue;
                }

                let used_name = match module.split_once(" as ") {
                    Some((_, alias)) => alias.trim(),
                    None => module.rsplit('.').next().unwrap_or(module).trim(),
                };

                if !used_name.is_empty() && used_name != "*" {
                    symbols.push((SymbolName::new(used_name), LineNumber::new(idx as i64 + 1)));
                }
            }
        }
    }

    symbols
}

pub fn js_imported_symbols(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("import ") && trimmed.contains('{') && trimmed.contains("from") {
            if let Some(open) = trimmed.find('{') {
                if let Some(close) = trimmed.find('}') {
                    let inside = &trimmed[open + 1..close];
                    for part in inside.split(',') {
                        let part = part.trim();
                        if part.is_empty() {
                            continue;
                        }

                        let name = match part.split_once(" as ") {
                            Some((_, alias)) => alias.trim(),
                            None => part.split_whitespace().next().unwrap_or_default(),
                        };

                        if !name.is_empty() && name != "type" {
                            symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
                        }
                    }
                }
            }
            continue;
        }

        if trimmed.starts_with("import ") && trimmed.contains(" from ") {
            if let Some(import_part) = trimmed.split_once("import ").map(|(_, p)| p) {
                let before_from = import_part
                    .split_once(" from ")
                    .map(|(n, _)| n)
                    .unwrap_or_default()
                    .trim();

                let name = match before_from.split_once(" as ") {
                    Some((_, alias)) => alias.trim(),
                    None => before_from,
                };

                if !name.is_empty() && name != "default" {
                    symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
                }
            }
            continue;
        }

        if trimmed.starts_with("const ") && trimmed.contains("require(") && trimmed.contains('{') {
            if let Some(open) = trimmed.find('{') {
                if let Some(close) = trimmed.find('}') {
                    let inside = &trimmed[open + 1..close];
                    for part in inside.split(',') {
                        let part = part.trim();
                        if part.is_empty() {
                            continue;
                        }

                        let name = match part.split_once(':') {
                            Some((_, alias)) => alias.trim(),
                            None => part,
                        };

                        if !name.is_empty() {
                            symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
                        }
                    }
                }
            }
        }
    }

    symbols
}

fn in_dummy_range(line_no: usize, ranges: &[(usize, usize)]) -> bool {
    ranges
        .iter()
        .any(|(start, end)| line_no >= *start && line_no <= *end)
}

fn impl_trait_name(line: &str) -> Option<String> {
    let after_impl = line.strip_prefix("impl ")?.trim();
    let (trait_part, _) = after_impl.split_once(" for ")?;
    let trait_name = match trait_part.split("::").last() {
        Some(n) => n.trim(),
        None => trait_part.trim(),
    };
    if trait_name.is_empty() {
        return None;
    }
    Some(trait_name.to_string())
}

fn impl_block<'a>(lines: &'a [&'a str], start: usize) -> (usize, Vec<&'a str>) {
    let mut depth = 0usize;
    let mut body = Vec::new();
    let mut end = start;

    for (idx, line) in lines.iter().enumerate().skip(start) {
        let trimmed = line.trim();
        depth = depth.saturating_add(trimmed.matches('{').count());
        depth = depth.saturating_sub(trimmed.matches('}').count());
        body.push(*line);
        end = idx;
        if depth == 0 && trimmed.contains('}') {
            break;
        }
    }

    (end + 1, body)
}

fn trait_impl_is_dummy(lines: &[&str]) -> bool {
    let mut method_count = 0usize;
    let mut dummy_count = 0usize;
    let mut i = 0usize;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("fn ") || trimmed.starts_with("async fn ") {
            method_count += 1;
            let (end, body) = function_body(lines, i);
            if function_body_is_dummy(&body) {
                dummy_count += 1;
            }
            i = end;
        } else {
            i += 1;
        }
    }

    method_count > 0 && dummy_count == method_count
}

fn function_body<'a>(lines: &'a [&'a str], start: usize) -> (usize, Vec<&'a str>) {
    let mut depth = 0usize;
    let mut body = Vec::new();
    let mut end = start;

    for (idx, line) in lines.iter().enumerate().skip(start) {
        let trimmed = line.trim();
        depth = depth.saturating_add(trimmed.matches('{').count());
        depth = depth.saturating_sub(trimmed.matches('}').count());
        body.push(*line);
        end = idx;
        if depth == 0 && trimmed.contains('}') {
            break;
        }
    }

    (end + 1, body)
}

fn function_body_is_dummy(lines: &[&str]) -> bool {
    // Collect the body lines (skip the fn signature line at index 0)
    let body_lines: Vec<&str> = lines
        .iter()
        .skip(1)
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .collect();

    if body_lines.is_empty() {
        return true;
    }

    // Single-line body like `{ 42 }` or `{ return x; }` — not dummy
    if body_lines.len() == 1 {
        let single = body_lines[0];
        if single.starts_with('{') && single.ends_with('}') {
            let inner = &single[1..single.len() - 1].trim();
            return inner.is_empty() || is_short_marker(inner);
        }
        return is_short_marker(single);
    }

    // Multi-line body: join and check
    let body: String = body_lines.join(" ");
    let trimmed = body.trim();
    if trimmed == "{}" || trimmed == "{ }" {
        return true;
    }

    let inner = trimmed.trim_start_matches('{').trim_end_matches('}').trim();
    if inner.is_empty() || is_short_marker(inner) {
        return true;
    }

    false
}

pub fn is_short_marker(inner: &str) -> bool {
    inner.starts_with("todo!(")
        || inner.starts_with("unimplemented!(")
        || inner.starts_with("panic!(")
        || inner.starts_with("unreachable!(")
}
```

---

## File: crates/shared/src/import-rules/utility_import_module_parser.rs

```rust
// PURPOSE: taxonomy_parser_helper — pure utility functions for import parsing and syntax token extraction
use crate::common::taxonomy_name_vo::SymbolName;

pub fn extract_import_modules(content: &str) -> Vec<SymbolName> {
    let mut modules = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("from ") {
            if let Some(module) = rest.split_whitespace().next() {
                modules.push(SymbolName::new(module));
            }
        } else if trimmed.starts_with("import ") {
            if let Some(pos) = trimmed.rfind(" from ") {
                let module_part = trimmed[pos + 6..].trim();
                let cleaned = module_part
                    .trim_end_matches(';')
                    .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                    .trim();
                modules.push(SymbolName::new(cleaned));
            } else if let Some(rest) = trimmed.strip_prefix("import ") {
                if rest.contains('"') || rest.contains('\'') || rest.contains('`') {
                    let cleaned = rest
                        .trim_end_matches(';')
                        .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                        .trim();
                    modules.push(SymbolName::new(cleaned));
                } else if let Some(first_token) = rest.split_whitespace().next() {
                    modules.push(SymbolName::new(first_token.trim_end_matches(',')));
                }
            }
        } else if let Some(rest) = trimmed.strip_prefix("use ") {
            let module = rest.trim_end_matches(';');
            modules.push(SymbolName::new(module));
        }
    }
    modules
}
```

---

## File: crates/shared/src/import-rules/utility_import_resolver.rs

```rust
// PURPOSE: Import parsing utility functions — stateless, domain-agnostic, multi-consumer
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_layer_vo::{Identity, LayerNameVO, LineContentVO};
use crate::common::taxonomy_path_vo::FilePath;
use crate::import_rules::utility_path_normalizer;

/// Convert a Result<FilePath, _> to FilePath, using default on error.
pub fn filepath_or_default(result: Result<FilePath, impl std::fmt::Debug>) -> FilePath {
    result.unwrap_or_default()
}

/// Convert an optional OsStr reference to a string slice.
pub fn os_str_to_str(opt: Option<&std::ffi::OsStr>) -> &str {
    opt.and_then(|o| o.to_str()).map_or("", |s| s)
}

/// Parse import lines from file content.
/// Handles: use, pub use, pub(crate) use, import, from, extern crate.
/// Skips: #[cfg(...)] conditional blocks (test, feature, etc.).
/// Handles multi-line use statements with braces and trailing commas.
pub fn parse_import_lines_helper(content: &str) -> Vec<(LineNumber, LineContentVO)> {
    let mut result = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    let mut in_cfg_block = false;
    let mut cfg_brace_depth = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();

        // Skip #[cfg(...)] attribute lines and their associated blocks
        if trimmed.starts_with("#[cfg(") {
            in_cfg_block = true;
            cfg_brace_depth = 0;
            i += 1;
            continue;
        }
        if in_cfg_block {
            // Count braces to find the end of the cfg block
            for ch in trimmed.chars() {
                match ch {
                    '{' => cfg_brace_depth += 1,
                    '}' => {
                        cfg_brace_depth -= 1;
                        if cfg_brace_depth <= 0 {
                            in_cfg_block = false;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            i += 1;
            continue;
        }

        if trimmed.starts_with("import ")
            || trimmed.starts_with("from ")
            || trimmed.starts_with("extern crate ")
        {
            result.push((
                LineNumber::new((i + 1) as i64),
                LineContentVO::new(lines[i].to_string()),
            ));
            i += 1;
            continue;
        }
        if trimmed.starts_with("use ")
            || trimmed.starts_with("pub use ")
            || trimmed.starts_with("pub(crate) use ")
        {
            let mut combined = lines[i].to_string();
            if combined.contains('{') && !combined.contains('}') {
                let start = i;
                i += 1;
                while i < lines.len() {
                    let part = lines[i].trim().to_string();
                    combined.push_str(&format!(" {}", part));
                    if part.contains('}') || combined.ends_with(';') {
                        break;
                    }
                    i += 1;
                }
                // Normalize whitespace and clean trailing commas before closing brace
                combined = combined.split_whitespace().collect::<Vec<&str>>().join(" ");
                combined = clean_trailing_commas(&combined);
                result.push((
                    LineNumber::new((start + 1) as i64),
                    LineContentVO::new(combined),
                ));
            } else if !combined.ends_with(';') {
                while i + 1 < lines.len() {
                    let next = lines[i + 1].trim();
                    if next.starts_with("use ")
                        || next.starts_with("pub use ")
                        || next.starts_with("pub(crate) use ")
                        || next.starts_with("#[cfg(")
                        || next.is_empty()
                    {
                        break;
                    }
                    combined.push_str(&format!(" {}", next));
                    if next.ends_with(';') {
                        i += 1;
                        break;
                    }
                    i += 1;
                }
                combined = combined.split_whitespace().collect::<Vec<&str>>().join(" ");
                combined = clean_trailing_commas(&combined);
                result.push((
                    LineNumber::new((i + 1) as i64),
                    LineContentVO::new(combined),
                ));
            } else {
                result.push((
                    LineNumber::new((i + 1) as i64),
                    LineContentVO::new(combined),
                ));
            }
        }
        i += 1;
    }
    result
}

/// Clean trailing commas before closing braces in use statements.
/// e.g., `use foo::{A, B,}` → `use foo::{A, B}`
fn clean_trailing_commas(s: &str) -> String {
    if let Some(brace_pos) = s.rfind('{') {
        let after_brace = &s[brace_pos..];
        if after_brace.ends_with("}") || after_brace.ends_with("};") {
            // Find the last non-whitespace char before }
            let bytes = after_brace.as_bytes();
            let mut end = bytes.len();
            // Skip trailing } and ;
            while end > 0 && (bytes[end - 1] == b'}' || bytes[end - 1] == b';') {
                end -= 1;
            }
            let inner = &after_brace[..end];
            if let Some(trimmed_inner) = inner.strip_suffix(',') {
                let before = &s[..brace_pos];
                let after = &after_brace[end..];
                return format!("{before}{trimmed_inner}{after}");
            }
        }
    }
    s.to_string()
}

/// Parse a scope value (e.g. "contract(protocol)", "taxonomy(entity,error,event)")
/// into layer + suffix matches. Returns (LayerNameVO, Vec<Identity>).
pub fn resolve_scope(scope: &Identity) -> (LayerNameVO, Vec<Identity>) {
    let scope_str = scope.value();
    if let Some(paren) = scope_str.find('(') {
        let layer = scope_str[..paren].trim();
        let inner = scope_str[paren + 1..].trim_end_matches(')').trim();
        let suffixes: Vec<Identity> = if inner.contains('|') {
            inner
                .split('|')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(Identity::new)
                .collect()
        } else {
            inner
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(Identity::new)
                .collect()
        };
        (LayerNameVO::new(layer), suffixes)
    } else {
        (LayerNameVO::new(scope_str.trim()), vec![])
    }
}

/// Check if an import line satisfies the given scope requirement.
pub fn import_matches_scope(
    import_line: &LineContentVO,
    layer: &LayerNameVO,
    suffixes: &[Identity],
) -> bool {
    let import_line_str = import_line.value();
    let segments: Vec<&str> = import_line_str
        .split(|c: char| {
            c == ':'
                || c == '.'
                || c == '/'
                || c == '\\'
                || c.is_whitespace()
                || c == '"'
                || c == '\''
                || c == '{'
                || c == '}'
                || c == ','
                || c == ';'
        })
        .filter(|s| !s.is_empty())
        .collect();
    let layer_lower = layer.value().to_lowercase();
    let layer_prefix = format!("{}_", layer_lower);
    let layer_match = segments.iter().any(|s| {
        let trimmed = s.trim().to_lowercase();
        trimmed == layer_lower || trimmed.starts_with(&layer_prefix)
    });
    if !layer_match || suffixes.is_empty() {
        return layer_match;
    }
    suffixes.iter().any(|s| {
        let s_val = s.value();
        segments.iter().any(|seg| {
            let cleaned = seg
                .trim_end_matches(';')
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .trim();
            cleaned.split(',').any(|t| {
                let name = t.trim();
                let name_lower = name.to_lowercase();
                if name_lower.ends_with(&format!("_{}", s_val)) {
                    return true;
                }
                if let Some(rest) = name_lower.strip_suffix(s_val) {
                    if rest.is_empty() || rest.ends_with('_') {
                        return true;
                    }
                    if name.len() >= s_val.len() {
                        let suffix_in_orig = &name[name.len() - s_val.len()..];
                        if suffix_in_orig.starts_with(|c: char| c.is_uppercase()) {
                            return true;
                        }
                    }
                }
                false
            })
        })
    })
}

/// Extract the module path from an import line.
pub fn extract_module_from_line(line: &LineContentVO) -> Option<Identity> {
    let trimmed = line.value().trim();
    if let Some(rest) = trimmed.strip_prefix("from ") {
        return Some(Identity::new(rest.split_whitespace().next()?.to_string()));
    }
    if trimmed.starts_with("import ") {
        if let Some(pos) = trimmed.rfind(" from ") {
            let module_part = trimmed[pos + 6..].trim();
            let cleaned = module_part
                .trim_end_matches(';')
                .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                .trim();
            return Some(Identity::new(cleaned.to_string()));
        }
        if let Some(rest) = trimmed.strip_prefix("import ") {
            if rest.contains('"') || rest.contains('\'') || rest.contains('`') {
                let cleaned = rest
                    .trim_end_matches(';')
                    .trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ';')
                    .trim();
                return Some(Identity::new(cleaned.to_string()));
            }
            let first_token = rest.split_whitespace().next()?;
            return Some(Identity::new(first_token.to_string()));
        }
    }
    if let Some(rest) = trimmed
        .strip_prefix("pub(crate) use ")
        .or_else(|| trimmed.strip_prefix("pub use "))
        .or_else(|| trimmed.strip_prefix("use "))
    {
        let module = rest.trim_end_matches(';').trim().to_string();
        if let Some(brace_pos) = module.find("::{") {
            return Some(Identity::new(module[..brace_pos].to_string()));
        }
        return Some(Identity::new(module));
    }
    None
}

/// Extract layer name from an import segment.
pub fn extract_layer_from_import(segment: &Identity) -> Option<LayerNameVO> {
    let segment_str = segment.value();
    if let Some(layer) = utility_path_normalizer::extract_layer_from_prefix(segment_str) {
        return Some(LayerNameVO::new(layer));
    }
    match segment_str {
        "taxonomy" => Some(LayerNameVO::new("taxonomy")),
        "contract" => Some(LayerNameVO::new("contract")),
        "capabilities" => Some(LayerNameVO::new("capabilities")),
        "utility" => Some(LayerNameVO::new("utility")),
        "agent" => Some(LayerNameVO::new("agent")),
        "surfaces" | "surface" => Some(LayerNameVO::new("surfaces")),
        "root" => Some(LayerNameVO::new("root")),
        _ => None,
    }
}

/// Find the line number of an import statement containing the given alias.
pub fn find_import_line_number(content: &str, alias: &str) -> LineNumber {
    let first_part = alias.split('.').next().unwrap_or("");
    let pos_opt = content.lines().position(|l| {
        l.trim().contains(&format!("import {}", alias))
            || l.trim().contains(&format!("from {} import", first_part))
    });
    let line = match pos_opt {
        Some(p) => p + 1,
        None => 1,
    };
    LineNumber::new(line as i64)
}
```

---

## File: crates/shared/src/import-rules/utility_import_symbol_extractor.rs

```rust
// PURPOSE: taxonomy_unused_helper — pure utility functions for unused import detection
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_name_vo::SymbolName;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};

static ALL_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r#"__all__\s*=\s*\[([^\]]*)\]"#).ok());

// Known derive-macro imports that Rust compiler consumes implicitly.
// These are never "used" as ordinary symbols — they're consumed by #[derive(...)]
// attributes, so they must never be flagged as unused.
const DERIVE_MACROS: &[&str] = &[
    "async_trait",
    "Serialize",
    "Deserialize",
    "Clone",
    "Debug",
    "Default",
    "PartialEq",
    "Eq",
    "Hash",
    "Ord",
    "PartialOrd",
    "Copy",
    "EnumIter",
    "Display",
    "EnumString",
    "AsRefStr",
    "IntoParallelRefIterator",
    "ParallelIterator",
    "IntoParallelIterator",
];

fn is_rust_trait_import(name: &str) -> bool {
    if name.starts_with('I') && name.len() > 1 && name.chars().nth(1).unwrap_or(' ').is_uppercase()
    {
        return true;
    }
    if name.ends_with("Protocol")
        || name.ends_with("Trait")
        || name.ends_with("Aggregate")
        || name.ends_with("Ext")
    {
        return true;
    }
    matches!(
        name,
        "Default"
            | "Debug"
            | "Display"
            | "Clone"
            | "Copy"
            | "PartialEq"
            | "Eq"
            | "PartialOrd"
            | "Ord"
            | "Hash"
            | "From"
            | "Into"
            | "TryFrom"
            | "TryInto"
            | "AsRef"
            | "AsMut"
            | "Deref"
            | "DerefMut"
            | "Iterator"
            | "IntoIterator"
            | "ParallelIterator"
            | "IntoParallelRefIterator"
            | "IntoParallelIterator"
            | "ExactSizeIterator"
            | "FusedIterator"
            | "Future"
            | "Stream"
            | "Read"
            | "Write"
            | "BufRead"
            | "Seek"
            | "Send"
            | "Sync"
            | "Unpin"
            | "Sized"
            | "Drop"
            | "Fn"
            | "FnMut"
            | "FnOnce"
            | "async_trait"
            | "Digest"
            | "Manager"
            | "Emitter"
            | "Serialize"
            | "Deserialize"
            | "EnumIter"
            | "EnumString"
            | "AsRefStr"
            | "Parser"
    )
}

pub fn extract_imported_aliases(content: &str) -> HashMap<Identity, Identity> {
    let mut aliases = HashMap::new();
    let mut in_cfg_block = false;
    let mut cfg_brace_depth = 0;
    for line in content.lines() {
        let trimmed = line.trim();

        // Skip any #[cfg(...)] attribute and its associated block
        // This covers #[cfg(test)], #[cfg(feature = "...")], #[cfg(not(...))], etc.
        if trimmed.starts_with("#[cfg(") {
            in_cfg_block = true;
            cfg_brace_depth = 0;
            continue;
        }
        if in_cfg_block {
            for ch in trimmed.chars() {
                match ch {
                    '{' => cfg_brace_depth += 1,
                    '}' => {
                        cfg_brace_depth -= 1;
                        if cfg_brace_depth <= 0 {
                            in_cfg_block = false;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            continue;
        }

        // Python: `from X import Y, Z` or `from X import Y as Z`
        if trimmed.starts_with("from ") && trimmed.contains(" import ") {
            if let Some((from_part, import_part)) = trimmed.split_once(" import ") {
                let module = from_part[5..].trim();
                for name in import_part.split(',') {
                    let name = name.trim();
                    if name.is_empty() || name == "*" {
                        continue;
                    }
                    if let Some((sym, alias)) = name.split_once(" as ") {
                        aliases.insert(
                            Identity::new(alias.trim()),
                            Identity::new(format!("{}.{}", module, sym.trim())),
                        );
                    } else {
                        aliases.insert(
                            Identity::new(name),
                            Identity::new(format!("{}.{}", module, name)),
                        );
                    }
                }
            }
            continue;
        }

        // Python: `import X` or `import X as Y` (without `from`)
        if let Some(import_part) = trimmed.strip_prefix("import ") {
            // Skip if this is actually a `from ... import ...` (already handled above)
            if !trimmed.starts_with("from ") {
                for name in import_part.split(',') {
                    let name = name.trim().trim_end_matches(';');
                    if name.is_empty() || name == "*" {
                        continue;
                    }
                    if let Some((sym, alias)) = name.split_once(" as ") {
                        aliases.insert(Identity::new(alias.trim()), Identity::new(sym.trim()));
                    } else {
                        let alias = name.rsplit('.').next().unwrap_or(name);
                        aliases.insert(Identity::new(alias), Identity::new(name));
                    }
                }
            }
            continue;
        }

        // Rust `use` statements: `use std::collections::HashMap;` or `use serde::{A, B};`
        if let Some(use_part) = trimmed.strip_prefix("use ") {
            let use_part = use_part.trim_end_matches(';').trim();
            if !use_part.is_empty()
                && !use_part.starts_with("crate::")
                && !use_part.starts_with("super::")
                && !use_part.starts_with("self::")
            {
                if let Some(brace_pos) = use_part.find("::{") {
                    let prefix = &use_part[..brace_pos];
                    let inner = use_part[brace_pos + 3..].trim_end_matches('}');
                    for name in inner.split(',') {
                        let name = name.trim().split(" as ").last().unwrap_or("").trim();
                        if !name.is_empty()
                            && name != "_"
                            && name != "*"
                            && !is_rust_trait_import(name)
                        {
                            aliases.insert(
                                Identity::new(name),
                                Identity::new(format!("{}::{}", prefix, name)),
                            );
                        }
                    }
                } else {
                    let raw_name = use_part.rsplit("::").next().unwrap_or(use_part);
                    let name = raw_name.split(" as ").last().unwrap_or(raw_name).trim();
                    if !name.is_empty() && name != "*" && !is_rust_trait_import(name) {
                        aliases.insert(Identity::new(name), Identity::new(use_part));
                    }
                }
            }
            continue;
        }

        // JavaScript: `const X = require('Y')` or `const { A, B } = require('Y')`
        if let Some(rest) = trimmed.strip_prefix("const ") {
            if let Some(eq_pos) = rest.find('=') {
                let left = rest[..eq_pos].trim();
                let right = rest[eq_pos + 1..].trim();
                if let Some(req_start) = right.find("require(") {
                    let after_require = &right[req_start + 8..];
                    if let Some(paren_end) = after_require.find(')') {
                        let _module = after_require[..paren_end]
                            .trim_matches(|c| c == '\'' || c == '"' || c == '`');
                        // Handle destructured require: `const { A, B } = require('Y')`
                        if left.starts_with('{') && left.ends_with('}') {
                            let inner = &left[1..left.len() - 1];
                            for name in inner.split(',') {
                                let name = name.trim();
                                if name.is_empty() || name == "*" {
                                    continue;
                                }
                                if let Some((sym, alias)) = name.split_once(" as ") {
                                    aliases.insert(
                                        Identity::new(alias.trim()),
                                        Identity::new(sym.trim()),
                                    );
                                } else {
                                    aliases.insert(Identity::new(name), Identity::new(name));
                                }
                            }
                        } else {
                            // Simple require: `const X = require('Y')`
                            aliases.insert(Identity::new(left), Identity::new(left));
                        }
                    }
                }
            }
        }
    }
    aliases
}

pub fn extract_exported_symbols(content: &str) -> HashSet<Identity> {
    let mut exported = HashSet::new();
    let code_lines = content
        .lines()
        .filter(|l| !l.trim().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");

    if let Some(ref re) = *ALL_RE {
        if let Some(caps) = re.captures(&code_lines) {
            if let Some(matched) = caps.get(1) {
                for item in matched.as_str().split(',') {
                    let item = item.trim().trim_matches(|c| c == '\'' || c == '"');
                    if !item.is_empty() {
                        exported.insert(Identity::new(item));
                    }
                }
            }
        }
    }
    exported
}

pub fn extract_used_symbols(
    content: &str,
    imported_aliases: &HashMap<Identity, Identity>,
) -> HashSet<Identity> {
    let mut used = HashSet::new();
    let code_lines = content
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.starts_with("import ")
                && !t.starts_with("from ")
                && !t.starts_with("use ")
                && !t.starts_with("pub use ")
                && !t.starts_with("pub(crate) use ")
        })
        .collect::<Vec<_>>()
        .join("\n");

    for alias in imported_aliases.keys() {
        let alias_str = alias.value();
        if DERIVE_MACROS.contains(&alias_str) {
            used.insert(Identity::new(alias_str));
        }
    }

    let non_derive_aliases: Vec<&str> = imported_aliases
        .keys()
        .map(|a| a.value())
        .filter(|a| !DERIVE_MACROS.contains(a))
        .collect();

    if !non_derive_aliases.is_empty() && !code_lines.is_empty() {
        let patterns: Vec<String> = non_derive_aliases
            .iter()
            .map(|a| regex::escape(a))
            .collect();
        let combined = format!(r"\b({})\b", patterns.join("|"));
        if let Ok(re) = Regex::new(&combined) {
            let matched_set: HashSet<&str> =
                re.find_iter(&code_lines).map(|m| m.as_str()).collect();
            for alias in non_derive_aliases {
                if matched_set.contains(alias) {
                    used.insert(Identity::new(alias));
                }
            }
        }
    }

    used
}

pub fn extract_rust_js_imports(content: &str) -> Vec<(SymbolName, LineNumber)> {
    let mut imports = Vec::new();
    let mut in_cfg_block = false;
    let mut cfg_brace_depth = 0;
    for (i, line) in content.lines().enumerate() {
        let t = line.trim();
        // Skip any #[cfg(...)] attribute and its associated block
        if t.starts_with("#[cfg(") {
            in_cfg_block = true;
            cfg_brace_depth = 0;
            continue;
        }
        if in_cfg_block {
            for ch in t.chars() {
                match ch {
                    '{' => cfg_brace_depth += 1,
                    '}' => {
                        cfg_brace_depth -= 1;
                        if cfg_brace_depth <= 0 {
                            in_cfg_block = false;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            continue;
        }

        let names: Vec<SymbolName> = if t.starts_with("use ")
            || t.starts_with("pub use ")
            || t.starts_with("pub(crate) use ")
        {
            let target = t.trim_end_matches(';').trim_start_matches("use ").trim();
            if target.starts_with("std::")
                || target.starts_with("core::")
                || target.starts_with("alloc::")
            {
                continue;
            }
            if let Some(brace_pos) = target.find("::{") {
                let inner = target[brace_pos + 3..].trim_end_matches('}').trim();
                inner
                    .split(',')
                    .map(|s| {
                        s.trim()
                            .split(" as ")
                            .last()
                            .unwrap_or("")
                            .trim()
                            .to_string()
                    })
                    .filter(|n| !n.is_empty() && n != "_" && n != "*")
                    .map(SymbolName::new)
                    .collect()
            } else {
                let name = target
                    .split("::")
                    .last()
                    .unwrap_or("")
                    .split(" as ")
                    .last()
                    .unwrap_or("")
                    .trim()
                    .to_string();
                if name.is_empty() || name == "_" || name == "*" {
                    continue;
                }
                vec![SymbolName::new(name)]
            }
        } else if t.starts_with("import ") {
            if let Some(from_idx) = t.find(" from ") {
                let import_part = t[7..from_idx].trim();
                let names: Vec<SymbolName> = if import_part.starts_with('{') {
                    import_part[1..import_part.len() - 1]
                        .split(',')
                        .map(|s| {
                            s.trim()
                                .split(" as ")
                                .last()
                                .unwrap_or("")
                                .trim()
                                .to_string()
                        })
                        .filter(|n| !n.is_empty())
                        .map(SymbolName::new)
                        .collect()
                } else {
                    vec![SymbolName::new(import_part.trim())]
                };
                names
            } else {
                continue;
            }
        } else {
            continue;
        };

        for name in names {
            let s = name.value();
            if (s.starts_with('I') && s.len() > 1 && s.chars().nth(1).unwrap_or(' ').is_uppercase())
                || s.ends_with("Protocol")
                || s.ends_with("Trait")
                || s.ends_with("Aggregate")
                || s == "Parser"
            {
                continue;
            }
            imports.push((name, LineNumber::new(i as i64 + 1)));
        }
    }
    imports
}

pub fn is_name_used(name: &str, content: &str, exclude_line: usize) -> bool {
    is_name_used_at(name, content, exclude_line)
}

/// `&str` overload for callers that track the exclude line as a 1-based `LineNumber`.
pub fn is_name_used_at(name: &str, content: &str, exclude_line: usize) -> bool {
    if is_rust_trait_import(name) || DERIVE_MACROS.contains(&name) {
        return true;
    }

    let rest = content
        .lines()
        .enumerate()
        .filter(|(j, _)| *j != exclude_line)
        .map(|(_, l)| l)
        .collect::<Vec<_>>()
        .join("\n");
    rest.contains(name)
}
```

---

## File: crates/shared/src/import-rules/utility_path_normalizer.rs

```rust
// PURPOSE: taxonomy_path_helper — pure utility functions for path matching and layer extraction
use std::path::Path;

pub fn extract_layer_from_prefix(filename: &str) -> Option<String> {
    let stem = Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    const PREFIX_MAP: &[(&str, &str)] = &[
        ("taxonomy_", "taxonomy"),
        ("contract_", "contract"),
        ("capabilities_", "capabilities"),
        ("utility_", "utility"),
        ("agent_", "agent"),
        ("surface_", "surfaces"),
        ("root_", "root"),
    ];

    for &(prefix, layer) in PREFIX_MAP {
        if stem.starts_with(prefix) {
            return Some(layer.to_string());
        }
    }

    None
}

pub fn get_relative_path(file_path: &str, root_dir: &str) -> String {
    let normalized_file = match Path::new(file_path)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
    {
        Ok(p) => p,
        Err(_) => file_path.replace('\\', "/"),
    };
    let normalized_root = match Path::new(root_dir)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
    {
        Ok(p) => p,
        Err(_) => root_dir.trim_end_matches('/').replace('\\', "/"),
    };

    let file_path = Path::new(&normalized_file);
    let root_path = Path::new(&normalized_root);

    match file_path.strip_prefix(root_path) {
        Ok(rel) => rel.to_string_lossy().replace('\\', "/"),
        Err(_) => {
            // Fallback: try string-based prefix removal
            // Ensure root ends with / for proper prefix matching
            let root_prefix = if normalized_root.ends_with('/') {
                normalized_root.clone()
            } else {
                format!("{}/", normalized_root)
            };

            if normalized_file.starts_with(&root_prefix) {
                normalized_file[root_prefix.len()..].to_string()
            } else if normalized_file == normalized_root {
                String::new()
            } else {
                normalized_file
            }
        }
    }
}
```

---

