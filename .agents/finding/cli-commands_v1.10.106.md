# Crate: cli-commands (v1.10.106)

This document contains the source code for feature crate `cli-commands` along with its corresponding and imported definitions from the `shared` crate.

## Problem Statement

The following issues were detected by `lint-arwaky-cli scan`:

```
============================================================
  AES Architecture Compliance Report
============================================================
  Project:
  Violations: 4
  [AES201] /home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_config_command.rs - AES201 FORBIDDEN_IMPORT: Layer 'surface' is importing from forbidden layer 'contract(protocol)'.
WHY? Layer 'surface' must not depend on 'contract(protocol)' to maintain architectural boundaries.
FIX: Remove the import or refactor to use one of the allowed layers: [taxonomy, contract, utility]
  [AES201] /home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_config_command.rs - AES201 FORBIDDEN_IMPORT: Layer 'surface' is importing from forbidden layer 'contract(protocol)'.
WHY? Layer 'surface' must not depend on 'contract(protocol)' to maintain architectural boundaries.
FIX: Remove the import or refactor to use one of the allowed layers: [taxonomy, contract, utility]
  [AES201] /home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_check_action.rs - AES201 FORBIDDEN_IMPORT: Layer 'surface' is importing from forbidden layer 'surface(command|controller|page|entry)'.
WHY? Layer 'surface' must not depend on 'surface(command|controller|page|entry)' to maintain architectural boundaries.
FIX: Remove the import or refactor to use one of the allowed layers: [taxonomy]
  [AES201] /home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_check_action.rs - AES201 FORBIDDEN_IMPORT: Layer 'surface' is importing from forbidden layer 'surface(command|controller|page|entry)'.
WHY? Layer 'surface' must not depend on 'surface(command|controller|page|entry)' to maintain architectural boundaries.
FIX: Remove the import or refactor to use one of the allowed layers: [taxonomy]
```

---

## File List

- [ARCHITECTURE.md](file:///home/raka/mcp-arwaky/lint-arwaky/ARCHITECTURE.md)
- [crates/cli-commands/Cargo.toml](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/Cargo.toml)
- [crates/cli-commands/FRD.md](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/FRD.md)
- [crates/cli-commands/src/agent_analysis_pipeline_orchestrator.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/agent_analysis_pipeline_orchestrator.rs)
- [crates/cli-commands/src/lib.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/lib.rs)
- [crates/cli-commands/src/root_cli_container.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/root_cli_container.rs)
- [crates/cli-commands/src/surface_check_action.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_check_action.rs)
- [crates/cli-commands/src/surface_check_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_check_command.rs)
- [crates/cli-commands/src/surface_common_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_common_command.rs)
- [crates/cli-commands/src/surface_config_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_config_command.rs)
- [crates/cli-commands/src/surface_fix_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_fix_command.rs)
- [crates/cli-commands/src/surface_git_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_git_command.rs)
- [crates/cli-commands/src/surface_maintenance_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_maintenance_command.rs)
- [crates/cli-commands/src/surface_plugin_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_plugin_command.rs)
- [crates/cli-commands/src/surface_setup_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_setup_command.rs)
- [crates/cli-commands/src/surface_watch_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/surface_watch_command.rs)
- [crates/cli-commands/src/utility_format_output.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/utility_format_output.rs)
- [crates/shared/src/auto-fix/contract_fix_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/contract_fix_aggregate.rs)
- [crates/shared/src/auto-fix/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/mod.rs)
- [crates/shared/src/cli-commands/contract_analysis_pipeline_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/contract_analysis_pipeline_aggregate.rs)
- [crates/shared/src/cli-commands/contract_report_formatter_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/contract_report_formatter_aggregate.rs)
- [crates/shared/src/cli-commands/contract_report_formatter_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/contract_report_formatter_protocol.rs)
- [crates/shared/src/cli-commands/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/mod.rs)
- [crates/shared/src/cli-commands/taxonomy_catalog_constant.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_catalog_constant.rs)
- [crates/shared/src/cli-commands/taxonomy_cli_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_cli_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_command_catalog_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_command_catalog_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_format_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_format_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_metadata_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_metadata_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_position_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_position_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_protocol_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_protocol_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_scan_report_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_scan_report_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_scan_request_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_scan_request_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_score_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_score_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_severity_vo.rs)
- [crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs)
- [crates/shared/src/code-analysis/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/mod.rs)
- [crates/shared/src/common/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/mod.rs)
- [crates/shared/src/common/taxonomy_git_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_git_vo.rs)
- [crates/shared/src/common/taxonomy_path_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_path_vo.rs)
- [crates/shared/src/common/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_severity_vo.rs)
- [crates/shared/src/common/taxonomy_threshold_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_threshold_vo.rs)
- [crates/shared/src/common/utility_file.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/utility_file.rs)
- [crates/shared/src/common/utility_language_detector.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/utility_language_detector.rs)
- [crates/shared/src/config-system/contract_config_orchestrator_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_config_orchestrator_aggregate.rs)
- [crates/shared/src/config-system/contract_reader_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_reader_protocol.rs)
- [crates/shared/src/config-system/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/mod.rs)
- [crates/shared/src/config-system/taxonomy_config_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_config_vo.rs)
- [crates/shared/src/external-lint/contract_external_lint_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/contract_external_lint_aggregate.rs)
- [crates/shared/src/external-lint/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/mod.rs)
- [crates/shared/src/file-watch/contract_watch_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/contract_watch_aggregate.rs)
- [crates/shared/src/file-watch/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/mod.rs)
- [crates/shared/src/file-watch/taxonomy_watch_config_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/file-watch/taxonomy_watch_config_vo.rs)
- [crates/shared/src/git-hooks/contract_git_hooks_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/contract_git_hooks_aggregate.rs)
- [crates/shared/src/git-hooks/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/git-hooks/mod.rs)
- [crates/shared/src/import-rules/contract_import_runner_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_import_runner_aggregate.rs)
- [crates/shared/src/import-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/mod.rs)
- [crates/shared/src/naming-rules/contract_naming_runner_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/contract_naming_runner_aggregate.rs)
- [crates/shared/src/naming-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/mod.rs)
- [crates/shared/src/orphan-detector/contract_orphan_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/contract_orphan_aggregate.rs)
- [crates/shared/src/orphan-detector/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/mod.rs)
- [crates/shared/src/project-setup/contract_maintenance_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/contract_maintenance_aggregate.rs)
- [crates/shared/src/project-setup/contract_setup_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/contract_setup_aggregate.rs)
- [crates/shared/src/project-setup/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/project-setup/mod.rs)
- [crates/shared/src/role-rules/contract_role_runner_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_role_runner_aggregate.rs)
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
| Smart surfaces   | command, controller, page, router | Taxonomy, Contract Aggregate, Utility | May initiate feature behavior through aggregate |
| Utility surfaces | hook, store, action, screen       | Taxonomy only                         | Support smart surfaces but must not import them |
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

## File: crates/cli-commands/Cargo.toml

```toml
[package]
name = "cli_commands-lint-arwaky"
version = "1.10.106"
edition = "2021"
description = "CLI command surfaces (`check`, `scan`, `fix`, `git`, `config`, `setup`, `tui`, `watch`) composing the agent orchestrators into the user-facing CLI."
license = "MIT"
repository = "https://github.com/rakaarwaky/lint-arwaky"
publish = true

[lints]
workspace = true

[dependencies]  # (unchanged)
serde.workspace = true
serde_json.workspace = true
async-trait.workspace = true
clap.workspace = true
console.workspace = true
dialoguer.workspace = true
futures.workspace = true
anyhow.workspace = true
serde_yaml_ng.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
ctrlc.workspace = true
tokio.workspace = true
shared.workspace = true
dirs.workspace = true
import_rules.workspace = true
naming_rules.workspace = true
role_rules.workspace = true
code_analysis.workspace = true
external_lint.workspace = true
orphan_detector.workspace = true
auto_fix.workspace = true
config_system.workspace = true
git_hooks.workspace = true
file_watch.workspace = true
project_setup.workspace = true
maintenance.workspace = true
report_formatter.workspace = true
regex.workspace = true
```

---

## File: crates/cli-commands/FRD.md

```rust
# FRD — cli-commands

## Feature Goal

The cli-commands crate provides a unified command-line interface (CLI) that drives the entire lint-arwaky linting pipeline. It implements thin surface handlers that delegate business logic to agent/orchestrator layers (IAnalysisPipelineAggregate, MaintenanceCommandsAggregate, etc.). Report formatting is delegated to the `report-formatter` crate via `IReportFormatterAggregate`.

## Commands & Scope

### Analysis Commands

- **check** — Run full architecture compliance analysis on a target path. Runs all 6 linter groups in sequence: code analysis (AES301-305), naming rules (AES101-102), import rules (AES201-205), external adapters (Clippy, Ruff, ESLint), role rules (AES401-406), orphan detection (AES501-506). Supports `--git-diff` for staged-only scanning.
- **scan** — Multi-workspace discovery scan that auto-detects Cargo.toml/pyproject.toml/package.json members, creates per-project DI containers, and runs the full analysis pipeline on each. Supports `--member <name>` to target a specific workspace member.
- **ci** — CI-optimized analysis with configurable threshold and exit codes. Auto-fails on CRITICAL violations regardless of score. Compares score against threshold as float comparison (not truncated integer).

### Fix Commands

- **fix** — Apply automatic fixes to files that violate rules. Supports `--dry-run` for preview mode. Only auto-fixes safe, non-destructive rule violations.

### Maintenance Commands

- **doctor** — Toolchain diagnostics: checks availability and version of cargo, python3, node, git, and other required tools. Returns exit code 0 regardless of findings (diagnostic only).
- **security** — Vulnerability scanning via cargo-audit (Rust) or bandit (Python). Returns exit code 3 when the scanning tool is missing, exit code 1 when vulnerabilities are found, exit code 0 when clean.
- **dependencies** — Dependency report from Cargo.lock / pyproject.toml / package.json. Lists all dependencies with version and type.

### Project Setup Commands

- **init** — Create default lint-arwaky configuration file in the current project directory (XDG-compliant).
- **install** — Install adapter dependencies (Clippy, Ruff, ESLint, etc.) for the detected language. Supports `--sudo` flag.
- **install-hook** — Install git pre-commit hook that runs lint-arwaky on staged files.
- **uninstall-hook** — Remove the installed git pre-commit hook.
- **mcp-config --client <name>** — Print MCP server configuration for the specified client (e.g., `claude`, `cursor`).
- **config-show** — Display active configuration files and their contents. Sensitive values (AWS keys, long base64 strings) are redacted before display.

### Utility Commands

- **adapters** — List enabled external lint adapters (Clippy, Ruff, ESLint, etc.) discovered by the external-lint layer.
- **watch** — Monitor file changes and trigger re-scans on modified files.
- **orphan <path>** — Check if a specific file is dead/unreachable code by analyzing the workspace import graph.
- **version** — Show version and build information (Git commit hash, Rustc version).

## Architecture

### Layer Delegation Pattern

All surface handlers follow strict AES406 rules:

- **Surfaces** (`surface_*.rs`) — Thin dispatch layer. Parse args, call aggregates, format output. Zero business logic.
- **Agents/Orchestrators** (`agent_*.rs`) — Orchestrate multi-linter pipelines. Depend on contracts only. No I/O, no formatting.
- **Capabilities** (`capabilities_*.rs`) — Single-responsibility implementations (report formatters, etc.). Implement contract protocols.

### Analysis Pipeline

The core analysis pipeline is defined by `IAnalysisPipelineAggregate` trait implemented by `AnalysisPipelineOrchestrator`:

1. Collect source files (ignore-aware via `collect_all_source_files`)
2. Run code analysis (AES301-305)
3. Run naming, import, external, and role audits concurrently
4. Run orphan detection across workspace
5. Merge results, apply path filtering, format output

### Formatters

Report formatting is delegated to the `report-formatter` crate via `IReportFormatterAggregate`. The surface layer never formats output directly — it calls `self.report_formatter.format(&report, format)`. Supported formats:

- **Text** — Human-readable formatted output with severity badges
- **JSON** — Machine-readable structured output for CI/CD integration
- **SARIF 2.1.0** — Standard static analysis results format (VS Code, GitHub Code Scanning)
- **JUnit XML** — Test report format for CI/CD pipelines

## Exit Codes

| Code | Meaning                                           |
| ---- | ------------------------------------------------- |
| 0    | Success — no violations found                     |
| 1    | Violations/findings detected                      |
| 2    | System/operational error                          |
| 3    | Required tool missing (e.g., cargo-audit, bandit) |

## Non-Functional Requirements

- **Cross-platform** — File walker uses canonical paths (not inodes), works on all platforms including Windows.
- **SARIF support** — Full SARIF 2.1.0 output for IDE integration and GitHub Code Scanning.
- **Performance** — Ignore-aware scanning excludes `target/`, `node_modules/`, `.git/`, `dist/`, `build/`, `coverage/`, `.venv/`. Symlink targets outside workspace root are pruned.
- **Concurrency** — Async linter groups run concurrently via `tokio::join!`. Deferred container construction for lightweight commands (version, adapters).
- **Multi-workspace** — Scan auto-discovers workspace members and runs per-project analysis with isolated DI containers.

## Configuration Resolution Algorithm

When loading a configuration file for a given project path, lint-arwaky searches in this priority order:

1. **Project-root YAML** — Search from the project root upward (max 3 levels) for language-specific config files:
   - Rust: `lint_arwaky.config.rust.yaml` or `lint_arwaky.config.yaml`
   - Python: `lint_arwaky.config.python.yaml` or `lint_arwaky.config.yaml`
   - TypeScript: `lint_arwaky.config.javascript.yaml` or `lint_arwaky.config.yaml`

2. **Parent directory traversal** — Walk up parent directories (depth ≤ 3) looking for config files, allowing shared configs at the workspace root to apply to all members.

3. **XDG user config** — Check `<XDG_CONFIG_HOME>/lint-arwaky/<config-file>` (default: `~/.config/lint-arwaky/`).

4. **System XDG dirs** — Check each entry in `$XDG_CONFIG_DIRS` (or `/etc/xdg/lint-arwaky/` by default) for `<config-file>`. Limited to 8 directories, only absolute paths accepted.

5. **Embedded defaults** — If no config file is found at any level, use compiled-in defaults appropriate for the detected language (based on `Cargo.toml`, `pyproject.toml`, or `package.json` presence).

When multiple config files are found across levels, the deepest match wins (most specific path takes priority). The loaded config is cached by file path to avoid re-parsing.

## Dependencies

- `report-formatter` — Report formatting capabilities (text, JSON, SARIF, JUnit)
- `shared` — Taxonomy, contracts, and utility types
- `code-analysis`, `naming-rules`, `import-rules`, `role-rules`, `orphan-detector`, `external-lint` — Linter subsystems

## Success Indicators

- [ ] AES compliance — the crate passes self-lint (`cargo run --bin lint-arwaky-cli -- check`)
- [ ] Surface thinness — surface handlers contain no business logic, only dispatch
- [ ] Formatters delegated — surface uses `IReportFormatterAggregate`, no inline formatting
- [ ] Exit code correctness — all commands follow the standardized exit code convention
- [ ] Secret redaction — config-show never leaks tokens or API keys
```

---

## File: crates/cli-commands/src/agent_analysis_pipeline_orchestrator.rs

```rust
// PURPOSE: AnalysisPipelineOrchestrator — implements IAnalysisPipelineAggregate
//
// This is the agent layer orchestrator that wires together all 6 linter groups
// and produces a unified ScanReport. It depends only on contracts (traits),
// never on concrete implementations.
use crate::utility_format_output::{format_junit_output, format_sarif_output};
use shared::cli_commands::contract_analysis_pipeline_aggregate::IAnalysisPipelineAggregate;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_scan_report_vo::{
    DiagnosticSeverity, PipelineDiagnostic, PipelineError, ScanReport,
};
use shared::cli_commands::taxonomy_scan_request_vo::ScanRequest;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use std::sync::Arc;

/// AnalysisPipelineOrchestrator — agent layer that coordinates the full lint pipeline.
///
/// Implements IAnalysisPipelineAggregate by running all 6 linter groups in sequence:
///   1. Code analysis (AES301-305)
///   2. Naming rules (AES101-102)
///   3. Import rules (AES201-205)
///   4. External linters (Clippy, Ruff, ESLint, etc.)
///   5. Role rules (AES401-406)
///   6. Orphan detection (AES501-506)
// ─── Block 1: Struct Definition ───────────────────────────
pub struct CheckArgs {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub format: Format,
}

pub struct AnalysisPipelineOrchestrator {
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    external_lint: Arc<dyn IExternalLintAggregate>,
    role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    format: Format,
    filter: Option<String>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
#[async_trait::async_trait]
impl IAnalysisPipelineAggregate for AnalysisPipelineOrchestrator {
    async fn run(&self, request: ScanRequest) -> Result<ScanReport, PipelineError> {
        self.run_pipeline(request).await
    }

    async fn run_with_discovery(&self) -> Result<ScanReport, PipelineError> {
        self.run_pipeline_with_discovery().await
    }

    fn check_orphan_single_file(
        &self,
        file_path: &str,
        workspace_root: &str,
    ) -> Result<Vec<LintResult>, PipelineError> {
        self.check_orphan_single_file_impl(file_path, workspace_root)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl AnalysisPipelineOrchestrator {
    pub fn new(args: CheckArgs) -> Self {
        Self {
            code_analysis_linter: args.code_analysis_linter,
            naming_orchestrator: args.naming_orchestrator,
            import_orchestrator: args.import_orchestrator,
            external_lint: args.external_lint,
            role_orchestrator: args.role_orchestrator,
            orphan_orchestrator: args.orphan_orchestrator,
            config_orchestrator: args.config_orchestrator,
            format: args.format,
            filter: None,
        }
    }

    /// Run the full analysis pipeline on a target path.
    ///
    /// This is the core scan pipeline. It runs all 6 linter groups in the
    /// same order every time and collects results into a ScanReport.
    pub async fn run_pipeline(&self, request: ScanRequest) -> Result<ScanReport, PipelineError> {
        let target = &request.target.value;
        let path_obj = FilePath::new(target.to_string()).map_err(PipelineError::InvalidPath)?;

        let mut all_results = Vec::new();
        let mut diagnostics = Vec::new();

        // 1. Run AES analysis (AES301-305) — file lines, bypass, mandatory defs
        let aes_results = self.code_analysis_linter.run_code_analysis(&path_obj);
        let aes_count = aes_results.len();
        all_results.extend(aes_results.values);
        diagnostics.push(PipelineDiagnostic::new(
            "code-analysis".to_string(),
            format!("AES analysis complete: {aes_count} violations"),
            DiagnosticSeverity::Info,
        ));

        // 2-5. Run async linter groups concurrently (tokio::join! works in existing async context)
        let (naming_results, import_results, external_results, role_results) = tokio::join!(
            self.naming_orchestrator.run_audit(&path_obj),
            self.import_orchestrator.run_audit(&path_obj),
            self.external_lint.scan_all(&path_obj),
            self.role_orchestrator.run_audit(&path_obj),
        );

        // Report audit failures instead of silently discarding them
        match naming_results {
            Ok(values) => {
                let naming_count = values.len();
                all_results.extend(values);
                diagnostics.push(PipelineDiagnostic::new(
                    "naming".to_string(),
                    format!("Naming audit complete: {naming_count} violations"),
                    DiagnosticSeverity::Info,
                ));
            }
            Err(e) => {
                eprintln!("[warn] naming audit failed: {e}");
                diagnostics.push(PipelineDiagnostic::new(
                    "naming".to_string(),
                    format!("Naming audit failed: {e}"),
                    DiagnosticSeverity::Warning,
                ));
            }
        }

        match import_results {
            Ok(values) => {
                let import_count = values.len();
                all_results.extend(values);
                diagnostics.push(PipelineDiagnostic::new(
                    "imports".to_string(),
                    format!("Import audit complete: {import_count} violations"),
                    DiagnosticSeverity::Info,
                ));
            }
            Err(e) => {
                eprintln!("[warn] import audit failed: {e}");
                diagnostics.push(PipelineDiagnostic::new(
                    "imports".to_string(),
                    format!("Import audit failed: {e}"),
                    DiagnosticSeverity::Warning,
                ));
            }
        }

        let external_count = external_results.len();
        all_results.extend(external_results.values);
        let role_count = role_results.len();
        all_results.extend(role_results);
        diagnostics.push(PipelineDiagnostic::new(
            "external".to_string(),
            format!("External lint complete: {external_count} violations"),
            DiagnosticSeverity::Info,
        ));
        diagnostics.push(PipelineDiagnostic::new(
            "roles".to_string(),
            format!("Role audit complete: {role_count} violations"),
            DiagnosticSeverity::Info,
        ));

        // 6. Run orphan detection (AES501-506) — dead code via import graph
        let orphan_results = self.run_orphan_detection(target).await;
        let orphan_count = orphan_results.len();
        all_results.extend(orphan_results);
        diagnostics.push(PipelineDiagnostic::new(
            "orphan".to_string(),
            format!("Orphan detection complete: {orphan_count} violations"),
            DiagnosticSeverity::Info,
        ));

        Ok(ScanReport::new(all_results, diagnostics))
    }

    /// Run orphan detection pass — scans workspace for cross-folder import graph.
    async fn run_orphan_detection(&self, path: &str) -> Vec<LintResult> {
        let scan_root = crate::surface_check_action::find_workspace_root(path);
        let orphan_scan_root = scan_root.as_ref().and_then(|r| r.to_str()).unwrap_or(".");
        let dir_path = DirectoryPath::new(orphan_scan_root.to_string()).unwrap_or_default();
        let ignored = self.config_orchestrator.ignored_paths(orphan_scan_root);
        let source_files = match shared::common::utility_file::scan_directory(&dir_path, &ignored) {
            Ok(list) => list.values,
            Err(_) => Vec::new(),
        };
        let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();
        self.orphan_orchestrator
            .check_orphans(&file_strs, orphan_scan_root)
    }

    /// Filter results to the target path and return formatted output string.
    pub fn format_results(&self, results: Vec<LintResult>, path: &str) -> String {
        let canonical_scan_path = std::path::PathBuf::from(path);
        let canonical_scan_path = canonical_scan_path
            .canonicalize()
            .unwrap_or(canonical_scan_path);
        let cwd = crate::surface_common_command::current_dir();

        // Filter results to the target path (P2.3: use Path::starts_with)
        let filtered_results: Vec<_> = results
            .into_iter()
            .filter(|r| {
                let abs_path = cwd.join(&r.file.value);
                abs_path.starts_with(&canonical_scan_path)
            })
            .collect();

        match self.format {
            Format::Text => {
                let results_list =
                    shared::cli_commands::taxonomy_result_vo::LintResultList::new(filtered_results);
                let report_path = FilePath::new(path.to_string()).unwrap_or_default();
                self.code_analysis_linter
                    .format_report(&results_list, &report_path)
            }
            Format::Json => {
                serde_json::to_string_pretty(&filtered_results).unwrap_or_else(|_| "[]".to_string())
            }
            Format::Sarif => format_sarif_output(&filtered_results),
            Format::Junit => format_junit_output(&filtered_results),
        }
    }

    /// Run the full analysis pipeline with multi-workspace discovery.
    ///
    /// Discovers workspace members (Cargo.toml, pyproject.toml, package.json workspaces),
    /// runs all 6 linter groups per member, runs cross-workspace orphan detection,
    /// filters results to each member's path, and aggregates into a single ScanReport.
    pub async fn run_pipeline_with_discovery(&self) -> Result<ScanReport, PipelineError> {
        // Discover workspaces
        let workspaces = self
            .config_orchestrator
            .discover_workspaces(
                &FilePath::new(".".to_string())
                    .map_err(|e| PipelineError::InvalidPath(e.to_string()))?,
            )
            .await;

        if workspaces.is_empty() {
            // No workspaces discovered — fall back to single-scan mode
            let request = ScanRequest {
                target: shared::cli_commands::taxonomy_scan_request_vo::ScanTarget::new(
                    ".".to_string(),
                ),
                mode: shared::cli_commands::taxonomy_scan_request_vo::ScanMode::default(),
                filter: self.filter.clone(),
                member: None,
                format: self.format,
            };
            return self.run(request).await;
        }

        let _multi = workspaces.len() > 1;
        let mut global_results = Vec::new();
        let global_diagnostics = Vec::new();

        // Collect ALL source files from workspace root for cross-workspace orphan detection
        let scan_root = crate::surface_check_action::find_workspace_root(".")
            .unwrap_or(std::path::PathBuf::from("."));
        let ignored = self
            .config_orchestrator
            .ignored_paths(scan_root.to_str().unwrap_or("."));
        let dir_path = DirectoryPath::new(scan_root.to_str().unwrap_or(".")).unwrap_or_default();
        let all_source_files: Vec<String> = {
            match shared::common::utility_file::scan_directory(&dir_path, &ignored) {
                Ok(list) => list.values.iter().map(|f| f.value.clone()).collect(),
                Err(_) => Vec::new(),
            }
        };

        // Run orphan detection once across all workspace members
        let orphan_results_all = self
            .orphan_orchestrator
            .check_orphans(&all_source_files, scan_root.to_str().unwrap_or("."));

        for ws in &workspaces {
            let mut all_results = Vec::new();

            // 1. Run AES analysis
            let aes_results = self.code_analysis_linter.run_code_analysis(&ws.path);
            all_results.extend(aes_results.values);

            // 2-5. Run async linter groups concurrently (tokio::join! works in existing async context)
            let (naming_results, import_results, external_results, role_results) = tokio::join!(
                self.naming_orchestrator.run_audit(&ws.path),
                self.import_orchestrator.run_audit(&ws.path),
                self.external_lint.scan_all(&ws.path),
                self.role_orchestrator.run_audit(&ws.path),
            );

            match naming_results {
                Ok(values) => all_results.extend(values),
                Err(e) => eprintln!("[warn] naming audit failed: {e}"),
            }
            match import_results {
                Ok(values) => all_results.extend(values),
                Err(e) => eprintln!("[warn] import audit failed: {e}"),
            }
            all_results.extend(external_results.values);
            all_results.extend(role_results);

            // Filter results to this workspace member's path
            let ws_canonical = std::path::Path::new(&ws.path.value).canonicalize().ok();
            let cwd_for_ws = match std::env::current_dir() {
                Ok(d) => d,
                Err(_) => std::path::PathBuf::new(),
            };
            let ws_fallback = {
                let raw = std::path::Path::new(&ws.path.value);
                if raw.is_absolute() {
                    raw.to_path_buf()
                } else {
                    cwd_for_ws.join(raw)
                }
            };
            let ws_fallback = std::fs::canonicalize(&ws_fallback).unwrap_or(ws_fallback);

            let filtered_results: Vec<_> = match &self.filter {
                Some(code) => all_results
                    .into_iter()
                    .filter(|r| {
                        let abs_path = cwd_for_ws.join(&r.file.value);
                        r.code.code() == code
                            && (ws_canonical
                                .as_ref()
                                .map(|c| abs_path.starts_with(c))
                                .unwrap_or(false)
                                || abs_path.starts_with(&ws_fallback))
                    })
                    .collect(),
                None => all_results
                    .into_iter()
                    .filter(|r| {
                        let abs_path = cwd_for_ws.join(&r.file.value);
                        ws_canonical
                            .as_ref()
                            .map(|c| abs_path.starts_with(c))
                            .unwrap_or(abs_path.starts_with(&ws_fallback))
                    })
                    .collect(),
            };

            // Filter orphan results to this workspace member's path
            let filtered_orphans: Vec<_> = match &self.filter {
                Some(code) => orphan_results_all
                    .iter()
                    .filter(|r| {
                        let abs_path = cwd_for_ws.join(&r.file.value);
                        r.code.code() == code
                            && (ws_canonical
                                .as_ref()
                                .map(|c| abs_path.starts_with(c))
                                .unwrap_or(false)
                                || abs_path.starts_with(&ws_fallback))
                    })
                    .cloned()
                    .collect(),
                None => orphan_results_all
                    .iter()
                    .filter(|r| {
                        let abs_path = cwd_for_ws.join(&r.file.value);
                        ws_canonical
                            .as_ref()
                            .map(|c| abs_path.starts_with(c))
                            .unwrap_or(abs_path.starts_with(&ws_fallback))
                    })
                    .cloned()
                    .collect(),
            };

            // Merge per-member results with filtered orphans
            let mut member_results = filtered_results;
            member_results.extend(filtered_orphans);
            global_results.extend(member_results);
        }

        Ok(ScanReport::new(global_results, global_diagnostics))
    }

    /// Check if a single file is an orphan.
    ///
    /// Scans ALL source files to build the import graph for reachability analysis,
    /// then filters results to only the specified file path.
    pub fn check_orphan_single_file_impl(
        &self,
        file_path: &str,
        _workspace_root: &str,
    ) -> Result<Vec<LintResult>, PipelineError> {
        let path_obj = std::path::Path::new(file_path);

        // Find workspace root for cross-crate graph building
        let scan_root = match crate::surface_check_action::find_workspace_root(file_path) {
            Some(r) => r,
            None => std::path::PathBuf::from("."),
        };
        let ignored = self
            .config_orchestrator
            .ignored_paths(scan_root.to_str().unwrap_or("."));
        let all_files: Vec<String> = shared::common::collect_all_source_files(&scan_root, &ignored)
            .iter()
            .map(|f| f.value.clone())
            .collect();

        // Normalize the target file path
        let target_path = if path_obj.is_absolute() {
            file_path.to_string()
        } else {
            let cwd = crate::surface_common_command::current_dir();
            cwd.join(file_path).to_string_lossy().to_string()
        };

        // Run orphan detection with workspace root
        let all_results = self
            .orphan_orchestrator
            .check_orphans(&all_files, &scan_root.to_string_lossy());

        // Filter results for the specific file — canonicalize for robust comparison
        let target_canonical = std::path::Path::new(&target_path).canonicalize().ok();
        let file_results: Vec<_> = all_results
            .into_iter()
            .filter(|r| {
                let r_canonical = std::path::Path::new(&r.file.value).canonicalize().ok();
                match (target_canonical.as_deref(), r_canonical.as_deref()) {
                    (Some(t), Some(r)) => t == r,
                    _ => r.file.value == target_path || r.file.value == file_path,
                }
            })
            .collect();

        Ok(file_results)
    }
}
```

---

## File: crates/cli-commands/src/lib.rs

```rust
// PURPOSE: Module declarations for cli-commands (surfaces, transport, container)
pub use shared::cli_commands::taxonomy_catalog_constant::COMMAND_CATALOG;
pub use shared::cli_commands::taxonomy_cli_vo::{get_cli, Cli, Commands};
pub use shared::cli_commands::taxonomy_command_catalog_vo::{command_catalog, CommandCatalogVO};
pub use shared::cli_commands::taxonomy_metadata_vo::CommandMetadataVO;
pub mod agent_analysis_pipeline_orchestrator;
pub use agent_analysis_pipeline_orchestrator::AnalysisPipelineOrchestrator;
pub mod utility_format_output;
// Re-export report-formatter capabilities for backward compatibility
pub use report_formatter::agent_report_formatter_orchestrator;
pub use report_formatter::capabilities_json_formatter;
pub use report_formatter::capabilities_junit_formatter;
pub use report_formatter::capabilities_sarif_formatter;
pub use report_formatter::capabilities_text_formatter;
pub use report_formatter::ReportFormatterOrchestrator;
pub mod surface_check_command;
pub use surface_check_command::CheckCommandsSurface;
pub mod surface_check_action;
pub mod surface_common_command;
pub mod surface_fix_command;
pub use surface_fix_command::FixCommandsSurface;
pub mod surface_git_command;
pub mod surface_maintenance_command;
pub mod surface_plugin_command;
pub mod surface_setup_command;
pub mod surface_watch_command;
pub use surface_watch_command::WatchCommandsSurface;
pub mod root_cli_container;
pub mod surface_config_command;
pub use root_cli_container::CliContainer;
```

---

## File: crates/cli-commands/src/root_cli_container.rs

```rust
// PURPOSE: CliContainer — DI wiring for CLI binary aggregates
use std::sync::Arc;

use shared::cli_commands::contract_analysis_pipeline_aggregate::IAnalysisPipelineAggregate;
use shared::cli_commands::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

pub struct CliContainer {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub git_aggregate: Arc<dyn GitHooksAggregate>,
    pub multi_project_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub report_formatter: Arc<dyn IReportFormatterAggregate>,
    pub analysis_pipeline: Arc<dyn IAnalysisPipelineAggregate>,
}

impl CliContainer {
    pub fn new_default() -> Self {
        // Create config orchestrator — single source of truth for config
        let config_container = config_system::root_config_system_container::ConfigContainer::new();
        let multi_project_orchestrator = config_container.orchestrator();

        // All containers get config from orchestrator
        let code_analysis_linter =
            code_analysis::root_code_analysis_container::CodeAnalysisContainer::from_orchestrator(
                &multi_project_orchestrator,
                ".",
            )
            .code_analysis_linter();

        let import_container =
            import_rules::root_import_rules_container::ImportContainer::from_orchestrator(
                &multi_project_orchestrator,
                ".",
            );
        let import_orchestrator = import_container.orchestrator();

        let role_container =
            role_rules::root_role_rules_container::RoleContainer::from_orchestrator(
                &multi_project_orchestrator,
                ".",
            );
        let role_orchestrator = role_container.orchestrator();

        let naming_container =
            naming_rules::root_naming_rules_container::NamingContainer::from_orchestrator(
                &multi_project_orchestrator,
                ".",
            );
        let naming_orchestrator = naming_container.orchestrator();

        let external_lint_container =
            external_lint::root_external_lint_container::ExternalLintContainer::new_default();
        let external_lint = external_lint_container.aggregate();

        let orphan_container =
            orphan_detector::root_orphan_detector_container::OrphanContainer::from_orchestrator(
                &multi_project_orchestrator,
                ".",
            );
        let orphan_orchestrator = orphan_container.analyzer();

        let git_container = git_hooks::root_git_hooks_container::GitContainer::new_default();
        let git_aggregate = git_container.aggregate();

        // Wire up report formatter capabilities → aggregate
        let text_formatter: Arc<dyn IReportFormatterProtocol> = Arc::new(
            report_formatter::TextFormatter::new(code_analysis_linter.clone()),
        );
        let json_formatter: Arc<dyn IReportFormatterProtocol> =
            Arc::new(report_formatter::JsonFormatter::new());
        let sarif_formatter: Arc<dyn IReportFormatterProtocol> =
            Arc::new(report_formatter::SarifFormatter::new());
        let junit_formatter: Arc<dyn IReportFormatterProtocol> =
            Arc::new(report_formatter::JunitFormatter::new());
        let report_formatter_agg: Arc<dyn IReportFormatterAggregate> =
            Arc::new(report_formatter::ReportFormatterOrchestrator::new(
                text_formatter,
                json_formatter,
                sarif_formatter,
                junit_formatter,
            ));

        // Wire analysis pipeline orchestrator
        let analysis_pipeline: Arc<dyn IAnalysisPipelineAggregate> =
            Arc::new(crate::AnalysisPipelineOrchestrator::new(
                crate::agent_analysis_pipeline_orchestrator::CheckArgs {
                    code_analysis_linter: code_analysis_linter.clone(),
                    naming_orchestrator: naming_orchestrator.clone(),
                    import_orchestrator: import_orchestrator.clone(),
                    external_lint: external_lint.clone(),
                    role_orchestrator: role_orchestrator.clone(),
                    orphan_orchestrator: orphan_orchestrator.clone(),
                    config_orchestrator: multi_project_orchestrator.clone(),
                    format: Format::Text,
                },
            ));

        Self {
            code_analysis_linter,
            import_orchestrator,
            naming_orchestrator,
            role_orchestrator,
            external_lint,
            orphan_orchestrator,
            git_aggregate,
            multi_project_orchestrator,
            report_formatter: report_formatter_agg,
            analysis_pipeline,
        }
    }

    pub fn pipeline_aggregate(&self) -> Arc<dyn IAnalysisPipelineAggregate> {
        self.analysis_pipeline.clone()
    }

    pub fn fix_orchestrator_factory(
        &self,
    ) -> std::sync::Arc<
        dyn Fn(
                bool,
            ) -> std::sync::Arc<
                dyn shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate,
            > + Send
            + Sync,
    > {
        let fix_linter = self.code_analysis_linter.clone();
        Arc::new(move |dry_run| {
            auto_fix::root_auto_fix_container::AutoFixContainer::new(fix_linter.clone())
                .orchestrator(dry_run)
        })
    }
}
```

---

## File: crates/cli-commands/src/surface_check_action.rs

```rust
// PURPOSE: Check/scan/CI entry points — thin wrappers around CheckCommandsSurface
//
// Three commands, distinguished by scope:
//   - check:  self-lint the lint-arwaky project itself (uses CheckCommandsSurface.scan)
//   - scan:   full analysis on external project + external adapters (uses scan_with_discovery)
//   - ci:     CI-mode with threshold comparison and critical-violation auto-fail
//
// find_workspace_root walks up from the given path looking for Cargo.toml/crates/packages/modules.
use std::collections::BTreeMap;
use std::process::ExitCode;
use std::sync::Arc;

use crate::surface_check_command::CheckCommandsSurface;
use shared::cli_commands::contract_analysis_pipeline_aggregate::IAnalysisPipelineAggregate;
use shared::cli_commands::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_threshold_vo::Threshold;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;

/// Walk up from `path` to find the workspace root (parent of `crates/`, `packages/`, or `modules/`).
pub fn find_workspace_root(path: &str) -> Option<std::path::PathBuf> {
    shared::common::find_workspace_root(path)
}

pub struct CheckOptions {
    pub path: Option<FilePath>,
    pub git_diff: bool,
    pub pipeline: Arc<dyn IAnalysisPipelineAggregate>,
    pub report_formatter: Arc<dyn IReportFormatterAggregate>,
    pub filter: Option<String>,
    pub git_aggregate: Option<Arc<dyn GitHooksAggregate>>,
    pub config: ArchitectureConfig,
    pub format: Format,
}

/// check = self-lint (AES analysis on current project, same algorithm as scan)
pub fn handle_check(opts: CheckOptions) -> ExitCode {
    let root = match &opts.path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !std::path::Path::new(&root).exists() {
        eprintln!("Error: path '{}' does not exist", root);
        return ExitCode::from(2);
    }
    let surface = CheckCommandsSurface::new(opts.pipeline, opts.report_formatter, None);
    surface.scan(&root, opts.filter.as_deref(), opts.format)
}

pub struct ScanOptions {
    pub path: Option<FilePath>,
    pub pipeline: Arc<dyn IAnalysisPipelineAggregate>,
    pub report_formatter: Arc<dyn IReportFormatterAggregate>,
    pub multi_project_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    pub filter: Option<String>,
    pub member: Option<String>,
    pub format: Format,
}

/// scan = AES analysis on external project + external adapters
pub fn handle_scan(opts: ScanOptions) -> ExitCode {
    let root = match &opts.path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !std::path::Path::new(&root).exists() {
        eprintln!("Error: path '{}' does not exist", root);
        return ExitCode::from(2);
    }
    let surface = CheckCommandsSurface::new(
        opts.pipeline,
        opts.report_formatter,
        opts.multi_project_orchestrator,
    );
    surface.scan_with_discovery(
        &root,
        opts.filter.as_deref(),
        opts.member.as_deref(),
        opts.format,
    )
}

pub fn handle_ci(
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    path: Option<FilePath>,
    threshold: Threshold,
) -> ExitCode {
    crate::surface_common_command::run_ci_analysis(code_analysis_linter, path, threshold)
}

/// Default check = self-lint when no args provided (runs `lint_path(".")`)
pub fn handle_default_check(
    project_root: &str,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
) -> ExitCode {
    let path = FilePath::new(project_root.to_string()).unwrap_or_default();
    let results = code_analysis_linter.run_code_analysis_path(&path);
    let mut lines: Vec<String> = Vec::new();
    lines.push("=".repeat(60));
    lines.push("  AES Architecture Compliance Report (Self-Lint)".to_string());
    lines.push("=".repeat(60));
    lines.push(format!("  Project: {}", project_root));
    lines.push(format!("  Files scanned: {}", results.len()));
    lines.push("=".repeat(60));
    lines.push("".to_string());
    let mut critical = Vec::new();
    let mut high = Vec::new();
    let mut medium = Vec::new();
    let mut low = Vec::new();
    for r in &results {
        match r.severity {
            Severity::CRITICAL => critical.push(r),
            Severity::HIGH => high.push(r),
            Severity::MEDIUM => medium.push(r),
            Severity::LOW => low.push(r),
            _ => medium.push(r),
        }
    }
    for (sev, items) in [
        ("CRITICAL", &critical),
        ("HIGH", &high),
        ("MEDIUM", &medium),
        ("LOW", &low),
    ] {
        if items.is_empty() {
            continue;
        }
        lines.push(format!("  [{}] {} violations", sev, items.len()));
        lines.push("-".repeat(60));
        for r in items.iter() {
            lines.push(format!("  [{}] {}", r.code, r.file.value));
            for msg_line in r.message.value.lines() {
                lines.push(format!("    {}", msg_line));
            }
        }
        lines.push("".to_string());
    }
    let total = results.len();
    let mut per_code: BTreeMap<String, usize> = BTreeMap::new();
    for r in &results {
        *per_code.entry(r.code.to_string()).or_insert(0) += 1;
    }
    lines.push("=".repeat(60));
    lines.push(format!("  Total AES Violations: {}", total));
    lines.push(format!(
        "  Total Category AES Violations: {}",
        per_code.len()
    ));
    if !per_code.is_empty() {
        lines.push("-".repeat(60));
        for (code, count) in &per_code {
            lines.push(format!("  {}: {}", code, count));
        }
    }
    lines.push("".to_string());
    if total == 0 {
        lines.push("  Status: PASS - No AES violations detected".to_string());
    } else {
        lines.push("  Status: FAIL - AES violations detected".to_string());
    }
    lines.push("=".repeat(60));
    println!("Lint Arwaky v{} (AES Self-Lint)", env!("CARGO_PKG_VERSION"));
    println!("Scanning: {}", project_root);
    println!();
    println!("{}", lines.join("\n"));
    if total > 0 {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
```

---

## File: crates/cli-commands/src/surface_check_command.rs

```rust
// PURPOSE: CheckCommandsSurface — CLI surface for check/scan commands
//
/// This is the thin CLI surface that delegates all pipeline logic to the agent layer.
/// It handles path resolution, request construction, and output formatting.
use shared::cli_commands::contract_analysis_pipeline_aggregate::IAnalysisPipelineAggregate;
use shared::cli_commands::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use shared::cli_commands::taxonomy_scan_request_vo::{ScanMode, ScanRequest, ScanTarget};
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use std::process::ExitCode;
use std::sync::Arc;

/// SurfaceContext — DI container struct holding surface-level dependencies.
/// The agent layer (pipeline) is passed via the contract trait.
pub struct SurfaceContext {
    pub pipeline: Arc<dyn IAnalysisPipelineAggregate>,
    pub report_formatter: Arc<dyn IReportFormatterAggregate>,
    pub multi_project_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
}

pub struct CheckCommandsSurface {
    pub pipeline: Arc<dyn IAnalysisPipelineAggregate>,
    pub report_formatter: Arc<dyn IReportFormatterAggregate>,
    pub multi_project_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
}

impl CheckCommandsSurface {
    pub fn new(
        pipeline: Arc<dyn IAnalysisPipelineAggregate>,
        report_formatter: Arc<dyn IReportFormatterAggregate>,
        multi_project_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    ) -> Self {
        Self {
            pipeline,
            report_formatter,
            multi_project_orchestrator,
        }
    }

    /// Run the full analysis pipeline on a target path.
    ///
    /// This is a thin wrapper that delegates to the agent layer (IAnalysisPipelineAggregate).
    pub fn scan(&self, path: &str, filter: Option<&str>, format: Format) -> ExitCode {
        // Construct request and delegate to agent layer
        let request = ScanRequest {
            target: ScanTarget::new(path.to_string()),
            mode: ScanMode::Scan,
            filter: filter.map(String::from),
            member: None,
            format,
        };

        // Run pipeline via contract — use current-thread runtime to avoid nested runtime panic
        let rt = match crate::surface_common_command::create_current_thread_runtime() {
            Ok(r) => r,
            Err(_) => return ExitCode::from(2),
        };

        let report = match rt.block_on(self.pipeline.run(request)) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("[error] pipeline failed: {e}");
                return ExitCode::from(2);
            }
        };

        // Filter results to target path and display
        let filtered = self.filter_results_to_path(report.results, path);
        let report = ScanReport::new(filtered, vec![]);
        let output = self.report_formatter.format(&report, format);
        println!("{output}");

        if report.violation_count() > 0 {
            ExitCode::from(1)
        } else {
            ExitCode::SUCCESS
        }
    }

    /// Run the full analysis pipeline with multi-workspace discovery.
    ///
    /// This is a thin wrapper that delegates per-member scanning to the agent layer.
    pub fn scan_with_discovery(
        &self,
        path: &str,
        filter: Option<&str>,
        member: Option<&str>,
        format: Format,
    ) -> ExitCode {
        let path_obj = match FilePath::new(path.to_string()) {
            Ok(fp) => fp,
            Err(_) => {
                eprintln!("[error] invalid path: {path}");
                return ExitCode::from(2);
            }
        };

        let orchestrator = match self.multi_project_orchestrator.as_ref() {
            Some(o) => o.clone(),
            None => {
                eprintln!("[error] multi-project orchestrator not available");
                return ExitCode::from(2);
            }
        };

        // Use current-thread runtime — multi-threaded runtime causes nested runtime panic
        let rt = match crate::surface_common_command::create_current_thread_runtime() {
            Ok(r) => r,
            Err(_) => return ExitCode::from(2),
        };

        let workspaces = rt.block_on(orchestrator.discover_workspaces(&path_obj));

        if workspaces.is_empty() {
            // No workspaces discovered — fall back to single-scan mode
            return self.scan(path, filter, format);
        }

        // Filter to specific member if requested
        let workspaces = if let Some(member_name) = member {
            let all_workspaces = workspaces.clone();
            let filtered: Vec<_> = workspaces
                .into_iter()
                .filter(|ws| {
                    let ws_file = std::path::Path::new(&ws.path.value)
                        .file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_default();
                    ws_file == member_name || ws.path.value == member_name
                })
                .collect();
            if filtered.is_empty() {
                eprintln!("[error] no workspace member matching '{member_name}'");
                eprintln!();
                eprintln!("Available members:");
                for ws in &all_workspaces {
                    let name = std::path::Path::new(&ws.path.value)
                        .file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_default();
                    eprintln!("  - {} ({})", name, ws.workspace_type);
                }
                eprintln!();
                eprintln!("Usage: lint-arwaky-cli scan {path} --member <name>");
                return ExitCode::from(2);
            }
            filtered
        } else {
            workspaces
        };

        let multi = workspaces.len() > 1;
        if multi && matches!(format, Format::Text) {
            println!(
                "Lint Arwaky v{} (Multi-Workspace Mode)",
                env!("CARGO_PKG_VERSION")
            );
            println!("Found {} workspaces in {path}", workspaces.len());
            println!();
        }

        let mut global_all_results = Vec::new();
        let filter_str = filter.map(String::from);

        for ws in &workspaces {
            // Run pipeline for this workspace member via agent layer
            let request = ScanRequest {
                target: ScanTarget::new(ws.path.value.clone()),
                mode: ScanMode::Scan,
                filter: filter_str.clone(),
                member: Some(ws.workspace_type.clone()),
                format: Format::Text, // Internal format, not displayed
            };

            let report = match rt.block_on(self.pipeline.run(request)) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("[warn] pipeline failed for {}: {e}", ws.path.value);
                    continue;
                }
            };

            // Filter results to this workspace member's path
            let ws_canonical = std::path::Path::new(&ws.path.value).canonicalize().ok();
            let cwd_for_ws = match std::env::current_dir() {
                Ok(d) => d,
                Err(_) => std::path::PathBuf::new(),
            };
            let ws_fallback = {
                let raw = std::path::Path::new(&ws.path.value);
                if raw.is_absolute() {
                    raw.to_path_buf()
                } else {
                    cwd_for_ws.join(raw)
                }
            };
            let ws_fallback = std::fs::canonicalize(&ws_fallback).unwrap_or(ws_fallback);

            let filtered_results: Vec<_> = if let Some(code) = filter {
                report
                    .results
                    .into_iter()
                    .filter(|r| {
                        let abs_path = cwd_for_ws.join(&r.file.value);
                        r.code.code() == code
                            && (ws_canonical
                                .as_ref()
                                .map(|c| abs_path.starts_with(c))
                                .unwrap_or(false)
                                || abs_path.starts_with(&ws_fallback))
                    })
                    .collect()
            } else {
                report
                    .results
                    .into_iter()
                    .filter(|r| {
                        let abs_path = cwd_for_ws.join(&r.file.value);
                        ws_canonical
                            .as_ref()
                            .map(|c| abs_path.starts_with(c))
                            .unwrap_or(abs_path.starts_with(&ws_fallback))
                    })
                    .collect()
            };

            global_all_results.extend(filtered_results);
        }

        // Print per-workspace results
        if multi && matches!(format, Format::Text) {
            for ws in &workspaces {
                let ws_name = std::path::Path::new(&ws.path.value)
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_default();
                let ws_type = &ws.workspace_type;

                // Re-filter for this workspace
                let ws_canonical = std::path::Path::new(&ws.path.value).canonicalize().ok();
                let cwd_for_ws = match std::env::current_dir() {
                    Ok(d) => d,
                    Err(_) => std::path::PathBuf::new(),
                };
                let ws_fallback = {
                    let raw = std::path::Path::new(&ws.path.value);
                    if raw.is_absolute() {
                        raw.to_path_buf()
                    } else {
                        cwd_for_ws.join(raw)
                    }
                };
                let ws_fallback = std::fs::canonicalize(&ws_fallback).unwrap_or(ws_fallback);

                let member_results: Vec<_> = if let Some(code) = filter {
                    global_all_results
                        .iter()
                        .filter(|r| {
                            let abs_path = cwd_for_ws.join(&r.file.value);
                            r.code.code() == code
                                && (ws_canonical
                                    .as_ref()
                                    .map(|c| abs_path.starts_with(c))
                                    .unwrap_or(false)
                                    || abs_path.starts_with(&ws_fallback))
                        })
                        .collect()
                } else {
                    global_all_results
                        .iter()
                        .filter(|r| {
                            let abs_path = cwd_for_ws.join(&r.file.value);
                            ws_canonical
                                .as_ref()
                                .map(|c| abs_path.starts_with(c))
                                .unwrap_or(abs_path.starts_with(&ws_fallback))
                        })
                        .collect()
                };

                let total = member_results.len();
                println!("── [{ws_type}] {ws_name} — {total} violations ──");
                if !member_results.is_empty() {
                    let mut code_counts: std::collections::HashMap<String, usize> =
                        std::collections::HashMap::new();
                    for r in &member_results {
                        *code_counts.entry(r.code.to_string()).or_insert(0) += 1;
                    }
                    let mut sorted: Vec<_> = code_counts.into_iter().collect();
                    sorted.sort_by_key(|b| std::cmp::Reverse(b.1));
                    for (code, count) in &sorted {
                        println!("       {code}: {count}");
                    }
                } else {
                    println!("   (clean)");
                }
                println!();
            }
        } else {
            // Single workspace or non-text format — delegate formatting to aggregate
            let report = ScanReport::new(global_all_results.clone(), vec![]);
            let output = self.report_formatter.format(&report, format);
            println!("{output}");
        }

        if global_all_results.is_empty() {
            ExitCode::SUCCESS
        } else {
            ExitCode::from(1)
        }
    }

    /// Check if a single file is an orphan.
    pub fn check_orphan_single_file(&self, file_path: &str) {
        let scan_root = match crate::surface_check_action::find_workspace_root(file_path) {
            Some(r) => r,
            None => std::path::PathBuf::from("."),
        };

        // Call agent layer for orphan detection
        let file_results = match self
            .pipeline
            .check_orphan_single_file(file_path, &scan_root.to_string_lossy())
        {
            Ok(results) => results,
            Err(e) => {
                eprintln!("[error] orphan check failed: {e}");
                return;
            }
        };

        if file_results.is_empty() {
            println!(
                "  {} is NOT an orphan (reachable from entry point)",
                file_path
            );
        } else {
            println!("  {} is an ORPHAN:", file_path);
            for r in &file_results {
                println!("    [{}] {}", r.code, r.message);
            }
        }
    }

    /// Filter results to the target path.
    fn filter_results_to_path(&self, results: Vec<LintResult>, path: &str) -> Vec<LintResult> {
        let canonical_scan_path = std::path::PathBuf::from(path);
        let canonical_scan_path = canonical_scan_path
            .canonicalize()
            .unwrap_or(canonical_scan_path);
        let cwd = crate::surface_common_command::current_dir();

        results
            .into_iter()
            .filter(|r| {
                let abs_path = cwd.join(&r.file.value);
                abs_path.starts_with(&canonical_scan_path)
            })
            .collect()
    }
}
```

---

## File: crates/cli-commands/src/surface_common_command.rs

```rust
// PURPOSE: Shared utilities for CLI command surfaces
//
// Provides:
//   - create_runtime / create_current_thread_runtime: tokio runtime factories
//   - resolve_file_path / canonicalize_path / current_dir: path resolution helpers
//   - run_ci_analysis: CI pipeline that runs code analysis, computes score, compares
//     against threshold, and returns pass/fail exit code. Detects CRITICAL violations
//     as auto-fail regardless of score.
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_threshold_vo::Threshold;
use std::process::ExitCode;
use std::sync::Arc;

pub fn create_runtime() -> Result<tokio::runtime::Runtime, ExitCode> {
    match tokio::runtime::Runtime::new() {
        Ok(r) => Ok(r),
        Err(_) => {
            eprintln!("[error] failed to create tokio runtime");
            Err(ExitCode::from(2))
        }
    }
}

pub fn create_current_thread_runtime() -> Result<tokio::runtime::Runtime, ExitCode> {
    match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(r) => Ok(r),
        Err(_) => {
            eprintln!("[error] failed to create tokio runtime");
            Err(ExitCode::from(2))
        }
    }
}

pub fn resolve_file_path(path: &str) -> FilePath {
    FilePath::new(path.to_string()).unwrap_or_default()
}

pub fn canonicalize_path(path: &str) -> String {
    match std::path::Path::new(path).canonicalize() {
        Ok(p) => p.to_string_lossy().to_string(),
        Err(_) => path.to_string(),
    }
}

pub fn current_dir() -> std::path::PathBuf {
    match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => std::path::PathBuf::new(),
    }
}

pub fn run_ci_analysis(
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    path: Option<FilePath>,
    threshold: Threshold,
) -> ExitCode {
    let root = match path {
        Some(p) => p,
        None => FilePath::new(".").unwrap_or_default(),
    };
    let results = code_analysis_linter.run_code_analysis_path(&root);
    let score = code_analysis_linter.calc_score(&results);
    let has_crit = code_analysis_linter.check_critical(&results);
    // P2.7: compare as floats, not truncated u32
    let below_threshold = score.value() < threshold.value() as f64;

    println!("Architecture Compliance CI");
    println!("Score: {:.1} / 100", score.value());
    println!("Threshold: {}", threshold.value());
    println!();

    let mut reasons: Vec<String> = Vec::new();
    if has_crit {
        reasons.push("CRITICAL violation(s) detected — auto-fail triggered".to_string());
    }
    if below_threshold {
        reasons.push(format!(
            "Score below threshold ({:.1} < {})",
            score.value(),
            threshold.value()
        ));
    }

    let (mut critical_count, mut high_count, mut medium_count, mut low_count) = (0usize, 0, 0, 0);
    for r in &results {
        match r.severity {
            Severity::CRITICAL => critical_count += 1,
            Severity::HIGH => high_count += 1,
            Severity::MEDIUM => medium_count += 1,
            Severity::LOW => low_count += 1,
            _ => {}
        }
    }

    println!(
        "CRITICAL: {} | HIGH: {} | MEDIUM: {} | LOW: {}",
        critical_count, high_count, medium_count, low_count
    );
    println!();

    if reasons.is_empty() {
        println!("Result: PASS (exit code 0)");
        ExitCode::SUCCESS
    } else {
        for r in &reasons {
            println!("  {}", r);
        }
        println!("Result: FAIL (exit code 1)");
        ExitCode::from(1)
    }
}
```

---

## File: crates/cli-commands/src/surface_config_command.rs

```rust
// PURPOSE: ConfigCommandsSurface — CLI surface for config show
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use std::process::ExitCode;
use std::sync::Arc;

/// Redact sensitive values from config content.
///
/// Scans for common secret patterns (AWS keys, base64-encoded secrets) and
/// replaces them with [REDACTED] placeholders. Uses simple string matching
/// without regex to avoid adding new dependencies.
fn redact_secrets(content: &str) -> String {
    let mut result = content.to_string();

    // Redact AWS access key IDs (AKIA followed by 12+ alphanumeric chars)
    if result.contains("AKIA") {
        // Simple heuristic: replace AKIA + 16 alphanumeric chars with [REDACTED]
        let re = regex::Regex::new(r"AKIA[0-9A-Z]{16}").ok();
        if let Some(re) = re {
            result = re.replace_all(&result, "[REDACTED-AWS-KEY]").to_string();
        }
    }

    // Redact very long base64-like strings (40+ chars of base64 alphabet)
    if result.len() > 100 {
        let words: Vec<String> = result.split_whitespace().map(|s| s.to_string()).collect();
        for word in &words {
            if word.len() >= 40
                && word
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '/' | '+' | '='))
            {
                result = result.replacen(word, "[REDACTED]", 1);
            }
        }
    }

    result
}

pub async fn handle_config_show(
    _orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    config_reader: Arc<dyn IConfigReaderProtocol>,
) -> ExitCode {
    let project_root = FilePath::new(".".to_string()).unwrap_or_default();

    match config_reader.list_config_files(&project_root).await {
        Ok(config_files) if !config_files.is_empty() => {
            for (lang, path) in &config_files {
                match config_reader.read_config(&project_root, *lang).await {
                    Ok(Some(source)) => {
                        let path_str = path.value.as_str();
                        if config_files.len() > 1 {
                            println!("── [{}] {} ──", lang.as_str(), path_str);
                        } else {
                            println!("Found: {}", path_str);
                        }
                        // P5.2: redact secrets before displaying config content
                        let safe_content = redact_secrets(&source.raw_content);
                        println!("{safe_content}");
                    }
                    Ok(None) => {
                        // Should not happen since list_config_files found it
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to read config: {}", e);
                    }
                }
            }
        }
        Ok(_) => {
            println!("No config file found. Run `lint-arwaky init` to create one.");
        }
        Err(e) => {
            eprintln!("Failed to list config files: {}", e);
        }
    }
    ExitCode::SUCCESS
}
```

---

## File: crates/cli-commands/src/surface_fix_command.rs

```rust
// PURPOSE: FixCommandsSurface — CLI surface for auto-fix operations
//
// Runs lint → apply auto-fixes → re-lint to measure improvement.
// Supports dry-run mode (preview only) via the fix_orchestrator_factory closure.
//
// The factory pattern allows the DI container to control whether fixes are
// actually applied (real mode) or just simulated (dry-run).
//
// Fixable violations: AES101 (naming), AES203 (unused imports), AES304 (bypass)
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

pub struct FixCommandsSurface {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub fix_orchestrator_factory:
        Arc<dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync>,
}

impl FixCommandsSurface {
    pub fn new(
        code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
        fix_orchestrator_factory: Arc<
            dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync,
        >,
    ) -> Self {
        Self {
            code_analysis_linter,
            fix_orchestrator_factory,
        }
    }

    pub fn fix(&self, path: &str) {
        let canonical = match PathBuf::from(path).canonicalize() {
            Ok(p) => p,
            Err(_) => PathBuf::from(path),
        };
        let project_path = FilePath {
            value: canonical.to_string_lossy().to_string(),
        };
        self.run_fix(project_path, false);
    }

    pub fn run_fix(&self, project_path: FilePath, dry_run: bool) -> ExitCode {
        if dry_run {
            println!("[DRY-RUN] Previewing fixes for {}...", project_path.value);
        } else {
            println!("Applying safe fixes to {}...", project_path.value);
        }

        let results = self.code_analysis_linter.run_code_analysis(&project_path);
        println!("Found {} violations before fix (AES301-305 only; other rules not included in count — #107 P1 #15)", results.len());

        let fix_orch = (self.fix_orchestrator_factory)(dry_run);
        let fix_result = fix_orch.execute(&project_path);

        println!("{}", fix_result.output.value);

        if !dry_run {
            let after_results = self.code_analysis_linter.run_code_analysis(&project_path);
            let fixed_count = results.len().saturating_sub(after_results.len());
            println!(
                "Fixed {} violations ({} remaining, AES301-305 only — #107 P1 #15)",
                fixed_count,
                after_results.len()
            );
            if after_results.is_empty() {
                println!("Fix complete — all violations resolved.");
                ExitCode::SUCCESS
            } else {
                println!("Fix complete — {} violations remain.", after_results.len());
                ExitCode::from(1)
            }
        } else {
            println!("Dry-run complete — no changes applied.");
            ExitCode::SUCCESS
        }
    }
}

pub fn handle_fix(
    path: Option<FilePath>,
    dry_run: bool,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    fix_orchestrator_factory: Arc<
        dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync,
    >,
) -> ExitCode {
    let root = match path {
        Some(p) => p,
        None => FilePath::new(".").unwrap_or_default(),
    };
    let fix_surface = FixCommandsSurface::new(code_analysis_linter, fix_orchestrator_factory);
    fix_surface.run_fix(root, dry_run)
}
```

---

## File: crates/cli-commands/src/surface_git_command.rs

```rust
// PURPOSE: GitCommandsSurface — CLI surface for git-diff integration
//
// Runs AES analysis only on files changed since the specified git base (e.g. HEAD).
// Filters changed files through the language detector to skip non-lintable files.
//
// Use-case: pre-commit hooks and CI workflows that want per-file diff analysis.
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_git_vo::GitBranchName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use std::process::ExitCode;
use std::sync::Arc;

pub struct GitCommandsSurface {}

impl Default for GitCommandsSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl GitCommandsSurface {
    pub fn new() -> Self {
        Self {}
    }
}

pub async fn handle_git_diff(
    git_aggregate: Arc<dyn GitHooksAggregate>,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    base: GitBranchName,
    project_path: Option<&str>,
    filter: Option<&str>,
) -> ExitCode {
    println!("Lint Arwaky v{} (Git-Diff Mode)", env!("CARGO_PKG_VERSION"));

    // P2.5: use user-provided path instead of hardcoded "."
    let project_path = FilePath::new(project_path.unwrap_or(".").to_string()).unwrap_or_default();

    let changed_files = git_aggregate
        .diff_protocol()
        .get_changed_files(&project_path, &base)
        .await;

    // P2.5: apply filter to changed files
    let files: Vec<&shared::common::taxonomy_path_vo::FilePath> = changed_files
        .values
        .iter()
        .filter(|fp| {
            shared::common::utility_language_detector::is_lintable(fp)
                && filter.map(|f| fp.value.contains(f)).unwrap_or(true)
        })
        .collect();

    println!("Base: {} (changed files)", base.value());
    println!("Files changed: {}", files.len());
    println!();

    let mut total_violations = 0;
    for f in &files {
        let results = code_analysis_linter.run_code_analysis_path(f);
        let fv = results.len();
        total_violations += fv;
        if fv > 0 {
            println!("  {}  -> {} violation(s)", f.value, fv);
            for r in results.iter().take(3) {
                println!(
                    "    {}:{} [{}] {}",
                    r.file.value(),
                    r.line.value(),
                    match r.severity {
                        shared::common::taxonomy_severity_vo::Severity::CRITICAL => "CRITICAL",
                        shared::common::taxonomy_severity_vo::Severity::HIGH => "HIGH",
                        shared::common::taxonomy_severity_vo::Severity::MEDIUM => "MEDIUM",
                        shared::common::taxonomy_severity_vo::Severity::LOW => "LOW",
                        _ => "INFO",
                    },
                    r.message.value()
                );
            }
        } else {
            println!("  {}  -> clean", f.value);
        }
    }

    println!();
    println!(
        "{} violations across {} changed files",
        total_violations,
        files.len()
    );
    if total_violations > 0 {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
```

---

## File: crates/cli-commands/src/surface_maintenance_command.rs

```rust
// PURPOSE: MaintenanceCommandsSurface — CLI surface for maintenance (doctor, security, dependencies)
//
// Three independent subcommands, all delegated to MaintenanceCommandsAggregate:
//   - doctor:     toolchain diagnostics (cargo, python3, node, git, etc.)
//   - security:   vulnerability scan via cargo-audit (Rust) or bandit (Python)
//   - deps:       dependency report from Cargo.lock / pyproject.toml / requirements.txt

use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use std::process::ExitCode;
use std::sync::Arc;

pub async fn handle_doctor(
    maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
) -> ExitCode {
    println!("Environment Diagnostics");
    println!();

    let diag = maintenance_orchestrator.diagnose_toolchain().await;

    println!("Rust Toolchain:");
    for status in &diag.rust_tools {
        println!(
            "  {} {} {}  ({})",
            if status.status == "OK" { "✓" } else { "✗" },
            status.name,
            status.version,
            status.status
        );
    }
    if !diag.binary_path.is_empty() {
        println!("  binary: {}", diag.binary_path);
    }

    println!();
    println!("Python Toolchain:");
    for status in &diag.python_tools {
        println!(
            "  {} {} {}  ({})",
            if status.status == "OK" { "✓" } else { "✗" },
            status.name,
            status.version,
            status.status
        );
    }

    println!();
    println!("JavaScript Toolchain:");
    for status in &diag.js_tools {
        println!(
            "  {} {} {}  ({})",
            if status.status == "OK" { "✓" } else { "✗" },
            status.name,
            status.version,
            status.status
        );
    }

    println!();
    println!("VCS:");
    for status in &diag.vcs_tools {
        println!(
            "  {} {} {}  ({})",
            if status.status == "OK" { "✓" } else { "✗" },
            status.name,
            status.version,
            status.status
        );
    }

    ExitCode::SUCCESS
}

pub async fn handle_security(
    maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<FilePath>,
) -> ExitCode {
    let target = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    let fp = match path {
        Some(p) => p,
        None => FilePath::new(".").unwrap_or_default(),
    };
    println!("Security Vulnerability Scan — {}", target);
    println!();

    let report = maintenance_orchestrator.run_security_scan(&fp).await;

    println!("Language: {}", report.language);
    println!("Tool: {}", report.tool_name);

    if !report.tool_installed {
        eprintln!("Error: {} is not installed.", report.tool_name);
        // P5.1: return exit code 3 (tool missing) instead of success
        return ExitCode::from(3);
    }

    println!("Findings: {}", report.findings.len());
    for f in &report.findings {
        println!(
            "  {} {} {}:{} {}",
            f.severity.to_uppercase(),
            f.test_id,
            f.file,
            f.line,
            f.issue
        );
    }

    ExitCode::SUCCESS
}

pub async fn handle_dependencies(
    maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<FilePath>,
) -> ExitCode {
    let target = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    let fp = match path {
        Some(p) => p,
        None => FilePath::new(".").unwrap_or_default(),
    };
    println!("Dependency Report — {}", target);
    println!();

    match maintenance_orchestrator.run_dependency_report(&fp).await {
        Ok(report) => {
            println!("Language: {}", report.language);
            println!("Dependencies: {} total", report.dependencies.len());
            println!();
            println!("{:<25} {:<12} Type", "Package", "Version");
            for dep in report.dependencies.iter().take(30) {
                println!("{:<25} {:<12} {}", dep.name, dep.version, dep.dep_type);
            }
            if report.dependencies.len() > 30 {
                println!("... and {} more", report.dependencies.len() - 30);
            }
        }
        Err(e) => {
            println!("{e}");
            return ExitCode::from(2);
        }
    }

    ExitCode::SUCCESS
}
```

---

## File: crates/cli-commands/src/surface_plugin_command.rs

```rust
// PURPOSE: PluginCommandsSurface — CLI surface for listing adapters/plugins
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use std::process::ExitCode;
use std::sync::Arc;

pub fn handle_adapters(external_lint: Arc<dyn IExternalLintAggregate>) -> ExitCode {
    println!("External lint adapters:");
    let adapters = external_lint.adapter_names();
    if adapters.is_empty() {
        println!("  (none enabled)");
    } else {
        for adapter in &adapters {
            println!("  - {adapter}");
        }
    }
    ExitCode::SUCCESS
}
```

---

## File: crates/cli-commands/src/surface_setup_command.rs

```rust
// PURPOSE: SetupCommandsSurface — CLI surface for project setup (init, install, mcp-config)
//
// Three subcommands:
//   - init:        writes lint_arwaky.config.<lang>.yaml (local)
//   - install:     pip install Python adapters (ruff, mypy, bandit) + npm install JS adapters (eslint, prettier, typescript)
//   - mcp-config:  prints MCP client config JSON for Claude/Cursor/Windsurf/Copilot
//
// Binary resolution for mcp-config: checks sibling of current exe first, fails closed (no PATH fallback).

use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use std::process::ExitCode;
use std::sync::Arc;

pub fn handle_init(setup_orchestrator: Arc<dyn SetupManagementAggregate>) -> ExitCode {
    // 1. Write language config files
    let mut all_ok = true;
    let languages = setup_orchestrator.detect_languages();
    for lang in languages.iter() {
        let lang_str = lang.value();
        let target = format!("lint_arwaky.config.{}.yaml", lang_str);
        if setup_orchestrator.file_exists(&target) {
            println!("Config already exists: {}", target);
        } else {
            let content = setup_orchestrator.get_config_template(lang_str);
            match setup_orchestrator.write_config_file(&target, content) {
                Ok(desc) => {
                    println!("Config created: {} (language: {})", target, lang_str);
                    println!("  {}", desc.value);
                }
                Err(e) => {
                    println!("Error creating config for {}: {e}", lang_str);
                    all_ok = false;
                }
            }
        }
    }

    // 2. Distribute docs + SKILL.md from XDG config to project
    let doc_files = [
        "SKILL.md",
        "ARCHITECTURE.md",
        "MIGRATION_RUST.md",
        "MIGRATION_PYTHON.md",
        "MIGRATION_TYPESCRIPT.md",
        "RULES_AES.md",
    ];
    if let Some(config_dir) = dirs::config_dir() {
        let xdg_base = config_dir.join("lint-arwaky");
        for doc in &doc_files {
            if setup_orchestrator.file_exists(doc) {
                println!("  {doc} — already exists, skipping");
                continue;
            }
            let xdg_src = xdg_base.join(doc);
            if !xdg_src.exists() {
                println!("  {doc} — not in XDG config, skipping");
                continue;
            }
            match std::fs::read_to_string(&xdg_src) {
                Ok(content) => {
                    if let Some(parent) = std::path::Path::new(doc).parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    match setup_orchestrator.write_config_file(doc, &content) {
                        Ok(_) => println!("  {doc} — distributed from XDG config"),
                        Err(e) => println!("  {doc} — error: {e}"),
                    }
                }
                Err(e) => println!("  {doc} — read error: {e}"),
            }
        }
    } else {
        println!("Warning: could not determine XDG config dir");
    }

    if all_ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

pub async fn handle_install(
    setup_orchestrator: Arc<dyn SetupManagementAggregate>,
    sudo: bool,
) -> ExitCode {
    println!("Lint Arwaky — Install Adapter Dependencies");
    println!("{}", "=".repeat(50));

    println!("\n[1/2] Installing Python adapters (ruff, mypy, bandit)...");
    let py_status = setup_orchestrator.install_python_adapters().await;
    if py_status.value {
        println!("  Python adapters installed");
    } else {
        println!("  Failed to install Python adapters");
    }

    println!("\n[2/2] Installing JavaScript adapters (eslint, prettier, typescript)...");
    let js_status = setup_orchestrator.install_javascript_adapters(sudo).await;
    if js_status.value {
        println!("  JavaScript adapters installed");
    } else {
        println!("  Failed to install JavaScript adapters");
    }

    println!("\n{}", "=".repeat(50));
    if py_status.value && js_status.value {
        println!("Done! Run `lint-arwaky doctor` to verify.");
        ExitCode::SUCCESS
    } else {
        println!("Installation failed. Run with `--sudo` if npm globally requires permissions.");
        ExitCode::from(1)
    }
}

pub fn handle_mcp_config(client: &str) -> ExitCode {
    let binary = which_mcp_binary();
    let config = match client {
        "claude-code" | "claude" => serde_json::json!({
            "mcpServers": {
                "lint-arwaky": {
                    "command": binary,
                    "args": [],
                    "env": {}
                }
            }
        }),
        "cursor" => serde_json::json!({
            "mcpServers": {
                "lint-arwaky": {
                    "command": binary,
                    "args": [],
                    "env": {}
                }
            }
        }),
        "windsurf" => serde_json::json!({
            "config:lint-arwaky": {
                "command": binary,
                "args": [],
                "env": {}
            }
        }),
        "copilot" => serde_json::json!({
            "inputs": [],
            "server": {
                "command": binary,
                "args": [],
                "env": {}
            }
        }),
        "hermes" => serde_json::json!({
            "mcpServers": {
                "lint-arwaky": {
                    "command": binary,
                    "args": [],
                    "env": {}
                }
            }
        }),
        "vscode" => serde_json::json!({
            "mcpServers": {
                "lint-arwaky": {
                    "command": binary,
                    "args": [],
                    "env": {}
                }
            }
        }),
        "all" => serde_json::json!({
            "mcpServers": {
                "lint-arwaky": {
                    "command": binary,
                    "args": [],
                    "env": {}
                }
            }
        }),
        _ => serde_json::json!({
            "mcpServers": {
                "lint-arwaky": {
                    "command": binary,
                    "args": [],
                    "env": {}
                }
            }
        }),
    };
    let json_str = serde_json::to_string_pretty(&config).unwrap_or_default();
    println!("MCP Client Configuration for: {}", client);
    println!("Binary: {}", binary);
    println!();
    println!("{}", json_str);
    ExitCode::SUCCESS
}

fn which_mcp_binary() -> String {
    match resolve_mcp_binary() {
        Ok(path) => path.to_string_lossy().into_owned(),
        Err(_) => {
            // Fail closed — do not fall back to bare PATH lookup.
            // Caller should handle the error gracefully; we provide a fallback hint.
            "lint-arwaky-mcp".to_string()
        }
    }
}

/// Resolve the MCP binary to an absolute canonicalized path.
///
/// Resolution order:
///   1. LINT_ARWAKY_MCP_BIN env var (explicit override)
///   2. Sibling of current executable
///   3. Fail closed — no bare PATH fallback (prevents PATH hijacking)
fn resolve_mcp_binary() -> Result<std::path::PathBuf, String> {
    // 1. Explicit override
    if let Ok(explicit) = std::env::var("LINT_ARWAKY_MCP_BIN") {
        let path = std::path::PathBuf::from(&explicit);
        if !path.is_file() {
            return Err(format!(
                "LINT_ARWAKY_MCP_BIN points to non-file: {}",
                path.display()
            ));
        }
        return path
            .canonicalize()
            .map_err(|e| format!("cannot canonicalize LINT_ARWAKY_MCP_BIN: {e}"));
    }

    // 2. Sibling of current executable
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let sibling = dir.join("lint-arwaky-mcp");
            if sibling.is_file() {
                return sibling
                    .canonicalize()
                    .map_err(|e| format!("cannot canonicalize sibling: {e}"));
            }
        }
    }

    // 3. Do NOT fall back to bare PATH — fail closed (P1.2)
    Err("lint-arwaky-mcp not found. Set LINT_ARWAKY_MCP_BIN to an absolute path.".into())
}
```

---

## File: crates/cli-commands/src/surface_watch_command.rs

```rust
// PURPOSE: WatchCommandsSurface — CLI surface for file watching with auto-lint on changes
//
// Creates a WatchConfig from the given path, sets up Ctrl+C signal handling,
// and delegates to IWatchAggregate.run() which blocks until interrupted.
//
// The atomic `running` flag coordinates graceful shutdown on Ctrl+C.
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use shared::common::taxonomy_path_vo::FilePath;
use shared::file_watch::contract_watch_aggregate::IWatchAggregate;
use shared::file_watch::taxonomy_watch_config_vo::WatchConfig;

pub struct WatchCommandsSurface {}

impl Default for WatchCommandsSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl WatchCommandsSurface {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn handle_watch(watch_aggregate: Arc<dyn IWatchAggregate>, path: Option<FilePath>) -> ExitCode {
    let root = match path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    let config = WatchConfig::from_path(root);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    if let Err(e) = ctrlc::set_handler(move || {
        eprintln!("\nStopping watcher...");
        r.store(false, Ordering::SeqCst);
    }) {
        eprintln!("[error] failed to set Ctrl+C handler: {}", e);
        return ExitCode::from(2);
    }

    watch_aggregate.run(config, running)
}
```

---

## File: crates/cli-commands/src/utility_format_output.rs

```rust
// PURPOSE: Stateless formatting utilities for JUnit XML and generic escaping
//
// SARIF output delegates to report_formatter::SarifFormatter to avoid duplication.

use report_formatter::SarifFormatter;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;

/// Format lint results as a SARIF 2.1.0 JSON string.
///
/// Delegates to the shared SarifFormatter from report-formatter crate
/// to avoid code duplication (AES305).
pub fn format_sarif_output(results: &[LintResult]) -> String {
    let formatter = SarifFormatter::new();
    formatter.format_sarif(results).to_string()
}

/// Format lint results as JUnit XML.
pub fn format_junit_output(results: &[LintResult]) -> String {
    let total = results.len();
    let failures: Vec<_> = results
        .iter()
        .filter(|r| {
            matches!(
                r.severity,
                Severity::CRITICAL | Severity::HIGH | Severity::MEDIUM | Severity::LOW
            )
        })
        .collect();
    let failure_count = failures.len();

    let mut xml = String::with_capacity(total.saturating_mul(256));
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str(&format!(
        "<testsuites name=\"lint-arwaky\" tests=\"{total}\" failures=\"{failure_count}\">\n"
    ));
    xml.push_str(&format!(
        "  <testsuite name=\"lint-arwaky\" tests=\"{total}\" failures=\"{failure_count}\">\n"
    ));

    for r in results {
        let classname = xml_escape(&r.code.to_string());
        let name = xml_escape(&format!("{}:{}", r.file.value, r.line.value()));
        let message = xml_escape(&r.message.value);
        let sev = r.severity.to_string();
        let is_info = r.severity == Severity::INFO;

        xml.push_str(&format!(
            "    <testcase classname=\"{classname}\" name=\"{name}\">\n"
        ));
        if !is_info {
            xml.push_str(&format!(
                "      <failure message=\"{sev}: {message}\" type=\"{sev}\">\n"
            ));
            xml.push_str(&format!("        {message}\n"));
            xml.push_str("      </failure>\n");
        }
        xml.push_str("    </testcase>\n");
    }

    xml.push_str("  </testsuite>\n");
    xml.push_str("</testsuites>\n");
    xml
}

/// XML-escape a string for safe inclusion in JUnit XML output.
pub fn xml_escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            other => escaped.push(other),
        }
    }
    escaped
}
```

---

## File: crates/shared/src/auto-fix/contract_fix_aggregate.rs

```rust
// PURPOSE: LintFixOrchestratorAggregate — aggregate trait for auto-fix orchestration
use crate::auto_fix::taxonomy_fix_vo::FixResult;
use crate::common::taxonomy_path_vo::FilePath;

/// Aggregate that drives the auto-fix pipeline for a single file.
///
/// Implementations coordinate protocol dependencies (file adapter, renamer,
/// etc.) and produce a [`FixResult`] summarising what was changed or why
/// the fix could not be applied.
pub trait LintFixOrchestratorAggregate: Send + Sync {
    fn execute(&self, path: &FilePath) -> FixResult;
}
```

---

## File: crates/shared/src/auto-fix/mod.rs

```rust
// auto-fix — taxonomy and contract types
pub mod contract_file_adapter_protocol;
pub mod contract_fix_aggregate;
pub mod contract_fix_protocol;
pub mod taxonomy_fix_applied_event;
pub mod taxonomy_fix_vo;
pub mod utility_symbol_renamer;
```

---

## File: crates/shared/src/cli-commands/contract_analysis_pipeline_aggregate.rs

```rust
// PURPOSE: IAnalysisPipelineAggregate — aggregate trait for the full analysis pipeline
//
// Defines the public API for running all linter groups in sequence and returning
// a unified ScanReport. This is what the surface layer depends on to orchestrate
// code-analysis, naming, imports, external adapters, roles, and orphan detection.
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_scan_report_vo::{PipelineError, ScanReport};
use crate::cli_commands::taxonomy_scan_request_vo::ScanRequest;

/// IAnalysisPipelineAggregate — aggregate port for full analysis pipeline orchestration.
///
/// Implemented by AnalysisPipelineOrchestrator (agent layer).
/// Provides methods for running the complete lint pipeline on a target,
/// with multi-workspace discovery support and single-file orphan checking.
#[async_trait::async_trait]
pub trait IAnalysisPipelineAggregate: Send + Sync {
    /// Run the full analysis pipeline on the request target.
    async fn run(&self, request: ScanRequest) -> Result<ScanReport, PipelineError>;

    /// Run the full analysis pipeline with multi-workspace discovery.
    ///
    /// Discovers workspace members (Cargo.toml, pyproject.toml, package.json workspaces),
    /// runs all 6 linter groups per member, runs cross-workspace orphan detection,
    /// filters results to each member's path, and aggregates into a single ScanReport.
    async fn run_with_discovery(&self) -> Result<ScanReport, PipelineError>;

    /// Check if a single file is an orphan.
    ///
    /// Scans ALL source files to build the import graph for reachability analysis,
    /// then filters results to only the specified file path.
    fn check_orphan_single_file(
        &self,
        file_path: &str,
        workspace_root: &str,
    ) -> Result<Vec<LintResult>, PipelineError>;
}
```

---

## File: crates/shared/src/cli-commands/contract_report_formatter_aggregate.rs

```rust
// PURPOSE: IReportFormatterAggregate — aggregate trait for report formatting
// AES402: All primitive types replaced with taxonomy VOs.
//   * `String` return → `DisplayContent` (semantic formatted output)
//
// Surface layer depends on this aggregate to format ScanReport output.
// The aggregate delegates to the appropriate capabilities formatter
// (text, json, sarif, junit) based on the requested format.
use crate::cli_commands::taxonomy_format_vo::Format;
use crate::cli_commands::taxonomy_scan_report_vo::ScanReport;
use crate::common::taxonomy_display_content_vo::DisplayContent;

/// IReportFormatterAggregate — aggregate port for report formatting.
///
/// Implemented by ReportFormatterOrchestrator (agent layer).
/// Provides a single method for formatting a ScanReport into any supported format.
pub trait IReportFormatterAggregate: Send + Sync {
    /// Format the scan report into the specified output format.
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent;
}
```

---

## File: crates/shared/src/cli-commands/contract_report_formatter_protocol.rs

```rust
// PURPOSE: IReportFormatterProtocol — protocol for formatting ScanReport output
// AES402: All primitive types replaced with taxonomy VOs.
//   * `String` return → `DisplayContent` (semantic formatted output)
//
// Defines the contract that all report formatters must implement. Each formatter
// (text, json, sarif, junit) implements this trait to produce output in its
// respective format.
use crate::cli_commands::taxonomy_format_vo::Format;
use crate::cli_commands::taxonomy_scan_report_vo::ScanReport;
use crate::common::taxonomy_display_content_vo::DisplayContent;

/// IReportFormatterProtocol — protocol for formatting analysis results.
///
/// Implemented by TextFormatter, JsonFormatter, SarifFormatter, and JunitFormatter.
/// Each formatter converts a ScanReport into its respective output format.
pub trait IReportFormatterProtocol: Send + Sync {
    /// Format the scan report into the specified output format.
    ///
    /// # Arguments
    /// * `report` - The ScanReport to format
    /// * `format` - The desired output format
    ///
    /// # Returns
    /// Formatted output wrapped in DisplayContent VO.
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent;

    /// Return the supported format name (e.g., "text", "json").
    fn supported_format(&self) -> Format;
}
```

---

## File: crates/shared/src/cli-commands/mod.rs

```rust
// cli-commands — taxonomy and contract types
pub mod contract_analysis_pipeline_aggregate;
pub mod contract_report_formatter_aggregate;
pub mod contract_report_formatter_protocol;
pub mod taxonomy_catalog_constant;

pub mod taxonomy_cli_vo;
pub mod taxonomy_command_catalog_vo;
pub mod taxonomy_format_vo;
pub mod taxonomy_metadata_vo;
pub mod taxonomy_position_vo;
pub mod taxonomy_protocol_vo;
pub mod taxonomy_result_vo;
pub mod taxonomy_scan_report_vo;
pub mod taxonomy_scan_request_vo;
pub mod taxonomy_score_vo;
pub mod taxonomy_severity_vo;
```

---

## File: crates/shared/src/cli-commands/taxonomy_catalog_constant.rs

```rust
// PURPOSE: COMMAND_CATALOG — static table of every public CLI/MCP command with description and usage example

pub static COMMAND_CATALOG: &[(&str, &str, &str)] = &[
    (
        "check",
        "Run full architecture compliance analysis",
        "lint-arwaky-cli check /path",
    ),
    (
        "scan",
        "Deep directory scan (alias for check)",
        "lint-arwaky-cli scan ./src/",
    ),
    ("fix", "Apply safe fixes", "lint-arwaky-cli fix file.py"),
    (
        "ci",
        "CI-optimized with exit codes",
        "lint-arwaky-cli ci /path --threshold 80",
    ),
    (
        "doctor",
        "Diagnose environment health",
        "lint-arwaky-cli doctor",
    ),
    (
        "orphan",
        "Check if file is dead/unreachable code",
        "lint-arwaky-cli orphan <path>",
    ),
    (
        "security",
        "Vulnerability scanning",
        "lint-arwaky-cli security /path",
    ),
    (
        "dependencies",
        "Dependency vulnerability scan",
        "lint-arwaky-cli dependencies .",
    ),
    (
        "watch",
        "Watch files for changes",
        "lint-arwaky-cli watch ./src/",
    ),
    (
        "install-hook",
        "Install git pre-commit hook",
        "lint-arwaky-cli install-hook",
    ),
    (
        "uninstall-hook",
        "Remove git pre-commit hook",
        "lint-arwaky-cli uninstall-hook",
    ),
    (
        "adapters",
        "List enabled adapters",
        "lint-arwaky-cli adapters",
    ),
    ("version", "Show version", "lint-arwaky-cli version"),
    ("init", "Create default config", "lint-arwaky-cli init"),
    (
        "install",
        "Install adapter dependencies",
        "lint-arwaky-cli install",
    ),
    (
        "mcp-config",
        "Print MCP server config",
        "lint-arwaky-cli mcp-config --client claude",
    ),
    (
        "config-show",
        "Show active configuration",
        "lint-arwaky-cli config-show",
    ),
];
```

---

## File: crates/shared/src/cli-commands/taxonomy_cli_vo.rs

```rust
// PURPOSE: Cli + Commands enums — clap-based CLI definition with all subcommands
use clap::{Parser, Subcommand};

use crate::cli_commands::taxonomy_format_vo::Format;

#[derive(Parser, Debug)]
#[command(name = "lint-arwaky")]
#[command(about = "Lint Arwaky CLI: Autonomous Code Quality Gatekeeper.", long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Show debug information
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Minimize output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Directory to save output reports (overrides config)
    #[arg(short, long, global = true)]
    pub output_dir: Option<String>,

    /// Filter output by AES rule code (e.g. AES101, AES102, AES301, AES303)
    #[arg(long, global = true)]
    pub filter: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run all linters and calculate score
    Check {
        /// Path to check
        path: Option<String>,
        /// Only check git diff
        #[arg(long)]
        git_diff: bool,
        /// Output format: text, json, sarif, junit
        #[arg(long, default_value_t = Format::Text)]
        format: Format,
    },

    /// Alias for check (CI-friendly). Discovers workspace members and runs all linters.
    /// Use `--member <name>` to scan a specific workspace member.
    Scan {
        /// Path to scan
        path: Option<String>,
        /// Scan only a specific workspace member by name (e.g. "shared", "import-rules")
        #[arg(long)]
        member: Option<String>,
        /// Output format: text, json, sarif, junit
        #[arg(long, default_value_t = Format::Text)]
        format: Format,
    },

    /// Apply safe automatic fixes
    Fix {
        /// Path to fix
        path: Option<String>,
        /// Preview changes without applying them
        #[arg(long)]
        dry_run: bool,
    },

    /// CI mode (exit 1 if score < threshold)
    Ci {
        /// Path to lint
        path: Option<String>,
        /// Minimum quality score to pass
        #[arg(long, default_value_t = 80)]
        threshold: u32,
    },

    /// Diagnose environment health
    Doctor,

    /// Check if a file is an orphan (AES501-AES506)
    Orphan {
        /// File path to check
        path: String,
    },

    /// Scan for security vulnerabilities
    Security {
        /// Path to scan
        path: Option<String>,
    },

    /// Scan for library vulnerabilities
    Dependencies {
        /// Path to scan
        path: Option<String>,
    },

    /// Watch and lint on changes
    Watch {
        /// Path to watch
        path: Option<String>,
    },

    /// Install git pre-commit hook
    InstallHook,

    /// Remove git pre-commit hook
    UninstallHook,

    /// Show version
    Version,

    /// List active linters/adapters
    Adapters,

    /// Create default config
    Init,

    /// Install linter adapter dependencies
    Install {
        /// Use sudo for npm global install
        #[arg(long)]
        sudo: bool,
    },

    /// Print MCP server config for clients
    McpConfig {
        /// Client type (claude, hermes, vscode, all)
        #[arg(long, default_value = "all")]
        client: String,
    },

    /// Show active configuration
    ConfigShow,
}

pub fn get_cli() -> Cli {
    Cli::parse()
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_command_catalog_vo.rs

```rust
// PURPOSE: CommandCatalogVO — maps ActionName to CommandMetadataVO for all CLI commands
// Derives from COMMAND_CATALOG (single source of truth in taxonomy_catalog_constant)
use crate::cli_commands::taxonomy_catalog_constant::COMMAND_CATALOG;
use crate::cli_commands::taxonomy_metadata_vo::CommandMetadataVO;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::common::taxonomy_suggestion_vo::Suggestion;
use crate::mcp_server::taxonomy_action_vo::ActionName;
use std::collections::HashMap;

pub struct CommandCatalogVO {}

impl CommandCatalogVO {
    /// Derive the full command catalog from COMMAND_CATALOG (single source of truth).
    pub fn command_catalog() -> HashMap<ActionName, CommandMetadataVO> {
        let mut catalog = HashMap::new();
        for (name, description, example) in COMMAND_CATALOG {
            catalog.insert(
                ActionName::from(*name),
                CommandMetadataVO::new(DescriptionVO::new(*description), Suggestion::new(*example)),
            );
        }
        catalog
    }
}

pub fn command_catalog() -> HashMap<ActionName, CommandMetadataVO> {
    CommandCatalogVO::command_catalog()
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_format_vo.rs

```rust
// PURPOSE: Format — output format enum for --format CLI arg (text, json, sarif, junit)
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Format {
    #[default]
    Text,
    Json,
    Sarif,
    Junit,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::Text => write!(f, "text"),
            Format::Json => write!(f, "json"),
            Format::Sarif => write!(f, "sarif"),
            Format::Junit => write!(f, "junit"),
        }
    }
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(Format::Text),
            "json" => Ok(Format::Json),
            "sarif" => Ok(Format::Sarif),
            "junit" => Ok(Format::Junit),
            other => Err(format!(
                "unknown format '{other}': expected one of text, json, sarif, junit"
            )),
        }
    }
}

impl ValueEnum for Format {
    fn value_variants<'a>() -> &'a [Self] {
        &[Format::Text, Format::Json, Format::Sarif, Format::Junit]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Format::Text => Some(clap::builder::PossibleValue::new("text")),
            Format::Json => Some(clap::builder::PossibleValue::new("json")),
            Format::Sarif => Some(clap::builder::PossibleValue::new("sarif")),
            Format::Junit => Some(clap::builder::PossibleValue::new("junit")),
        }
    }
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_metadata_vo.rs

```rust
// PURPOSE: CommandMetadataVO — value object wrapping description + usage example for each CLI command
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::common::taxonomy_suggestion_vo::Suggestion;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandMetadataVO {
    pub description: DescriptionVO,
    pub example: Suggestion,
}

impl CommandMetadataVO {
    pub fn new(description: DescriptionVO, example: Suggestion) -> Self {
        Self {
            description,
            example,
        }
    }
}

impl std::fmt::Display for CommandMetadataVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.description, self.example)
    }
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_position_vo.rs

```rust
// PURPOSE: Position — value object for source code position tracking (file, line, column)
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub line: LineNumber,
    #[serde(default)]
    pub column: ColumnNumber,
}

impl Position {
    pub fn new(line: LineNumber) -> Self {
        Self {
            line,
            column: ColumnNumber::new(0),
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.column.value > 0 {
            write!(f, "{}:{}", self.line, self.column)
        } else {
            write!(f, "{}", self.line)
        }
    }
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_protocol_vo.rs

```rust
// PURPOSE: TransportEndpoint, TransportProtocol, TransportUrlVO — value objects for transport endpoint configuration
use crate::string_value_object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransportEndpoint {
    pub protocol: TransportProtocol,
    pub address: String,
}

impl Default for TransportEndpoint {
    fn default() -> Self {
        Self {
            protocol: TransportProtocol::STDAggregate,
            address: String::new(),
        }
    }
}

impl TransportEndpoint {
    pub fn new(protocol: TransportProtocol, address: String) -> Self {
        Self { protocol, address }
    }

    pub fn display_name(&self) -> String {
        match self.protocol {
            TransportProtocol::HTTP => format!("HTTP({})", self.address),
            TransportProtocol::UnixSocket => format!("Socket({})", self.address),
            TransportProtocol::STDAggregate => "Stdio(direct)".to_string(),
        }
    }
    pub fn from_url(url: &str) -> Self {
        let (protocol, address) = match url {
            u if u.starts_with("http://") || u.starts_with("https://") => {
                (TransportProtocol::HTTP, u.to_string())
            }
            "stdio" => (TransportProtocol::STDAggregate, "stdio".to_string()),
            u if u.starts_with('/') || u.starts_with('.') => {
                (TransportProtocol::UnixSocket, u.to_string())
            }
            _ => (TransportProtocol::STDAggregate, "stdio".to_string()),
        };
        Self { protocol, address }
    }
}

impl std::fmt::Display for TransportEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.protocol, self.address)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TransportProtocol {
    #[serde(rename = "HTTP")]
    HTTP,
    #[serde(rename = "UnixSocket")]
    UnixSocket,
    #[serde(rename = "Stdio")]
    STDAggregate,
}

impl std::fmt::Display for TransportProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportProtocol::HTTP => write!(f, "HTTP"),
            TransportProtocol::UnixSocket => write!(f, "UnixSocket"),
            TransportProtocol::STDAggregate => write!(f, "Stdio"),
        }
    }
}

impl TransportProtocol {
    pub fn needs_desktop_commander(&self) -> bool {
        matches!(
            self,
            TransportProtocol::HTTP | TransportProtocol::UnixSocket
        )
    }
}

string_value_object!(TransportUrlVO);
```

---

## File: crates/shared/src/cli-commands/taxonomy_result_vo.rs

```rust
// PURPOSE: LintResult, LintResultList, FilePathSet — value objects for lint violation results
use serde::{Deserialize, Serialize};

use crate::cli_commands::taxonomy_position_vo::Position;
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_lint_vo::LocationList;
use crate::common::taxonomy_lint_vo::ScopeRef;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;
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

## File: crates/shared/src/cli-commands/taxonomy_scan_report_vo.rs

```rust
// PURPOSE: ScanReport VO — output of the analysis pipeline
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_common_vo::Score;

/// Severity level for pipeline diagnostics.
#[derive(Debug, Clone)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

/// A diagnostic message from a pipeline subsystem.
pub struct PipelineDiagnostic {
    pub source: String,
    pub message: String,
    pub severity: DiagnosticSeverity,
}

impl PipelineDiagnostic {
    pub fn new(source: String, message: String, severity: DiagnosticSeverity) -> Self {
        Self {
            source,
            message,
            severity,
        }
    }
}

/// Error types that can occur during pipeline execution.
#[derive(Debug, Clone)]
pub enum PipelineError {
    PathNotFound(String),
    InvalidPath(String),
    WorkspaceDiscovery(String),
    Analysis(String),
    Io(String),
}

impl std::fmt::Display for PipelineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineError::PathNotFound(p) => write!(f, "path not found: {p}"),
            PipelineError::InvalidPath(p) => write!(f, "invalid path: {p}"),
            PipelineError::WorkspaceDiscovery(e) => write!(f, "workspace discovery failed: {e}"),
            PipelineError::Analysis(e) => write!(f, "analysis failed: {e}"),
            PipelineError::Io(e) => write!(f, "io error: {e}"),
        }
    }
}

impl std::error::Error for PipelineError {}

/// Results of the full analysis pipeline.
pub struct ScanReport {
    pub results: Vec<LintResult>,
    pub diagnostics: Vec<PipelineDiagnostic>,
    pub score: Option<Score>,
}

impl ScanReport {
    pub fn new(results: Vec<LintResult>, diagnostics: Vec<PipelineDiagnostic>) -> Self {
        Self {
            results,
            diagnostics,
            score: None,
        }
    }

    /// Return the number of violations (results with severity > INFO).
    pub fn violation_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.severity != crate::cli_commands::taxonomy_severity_vo::Severity::INFO)
            .count()
    }

    /// Attach a score to the report.
    pub fn with_score(mut self, score: Score) -> Self {
        self.score = Some(score);
        self
    }
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_scan_request_vo.rs

```rust
// PURPOSE: ScanRequest VO — request payload for the analysis pipeline
use crate::cli_commands::taxonomy_format_vo::Format;

/// Target path for the scan.
pub struct ScanTarget {
    pub value: String,
}

impl ScanTarget {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Default for ScanTarget {
    fn default() -> Self {
        Self {
            value: ".".to_string(),
        }
    }
}

/// Mode of analysis to run.
#[derive(Debug, Clone, Default)]
pub enum ScanMode {
    #[default]
    Check,
    Scan,
    Ci {
        threshold: u32,
    },
}

/// Request to run the full analysis pipeline.
pub struct ScanRequest {
    pub target: ScanTarget,
    pub mode: ScanMode,
    pub filter: Option<String>,
    pub member: Option<String>,
    pub format: Format,
}

impl ScanRequest {
    pub fn new(target: ScanTarget, mode: ScanMode) -> Self {
        Self {
            target,
            mode,
            filter: None,
            member: None,
            format: Format::Text,
        }
    }
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_score_vo.rs

```rust
// PURPOSE: FileFormat — value object for output file format enums
use crate::string_value_object;

string_value_object!(FileFormat);

impl FileFormat {
    /// Returns the underlying format name as a string slice.
    pub fn name(&self) -> &str {
        &self.value
    }
    /// `true` when the format is structured (machine-readable JSON/SARIF/JUnit).
    pub fn is_structured(&self) -> bool {
        matches!(self.value.as_str(), "json" | "sarif" | "junit")
    }
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_severity_vo.rs

```rust
// PURPOSE: Severity — re-export from common for backward compatibility
//
// This module exists so dependents can keep using the
// `cli_commands::taxonomy_severity_vo::Severity` import path. The real
// definition lives in `common::taxonomy_severity_vo` and is re-exported
// here to avoid breaking any code that still imports from the legacy path.
//
// New code should import directly from `common::taxonomy_severity_vo`.
/// Re-exported for backward compatibility with legacy import paths.
pub use crate::common::taxonomy_severity_vo::Severity;
```

---

## File: crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs

```rust
// PURPOSE: ICodeAnalysisAggregate — aggregate trait for code-analysis checks (AES301–AES305) and formatting reports
//
// Defines the public API for the code-analysis feature. This is what the
// surface layer (CLI, MCP, TUI) depends on to run quality checks, calculate
// scores, and generate reports.
//
// Unlike other aggregates (IImportRunnerAggregate, INamingRunnerAggregate),
// this one also handles report formatting and score calculation — it's both
// an orchestrator and a presentation boundary.
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use crate::common::taxonomy_common_vo::Score;
use crate::common::taxonomy_path_vo::FilePath;

/// ICodeAnalysisAggregate — aggregate port for code-analysis orchestration.
///
/// Implemented by CodeAnalysisOrchestrator (agent layer).
/// Provides methods for:
///   - Running analysis on a single project or directory
///   - Calculating quality scores from violation results
///   - Checking for CRITICAL severity violations
///   - Formatting results as human-readable reports
///   - Querying active rule configurations
pub trait ICodeAnalysisAggregate: Send + Sync {
    /// Run complete AES analysis on a project root directory.
    fn run_code_analysis(&self, project_root: &FilePath) -> LintResultList;
    /// Run AES analysis on a specific source directory (e.g., crates/, src/).
    fn run_code_analysis_dir(&self, src_dir: &FilePath) -> LintResultList;
    /// Run analysis on an arbitrary path (file or directory).
    fn run_code_analysis_path(&self, path: &FilePath) -> Vec<LintResult>;
    /// Calculate a quality score (0.0–100.0) from violation results.
    fn calc_score(&self, results: &[LintResult]) -> Score;
    /// Check if any CRITICAL violations exist in the results.
    fn check_critical(&self, results: &[LintResult]) -> bool;
    /// Format violations into a human-readable compliance report.
    fn format_report(&self, results: &LintResultList, project_root: &FilePath) -> String;
    /// Return list of currently active (enabled) rule configurations.
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO>;
}
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
pub mod contract_layer_detection_aggregate;
pub mod contract_line_protocol;
pub mod taxonomy_analysis_vo;
pub mod taxonomy_code_analysis_rule_vo;
pub mod taxonomy_import_source_vo;
pub mod taxonomy_operation_error;
pub mod taxonomy_violation_code_analysis_vo;
pub mod utility_bypass;
pub mod utility_column;
pub mod utility_duplication;
pub mod utility_file_reader;
pub mod utility_language_mapper;
pub mod utility_mandatory;
pub mod utility_target;
pub use taxonomy_violation_code_analysis_vo::{AesCodeAnalysisViolation, Language};
```

---

## File: crates/shared/src/common/mod.rs

```rust
// common — truly shared types used by multiple features
pub mod contract_executor_protocol;
pub mod taxonomy_action_vo;
pub mod taxonomy_adapter_list_vo;
pub use utility_file::{
    collect_all_source_files, collect_all_source_files_raw, find_workspace_root, scan_directory,
};
pub mod taxonomy_adapter_error;
pub mod taxonomy_adapter_name_vo;
pub mod taxonomy_byte_count_vo;
pub mod taxonomy_common_error;
pub mod taxonomy_common_vo;
pub mod taxonomy_definition_vo;
pub mod taxonomy_depth_vo;
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
pub mod taxonomy_naming_list_vo;
pub mod taxonomy_package_name_vo;
pub mod taxonomy_parser_error;
pub mod taxonomy_path_utils_vo;
pub mod taxonomy_path_vo;
pub mod taxonomy_paths_vo;
pub mod taxonomy_percentage_vo;
pub mod taxonomy_response_data_vo;
pub mod taxonomy_severity_vo;
pub mod taxonomy_source_vo;
pub mod taxonomy_suffix_vo;
pub mod taxonomy_suggestion_vo;
pub mod taxonomy_threshold_vo;
pub mod utility_command_runner;
pub mod utility_file;
pub mod utility_language_detector;
pub mod utility_layer_detector;
pub mod utility_path_normalization;
pub mod utility_value_object_generator;
pub use utility_signature_parser::{
    extract_python_method_signatures, extract_trait_method_signatures,
    extract_typescript_method_signatures, python_signature_uses_forbidden_primitive,
    signature_uses_forbidden_primitive, typescript_signature_uses_forbidden_primitive,
};
pub mod utility_compliance_score;
pub mod utility_signature_parser;
```

---

## File: crates/shared/src/common/taxonomy_git_vo.rs

```rust
// PURPOSE: GitBranchName — value object for git branch identifiers
//
// This value object wraps a String that represents a Git branch name.
// It is generated via the `string_value_object!` macro which provides
// basic equality, display, and conversion traits out of the box.
use crate::string_value_object;

// GitBranchName is a type-safe wrapper around String, generated by the
// string_value_object! macro. Branch names are used throughout the hook
// and watch systems to identify Git branches without raw string types.
string_value_object!(GitBranchName);
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

## File: crates/shared/src/common/utility_file.rs

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

/// Read file content with generic path.
pub fn read_file_generic<P: AsRef<std::path::Path>>(path: P) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
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
            // Check if parent has workspace markers — if so, keep walking up
            if let Some(parent) = dir.parent() {
                if parent.join("crates").is_dir()
                    || parent.join("packages").is_dir()
                    || parent.join("modules").is_dir()
                {
                    // Don't return yet — parent is the real workspace root
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

## File: crates/shared/src/common/utility_language_detector.rs

```rust
// PURPOSE: Language detection — pure, stateless taxonomy utility.
use crate::common::taxonomy_language_info_vo::LanguageInfo;
use crate::common::taxonomy_language_vo::Language;
use crate::common::taxonomy_path_vo::FilePath;
use crate::taxonomy_source_vo::SourceContentVO;

/// Detect the programming language of a file from its extension.
pub fn detect_language(path: &FilePath) -> Language {
    match path.extension().as_str() {
        "py" => Language::Python,
        "js" | "jsx" | "mjs" | "cjs" => Language::JavaScript,
        "ts" | "tsx" | "mts" | "cts" => Language::TypeScript,
        "rs" => Language::Rust,
        _ => Language::Unknown,
    }
}

/// Whether the file's language is one this linter can process.
pub fn is_lintable(path: &FilePath) -> bool {
    matches!(
        detect_language(path),
        Language::Python | Language::JavaScript | Language::TypeScript | Language::Rust
    )
}

/// Detect language info (pre-computed flags) from a FilePath.
pub fn detect_language_info(path: &FilePath) -> LanguageInfo {
    let lang = detect_language(path);
    flags_from_lang(lang)
}

/// Detect language info (pre-computed flags) from a SourceContentVO.
pub fn detect_language_info_from_source(source: &SourceContentVO) -> LanguageInfo {
    detect_language_info(&source.file_path)
}

fn flags_from_lang(lang: Language) -> LanguageInfo {
    let is_rs = lang == Language::Rust;
    let is_py = lang == Language::Python;
    let is_js = lang == Language::JavaScript || lang == Language::TypeScript;
    LanguageInfo {
        is_rs,
        is_py,
        is_js,
        lang,
    }
}
```

---

## File: crates/shared/src/config-system/contract_config_orchestrator_aggregate.rs

```rust
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_language_vo::ConfigLanguage;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use crate::config_system::taxonomy_source_vo::ConfigResult;
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
    fn load_config_sync(&self, project_root: &str) -> ArchitectureConfig;

    /// Get ignored paths from config (hardcoded defaults + config values).
    fn ignored_paths(&self, project_root: &str) -> Vec<String>;
}
```

---

## File: crates/shared/src/config-system/contract_reader_protocol.rs

```rust
// PURPOSE: IConfigReaderProtocol — protocol trait for reading configuration from external sources

use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_error::ConfigError;
use crate::config_system::taxonomy_config_language_vo::ConfigLanguage;
use crate::config_system::taxonomy_source_vo::ConfigSource;
use async_trait::async_trait;

#[async_trait]
pub trait IConfigReaderProtocol: Send + Sync {
    async fn read_config(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<Option<ConfigSource>, ConfigError>;

    async fn list_config_files(
        &self,
        project_root: &FilePath,
    ) -> Result<Vec<(ConfigLanguage, FilePath)>, ConfigError>;
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
pub mod taxonomy_multi_project_vo;
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
    pub naming: crate::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO,
    #[serde(flatten)]
    pub code_analysis: crate::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO,
    #[serde(flatten)]
    pub role: crate::role_rules::taxonomy_role_rule_vo::RoleRuleVO,
    #[serde(flatten)]
    pub orphan: crate::orphan_detector::taxonomy_orphan_rule_vo::OrphanRuleVO,
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
            naming: NamingConfig::new(Count::new(2)),
            ignored_paths: FilePathList { values: vec![] },
            mandatory_class_definition: BooleanVO::new(false),
        }
    }
}
```

---

## File: crates/shared/src/external-lint/contract_external_lint_aggregate.rs

```rust
// PURPOSE: IExternalLintAggregate — contract for running external linter adapters
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait IExternalLintAggregate: Send + Sync {
    async fn scan_all(&self, path: &FilePath) -> LintResultList;
    fn adapter_names(&self) -> Vec<String>;
}
```

---

## File: crates/shared/src/external-lint/mod.rs

```rust
// external-lint — taxonomy types for adapter utilities
pub mod contract_external_lint_aggregate;
pub mod contract_external_lint_executor_protocol;
pub mod contract_external_lint_language_detector_protocol;
pub mod contract_external_lint_selector_protocol;
pub mod contract_external_lint_utility_protocol;
pub mod utility_external_lint;
pub mod utility_external_lint_io;
```

---

## File: crates/shared/src/file-watch/contract_watch_aggregate.rs

```rust
// PURPOSE: IWatchAggregate — contract trait for watch operations used by surfaces
use crate::file_watch::taxonomy_watch_config_vo::WatchConfig;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

/// Aggregate that runs the file-watch loop.
///
/// Implementations create a file-system watcher, process events through
/// [`IChangeAnalyzerProtocol`], and trigger re-lints when relevant files
/// change. The `running` flag signals when to shut down.
pub trait IWatchAggregate: Send + Sync {
    fn run(&self, config: WatchConfig, running: Arc<AtomicBool>) -> std::process::ExitCode;
}
```

---

## File: crates/shared/src/file-watch/mod.rs

```rust
// file-watch — taxonomy and contract types
pub mod contract_change_analyzer_protocol;
pub mod contract_provider_protocol;
pub mod contract_watch_aggregate;
pub mod taxonomy_diff_result_vo;
pub mod taxonomy_service_error;
pub mod taxonomy_watch_config_vo;
pub mod taxonomy_watch_event_vo;
```

---

## File: crates/shared/src/file-watch/taxonomy_watch_config_vo.rs

```rust
// PURPOSE: WatchConfig — value object for file watch configuration parameters
use crate::common::taxonomy_path_vo::FilePath;

pub struct WatchConfig {
    pub path: FilePath,
    pub recursive: bool,
    pub debounce_ms: u64,
    pub ignore_patterns: Vec<String>,
}

impl WatchConfig {
    pub fn from_path(path: String) -> Self {
        Self {
            path: FilePath::new(path).unwrap_or_default(),
            recursive: true,
            debounce_ms: 500,
            ignore_patterns: vec![
                ".git".to_string(),
                "node_modules".to_string(),
                "__pycache__".to_string(),
                "target".to_string(),
                ".venv".to_string(),
                "dist".to_string(),
                "build".to_string(),
            ],
        }
    }
}
```

---

## File: crates/shared/src/git-hooks/contract_git_hooks_aggregate.rs

```rust
// PURPOSE: GitHooksAggregate — unified aggregate trait for git hooks orchestration
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::git_hooks::contract_diff_protocol::IDiffProtocol;
use crate::git_hooks::contract_hook_protocol::IHookProtocol;
use async_trait::async_trait;

#[async_trait]
pub trait GitHooksAggregate: Send + Sync {
    /// Access to diff protocol (read operations)
    fn diff_protocol(&self) -> &dyn IDiffProtocol;

    /// Access to hook protocol (write/management operations)
    fn hook_protocol(&self) -> &dyn IHookProtocol;

    /// Run full git hooks check on a path
    async fn run_git_hooks_check(&self, path: &FilePath) -> LintResultList {
        self.diff_protocol().run_git_diff_check(path).await
    }

    /// Install pre-commit hook
    async fn install_hook(
        &self,
        executable_path: &FilePath,
    ) -> Result<
        crate::mcp_server::taxonomy_job_vo::SuccessStatus,
        crate::git_hooks::taxonomy_hook_error::GitHookError,
    > {
        self.hook_protocol()
            .install_pre_commit(executable_path)
            .await
    }

    /// Uninstall pre-commit hook
    async fn uninstall_hook(
        &self,
    ) -> Result<
        crate::mcp_server::taxonomy_job_vo::SuccessStatus,
        crate::git_hooks::taxonomy_hook_error::GitHookError,
    > {
        self.hook_protocol().uninstall_pre_commit().await
    }
}
```

---

## File: crates/shared/src/git-hooks/mod.rs

```rust
pub mod contract_diff_protocol;
pub mod contract_git_hooks_aggregate;
pub mod contract_hook_protocol;
pub mod contract_manager_protocol;
pub mod contract_orchestrator_aggregate;
pub mod taxonomy_git_diff_data_vo;
pub mod taxonomy_hook_error;
pub mod utility_git_io;
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

/// IImportRunnerAggregate — aggregate port for import-rules orchestration.
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
pub mod taxonomy_import_constant;
pub mod taxonomy_import_rule_vo;
pub mod taxonomy_language_vo;
pub mod taxonomy_violation_import_vo;
pub mod utility_cycle_detector;
pub mod utility_dummy_detector;
pub mod utility_import_module_parser;
pub mod utility_import_resolver;
pub mod utility_import_symbol_extractor;
pub mod utility_path_normalizer;

pub use taxonomy_dependency_edge_vo::DependencyEdge;
pub use taxonomy_language_vo::LanguageVO;
pub use taxonomy_violation_import_vo::AesImportViolation;
```

---

## File: crates/shared/src/naming-rules/contract_naming_runner_aggregate.rs

```rust
// PURPOSE: INamingRunnerAggregate — contract for naming-rules feature orchestrator
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_adapter_error::ScanError;
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait INamingRunnerAggregate: Send + Sync {
    async fn run_audit(&self, target: &FilePath) -> Result<Vec<LintResult>, ScanError>;
    fn name(&self) -> &str;
}
```

---

## File: crates/shared/src/naming-rules/mod.rs

```rust
pub mod contract_naming_checker_protocol;
pub mod contract_naming_runner_aggregate;
pub mod taxonomy_naming_constant;
pub mod taxonomy_naming_rule_vo;
pub mod taxonomy_naming_violation_vo;
pub mod utility_naming;
pub mod utility_naming_checker;
pub mod utility_naming_filesystem;
pub use taxonomy_naming_violation_vo::NamingViolation;
```

---

## File: crates/shared/src/orphan-detector/contract_orphan_aggregate.rs

```rust
// PURPOSE: IOrphanAggregate — aggregate trait for orphan detection (AES308)
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use std::collections::HashSet;

/// Aggregate that detects orphan (unreferenced) files in a project.
///
/// AES308 requires that every source file be reachable from at least one
/// entry point. This aggregate builds a dependency graph, identifies
/// orphan entry points, and reports violations.
pub trait IOrphanAggregate: Send + Sync {
    fn build_orphan_graph_context(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext;
    fn identify_orphan_entry_points(&self, files: &[String]) -> HashSet<String>;
    fn check_orphans(&self, files: &[String], root_dir: &str) -> Vec<LintResult>;
}
```

---

## File: crates/shared/src/orphan-detector/mod.rs

```rust
pub mod contract_orphan_aggregate;
pub mod contract_orphan_graph_resolver_protocol;
pub mod contract_orphan_protocol;
pub mod taxonomy_orphan_contract_vo;
pub mod taxonomy_orphan_rule_vo;
pub mod taxonomy_violation_orphan_vo;
pub mod utility_file_cache;
pub mod utility_orphan;
pub mod utility_orphan_filename;
pub mod utility_orphan_io;
pub mod utility_orphan_path;
pub mod utility_workspace;
pub use taxonomy_orphan_contract_vo::{OrphanEntryPatternListVO, OrphanFileListVO};
pub use taxonomy_violation_orphan_vo::AesOrphanViolation;
```

---

## File: crates/shared/src/project-setup/contract_maintenance_aggregate.rs

```rust
// PURPOSE: Aggregate: MaintenanceCommandsAggregate trait — contract for maintenance operations (stats, doctor, clean, update, cancel)
use crate::common::taxonomy_path_vo::FilePath;
use crate::mcp_server::taxonomy_action_vo::JobId;
use crate::project_setup::taxonomy_doctor_vo::{
    DependencyReport, DoctorResultVO, SecurityScanReport, ToolchainDiagnostics,
};
use crate::project_setup::taxonomy_stats_vo::MaintenanceStatsVO;
use async_trait::async_trait;

#[async_trait]
pub trait MaintenanceCommandsAggregate: Send + Sync {
    async fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO;
    async fn clean(&self);
    async fn update(&self);
    async fn doctor(&self) -> DoctorResultVO;
    async fn cancel(&self, job_id: JobId);
    async fn diagnose_toolchain(&self) -> ToolchainDiagnostics;
    async fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport;
    async fn run_dependency_report(
        &self,
        project_path: &FilePath,
    ) -> Result<DependencyReport, String>;
}
```

---

## File: crates/shared/src/project-setup/contract_setup_aggregate.rs

```rust
// PURPOSE: SetupAggregate — aggregate trait for project setup orchestration
use crate::cli_commands::taxonomy_protocol_vo::TransportProtocol;
use crate::cli_commands::taxonomy_protocol_vo::TransportUrlVO;
use crate::common::taxonomy_path_vo::DirectoryPath;
use crate::mcp_server::taxonomy_job_vo::EnvContentVO;
use crate::mcp_server::taxonomy_job_vo::McpConfigVO;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;
use crate::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use crate::project_setup::taxonomy_setup_contract_vo::{
    CreateConfigDirResult, ProjectLanguageVO, ProjectLanguagesVO, WriteConfigResult,
};

pub type SetupMgmtProtocol = Box<dyn ISetupManagementProtocol>;

#[async_trait::async_trait]
pub trait SetupManagementAggregate: Send + Sync {
    fn check_http(&self, url: &TransportUrlVO) -> SuccessStatus;
    fn generate_env(&self, transport: &TransportProtocol, home: &DirectoryPath) -> EnvContentVO;
    fn generate_mcp_config(&self, transport: &TransportProtocol) -> McpConfigVO;
    fn mcp_config_claude(&self, transport: &TransportProtocol) -> McpConfigVO;
    fn mcp_config_hermes(&self, transport: &TransportProtocol) -> McpConfigVO;
    fn mcp_config_vscode(&self, transport: &TransportProtocol) -> McpConfigVO;
    async fn install_python_adapters(&self) -> SuccessStatus;
    async fn install_javascript_adapters(&self, sudo: bool) -> SuccessStatus;
    fn detect_language(&self) -> ProjectLanguageVO;
    fn detect_languages(&self) -> ProjectLanguagesVO;
    fn get_config_template(&self, language: &str) -> &'static str;
    fn write_config_file(&self, filename: &str, content: &str) -> WriteConfigResult;
    fn create_global_config_dir(&self) -> CreateConfigDirResult;
    fn file_exists(&self, path: &str) -> bool;
}
```

---

## File: crates/shared/src/project-setup/mod.rs

```rust
pub mod contract_maintenance_aggregate;
pub mod contract_maintenance_protocol;
pub mod contract_setup_aggregate;
pub mod contract_setup_protocol;
pub mod contract_tool_executor_protocol;
pub mod taxonomy_doctor_vo;
pub mod taxonomy_language_vo;
pub mod taxonomy_setup_contract_vo;
pub mod taxonomy_stats_vo;
pub mod utility_filesystem_checker;
pub mod utility_setup_io;

pub use taxonomy_setup_contract_vo::{
    CreateConfigDirResult, McpBinaryNameVO, ProjectLanguageVO, SetupError, WriteConfigResult,
};
```

---

## File: crates/shared/src/role-rules/contract_role_runner_aggregate.rs

```rust
// PURPOSE: IRoleRunnerAggregate — contract for role-rules feature orchestrator
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

use crate::role_rules::taxonomy_layer_names_constant::LAYER_AGENT;
use crate::role_rules::taxonomy_layer_names_vo::LayerNames;
use crate::role_rules::taxonomy_role_rule_vo::RoleRuleVO;
use crate::role_rules::taxonomy_violation_role_vo::AesRoleViolation;

pub fn anchor_taxonomy() {
    let _ = LAYER_AGENT;
}
type _LayerNamesVORef = LayerNames;
type _RoleRuleVORef = RoleRuleVO;
type _AesRoleViolationRef = AesRoleViolation;

#[async_trait]
pub trait IRoleRunnerAggregate: Send + Sync {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
    fn name(&self) -> &str;
}
```

---

## File: crates/shared/src/role-rules/mod.rs

```rust
// role-rules — taxonomy and contract types
pub mod contract_agent_role_protocol;
pub mod contract_capabilities_role_protocol;
pub mod contract_role_aggregate;
pub mod contract_role_protocol;
pub mod contract_role_runner_aggregate;
pub mod contract_surface_role_protocol;
pub mod contract_taxonomy_role_protocol;
pub mod contract_utility_role_protocol;
pub mod taxonomy_layer_names_constant;
pub mod taxonomy_layer_names_vo;
pub mod taxonomy_role_rule_vo;
pub mod taxonomy_violation_role_vo;
pub use taxonomy_violation_role_vo::AesRoleViolation;
```

---

