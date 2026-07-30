# Crate: orphan-detector (v1.11.0)

This document contains the source code for feature crate `orphan-detector` along with its corresponding and imported definitions from the `shared` crate.

## Problem Statement

The following issues were detected by `lint-arwaky-cli scan`:

```
Lint Arwaky v1.11.0 — Scan Report
Target: /home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector
Total: 0 violations
Tip: Scan a specific file for detailed violations:
  lint-arwaky-cli scan <file-path>
```

---

## File List

- [ARCHITECTURE.md](file:///home/raka/mcp-arwaky/lint-arwaky/ARCHITECTURE.md)
- [crates/orphan-detector/Cargo.toml](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/Cargo.toml)
- [crates/orphan-detector/FRD.md](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/FRD.md)
- [crates/orphan-detector/src/agent_orphan_orchestrator.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/agent_orphan_orchestrator.rs)
- [crates/orphan-detector/src/capabilities_orphan_agent_analyzer.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/capabilities_orphan_agent_analyzer.rs)
- [crates/orphan-detector/src/capabilities_orphan_capabilities_analyzer.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/capabilities_orphan_capabilities_analyzer.rs)
- [crates/orphan-detector/src/capabilities_orphan_contract_analyzer.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/capabilities_orphan_contract_analyzer.rs)
- [crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs)
- [crates/orphan-detector/src/capabilities_orphan_surfaces_analyzer.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/capabilities_orphan_surfaces_analyzer.rs)
- [crates/orphan-detector/src/capabilities_orphan_taxonomy_analyzer.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/capabilities_orphan_taxonomy_analyzer.rs)
- [crates/orphan-detector/src/capabilities_orphan_utility_analyzer.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/capabilities_orphan_utility_analyzer.rs)
- [crates/orphan-detector/src/lib.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/lib.rs)
- [crates/orphan-detector/src/root_orphan_detector_container.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/root_orphan_detector_container.rs)
- [crates/orphan-detector/src/utility_orphan_graph_resolver.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/utility_orphan_graph_resolver.rs)
- [crates/orphan-detector/src/utility_orphan_regex_patterns.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/orphan-detector/src/utility_orphan_regex_patterns.rs)
- [crates/shared/src/cli-commands/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/mod.rs)
- [crates/shared/src/cli-commands/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs)
- [crates/shared/src/code-analysis/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/mod.rs)
- [crates/shared/src/code-analysis/taxonomy_analysis_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_analysis_vo.rs)
- [crates/shared/src/common/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/mod.rs)
- [crates/shared/src/common/taxonomy_adapter_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_error.rs)
- [crates/shared/src/common/taxonomy_adapter_list_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_list_vo.rs)
- [crates/shared/src/common/taxonomy_adapter_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_name_vo.rs)
- [crates/shared/src/common/taxonomy_common_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_error.rs)
- [crates/shared/src/common/taxonomy_common_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_vo.rs)
- [crates/shared/src/common/taxonomy_definition_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_definition_vo.rs)
- [crates/shared/src/common/taxonomy_display_content_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_display_content_vo.rs)
- [crates/shared/src/common/taxonomy_duration_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_duration_vo.rs)
- [crates/shared/src/common/taxonomy_error_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_error_vo.rs)
- [crates/shared/src/common/taxonomy_filesystem_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_filesystem_error.rs)
- [crates/shared/src/common/taxonomy_job_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_job_vo.rs)
- [crates/shared/src/common/taxonomy_language_info_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_language_info_vo.rs)
- [crates/shared/src/common/taxonomy_line_count_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_line_count_vo.rs)
- [crates/shared/src/common/taxonomy_lint_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_lint_vo.rs)
- [crates/shared/src/common/taxonomy_message_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_message_vo.rs)
- [crates/shared/src/common/taxonomy_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_name_vo.rs)
- [crates/shared/src/common/taxonomy_path_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_path_vo.rs)
- [crates/shared/src/common/taxonomy_paths_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_paths_vo.rs)
- [crates/shared/src/common/taxonomy_response_data_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_response_data_vo.rs)
- [crates/shared/src/common/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_severity_vo.rs)
- [crates/shared/src/common/taxonomy_source_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_source_vo.rs)
- [crates/shared/src/common/taxonomy_suffix_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_suffix_vo.rs)
- [crates/shared/src/common/taxonomy_suggestion_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_suggestion_vo.rs)
- [crates/shared/src/common/taxonomy_threshold_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_threshold_vo.rs)
- [crates/shared/src/common/utility_file_handler.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/utility_file_handler.rs)
- [crates/shared/src/common/utility_layer_detector.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/utility_layer_detector.rs)
- [crates/shared/src/common/utility_value_object_generator.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/utility_value_object_generator.rs)
- [crates/shared/src/config-system/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/mod.rs)
- [crates/shared/src/config-system/taxonomy_config_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_config_vo.rs)
- [crates/shared/src/orphan-detector/contract_orphan_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/contract_orphan_aggregate.rs)
- [crates/shared/src/orphan-detector/contract_orphan_graph_resolver_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/contract_orphan_graph_resolver_protocol.rs)
- [crates/shared/src/orphan-detector/contract_orphan_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/contract_orphan_protocol.rs)
- [crates/shared/src/orphan-detector/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/mod.rs)
- [crates/shared/src/orphan-detector/taxonomy_orphan_contract_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/taxonomy_orphan_contract_vo.rs)
- [crates/shared/src/orphan-detector/taxonomy_violation_orphan_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/taxonomy_violation_orphan_vo.rs)
- [crates/shared/src/orphan-detector/utility_file_cache.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/utility_file_cache.rs)
- [crates/shared/src/orphan-detector/utility_orphan_detector.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/utility_orphan_detector.rs)
- [crates/shared/src/orphan-detector/utility_orphan_filename.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/utility_orphan_filename.rs)
- [crates/shared/src/orphan-detector/utility_orphan_io.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/utility_orphan_io.rs)
- [crates/shared/src/orphan-detector/utility_orphan_path.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/utility_orphan_path.rs)
- [crates/shared/src/orphan-detector/utility_workspace_scanner.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/utility_workspace_scanner.rs)
- [crates/shared/src/role-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/mod.rs)

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

## File: crates/orphan-detector/Cargo.toml

```toml
[package]
name = "orphan-detector-lint-arwaky"
version = "1.11.0"
edition = "2021"
description = "Unreachable/dead component detector covering AES501–AES506. Flags roles, capabilities, and surfaces that are never wired into an entry point."
license = "MIT"
repository = "https://github.com/rakaarwaky/lint-arwaky"
publish = true

[lints]
workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
regex.workspace = true
once_cell.workspace = true
rayon.workspace = true
shared.workspace = true

[dev-dependencies]
tokio.workspace = true
tempfile = "3"
criterion = { version = "0.8.2", features = ["html_reports"] }

[[bench]]
name = "bench_orphan_detector_graph"
path = "benches/bench_orphan_detector_graph.rs"
harness = false

```

---

## File: crates/orphan-detector/FRD.md

```rust
# FRD — orphan-detector

## System Overview

The orphan-detector crate identifies dead, unused, or unreachable code components across the 7-layer AES architecture. It builds an import reachability graph starting from valid entry points (containers, binary entries, main files), then flags any source file that has been orphaned. The orchestrator dispatches to 6 layer-specific analyzers (taxonomy, contract, capabilities, utility, agent, surfaces) and produces lint violations.

``` `
Entry Points (main.*, lib.rs, *_entry.*, *_container.*)
        │
        ▼
┌─────────────────────┐
│ Orphan Orchestrator │  ← orphan detection aggregate trait
│ (orchestrator)      │
└────┬────┬────┬──────┘
     │    │    │
     ▼    ▼    ▼
  Taxonomy  Contract  Capabilities  Utility  Agent  Surfaces
  Analyzer  Analyzer  Analyzer      Analyzer Analyzer Analyzer
``` `

**Scope:** All `.rs`, `.py`, `.ts`, `.js` source files in the workspace. Naming convention validation is handled by the naming rules crate — orphan-detector assumes naming is already correct and focuses solely on reachability.

## Functional Requirements

### FR-001: Import Graph Construction

- **Description**: Build a bidirectional import graph from all workspace source files, resolving cross-crate and cross-language imports.
- **Input**: List of source file paths (`Vec<String>`) and workspace root directory.
- **Output**: A graph analysis context containing the forward import graph, reverse link index, trait/class definition map, and implementation relationship map.
- **Business Rules**:
  - Scan all `crates/`, `packages/`, `modules/` subdirectories recursively for source files.
  - Resolve `crate::`, `super::`, `mod` imports (Rust); `from module import` (Python); `import { X } from` (TypeScript).
  - Expand file list to include all workspace source files for cross-crate import resolution.
- **Edge Cases**: Files with circular imports form cycles in the graph but do not cause infinite loops (BFS with visited set). Files outside supported extensions are silently skipped.
- **Error Handling**: Unreadable files are skipped with no error. Invalid paths produce no entry in the graph.

### FR-002: Entry Point Discovery

- **Description**: Identify valid entry points that anchor the reachability graph.
- **Input**: List of file paths and optional configured entry point patterns from the architecture configuration.
- **Output**: Set of entry point file paths.
- **Business Rules**:
  - Default entry point patterns: `main.rs`, `lib.rs`, `main.py`, `__main__.py`, `main.ts`, `main.js`, `index.ts`, `index.js`, `*_container.*`, `*_entry.*`.
  - Merges configured additional entry point patterns from each layer definition in the architecture configuration.
  - Deduplicates and sorts the final list.
- **Edge Cases**: Workspace with zero entry points results in all files flagged as orphans. Workspace with entry points in non-standard locations requires config override.
- **Error Handling**: Missing or inaccessible entry point files are excluded from the set.

### FR-003: Reachability Tracing

- **Description**: Perform BFS from all entry points through the import graph to determine which files are transitively reachable ("alive").
- **Input**: Entry point set (`Vec<String>`) and the forward import graph.
- **Output**: `Vec<String>` of all reachable file paths.
- **Business Rules**:
  - Uses breadth-first search with a visited tracker to avoid revisiting nodes.
  - A file is "alive" if it is transitively reachable from any entry point.
  - The alive set is used by capabilities, agent, and surfaces orphan analyzers.
- **Edge Cases**: Isolated files with no imports from any entry point are flagged. Entry points that import nothing are valid (they are roots).
- **Error Handling**: Cycles in the graph are handled by the visited set — no infinite loops.

### FR-004: Taxonomy Orphan Detection (AES501)

- **Description**: Check that taxonomy layer files (`taxonomy_*`) are imported by at least one file from any other layer.
- **Input**: File path, root directory, reverse link map.
- **Output**: An orphan indicator result with is_orphan flag, reason, and severity.
- **Business Rules**:
  - A taxonomy file is orphan if no contract, capabilities, agent, utility, or surface file imports it.
  - Internal taxonomy imports (from capabilities) are valid — taxonomy types used within capabilities but not exposed in contract signatures are not orphans.
- **Edge Cases**: Taxonomy files imported only by other taxonomy files are flagged (no consumer outside taxonomy).
- **Error Handling**: Files with no detectable imports are treated as orphan candidates.

### FR-005: Contract Orphan Detection (AES502)

- **Description**: Check that contract files have at least one implementation or consumer.
- **Input**: File path, root directory, definition map, inheritance map, all workspace files.
- **Output**: An orphan indicator result with is_orphan flag, reason, and severity.
- **Business Rules**:
  - Protocol contracts — Must be implemented by a capabilities file AND called by an agent or capability via dependency injection.
  - Aggregate contracts — Must be implemented by an agent file AND called by a surface file via dependency injection.
  - Detects `impl Trait for Type` (Rust), `class Foo(Trait)` (Python), `class Foo implements Trait` (TS).
- **Edge Cases**: Protocol with an implementation but zero callers is still reachable (implementation exists).
- **Error Handling**: Files with unparseable signatures are not flagged as orphan (fail-safe).

### FR-006: Capabilities Orphan Detection (AES503)

- **Description**: Check that capability files are wired in a root container or reachable from entry points.
- **Input**: File path, root directory, reachable file paths set.
- **Output**: An orphan indicator result with is_orphan flag, reason, and severity.
- **Business Rules**:
  - Capabilities use dependency injection (`Arc<T>` in Rust, DI containers in Python/TS).
  - A capability is orphan if its struct/trait names do not appear in any container file AND the file is not transitively reachable.
  - Capabilities should NOT directly import agent or other capability files (enforced by role-rules, not here).
- **Edge Cases**: A capability imported only by other capabilities in a chain is alive if any link in the chain reaches a container.
- **Error Handling**: Files with no struct/trait names detectable are treated as potential orphans.

### FR-007: Utility Orphan Detection (AES504)

- **Description**: Check that utility files are imported by at least one consumer layer (agent, capability, or surface).
- **Input**: File path, root directory, all workspace files, reverse link map.
- **Output**: An orphan indicator result with is_orphan flag, reason, and severity.
- **Business Rules**:
  - Utility-only import chains are flagged as dead code (utility importing utility does not count).
  - Must have inbound imports from capabilities, agent, or surface files.
- **Edge Cases**: Utility imported by another utility that is itself orphaned — the chain is dead.
- **Error Handling**: Unparseable import statements cause the utility to be treated as a candidate orphan.

### FR-008: Agent Orphan Detection (AES505)

- **Description**: Check that agent orchestrator files are called by surface layer files or binary entry points.
- **Input**: File path, root directory, all workspace files.
- **Output**: An orphan indicator result with is_orphan flag, reason, and severity.
- **Business Rules**:
  - Extract aggregate trait names from the agent file (e.g., the implementation of the orphan detection aggregate trait).
  - Check if any surface, entry, main, index, or container file references these aggregate names.
  - Severity: HIGH — orphaned agent means entire feature behavior is unreachable.
- **Edge Cases**: Agent file with no aggregate implementation is flagged immediately.
- **Error Handling**: Files that cannot be parsed for trait implementations are flagged as HIGH severity orphans.

### FR-009: Surface Orphan Detection (AES506)

- **Description**: Check that surface files are reachable based on their group classification (Smart, Utility, Passive).
- **Input**: File path, root directory, reachable file paths set, optional layer definition.
- **Output**: An orphan indicator result with is_orphan flag, reason, and severity.
- **Business Rules**:
  - **Smart** (`_command`, `_controller`, `_page`, `_entry`): Must be imported by entry point or container.
  - **Utility** (`_hook`, `_store`, `_action`, `_screen`): Must be imported by a Smart surface.
  - **Passive** (all other surface files): Must be imported by Smart OR Utility surface.
  - Dependency chain: `Entry → Smart → Utility → Passive`.
  - Detection uses inbound import graph only (no identifier-based fallback).
- **Edge Cases**: A passive surface imported only by another passive surface is orphan. Smart surfaces bypass passive checks.
- **Error Handling**: Files with unclassifiable suffixes default to Passive group.

### FR-010: Barrel File Exception Handling

- **Description**: Skip known barrel/package marker files from orphan detection.
- **Input**: File path.
- **Output**: Skip signal (no violation produced).
- **Business Rules**:
  - `__init__.py` — Python package marker.
  - `mod.rs` — Rust module re-export.
  - `index.ts` / `index.js` — TypeScript/JavaScript barrel files.
  - These files are package markers or re-export files, not logic.
- **Edge Cases**: A file named `mod.rs` inside a deeply nested module is still skipped.
- **Error Handling**: N/A — simple filename suffix check.

## API Contract

| Function                           | Input                                                                 | Output                     | Description                                       |
| ---------------------------------- | --------------------------------------------------------------------- | -------------------------- | ------------------------------------------------- |
| Build orphan graph context         | File list, root directory                                             | Graph analysis context     | Build full import graph for the workspace         |
| Identify orphan entry points       | File list                                                             | Set of entry point paths   | Discover all valid entry points                   |
| Full orphan scan                   | File list, root directory                                             | Lint results               | Full orphan scan with graph construction          |
| Orphan scan with context           | File list, root directory, pre-built graph                            | Lint results               | Orphan scan with pre-built graph (avoids rebuild) |
| Check taxonomy orphan              | File path, root directory, layer definition, reverse link map         | Orphan indicator result    | Check single taxonomy file for orphan status      |
| Check contract orphan              | File path, root directory, definition map, inheritance map, all files | Orphan indicator result    | Check single contract file for orphan status      |
| Check capabilities orphan          | File path, root directory, reachable file set                         | Orphan indicator result    | Check single capabilities file for orphan status  |
| Check utility orphan               | File path, root directory, all files, reverse link map                | Orphan indicator result    | Check single utility file for orphan status       |
| Check agent orphan                 | File path, root directory, all files                                  | Orphan indicator result    | Check single agent file for orphan status         |
| Check surface orphan               | File path, root directory, reachable file set, layer definition       | Orphan indicator result    | Check single surface file for orphan status       |
| Create default DI container        | —                                                                     | Orphan detection container | Default dependency injection container            |
| Create DI container with config    | Architecture configuration                                            | Orphan detection container | DI container with custom config                   |
| Create DI from config orchestrator | Config orchestrator reference, root directory                         | Orphan detection container | Canonical DI from config orchestrator             |

## Integration Points

- **Internal**:
  - The code analysis shared module — graph analysis context, import graph, reverse link map, orphan indicator result, and reachability result value objects.
  - The orphan detection aggregate contract — aggregate trait defining the public API surface.
  - The orphan detection protocol contracts — 6 layer-specific orphan indicator protocols.
  - The orphan detection file I/O utility — file reading, scanning, directory checks.
  - The orphan detection filename utility — filename parsing (stem, suffix, basename).
  - The orphan detection path utility — path resolution and ignore checking.
  - The common layer detection utility — layer detection from filename prefix.
  - The config system configuration value objects — architecture config for exceptions and rules.
  - The lint result value objects — lint result, severity, and location types.
  - The config system orchestrator aggregate — config loading from orchestrator.
- **External**: None — pure static analysis, no network or filesystem writes.

## Non-functional Requirements (Detailed)

- **Performance**: 1,000 files < 500ms; 5,000 files < 2s; 10,000 files < 5s.
- **Memory**: Graph construction should hold all edges in memory; for 10,000 files with average 10 imports each, peak memory < 50MB.
- **Accuracy**: Zero false positives on transitively reachable code. A file is valid if it is transitively reachable from an entry point.
- **Concurrency**: Thread-safe via async runtime. Protocol-based shared ownership for concurrent analysis.
- **Configurability**: All behavior overridable via the architecture configuration. No hardcoded assumptions about project structure beyond workspace directory conventions.

## Test Scenarios / QA Checklist

- [ ] Workspace with 100 files, 5 orphans across 3 layers — all 5 detected, 0 false positives.
- [ ] Python nested `__init__.py` packages — barrel files skipped, not flagged as orphan.
- [ ] TypeScript barrel `index.ts` re-exports — barrel files skipped.
- [ ] Rust `mod.rs` re-exports — barrel files skipped.
- [ ] Circular imports between two capabilities — both reachable, neither flagged.
- [ ] Contract protocol with implementation but zero callers — still flagged as orphan.
- [ ] Agent file with no aggregate implementation — flagged as HIGH severity orphan.
- [ ] Surface dependency chain: Smart → Utility → Passive — all alive. Remove Smart import — Utility + Passive flagged.
- [ ] Config with `check_orphan: false` for a layer — no violations for that layer.
- [ ] Config with exceptions list — excepted files produce no violations.
- [ ] Config with `ignored_paths` — excluded paths produce no violations.
- [ ] Workspace with zero entry points — all non-barrel files flagged as orphans.
- [ ] Cross-crate imports (crate A imports from crate B) — graph resolves correctly.
- [ ] 10,000 file workspace completes in under 5 seconds.
- [ ] Configuration disabled — full orphan scan returns empty immediately.

## Assumptions & Constraints

- Workspace follows AES convention with `crates/`, `packages/`, `modules/` directories.
- Naming convention validation is handled by the naming rules crate; orphan-detector assumes filenames are correctly named.
- Entry points are identified by filename patterns, not by content analysis.
- Import resolution is language-specific (Rust `mod`/`crate`, Python `import`/`from`, TypeScript `import`).
- No network calls are required; all analysis is local filesystem.
- Configuration is loaded once and reused across all checks in a scan.

## Glossary

| Term             | Definition                                                                 |
| ---------------- | -------------------------------------------------------------------------- |
| **Orphan**       | A source file not transitively reachable from any entry point              |
| **Entry point**  | A file that anchors the reachability graph (main, lib, container, entry)   |
| **Barrel file**  | A package marker or re-export file (`__init__.py`, `mod.rs`, `index.ts`)   |
| **Alive file**   | A file reachable via BFS from any entry point through the import graph     |
| **AES**          | Architecture Enforcement Standard — the 7-layer coding convention          |
| **DI**           | Dependency Injection — wiring implementations to trait/interface contracts |
| **Inbound link** | A file that imports the target file (reverse import edge)                  |

## Reference

- PRD: [PRD.md](../../PRD.md)
```

---

## File: crates/orphan-detector/src/agent_orphan_orchestrator.rs

```rust
use shared::cli_commands::LintResult;
use shared::code_analysis::{
    GraphAnalysisContext, ImportGraph, OrphanIndicatorResult, ReachabilityResult,
};

use shared::common::FilePath;

use shared::common::Severity;
use shared::config_system::ArchitectureConfig;
use shared::orphan_detector::{IOrphanAggregate, IOrphanGraphResolverProtocol};

use shared::orphan_detector::OrphanFileListVO;
use shared::orphan_detector::{
    IAgentOrphanProtocol, ICapabilitiesOrphanProtocol, IContractOrphanProtocol,
    ISurfacesOrphanProtocol, ITaxonomyOrphanProtocol, IUtilityOrphanProtocol,
};

use shared::common::taxonomy_common_vo::BooleanVO;
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::{
    AdapterName, ColumnNumber, DescriptionVO, ErrorCode, LayerNameVO, LineNumber, LintMessage,
    LocationList, ScopeRef,
};
use shared::config_system::taxonomy_config_vo::OrphanRuleVO;
use shared::role_rules::{
    LAYER_AGENT, LAYER_CAPABILITIES, LAYER_CONTRACT, LAYER_SURFACES, LAYER_TAXONOMY, LAYER_UTILITY,
};

use std::collections::HashSet;
use std::collections::VecDeque;
use std::sync::Arc;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

// ─── Block 1: Struct Definition ───────────────────────────

/// Dependencies for ArchOrphanAnalyzer to avoid too_many_arguments.
pub struct ArchOrphanDeps {
    pub resolver: Arc<dyn IOrphanGraphResolverProtocol>,
    pub taxonomy_analyzer: Arc<dyn ITaxonomyOrphanProtocol>,
    pub contract_analyzer: Arc<dyn IContractOrphanProtocol>,
    pub capabilities_analyzer: Arc<dyn ICapabilitiesOrphanProtocol>,
    pub utility_analyzer: Arc<dyn IUtilityOrphanProtocol>,
    pub agent_analyzer: Arc<dyn IAgentOrphanProtocol>,
    pub surfaces_analyzer: Arc<dyn ISurfacesOrphanProtocol>,
}

pub struct ArchOrphanAnalyzer {
    deps: ArchOrphanDeps,
    config: ArchitectureConfig,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
impl IOrphanAggregate for ArchOrphanAnalyzer {
    fn build_orphan_graph_context(
        &self,
        files: &OrphanFileListVO,
        root_dir: &FilePath,
    ) -> GraphAnalysisContext {
        let all_workspace_files = self._expand_workspace_files(files, root_dir);
        let full_files_vo = OrphanFileListVO::new(all_workspace_files);
        self.deps
            .resolver
            .build_graph_context(std::slice::from_ref(&full_files_vo), root_dir.value())
    }

    fn identify_orphan_entry_points(&self, files: &OrphanFileListVO) -> OrphanFileListVO {
        self.deps
            .resolver
            .identify_entry_points(std::slice::from_ref(files), &[])
    }

    fn check_orphans(&self, files: &OrphanFileListVO, root_dir: &FilePath) -> Vec<LintResult> {
        if !self.config.enabled.value {
            return Vec::new();
        }

        let all_workspace_files = self._expand_workspace_files(files, root_dir);

        let file_vo = OrphanFileListVO::new(all_workspace_files);
        let context = self
            .deps
            .resolver
            .build_graph_context(std::slice::from_ref(&file_vo), root_dir.value());

        self._check_orphans_inner(files, root_dir, &context, &file_vo)
    }

    fn scan_orphans(
        &self,
        root_dir: &FilePath,
        ignored: &[String],
    ) -> (GraphAnalysisContext, Vec<LintResult>) {
        let root_path = std::path::Path::new(root_dir.value());
        let mut all_files = Vec::new();
        if root_path.is_dir() {
            if let Ok(dir_path) =
                shared::common::taxonomy_path_vo::DirectoryPath::new(root_dir.value().to_string())
            {
                if let Ok(list) =
                    shared::common::utility_file_handler::scan_directory(&dir_path, ignored)
                {
                    all_files = list.values.iter().map(|f| f.value.clone()).collect();
                }
            }
        } else if root_path.is_file() {
            // Single file scan — include the file directly
            let ext = root_path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx")
                && !shared::common::utility_file_handler::is_path_ignored(
                    &root_path.to_string_lossy(),
                    ignored,
                )
            {
                all_files.push(root_dir.value().to_string());
            }
        } // Normalize all file paths to be relative to workspace root so that
          // inbound_links (built by the graph resolver) and orphan analyzers
          // use a consistent path format.
        let top_root = shared::common::utility_file_handler::find_workspace_root(root_dir.value())
            .unwrap_or_else(|| root_path.to_path_buf());
        let all_files: Vec<String> = all_files
            .into_iter()
            .map(|f| {
                std::path::Path::new(&f)
                    .strip_prefix(&top_root)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or(f)
            })
            .collect();

        let files_vo = OrphanFileListVO::new(all_files);

        // Expand workspace files BEFORE building the graph context so that the
        // import graph has all workspace files to resolve cross-file imports.
        // Without this, single-file scans build a graph from only 1 file,
        // causing orphan detection to miss connections to container/surface files.
        let expanded_files = self._expand_workspace_files(&files_vo, root_dir);
        let expanded_vo = OrphanFileListVO::new(expanded_files);

        let context = self.build_orphan_graph_context(&expanded_vo, root_dir);
        let results = self.check_orphans_with_context(&expanded_vo, root_dir, &context);
        (context, results)
    }

    fn check_orphans_with_context(
        &self,
        files: &OrphanFileListVO,
        root_dir: &FilePath,
        context: &GraphAnalysisContext,
    ) -> Vec<LintResult> {
        if !self.config.enabled.value {
            return Vec::new();
        }

        // Files are already expanded before graph context build (in scan_orphans),
        // so file_vo is the same as files — no need to expand again.
        self._check_orphans_inner(files, root_dir, context, files)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl ArchOrphanAnalyzer {
    pub fn new(deps: ArchOrphanDeps, config: ArchitectureConfig) -> Self {
        Self { deps, config }
    }

    fn _expand_workspace_files(
        &self,
        files: &OrphanFileListVO,
        root_dir: &FilePath,
    ) -> Vec<String> {
        let root_path = std::path::Path::new(root_dir.value());
        let top_root = shared::common::utility_file_handler::find_workspace_root(root_dir.value())
            .unwrap_or_else(|| root_path.to_path_buf());
        let mut seen: HashSet<String> = files.values.iter().cloned().collect();
        let mut result: Vec<String> = files.values.clone();
        for ws_dir in &["crates", "packages", "modules"] {
            let ws_path = top_root.join(ws_dir);
            if shared::orphan_detector::utility_orphan_io::is_dir(&ws_path) {
                let entries = shared::orphan_detector::utility_orphan_io::scan_directory(&ws_path);
                for (name, _path_str, is_dir_entry) in entries {
                    if !is_dir_entry {
                        continue;
                    }
                    // Skip ignored workspace members (e.g. tests/, target/)
                    let member_dir = top_root.join(ws_dir).join(&name);
                    if shared::common::utility_file_handler::is_ignored_dir(
                        &member_dir,
                        &self
                            .config
                            .ignored_paths
                            .values
                            .iter()
                            .map(|v| v.value.clone())
                            .collect::<Vec<_>>(),
                    ) {
                        continue;
                    }
                    let src_dir = top_root.join(ws_dir).join(&name).join("src");
                    if shared::orphan_detector::utility_orphan_io::is_dir(&src_dir) {
                        let workspace_files =
                            shared::orphan_detector::utility_orphan_io::scan_directory_recursive(
                                &src_dir,
                            );
                        for f in workspace_files {
                            // Skip ignored paths (e.g. tests/, target/)
                            if shared::common::utility_file_handler::is_path_ignored(
                                &f,
                                &self
                                    .config
                                    .ignored_paths
                                    .values
                                    .iter()
                                    .map(|v| v.value.clone())
                                    .collect::<Vec<_>>(),
                            ) {
                                continue;
                            }
                            let rel = std::path::Path::new(&f)
                                .strip_prefix(&top_root)
                                .map(|p| p.to_string_lossy().to_string())
                                .unwrap_or(f);
                            if seen.insert(rel.clone()) {
                                result.push(rel);
                            }
                        }
                    }
                }
                // Also scan source files directly in the workspace dir (e.g. modules/root_cli_main_entry.py)
                let root_files =
                    shared::orphan_detector::utility_orphan_io::scan_directory_recursive(&ws_path);
                for f in root_files {
                    // Skip ignored paths (e.g. tests/, target/)
                    if shared::common::utility_file_handler::is_path_ignored(
                        &f,
                        &self
                            .config
                            .ignored_paths
                            .values
                            .iter()
                            .map(|v| v.value.clone())
                            .collect::<Vec<_>>(),
                    ) {
                        continue;
                    }
                    let rel = std::path::Path::new(&f)
                        .strip_prefix(&top_root)
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or(f);
                    if seen.insert(rel.clone()) {
                        result.push(rel);
                    }
                }
            }
        }
        result
    }

    fn _check_orphans_inner(
        &self,
        files: &OrphanFileListVO,
        root_dir: &FilePath,
        context: &GraphAnalysisContext,
        file_vo: &OrphanFileListVO,
    ) -> Vec<LintResult> {
        let configured = self.get_orphan_entry_points();
        let configured_vo =
            shared::orphan_detector::taxonomy_orphan_contract_vo::OrphanEntryPatternListVO::new(
                configured,
            );
        let entry_points = self
            .deps
            .resolver
            .identify_entry_points(std::slice::from_ref(file_vo), &[configured_vo]);
        // Compute top_root early so alive_result can use absolute paths (matching
        // the format used by _process_file for file_fp — fixes path format mismatch
        // that caused false-positive AES506/AES503 orphan violations)
        let root_path = std::path::Path::new(root_dir.value());
        let top_root = shared::common::utility_file_handler::find_workspace_root(root_dir.value())
            .unwrap_or_else(|| root_path.to_path_buf());
        let alive_set = self._trace_reachability(&entry_points.values, &context.import_graph);
        let alive_result = ReachabilityResult::new(
            alive_set
                .iter()
                .filter_map(|s| {
                    let abs = top_root.join(s);
                    FilePath::new(abs.to_string_lossy().to_string()).ok()
                })
                .collect(),
        );
        let layer_keys: Vec<String> = self
            .config
            .layers
            .keys()
            .map(|k| k.value.to_string())
            .collect();

        // Convert all_files to absolute paths so sub-analyzers can read file contents.
        // file_vo.values are relative to top_root (workspace root), but sub-analyzers
        // (contract, agent, utility) use orphan_io::read_file_safe() which needs
        // paths resolvable from CWD. Make them absolute by prepending top_root.
        let top_root_str = top_root.to_string_lossy().to_string();

        let all_files: Vec<String> = file_vo
            .values
            .iter()
            .map(|rel| {
                let abs = top_root.join(rel);
                abs.to_string_lossy().to_string()
            })
            .collect();
        files
            .values
            .par_iter()
            .filter_map(|f| {
                self._process_file(
                    f,
                    context,
                    &alive_result,
                    &layer_keys,
                    &all_files,
                    &top_root_str,
                )
            })
            .collect()
    }

    fn _process_file(
        &self,
        f: &str,
        context: &GraphAnalysisContext,
        alive_result: &ReachabilityResult,
        layer_keys: &[String],
        all_files: &[String],
        top_root_str: &str,
    ) -> Option<LintResult> {
        // Resolve relative path to absolute so sub-analyzers can read file contents
        let abs_f = std::path::Path::new(top_root_str).join(f);
        let abs_f_str = abs_f.to_string_lossy().to_string();
        let file_fp = FilePath::new(&abs_f_str).ok()?;
        let filename = shared::common::utility_layer_detector::extract_filename(file_fp.value());
        let base_layer =
            shared::common::utility_layer_detector::detect_layer_from_prefix(filename)?;
        let layer_str = shared::common::utility_layer_detector::resolve_specialized_layer(
            &base_layer,
            file_fp.value(),
            layer_keys,
        );
        let definition =
            shared::common::utility_layer_detector::get_layer_def(&layer_str, &self.config.layers)
                .cloned()
                .unwrap_or_else(|| LayerDefinition {
                    orphan: OrphanRuleVO {
                        check_orphan: BooleanVO::new(true),
                        ..Default::default()
                    },
                    ..Default::default()
                });

        let basename = file_fp.basename();
        if definition.exceptions.values.contains(&basename) {
            return None;
        }
        if !definition.orphan.check_orphan.value {
            return None;
        }

        let layer_vo = LayerNameVO::new(&layer_str);
        let res = self._evaluate_layer(
            &abs_f_str,
            context,
            alive_result,
            &layer_vo,
            all_files,
            top_root_str,
        );
        if res.is_orphan {
            let code = match layer_str.to_lowercase() {
                s if s.contains(LAYER_TAXONOMY) => "AES501",
                s if s.contains(LAYER_CONTRACT) => "AES502",
                s if s.contains(LAYER_CAPABILITIES) => "AES503",
                s if s.contains(LAYER_UTILITY) => "AES504",
                s if s.contains(LAYER_AGENT) => "AES505",
                s if s.contains(LAYER_SURFACES) => "AES506",
                _ => return None,
            };
            return Some(self._make_result(f, &res.reason, res.severity, code));
        }
        None
    }

    fn _make_result(&self, file: &str, msg: &str, sev: Severity, code: &str) -> LintResult {
        LintResult {
            file: FilePath {
                value: file.to_string(),
            },
            line: LineNumber::new(1),
            column: ColumnNumber::new(1),
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

    fn _trace_reachability(&self, entry_points: &[String], graph: &ImportGraph) -> HashSet<String> {
        let mut reachable: HashSet<String> = entry_points.iter().cloned().collect();
        let mut queue: VecDeque<String> = entry_points.iter().cloned().collect();

        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = graph.mapping.get(&current) {
                for neighbor in neighbors {
                    if reachable.insert(neighbor.clone()) {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        reachable
    }

    fn _evaluate_layer(
        &self,
        f: &str,
        context: &GraphAnalysisContext,
        alive_result: &ReachabilityResult,
        layer_vo: &LayerNameVO,
        all_files: &[String],
        top_root: &str,
    ) -> OrphanIndicatorResult {
        // Barrel file exceptions — package markers and re-export files, not logic
        if f.ends_with("__init__.py")
            || f.ends_with("/mod.rs")
            || f.ends_with("\\mod.rs")
            || f.ends_with("/index.ts")
            || f.ends_with("\\index.ts")
            || f.ends_with("/index.js")
            || f.ends_with("\\index.js")
        {
            return OrphanIndicatorResult::new(false, String::new(), Severity::HIGH);
        }

        let layer_str = layer_vo.value.to_lowercase();
        let fp = match FilePath::new(f.to_string()) {
            Ok(fp) => fp,
            Err(_) => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        };
        // Use top_root as the root directory (absolute path so file operations work)
        let root = FilePath {
            value: top_root.to_string(),
        };

        if layer_str.contains(LAYER_TAXONOMY) {
            return self.deps.taxonomy_analyzer.is_taxonomy_orphan(
                &fp,
                &root,
                None,
                &context.inbound_links,
            );
        }

        if layer_str.contains(LAYER_CONTRACT) {
            return self.deps.contract_analyzer.is_contract_orphan(
                &fp,
                &root,
                &context.file_definitions,
                &context.inheritance_map,
                all_files,
            );
        }

        if layer_str.contains(LAYER_CAPABILITIES) {
            return self.deps.capabilities_analyzer.is_capabilities_orphan(
                &fp,
                &root,
                alive_result,
            );
        }

        if layer_str.contains(LAYER_UTILITY) {
            return self.deps.utility_analyzer.is_utility_orphan(
                &fp,
                &root,
                all_files,
                &context.inbound_links,
            );
        }

        if layer_str.contains(LAYER_AGENT) {
            return self
                .deps
                .agent_analyzer
                .is_agent_orphan(&fp, &root, all_files);
        }

        if layer_str.contains(LAYER_SURFACES) {
            return self
                .deps
                .surfaces_analyzer
                .is_surface_orphan(&fp, &root, alive_result, None);
        }

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }

    fn get_orphan_entry_points(&self) -> Vec<String> {
        let mut entry_points = vec![
            "_container.rs".into(),
            "_container.py".into(),
            "_container.ts".into(),
            "_container.js".into(),
            "_entry.rs".into(),
            "_entry.py".into(),
            "_entry.ts".into(),
            "_entry.js".into(),
            "main.rs".into(),
            "lib.rs".into(),
            "main.py".into(),
            "__main__.py".into(),
            "main.ts".into(),
            "main.js".into(),
            "index.ts".into(),
            "index.js".into(),
        ];
        for layer_def in self.config.layers.values() {
            entry_points.extend(layer_def.orphan.orphan_entry_points.values.iter().cloned());
        }
        entry_points.sort();
        entry_points.dedup();
        entry_points
    }
}
```

---

## File: crates/orphan-detector/src/capabilities_orphan_agent_analyzer.rs

```rust
// PURPOSE: AgentOrphanAnalyzer — IAgentOrphanProtocol for detecting orphan agent files
use shared::code_analysis::OrphanIndicatorResult;
use shared::common::{FilePath, Severity};

use shared::orphan_detector::{AesOrphanViolation, IAgentOrphanProtocol};

use regex::Regex;
use std::sync::OnceLock;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct AgentOrphanAnalyzer {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IAgentOrphanProtocol for AgentOrphanAnalyzer {
    fn is_agent_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        all_files: &[String],
    ) -> OrphanIndicatorResult {
        let fp = f.value();
        let content = match shared::orphan_detector::utility_orphan_io::read_file_safe(fp) {
            c if c.is_empty() => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
            }
            c => c,
        };

        // Step 1: Find aggregate traits this agent implements
        let aggregate_traits = Self::extract_aggregate_traits(&content);
        if aggregate_traits.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Bug 2 fix: agent is orphan only if ALL aggregates are uncalled (not ANY)
        // Pre-filter candidate files (surfaces, containers, entries, mains) once
        let candidates: Vec<&String> = all_files
            .iter()
            .filter(|cf| {
                let cb = match cf.split('/').next_back() {
                    Some(b) => b,
                    None => return false,
                };
                cb.starts_with("surface_")
                    || cb.ends_with("_container.rs")
                    || cb.ends_with("_container.py")
                    || cb.ends_with("_container.ts")
                    || cb.ends_with("_container.js")
                    || cb.ends_with("_entry.rs")
                    || cb.ends_with("_entry.py")
                    || cb.ends_with("_entry.ts")
                    || cb.ends_with("_entry.js")
                    || matches!(
                        cb,
                        "main.rs"
                            | "lib.rs"
                            | "main.py"
                            | "__main__.py"
                            | "main.ts"
                            | "main.js"
                            | "index.ts"
                            | "index.js"
                    )
            })
            .collect();

        let mut any_called = false;
        'outer: for agg_name in &aggregate_traits {
            for cf in &candidates {
                let c = shared::orphan_detector::utility_orphan_io::read_file_safe(cf);
                if Self::content_contains_word(&c, agg_name) {
                    any_called = true;
                    break 'outer;
                }
            }
        }

        if !any_called {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::AgentOrphan {
                    agg_name: aggregate_traits.join(", "),
                    reason: Some(
                        format!(
                            "Agent orphan: aggregates [{}] not called by any surface.",
                            aggregate_traits.join(", ")
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::HIGH,
            );
        }

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for AgentOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentOrphanAnalyzer {
    /// Check if `text` contains `word` as a whole word (not substring).
    fn content_contains_word(text: &str, word: &str) -> bool {
        text.split(|c: char| !c.is_alphanumeric() && c != '_')
            .any(|w| w == word)
    }
    pub fn new() -> Self {
        Self {}
    }

    /// Extract aggregate trait names from agent file content.
    /// Looks for: impl IAggregateTrait for Struct, Box<dyn IAggregateTrait>, Arc<dyn IAggregateTrait>
    fn extract_aggregate_traits(content: &str) -> Vec<String> {
        let mut traits = Vec::new();

        // Rust: impl ITrait for Struct (with optional generics: impl<T> Trait for Struct)
        if let Some(re) = Self::re_impl_generic() {
            for cap in re.captures_iter(content) {
                let name = cap[1].to_string();
                if name.contains("Aggregate") || name.ends_with("Aggregate") {
                    traits.push(name);
                }
            }
        }

        // Rust: Box<dyn ITrait> or Arc<dyn ITrait>
        if let Some(re) = Self::re_dyn() {
            for cap in re.captures_iter(content) {
                let name = cap[1].to_string();
                if name.contains("Aggregate") || name.ends_with("Aggregate") {
                    traits.push(name);
                }
            }
        }

        // Python: class Struct(ITrait):
        if let Some(re) = Self::re_py_class() {
            for cap in re.captures_iter(content) {
                for part in cap[1].split(',') {
                    let name = part.trim().to_string();
                    if name.contains("Aggregate") || name.ends_with("Aggregate") {
                        traits.push(name);
                    }
                }
            }
        }

        // JS/TS: class Struct implements IAggregateTrait
        if let Some(re) = Self::re_ts_implements() {
            for cap in re.captures_iter(content) {
                let name = cap[1].to_string();
                if name.contains("Aggregate") || name.ends_with("Aggregate") {
                    traits.push(name);
                }
            }
        }

        traits.sort();
        traits.dedup();
        traits
    }

    /// Cached regex for Rust impl with optional generics.
    /// Handles nested angle brackets like `<T: Into<U>>` via balanced `<>` matching.
    fn re_impl_generic() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        // Use `[^<>]*(?:<[^<>]*>[^<>]*)*` for one level of nested angle brackets.
        RE.get_or_init(|| {
            Regex::new(r"impl\s*(?:<[^<>]*(?:<[^<>]*>[^<>]*)*>)?\s+([A-Za-z0-9_]+)\s+for\s+").ok()
        })
        .as_ref()
    }

    fn re_dyn() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"(?:Box|Arc)<dyn\s+([A-Za-z0-9_]+)>").ok())
            .as_ref()
    }

    fn re_py_class() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"class\s+\w+\(([^)]+)\)").ok())
            .as_ref()
    }

    fn re_ts_implements() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"class\s+\w+\s+implements\s+(\w+)").ok())
            .as_ref()
    }
}
```

---

## File: crates/orphan-detector/src/capabilities_orphan_capabilities_analyzer.rs

```rust
// PURPOSE: CapabilitiesOrphanAnalyzer — ICapabilitiesOrphanProtocol for orphan capability detection
use shared::code_analysis::{OrphanIndicatorResult, ReachabilityResult};

use shared::common::{FilePath, Severity};

use shared::orphan_detector::{AesOrphanViolation, ICapabilitiesOrphanProtocol};

use shared::orphan_detector::utility_file_cache;
use shared::orphan_detector::utility_orphan_detector::{extract_struct_names, extract_trait_names};
use shared::orphan_detector::utility_orphan_filename::file_stem;
use shared::orphan_detector::utility_workspace_scanner::{
    check_wired_in_container, find_workspace_root,
};

use std::sync::Mutex;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CapabilitiesOrphanAnalyzer {
    container_cache: Mutex<Option<(std::path::PathBuf, Vec<std::path::PathBuf>)>>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ICapabilitiesOrphanProtocol for CapabilitiesOrphanAnalyzer {
    fn is_capabilities_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult {
        let is_reachable = alive_files.paths.contains(f);
        if is_reachable {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Check if wired in any container
        let fp = f.value();
        let stem = file_stem(fp);

        if !fp.is_empty() {
            let path = FilePath::new(fp).unwrap_or_default();
            let content = utility_file_cache::read_cached(&path);
            let mut identifiers: Vec<String> = Vec::new();
            let content_ref = content.value();
            identifiers.extend(extract_struct_names(content_ref));
            identifiers.extend(extract_trait_names(content_ref));
            identifiers.push(stem.clone());

            let pascal_stem: String = stem
                .split('_')
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let mut c = s.chars();
                    match c.next() {
                        Some(f) => f.to_uppercase().to_string() + c.as_str(),
                        None => String::new(),
                    }
                })
                .collect();
            identifiers.push(pascal_stem);

            // Search for container files in workspace root (cached)
            let root = std::path::Path::new(root_dir.value());
            if let Ok(workspace_root) = find_workspace_root(root) {
                let _container_files = self.cached_container_files(&workspace_root);
                let wired = check_wired_in_container(&workspace_root, &identifiers);
                if wired {
                    return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                }
            }
        }

        OrphanIndicatorResult::new(
            true,
            AesOrphanViolation::CapabilitiesOrphan {
                stem,
                reason: Some("Not reachable from any entry point.".into()),
            }
            .to_string(),
            Severity::MEDIUM,
        )
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for CapabilitiesOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilitiesOrphanAnalyzer {
    pub fn new() -> Self {
        Self {
            container_cache: Mutex::new(None),
        }
    }

    fn cached_container_files(
        &self,
        workspace_root: &std::path::Path,
    ) -> Option<Vec<std::path::PathBuf>> {
        if let Ok(mut guard) = self.container_cache.lock() {
            if let Some((cached_root, cached_files)) = &*guard {
                if cached_root == workspace_root {
                    return Some(cached_files.clone());
                }
            }
            // Cache miss: find container files
            let mut container_files = Vec::new();
            for dir_name in &["crates", "packages", "modules"] {
                let dir = workspace_root.join(dir_name);
                if dir.is_dir() {
                    let files =
                        shared::orphan_detector::utility_orphan_io::scan_directory_recursive(&dir);
                    for file_path in &files {
                        if let Some(name) = std::path::Path::new(file_path)
                            .file_name()
                            .and_then(|n| n.to_str())
                        {
                            if name.ends_with("_container.rs")
                                || name.ends_with("_container.py")
                                || name.ends_with("_container.ts")
                                || name.ends_with("_container.js")
                            {
                                container_files.push(std::path::PathBuf::from(file_path));
                            }
                        }
                    }
                }
            }
            *guard = Some((workspace_root.to_path_buf(), container_files.clone()));
            Some(container_files)
        } else {
            None
        }
    }
}
```

---

## File: crates/orphan-detector/src/capabilities_orphan_contract_analyzer.rs

```rust
// PURPOSE: ContractOrphanAnalyzer — IContractOrphanProtocol for orphan contract detection
use regex::Regex;
use shared::code_analysis::{FileDefinitionMap, InheritanceMap, OrphanIndicatorResult};

use shared::common::{FilePath, Severity};

use shared::orphan_detector::{AesOrphanViolation, IContractOrphanProtocol};

use shared::orphan_detector::utility_orphan_filename::{file_basename, file_suffix};
use shared::orphan_detector::utility_orphan_io as orphan_io;
use shared::orphan_detector::utility_workspace_scanner::collect_source_files;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;

// ─── Block 1: Struct Definition ───────────────────────────

#[derive(Clone)]
struct SearchFilesCache {
    root: std::path::PathBuf,
    file_count: usize,
    files: Arc<Vec<String>>,
}

impl Default for SearchFilesCache {
    fn default() -> Self {
        Self {
            root: std::path::PathBuf::new(),
            file_count: 0,
            files: Arc::new(Vec::new()),
        }
    }
}

pub struct ContractOrphanAnalyzer {
    search_cache: Mutex<Option<SearchFilesCache>>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IContractOrphanProtocol for ContractOrphanAnalyzer {
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        _file_definitions: &FileDefinitionMap,
        _inheritance_map: &InheritanceMap,
        all_files: &[String],
    ) -> OrphanIndicatorResult {
        let fp = f.value();
        let suffix = file_suffix(fp);

        let content = orphan_io::read_file_safe(fp);
        if content.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Extract ALL trait/interface names from the contract file.
        let trait_names = Self::extract_contract_trait_names(&content);
        if trait_names.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Build search_files: combine scan-directory files with all workspace .rs files (cached).
        let search_files = self.cached_search_files(root_dir, all_files);

        // Check 1: contracts not implemented by expected layer.
        // For each trait, check if it's implemented by the target layer.
        let unimplemented = Self::find_unimplemented_traits(&trait_names, search_files.as_slice());
        if !unimplemented.is_empty() {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::ContractOrphan {
                    suffix: suffix.clone(),
                    trait_name: unimplemented.join(", "),
                    target_layer: "expected",
                    reason: Some(
                        format!(
                            "Contract {} '{}' not implemented by any expected layer file.",
                            suffix,
                            unimplemented.join(", ")
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::MEDIUM,
            );
        }

        // Check 2: protocol not called by any orchestrator, container, capabilities, or surface.
        if suffix == "protocol" {
            let mut called_by_impl_or_user = false;
            for cf in search_files.as_ref() {
                let cb = file_basename(cf);
                let is_orchestrator = cb.starts_with("agent_")
                    && (cb.ends_with("_orchestrator.rs")
                        || cb.ends_with("_orchestrator.py")
                        || cb.ends_with("_orchestrator.ts")
                        || cb.ends_with("_orchestrator.js"));
                let is_container = cb.ends_with("_container.rs")
                    || cb.ends_with("_container.py")
                    || cb.ends_with("_container.ts")
                    || cb.ends_with("_container.js");
                let is_capabilities = cb.starts_with("capabilities_");
                let is_surface = cb.starts_with("surface_");

                if !is_orchestrator && !is_container && !is_capabilities && !is_surface {
                    continue;
                }
                let c = orphan_io::read_file_safe(cf);
                for trait_name in &trait_names {
                    if Self::content_contains_word(&c, trait_name) {
                        called_by_impl_or_user = true;
                        break;
                    }
                }
                if called_by_impl_or_user {
                    break;
                }
            }
            if !called_by_impl_or_user {
                return OrphanIndicatorResult::new(
                    true,
                    AesOrphanViolation::ContractOrphan {
                        suffix: suffix.clone(),
                        trait_name: trait_names.join(", "),
                        target_layer: "orchestrator/container",
                        reason: Some(
                            format!(
                                "Contract {} '{}' not called by any orchestrator or container.",
                                suffix,
                                trait_names.join(", ")
                            )
                            .into(),
                        ),
                    }
                    .to_string(),
                    Severity::MEDIUM,
                );
            }
        }

        // Check 3: aggregate not called by any surface OR container.
        if suffix == "aggregate" {
            let mut called_by_surface_or_container = false;
            for cf in search_files.as_ref() {
                let cb = file_basename(cf);
                let is_surface = cb.starts_with("surface_");
                let is_container = cb.ends_with("_container.rs")
                    || cb.ends_with("_container.py")
                    || cb.ends_with("_container.ts")
                    || cb.ends_with("_container.js");

                if !is_surface && !is_container {
                    continue;
                }
                let c = orphan_io::read_file_safe(cf);
                for trait_name in &trait_names {
                    if Self::content_contains_word(&c, trait_name) {
                        called_by_surface_or_container = true;
                        break;
                    }
                }
                if called_by_surface_or_container {
                    break;
                }
            }
            if !called_by_surface_or_container {
                return OrphanIndicatorResult::new(
                    true,
                    AesOrphanViolation::ContractOrphan {
                        suffix: suffix.clone(),
                        trait_name: trait_names.join(", "),
                        target_layer: "surface/container",
                        reason: Some(
                            format!(
                                "Contract aggregate '{}' not called by any surface or container.",
                                trait_names.join(", ")
                            )
                            .into(),
                        ),
                    }
                    .to_string(),
                    Severity::MEDIUM,
                );
            }
        }

        OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for ContractOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractOrphanAnalyzer {
    pub fn new() -> Self {
        Self {
            search_cache: Mutex::new(None),
        }
    }

    /// Check if `text` contains `word` as a whole word (not as a substring).
    fn content_contains_word(text: &str, word: &str) -> bool {
        text.split(|c: char| !c.is_alphanumeric() && c != '_')
            .any(|w| w == word)
    }

    fn cached_search_files(&self, root_dir: &FilePath, all_files: &[String]) -> Arc<Vec<String>> {
        let root = std::path::Path::new(root_dir.value()).to_path_buf();
        let top_root =
            shared::orphan_detector::utility_workspace_scanner::find_workspace_root(&root)
                .unwrap_or_else(|_| root.clone());
        if let Ok(mut guard) = self.search_cache.lock() {
            if let Some(cache) = guard.as_ref() {
                if cache.root == top_root && cache.file_count == all_files.len() {
                    return cache.files.clone();
                }
            }
            let mut search_files: Vec<String> = all_files.to_vec();
            for ws_dir in &["crates", "packages", "modules"] {
                let ws_path = top_root.join(ws_dir);
                if ws_path.exists() {
                    collect_source_files(&ws_path, &mut search_files);
                }
            }
            let files = Arc::new(search_files);
            *guard = Some(SearchFilesCache {
                root: top_root,
                file_count: all_files.len(),
                files: files.clone(),
            });
            files
        } else {
            Arc::new(all_files.to_vec())
        }
    }

    fn re_contract_rust() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"(?:pub\s+)?trait\s+([A-Za-z0-9_]+)").ok())
            .as_ref()
    }

    fn re_contract_py() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"class\s+([A-Za-z0-9_]+)").ok())
            .as_ref()
    }

    fn re_ts_interface_export() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"export\s+interface\s+([A-Za-z0-9_]+)").ok())
            .as_ref()
    }

    fn re_interface() -> Option<&'static Regex> {
        static RE: OnceLock<Option<Regex>> = OnceLock::new();
        RE.get_or_init(|| Regex::new(r"interface\s+([A-Za-z0-9_]+)").ok())
            .as_ref()
    }

    /// Extract ALL trait/interface names from contract file content.
    /// Uses captures_iter to find multiple matches instead of just the first.
    fn extract_contract_trait_names(content: &str) -> Vec<String> {
        let code_lines: String = content
            .lines()
            .filter(|l| {
                let t = l.trim();
                !t.starts_with("//") && !t.starts_with("/*") && !t.starts_with("*")
            })
            .collect::<Vec<_>>()
            .join("\n");

        let mut traits = Vec::new();

        if let Some(re) = Self::re_contract_rust() {
            for caps in re.captures_iter(&code_lines) {
                traits.push(caps[1].to_string());
            }
        }
        if let Some(re) = Self::re_ts_interface_export() {
            for caps in re.captures_iter(&code_lines) {
                traits.push(caps[1].to_string());
            }
        }
        if let Some(re) = Self::re_interface() {
            for caps in re.captures_iter(&code_lines) {
                traits.push(caps[1].to_string());
            }
        }
        if let Some(re) = Self::re_contract_py() {
            for caps in re.captures_iter(&code_lines) {
                traits.push(caps[1].to_string());
            }
        }

        traits.sort();
        traits.dedup();
        traits
    }

    /// Check which traits are NOT implemented by any expected layer file.
    fn find_unimplemented_traits(trait_names: &[String], search_files: &[String]) -> Vec<String> {
        trait_names
            .iter()
            .filter(|trait_name| !Self::has_trait_implementation(search_files, trait_name))
            .cloned()
            .collect()
    }

    /// Check if any file in the search list implements the given trait.
    fn has_trait_implementation(search_files: &[String], trait_name: &str) -> bool {
        for cf in search_files {
            let c = orphan_io::read_file_safe(cf);
            if Self::check_trait_impl(&c, trait_name) {
                return true;
            }
        }
        false
    }

    /// Check if content contains an implementation of the given trait.
    fn check_trait_impl(content: &str, trait_name: &str) -> bool {
        for line in content.lines() {
            let trimmed = line.trim();
            // Skip comment lines
            if trimmed.starts_with("//")
                || trimmed.starts_with("/*")
                || trimmed.starts_with('*')
                || trimmed.starts_with("#")
            {
                continue;
            }

            // Rust: impl Trait for Type / impl<T> Trait for Type
            if trimmed.starts_with("impl")
                && trimmed.contains(" for ")
                && Self::content_contains_word(trimmed, trait_name)
            {
                return true;
            }

            // Python: class Foo(Trait): / class Foo(Base, Trait):
            if let Some(class_pos) = trimmed.find("class ") {
                let after_class = &trimmed[class_pos + 6..];
                if let Some(paren_pos) = after_class.find('(') {
                    let bases = &after_class[paren_pos + 1..];
                    if let Some(paren_end) = bases.find(')') {
                        let base_list = &bases[..paren_end];
                        for base in base_list.split(',') {
                            let cleaned = base.trim();
                            if cleaned == trait_name {
                                return true;
                            }
                        }
                    }
                }
            }

            // TS: class Foo implements Trait
            if let Some(impl_pos) = trimmed.find(" implements ") {
                let after_impl = &trimmed[impl_pos + 12..];
                for implemented in after_impl.split(',').map(|s| s.trim()) {
                    if implemented == trait_name {
                        return true;
                    }
                }
            }
        }
        false
    }
}
```

---

## File: crates/orphan-detector/src/capabilities_orphan_graph_resolver.rs

```rust
// PURPOSE: OrphanGraphResolver — build graph context and identify entry points for orphan analysis.
use shared::code_analysis::{
    FileDefinitionMap, GraphAnalysisContext, ImportGraph, InboundLinkMap, InheritanceMap,
};

use crate::utility_orphan_graph_resolver;
use crate::utility_orphan_regex_patterns;
use shared::orphan_detector::utility_orphan_filename::file_stem;
use shared::orphan_detector::utility_orphan_io;
use shared::orphan_detector::IOrphanGraphResolverProtocol;
use shared::orphan_detector::{OrphanEntryPatternListVO, OrphanFileListVO};
use std::collections::{HashMap, HashSet};

// ─── Block 1: Struct Definition ───────────────────────────

/// Build graph context and identify entry points for orphan analysis.
pub struct OrphanGraphResolver {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IOrphanGraphResolverProtocol for OrphanGraphResolver {
    fn build_graph_context(
        &self,
        files: &[OrphanFileListVO],
        root_dir: &str,
    ) -> GraphAnalysisContext {
        // Bridge the contract-level VO collection to the internal helper
        // which still uses raw `&[String]` for backward compatibility with
        // the rest of the orphan-detector graph builder.
        let raw_paths: Vec<String> = files
            .iter()
            .flat_map(|v| v.values.iter().cloned())
            .collect();
        self.build_graph_context_inner(&raw_paths, root_dir)
    }

    fn identify_entry_points(
        &self,
        files: &[OrphanFileListVO],
        configured: &[OrphanEntryPatternListVO],
    ) -> OrphanFileListVO {
        let file_strs: Vec<String> = files
            .iter()
            .flat_map(|v| v.values.iter().cloned())
            .collect();

        let configured_strs: Vec<String> = configured
            .iter()
            .flat_map(|p| p.values.iter().cloned())
            .collect();

        let mut matched: Vec<String> = if configured_strs.is_empty() {
            file_strs
                .iter()
                .filter(|f| {
                    let basename = f.rsplit('/').next().unwrap_or(f);
                    basename.ends_with("_container.rs")
                        || basename.ends_with("_container.py")
                        || basename.ends_with("_container.ts")
                        || basename.ends_with("_container.js")
                        || basename.ends_with("_entry.rs")
                        || basename.ends_with("_entry.py")
                        || basename.ends_with("_entry.ts")
                        || basename.ends_with("_entry.js")
                        || basename.starts_with("root_")
                        || basename == "main.rs"
                        || basename == "lib.rs"
                        || basename == "main.py"
                        || basename == "__main__.py"
                        || basename == "main.ts"
                        || basename == "main.js"
                        || basename == "index.ts"
                        || basename == "index.js"
                })
                .cloned()
                .collect()
        } else {
            file_strs
                .iter()
                .filter(|f| {
                    let basename = f.rsplit('/').next().unwrap_or(f);
                    // Bug 5 fix: Use exact/prefix/suffix match instead of contains()
                    // prevents false positives like "germanic_utils" matching "main"
                    let stem =
                        shared::orphan_detector::utility_orphan_filename::file_stem(basename);
                    configured_strs.iter().any(|pattern| {
                        basename == pattern
                            || basename.ends_with(pattern)
                            || stem == *pattern
                            || stem.starts_with(pattern.as_str())
                            || stem.ends_with(pattern.as_str())
                    })
                })
                .cloned()
                .collect()
        };
        matched.sort();
        matched.dedup();
        OrphanFileListVO::new(matched)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for OrphanGraphResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl OrphanGraphResolver {
    pub fn new() -> Self {
        Self {}
    }

    fn build_graph_context_inner(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext {
        let mut import_graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut inbound_links: HashMap<String, Vec<String>> = HashMap::new();
        let mut inheritance_map: HashMap<String, Vec<String>> = HashMap::new();
        let file_definitions: HashMap<String, Vec<String>> = HashMap::new();

        // Bug 11 fix: Use workspace_root consistently instead of root_dir for path resolution
        let workspace_root = utility_orphan_graph_resolver::find_workspace_root(root_dir);
        let root_path = std::path::Path::new(&workspace_root);

        // Build set of known workspace crate dirs for external dep detection
        let mut workspace_modules: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        // Perf 10: Pre-compute crate_name -> src_dir map
        let mut crate_src_dirs: HashMap<String, std::path::PathBuf> = HashMap::new();
        for ws_dir in &["crates", "packages", "modules"] {
            let ws_path = root_path.join(ws_dir);
            if shared::orphan_detector::utility_orphan_io::is_dir(&ws_path) {
                let entries = shared::orphan_detector::utility_orphan_io::scan_directory(&ws_path);
                for (name, path_str, is_dir_entry) in entries {
                    if !is_dir_entry {
                        continue;
                    }
                    workspace_modules.insert(name.clone());
                    workspace_modules.insert(name.replace('-', "_"));
                    let src_dir = std::path::PathBuf::from(&path_str).join("src");
                    if shared::orphan_detector::utility_orphan_io::is_dir(&src_dir) {
                        crate_src_dirs.insert(name.clone(), src_dir.clone());
                        crate_src_dirs.insert(name.replace('-', "_"), src_dir);
                    }
                }
            }
        }

        // Build crate module index for hyphen-aware resolution
        let crate_module_index =
            utility_orphan_graph_resolver::build_crate_module_index(&crate_src_dirs);

        // Expand files to include all workspace source files for cross-crate import resolution
        // This ensures that when scanning a subfolder, imports from other crates are visible
        let mut all_workspace_files: Vec<String> = files.to_vec();
        let mut seen: HashSet<String> = files.iter().cloned().collect();
        let root_path_obj = std::path::Path::new(&workspace_root);
        for src_dir in crate_src_dirs.values() {
            let workspace_files =
                shared::orphan_detector::utility_orphan_io::scan_directory_recursive(src_dir);
            for f in workspace_files {
                // Normalize to relative path for consistency with initial files
                let rel = std::path::Path::new(&f)
                    .strip_prefix(root_path_obj)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or(f);
                if seen.insert(rel.clone()) {
                    all_workspace_files.push(rel);
                }
            }
        }
        // Also scan root_*.rs files directly in crates/ directory (not in src/)
        for ws_dir in &["crates", "packages", "modules"] {
            let ws_path = root_path.join(ws_dir);
            if shared::orphan_detector::utility_orphan_io::is_dir(&ws_path) {
                let entries = shared::orphan_detector::utility_orphan_io::scan_directory(&ws_path);
                for (name, path_str, is_dir_entry) in entries {
                    if is_dir_entry {
                        continue; // Skip directories, we already scanned their src/
                    }
                    // Include root_*.rs files directly in crates/
                    if name.starts_with("root_")
                        && (name.ends_with(".rs")
                            || name.ends_with(".py")
                            || name.ends_with(".ts")
                            || name.ends_with(".js"))
                        && !seen.contains(&path_str)
                    {
                        let rel = std::path::Path::new(&path_str)
                            .strip_prefix(root_path_obj)
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or(path_str.clone());
                        seen.insert(rel.clone());
                        all_workspace_files.push(rel);
                    }
                }
            }
        }
        let files = &all_workspace_files;

        // Build a lookup: module_name -> file_path for crate:: resolution
        let mut module_to_file: HashMap<String, String> = HashMap::new();
        for f in files {
            let stem = file_stem(f);
            // Bug 3 fix: Use first-write-wins to prevent HashMap collision
            // when multiple crates have files with the same stem (e.g., helper.rs)
            if !module_to_file.contains_key(&stem) {
                module_to_file.insert(stem.clone(), f.clone());
            }
            if let Some(parent) = f.rsplit('/').nth(1) {
                let module_path = format!("{}/{}", parent, stem);
                if !module_to_file.contains_key(&module_path) {
                    module_to_file.insert(module_path.clone(), f.clone());
                    let normalized_path = module_path.replace('-', "_");
                    if normalized_path != module_path
                        && !module_to_file.contains_key(&normalized_path)
                    {
                        module_to_file.insert(normalized_path, f.clone());
                    }
                }
            }
            // Bug 3 fix: mod.rs -> map by parent directory name (first-write-wins)
            if stem == "mod" {
                if let Some(parent_dir) = f.rsplit('/').nth(1) {
                    if !module_to_file.contains_key(parent_dir) {
                        module_to_file.insert(parent_dir.to_string(), f.clone());
                        let normalized = parent_dir.replace('-', "_");
                        if normalized != parent_dir && !module_to_file.contains_key(&normalized) {
                            module_to_file.insert(normalized, f.clone());
                        }
                    }
                    if let Some(grandparent) = f.rsplit('/').nth(2) {
                        let composite = format!("{}/{}", grandparent, parent_dir);
                        if !module_to_file.contains_key(&composite) {
                            module_to_file.insert(composite.clone(), f.clone());
                            let normalized_composite = composite.replace('-', "_");
                            if normalized_composite != composite
                                && !module_to_file.contains_key(&normalized_composite)
                            {
                                module_to_file.insert(normalized_composite, f.clone());
                            }
                        }
                    }
                }
            }
            // Python __init__.py -> map by parent directory name (same as mod.rs for Python packages)
            if stem == "__init__" {
                if let Some(parent_dir) = f.rsplit('/').nth(1) {
                    if !module_to_file.contains_key(parent_dir) {
                        module_to_file.insert(parent_dir.to_string(), f.clone());
                        let normalized = parent_dir.replace('-', "_");
                        if normalized != parent_dir && !module_to_file.contains_key(&normalized) {
                            module_to_file.insert(normalized, f.clone());
                        }
                    }
                    if let Some(grandparent) = f.rsplit('/').nth(2) {
                        let composite = format!("{}/{}", grandparent, parent_dir);
                        if !module_to_file.contains_key(&composite) {
                            module_to_file.insert(composite.clone(), f.clone());
                            let normalized_composite = composite.replace('-', "_");
                            if normalized_composite != composite
                                && !module_to_file.contains_key(&normalized_composite)
                            {
                                module_to_file.insert(normalized_composite, f.clone());
                            }
                        }
                    }
                }
            }
        }

        // Perf 8: Single-pass file reading
        for f in files {
            import_graph.entry(f.clone()).or_default();
            let content = utility_orphan_io::read_file_safe(f);
            if content.is_empty()
                && !shared::orphan_detector::utility_orphan_io::is_file(&std::path::PathBuf::from(
                    f,
                ))
            {
                continue;
            }

            // Pass 1: #[path = "..."] pub mod (Bug 14 fix — link only the referenced file)
            if let Some(re) = utility_orphan_regex_patterns::pub_mod_path_re() {
                for cap in re.captures_iter(&content) {
                    let mod_path = cap[1].to_string();
                    let base_dir = match std::path::Path::new(f).parent() {
                        Some(p) => p.to_path_buf(),
                        None => continue,
                    };
                    // Bug 11 fix: Use workspace_root consistently for path resolution
                    let root_path = std::path::Path::new(&workspace_root);
                    let Some(resolved_path) =
                        shared::orphan_detector::utility_orphan_path::resolve_module_path(
                            root_path, &base_dir, &mod_path,
                        )
                    else {
                        continue;
                    };
                    let resolved = resolved_path.to_string_lossy().to_string();
                    if shared::orphan_detector::utility_orphan_io::is_file(
                        &std::path::PathBuf::from(&resolved),
                    ) && resolved != *f
                    {
                        import_graph
                            .entry(f.clone())
                            .or_default()
                            .push(resolved.clone());
                        inbound_links.entry(resolved).or_default().push(f.clone());
                    }
                }
            }

            // Pass 2: plain mod (Bug 10 fix)
            if let Some(re) = utility_orphan_regex_patterns::plain_mod_re() {
                for cap in re.captures_iter(&content) {
                    let mod_name = cap[1].to_string();
                    let parent = match std::path::Path::new(f).parent() {
                        Some(p) => p,
                        None => continue,
                    };
                    let candidates = [
                        parent.join(format!("{}.rs", mod_name)),
                        parent.join(&mod_name).join("mod.rs"),
                        parent.join(format!("{}.py", mod_name)),
                        parent.join(&mod_name).join("__init__.py"),
                    ];
                    for candidate in &candidates {
                        if shared::orphan_detector::utility_orphan_io::is_file(candidate) {
                            if let Some(path_str) = candidate.to_str() {
                                let resolved = path_str.to_string();
                                if resolved != *f {
                                    import_graph
                                        .entry(f.clone())
                                        .or_default()
                                        .push(resolved.clone());
                                    inbound_links.entry(resolved).or_default().push(f.clone());
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            // Pass 3: use/import/from
            let Some(import_re) = utility_orphan_regex_patterns::import_re() else {
                continue;
            };
            for cap in import_re.captures_iter(&content) {
                let full_import = cap[1].to_string();
                let cap_end = cap.get(0).map(|m| m.end()).unwrap_or(0);

                // Handle crate:: and lint_arwaky:: imports
                let normalized = if let Some(stripped) = full_import.strip_prefix("lint_arwaky::") {
                    format!("crate::{}", stripped)
                } else {
                    full_import.clone()
                };
                let full_import = &normalized;
                if let Some(path_part) = full_import.strip_prefix("crate::") {
                    let segments: Vec<&str> = path_part.split("::").collect();
                    if segments.len() >= 2 {
                        let mut resolved = false;
                        for i in (1..segments.len()).rev() {
                            let composite = segments[..i].join("/");
                            if let Some(file_path) = module_to_file.get(composite.as_str()) {
                                if file_path != f {
                                    import_graph
                                        .entry(f.clone())
                                        .or_default()
                                        .push(file_path.clone());
                                    inbound_links
                                        .entry(file_path.clone())
                                        .or_default()
                                        .push(f.clone());
                                    resolved = true;
                                    break;
                                }
                            }
                        }
                        if resolved {
                            continue;
                        }
                        for seg in segments[..segments.len() - 1].iter().rev() {
                            if let Some(file_path) = module_to_file.get(*seg) {
                                if file_path != f {
                                    import_graph
                                        .entry(f.clone())
                                        .or_default()
                                        .push(file_path.clone());
                                    inbound_links
                                        .entry(file_path.clone())
                                        .or_default()
                                        .push(f.clone());
                                    resolved = true;
                                    break;
                                }
                            }
                        }
                        if resolved {
                            continue;
                        }
                    }
                    if let Some(seg) = segments.first() {
                        if let Some(file_path) = module_to_file.get(*seg) {
                            if file_path != f {
                                import_graph
                                    .entry(f.clone())
                                    .or_default()
                                    .push(file_path.clone());
                                inbound_links
                                    .entry(file_path.clone())
                                    .or_default()
                                    .push(f.clone());
                                continue;
                            }
                        }
                    }
                    continue;
                }

                if let Some(path_part) = full_import.strip_prefix("super::") {
                    let segments: Vec<&str> = path_part.split("::").collect();
                    if segments.len() >= 2 {
                        let mut found = false;
                        for i in (1..segments.len()).rev() {
                            let composite = segments[..i].join("/");
                            if let Some(file_path) = module_to_file.get(composite.as_str()) {
                                if file_path != f {
                                    import_graph
                                        .entry(f.clone())
                                        .or_default()
                                        .push(file_path.clone());
                                    inbound_links
                                        .entry(file_path.clone())
                                        .or_default()
                                        .push(f.clone());
                                    found = true;
                                    break;
                                }
                            }
                        }
                        if found {
                            continue;
                        }
                        for seg in segments[..segments.len() - 1].iter().rev() {
                            if let Some(resolved) = module_to_file.get(*seg) {
                                if resolved != f {
                                    import_graph
                                        .entry(f.clone())
                                        .or_default()
                                        .push(resolved.clone());
                                    inbound_links
                                        .entry(resolved.clone())
                                        .or_default()
                                        .push(f.clone());
                                    break;
                                }
                            }
                        }
                    } else if let Some(seg) = segments.first() {
                        if let Some(resolved) = module_to_file.get(*seg) {
                            if resolved != f {
                                import_graph
                                    .entry(f.clone())
                                    .or_default()
                                    .push(resolved.clone());
                                inbound_links
                                    .entry(resolved.clone())
                                    .or_default()
                                    .push(f.clone());
                            }
                        }
                    }
                    continue;
                }

                let mut dep = full_import.clone();
                if let Some(dot) = dep.find('.') {
                    dep = dep[..dot].to_string();
                }
                if let Some(colon) = dep.find("::") {
                    dep = dep[..colon].to_string();
                }
                let is_known_local = module_to_file.contains_key(&dep)
                    || (workspace_modules.contains(&dep) && !full_import.contains('.'))
                    || matches!(dep.as_str(), "crate" | "self" | "super");
                if !is_known_local {
                    // Python dotted absolute paths (e.g., modules.cli.src, modules.shared.src.asset)
                    if full_import.contains('.') {
                        // Step 1: Walk dotted path as directories from workspace root
                        // e.g., "modules.cli.src" → <workspace>/modules/cli/src/__init__.py
                        let segments: Vec<&str> = full_import.split('.').collect();
                        let mut walk_dir = std::path::PathBuf::from(&workspace_root);
                        let mut walk_ok = true;
                        for seg in &segments {
                            walk_dir = walk_dir.join(seg);
                            if !shared::orphan_detector::utility_orphan_io::is_dir(&walk_dir) {
                                walk_ok = false;
                                break;
                            }
                        }
                        if walk_ok {
                            // Look for __init__.py or mod.rs in the resolved directory
                            for marker in &["__init__.py", "mod.rs", "index.ts", "index.js"] {
                                let candidate = walk_dir.join(marker);
                                if shared::orphan_detector::utility_orphan_io::is_file(&candidate) {
                                    // Convert absolute path to relative (matching graph convention)
                                    let cand_rel = candidate
                                        .strip_prefix(root_path)
                                        .map(|p| p.to_string_lossy().to_string())
                                        .unwrap_or_else(|_| {
                                            candidate.to_string_lossy().to_string()
                                        });
                                    if cand_rel != *f {
                                        utility_orphan_graph_resolver::add_edge(
                                            &mut import_graph,
                                            &mut inbound_links,
                                            f,
                                            &cand_rel,
                                        );
                                        break;
                                    }
                                }
                            }
                        } else {
                            // Step 2: Try last segment as filename stem
                            if let Some(last_seg) = full_import.rsplit('.').next() {
                                if let Some(target) = module_to_file.get(last_seg) {
                                    if target != f {
                                        utility_orphan_graph_resolver::add_edge(
                                            &mut import_graph,
                                            &mut inbound_links,
                                            f,
                                            target,
                                        );
                                    }
                                }
                            }
                        }

                        // Step 3: Always resolve individual names from `from X import (Y, Z)`
                        // Limit search to current line to avoid matching "import" in comments
                        let line_end = content[cap_end..]
                            .find('\n')
                            .map(|p| cap_end + p)
                            .unwrap_or(content.len());
                        if let Some(import_pos) = content[cap_end..line_end].find("import") {
                            let stmt_start = cap_end + import_pos + 6; // skip "import"
                            let stmt_end = content[stmt_start..]
                                .find('\n')
                                .map(|p| stmt_start + p)
                                .unwrap_or(content.len());
                            let stmt_slice = &content[stmt_start..stmt_end];

                            let names: Vec<&str> = if stmt_slice.contains('(') {
                                // Multi-line: collect from rest of content until ')'
                                let after_paren = &content[stmt_start..];
                                if let Some(close) = after_paren.find(')') {
                                    after_paren[..close]
                                        .split(|c: char| c == ',' || c.is_whitespace())
                                        .map(|s| s.trim())
                                        .filter(|s| {
                                            !s.is_empty()
                                                && s.chars()
                                                    .all(|c| c.is_alphanumeric() || c == '_')
                                        })
                                        .collect()
                                } else {
                                    vec![]
                                }
                            } else {
                                // Single-line: `import Y, Z`
                                stmt_slice
                                    .split(',')
                                    .map(|s| s.trim())
                                    .filter(|s| {
                                        !s.is_empty()
                                            && s.chars().all(|c| c.is_alphanumeric() || c == '_')
                                    })
                                    .collect()
                            };

                            for name in names {
                                // Try stem lookup first
                                if let Some(target) = module_to_file.get(name) {
                                    if target != f {
                                        utility_orphan_graph_resolver::add_edge(
                                            &mut import_graph,
                                            &mut inbound_links,
                                            f,
                                            target,
                                        );
                                        continue;
                                    }
                                }
                                // Try composite dotted path → last segment
                                let full_name_path = format!("{}.{}", full_import, name);
                                for seg in full_name_path.rsplit('.') {
                                    if let Some(target) = module_to_file.get(seg) {
                                        if target != f {
                                            utility_orphan_graph_resolver::add_edge(
                                                &mut import_graph,
                                                &mut inbound_links,
                                                f,
                                                target,
                                            );
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    continue;
                }

                // Workspace crate import resolution using pre-computed crate_src_dirs (Perf 10)
                if let Some(colon_idx) = full_import.find("::") {
                    let crate_name = &full_import[..colon_idx];
                    let rest = &full_import[colon_idx + 2..];
                    let import_list: Vec<String> = if let Some(open_brace) = rest.find('{') {
                        let prefix = &rest[..open_brace];
                        let inner = &rest[open_brace + 1..];
                        let close_brace = inner.rfind('}').unwrap_or(inner.len());
                        let items = inner[..close_brace]
                            .split(',')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty());
                        items.map(|item| format!("{}{}", prefix, item)).collect()
                    } else {
                        vec![rest.to_string()]
                    };
                    for import_path in &import_list {
                        let segments: Vec<&str> = import_path.split("::").collect();
                        if segments.is_empty() {
                            continue;
                        }
                        let module_name = segments[0];
                        if let Some(resolved) =
                            utility_orphan_graph_resolver::resolve_workspace_module(
                                &crate_module_index,
                                crate_name,
                                &segments,
                                f,
                            )
                        {
                            utility_orphan_graph_resolver::add_edge(
                                &mut import_graph,
                                &mut inbound_links,
                                f,
                                &resolved,
                            );
                            continue;
                        }
                        if let Some(src_dir) = crate_src_dirs.get(crate_name) {
                            let entries =
                                shared::orphan_detector::utility_orphan_io::scan_directory(src_dir);
                            for (_name, path_str, _is_dir) in entries {
                                let path = std::path::PathBuf::from(&path_str);
                                let stem = path
                                    .file_stem()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or_default();
                                let normalized_stem =
                                    shared::orphan_detector::utility_orphan_detector::normalize_module_component(stem);
                                if (stem == module_name || normalized_stem == module_name)
                                    && path_str != *f
                                {
                                    import_graph
                                        .entry(f.clone())
                                        .or_default()
                                        .push(path_str.to_string());
                                    inbound_links
                                        .entry(path_str.to_string())
                                        .or_default()
                                        .push(f.clone());
                                }
                            }
                        }
                    }
                    continue;
                }

                // Bug 7 fix: Only add edge if dep exists in module_to_file or is known external
                // prevents non-file nodes (e.g., "os", "serde") from corrupting the graph
                let is_external_dep =
                    !module_to_file.contains_key(&dep) && !workspace_modules.contains(&dep);
                if is_external_dep {
                    // Skip unknown external deps — don't add ghost nodes to graph
                    continue;
                }
                // Only add edge for known local modules
                if module_to_file.contains_key(&dep) {
                    import_graph.entry(f.clone()).or_default().push(dep.clone());
                    inbound_links.entry(dep).or_default().push(f.clone());
                }
            }

            // Pass 3c: TypeScript/JavaScript imports
            if f.ends_with(".ts") || f.ends_with(".js") {
                if let Some(ts_import_re) = utility_orphan_regex_patterns::ts_import_re() {
                    for cap in ts_import_re.captures_iter(&content) {
                        // Try to extract import path from groups:
                        // Group 2 = named import path `import { X } from './path'`
                        // Group 5 = default+named path `import Def, { X } from './path'`
                        // Group 7 = default import path `import X from './path'`
                        // Group 9 = namespace import path `import * as X from './path'`
                        // Group 10 = side-effect import path `import './path'`
                        let import_path = cap
                            .get(2)
                            .or_else(|| cap.get(5))
                            .or_else(|| cap.get(7))
                            .or_else(|| cap.get(9))
                            .or_else(|| cap.get(10));

                        if let Some(path_m) = import_path {
                            let raw = path_m.as_str();
                            // Only resolve relative imports (start with '.')
                            // Package imports (e.g., 'react', 'lodash') are external
                            if raw.starts_with('.') {
                                if let Some(resolved) =
                                    utility_orphan_graph_resolver::resolve_ts_relative(
                                        f, raw, root_path,
                                    )
                                {
                                    utility_orphan_graph_resolver::add_edge(
                                        &mut import_graph,
                                        &mut inbound_links,
                                        f,
                                        &resolved,
                                    );
                                }
                            }
                        }
                    }
                }

                // Pass 3d: TypeScript/JavaScript export re-exports
                if let Some(ts_export_re) = utility_orphan_regex_patterns::ts_export_re() {
                    for cap in ts_export_re.captures_iter(&content) {
                        // Group 2 = named export path `export { X } from './path'`
                        // Group 3 = re-export all path `export * from './path'`
                        let import_path = cap.get(2).or_else(|| cap.get(3));

                        if let Some(path_m) = import_path {
                            let raw = path_m.as_str();
                            if raw.starts_with('.') {
                                if let Some(resolved) =
                                    utility_orphan_graph_resolver::resolve_ts_relative(
                                        f, raw, root_path,
                                    )
                                {
                                    utility_orphan_graph_resolver::add_edge(
                                        &mut import_graph,
                                        &mut inbound_links,
                                        f,
                                        &resolved,
                                    );
                                }
                            }
                        }
                    }
                }
            }

            // Pass 3b: Python relative imports (`from . import X`, `from .module import X`)
            if let Some(rel_re) = utility_orphan_regex_patterns::python_relative_import_re() {
                for cap in rel_re.captures_iter(&content) {
                    // Group 1 = dots (parenthesized variant), Group 3 = dots (inline variant)
                    let dots = cap.get(1).or_else(|| cap.get(3));
                    // Group 2 = names (parenthesized variant), Group 4 = names (inline variant)
                    let names_raw = cap.get(2).or_else(|| cap.get(4));
                    let (Some(dots_m), Some(names_m)) = (dots, names_raw) else {
                        continue;
                    };
                    let dot_count = dots_m.as_str().len(); // 1=., 2=.., 3=...
                    let names_str = names_m.as_str();

                    // Resolve base directory: . = current file dir, .. = parent, ... = grandparent
                    let file_path = std::path::Path::new(f);
                    let mut base_dir = file_path.parent().map(|p| p.to_path_buf());
                    for _ in 1..dot_count {
                        if let Some(ref dir) = base_dir {
                            base_dir = dir.parent().map(|p| p.to_path_buf());
                        }
                    }
                    let Some(base) = base_dir else { continue };

                    // Parse imported names
                    let names: Vec<&str> = names_str
                        .split(|c: char| c == ',' || c == '(' || c == ')' || c.is_whitespace())
                        .map(|s| s.trim())
                        .filter(|s| {
                            !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || c == '_')
                        })
                        .collect();

                    for name in names {
                        // Try module_to_file lookup (stem match)
                        if let Some(target) = module_to_file.get(name) {
                            if target != f {
                                utility_orphan_graph_resolver::add_edge(
                                    &mut import_graph,
                                    &mut inbound_links,
                                    f,
                                    target,
                                );
                                continue;
                            }
                        }
                        // Try resolving as file in base directory
                        for ext in &[".py", ".rs", ".ts", ".js"] {
                            let candidate = base.join(format!("{}{}", name, ext));
                            if shared::orphan_detector::utility_orphan_io::is_file(&candidate) {
                                // Normalize to relative path for consistent comparison with f
                                let cand_rel = candidate
                                    .strip_prefix(root_path)
                                    .map(|p| p.to_string_lossy().to_string())
                                    .unwrap_or_else(|_| candidate.to_string_lossy().to_string());
                                if cand_rel != *f {
                                    utility_orphan_graph_resolver::add_edge(
                                        &mut import_graph,
                                        &mut inbound_links,
                                        f,
                                        &cand_rel,
                                    );
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            // Pass 3c: TypeScript/JavaScript imports
            if f.ends_with(".ts")
                || f.ends_with(".js")
                || f.ends_with(".tsx")
                || f.ends_with(".jsx")
            {
                if let Some(ts_re) = utility_orphan_regex_patterns::ts_import_re() {
                    for cap in ts_re.captures_iter(&content) {
                        // Group 2 = named path, 5 = default+named path, 7 = default path,
                        // 9 = namespace path, 11 = side-effect path
                        let import_path = cap
                            .get(2)
                            .or_else(|| cap.get(5))
                            .or_else(|| cap.get(7))
                            .or_else(|| cap.get(9))
                            .or_else(|| cap.get(11));
                        if let Some(path_m) = import_path {
                            let raw = path_m.as_str();
                            if raw.starts_with('.') {
                                if let Some(resolved) =
                                    utility_orphan_graph_resolver::resolve_ts_relative(
                                        f, raw, root_path,
                                    )
                                {
                                    utility_orphan_graph_resolver::add_edge(
                                        &mut import_graph,
                                        &mut inbound_links,
                                        f,
                                        &resolved,
                                    );
                                }
                            }
                        }
                    }
                }
                if let Some(ts_re) = utility_orphan_regex_patterns::ts_export_re() {
                    for cap in ts_re.captures_iter(&content) {
                        // Group 2 = named export path, 3 = wildcard export path
                        let export_path = cap.get(2).or_else(|| cap.get(3));
                        if let Some(path_m) = export_path {
                            let raw = path_m.as_str();
                            if raw.starts_with('.') {
                                if let Some(resolved) =
                                    utility_orphan_graph_resolver::resolve_ts_relative(
                                        f, raw, root_path,
                                    )
                                {
                                    utility_orphan_graph_resolver::add_edge(
                                        &mut import_graph,
                                        &mut inbound_links,
                                        f,
                                        &resolved,
                                    );
                                }
                            }
                        }
                    }
                }
            }

            // Bug 12 fix: Only run class inheritance detection for Python files (.py)
            // prevents false matches in Rust/JS/TS files
            if f.ends_with(".py") {
                // Pass 4: Python class inheritance
                if let Some(re) = utility_orphan_regex_patterns::inh_re() {
                    for cap in re.captures_iter(&content) {
                        for base in cap[1].split(',') {
                            inheritance_map
                                .entry(f.clone())
                                .or_default()
                                .push(base.trim().to_string());
                        }
                    }
                }
            }

            // Pass 5: pub use re-exports (e.g. `pub use crate::common::taxonomy_action_vo;`)
            if let Some(re) = utility_orphan_regex_patterns::pub_use_re() {
                for cap in re.captures_iter(&content) {
                    let path = &cap[1]; // e.g. common::taxonomy_action_vo
                    let segments: Vec<&str> = path.split("::").collect();

                    // Try to resolve the re-exported module to a file
                    // First try the last segment (file stem)
                    if let Some(seg) = segments.last() {
                        if let Some(file_path) = module_to_file.get(*seg) {
                            if file_path != f {
                                utility_orphan_graph_resolver::add_edge(
                                    &mut import_graph,
                                    &mut inbound_links,
                                    f,
                                    file_path,
                                );
                            }
                        }
                    }
                    // Also try composite path with / separator
                    if segments.len() >= 2 {
                        let composite = segments.join("/");
                        if let Some(file_path) = module_to_file.get(composite.as_str()) {
                            if file_path != f {
                                utility_orphan_graph_resolver::add_edge(
                                    &mut import_graph,
                                    &mut inbound_links,
                                    f,
                                    file_path,
                                );
                            }
                        }
                    }
                }
            }

            // Pass 5b: pub use relative re-exports (e.g. `pub use taxonomy_language_vo::LanguageVO;`)
            if let Some(re) = utility_orphan_regex_patterns::pub_use_relative_re() {
                for cap in re.captures_iter(&content) {
                    let path = &cap[1]; // e.g. taxonomy_language_vo::LanguageVO
                                        // Skip prefixes already handled by pub_use_re (Pass 5)
                    if path.starts_with("crate::")
                        || path.starts_with("super::")
                        || path.starts_with("self::")
                        || path == "crate"
                        || path == "super"
                        || path == "self"
                    {
                        continue;
                    }
                    let segments: Vec<&str> = path.split("::").collect();

                    // Try to resolve the re-exported module to a file
                    if let Some(seg) = segments.first() {
                        if let Some(file_path) = module_to_file.get(*seg) {
                            if file_path != f {
                                utility_orphan_graph_resolver::add_edge(
                                    &mut import_graph,
                                    &mut inbound_links,
                                    f,
                                    file_path,
                                );
                            }
                        }
                    }
                }
            }
        }

        // Bug 9 fix: Deduplicate edges to prevent inflating inbound link count
        utility_orphan_graph_resolver::dedup_edges(&mut import_graph);
        utility_orphan_graph_resolver::dedup_edges(&mut inbound_links);

        GraphAnalysisContext::new(
            ImportGraph::new(import_graph),
            InboundLinkMap::new(inbound_links),
            InheritanceMap::new(inheritance_map),
            FileDefinitionMap::new(file_definitions),
        )
    }
}
```

---

## File: crates/orphan-detector/src/capabilities_orphan_surfaces_analyzer.rs

```rust
// PURPOSE: SurfacesOrphanAnalyzer — ISurfacesOrphanProtocol for orphan surface detection
use shared::code_analysis::{OrphanIndicatorResult, ReachabilityResult};

use shared::common::{FilePath, Severity};

use shared::orphan_detector::{AesOrphanViolation, ISurfacesOrphanProtocol};

use shared::common::LayerDefinition;
use shared::orphan_detector::utility_orphan_filename::{file_basename, file_stem, file_suffix};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct SurfacesOrphanAnalyzer {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ISurfacesOrphanProtocol for SurfacesOrphanAnalyzer {
    fn is_surface_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        alive_files: &ReachabilityResult,
        _definition: Option<&LayerDefinition>,
    ) -> OrphanIndicatorResult {
        // Import graph only — no identifier-based fallback
        let is_reachable = alive_files.paths.contains(f);
        if is_reachable {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        let fp_val = f.value();
        let basename = file_basename(fp_val);
        let stem = file_stem(fp_val);
        let suffix = Self::get_surface_suffix(&basename);
        let category = Self::surface_category(&suffix);

        OrphanIndicatorResult::new(
            true,
            AesOrphanViolation::SurfaceOrphan {
                category,
                stem,
                reason: Some("Surface is unreachable from entry points.".into()),
            }
            .to_string(),
            Severity::HIGH,
        )
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for SurfacesOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SurfacesOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    /// Get surface suffix from filename
    fn get_surface_suffix(basename: &str) -> String {
        file_suffix(basename)
    }

    /// Surface category
    fn surface_category(suffix: &str) -> &'static str {
        match suffix {
            "command" | "controller" | "page" => "smart",
            "hook" | "store" | "action" | "screen" | "router" => "utility",
            "component" | "view" | "layout" => "passive",
            _ => "unknown",
        }
    }
}
```

---

## File: crates/orphan-detector/src/capabilities_orphan_taxonomy_analyzer.rs

```rust
// PURPOSE: TaxonomyOrphanAnalyzer — ITaxonomyOrphanProtocol for orphan taxonomy detection
use shared::code_analysis::{InboundLinkMap, OrphanIndicatorResult};

use shared::common::{FilePath, Severity};

use shared::orphan_detector::{AesOrphanViolation, ITaxonomyOrphanProtocol};

use shared::common::LayerDefinition;
use shared::orphan_detector::utility_orphan_filename::file_stem;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct TaxonomyOrphanAnalyzer {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ITaxonomyOrphanProtocol for TaxonomyOrphanAnalyzer {
    fn is_taxonomy_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        _definition: Option<&LayerDefinition>,
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult {
        let stem = file_stem(f.value());

        let suffix = match stem.rfind('_') {
            Some(pos) => &stem[pos + 1..],
            None => "",
        };

        let is_utility_or_helper = matches!(suffix, "utility" | "helper");

        let category = if is_utility_or_helper {
            "utility"
        } else {
            "taxonomy"
        };

        // Taxonomy orphan = no file from other layers imports it.
        // Other layers: contract, capabilities, agent, utility, surface, root, main, entry, container
        let importers = match inbound_links.get_importers(f.value()) {
            Some(v) => v,
            None => {
                // Fallback: graph resolver may not catch crate:: imports within same crate
                if Self::has_crate_self_import(f.value()) {
                    return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
                }
                return OrphanIndicatorResult::new(
                    true,
                    AesOrphanViolation::TaxonomyOrphan {
                        stem: stem.clone(),
                        category,
                        reason: Some(
                            format!(
                                "Taxonomy '{}' is not imported by any other layer file.",
                                stem
                            )
                            .into(),
                        ),
                    }
                    .to_string(),
                    Severity::LOW,
                );
            }
        };

        // If importers list is empty or only has mod.rs, also check for crate:: self-imports
        // The graph resolver may not track all crate:: imports within the same crate
        let has_only_mod_or_taxonomy = importers.iter().all(|importer| {
            let b = importer.rsplit('/').next().unwrap_or(importer);
            b == "mod.rs" || b.starts_with("taxonomy_")
        });
        if (importers.is_empty() || has_only_mod_or_taxonomy)
            && Self::has_crate_self_import(f.value())
        {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // Check if any importer is from another layer or non-taxonomy file
        let has_other_layer_importer = importers.iter().any(|importer| {
            let b = importer.rsplit('/').next().unwrap_or(importer);
            if b == "mod.rs" || b == "__init__.py" || b == "index.ts" || b == "index.js" {
                return false;
            }
            !b.starts_with("taxonomy_")
        });
        let is_orphan = !has_other_layer_importer;

        if is_orphan {
            OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::TaxonomyOrphan {
                    stem: stem.clone(),
                    category,
                    reason: Some(
                        format!(
                            "Taxonomy '{}' is not imported by any other layer file.",
                            stem
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::LOW,
            )
        } else {
            OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for TaxonomyOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl TaxonomyOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    /// Cached: check if any source file in the same crate imports this module.
    /// Uses a global cache to avoid re-scanning files for each taxonomy file.
    fn has_crate_self_import(file_path: &str) -> bool {
        let stem = std::path::Path::new(file_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        if stem.is_empty() {
            return false;
        }

        // Find the crate's src/ directory
        let file_path_obj = std::path::Path::new(file_path);
        let src_dir = if let Some(parent) = file_path_obj.parent() {
            let mut current = parent.to_path_buf();
            loop {
                if current.ends_with("src") {
                    break;
                }
                if !current.pop() {
                    return false;
                }
            }
            current
        } else {
            return false;
        };

        // Use a static cache: maps (src_dir, stem) -> has_import
        static CACHE: OnceLock<std::sync::Mutex<HashMap<(String, String), bool>>> = OnceLock::new();
        let cache = CACHE.get_or_init(|| std::sync::Mutex::new(HashMap::new()));
        let key = (src_dir.to_string_lossy().to_string(), stem.to_string());

        if let Ok(guard) = cache.lock() {
            if let Some(&result) = guard.get(&key) {
                return result;
            }
        }

        // Cache miss: scan files and build a set of all content
        let all_files =
            shared::orphan_detector::utility_orphan_io::scan_directory_recursive(&src_dir);
        let mut importers: HashSet<String> = HashSet::new();
        for f in &all_files {
            if f == file_path {
                continue;
            }
            let path = std::path::PathBuf::from(f);
            if path.extension().is_some_and(|e| {
                let ext = e.to_str().unwrap_or("");
                matches!(ext, "rs" | "py" | "ts" | "js")
            }) {
                let content = shared::orphan_detector::utility_orphan_io::read_file_safe(f);
                if content.contains(stem) {
                    importers.insert(f.clone());
                }
            }
        }

        let result = !importers.is_empty();
        if let Ok(mut guard) = cache.lock() {
            guard.insert(key, result);
        }
        result
    }
}
```

---

## File: crates/orphan-detector/src/capabilities_orphan_utility_analyzer.rs

```rust
use shared::code_analysis::{InboundLinkMap, OrphanIndicatorResult};

use shared::common::{FilePath, Severity};

use shared::common::utility_layer_detector;
use shared::orphan_detector::{AesOrphanViolation, IUtilityOrphanProtocol};

// Layers that are valid consumers of utility files
const CONSUMER_LAYERS: &[&str] = &["capabilities", "agent", "surface", "root"];

// ─── Block 1: Struct Definition ───────────────────────────
pub struct UtilityOrphanAnalyzer {}

// ─── Block 2: Protocol Trait Implementation ───────────────
impl IUtilityOrphanProtocol for UtilityOrphanAnalyzer {
    fn is_utility_orphan(
        &self,
        f: &FilePath,
        _root_dir: &FilePath,
        all_files: &[String],
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult {
        let fp = f.value();

        let module_name = match std::path::Path::new(fp)
            .file_stem()
            .and_then(|s| s.to_str())
        {
            Some(name) => name.to_string(),
            None => {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        };

        let mut consumer_importers: Vec<String> = Vec::new();
        let mut utility_importers: Vec<String> = Vec::new();

        // Phase 1: Check import graph for consumer-layer importers
        if let Some(importers) = inbound_links.get_importers(fp) {
            let external_importers: Vec<&String> = importers
                .iter()
                .filter(|importer| *importer != fp)
                .collect();

            for importer in external_importers {
                let filename = utility_layer_detector::extract_filename(importer);
                let is_consumer = utility_layer_detector::detect_layer_from_prefix(filename)
                    .map(|layer| CONSUMER_LAYERS.contains(&layer.as_str()))
                    .unwrap_or(false);

                let stem = std::path::Path::new(importer)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                if is_consumer {
                    consumer_importers.push(stem);
                } else {
                    utility_importers.push(stem);
                }
            }

            if !consumer_importers.is_empty() {
                return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
            }
        }

        // Phase 2: Fallback — token-based matching across all files
        let tokens = shared::orphan_detector::utility_orphan_detector::import_tokens(fp);

        // Pre-filter: only check files from consumer layers
        let consumer_files: Vec<&String> = all_files
            .iter()
            .filter(|other_file| {
                if *other_file == fp {
                    return false;
                }
                let filename = utility_layer_detector::extract_filename(other_file);
                utility_layer_detector::detect_layer_from_prefix(filename)
                    .map(|layer| CONSUMER_LAYERS.contains(&layer.as_str()))
                    .unwrap_or(false)
            })
            .collect();

        for other_file in consumer_files {
            let other_content = shared::common::utility_file_handler::read_file_safe(other_file);
            if other_content.is_empty() {
                continue;
            }

            let imported = self.check_import_pattern(&other_content, &module_name)
                || tokens.iter().any(|token| {
                    shared::orphan_detector::utility_orphan_detector::contains_delimited(
                        &other_content,
                        token,
                    )
                });

            if imported {
                let stem = std::path::Path::new(other_file)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                consumer_importers.push(stem);
            }
        }

        // If imported by consumer layers — not dead
        if !consumer_importers.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::LOW);
        }

        // If only imported by other utilities — dead code
        if !utility_importers.is_empty() {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::UtilityDeadCode {
                    stem: module_name.clone(),
                    imported_by: utility_importers,
                    reason: Some(
                        format!(
                            "Utility file '{}' is only imported by other utility files, not by capability, agent, or surfaces layers.",
                            module_name
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::MEDIUM,
            );
        }

        // Not imported by anyone — orphan
        OrphanIndicatorResult::new(
            true,
            AesOrphanViolation::UtilityOrphan {
                stem: module_name.clone(),
                reason: Some(
                    format!(
                        "Utility file '{}' is not imported by any other file.",
                        module_name
                    )
                    .into(),
                ),
            }
            .to_string(),
            Severity::MEDIUM,
        )
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl Default for UtilityOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl UtilityOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check_import_pattern(&self, content: &str, module_name: &str) -> bool {
        // Check for Rust use statements with the module name anywhere in the path
        // This handles:
        // - `use utility_target`
        // - `use utility_target::`
        // - `use crate::utility_target`
        // - `use shared::utility_target`
        // - `use shared::code_analysis::utility_target` (nested paths)
        // - `use crate::code_analysis::utility_target` (nested paths)
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("use ")
                || trimmed.starts_with("pub use ")
                || trimmed.starts_with("pub(crate) use ")
            {
                // Check if module_name appears as a path segment
                if self.path_contains_module(trimmed, module_name) {
                    return true;
                }
            }
        }

        // Check for inline fully qualified calls: module_name::function()
        if content.contains(&format!("{}::", module_name)) {
            return true;
        }

        // Check for grouped imports: use { module_name, ... }
        if content.contains(&format!("::{{{}}}", module_name))
            || content.contains(&format!("::{{{},", module_name))
            || content.contains(&format!(", {}::", module_name))
            || content.contains(&format!(", {}}}", module_name))
        {
            return true;
        }

        // Check for Python imports
        if content.contains(&format!("import {}", module_name))
            || content.contains(&format!("from {} import", module_name))
        {
            return true;
        }

        // Check for JavaScript/TypeScript imports
        if content.contains(&format!("from '{}'", module_name))
            || content.contains(&format!("from \"{}\"", module_name))
            || content.contains(&format!("require('{}')", module_name))
            || content.contains(&format!("require(\"{}\")", module_name))
        {
            return true;
        }

        false
    }

    /// Check if a use statement path contains the module name as a segment.
    fn path_contains_module(&self, use_statement: &str, module_name: &str) -> bool {
        // Extract the path part after "use " or "pub use " etc.
        let path = if let Some(pos) = use_statement.find("use ") {
            &use_statement[pos + 4..]
        } else {
            return false;
        };

        // Remove trailing semicolon and braces
        let path = path.trim_end_matches(';').trim();

        // Handle grouped imports: use path::{A, B, C}
        let path = if let Some(pos) = path.find("::{") {
            &path[..pos]
        } else {
            path
        };

        // Split by :: and check if any segment matches the module name or prefix
        let segments: Vec<&str> = path.split("::").collect();
        segments
            .iter()
            .any(|seg| *seg == module_name || seg.starts_with(&format!("{}_", module_name)))
    }
}
```

---

## File: crates/orphan-detector/src/lib.rs

```rust
/// PURPOSE: Module declarations for orphan-detector (orchestrator, analyzers, container)
pub mod agent_orphan_orchestrator;
pub mod capabilities_orphan_agent_analyzer;
pub mod capabilities_orphan_capabilities_analyzer;
pub mod capabilities_orphan_contract_analyzer;
pub mod capabilities_orphan_graph_resolver;
pub mod capabilities_orphan_surfaces_analyzer;
pub mod capabilities_orphan_taxonomy_analyzer;
pub mod capabilities_orphan_utility_analyzer;
pub mod root_orphan_detector_container;
pub mod utility_orphan_graph_resolver;
pub mod utility_orphan_regex_patterns;
```

---

## File: crates/orphan-detector/src/root_orphan_detector_container.rs

```rust
use crate::agent_orphan_orchestrator::{ArchOrphanAnalyzer, ArchOrphanDeps};
use crate::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use shared::common::FilePath;
use shared::config_system::{ArchitectureConfig, IConfigOrchestratorAggregate};

use shared::orphan_detector::{IOrphanAggregate, IOrphanGraphResolverProtocol};

use std::sync::Arc;

pub struct OrphanContainer {
    analyzer: Arc<dyn IOrphanAggregate>,
}

impl OrphanContainer {
    pub fn new() -> Self {
        Self::new_with_config(ArchitectureConfig::default())
    }

    pub fn new_with_ignored(ignored_paths: Vec<String>) -> Self {
        let config = ArchitectureConfig {
            ignored_paths: shared::common::taxonomy_paths_vo::FilePathList::new(
                ignored_paths
                    .into_iter()
                    .filter_map(|p| shared::common::taxonomy_path_vo::FilePath::new(p).ok())
                    .collect(),
            ),
            ..Default::default()
        };
        Self::new_with_config(config)
    }

    pub fn new_with_config(config: ArchitectureConfig) -> Self {
        let resolver: Arc<dyn IOrphanGraphResolverProtocol> = Arc::new(OrphanGraphResolver::new());
        let arch = Arc::new(ArchOrphanAnalyzer::new(
            ArchOrphanDeps {
                resolver,
                taxonomy_analyzer: Arc::new(
                    crate::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer::new(),
                ),
                contract_analyzer: Arc::new(
                    crate::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer::new(),
                ),
                capabilities_analyzer: Arc::new(
                    crate::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer::new(),
                ),
                utility_analyzer: Arc::new(
                    crate::capabilities_orphan_utility_analyzer::UtilityOrphanAnalyzer::new(),
                ),
                agent_analyzer: Arc::new(
                    crate::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer::new(),
                ),
                surfaces_analyzer: Arc::new(
                    crate::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer::new(),
                ),
            },
            config,
        ));
        Self {
            analyzer: arch.clone() as Arc<dyn IOrphanAggregate>,
        }
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

    pub fn analyzer(&self) -> Arc<dyn IOrphanAggregate> {
        self.analyzer.clone()
    }
}

impl Default for OrphanContainer {
    fn default() -> Self {
        Self::new()
    }
}
```

---

## File: crates/orphan-detector/src/utility_orphan_graph_resolver.rs

```rust
// PURPOSE: utility_orphan_graph_resolver — helper functions for graph resolution
use std::collections::HashMap;

/// Build a crate module index for hyphen-aware resolution.
/// Maps normalized module paths to canonical file paths.
pub fn build_crate_module_index(
    crate_src_dirs: &HashMap<String, std::path::PathBuf>,
) -> HashMap<String, HashMap<String, String>> {
    let mut index: HashMap<String, HashMap<String, String>> = HashMap::new();
    for (crate_name, src_dir) in crate_src_dirs {
        let mut module_map: HashMap<String, String> = HashMap::new();
        let canonical_src = std::fs::canonicalize(src_dir).unwrap_or_else(|_| src_dir.clone());
        let all_files =
            shared::orphan_detector::utility_orphan_io::scan_directory_recursive(&canonical_src);
        for path_str in all_files {
            if !path_str.ends_with(".rs")
                && !path_str.ends_with(".py")
                && !path_str.ends_with(".ts")
                && !path_str.ends_with(".js")
            {
                continue;
            }
            let canonical_path = match std::fs::canonicalize(&path_str) {
                Ok(p) => p,
                Err(_) => std::path::PathBuf::from(&path_str),
            };
            let stem = canonical_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_string();
            if stem.is_empty() {
                continue;
            }
            let canon_str = canonical_path.to_string_lossy().to_string();
            let rel_path = canonical_path
                .strip_prefix(&canonical_src)
                .unwrap_or(&canonical_path);
            let rel_str = rel_path.with_extension("").to_string_lossy().to_string();
            let normalized_rel =
                shared::orphan_detector::utility_orphan_detector::normalize_module_path(
                    &rel_str.replace(std::path::MAIN_SEPARATOR, "/"),
                );
            module_map.insert(normalized_rel, canon_str.clone());
            module_map.insert(stem.clone(), canon_str.clone());
            module_map.insert(
                shared::orphan_detector::utility_orphan_detector::normalize_module_component(&stem),
                canon_str.clone(),
            );
            if stem == "mod" || stem == "__init__" || stem == "index" {
                if let Some(parent_dir) = canonical_path.parent().and_then(|p| p.file_name()) {
                    let parent = parent_dir.to_string_lossy().to_string();
                    module_map.insert(parent.clone(), canon_str.clone());
                    module_map.insert(
                        shared::orphan_detector::utility_orphan_detector::normalize_module_component(
                            &parent,
                        ),
                        canon_str.clone(),
                    );
                }
            }
        }
        let normalized_name =
            shared::orphan_detector::utility_orphan_detector::normalize_module_component(
                crate_name,
            );
        index.insert(crate_name.clone(), module_map.clone());
        index.insert(normalized_name, module_map);
    }
    index
}

/// Resolve a workspace module path to its canonical file path.
pub fn resolve_workspace_module(
    index: &HashMap<String, HashMap<String, String>>,
    crate_name: &str,
    segments: &[&str],
    current_file: &str,
) -> Option<String> {
    let map = index.get(crate_name)?;
    let seg_str = segments.join("/");
    let normalized =
        shared::orphan_detector::utility_orphan_detector::normalize_module_path(&seg_str);
    if let Some(path) = map.get(&normalized) {
        if path != current_file {
            return Some(path.clone());
        }
    }
    for i in (1..segments.len()).rev() {
        let candidate = segments[..i].join("/");
        let normalized =
            shared::orphan_detector::utility_orphan_detector::normalize_module_path(&candidate);
        if let Some(path) = map.get(&normalized) {
            if path != current_file {
                return Some(path.clone());
            }
        }
    }
    None
}

/// Add an edge to the import graph and inbound links.
pub fn add_edge(
    import_graph: &mut HashMap<String, Vec<String>>,
    inbound_links: &mut HashMap<String, Vec<String>>,
    source: &str,
    target: &str,
) {
    import_graph
        .entry(source.to_string())
        .or_default()
        .push(target.to_string());
    inbound_links
        .entry(target.to_string())
        .or_default()
        .push(source.to_string());
}

/// Find workspace root by walking up from start_dir until we find a directory
/// with a project manifest (Cargo.toml, pyproject.toml, package.json) AND
/// a member directory (crates/, packages/, modules/).
pub fn find_workspace_root(start_dir: &str) -> String {
    let mut current = std::path::PathBuf::from(start_dir);
    // Make absolute so parent() doesn't return "" for relative single-segment paths
    if current.is_relative() {
        if let Ok(cwd) = std::env::current_dir() {
            current = cwd.join(&current);
        }
    }
    loop {
        let has_manifest = current.join("Cargo.toml").exists()
            || current.join("pyproject.toml").exists()
            || current.join("package.json").exists();
        let has_members = current.join("crates").exists()
            || current.join("packages").exists()
            || current.join("modules").exists();
        if has_manifest && has_members {
            return current.to_string_lossy().to_string();
        }
        // Move up one directory
        if !current.pop() {
            // Reached filesystem root without finding workspace root
            return start_dir.to_string();
        }
    }
}

/// Deduplicate edges in a HashMap<String, Vec<String>> by key.
/// This prevents inflating inbound link counts with duplicate entries.
pub fn dedup_edges(map: &mut HashMap<String, Vec<String>>) {
    let keys: Vec<String> = map.keys().cloned().collect();
    for k in keys {
        if let Some(values) = map.get(&k) {
            let mut seen = std::collections::HashSet::new();
            let deduped: Vec<String> = values
                .iter()
                .filter(|x| seen.insert(x.as_str()))
                .cloned()
                .collect();
            if deduped.len() != values.len() {
                map.insert(k, deduped);
            }
        }
    }
}

/// Resolve a TypeScript/JavaScript relative import path against the current file's directory.
/// Handles extensionless imports by trying .ts, .js, .tsx, .jsx, and /index.* files.
/// Returns the relative path (matching graph convention) or None if not found.
pub fn resolve_ts_relative(
    current_file: &str,
    import_path: &str,
    workspace_root: &std::path::Path,
) -> Option<String> {
    let base = std::path::Path::new(current_file).parent()?;
    let joined = base.join(import_path);

    // TS imports omit extension — try .ts, .js, /index.ts, /index.js, .tsx, .jsx
    let candidates: Vec<std::path::PathBuf> = vec![
        joined.with_extension("ts"),
        joined.with_extension("js"),
        joined.join("index.ts"),
        joined.join("index.js"),
        joined.with_extension("tsx"),
        joined.with_extension("jsx"),
    ];

    for cand in &candidates {
        if shared::orphan_detector::utility_orphan_io::is_file(cand) {
            // Return RELATIVE path (matching graph convention)
            let rel = cand
                .strip_prefix(workspace_root)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| cand.to_string_lossy().to_string());
            if rel != current_file {
                return Some(rel);
            }
        }
    }
    None
}
```

---

## File: crates/orphan-detector/src/utility_orphan_regex_patterns.rs

```rust
// PURPOSE: utility_orphan_regex_patterns — regex patterns for graph resolution
use regex::Regex;
use std::sync::OnceLock;

/// Bug 8 fix: Allow digits in module name (e.g., `mod foo2;`, `mod v2_api;`)
pub fn pub_mod_path_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"#\[path\s*=\s*"([^"]+)"\]\s*(?:pub\s+)?mod\s+([a-zA-Z_][a-zA-Z0-9_]*)"#).ok()
    })
    .as_ref()
}

/// Bug 6 fix: Handle `pub(crate) mod foo;` visibility qualifiers
pub fn plain_mod_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*;").ok()
    })
    .as_ref()
}

/// Bug 4 fix: Add word boundary \b to prevent matching in comments/strings/literals
pub fn import_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"\b(?:use|import|from)\s+([a-zA-Z_][a-zA-Z0-9_\.:]*(?:\{[^}]*\})?)").ok()
    })
    .as_ref()
}

/// Regex for Python relative imports: `from . import X` or `from .module import X`
pub fn python_relative_import_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?s)from\s+(\.{1,3})\s+import\s*\(([^)]+)\)|from\s+(\.{1,3})\s+import\s+([^\n]+)",
        )
        .ok()
    })
    .as_ref()
}

/// Bug 12 fix: Only run for Python files (class inheritance pattern)
pub fn inh_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"class\s+\w+\(([^)]+)\)").ok())
        .as_ref()
}

/// TypeScript/JavaScript import patterns:
/// - `import { X } from './path'` (named)
/// - `import X from './path'` (default)
/// - `import * as X from './path'` (namespace)
/// - `import './path'` (side-effect)
/// - `import Default, { Named } from './path'` (default + named)
pub fn ts_import_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"\bimport\s+(?:\{([^}]*)\}\s*from\s*['"]([^'"]+)['"]|(\w+)\s*,\s*\{([^}]*)\}\s*from\s*['"]([^'"]+)['"]|(\w+)\s+from\s*['"]([^'"]+)['"]|\*\s*as\s+(\w+)\s+from\s*['"]([^'"]+)['"]|(['"]([^'"]+)['"]))"#).ok()
    })
    .as_ref()
}

/// TypeScript/JavaScript export re-exports:
/// - `export { X } from './path'` (named)
/// - `export * from './path'` (re-export all)
/// - `export { X } from './path'` (side-effect, no from)
pub fn ts_export_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| {
        // Match: export { X, Y } from './path', export * from './path'
        Regex::new(
            r#"\bexport\s+(?:\{([^}]*)\}\s*from\s*['"]([^'"]+)['"]|\*\s*from\s*['"]([^'"]+)['"])?"#,
        )
        .ok()
    })
    .as_ref()
}

/// Bug 1 fix: Match multi-segment `pub use crate::common::taxonomy_action_vo;`
/// No lookahead (regex crate doesn't support it) — filter at code level
pub fn pub_use_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| {
            // Match `pub use crate::module::submodule;` multi-segment paths
            Regex::new(r"pub\s+use\s+(?:crate|super|self)::([a-zA-Z_][a-zA-Z0-9_]*(?:::[a-zA-Z_][a-zA-Z0-9_]*)*)\s*;").ok()
        })
    .as_ref()
}

/// Bug 2 fix: No negative lookahead (regex crate doesn't support it)
/// Filter crate::/super::/self:: prefixes at code level in Pass 5b
pub fn pub_use_relative_re() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"pub\s+use\s+([a-zA-Z_][a-zA-Z0-9_]*(?:::[a-zA-Z_][a-zA-Z0-9_]*)*)\s*;").ok()
    })
    .as_ref()
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

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Taxonomy types ──
pub use taxonomy_cli_vo::Cli;
pub use taxonomy_cli_vo::Commands;
pub use taxonomy_command_catalog_vo::CommandCatalogVO;
pub use taxonomy_command_catalog_vo::CommandMetadataVO;
pub use taxonomy_format_vo::Format;
pub use taxonomy_position_vo::Position;
pub use taxonomy_protocol_vo::TransportEndpoint;
pub use taxonomy_protocol_vo::TransportProtocol;
pub use taxonomy_protocol_vo::TransportUrlVO;
pub use taxonomy_result_vo::LintResult;
pub use taxonomy_result_vo::LintResultList;
pub use taxonomy_scan_report_vo::DiagnosticSeverity;
pub use taxonomy_scan_report_vo::PipelineDiagnostic;
pub use taxonomy_scan_report_vo::PipelineError;
pub use taxonomy_scan_report_vo::ScanReport;
pub use taxonomy_scan_request_vo::ScanMode;
pub use taxonomy_scan_request_vo::ScanRequest;
pub use taxonomy_scan_request_vo::ScanTarget;
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

## File: crates/shared/src/code-analysis/mod.rs

```rust
// code-analysis — taxonomy and contract types
pub mod contract_adapter_protocol;
pub mod contract_bypass_checker_protocol;
pub mod contract_class_protocol;
pub mod contract_code_analysis_aggregate;
pub mod contract_code_metric_analyzer_protocol;
pub mod contract_dead_inheritance_protocol;
pub mod contract_line_protocol;
pub mod taxonomy_analysis_vo;
pub mod taxonomy_code_analysis_rule_vo;
pub mod taxonomy_operation_error;
pub mod taxonomy_violation_code_analysis_vo;
pub mod utility_bypass_detector;
pub mod utility_code_duplication_detector;
pub mod utility_column_index;
pub mod utility_file_reader;
pub mod utility_language_mapper;
pub mod utility_mandatory_checker;
pub mod utility_target_resolver;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_adapter_protocol::ILinterAdapterProtocol;
pub use contract_bypass_checker_protocol::IBypassCheckerProtocol;
pub use contract_class_protocol::IMandatoryClassProtocol;
pub use contract_code_analysis_aggregate::ICodeAnalysisAggregate;
pub use contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
pub use contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
pub use contract_line_protocol::ILineCheckerProtocol;

// ── Taxonomy types ──
pub use taxonomy_analysis_vo::FileDefinitionMap;
pub use taxonomy_analysis_vo::GraphAnalysisContext;
pub use taxonomy_analysis_vo::ImportGraph;
pub use taxonomy_analysis_vo::InboundLinkMap;
pub use taxonomy_analysis_vo::InheritanceMap;
pub use taxonomy_analysis_vo::OrphanIndicatorResult;
pub use taxonomy_analysis_vo::ReachabilityResult;
pub use taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
pub use taxonomy_code_analysis_rule_vo::MandatoryImportRuleVO;
pub use taxonomy_operation_error::LinterOperationError;
pub use taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
pub use taxonomy_violation_code_analysis_vo::Language;
pub use taxonomy_violation_code_analysis_vo::ViolationKind;
pub use taxonomy_violation_code_analysis_vo::WORD_PATTERN_TOKENS;
```

---

## File: crates/shared/src/code-analysis/taxonomy_analysis_vo.rs

```rust
// PURPOSE: FileDefinitionMap, GraphAnalysisContext, ImportGraph, InboundLinkMap, InheritanceMap — analysis value objects for code structure
// Re-export LintResultList so code_analysis contracts stay within their own domain.
pub use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_severity_vo::Severity;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// A set of file paths.
pub type FilePathSet = HashSet<FilePath>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileDefinitionMap {
    pub mapping: std::collections::HashMap<String, Vec<String>>,
}

impl FileDefinitionMap {
    pub fn new(value: std::collections::HashMap<String, Vec<String>>) -> Self {
        Self { mapping: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphAnalysisContext {
    pub import_graph: ImportGraph,
    pub inbound_links: InboundLinkMap,
    pub inheritance_map: InheritanceMap,
    pub file_definitions: FileDefinitionMap,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportGraph {
    pub mapping: std::collections::HashMap<String, Vec<String>>,
}

impl ImportGraph {
    pub fn new(value: std::collections::HashMap<String, Vec<String>>) -> Self {
        Self { mapping: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InboundLinkMap {
    pub mapping: std::collections::HashMap<String, Vec<String>>,
}

impl InboundLinkMap {
    pub fn new(value: std::collections::HashMap<String, Vec<String>>) -> Self {
        Self { mapping: value }
    }

    /// Retrieve importers for a file path, falling back to canonical or suffix matching if needed.
    /// Merges importers from both exact match and ./ prefix match to handle path normalization.
    pub fn get_importers(&self, path: &str) -> Option<&Vec<String>> {
        let mut result: Option<&Vec<String>> = None;

        // Try exact match first
        if let Some(v) = self.mapping.get(path) {
            result = Some(v);
        }

        // Try with ./ prefix at beginning (graph resolver may add this)
        let with_prefix = format!("./{}", path);
        if let Some(v) = self.mapping.get(&with_prefix) {
            // If we already have a result, prefer the ./ prefix version if it has more importers
            if let Some(existing) = result {
                if v.len() > existing.len() {
                    result = Some(v);
                }
            } else {
                result = Some(v);
            }
        }

        // Try with ./ in the middle (graph resolver may add this)
        // e.g., /home/user/project/./crates/... instead of /home/user/project/crates/...
        if let Some(pos) = path.find("/crates/") {
            let with_middle_dot = format!("{}/.{}", &path[..pos], &path[pos..]);
            if let Some(v) = self.mapping.get(&with_middle_dot) {
                if let Some(existing) = result {
                    if v.len() > existing.len() {
                        result = Some(v);
                    }
                } else {
                    result = Some(v);
                }
            }
        }

        // Try canonical path
        if result.is_none() {
            if let Ok(canon) = std::fs::canonicalize(path) {
                if let Some(canon_str) = canon.to_str() {
                    if let Some(v) = self.mapping.get(canon_str) {
                        result = Some(v);
                    }
                    // Try canonical with ./ prefix
                    let canon_with_prefix = format!("./{}", canon_str);
                    if let Some(v) = self.mapping.get(&canon_with_prefix) {
                        if let Some(existing) = result {
                            if v.len() > existing.len() {
                                result = Some(v);
                            }
                        } else {
                            result = Some(v);
                        }
                    }
                }
            }
        }

        // Try clean path (without ./ prefix)
        if result.is_none() {
            let clean = path.strip_prefix("./").unwrap_or(path);
            if let Some(v) = self.mapping.get(clean) {
                result = Some(v);
            }
        }

        // Try exact match after stripping ./ from keys
        if result.is_none() {
            let clean = path.strip_prefix("./").unwrap_or(path);
            for (k, v) in &self.mapping {
                let k_clean = k.strip_prefix("./").unwrap_or(k);
                if k_clean == clean {
                    result = Some(v);
                    break;
                }
            }
        }

        // Try suffix match
        if result.is_none() {
            let clean = path.strip_prefix("./").unwrap_or(path);
            for (k, v) in &self.mapping {
                let k_clean = k.strip_prefix("./").unwrap_or(k);
                if k_clean.ends_with(clean) || clean.ends_with(k_clean) {
                    result = Some(v);
                    break;
                }
            }
        }

        result
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InheritanceMap {
    pub mapping: std::collections::HashMap<String, Vec<String>>,
}

impl InheritanceMap {
    pub fn new(value: std::collections::HashMap<String, Vec<String>>) -> Self {
        Self { mapping: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrphanIndicatorResult {
    pub is_orphan: bool,
    pub reason: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReachabilityResult {
    pub paths: FilePathSet,
}

impl ReachabilityResult {
    pub fn new(value: FilePathSet) -> Self {
        Self { paths: value }
    }
}

impl GraphAnalysisContext {
    pub fn new(
        import_graph: ImportGraph,
        inbound_links: InboundLinkMap,
        inheritance_map: InheritanceMap,
        file_definitions: FileDefinitionMap,
    ) -> Self {
        Self {
            import_graph,
            inbound_links,
            inheritance_map,
            file_definitions,
        }
    }
}

impl OrphanIndicatorResult {
    pub fn new(is_orphan: bool, reason: String, severity: Severity) -> Self {
        Self {
            is_orphan,
            reason,
            severity,
        }
    }
}
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

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly
// NOTE: lib.rs re-exports all of common via `pub use common::*`

// ── Contract traits ──
pub use contract_executor_protocol::ICommandExecutorProtocol;

// ── Taxonomy types ──
pub use taxonomy_action_vo::ActionName;
pub use taxonomy_action_vo::JobId;
pub use taxonomy_adapter_error::AdapterError;
pub use taxonomy_adapter_error::ScanError;
pub use taxonomy_adapter_error::ValidationError;
pub use taxonomy_adapter_list_vo::AdapterNameList;
pub use taxonomy_adapter_name_vo::AdapterName;
pub use taxonomy_common_error::Cause;
pub use taxonomy_common_error::Constraint;
pub use taxonomy_common_error::ExitCode;
pub use taxonomy_common_error::FieldName;
pub use taxonomy_common_error::ModuleName;
pub use taxonomy_common_error::PrimitiveTypeName;
pub use taxonomy_common_vo::BooleanVO;
pub use taxonomy_common_vo::ColumnNumber;
pub use taxonomy_common_vo::Count;
pub use taxonomy_common_vo::DataFlowList;
pub use taxonomy_common_vo::ErrorMessage;
pub use taxonomy_common_vo::IntoPatternListValues;
pub use taxonomy_common_vo::JobIdList;
pub use taxonomy_common_vo::LanguageVO;
pub use taxonomy_common_vo::LineContentList;
pub use taxonomy_common_vo::LineNumber;
pub use taxonomy_common_vo::PatternList;
pub use taxonomy_common_vo::ResponseDataList;
pub use taxonomy_common_vo::Score;
pub use taxonomy_common_vo::SuffixPolicyVO;
pub use taxonomy_common_vo::SuffixVO;
pub use taxonomy_common_vo::Timestamp;
pub use taxonomy_definition_vo::LayerDefinition;
pub use taxonomy_definition_vo::LayerMapVO;
pub use taxonomy_definition_vo::NamingConfig;
pub use taxonomy_display_content_vo::DisplayContent;
pub use taxonomy_duration_vo::Timeout;
pub use taxonomy_error_vo::ErrorCode;
pub use taxonomy_filesystem_error::FileSystemError;
pub use taxonomy_git_vo::GitBranchName;
pub use taxonomy_job_vo::AdapterMetadata;
pub use taxonomy_job_vo::EnvContentVO;
pub use taxonomy_job_vo::McpConfigVO;
pub use taxonomy_job_vo::SuccessStatus;
pub use taxonomy_language_info_vo::LanguageInfo;
pub use taxonomy_language_vo::Language;
pub use taxonomy_layer_vo::FileContentVO;
pub use taxonomy_layer_vo::Identity;
pub use taxonomy_layer_vo::LayerNameVO;
pub use taxonomy_layer_vo::LineContentVO;
pub use taxonomy_line_count_vo::LineCount;
pub use taxonomy_lint_vo::CommandArgs;
pub use taxonomy_lint_vo::Location;
pub use taxonomy_lint_vo::LocationList;
pub use taxonomy_lint_vo::ScopeBounds;
pub use taxonomy_lint_vo::ScopeRef;
pub use taxonomy_lint_vo::ViolationConstraint;
pub use taxonomy_message_vo::ComplianceStatus;
pub use taxonomy_message_vo::LintMessage;
pub use taxonomy_name_vo::NameVariants;
pub use taxonomy_name_vo::SymbolName;
pub use taxonomy_path_utils_vo::PathUtils;
pub use taxonomy_path_vo::DirectoryPath;
pub use taxonomy_path_vo::FilePath;
pub use taxonomy_paths_vo::FilePathList;
pub use taxonomy_paths_vo::RenamedFile;
pub use taxonomy_paths_vo::RenamedFileList;
pub use taxonomy_response_data_vo::ResponseData;
pub use taxonomy_severity_vo::Severity;
pub use taxonomy_source_vo::ContentString;
pub use taxonomy_source_vo::SourceContentVO;
pub use taxonomy_suffix_vo::SuffixPolicyVO as SuffixPolicyVOMod;
pub use taxonomy_suffix_vo::SuffixVO as SuffixVOMod;
pub use taxonomy_suggestion_vo::DescriptionVO;
pub use taxonomy_suggestion_vo::MetadataVO;
pub use taxonomy_threshold_vo::Threshold;
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
use crate::import_rules::taxonomy_import_error::ImportError;
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

impl From<ImportError> for ScanError {
    fn from(e: ImportError) -> Self {
        let path = match &e {
            ImportError::CircularDependency { file: Some(p), .. } => p.clone(),
            _ => FilePath::new(".").unwrap_or_default(),
        };
        ScanError {
            path,
            message: ErrorMessage::new(e.to_string()),
            error_code: Some(ErrorCode::raw("IMPORT_ERR")),
            adapter_name: Some(AdapterName::raw("import-rules")),
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

## File: crates/shared/src/common/taxonomy_adapter_list_vo.rs

```rust
// PURPOSE: AdapterNameList — value object for a list of adapter names
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_adapter_name_vo::AdapterName;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AdapterNameList {
    pub values: Vec<AdapterName>,
}

impl AdapterNameList {
    pub fn new(value: Vec<AdapterName>) -> Self {
        Self { values: value }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, AdapterName> {
        self.values.iter()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn push(&mut self, item: AdapterName) {
        self.values.push(item);
    }
}

impl std::ops::Deref for AdapterNameList {
    type Target = Vec<AdapterName>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}
```

---

## File: crates/shared/src/common/taxonomy_adapter_name_vo.rs

```rust
// PURPOSE: AdapterName — validated newtype for adapter/linter name strings
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// adapter_name_vo — Adapter and tool identifier value objects.
///
/// Adapter/tool identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AdapterName {
    pub value: String,
}

impl AdapterName {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new AdapterName from a string.
    ///
    /// # Errors
    /// Returns an error if the adapter name is empty or only whitespace.
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("Adapter name cannot be empty".to_string());
        }
        Ok(AdapterName {
            value: value.trim().to_string(),
        })
    }

    /// Create a raw AdapterName without error validation (for static compile-time safe inputs).
    pub fn raw<S: Into<String>>(value: S) -> Self {
        AdapterName {
            value: value.into(),
        }
    }
}

impl std::ops::Deref for AdapterName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for AdapterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Hash for AdapterName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
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

## File: crates/shared/src/common/taxonomy_definition_vo.rs

```rust
// PURPOSE: LayerDefinition, LayerMapVO, NamingConfig — VOs for AES layer definitions and naming policies
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use serde::{Deserialize, Serialize};

/// Wrap a single-field VO that exposes a `new(value)` constructor plus the
/// default `derive`s needed by the codebase. Used to keep the boilerplate
/// for `LayerMapVO`/`NamingConfig` uniform without introducing a new linter
/// violation cluster.
macro_rules! single_field_vo {
    ($name:ident, $field:ident: $field_ty:ty) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
        #[serde(crate = "serde")]
        pub struct $name {
            pub $field: $field_ty,
        }

        impl $name {
            pub fn new($field: $field_ty) -> Self {
                Self { $field }
            }
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LayerDefinition {
    #[serde(default)]
    pub allowed: PatternList,
    #[serde(default)]
    pub forbidden: PatternList,
    #[serde(default)]
    pub mandatory: PatternList,
    #[serde(default)]
    pub word_count: Count,
    #[serde(default)]
    pub exceptions: PatternList,
    #[serde(default)]
    pub recursive: BooleanVO,

    #[serde(flatten)]
    pub naming: crate::config_system::taxonomy_config_vo::NamingRuleVO,
    #[serde(flatten)]
    pub code_analysis: crate::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO,
    #[serde(flatten)]
    pub role: crate::config_system::taxonomy_config_vo::RoleRuleVO,
    #[serde(flatten)]
    pub orphan: crate::config_system::taxonomy_config_vo::OrphanRuleVO,
}

single_field_vo!(LayerMapVO, values: std::collections::HashMap<LayerNameVO, LayerDefinition>);
single_field_vo!(NamingConfig, word_count: Count);
```

---

## File: crates/shared/src/common/taxonomy_display_content_vo.rs

```rust
// PURPOSE: DisplayContent — value object for formatted display output (previews, human-readable sizes, etc.)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisplayContent {
    pub value: String,
}

impl DisplayContent {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl From<String> for DisplayContent {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl std::fmt::Display for DisplayContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
```

---

## File: crates/shared/src/common/taxonomy_duration_vo.rs

```rust
// PURPOSE: Duration, Timeout — value objects for duration and timeout tracking
use serde::Serialize;

/// Wrap a `f64` value object that should be clamped to a minimum during
/// construction. Emit the struct, manual `new`/`value`/`Display`/`From`
/// impls, and a serde `Deserialize` that respects the clamp.
macro_rules! clamped_f64_vo {
    ($name:ident, $min:expr, $display_fmt:literal) => {
        #[derive(Debug, Clone, Serialize, PartialEq)]
        #[serde(transparent)]
        pub struct $name {
            pub value: f64,
        }

        impl $name {
            pub fn new(value: f64) -> Self {
                Self {
                    value: value.max($min),
                }
            }
            pub fn value(&self) -> f64 {
                self.value
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $display_fmt, self.value)
            }
        }

        impl From<f64> for $name {
            fn from(v: f64) -> Self {
                Self { value: v }
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(serde::Deserialize)]
                #[serde(transparent)]
                struct W {
                    value: f64,
                }
                let w = W::deserialize(deserializer)?;
                Ok(Self {
                    value: w.value.max($min),
                })
            }
        }
    };
}

clamped_f64_vo!(Timeout, 0.001, "{}s");
```

---

## File: crates/shared/src/common/taxonomy_error_vo.rs

```rust
// PURPOSE: ErrorCode — value object for AES error code identification
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// error_code_vo — Error code value object.
///
/// Linter error code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ErrorCode {
    code: String,
}

impl ErrorCode {
    pub fn code(&self) -> &str {
        &self.code
    }
    /// Create a new ErrorCode from a string.
    ///
    /// # Errors
    /// Returns an error if the code is empty.
    pub fn new<S: Into<String>>(code: S) -> Result<Self, String> {
        let code = code.into();
        if code.is_empty() {
            return Err("Error code cannot be empty".to_string());
        }
        Ok(ErrorCode { code })
    }

    /// Create a raw ErrorCode without error validation.
    pub fn raw<S: Into<String>>(code: S) -> Self {
        ErrorCode { code: code.into() }
    }

    /// Returns true if the code is a style error (starts with E, W, or D).
    pub fn is_style(&self) -> bool {
        self.code.starts_with('E') || self.code.starts_with('W') || self.code.starts_with('D')
    }
    pub fn is_logic(&self) -> bool {
        self.code.starts_with('F') || self.code.starts_with('I')
    }
    pub fn is_security(&self) -> bool {
        self.code.starts_with('B')
    }
    pub fn is_architecture(&self) -> bool {
        self.code.starts_with("AES")
    }
}

impl std::ops::Deref for ErrorCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.code
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl Hash for ErrorCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}
```

---

## File: crates/shared/src/common/taxonomy_filesystem_error.rs

```rust
// PURPOSE: FileSystemError — structured error type for filesystem operation failures
use crate::common::taxonomy_action_vo::ActionName;
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct FileSystemError {
    pub path: FilePath,
    pub message: ErrorMessage,
    pub operation: ActionName,
    #[serde(default)]
    pub error_code: ErrorCode,
    #[serde(default)]
    pub cause: Cause,
}

impl FileSystemError {
    pub fn new(path: FilePath, message: ErrorMessage, operation: ActionName) -> Self {
        Self {
            path,
            message,
            operation,
            error_code: ErrorCode::default(),
            cause: Cause::default(),
        }
    }
}

impl std::fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = if self.error_code.code().is_empty() {
            String::new()
        } else {
            format!(" [{}]", self.error_code.code())
        };
        write!(
            f,
            "FS Error during {} on {}{}: {}",
            self.operation, self.path, code, self.message
        )
    }
}
```

---

## File: crates/shared/src/common/taxonomy_job_vo.rs

```rust
// PURPOSE: PipelineJob, SuccessStatus, EnvContentVO, McpConfigVO — value objects for pipeline job lifecycle tracking
// ResponseData is re-exported from common for backward compatibility
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::string_value_object;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::common::taxonomy_response_data_vo::ResponseData;

// Manual impl: `SuccessStatus` overrides `Display` to render "SUCCESS"/"FAILURE"
// instead of `true`/`false`, and the macro does not currently support a clean
// `bool` cast (Rust forbids `i64 as bool`). Kept as a hand-rolled VO.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SuccessStatus {
    pub value: bool,
}

impl Default for SuccessStatus {
    fn default() -> Self {
        Self::new(false)
    }
}

impl SuccessStatus {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}

impl std::fmt::Display for SuccessStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value {
            write!(f, "SUCCESS")
        } else {
            write!(f, "FAILURE")
        }
    }
}

impl std::ops::Deref for SuccessStatus {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.value
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterMetadata {
    pub name: AdapterName,
    pub class_path: String,
    #[serde(default)]
    pub description: String,
}

impl AdapterMetadata {
    pub fn new(name: AdapterName, class_path: String) -> Self {
        Self {
            name,
            class_path,
            description: String::new(),
        }
    }
}

string_value_object!(EnvContentVO);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpConfigVO {
    #[serde(default)]
    pub value: HashMap<String, serde_json::Value>,
}

impl McpConfigVO {
    pub fn new(value: HashMap<String, serde_json::Value>) -> Self {
        Self { value }
    }
    pub fn value(&self) -> &HashMap<String, serde_json::Value> {
        &self.value
    }
}
```

---

## File: crates/shared/src/common/taxonomy_language_info_vo.rs

```rust
// PURPOSE: LanguageInfo — value object for pre-computed language flags (is_rs, is_py, is_js, lang)
use crate::common::taxonomy_language_vo::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageInfo {
    pub is_rs: bool,
    pub is_py: bool,
    pub is_js: bool,
    pub lang: Language,
}

impl LanguageInfo {
    pub fn new(is_rs: bool, is_py: bool, is_js: bool, lang: Language) -> Self {
        Self {
            is_rs,
            is_py,
            is_js,
            lang,
        }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_line_count_vo.rs

```rust
// PURPOSE: LineCount — value object for the number of lines (preview, file limits, etc.)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineCount {
    pub value: usize,
}

impl LineCount {
    pub fn new(value: usize) -> Self {
        Self { value }
    }

    pub fn value(&self) -> usize {
        self.value
    }
}

impl From<usize> for LineCount {
    fn from(value: usize) -> Self {
        Self { value }
    }
}

impl std::fmt::Display for LineCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
```

---

## File: crates/shared/src/common/taxonomy_lint_vo.rs

```rust
// PURPOSE: CommandArgs, Location, LocationList, ScopeBounds, ScopeRef, ViolationConstraint — VOs for lint violations
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeRef {
    pub name: DescriptionVO,
    #[serde(default)]
    pub kind: DescriptionVO,
    #[serde(default)]
    pub file: Option<FilePath>,
    #[serde(default)]
    pub start_line: Option<LineNumber>,
    #[serde(default)]
    pub end_line: Option<LineNumber>,
}

impl ScopeRef {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: DescriptionVO::new(name),
            kind: DescriptionVO::new("function"),
            file: None,
            start_line: None,
            end_line: None,
        }
    }
    pub fn has_range(&self) -> bool {
        self.start_line.as_ref().is_some_and(|l| l.value > 0)
            && self.end_line.as_ref().is_some_and(|l| l.value > 0)
    }
}

impl std::fmt::Display for ScopeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref file) = self.file {
            write!(f, "{} {} in {}", self.kind.value, self.name.value, file)
        } else if !self.kind.value.is_empty() {
            write!(f, "{} {}", self.kind.value, self.name.value)
        } else {
            write!(f, "{}", self.name.value)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    #[serde(default)]
    pub file: Option<FilePath>,
    #[serde(default)]
    pub line: Option<LineNumber>,
    #[serde(default)]
    pub column: Option<ColumnNumber>,
    #[serde(default)]
    pub description: DescriptionVO,
}

impl Default for Location {
    fn default() -> Self {
        Self::new()
    }
}

impl Location {
    pub fn new() -> Self {
        Self {
            file: None,
            line: None,
            column: None,
            description: DescriptionVO::new(String::new()),
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        if let Some(ref file) = self.file {
            parts.push(file.value.clone());
        }
        if let Some(ref line) = self.line {
            let mut s = line.value.to_string();
            if let Some(ref col) = self.column {
                if col.value > 0 {
                    s = format!("{}:{}", line.value, col.value);
                }
            }
            parts.push(s);
        }
        let result = if parts.is_empty() {
            "unknown".to_string()
        } else {
            parts.join(":")
        };
        if self.description.value.is_empty() {
            write!(f, "{}", result)
        } else {
            write!(f, "{} — {}", result, self.description.value)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LocationList {
    #[serde(default)]
    pub values: Vec<Location>,
}

impl LocationList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
}

impl LocationList {
    pub fn push(&mut self, item: Location) {
        self.values.push(item);
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl std::ops::Deref for LocationList {
    type Target = Vec<Location>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ViolationConstraint {
    pub rule: DescriptionVO,
    #[serde(default)]
    pub min_value: DescriptionVO,
    #[serde(default)]
    pub max_value: DescriptionVO,
}

impl ViolationConstraint {
    pub fn new(rule: impl Into<String>) -> Self {
        Self {
            rule: DescriptionVO::new(rule),
            min_value: DescriptionVO::new(String::new()),
            max_value: DescriptionVO::new(String::new()),
        }
    }
}

impl std::fmt::Display for ViolationConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rule.value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandArgs {
    #[serde(default)]
    pub args: Vec<ContentString>,
}

impl Default for CommandArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandArgs {
    pub fn new() -> Self {
        Self { args: Vec::new() }
    }
}

impl std::fmt::Display for CommandArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.args
                .iter()
                .map(|a| a.value.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeBounds {
    #[serde(default)]
    pub start: Option<LineNumber>,
    #[serde(default)]
    pub end: Option<LineNumber>,
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

## File: crates/shared/src/common/taxonomy_response_data_vo.rs

```rust
// PURPOSE: ResponseData — value object for pipeline job response data
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseData {
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[serde(default)]
    pub stdout: String,
    #[serde(default)]
    pub stderr: String,
    #[serde(default)]
    pub returncode: i64,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Default for ResponseData {
    fn default() -> Self {
        Self::new()
    }
}

impl ResponseData {
    pub fn new() -> Self {
        Self {
            value: None,
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata: HashMap::new(),
        }
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.value.as_ref()
    }
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.value.as_ref().and_then(|v| v.get(key))
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

## File: crates/shared/src/common/taxonomy_suffix_vo.rs

```rust
// PURPOSE: SuffixPolicyVO, SuffixVO — value objects for suffix naming rules
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(transparent)]
pub struct SuffixPolicyVO {
    pub value: String,
}

impl SuffixPolicyVO {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn is_empty(&self) -> bool {
        self.value.trim().is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SuffixVO {
    pub values: crate::common::taxonomy_common_vo::PatternList,
}

impl SuffixVO {
    pub fn new(values: crate::common::taxonomy_common_vo::PatternList) -> Self {
        Self { values }
    }

    pub fn is_empty(&self) -> bool {
        self.values.values.is_empty()
    }
}
```

---

## File: crates/shared/src/common/taxonomy_suggestion_vo.rs

```rust
// PURPOSE: ClassPath, DescriptionVO, LogOutput, MetadataVO, StdError, StdOutput, Suggestion — domain value objects for CLI suggestion/result data
use crate::string_value_object;
use serde::{Deserialize, Serialize};

// ClassPath, DescriptionVO, LogOutput, StdError, StdOutput, and Suggestion all
// follow the standard String-wrapper VO pattern; the macro emits the
// new/value/Display/From/Hash/PartialEq/Deserialize impls they need.
string_value_object!(ClassPath);
string_value_object!(DescriptionVO);
string_value_object!(LogOutput);
string_value_object!(StdError);
string_value_object!(StdOutput);
string_value_object!(Suggestion);

/// Strongly-typed replacement for the previous
/// `HashMap<String, serde_json::Value>` return type. Each field has a real
/// domain meaning — there is no `serde_json::Value` in the contract surface.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetadataVO {
    pub values: std::collections::HashMap<String, serde_json::Value>,
}

impl MetadataVO {
    pub fn new(value: std::collections::HashMap<String, serde_json::Value>) -> Self {
        Self { values: value }
    }
    pub fn value(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.values
    }
}
```

---

## File: crates/shared/src/common/taxonomy_threshold_vo.rs

```rust
// PURPOSE: Threshold — value object for CI compliance threshold percentage
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Threshold {
    pub value: u32,
}

impl Threshold {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

impl From<u32> for Threshold {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl std::fmt::Display for Threshold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Default for Threshold {
    fn default() -> Self {
        Self { value: 100 }
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
    let restrict = workspace_restrict(&root);
    walk_source_files_inner(&root, files, ignored, &mut visited, &root, &restrict)
}

const WORKSPACE_DIRS: [&str; 3] = ["crates", "packages", "modules"];

/// Returns Some(set) of allowed workspace directory names if root contains
/// any of crates/packages/modules. When set, only those subdirectories are
/// scanned — everything else at root level is skipped (avoids walking
/// test-workspaces, scripts, docs, etc.).
fn workspace_restrict(root: &Path) -> Option<HashSet<&str>> {
    let allowed: HashSet<&str> = WORKSPACE_DIRS
        .iter()
        .filter(|d| root.join(d).is_dir())
        .copied()
        .collect();
    if allowed.is_empty() {
        None
    } else {
        Some(allowed)
    }
}

fn walk_source_files_inner(
    dir: &Path,
    files: &mut Vec<FilePath>,
    ignored: &[String],
    visited: &mut HashSet<PathBuf>,
    root: &Path,
    workspace_restrict: &Option<HashSet<&str>>,
) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            // At root level: only descend into workspace dirs (crates/packages/modules)
            if dir == root {
                if let Some(ref restrict) = workspace_restrict {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if path.is_dir() && !restrict.contains(name) {
                            continue;
                        }
                    }
                }
            }

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
                                walk_source_files_inner(
                                    &target,
                                    files,
                                    ignored,
                                    visited,
                                    root,
                                    workspace_restrict,
                                );
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
                walk_source_files_inner(&path, files, ignored, visited, root, workspace_restrict);
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
    let restrict = workspace_restrict(&root);
    walk_rs_files_inner(&root, cb, ignored, &mut visited, &root, &restrict)
}

fn walk_rs_files_inner(
    dir: &Path,
    cb: &mut dyn FnMut(PathBuf),
    ignored: &[String],
    visited: &mut HashSet<PathBuf>,
    root: &Path,
    workspace_restrict: &Option<HashSet<&str>>,
) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();

            // At root level: only descend into workspace dirs (crates/packages/modules)
            if dir == root {
                if let Some(ref restrict) = workspace_restrict {
                    if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                        if p.is_dir() && !restrict.contains(name) {
                            continue;
                        }
                    }
                }
            }

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
                                walk_rs_files_inner(
                                    &target,
                                    cb,
                                    ignored,
                                    visited,
                                    root,
                                    workspace_restrict,
                                );
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
                walk_rs_files_inner(&p, cb, ignored, visited, root, workspace_restrict);
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
/// Tries multiple strategies:
/// 1. Direct segment match with layer names
/// 2. Prefix-based match on segments (e.g., "contract_" → "contract")
/// 3. Filename suffix extraction from path segments
/// 4. Module-path-to-filename resolution for structured projects
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

    // Strategy 2: Prefix-based match on segments
    for part in &meaningful_parts {
        if let Some(layer) = detect_layer_from_prefix(part) {
            return Some(layer);
        }
    }

    // Strategy 3: Extract filename from path and detect layer from filename prefix.
    // Handles patterns like "modules/shared/src/server/contract_protocol" or
    // "mypackage.capabilities_payment_service" where the layer is encoded in
    // the last segment's filename prefix/suffix.
    if let Some(last_part) = meaningful_parts.last() {
        // Handle Rust module paths: "crate::features::payment_service::PaymentService"
        // → extract "payment_service" and detect layer from its suffix
        if last_part.contains("::") {
            if let Some(stem) = last_part.rsplit("::").next() {
                // Check if this segment itself has a prefix (e.g., "contract_")
                if let Some(layer) = detect_layer_from_prefix(stem) {
                    return Some(layer);
                }
            }
        }

        // Handle Python/JS paths: "modules.shared.src.server.contract_protocol"
        // → extract filename, detect layer from prefix
        for part in &meaningful_parts {
            if let Some(layer) = detect_layer_from_prefix(part) {
                return Some(layer);
            }
        }

        // Strategy 4: Try to find layer from the full path by looking for
        // known layer-prefixed segments anywhere in the path.
        // e.g., "shared.src.contract_*" → contract, "shared.src.capabilities_*" → capabilities
        for part in &meaningful_parts {
            if let Some(layer) = detect_layer_from_prefix(part) {
                return Some(layer);
            }
        }

        // Strategy 5: Check if any segment ends with a layer name + underscore
        // (e.g., "contract_protocol" → contract, "capabilities_adapter" → capabilities)
        for part in &meaningful_parts {
            let parts: Vec<&str> = part.split('_').collect();
            for (_i, _seg) in parts.iter().enumerate() {
                let _combined: Vec<&str> = parts[_i..].join("_").split('_').collect();
                // Try combining remaining segments
                let combined_str = parts[_i..].join("_");
                if detect_layer_from_prefix(&combined_str).is_some() {
                    continue;
                }
            }
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

/// Resolve a module path to its layer by scanning the filesystem for
/// layer-prefixed filenames. Handles structured project paths like:
/// - "modules/shared/src/server" → scans for contract_*, capabilities_* files
/// - "mypackage" → scans for taxonomy_*, utility_* files
///
/// # Arguments
/// * `module_path` - The module path from import (e.g., "modules.shared.src.server")
/// * `root_dir` - The project root directory for scanning
///
/// # Returns
/// The detected layer name if a layer-prefixed file is found, None otherwise.
pub fn resolve_module_path_to_layer(module_path: &str, root_dir: &str) -> Option<String> {
    // Convert dotted module path to filesystem path
    let dir_path = root_dir
        .trim_end_matches(std::path::MAIN_SEPARATOR)
        .trim_end_matches('/');

    // Build relative path from module path
    let rel_path = module_path.replace('.', std::path::MAIN_SEPARATOR_STR);

    let scan_dir = format!("{}/{}", dir_path, rel_path);

    // Read directory entries and look for layer-prefixed files
    if let Ok(entries) = std::fs::read_dir(&scan_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    if let Some(filename) = entry.file_name().to_str() {
                        // Check if filename has a layer prefix
                        if let Some(layer) = detect_layer_from_prefix(filename) {
                            return Some(layer);
                        }
                    }
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_module_layer_with_prefix() {
        let layer_names: Vec<String> = vec![
            "taxonomy".into(),
            "contract".into(),
            "utility".into(),
            "capabilities".into(),
            "agent".into(),
            "surface".into(),
        ];

        // Standard module path with layer prefix in segment
        assert_eq!(
            detect_module_layer("shared.src.contract_protocol", &layer_names),
            Some("contract".to_string())
        );
    }

    #[test]
    fn test_resolve_module_path_to_layer() {
        // Test with blender-arwaky structure
        // modules/shared/src/common/ has contract_* and taxonomy_* files
        let result = resolve_module_path_to_layer(
            "modules.shared.src.common",
            "/home/raka/mcp-arwaky/blender-arwaky",
        );
        assert!(
            result.is_some(),
            "Should detect layer from common directory (has contract_* and taxonomy_* files)"
        );
        assert!(
            result.as_ref().unwrap() == "contract" || result.as_ref().unwrap() == "taxonomy",
            "Detected layer should be contract or taxonomy"
        );
    }
}
```

---

## File: crates/shared/src/common/utility_value_object_generator.rs

```rust
// PURPOSE: Macros for generating boilerplate impls on String/primitive wrapper value objects.
//
// These macros emit the impls that every String-wrapper VO needs:
//   - `new(value)` constructor
//   - `value()` accessor
//   - `Display`
//   - `Hash` / `PartialEq` / `Eq` (optional)
//   - `From<&str>` / `From<String>` / `From<$Inner>` (for primitives)
//   - serde `Deserialize` (accepts either a primitive or a map with a `value` key)
//
// Using the macro keeps each VO file to its domain-specific surface and stops
// AES305 from flagging the same serde visitor across ~13 files.

/// Generate a String-wrapped value object with the standard VO surface.
///
/// # Usage
/// ``` `ignore
/// // in any sibling module file:
/// use crate::string_value_object;
/// string_value_object!(FooName);
/// ``` `
///
/// The macro is `#[macro_export]`-ed so it is accessible at the crate root.
/// Each VO file `use crate::string_value_object;` once and then invokes the
/// macro locally.
#[macro_export]
macro_rules! string_value_object {
    ($name:ident) => {
        #[derive(Default, Debug, Clone, serde::Serialize)]
        #[serde(transparent)]
        pub struct $name {
            pub value: String,
        }

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self {
                    value: value.into(),
                }
            }

            pub fn value(&self) -> &str {
                &self.value
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.value.hash(state);
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        }

        impl Eq for $name {}

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self {
                    value: s.to_string(),
                }
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self { value: s }
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct V {}
                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = $name;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("primitive or map with 'value' key")
                    }
                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name {
                            value: v.to_string(),
                        })
                    }
                    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v })
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
                        Ok($name { value: val })
                    }
                }
                deserializer.deserialize_any(V {})
            }
        }
    };
}

/// Generate a primitive-wrapped value object (e.g. `i64`, `f64`, `bool`).
///
/// # Usage
/// ``` `ignore
/// primitive_value_object!(LineNumber, i64);
/// ``` `
///
/// Emits the same surface as `string_value_object!` but with `From<$Inner>`,
/// `From<$Inner>` conversions, and a serde visitor that accepts the inner
/// type or a `{"value": ...}` map.
#[macro_export]
macro_rules! primitive_value_object {
    ($name:ident, $inner:ty) => {
        #[derive(Default, Debug, Clone, serde::Serialize)]
        #[serde(transparent)]
        pub struct $name {
            pub value: $inner,
        }

        impl $name {
            pub fn new(value: $inner) -> Self {
                Self { value }
            }

            pub fn value(&self) -> $inner {
                self.value
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        }

        impl Eq for $name {}

        impl From<$inner> for $name {
            fn from(v: $inner) -> Self {
                Self { value: v }
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct V {}
                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = $name;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(concat!(
                            "primitive or map with 'value' key (",
                            stringify!($inner),
                            ")"
                        ))
                    }
                    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v as $inner })
                    }
                    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v as $inner })
                    }
                    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v as $inner })
                    }
                    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name { value: v as $inner })
                    }
                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        let mut value: Option<$inner> = None;
                        while let Some(k) = map.next_key::<String>()? {
                            if k == "value" {
                                value = Some(map.next_value::<$inner>()?);
                            } else {
                                let _: serde::de::IgnoredAny = map.next_value()?;
                            }
                        }
                        let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                        Ok($name { value: val })
                    }
                }
                deserializer.deserialize_any(V {})
            }
        }
    };
}

pub fn is_generator_enabled() -> bool {
    true
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

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
pub use contract_parser_protocol::IConfigParserProtocol;
pub use contract_reader_protocol::IConfigReaderProtocol;
pub use contract_validator_protocol::IConfigValidatorProtocol;
pub use contract_workspace_detector_protocol::IWorkspaceDetectorProtocol;
pub use contract_workspace_detector_protocol::WorkspaceType;

// ── Taxonomy types ──
pub use taxonomy_config_error::ConfigError;
pub use taxonomy_config_language_vo::ConfigLanguage;
pub use taxonomy_config_vo::ArchitectureConfig;
pub use taxonomy_config_vo::ArchitectureRule;
pub use taxonomy_config_vo::NamingRuleVO;
pub use taxonomy_config_vo::OrphanRuleVO;
pub use taxonomy_config_vo::RoleRuleVO;
pub use taxonomy_identifier_vo::ConfigKey;
pub use taxonomy_multi_project_summary_vo::AggregatedResults;
pub use taxonomy_multi_project_summary_vo::ProjectResult;
pub use taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
pub use taxonomy_setting_vo::AdapterEntry;
pub use taxonomy_setting_vo::AdapterStatus;
pub use taxonomy_setting_vo::ProjectConfig;
pub use taxonomy_setting_vo::Thresholds;
pub use taxonomy_source_vo::ConfigResult;
pub use taxonomy_source_vo::ConfigSource;
pub use taxonomy_validation_vo::ValidationResult;
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

## File: crates/shared/src/orphan-detector/contract_orphan_aggregate.rs

```rust
// PURPOSE: IOrphanAggregate — aggregate trait for orphan detection (AES308)
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use crate::common::taxonomy_path_vo::FilePath;
use crate::orphan_detector::taxonomy_orphan_contract_vo::OrphanFileListVO;

/// Aggregate that detects orphan (unreferenced) files in a project.
///
/// AES308 requires that every source file be reachable from at least one
/// entry point. This aggregate builds a dependency graph, identifies
/// orphan entry points, and reports violations.
pub trait IOrphanAggregate: Send + Sync {
    fn build_orphan_graph_context(
        &self,
        files: &OrphanFileListVO,
        root_dir: &FilePath,
    ) -> GraphAnalysisContext;
    fn identify_orphan_entry_points(&self, files: &OrphanFileListVO) -> OrphanFileListVO;
    fn check_orphans(&self, files: &OrphanFileListVO, root_dir: &FilePath) -> Vec<LintResult>;
    /// Check orphans using a pre-built graph context (avoids rebuilding inbound_links per call)
    fn check_orphans_with_context(
        &self,
        files: &OrphanFileListVO,
        root_dir: &FilePath,
        context: &GraphAnalysisContext,
    ) -> Vec<LintResult>;
    /// Scan a directory and run orphan detection — encapsulates all file I/O.
    /// Takes a root path and ignored path patterns (as string slices).
    /// Returns (graph_context, orphan_results) — context can be reused for filtered queries.
    fn scan_orphans(
        &self,
        root_dir: &FilePath,
        ignored: &[String],
    ) -> (GraphAnalysisContext, Vec<LintResult>);
}
```

---

## File: crates/shared/src/orphan-detector/contract_orphan_graph_resolver_protocol.rs

```rust
// PURPOSE: IOrphanGraphResolverProtocol — contract trait for building orphan analysis graph context
// AES402: All primitive `&[String]` parameter types and `Vec<String>` return
// types in this contract have been replaced with strongly-typed VOs.
//   * `&[String] files` → `&[OrphanFileListVO]` (per analysis pass)
//   * `Vec<String>` returns → `OrphanFileListVO`
//   * `&[String] configured` → `&[OrphanEntryPatternListVO]`
//   * `&str root_dir` → kept as `&str` (idiomatic borrow, AES402 allows)
use crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use crate::orphan_detector::taxonomy_orphan_contract_vo::{
    OrphanEntryPatternListVO, OrphanFileListVO,
};

pub trait IOrphanGraphResolverProtocol: Send + Sync {
    /// Build the orphan-detection graph context for a set of source files.
    /// `files` is the list of file paths to include in the graph; `root_dir`
    /// is the project root used to compute relative paths.
    fn build_graph_context(
        &self,
        files: &[OrphanFileListVO],
        root_dir: &str,
    ) -> GraphAnalysisContext;

    /// Identify which of the supplied files count as entry points. A file
    /// is an entry point if its path matches any of the configured patterns
    /// (substring or suffix match). Returns the filtered list as a
    /// strongly-typed VO.
    fn identify_entry_points(
        &self,
        files: &[OrphanFileListVO],
        configured: &[OrphanEntryPatternListVO],
    ) -> OrphanFileListVO;
}
```

---

## File: crates/shared/src/orphan-detector/contract_orphan_protocol.rs

```rust
// PURPOSE: ITaxonomyOrphanProtocol + layer-specific orphan indicator protocols (agent, contract, capabilities, utility, surfaces)
use crate::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use crate::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use crate::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use crate::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use crate::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_path_vo::FilePath;

pub trait ITaxonomyOrphanProtocol: Send + Sync {
    fn is_taxonomy_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        definition: Option<&LayerDefinition>,
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult;
}

pub trait IContractOrphanProtocol: Send + Sync {
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        file_definitions: &FileDefinitionMap,
        inheritance_map: &InheritanceMap,
        all_files: &[String],
    ) -> OrphanIndicatorResult;
}

pub trait ICapabilitiesOrphanProtocol: Send + Sync {
    fn is_capabilities_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
    ) -> OrphanIndicatorResult;
}

pub trait IUtilityOrphanProtocol: Send + Sync {
    fn is_utility_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        all_files: &[String],
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult;
}

pub trait IAgentOrphanProtocol: Send + Sync {
    fn is_agent_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        all_files: &[String],
    ) -> OrphanIndicatorResult;
}

pub trait ISurfacesOrphanProtocol: Send + Sync {
    fn is_surface_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        alive_files: &ReachabilityResult,
        definition: Option<&LayerDefinition>,
    ) -> OrphanIndicatorResult;
}
```

---

## File: crates/shared/src/orphan-detector/mod.rs

```rust
pub mod contract_orphan_aggregate;
pub mod contract_orphan_graph_resolver_protocol;
pub mod contract_orphan_protocol;
pub mod taxonomy_orphan_contract_vo;
pub mod taxonomy_violation_orphan_vo;
pub mod utility_file_cache;
pub mod utility_orphan_detector;
pub mod utility_orphan_filename;
pub mod utility_orphan_io;
pub mod utility_orphan_path;
pub mod utility_workspace_scanner;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_orphan_aggregate::IOrphanAggregate;
pub use contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
pub use contract_orphan_protocol::IAgentOrphanProtocol;
pub use contract_orphan_protocol::ICapabilitiesOrphanProtocol;
pub use contract_orphan_protocol::IContractOrphanProtocol;
pub use contract_orphan_protocol::ISurfacesOrphanProtocol;
pub use contract_orphan_protocol::ITaxonomyOrphanProtocol;
pub use contract_orphan_protocol::IUtilityOrphanProtocol;

// ── Taxonomy types ──
pub use taxonomy_orphan_contract_vo::OrphanEntryPatternListVO;
pub use taxonomy_orphan_contract_vo::OrphanFileListVO;
pub use taxonomy_violation_orphan_vo::AesOrphanViolation;
```

---

## File: crates/shared/src/orphan-detector/taxonomy_orphan_contract_vo.rs

```rust
// PURPOSE: OrphanContractVOs — value objects used by IOrphanGraphResolverProtocol.
//
// AES402: All primitive `&[String]` / `Vec<String>` parameter types and return
// types in IOrphanGraphResolverProtocol are replaced with strongly-typed VOs
// so the contract surface has no primitive collections.
//
// Why a dedicated VO instead of reusing `FilePathList` or `PatternList`?
//   * `FilePathList` (source_parsing/taxonomy_paths_vo) wraps `Vec<FilePath>`,
//     but the orphan graph resolver receives and emits file paths as `String`
//     (it does not own the underlying file system resolution — the surface
//     layer feeds it raw strings from a directory walk).
//   * `PatternList` (common/taxonomy_common_vo) wraps `Vec<String>` but is
//     semantically about exclusion patterns, not about file or pattern
//     identifiers in a graph context.
//
// The two VOs below mirror the parameter roles of the contract:
//   * `OrphanFileListVO` — list of file paths under analysis
//   * `OrphanEntryPatternListVO` — list of configured entry-point patterns
// Both are intentionally minimal wrappers around `Vec<String>`; the point
// is to take the *name* of the field out of the contract surface and put
// it in a typed wrapper, not to invent new functionality.
use serde::{Deserialize, Serialize};

/// List of file paths under orphan-detection analysis. Wraps `Vec<String>`
/// (raw path strings as emitted by the directory walker). Replaces the
/// previous `&[String]` parameter and `Vec<String>` return type used in
/// `IOrphanGraphResolverProtocol::build_graph_context` and
/// `identify_entry_points`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrphanFileListVO {
    pub values: Vec<String>,
}

impl OrphanFileListVO {
    pub fn new(values: Vec<String>) -> Self {
        Self { values }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.values.iter()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// List of configured entry-point patterns (e.g. glob prefixes or exact
/// paths) the resolver should treat as reachable entry points. Replaces
/// the previous `&[String]` parameter on
/// `IOrphanGraphResolverProtocol::identify_entry_points`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrphanEntryPatternListVO {
    pub values: Vec<String>,
}

impl OrphanEntryPatternListVO {
    pub fn new(values: Vec<String>) -> Self {
        Self { values }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.values.iter()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}
```

---

## File: crates/shared/src/orphan-detector/taxonomy_violation_orphan_vo.rs

```rust
use crate::common::taxonomy_message_vo::LintMessage;
use std::fmt;

#[derive(Debug, Clone)]
pub enum AesOrphanViolation {
    TaxonomyOrphan {
        stem: String,
        category: &'static str,
        reason: Option<LintMessage>,
    },
    ContractOrphan {
        suffix: String,
        trait_name: String,
        target_layer: &'static str,
        reason: Option<LintMessage>,
    },
    CapabilitiesOrphan {
        stem: String,
        reason: Option<LintMessage>,
    },
    UtilityOrphan {
        stem: String,
        reason: Option<LintMessage>,
    },
    UtilityDeadCode {
        stem: String,
        imported_by: Vec<String>,
        reason: Option<LintMessage>,
    },
    AgentOrphan {
        agg_name: String,
        reason: Option<LintMessage>,
    },
    SurfaceOrphan {
        category: &'static str,
        stem: String,
        reason: Option<LintMessage>,
    },
}

impl fmt::Display for AesOrphanViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AesOrphanViolation::TaxonomyOrphan {
                stem,
                category,
                reason,
            } => {
                let target_hint = match *category {
                    "utility" | "helper" => "any file that needs its functionality".to_string(),
                    _ => "a contract_* file (contract_port, contract_protocol, or contract_aggregate)".to_string(),
                };
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => {
                        format!("Taxonomy file '{}' is not imported by any file.", stem)
                    }
                };
                write!(f, "AES501 TAXONOMY_ORPHAN: '{}' is not imported.\nWHY? {}\nFIX: Import '{}' in {}.", stem, why, stem, target_hint)
            }
            AesOrphanViolation::ContractOrphan {
                suffix,
                trait_name,
                target_layer,
                reason,
            } => {
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => format!(
                        "Contract {} '{}' is not implemented by any {} file.",
                        suffix, trait_name, target_layer
                    ),
                };
                let fix = match suffix.as_str() {
                    "protocol" => format!("Implement '{}' in a capabilities_* file, or wire it in agent_*_orchestrator.rs if already implemented.", trait_name),
                    "aggregate" => format!("Import and use '{}' in a surface_* file or root_*_container.rs.", trait_name),
                    _ => format!("Implement '{}' in the appropriate layer.", trait_name),
                };
                write!(
                    f,
                    "AES502 CONTRACT_ORPHAN: Contract {} '{}' is orphaned.\nWHY? {}\nFIX: {}",
                    suffix, trait_name, why, fix
                )
            }
            AesOrphanViolation::CapabilitiesOrphan { stem, reason } => {
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => format!(
                        "Capabilities file '{}' is not wired in any container.",
                        stem
                    ),
                };
                write!(f, "AES503 CAPABILITIES_ORPHAN: '{}' is not wired.\nWHY? {}\nFIX: Register '{}' in root_*_entry.rs or root_*_container.rs via `use {}::...;` and wire it into the container's constructor. If this file is obsolete, delete it and remove its module declaration from lib.rs.", stem, why, stem, stem)
            }
            AesOrphanViolation::UtilityOrphan { stem, reason } => {
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => format!(
                        "Utility file '{}' is not imported by any capabilities or other layer file.",
                        stem
                    ),
                };
                write!(f, "AES504 UTILITY_ORPHAN: '{}' is not imported.\nWHY? {}\nFIX: Import '{}' in a capabilities_* file that needs its functionality. Utility files must be consumed by other layers. If this file is obsolete, delete it and remove its module declaration from lib.rs.", stem, why, stem)
            }
            AesOrphanViolation::UtilityDeadCode {
                stem,
                imported_by,
                reason,
            } => {
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => {
                        let importers = imported_by.join(", ");
                        format!(
                            "Utility file '{}' is only imported by other utility files ({}), not by capability, agent, or surfaces layers.",
                            stem, importers
                        )
                    }
                };
                write!(f, "AES504 UTILITY_DEAD_CODE: '{}' has no consumers in capability/agent/surfaces layers.\nWHY? {}\nFIX: Import '{}' in a capabilities_* file that needs its functionality, or delete it if unused. Utility files must be consumed by higher layers, not just other utilities.", stem, why, stem)
            }
            AesOrphanViolation::AgentOrphan { agg_name, reason } => {
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => format!(
                        "Agent aggregate '{}' is not called by any surface or container.",
                        agg_name
                    ),
                };
                write!(f, "AES505 AGENT_ORPHAN: Aggregate '{}' is unreachable from any surface.\nWHY? {}\nFIX: Import and use '{}' in a surface_* file or root_*_container.rs via `Arc<dyn {}>`. If the orchestrator is unused, delete it and remove its module declaration.", agg_name, why, agg_name, agg_name)
            }
            AesOrphanViolation::SurfaceOrphan {
                category,
                stem,
                reason,
            } => {
                let (where_hint, fix_hint) = match *category {
                    "smart" => ("entry point or router", "an entry point (root_*_entry.rs, cli_*, mcp_*) or router file"),
                    "utility" => ("smart surface", "a smart surface (command, controller, page)"),
                    "passive" => ("smart or utility surface", "a smart surface (command, controller, page) or utility surface (hook, store, action, screen, router)"),
                    _ => ("the appropriate importer", "an appropriate importer file"),
                };
                let why = match reason.as_ref() {
                    Some(r) => r.to_string(),
                    None => format!(
                        "{} surface '{}' is not imported by any {}.",
                        category, stem, where_hint
                    ),
                };
                write!(f, "AES506 SURFACE_ORPHAN: {} surface '{}' is orphaned.\nWHY? {}\nFIX: Import '{}' in {}. If this surface is dead code, remove the file and its module declaration from lib.rs.", category, stem, why, stem, fix_hint)
            }
        }
    }
}

impl From<AesOrphanViolation> for String {
    fn from(v: AesOrphanViolation) -> String {
        v.to_string()
    }
}
```

---

## File: crates/shared/src/orphan-detector/utility_file_cache.rs

```rust
// PURPOSE: Orphan file cache utility — stateless interface to bounded file cache
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use std::collections::HashMap;
use std::fs;
use std::sync::{Mutex, OnceLock};

const MAX_CACHE_ENTRIES: usize = 20_000;

static FILE_CACHE: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

fn cache() -> &'static Mutex<HashMap<String, String>> {
    FILE_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn read_cached(path: &FilePath) -> ContentString {
    let mut cache = cache().lock().unwrap_or_else(|e| e.into_inner());

    if let Some(content) = cache.get(path.value()) {
        return ContentString::new(content.clone());
    }

    let content = fs::read_to_string(path.value()).unwrap_or_default();

    if cache.len() < MAX_CACHE_ENTRIES {
        cache.insert(path.value().to_string(), content.clone());
    }

    ContentString::new(content)
}

pub fn read_dir(dir_path: &FilePath) -> Vec<FilePath> {
    let mut entries = Vec::new();
    if let Ok(read_dir) = fs::read_dir(dir_path.value()) {
        for entry in read_dir.flatten() {
            if let Some(s) = entry.path().to_str() {
                if let Ok(fp) = FilePath::new(s) {
                    entries.push(fp);
                }
            }
        }
    }
    entries
}

pub fn is_symlink(path: &FilePath) -> bool {
    std::fs::symlink_metadata(path.value())
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
}

pub fn clear_cache() {
    let mut cache = cache().lock().unwrap_or_else(|e| e.into_inner());
    cache.clear();
}
```

---

## File: crates/shared/src/orphan-detector/utility_orphan_detector.rs

```rust
use once_cell::sync::OnceCell;
use regex::Regex;

static STRUCT_RE: OnceCell<Option<Regex>> = OnceCell::new();
static TRAIT_RE: OnceCell<Option<Regex>> = OnceCell::new();

fn struct_re() -> Option<&'static Regex> {
    STRUCT_RE
        .get_or_init(|| Regex::new(r"(?:pub\s+)?struct\s+([A-Za-z0-9_]+)").ok())
        .as_ref()
}

fn trait_re() -> Option<&'static Regex> {
    TRAIT_RE
        .get_or_init(|| Regex::new(r"(?:pub\s+)?trait\s+([A-Za-z0-9_]+)").ok())
        .as_ref()
}

pub fn extract_struct_names(content: &str) -> Vec<String> {
    let mut names = Vec::new();
    if let Some(re) = struct_re() {
        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            if name != "Self" && !name.is_empty() {
                names.push(name);
            }
        }
    }
    names
}

pub fn extract_trait_names(content: &str) -> Vec<String> {
    let mut names = Vec::new();
    if let Some(re) = trait_re() {
        for cap in re.captures_iter(content) {
            names.push(cap[1].to_string());
        }
    }
    names
}

pub fn normalize_module_component(value: &str) -> String {
    value.replace(['-', '.'], "_")
}

pub fn normalize_module_path(value: &str) -> String {
    value
        .split('/')
        .map(normalize_module_component)
        .collect::<Vec<_>>()
        .join("/")
}

pub fn contains_delimited(content: &str, token: &str) -> bool {
    if !content.contains(token) {
        return false;
    }

    let delimiters: &[char] = &[
        ' ', '\t', '\n', '\r', ';', ',', '(', ')', '{', '}', '"', '\'', ':', '.',
    ];

    for (idx, _) in content.char_indices() {
        // Only check at character boundaries (skip multi-byte sequences)
        if idx > 0 && !is_char_boundary(content, idx) {
            continue;
        }

        let remaining = &content[idx..];
        if !remaining.starts_with(token) {
            continue;
        }

        let before = if idx == 0 {
            ' '
        } else {
            // Safe: char_indices guarantees idx is at a valid char boundary
            content[..idx].chars().next_back().unwrap_or(' ')
        };

        let after_pos = idx + token.chars().map(|c| c.len_utf8()).sum::<usize>();
        let after = content[after_pos..].chars().next().unwrap_or(' ');

        let boundary_before = before.is_whitespace() || delimiters.contains(&before);
        let boundary_after =
            after.is_whitespace() || delimiters.contains(&after) || after == '\n' || after == '\r';

        if boundary_before && boundary_after {
            return true;
        }
    }

    false
}

fn is_char_boundary(s: &str, pos: usize) -> bool {
    if pos >= s.len() {
        return true;
    }
    let bytes = s.as_bytes();
    (bytes[pos] & 0xC0) != 0x80
}

pub fn import_tokens(path: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let path = path.replace('\\', "/");
    let path = path.trim_start_matches('/');

    let stem = std::path::Path::new(&path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();
    if stem.is_empty() {
        return tokens;
    }

    tokens.push(stem.clone());

    let normalized_stem = normalize_module_component(&stem);
    if normalized_stem != stem {
        tokens.push(normalized_stem);
    }

    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() >= 2 {
        let parent = parts[parts.len() - 2];
        let partial = format!("{}/{}", parent, stem);
        tokens.push(partial.clone());
        tokens.push(partial.replace('/', "::"));

        let normalized_partial = normalize_module_path(&partial);
        if normalized_partial != partial {
            tokens.push(normalized_partial.clone());
            tokens.push(normalized_partial.replace('/', "::"));
        }
    }

    for i in 2..parts.len() {
        let partial = parts[parts.len() - i..].join("/");
        tokens.push(partial);
    }

    let source_prefixes = ["crate::", "shared::", "self::", "super::"];
    let existing: Vec<String> = tokens.clone();
    for prefix in &source_prefixes {
        for tok in &existing {
            tokens.push(format!("{prefix}{tok}"));
        }
    }

    tokens.sort();
    tokens.dedup();
    tokens
}

/// Strip leading generic parameter lists (e.g., `<T>`, `<T: Clone>`) from a string.
fn strip_leading_generics(s: &str) -> &str {
    let mut s = s.trim();

    while let Some(rest) = s.strip_prefix('<') {
        let mut depth = 1usize;
        let mut end = None;

        for (idx, ch) in rest.char_indices() {
            match ch {
                '<' => depth += 1,
                '>' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(idx);
                        break;
                    }
                }
                _ => {}
            }
        }

        match end {
            Some(pos) => s = rest[pos + 1..].trim(),
            None => break,
        }
    }

    s
}

pub fn has_trait_implementation(content: &str, trait_name: &str) -> bool {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("//")
            || trimmed.starts_with("/*")
            || trimmed.starts_with('*')
            || trimmed.starts_with('#')
        {
            continue;
        }
        if trimmed.starts_with("impl") && trimmed.contains(" for ") {
            let after_impl = trimmed[4..].trim();

            let trait_part = match after_impl.find(" for ") {
                Some(pos) => after_impl[..pos].trim(),
                None => continue,
            };

            let trait_part = strip_leading_generics(trait_part);
            let trait_base = trait_part.split('<').next().unwrap_or(trait_part).trim();
            let trait_last = trait_base.split("::").last().unwrap_or(trait_base);

            if trait_last == trait_name
                || trait_last.ends_with(trait_name)
                || trait_name.ends_with(trait_last)
            {
                return true;
            }
        }
        if let Some(class_pos) = trimmed.find("class ") {
            let after_class = &trimmed[class_pos + 6..];
            if let Some(paren_pos) = after_class.find('(') {
                let bases = &after_class[paren_pos + 1..];
                if let Some(close_paren) = bases.find(')') {
                    for base in bases[..close_paren].split(',') {
                        if base.trim() == trait_name {
                            return true;
                        }
                    }
                }
            }
        }
        if let Some(impl_pos) = trimmed.find(" implements ") {
            let after_impl = &trimmed[impl_pos + 13..];
            for iface in after_impl.split(',') {
                let iface = iface.trim().trim_end_matches('{').trim();
                if iface == trait_name {
                    return true;
                }
            }
        }
    }
    false
}
```

---

## File: crates/shared/src/orphan-detector/utility_orphan_filename.rs

```rust
// PURPOSE: Pure filename utility functions for orphan detection (AES layer naming)
// These are stateless, domain-agnostic, reusable across multiple capabilities.

/// Extract basename from path: "crates/shared/src/lib.rs" → "lib.rs"
pub fn file_basename(path: &str) -> String {
    match path.rsplit('/').next() {
        Some(f) => f.to_string(),
        None => path.to_string(),
    }
}

/// Extract stem from path: "checker.rs" → "checker", "capabilities_checker.rs" → "capabilities_checker"
pub fn file_stem(path: &str) -> String {
    let base = file_basename(path);
    if let Some(pos) = base.rfind('.') {
        base[..pos].to_string()
    } else {
        base
    }
}

/// Extract suffix after last underscore in stem: "capabilities_checker.rs" → "checker"
pub fn file_suffix(path: &str) -> String {
    let st = file_stem(path);
    match st.rfind('_') {
        Some(pos) => st[pos + 1..].to_string(),
        None => String::new(),
    }
}
```

---

## File: crates/shared/src/orphan-detector/utility_orphan_io.rs

```rust
// PURPOSE: utility_orphan_io — stateless I/O utilities for orphan detection graph building
use std::path::Path;

/// Read file contents, returning empty string on error (backward compatible).
pub fn read_file_safe(path: &str) -> String {
    crate::common::utility_file_handler::read_file_safe(path)
}

/// Read file with diagnostic info — returns content or error details.
pub fn read_file_with_diagnostic(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|err| format!("{}: {}", path, err))
}

/// List directory entries, skipping hidden files (starting with '.').
/// Returns vector of (file_name, file_path, is_dir) tuples.
pub fn list_directory_entries(dir_path: &Path) -> Vec<(String, String, bool)> {
    let mut entries = Vec::new();
    if let Ok(read_dir) = dir_path.read_dir() {
        for dir_entry in read_dir.flatten() {
            if let Some(name) = dir_entry.file_name().to_str() {
                if name.starts_with('.') {
                    continue;
                }
                let path = dir_entry.path();
                let is_dir = crate::common::utility_file_handler::is_dir(&path);
                entries.push((name.to_string(), path.to_string_lossy().to_string(), is_dir));
            }
        }
    }
    entries
}

/// Check if a path exists and is a file.
pub fn is_file(path: &Path) -> bool {
    crate::common::utility_file_handler::is_file_generic(path)
}

/// Check if a path exists and is a directory.
pub fn is_dir(path: &Path) -> bool {
    crate::common::utility_file_handler::is_dir(path)
}

/// Scan directory entries, returning vector of (file_name, file_path, is_dir) tuples.
/// Returns empty vec on error (same as list_directory_entries).
pub fn scan_directory(dir_path: &Path) -> Vec<(String, String, bool)> {
    let mut entries = Vec::new();
    if let Ok(read_dir) = dir_path.read_dir() {
        for dir_entry in read_dir.flatten() {
            if let Some(name) = dir_entry.file_name().to_str() {
                let path = dir_entry.path();
                let is_dir = crate::common::utility_file_handler::is_dir(&path);
                entries.push((name.to_string(), path.to_string_lossy().to_string(), is_dir));
            }
        }
    }
    entries
}

/// Recursively scan directory for files, returning vector of file paths.
/// Skips hidden directories and common heavy dependency/build directories.
pub fn scan_directory_recursive(dir_path: &Path) -> Vec<String> {
    let mut files = Vec::new();

    if let Ok(entries) = dir_path.read_dir() {
        for dir_entry in entries.flatten() {
            if let Some(name) = dir_entry.file_name().to_str() {
                if name.starts_with('.') {
                    continue;
                }

                let path = dir_entry.path();

                if crate::common::utility_file_handler::is_dir(&path) {
                    if matches!(
                        name,
                        "target" | "node_modules" | "dist" | "build" | "__pycache__" | ".venv"
                    ) {
                        continue;
                    }

                    files.extend(scan_directory_recursive(&path));
                } else if let Some(path_str) = path.to_str() {
                    files.push(path_str.to_string());
                }
            }
        }
    }

    files
}
```

---

## File: crates/shared/src/orphan-detector/utility_orphan_path.rs

```rust
use std::fs;
use std::path::{Component, Path, PathBuf};

pub fn normalize_lexical(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}

pub fn confine_under_root(root: &Path, candidate: &Path) -> Option<PathBuf> {
    let canonical_root = fs::canonicalize(root).ok()?;

    let absolute = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        canonical_root.join(candidate)
    };

    // If the candidate exists, canonicalize it directly.
    if let Ok(canonical_candidate) = fs::canonicalize(&absolute) {
        return canonical_candidate
            .starts_with(&canonical_root)
            .then_some(canonical_candidate);
    }

    // If the candidate does not exist yet, canonicalize the parent
    // and reattach the final component.
    let parent = absolute.parent()?;
    let file_name = absolute.file_name()?;

    let canonical_parent = fs::canonicalize(parent).ok()?;
    let canonical_candidate = canonical_parent.join(file_name);

    canonical_candidate
        .starts_with(&canonical_root)
        .then_some(canonical_candidate)
}

pub fn resolve_module_path(root: &Path, base_dir: &Path, module_path: &str) -> Option<PathBuf> {
    let candidate = if Path::new(module_path).is_absolute() {
        PathBuf::from(module_path)
    } else {
        base_dir.join(module_path)
    };
    confine_under_root(root, &candidate)
}

pub fn is_path_ignored(file: &str, patterns: &[String]) -> bool {
    let file = file.replace('\\', "/");
    patterns.iter().any(|pattern| {
        let raw = pattern.replace('\\', "/");
        if raw.is_empty() {
            return false;
        }
        if file == raw || file.ends_with(&raw) {
            return true;
        }
        let normalized = raw.trim_start_matches('/');
        if normalized.is_empty() {
            return false;
        }
        file.starts_with(&format!("{normalized}/"))
            || file.contains(&format!("/{normalized}/"))
            || file.contains(&format!("/{normalized}"))
    })
}
```

---

## File: crates/shared/src/orphan-detector/utility_workspace_scanner.rs

```rust
// PURPOSE: Workspace utility — locate workspace root and verify container wiring without dependency injection
use crate::common::taxonomy_path_vo::FilePath;
use crate::orphan_detector::utility_file_cache;

/// Walk parent directories from `start` to locate the workspace root:
/// a directory that holds a member dir (crates/packages/modules) AND a
/// manifest (Cargo.toml / package.json / pyproject.toml).
pub fn find_workspace_root(start: &std::path::Path) -> Result<std::path::PathBuf, std::io::Error> {
    let member_dirs = ["crates", "packages", "modules"];
    let mut current = start.to_path_buf();
    loop {
        let has_cargo = current.join("Cargo.toml").exists();
        let has_package_json = current.join("package.json").exists();
        let has_pyproject = current.join("pyproject.toml").exists();
        let has_member_dir = member_dirs.iter().any(|d| current.join(d).is_dir());

        if has_member_dir && (has_cargo || has_package_json || has_pyproject) {
            return Ok(current);
        }

        if !current.pop() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "workspace root not found",
            ));
        }
    }
}

/// Returns true if any container/entry file under the workspace root references
/// one of `identifiers`.
pub fn check_wired_in_container(workspace_root: &std::path::Path, identifiers: &[String]) -> bool {
    for dir_name in &["crates", "packages", "modules"] {
        let dir = workspace_root.join(dir_name);
        if dir.is_dir() && check_dir_containers(&dir, identifiers) {
            return true;
        }
    }
    false
}

fn check_dir_containers(dir: &std::path::Path, identifiers: &[String]) -> bool {
    if let Ok(fp) = FilePath::new(dir.to_str().unwrap_or("")) {
        let entries = utility_file_cache::read_dir(&fp);
        for entry_path in &entries {
            let path = std::path::Path::new(entry_path.value());
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                if matches!(
                    name,
                    "target"
                        | ".git"
                        | "node_modules"
                        | "dist"
                        | "build"
                        | "__pycache__"
                        | ".venv"
                        | "tests"
                ) {
                    continue;
                }

                if check_dir_containers(path, identifiers) {
                    return true;
                }
            } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.ends_with("_container.rs")
                    || name.ends_with("_container.py")
                    || name.ends_with("_container.ts")
                    || name.ends_with("_container.js")
                    || name.ends_with("_entry.rs")
                    || name.ends_with("_entry.py")
                    || name.ends_with("_entry.ts")
                    || name.ends_with("_entry.js")
                {
                    let fp = FilePath {
                        value: entry_path.value.clone(),
                    };
                    let content = utility_file_cache::read_cached(&fp).value;
                    for id in identifiers {
                        if content.contains(id) {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

/// Walk directory and collect paths of all source files (*.rs, *.py, *.ts, *.js, etc.)
pub fn collect_source_files(dir: &std::path::Path, files: &mut Vec<String>) {
    if let Ok(fp) = FilePath::new(dir.to_str().unwrap_or("")) {
        let entries = utility_file_cache::read_dir(&fp);
        for entry_path in &entries {
            let path = std::path::Path::new(entry_path.value());
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name == "target" || name == ".git" || name == "node_modules" || name == "tests" {
                    continue;
                }
                collect_source_files(path, files);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx") {
                    files.push(entry_path.value().to_string());
                }
            }
        }
    }
}
```

---

## File: crates/shared/src/role-rules/mod.rs

```rust
// role-rules — taxonomy and contract types
pub mod contract_agent_role_protocol;
pub mod contract_capabilities_role_protocol;
pub mod contract_role_contract_protocol;
pub mod contract_role_runner_aggregate;
pub mod contract_surface_role_protocol;
pub mod contract_taxonomy_role_protocol;
pub mod contract_utility_role_protocol;
pub mod taxonomy_layer_names_constant;
pub mod taxonomy_layer_names_vo;
pub mod taxonomy_violation_role_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_agent_role_protocol::IAgentRoleChecker;
pub use contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
pub use contract_role_contract_protocol::IContractRoleChecker;
pub use contract_role_runner_aggregate::IRoleRunnerAggregate;
pub use contract_surface_role_protocol::ISurfaceRoleChecker;
pub use contract_taxonomy_role_protocol::ITaxonomyRoleChecker;
pub use contract_utility_role_protocol::IUtilityRoleChecker;

// ── Taxonomy types ──
pub use taxonomy_layer_names_constant::LAYER_AGENT;
pub use taxonomy_layer_names_constant::LAYER_CAPABILITIES;
pub use taxonomy_layer_names_constant::LAYER_CONTRACT;
pub use taxonomy_layer_names_constant::LAYER_GLOBAL;
pub use taxonomy_layer_names_constant::LAYER_ROOT;
pub use taxonomy_layer_names_constant::LAYER_SURFACES;
pub use taxonomy_layer_names_constant::LAYER_TAXONOMY;
pub use taxonomy_layer_names_constant::LAYER_UTILITY;
pub use taxonomy_layer_names_vo::layer_surfaces;
pub use taxonomy_violation_role_vo::AesRoleViolation;
pub use taxonomy_violation_role_vo::LabeledRoleViolation;
```

---

