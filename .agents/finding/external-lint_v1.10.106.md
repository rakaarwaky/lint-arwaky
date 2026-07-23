# Crate: external-lint (v1.10.106)

This document contains the source code for feature crate `external-lint` along with its corresponding and imported definitions from the `shared` crate.

## Problem Statement

The following issues were detected by `lint-arwaky-cli scan`:

```
============================================================
  AES Architecture Compliance Report
============================================================
  Project:
  Violations: 2
  [AES303] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/capabilities_external_lint_adapter.rs - AES305 DEAD_INHERITANCE: Empty struct, class, or trait implementation block detected.
WHY? Empty implements implementation blocks do not add behavior and indicate dead or incomplete code.
FIX: Implement the necessary methods/fields or remove the empty definition block.
  [AES303] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/capabilities_language_detector_adapter.rs - AES305 DEAD_INHERITANCE: Empty struct, class, or trait implementation block detected.
WHY? Empty implements implementation blocks do not add behavior and indicate dead or incomplete code.
FIX: Implement the necessary methods/fields or remove the empty definition block.
```

---

## File List

- [ARCHITECTURE.md](file:///home/raka/mcp-arwaky/lint-arwaky/ARCHITECTURE.md)
- [crates/external-lint/Cargo.toml](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/Cargo.toml)
- [crates/external-lint/FRD.md](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/FRD.md)
- [crates/external-lint/src/agent_external_lint_orchestrator.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/agent_external_lint_orchestrator.rs)
- [crates/external-lint/src/capabilities_external_lint_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/capabilities_external_lint_adapter.rs)
- [crates/external-lint/src/capabilities_external_lint_executor.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/capabilities_external_lint_executor.rs)
- [crates/external-lint/src/capabilities_external_lint_selector.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/capabilities_external_lint_selector.rs)
- [crates/external-lint/src/capabilities_js_eslint_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/capabilities_js_eslint_adapter.rs)
- [crates/external-lint/src/capabilities_js_prettier_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/capabilities_js_prettier_adapter.rs)
- [crates/external-lint/src/capabilities_js_tsc_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/capabilities_js_tsc_adapter.rs)
- [crates/external-lint/src/capabilities_language_detector_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/capabilities_language_detector_adapter.rs)
- [crates/external-lint/src/capabilities_py_bandit_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/capabilities_py_bandit_adapter.rs)
- [crates/external-lint/src/capabilities_py_mypy_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/capabilities_py_mypy_adapter.rs)
- [crates/external-lint/src/capabilities_py_ruff_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/capabilities_py_ruff_adapter.rs)
- [crates/external-lint/src/capabilities_rs_audit_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/capabilities_rs_audit_adapter.rs)
- [crates/external-lint/src/capabilities_rs_clippy_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/capabilities_rs_clippy_adapter.rs)
- [crates/external-lint/src/capabilities_rs_fmt_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/capabilities_rs_fmt_adapter.rs)
- [crates/external-lint/src/capabilities_stdio_client.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/capabilities_stdio_client.rs)
- [crates/external-lint/src/lib.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/lib.rs)
- [crates/external-lint/src/root_external_lint_container.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/root_external_lint_container.rs)
- [crates/shared/src/auto-fix/taxonomy_fix_applied_event.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/taxonomy_fix_applied_event.rs)
- [crates/shared/src/cli-commands/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/mod.rs)
- [crates/shared/src/cli-commands/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_severity_vo.rs)
- [crates/shared/src/code-analysis/contract_adapter_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_adapter_protocol.rs)
- [crates/shared/src/code-analysis/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/mod.rs)
- [crates/shared/src/code-analysis/taxonomy_operation_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_operation_error.rs)
- [crates/shared/src/common/contract_executor_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/contract_executor_protocol.rs)
- [crates/shared/src/common/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/mod.rs)
- [crates/shared/src/common/taxonomy_adapter_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_error.rs)
- [crates/shared/src/common/taxonomy_adapter_list_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_list_vo.rs)
- [crates/shared/src/common/taxonomy_adapter_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_name_vo.rs)
- [crates/shared/src/common/taxonomy_common_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_error.rs)
- [crates/shared/src/common/taxonomy_common_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_vo.rs)
- [crates/shared/src/common/taxonomy_duration_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_duration_vo.rs)
- [crates/shared/src/common/taxonomy_error_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_error_vo.rs)
- [crates/shared/src/common/taxonomy_job_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_job_vo.rs)
- [crates/shared/src/common/taxonomy_lint_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_lint_vo.rs)
- [crates/shared/src/common/taxonomy_message_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_message_vo.rs)
- [crates/shared/src/common/taxonomy_path_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_path_vo.rs)
- [crates/shared/src/common/taxonomy_response_data_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_response_data_vo.rs)
- [crates/shared/src/common/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_severity_vo.rs)
- [crates/shared/src/common/utility_file.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/utility_file.rs)
- [crates/shared/src/common/utility_path_normalization.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/utility_path_normalization.rs)
- [crates/shared/src/external-lint/contract_external_lint_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/contract_external_lint_aggregate.rs)
- [crates/shared/src/external-lint/contract_external_lint_executor_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/contract_external_lint_executor_protocol.rs)
- [crates/shared/src/external-lint/contract_external_lint_language_detector_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/contract_external_lint_language_detector_protocol.rs)
- [crates/shared/src/external-lint/contract_external_lint_selector_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/contract_external_lint_selector_protocol.rs)
- [crates/shared/src/external-lint/contract_external_lint_utility_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/contract_external_lint_utility_protocol.rs)
- [crates/shared/src/external-lint/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/mod.rs)
- [crates/shared/src/external-lint/utility_external_lint.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/utility_external_lint.rs)
- [crates/shared/src/external-lint/utility_external_lint_io.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/utility_external_lint_io.rs)
- [crates/shared/src/mcp-server/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/mcp-server/mod.rs)

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

## File: crates/external-lint/Cargo.toml

```toml
[package]
name = "external_lint-lint-arwaky"
version = "1.10.106"
edition = "2021"
description = "Adapters for external linters — Rust (clippy, rustfmt, cargo-audit), Python (ruff, mypy, bandit), JavaScript/TypeScript (eslint, prettier, tsc). Invoked via stdio."
license = "MIT"
repository = "https://github.com/rakaarwaky/lint-arwaky"
publish = true

[lints]
workspace = true

[dependencies]  # (unchanged)
anyhow.workspace = true
serde.workspace = true
serde_json.workspace = true
async-trait.workspace = true
regex.workspace = true
tracing.workspace = true
rustsec.workspace = true
dirs.workspace = true
futures.workspace = true
tokio.workspace = true
shared.workspace = true
toml.workspace = true

[dev-dependencies]
tokio.workspace = true
```

---

## File: crates/external-lint/FRD.md

```rust
# FRD — external-lint

## Feature Goal

The external-lint crate is an aggregate bridge to external, industry-standard linters and formatters. It coordinates and executes Cargo Clippy, Ruff, Mypy, ESLint, Prettier, and others on Rust, Python, and JS/TS files, normalizes their stdout/JSON reports, and integrates them into the Lint Arwaky compliance report.

## Requirements & Scope

- Supported Linters & Tools
  - Rust Ecosystem
    - `cargo clippy`: Catches idioms, performance bugs, and style issues.
    - `rustfmt`: Verifies codebase-wide formatting guidelines.
    - `cargo-audit`: Audits dependencies listed in `Cargo.lock` for known vulnerabilities.
  - Python Ecosystem
    - `ruff`: Extremely fast linter replacing flake8/autoflake/isort.
    - `mypy`: Performs static type checking on Python source code.
    - `bandit`: Scans Python code for common security vulnerabilities (e.g. SQLi, unsafe imports).
  - JavaScript / TypeScript Ecosystem
    - `eslint`: Checks JS/TS styling and syntax rules.
    - `prettier`: Ensures consistent formatting rules.
    - `tsc`: Checks TypeScript compiler/typing errors.
- Report Normalization
  - Normalize external tool reports (stdout/JSON) into the unified Lint Arwaky format.
  - Map tool-specific severity levels (error, warning, info, refactor) to Lint Arwaky Severity (CRITICAL, HIGH, MEDIUM, LOW).
  - Combine local AES rule violations with external linter violations in a single unified terminal report or MCP response.
- Execution
  - Run subprocesses asynchronously or parallelized where possible to prevent blocking CLI feedback.
  - Safely ignore or warn about missing tools without crashing the run process.

## Success Indicators

- [ ] Tool discovery and fallback — missing tools are safely ignored or warned about without crashing the run.
- [ ] Seamless report unification — AES and external violations combined in a single unified report or MCP response.
- [ ] Error level translation — tool severities are correctly mapped to Lint Arwaky Severity.
- [ ] Performance control — subprocesses run async/parallel to prevent blocking CLI feedback.
```

---

## File: crates/external-lint/src/agent_external_lint_orchestrator.rs

```rust
// PURPOSE: ExternalLintOrchestrator — agent layer, orchestrates external linter adapters
//
// The orchestrator dynamically selects which adapters to run based on the
// languages detected in the project (Rust, Python, JavaScript/TypeScript).
// It performs a file-system scan to detect language usage before running
// any adapters — avoids running rustfmt on Python-only projects.
//
// Adapters are run concurrently via future::join_all. If an adapter's binary
// is not installed, a warning is printed (not an error) — the scan continues
// with the remaining adapters.
//
// Language detection is recursive but skips node_modules, target, .git, and .jj dirs.
use std::collections::HashMap;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

use async_trait::async_trait;
use futures::future;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;

pub struct ExternalLintOrchestrator {
    adapters: HashMap<String, Arc<dyn ILinterAdapterProtocol>>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

#[async_trait]
impl IExternalLintAggregate for ExternalLintOrchestrator {
    async fn scan_all(&self, path: &FilePath) -> LintResultList {
        let mut has_rs = false;
        let mut has_py = false;
        let mut has_js = false;

        fn detect_languages(
            dir: &std::path::Path,
            has_rs: &mut bool,
            has_py: &mut bool,
            has_js: &mut bool,
        ) {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let name = match path.file_name() {
                            Some(n) => n.to_string_lossy(),
                            None => continue,
                        };
                        if !matches!(
                            name.as_ref(),
                            "node_modules" | "target" | ".git" | ".jj" | "Graph-It-Live"
                        ) {
                            detect_languages(&path, has_rs, has_py, has_js);
                        }
                    } else if let Some(ext) = path.extension() {
                        match ext.to_str() {
                            Some("rs") => *has_rs = true,
                            Some("py") => *has_py = true,
                            Some("js" | "ts" | "jsx" | "tsx") => *has_js = true,
                            _ => {}
                        }
                    }
                    if *has_rs && *has_py && *has_js {
                        break;
                    }
                }
            }
        }

        let root_path = std::path::Path::new(&path.value);
        if root_path.is_file() {
            if let Some(ext) = root_path.extension() {
                match ext.to_str() {
                    Some("rs") => has_rs = true,
                    Some("py") => has_py = true,
                    Some("js" | "ts" | "jsx" | "tsx") => has_js = true,
                    _ => {}
                }
            }
        } else {
            detect_languages(root_path, &mut has_rs, &mut has_py, &mut has_js);
        }

        let mut adapter_names = Vec::new();
        if has_rs {
            adapter_names.push("clippy");
            adapter_names.push("rustfmt");
            adapter_names.push("cargo-audit");
        }
        if has_py {
            adapter_names.push("ruff");
            adapter_names.push("mypy");
            adapter_names.push("bandit");
        }
        if has_js {
            adapter_names.push("eslint");
            adapter_names.push("prettier");
            adapter_names.push("tsc");
        }

        let mut futures = Vec::new();
        for name in &adapter_names {
            if let Some(adapter) = self.adapters.get(*name) {
                let adapter: Arc<dyn ILinterAdapterProtocol> = adapter.clone();
                let path_clone = path.clone();
                let name_owned = name.to_string();
                futures.push(async move {
                    match adapter.scan(&path_clone).await {
                        Ok(results) => Ok::<Vec<_>, String>(results.values),
                        Err(e) => {
                            let err_msg = e.to_string();
                            if err_msg.contains("No such file or directory")
                                || err_msg.contains("os error 2")
                            {
                                eprintln!(
                                    "[warn] {} is not installed or not in system PATH. Skipping.",
                                    name_owned
                                );
                            } else {
                                eprintln!("[warn] {} adapter failed: {}", name_owned, err_msg);
                            }
                            Ok(Vec::new())
                        }
                    }
                });
            }
        }

        let results = future::join_all(futures).await;
        let mut all = Vec::new();
        for values in results.into_iter().flatten() {
            all.extend(values);
        }
        LintResultList::new(all)
    }

    fn adapter_names(&self) -> Vec<String> {
        self.adapters.keys().cloned().collect()
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl ExternalLintOrchestrator {
    pub fn new(adapters: HashMap<String, Arc<dyn ILinterAdapterProtocol>>) -> Self {
        Self { adapters }
    }
}
```

---

## File: crates/external-lint/src/capabilities_external_lint_adapter.rs

```rust
use shared::common::contract_executor_protocol::ICommandExecutorProtocol;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_adapter_error::AdapterError;
use shared::common::taxonomy_adapter_error::ScanError;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_error::ErrorMessage;
use shared::common::taxonomy_common_vo::{bool, PatternList};
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::contract_external_lint_utility_protocol::{
    IExternalLintCargoProtocol,
    IExternalLintCommandProtocol,
    IExternalLintJsProtocol,
    IExternalLintLanguageProtocol,
    IExternalLintPathProtocol,
};
use shared::external_lint::utility_external_lint_io as ext_io;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ExternalLintUtilityAdapter;

// ─── Block 2: Protocol Trait Implementations ──────────────

impl IExternalLintPathProtocol for ExternalLintUtilityAdapter {
    fn canonicalize_path(&self, path_str: &str) -> FilePath {
        let p = ext_io::canonicalize_path(path_str);
        FilePath::new(p.to_string_lossy().to_string()).unwrap_or_default()
    }

    fn default_working_dir(&self, path: &FilePath) -> FilePath {
        FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone())
    }
}

impl IExternalLintLanguageProtocol for ExternalLintUtilityAdapter {
    fn has_python_files(&self, path: &FilePath) -> bool {
        let p = std::path::Path::new(&path.value);
        if !ext_io::path_exists(p) {
            return bool::new(p.extension().map(|e| e == "py").unwrap_or(false));
        }
        if ext_io::is_file(p) {
            return bool::new(p.extension().map(|e| e == "py").unwrap_or(false));
        }
        if let Ok(dir) = DirectoryPath::new(path.value.clone()) {
            self.has_py_in_dir(&dir)
        } else {
            bool::new(false)
        }
    }

    fn has_py_in_dir(&self, dir: &DirectoryPath) -> bool {
        ext_io::has_python_files(&dir.value)
    }

    fn is_in_path(&self, executable: &str) -> bool {
        ext_io::is_executable_in_path(executable)
    }
}

impl IExternalLintJsProtocol for ExternalLintUtilityAdapter {
    fn resolve_js_cmd(
        &self,
        executable: &str,
        args: PatternList,
        working_dir: &FilePath,
    ) -> PatternList {
        let wd = std::path::Path::new(&working_dir.value);
        if ext_io::has_local_bin(wd, executable) {
            let local_bin = wd.join("node_modules").join(".bin").join(executable);
            let mut cmd = vec![local_bin.to_string_lossy().to_string()];
            cmd.extend(args.values);
            return PatternList::new(cmd);
        }
        if self.is_in_path(executable).value {
            let mut cmd = vec![executable.to_string()];
            cmd.extend(args.values);
            return PatternList::new(cmd);
        }
        let mut cmd = vec![executable.to_string()];
        cmd.extend(args.values);
        PatternList::new(cmd)
    }

    fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath {
        let path_str = &path.value;
        let abs_path = ext_io::canonicalize_path(path_str);
        let mut current = if ext_io::is_file(&abs_path) {
            abs_path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from("."))
        } else {
            abs_path.clone()
        };
        for _ in 0..10 {
            if ext_io::has_config_file(&current) {
                return FilePath::new(current.to_string_lossy().to_string())
                    .unwrap_or_default();
            }
            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => break,
            }
        }
        FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default()
    }

    async fn js_apply_fix(
        &self,
        executor: &dyn ICommandExecutorProtocol,
        path: &FilePath,
        tool: &str,
        fix_arg: &str,
    ) -> Result<ComplianceStatus, LinterOperationError> {
        let wd = self.resolve_js_working_dir(path);
        let abs_path = self.canonicalize_path(&path.value);
        let cmd = self.resolve_js_cmd(
            tool,
            PatternList::new(vec![abs_path.value, fix_arg.to_string()]),
            &wd,
        );
        let response = self
            .exec_cmd_adapter(
                executor,
                cmd,
                wd,
                Timeout::new(60.0),
                AdapterName::raw(tool),
            )
            .await?;
        Ok(ComplianceStatus::new(response.returncode == 0))
    }
}

impl IExternalLintCargoProtocol for ExternalLintUtilityAdapter {
    fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath {
        let path_str = &path.value;
        if path_str.is_empty() {
            return path.clone();
        }
        if let Some(resolved) = ext_io::has_cargo_toml(path_str) {
            return FilePath::new(resolved).unwrap_or_else(|_| path.clone());
        }
        FilePath::new("nonexistent_directory_for_cargo_toml".to_string()).unwrap_or_default()
    }

    fn resolve_cargo_lock_working_dir(&self, path: &FilePath) -> FilePath {
        let path_str = &path.value;
        if path_str.is_empty() {
            return path.clone();
        }
        if let Some(resolved) = ext_io::has_cargo_lock(path_str) {
            return FilePath::new(resolved).unwrap_or_else(|_| path.clone());
        }
        FilePath::new("nonexistent_directory_for_cargo_lock".to_string()).unwrap_or_default()
    }
}

impl IExternalLintCommandProtocol for ExternalLintUtilityAdapter {
    async fn exec_cmd_scan(
        &self,
        executor: &dyn ICommandExecutorProtocol,
        args: PatternList,
        working_dir: FilePath,
        timeout_secs: Timeout,
        adapter_name: Option<AdapterName>,
        path: &FilePath,
    ) -> Result<ResponseData, LinterOperationError> {
        executor
            .execute_command(args, working_dir, Some(timeout_secs))
            .await
            .map_err(|e| {
                LinterOperationError::Scan(ScanError {
                    path: path.clone(),
                    message: ErrorMessage::new(e.to_string()),
                    error_code: None,
                    adapter_name,
                    cause: None,
                })
            })
    }

    async fn exec_cmd_adapter(
        &self,
        executor: &dyn ICommandExecutorProtocol,
        args: PatternList,
        working_dir: FilePath,
        timeout_secs: Timeout,
        adapter_name: AdapterName,
    ) -> Result<ResponseData, LinterOperationError> {
        executor
            .execute_command(args, working_dir, Some(timeout_secs))
            .await
            .map_err(|e| {
                LinterOperationError::Adapter(AdapterError::new(
                    adapter_name,
                    ErrorMessage::new(e.to_string()),
                ))
            })
    }

    async fn noop_apply_fix(&self) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for ExternalLintUtilityAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ExternalLintUtilityAdapter {
    pub fn new() -> Self {
        Self
    }
}

```

---

## File: crates/external-lint/src/capabilities_external_lint_executor.rs

```rust
// PURPOSE: ExternalLintExecutor — implements IExternalLintExecutorProtocol
// Wraps ICommandExecutorProtocol and adds error mapping for scan/adapter operations.

use std::sync::Arc;

use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::contract_executor_protocol::ICommandExecutorProtocol;
use shared::common::taxonomy_adapter_error::{AdapterError, ScanError};
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_error::ErrorMessage;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::contract_external_lint_executor_protocol::IExternalLintExecutorProtocol;

use shared::external_lint::utility_external_lint::{
    canonicalize_path, resolve_js_cmd, resolve_js_working_dir,
};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ExternalLintExecutor {
    executor: Arc<dyn ICommandExecutorProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait::async_trait]
impl IExternalLintExecutorProtocol for ExternalLintExecutor {
    async fn exec_cmd_scan(
        &self,
        args: Vec<String>,
        working_dir: FilePath,
        timeout_secs: f64,
        adapter_name: Option<AdapterName>,
        path: &FilePath,
    ) -> Result<ResponseData, LinterOperationError> {
        self.executor
            .execute_command(
                PatternList::new(args),
                working_dir,
                Some(Timeout::new(timeout_secs)),
            )
            .await
            .map_err(|e| {
                LinterOperationError::Scan(ScanError {
                    path: path.clone(),
                    message: ErrorMessage::new(e.to_string()),
                    error_code: None,
                    adapter_name,
                    cause: None,
                })
            })
    }

    async fn exec_cmd_adapter(
        &self,
        args: Vec<String>,
        working_dir: FilePath,
        timeout_secs: f64,
        adapter_name: AdapterName,
    ) -> Result<ResponseData, LinterOperationError> {
        self.executor
            .execute_command(
                PatternList::new(args),
                working_dir,
                Some(Timeout::new(timeout_secs)),
            )
            .await
            .map_err(|e| {
                LinterOperationError::Adapter(AdapterError::new(
                    adapter_name,
                    ErrorMessage::new(e.to_string()),
                ))
            })
    }

    async fn js_apply_fix(
        &self,
        path: &FilePath,
        tool: &str,
        fix_arg: &str,
    ) -> Result<ComplianceStatus, LinterOperationError> {
        let wd = resolve_js_working_dir(path);
        let abs_path = canonicalize_path(&path.value);
        let cmd = resolve_js_cmd(tool, vec![abs_path, fix_arg.to_string()], &wd.value);
        let response = self
            .exec_cmd_adapter(cmd, wd, 60.0, AdapterName::raw(tool))
            .await?;
        Ok(ComplianceStatus::new(response.returncode == 0))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl ExternalLintExecutor {
    pub fn new(executor: Arc<dyn ICommandExecutorProtocol>) -> Self {
        Self { executor }
    }
}
```

---

## File: crates/external-lint/src/capabilities_external_lint_selector.rs

```rust
use shared::common::taxonomy_adapter_list_vo::{AdapterName, AdapterNameList};
use shared::common::taxonomy_common_vo::bool;
use shared::external_lint::contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;

// PURPOSE: CapabilitiesExternalLintSelector — selects adapters based on detected languages
//
// Pure business logic: maps language flags to adapter name lists.
// No I/O, no external dependencies.

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CapabilitiesExternalLintSelector {
    rust_adapters: Vec<AdapterName>,
    python_adapters: Vec<AdapterName>,
    js_adapters: Vec<AdapterName>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IExternalLintSelectorProtocol for CapabilitiesExternalLintSelector {
    fn select_adapters(
        &self,
        has_rs: bool,
        has_py: bool,
        has_js: bool,
    ) -> AdapterNameList {
        let mut adapter_names = Vec::new();
        if has_rs.value() {
            for name in self.rust_adapters.iter() {
                adapter_names.push(name.clone());
            }
        }
        if has_py.value() {
            for name in self.python_adapters.iter() {
                adapter_names.push(name.clone());
            }
        }
        if has_js.value() {
            for name in self.js_adapters.iter() {
                adapter_names.push(name.clone());
            }
        }
        AdapterNameList::new(adapter_names)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl CapabilitiesExternalLintSelector {
    pub fn new(
        rust_adapters: Vec<AdapterName>,
        python_adapters: Vec<AdapterName>,
        js_adapters: Vec<AdapterName>,
    ) -> Self {
        Self {
            rust_adapters,
            python_adapters,
            js_adapters,
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(
            vec![
                AdapterName::raw("clippy"),
                AdapterName::raw("rustfmt"),
                AdapterName::raw("cargo-audit"),
            ],
            vec![
                AdapterName::raw("ruff"),
                AdapterName::raw("mypy"),
                AdapterName::raw("bandit"),
            ],
            vec![
                AdapterName::raw("eslint"),
                AdapterName::raw("prettier"),
                AdapterName::raw("tsc"),
            ],
        )
    }
}

```

---

## File: crates/external-lint/src/capabilities_js_eslint_adapter.rs

```rust
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_adapter_error::ScanError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::utility_file;
use shared::external_lint::contract_external_lint_executor_protocol::IExternalLintExecutorProtocol;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
use std::path::Path;
use std::sync::Arc;

use shared::external_lint::utility_external_lint::{
    canonicalize_path, resolve_js_cmd, resolve_js_working_dir as resolve_working_dir,
};

// (No protocol implementation found in this file)

// PURPOSE: ESLintAdapter — ILinterAdapterProtocol implementation for ESLint integration
//
// Executes `npx eslint --format=json` as a subprocess and parses the
// JSON output. ESLint outputs a JSON array of per-file results, each
// containing an array of messages with rule IDs, severity, and location.
//
// Key handling:
//   - Resolves the correct working directory (package.json parent)
//   - Uses npx to find eslint (works for both local and global installs)
//   - Mirrors Ruff adapter's path-fallback logic for consistency
//   - Returns empty results for non-JS/TS files (no error)
//   - Maps ESLint severity (1=warning, 2=error) to AES severity levels

use serde_json::Value;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ESLintAdapter {
    lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait::async_trait]
impl ILinterAdapterProtocol for ESLintAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("eslint")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let path_str = &path.value;
        if utility_file::is_file_generic(Path::new(path_str))
            && !path_str.ends_with(".ts")
            && !path_str.ends_with(".tsx")
            && !path_str.ends_with(".js")
            && !path_str.ends_with(".jsx")
        {
            return Ok(LintResultList::default());
        }

        let wd = resolve_working_dir(path);
        let abs_path = canonicalize_path(path_str);

        let cmd = resolve_js_cmd(
            "eslint",
            vec![abs_path, "--format".to_string(), "json".to_string()],
            &wd.value,
        );

        let response = self
            .lint_executor
            .exec_cmd_scan(cmd, wd.clone(), 60.0, Some(self.name()), path)
            .await?;

        let stdout_str = response.stdout.to_string();
        if stdout_str.trim().is_empty() {
            return Ok(LintResultList::default());
        }

        let parsed: Value = serde_json::from_str(&stdout_str).map_err(|e| {
            LinterOperationError::Scan(ScanError {
                path: path.clone(),
                message: ErrorMessage::new(format!("Failed to parse JSON: {}", e)),
                error_code: None,
                adapter_name: Some(self.name()),
                cause: None,
            })
        })?;

        let mut results = Vec::new();
        if let Some(files) = parsed.as_array() {
            for file_data in files {
                let filename = match file_data["filePath"].as_str() {
                    Some(s) => s.to_string(),
                    None => String::new(),
                };
                let filename_vo =
                    shared::common::utility_path_normalization::resolve_capabilities_path(
                        FilePath::new(filename).unwrap_or(path.clone()),
                        Some(path.clone()),
                    );

                if let Some(messages) = file_data["messages"].as_array() {
                    for msg in messages {
                        let line_num = match msg["line"].as_u64() {
                            Some(v) => v as usize,
                            None => 1,
                        };
                        let col_num = match msg["column"].as_u64() {
                            Some(v) => v as usize,
                            None => 0,
                        };
                        let rule_id = match msg["ruleId"].as_str() {
                            Some(s) => s.to_string(),
                            None => "ESLINT".to_string(),
                        };
                        let message_text = match msg["message"].as_str() {
                            Some(s) => s.to_string(),
                            None => String::new(),
                        };
                        let sev_code = msg["severity"].as_u64().unwrap_or(1);

                        let severity = if sev_code == 2 {
                            Severity::HIGH
                        } else {
                            Severity::MEDIUM
                        };

                        results.push(LintResult {
                            file: filename_vo.clone(),
                            line: LineNumber::new(line_num as i64),
                            column: ColumnNumber::new(col_num as i64),
                            code: ErrorCode::raw(rule_id),
                            message: LintMessage::new(message_text),
                            source: Some(self.name()),
                            severity,
                            enclosing_scope: Default::default(),
                            related_locations: Default::default(),
                        });
                    }
                }
            }
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        self.lint_executor
            .js_apply_fix(path, "eslint", "--fix")
            .await
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl ESLintAdapter {
    pub fn new(lint_executor: Arc<dyn IExternalLintExecutorProtocol>) -> Self {
        Self { lint_executor }
    }
}
```

---

## File: crates/external-lint/src/capabilities_js_prettier_adapter.rs

```rust
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::utility_file;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
use std::path::Path;
use std::sync::Arc;

use shared::external_lint::contract_external_lint_executor_protocol::IExternalLintExecutorProtocol;
use shared::external_lint::utility_external_lint::{
    canonicalize_path, resolve_js_cmd, resolve_js_working_dir as resolve_working_dir,
};

// (No protocol implementation found in this file)

// PURPOSE: PrettierAdapter — ILinterAdapterProtocol implementation for Prettier integration
//
// Runs `prettier --check <path>` on JS/TS files via
// resolve_js_cmd (npx). Only files with .ts/.tsx/.js/.jsx extensions are scanned.
// apply_fix runs `prettier --write <path>` to auto-format.
//
// Key details:
//   - Early-returns empty results for non-JS/TS files
//   - Uses canonical absolute paths for reliable prettier invocation
//   - Detects warnings by checking for "[warn]" in combined stdout+stderr
//   - Reports a single LintResult per file (not per-difference)

// ─── Block 1: Struct Definition ───────────────────────────

pub struct PrettierAdapter {
    lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait::async_trait]
impl ILinterAdapterProtocol for PrettierAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("prettier")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let path_str = &path.value;
        if utility_file::is_file_generic(Path::new(path_str))
            && !path_str.ends_with(".ts")
            && !path_str.ends_with(".tsx")
            && !path_str.ends_with(".js")
            && !path_str.ends_with(".jsx")
        {
            return Ok(LintResultList::default());
        }

        let wd = resolve_working_dir(path);
        let abs_path = canonicalize_path(path_str);

        let cmd = resolve_js_cmd("prettier", vec!["--check".to_string(), abs_path], &wd.value);

        let response = self
            .lint_executor
            .exec_cmd_scan(cmd, wd.clone(), 60.0, Some(self.name()), path)
            .await?;

        let mut results = Vec::new();
        let combined_output = format!("{}{}", response.stdout, response.stderr);

        if combined_output.contains("[warn]") {
            let filename_vo = shared::common::utility_path_normalization::resolve_capabilities_path(
                path.clone(),
                Some(path.clone()),
            );
            results.push(LintResult {
                file: filename_vo,
                line: LineNumber::new(1),
                column: ColumnNumber::new(0),
                code: ErrorCode::raw("formatting"),
                message: LintMessage::new("Code style issues found. Run Prettier to fix."),
                source: Some(self.name()),
                severity: Severity::LOW,
                enclosing_scope: Default::default(),
                related_locations: Default::default(),
            });
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        self.lint_executor
            .js_apply_fix(path, "prettier", "--write")
            .await
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

// (No protocol implementation found in this file)

// (No protocol implementation found in this file)

impl PrettierAdapter {
    pub fn new(lint_executor: Arc<dyn IExternalLintExecutorProtocol>) -> Self {
        Self { lint_executor }
    }
}
```

---

## File: crates/external-lint/src/capabilities_js_tsc_adapter.rs

```rust
use std::sync::OnceLock;

use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::utility_file;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
use std::path::Path;
use std::sync::Arc;

use shared::external_lint::contract_external_lint_executor_protocol::IExternalLintExecutorProtocol;
use shared::external_lint::utility_external_lint::{
    canonicalize_path, noop_apply_fix, resolve_js_cmd,
    resolve_js_working_dir as resolve_working_dir,
};

// (No protocol implementation found in this file)

// PURPOSE: TSCAdapter — ILinterAdapterProtocol implementation for TypeScript compiler integration
//
// Runs `tsc --noEmit --pretty false <path>` to type-check TypeScript files.
// Parses compiler output with two regex patterns (parenthesized format and
// colon-delimited format). apply_fix always returns false (tsc is a compiler).
//
// Key details:
//   - `--noEmit` prevents output files, only runs type checking
//   - `--pretty false` ensures machine-parseable output
//   - Two regex patterns handle different tsc output formats across versions
//   - Skips files that don't end in .ts or .tsx
//   - All tsc errors are reported as HIGH severity

use regex::Regex;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct TSCAdapter {
    lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait::async_trait]
impl ILinterAdapterProtocol for TSCAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("tsc")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let path_str = &path.value;
        if utility_file::is_file_generic(Path::new(path_str))
            && !path_str.ends_with(".ts")
            && !path_str.ends_with(".tsx")
        {
            return Ok(LintResultList::default());
        }

        let wd = resolve_working_dir(path);
        let abs_path = canonicalize_path(path_str);

        let mut args = vec![
            "--noEmit".to_string(),
            "--pretty".to_string(),
            "false".to_string(),
        ];
        if abs_path != "." && abs_path != "./" {
            args.push(abs_path);
        }

        let cmd = resolve_js_cmd("tsc", args, &wd.value);

        let response = self
            .lint_executor
            .exec_cmd_scan(cmd, wd.clone(), 60.0, Some(self.name()), path)
            .await?;

        let output = format!("{}{}", response.stdout, response.stderr);
        let mut results = Vec::new();

        let pattern1 = match tsc_pattern1() {
            Some(r) => r,
            None => return Ok(LintResultList::new(vec![])),
        };
        let pattern2 = match tsc_pattern2() {
            Some(r) => r,
            None => return Ok(LintResultList::new(vec![])),
        };

        for line in output.lines() {
            let line = line.trim();
            if let Some(caps) = pattern1.captures(line).or_else(|| pattern2.captures(line)) {
                let filename = match caps.get(1) {
                    Some(m) => m.as_str().to_string(),
                    None => String::new(),
                };
                let line_num = caps
                    .get(2)
                    .and_then(|m| m.as_str().parse::<usize>().ok())
                    .unwrap_or(1);
                let col_num = caps
                    .get(3)
                    .and_then(|m| m.as_str().parse::<usize>().ok())
                    .unwrap_or_default();
                let code = match caps.get(4) {
                    Some(m) => m.as_str().to_string(),
                    None => String::new(),
                };
                let msg = match caps.get(5) {
                    Some(m) => m.as_str().to_string(),
                    None => String::new(),
                };

                let filename_vo =
                    shared::common::utility_path_normalization::resolve_capabilities_path(
                        FilePath::new(filename).unwrap_or(path.clone()),
                        Some(path.clone()),
                    );

                results.push(LintResult {
                    file: filename_vo,
                    line: LineNumber::new(line_num as i64),
                    column: ColumnNumber::new(col_num as i64),
                    code: ErrorCode::raw(&code),
                    message: LintMessage::new(msg),
                    source: Some(self.name()),
                    severity: Severity::HIGH,
                    enclosing_scope: Default::default(),
                    related_locations: Default::default(),
                });
            }
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        noop_apply_fix().await
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

// (No protocol implementation found in this file)

// (No protocol implementation found in this file)

fn tsc_pattern1() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^([^(]+)\((\d+),(\d+)\):\s+error\s+(TS\d+):\s+(.*)$").ok())
        .as_ref()
}

fn tsc_pattern2() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^([^:]+):(\d+):(\d+)\s+-\s+error\s+(TS\d+):\s+(.*)$").ok())
        .as_ref()
}

impl TSCAdapter {
    pub fn new(lint_executor: Arc<dyn IExternalLintExecutorProtocol>) -> Self {
        Self { lint_executor }
    }
}
```

---

## File: crates/external-lint/src/capabilities_language_detector_adapter.rs

```rust

use std::path::Path;

use async_trait::async_trait;
use shared::common::taxonomy_common_vo::bool;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_language_detector_protocol::{
    DetectedLanguages, IExternalLintLanguageDetectorProtocol,
};
use shared::external_lint::utility_external_lint_io as ext_io;

// PURPOSE: ExternalLintLanguageDetectorAdapter — IExternalLintLanguageDetectorProtocol implementation
//
// Scans a directory tree to detect which programming languages are present.
// Skips node_modules, target, .git, .jj directories.

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ExternalLintLanguageDetectorAdapter;

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl IExternalLintLanguageDetectorProtocol for ExternalLintLanguageDetectorAdapter {
    async fn detect_languages(&self, path: &FilePath) -> DetectedLanguages {
        let mut has_rs = false;
        let mut has_py = false;
        let mut has_js = false;

        let root_path = Path::new(&path.value);
        if ext_io::is_file(root_path) {
            Self::detect_from_file(root_path, &mut has_rs, &mut has_py, &mut has_js);
        } else {
            Self::detect_in_dir(root_path, &mut has_rs, &mut has_py, &mut has_js);
        }

        DetectedLanguages {
            has_rs: bool::new(has_rs),
            has_py: bool::new(has_py),
            has_js: bool::new(has_js),
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

const SKIP_DIRS: &[&str] = &["node_modules", "target", ".git", ".jj", "Graph-It-Live"];

impl ExternalLintLanguageDetectorAdapter {
    pub fn new() -> Self {
        Self
    }

    fn detect_in_dir(dir: &Path, has_rs: &mut bool, has_py: &mut bool, has_js: &mut bool) {
        let entries = ext_io::scan_directory(dir);
        for (name, path_str, is_dir_entry) in entries {
            if is_dir_entry {
                if !SKIP_DIRS.contains(&name.as_str()) {
                    Self::detect_in_dir(Path::new(&path_str), has_rs, has_py, has_js);
                }
            } else if let Some(ext) = Path::new(&path_str).extension() {
                match ext.to_str() {
                    Some("rs") => *has_rs = true,
                    Some("py") => *has_py = true,
                    Some("js" | "ts" | "jsx" | "tsx") => *has_js = true,
                    _ => {}
                }
            }
            if *has_rs && *has_py && *has_js {
                break;
            }
        }
    }

    fn detect_from_file(path: &Path, has_rs: &mut bool, has_py: &mut bool, has_js: &mut bool) {
        if let Some(ext) = path.extension() {
            match ext.to_str() {
                Some("rs") => *has_rs = true,
                Some("py") => *has_py = true,
                Some("js" | "ts" | "jsx" | "tsx") => *has_js = true,
                _ => {}
            }
        }
    }
}

impl Default for ExternalLintLanguageDetectorAdapter {
    fn default() -> Self {
        Self::new()
    }
}

```

---

## File: crates/external-lint/src/capabilities_py_bandit_adapter.rs

```rust
use serde_json::Value;
use std::sync::Arc;

use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;

use shared::external_lint::contract_external_lint_executor_protocol::IExternalLintExecutorProtocol;
use shared::external_lint::utility_external_lint::{
    default_working_dir, has_python_files, noop_apply_fix,
};

// PURPOSE: PyBanditAdapter — ILinterAdapterProtocol implementation for Bandit security scanner integration
//
// Runs `bandit -r <path> --format json --exit-zero` to scan Python files for
// security vulnerabilities. Parses JSON output to extract findings (filename,
// line_range, test_id, issue_text, severity).
//
// Key details:
//   - `--exit-zero` ensures bandit always exits 0 regardless of findings
//   - JSON output avoids fragile regex parsing
//   - Severity is directly mapped: HIGH→HIGH, MEDIUM→MEDIUM, LOW→LOW
//   - apply_fix always returns false (Bandit is a scanner, not a fixer)

use async_trait::async_trait;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct BanditAdapter {
    lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
    bin_path: Option<FilePath>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl ILinterAdapterProtocol for BanditAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("bandit")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        // Skip if no Python files exist in the target path
        if !has_python_files(path) {
            return Ok(LintResultList::new(vec![]));
        }

        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            "-r".to_string(),
            path.value.clone(),
            "--format".to_string(),
            "json".to_string(),
            "--exit-zero".to_string(),
        ];
        let working_dir = default_working_dir(path);

        let response = self
            .lint_executor
            .exec_cmd_adapter(cmd, working_dir, 120.0, self.name())
            .await?;

        let stdout = &response.stdout;
        let parsed: Value = match serde_json::from_str(stdout) {
            Ok(v) => v,
            Err(_) => Value::Object(serde_json::Map::new()),
        };
        let findings = match parsed.get("results").and_then(|v| v.as_array()) {
            Some(arr) => arr.clone(),
            None => Vec::new(),
        };
        let mut results = Vec::new();

        for f in findings {
            let filename = f
                .get("filename")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let line_number = f
                .get("line_number")
                .and_then(|v| v.as_i64())
                .unwrap_or_default();
            let line_range = f
                .get("line_range")
                .and_then(|v| v.as_array())
                .and_then(|a| a.first())
                .and_then(|v| v.as_i64())
                .unwrap_or_default();
            let test_id = f.get("test_id").and_then(|v| v.as_str()).unwrap_or("B000");
            let issue_text = f
                .get("issue_text")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let issue_severity = f
                .get("issue_severity")
                .and_then(|v| v.as_str())
                .unwrap_or("MEDIUM");

            let resolved = shared::common::utility_path_normalization::resolve_capabilities_path(
                match FilePath::new(filename.to_string()) {
                    Ok(fp) => fp,
                    Err(_) => path.clone(),
                },
                Some(path.clone()),
            );

            results.push(LintResult {
                file: resolved,
                line: LineNumber::new(line_number),
                column: ColumnNumber::new(line_range),
                code: ErrorCode::raw(test_id),
                message: LintMessage::new(issue_text),
                source: Some(self.name()),
                severity: self.map_severity(issue_severity),
                enclosing_scope: None,
                related_locations: LocationList::new(),
            });
        }
        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        noop_apply_fix().await
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl BanditAdapter {
    pub fn new(
        lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
        bin_path: Option<FilePath>,
    ) -> Self {
        Self {
            lint_executor,
            bin_path,
        }
    }

    fn resolve_executable(&self) -> String {
        match self.bin_path.as_ref() {
            Some(p) => p.value.clone(),
            None => "bandit".to_string(),
        }
    }

    fn map_severity(&self, severity: &str) -> Severity {
        match severity {
            "HIGH" => Severity::HIGH,
            "MEDIUM" => Severity::MEDIUM,
            "LOW" => Severity::LOW,
            _ => Severity::MEDIUM,
        }
    }
}
```

---

## File: crates/external-lint/src/capabilities_py_mypy_adapter.rs

```rust
use regex::Regex;
use std::sync::Arc;
use std::sync::OnceLock;

use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;

use shared::external_lint::contract_external_lint_executor_protocol::IExternalLintExecutorProtocol;
use shared::external_lint::utility_external_lint::{
    default_working_dir, has_python_files, noop_apply_fix,
};

// PURPOSE: PyMypyAdapter — ILinterAdapterProtocol implementation for MyPy type checker integration
//
// Runs `mypy <path>` on Python files and parses its structured output with
// two regex patterns (with/without column numbers). Severity is mapped
// heuristically: notes → LOW, warnings → MEDIUM, errors → HIGH,
// syntax/parse errors → CRITICAL.
//
// Key details:
//   - `--no-error-summary` avoids summary lines, keeping output parseable
//   - `--pretty false` ensures machine-parseable single-line output
//   - Falls back to column-less regex if column-full regex doesn't match
//   - apply_fix always returns false (mypy is a type checker, not a formatter)

use async_trait::async_trait;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct MyPyAdapter {
    lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
    bin_path: Option<FilePath>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl ILinterAdapterProtocol for MyPyAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("mypy")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        // Skip if no Python files exist in the target path
        if !has_python_files(path) {
            return Ok(LintResultList::new(vec![]));
        }

        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            path.value.clone(),
            "--no-error-summary".to_string(),
            "--pretty".to_string(),
            "false".to_string(),
        ];
        let working_dir = default_working_dir(path);

        let response = self
            .lint_executor
            .exec_cmd_adapter(cmd, working_dir, 120.0, self.name())
            .await?;

        let stdout = &response.stdout;
        let re = match mypy_re_with_col() {
            Some(r) => r,
            None => match mypy_re_without_col() {
                Some(r) => r,
                None => return Ok(LintResultList::new(vec![])),
            },
        };
        let re_simple = match mypy_re_without_col() {
            Some(r) => r,
            None => return Ok(LintResultList::new(vec![])),
        };
        let mut results = Vec::new();

        for line in stdout.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some(caps) = re.captures(line) {
                let filename = match caps.get(1) {
                    Some(m) => m.as_str(),
                    None => "",
                };
                let line_number: i64 = caps
                    .get(2)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or_default();
                let column: i64 = caps
                    .get(3)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or_default();
                let msg_type = match caps.get(4) {
                    Some(m) => m.as_str(),
                    None => "error",
                };
                let message = match caps.get(5) {
                    Some(m) => m.as_str(),
                    None => "",
                };
                let code = match caps.get(6) {
                    Some(m) => m.as_str(),
                    None => "",
                };

                let resolved =
                    shared::common::utility_path_normalization::resolve_capabilities_path(
                        match FilePath::new(filename.to_string()) {
                            Ok(fp) => fp,
                            Err(_) => path.clone(),
                        },
                        Some(path.clone()),
                    );

                results.push(LintResult {
                    file: resolved,
                    line: LineNumber::new(line_number),
                    column: ColumnNumber::new(column),
                    code: ErrorCode::raw(code),
                    message: LintMessage::new(message),
                    source: Some(self.name()),
                    severity: Self::map_severity(msg_type, message),
                    enclosing_scope: None,
                    related_locations: LocationList::new(),
                });
            } else if let Some(caps) = re_simple.captures(line) {
                let filename = match caps.get(1) {
                    Some(m) => m.as_str(),
                    None => "",
                };
                let line_number: i64 = caps
                    .get(2)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or_default();
                let msg_type = match caps.get(3) {
                    Some(m) => m.as_str(),
                    None => "error",
                };
                let message = match caps.get(4) {
                    Some(m) => m.as_str(),
                    None => "",
                };
                let code = match caps.get(5) {
                    Some(m) => m.as_str(),
                    None => "",
                };

                let resolved =
                    shared::common::utility_path_normalization::resolve_capabilities_path(
                        match FilePath::new(filename.to_string()) {
                            Ok(fp) => fp,
                            Err(_) => path.clone(),
                        },
                        Some(path.clone()),
                    );

                results.push(LintResult {
                    file: resolved,
                    line: LineNumber::new(line_number),
                    column: ColumnNumber::new(0),
                    code: ErrorCode::raw(code),
                    message: LintMessage::new(message),
                    source: Some(self.name()),
                    severity: Self::map_severity(msg_type, message),
                    enclosing_scope: None,
                    related_locations: LocationList::new(),
                });
            }
        }
        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        noop_apply_fix().await
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

fn mypy_re_with_col() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^([^:]+):(\d+):(\d+):\s+(\w+):\s+(.+?)\s+\[([\w-]+)\]$").ok())
        .as_ref()
}

fn mypy_re_without_col() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^([^:]+):(\d+):\s+(\w+):\s+(.+?)\s+\[([\w-]+)\]$").ok())
        .as_ref()
}

impl MyPyAdapter {
    pub fn new(
        lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
        bin_path: Option<FilePath>,
    ) -> Self {
        Self {
            lint_executor,
            bin_path,
        }
    }

    fn resolve_executable(&self) -> String {
        match self.bin_path.as_ref() {
            Some(p) => p.value.clone(),
            None => "mypy".to_string(),
        }
    }

    fn map_severity(msg_type: &str, msg: &str) -> Severity {
        let m = msg.to_lowercase();
        if msg_type == "note" {
            return Severity::LOW;
        }
        if m.contains("syntax") || m.contains("parse") {
            return Severity::CRITICAL;
        }
        if msg_type == "warning" {
            return Severity::MEDIUM;
        }
        Severity::HIGH
    }
}
```

---

## File: crates/external-lint/src/capabilities_py_ruff_adapter.rs

```rust
use serde_json::Value;
use std::sync::Arc;

use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_adapter_error::AdapterError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;

use shared::external_lint::contract_external_lint_executor_protocol::IExternalLintExecutorProtocol;
use shared::external_lint::utility_external_lint::{default_working_dir, has_python_files};

// PURPOSE: PyRuffAdapter — ILinterAdapterProtocol implementation for Ruff linter integration
//
// Executes `ruff check --output-format=json` as a subprocess and parses
// the JSON output. Ruff outputs a JSON array of diagnostics with file paths,
// line numbers, severity levels, and rule codes.
//
// Key handling:
//   - Falls back to parent directory if target is a file (Ruff requires a directory)
//   - Searches for pyproject.toml to determine the correct working directory
//   - Maps Ruff severity levels (error/warning/info) to AES severity
//   - Converts relative Ruff paths to absolute project paths

use async_trait::async_trait;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct RuffAdapter {
    lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
    bin_path: Option<FilePath>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl ILinterAdapterProtocol for RuffAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("ruff")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        // Skip if no Python files exist in the target path
        if !has_python_files(path) {
            return Ok(LintResultList::new(vec![]));
        }

        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            "check".to_string(),
            path.value.clone(),
            "--output-format=json".to_string(),
            "--exit-zero".to_string(),
            "--no-cache".to_string(),
        ];
        let working_dir = default_working_dir(path);

        let response = self
            .lint_executor
            .exec_cmd_adapter(cmd, working_dir, 60.0, self.name())
            .await?;

        let stdout = &response.stdout;
        // Empty output — tool found nothing to report (or no applicable files)
        if stdout.trim().is_empty() {
            return Ok(LintResultList::new(vec![]));
        }
        let findings: Vec<Value> = match serde_json::from_str(stdout) {
            Ok(v) => v,
            Err(e) => {
                return Err(LinterOperationError::Adapter(AdapterError::new(
                    self.name(),
                    ErrorMessage::new(format!(
                        "Failed to parse ruff JSON output: {}. Output was: {:?}",
                        e,
                        stdout.chars().take(200).collect::<String>()
                    )),
                )));
            }
        };
        let mut results = Vec::new();

        for f in findings {
            let filename = f
                .get("filename")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let row = f
                .get("location")
                .and_then(|l| l.get("row"))
                .and_then(|v| v.as_i64())
                .unwrap_or_default();
            let col = f
                .get("location")
                .and_then(|l| l.get("column"))
                .and_then(|v| v.as_i64())
                .unwrap_or_default();
            let code = f.get("code").and_then(|v| v.as_str()).unwrap_or("UNKNOWN");
            let message = f
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let severity_str = f
                .get("severity")
                .and_then(|v| v.as_str())
                .unwrap_or_default();

            let resolved = shared::common::utility_path_normalization::resolve_capabilities_path(
                match FilePath::new(filename.to_string()) {
                    Ok(fp) => fp,
                    Err(_) => path.clone(),
                },
                Some(path.clone()),
            );

            results.push(LintResult {
                file: resolved,
                line: LineNumber::new(row),
                column: ColumnNumber::new(col),
                code: ErrorCode::raw(code),
                message: LintMessage::new(message),
                source: Some(self.name()),
                severity: self.map_severity(severity_str, code),
                enclosing_scope: None,
                related_locations: LocationList::new(),
            });
        }
        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            "check".to_string(),
            path.value.clone(),
            "--fix".to_string(),
            "--exit-zero".to_string(),
        ];
        let working_dir = default_working_dir(path);

        let _ = self
            .lint_executor
            .exec_cmd_adapter(cmd, working_dir, 60.0, self.name())
            .await?;
        Ok(ComplianceStatus::new(true))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl RuffAdapter {
    pub fn new(
        lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
        bin_path: Option<FilePath>,
    ) -> Self {
        Self {
            lint_executor,
            bin_path,
        }
    }

    fn resolve_executable(&self) -> String {
        match self.bin_path.as_ref() {
            Some(p) => p.value.clone(),
            None => "ruff".to_string(),
        }
    }

    fn map_severity(&self, severity: &str, _code: &str) -> Severity {
        match severity {
            "error" => Severity::HIGH,
            "warning" => Severity::MEDIUM,
            "info" => Severity::LOW,
            _ => Severity::MEDIUM,
        }
    }
}
```

---

## File: crates/external-lint/src/capabilities_rs_audit_adapter.rs

```rust
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::utility_file;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
use tracing::debug;

use shared::external_lint::utility_external_lint::resolve_cargo_lock_working_dir;

// PURPOSE: RsAuditAdapter — ILinterAdapterProtocol implementation for cargo-audit security scanning
//
// Uses the `rustsec` crate directly (not subprocess) to parse Cargo.lock and
// check against the RustSec Advisory Database. Reports vulnerabilities as
// LintResults with CVE/RUSTSEC IDs as error codes.
//
// Key details:
//   - Finds Cargo.lock via resolve_cargo_lock_working_dir (walks up from path)
//   - Uses local advisory DB from ~/.cargo/advisory-db, or fetches if missing
//   - No subprocess overhead — uses rustsec library API directly
//   - CVSS severity is mapped: critical→CRITICAL, high→HIGH, medium→MEDIUM, else→LOW
//   - apply_fix returns true (cargo-audit has no fix command; affected packages
//     must be updated manually via cargo update)
use std::path::Path;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CargoAuditAdapter {}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl ILinterAdapterProtocol for CargoAuditAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("cargo-audit")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let mut results = Vec::new();
        let working_dir = resolve_cargo_lock_working_dir(path);
        let working_dir_str = &working_dir.value;

        let cargo_lock = Path::new(working_dir_str).join("Cargo.lock");
        if !cargo_lock.exists() {
            debug!(
                "Skipping cargo-audit: Cargo.lock not found at {:?}",
                cargo_lock
            );
            return Ok(LintResultList::new(results));
        }

        let lockfile = match rustsec::Lockfile::load(&cargo_lock) {
            Ok(lf) => lf,
            Err(e) => {
                debug!("Failed to parse Cargo.lock: {}", e);
                return Ok(LintResultList::new(results));
            }
        };

        let db_dir = match dirs::home_dir() {
            Some(p) => p,
            None => std::path::PathBuf::from("."),
        }
        .join(".cargo")
        .join("advisory-db");
        let db = if db_dir.exists() {
            match rustsec::Database::open(&db_dir) {
                Ok(db) => db,
                Err(_) => {
                    debug!("Failed to open advisory DB, will fetch...");
                    match rustsec::Database::fetch() {
                        Ok(db) => db,
                        Err(e) => {
                            debug!("Failed to fetch advisory DB: {}", e);
                            return Ok(LintResultList::new(results));
                        }
                    }
                }
            }
        } else {
            match rustsec::Database::fetch() {
                Ok(db) => db,
                Err(e) => {
                    debug!("Failed to fetch advisory DB: {}", e);
                    return Ok(LintResultList::new(results));
                }
            }
        };

        let mut ignored_advisories = std::collections::HashSet::new();
        let deny_toml_path = Path::new(working_dir_str).join("deny.toml");
        let deny_toml_str = deny_toml_path.to_string_lossy();
        if utility_file::is_file_generic(&deny_toml_path) {
            let content = utility_file::read_file_safe(&deny_toml_str);
            if let Ok(deny_cfg) = toml::from_str::<toml::Value>(&content) {
                if let Some(advisories) = deny_cfg.get("advisories") {
                    if let Some(ignore) = advisories.get("ignore") {
                        if let Some(ignore_arr) = ignore.as_array() {
                            for val in ignore_arr {
                                if let Some(id) = val.as_str() {
                                    ignored_advisories.insert(id.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        let settings = rustsec::report::Settings::default();
        let report = rustsec::Report::generate(&db, &lockfile, &settings);

        for vuln in &report.vulnerabilities.list {
            let id = vuln.advisory.id.to_string();
            if ignored_advisories.contains(&id) {
                continue;
            }
            let title = &vuln.advisory.title;
            let severity_str = match vuln.advisory.cvss.as_ref() {
                Some(c) => c.severity().to_string().to_lowercase(),
                None => "moderate".to_string(),
            };

            let severity = match severity_str.as_str() {
                "critical" => Severity::CRITICAL,
                "high" => Severity::HIGH,
                "medium" => Severity::MEDIUM,
                _ => Severity::LOW,
            };

            let resolved = shared::common::utility_path_normalization::resolve_capabilities_path(
                match FilePath::new("Cargo.lock".to_string()) {
                    Ok(fp) => fp,
                    Err(_) => path.clone(),
                },
                Some(path.clone()),
            );
            results.push(LintResult {
                file: resolved,
                line: LineNumber::new(0),
                column: ColumnNumber::new(0),
                code: ErrorCode::raw(format!("cargo-audit::{}", id)),
                message: LintMessage::new(format!(
                    "{}: {} ({} v{})",
                    id, title, vuln.package.name, vuln.package.version
                )),
                source: Some(AdapterName::raw("cargo-audit")),
                severity,
                enclosing_scope: None,
                related_locations: LocationList::new(),
            });
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(true))
    }
}

// ─── Block 2: Protocol Trait Implementation ───────────────
// (No protocol implementation found in this file)

// ─── Block 3: Constructors, Helpers, Private Methods ──────

// (No protocol implementation found in this file)

// (No protocol implementation found in this file)

impl CargoAuditAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for CargoAuditAdapter {
    fn default() -> Self {
        Self::new()
    }
}
```

---

## File: crates/external-lint/src/capabilities_rs_clippy_adapter.rs

```rust
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::contract_executor_protocol::ICommandExecutorProtocol;
use shared::common::taxonomy_adapter_error::AdapterError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
use tracing::debug;

use shared::external_lint::utility_external_lint::resolve_cargo_working_dir;

// ─── Block 1: Struct Definition ───────────────────────────

/// Adapter for Rust Clippy static analysis.
pub struct RustLinterAdapter {
    executor: Arc<dyn ICommandExecutorProtocol>,
    _bin_path: Option<FilePath>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl ILinterAdapterProtocol for RustLinterAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("clippy")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let mut results = Vec::new();
        let working_dir = resolve_cargo_working_dir(path);
        let working_dir_str = &working_dir.value;

        let cargo_toml = Path::new(working_dir_str).join("Cargo.toml");
        if !cargo_toml.exists() {
            debug!(
                "Skipping clippy scan: Cargo.toml not found at {:?}",
                cargo_toml
            );
            return Ok(LintResultList::new(results));
        }

        let cmd = vec![
            "cargo".to_string(),
            "clippy".to_string(),
            "--message-format=json".to_string(),
        ];
        let result = self
            .executor
            .execute_command(
                PatternList::new(cmd),
                working_dir.clone(),
                Some(shared::taxonomy_duration_vo::Timeout::new(180.0)),
            )
            .await
            .map_err(|e| {
                LinterOperationError::Adapter(AdapterError::new(
                    self.name(),
                    ErrorMessage::new(e.to_string()),
                ))
            })?;

        let output = if result.stdout.trim().is_empty() {
            result.stderr.clone()
        } else {
            result.stdout.clone()
        };

        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() || !line.starts_with('{') {
                continue;
            }
            match serde_json::from_str::<Value>(line) {
                Ok(data) => {
                    if data.get("reason").and_then(|r| r.as_str()) != Some("compiler-message") {
                        continue;
                    }
                    let msg = match data.get("message") {
                        Some(m) => m,
                        None => continue,
                    };
                    let level = match msg.get("level").and_then(|l| l.as_str()) {
                        Some(l) => l.to_lowercase(),
                        None => "warning".to_string(),
                    };
                    let code = match msg
                        .get("code")
                        .and_then(|c| c.get("code"))
                        .and_then(|c| c.as_str())
                    {
                        Some(c) => c.to_string(),
                        None => "clippy::warning".to_string(),
                    };
                    let message_text = match msg.get("message").and_then(|m| m.as_str()) {
                        Some(m) => m.to_string(),
                        None => "Clippy finding".to_string(),
                    };
                    let spans: Vec<Value> = match msg.get("spans").and_then(|s| s.as_array()) {
                        Some(s) => s.clone(),
                        None => Vec::new(),
                    };

                    for span in &spans {
                        let is_primary = span
                            .get("is_primary")
                            .and_then(|v| v.as_bool())
                            .unwrap_or_default();
                        if !is_primary {
                            continue;
                        }
                        let filename = match span.get("file_name").and_then(|f| f.as_str()) {
                            Some(f) if !f.is_empty() => f,
                            _ => continue,
                        };
                        let resolved_file =
                            shared::common::utility_path_normalization::resolve_capabilities_path(
                                match FilePath::new(filename.to_string()) {
                                    Ok(fp) => fp,
                                    Err(_) => path.clone(),
                                },
                                Some(path.clone()),
                            );
                        let line_num = match span.get("line_start").and_then(|v| v.as_u64()) {
                            Some(v) => v as i64,
                            None => 1,
                        };
                        let column_num = match span.get("column_start").and_then(|v| v.as_u64()) {
                            Some(v) => v as i64,
                            None => 1,
                        };
                        let severity = if level == "error" {
                            Severity::HIGH
                        } else {
                            Severity::MEDIUM
                        };
                        results.push(LintResult {
                            file: resolved_file,
                            line: LineNumber::new(line_num),
                            column: ColumnNumber::new(column_num),
                            code: ErrorCode::raw(code.as_str()),
                            message: LintMessage::new(message_text.as_str()),
                            source: Some(AdapterName::raw("clippy")),
                            severity,
                            enclosing_scope: None,
                            related_locations: LocationList::new(),
                        });
                    }
                }
                Err(_) => continue,
            }
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        let working_dir = resolve_cargo_working_dir(path);
        let cmd = vec![
            "cargo".to_string(),
            "clippy".to_string(),
            "--fix".to_string(),
            "--allow-dirty".to_string(),
            "--allow-staged".to_string(),
        ];
        let _ = self
            .executor
            .execute_command(
                PatternList::new(cmd),
                working_dir,
                Some(shared::taxonomy_duration_vo::Timeout::new(180.0)),
            )
            .await;
        Ok(ComplianceStatus::new(true))
    }
}

// PURPOSE: RsClippyAdapter — ILinterAdapterProtocol implementation for Clippy linting integration
//
// Executes `cargo clippy --message-format=json` as a subprocess, then parses
// the JSON output line by line. Clippy outputs one JSON object per diagnostic
// message, each containing spans (source locations), severity levels, and
// lint codes.
//
// The adapter handles:
//   - Finding the correct Cargo.toml parent directory
//   - Parsing the JSON stream (filtering for compiler-message reasons)
//   - Resolving relative file paths to absolute across workspaces
//   - Converting Clippy severity levels to AES severity levels
//   - Falling back to stderr if stdout is empty (Clippy sometimes outputs to stderr)
//
// NOTE: apply_fix runs `cargo clippy --fix` which modifies files in place.
// This is the only adapter that supports auto-fix.
use std::path::Path;

impl RustLinterAdapter {
    pub fn new(executor: Arc<dyn ICommandExecutorProtocol>, bin_path: Option<FilePath>) -> Self {
        Self {
            executor,
            _bin_path: bin_path,
        }
    }
}

// (No constructors or helpers found in this file)

// (No constructors or helpers found in this file)

// ─── Block 3: Constructors, Helpers, Private Methods ──────
// (No constructors or helpers found in this file)
```

---

## File: crates/external-lint/src/capabilities_rs_fmt_adapter.rs

```rust
use std::sync::Arc;

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::contract_executor_protocol::ICommandExecutorProtocol;
use shared::common::taxonomy_adapter_error::AdapterError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
use tracing::debug;

use shared::external_lint::utility_external_lint::resolve_cargo_working_dir;

// PURPOSE: RsFmtAdapter — ILinterAdapterProtocol implementation for rustfmt integration
//
// Runs `cargo fmt --check` on Rust projects. Since rustfmt is a formatter
// (not a linter), the adapter parses diff output lines to report each
// formatting difference as an individual LintResult.
//
// Key design decisions:
//   - Resolves Cargo.toml parent dir as working directory (via resolve_cargo_working_dir)
//   - Uses ICommandExecutorProtocol for subprocess execution with 120s timeout
//   - apply_fix runs `cargo fmt` (without --check) to auto-format
//   - Only reports added lines (+ prefix) as violations, not context lines
use std::path::Path;

// ─── Block 1: Struct Definition ───────────────────────────

/// Adapter that wraps `cargo fmt --check` as an ILinterAdapterProtocol.
///
/// Parses rustfmt's unified diff output to create per-difference LintResults.
/// When no Cargo.toml is found, the scan is silently skipped.
pub struct RustFmtAdapter {
    executor: Arc<dyn ICommandExecutorProtocol>,
    _bin_path: Option<FilePath>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl ILinterAdapterProtocol for RustFmtAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("rustfmt")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let mut results = Vec::new();

        // Find the Cargo.toml parent to use as working directory — resolves workspace roots
        let working_dir = resolve_cargo_working_dir(path);
        let working_dir_str = &working_dir.value;

        let cargo_toml = Path::new(working_dir_str).join("Cargo.toml");
        if !cargo_toml.exists() {
            debug!("Skipping rustfmt: Cargo.toml not found at {:?}", cargo_toml);
            return Ok(LintResultList::new(results));
        }

        // Run `cargo fmt --check` — exits non-zero when formatting differs
        let cmd = vec![
            "cargo".to_string(),
            "fmt".to_string(),
            "--check".to_string(),
        ];
        let result = self
            .executor
            .execute_command(
                PatternList::new(cmd),
                working_dir.clone(),
                Some(shared::taxonomy_duration_vo::Timeout::new(120.0)),
            )
            .await
            .map_err(|e| {
                LinterOperationError::Adapter(AdapterError::new(
                    self.name(),
                    ErrorMessage::new(e.to_string()),
                ))
            })?;

        if result.returncode == 0 {
            return Ok(LintResultList::new(results));
        }

        // Parse rustfmt's unified diff output.
        // Format: "Diff in <file> at line N:" followed by diff hunks
        let output = result.stdout + &result.stderr;
        let mut current_file = String::new();
        for line in output.lines() {
            // Track which file the current diff hunk belongs to
            if line.starts_with("Diff in ") {
                current_file = line
                    .trim_start_matches("Diff in ")
                    .trim_end_matches(':')
                    .trim()
                    .to_string();
            } else if line.starts_with("--- ") || line.starts_with("+++ ") {
                continue;
            }

            // Report added lines (+) as formatting violations
            if line.starts_with('+') && !line.starts_with("+++") {
                let resolved =
                    shared::common::utility_path_normalization::resolve_capabilities_path(
                        match FilePath::new(current_file.clone()) {
                            Ok(fp) => fp,
                            Err(_) => path.clone(),
                        },
                        Some(path.clone()),
                    );
                results.push(LintResult {
                    file: resolved,
                    line: LineNumber::new(0),
                    column: ColumnNumber::new(0),
                    code: ErrorCode::raw("rustfmt::unformatted"),
                    message: LintMessage::new(line.trim().to_string()),
                    source: Some(AdapterName::raw("rustfmt")),
                    severity: Severity::MEDIUM,
                    enclosing_scope: None,
                    related_locations: LocationList::new(),
                });
            }
        }

        if results.is_empty() {
            // No diff lines parsed — cargo fmt --check may have exited non-zero
            // for a reason unrelated to formatting (e.g., parse error). Don't
            // create a fake violation.
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        let working_dir = resolve_cargo_working_dir(path);
        let cmd = vec!["cargo".to_string(), "fmt".to_string()];
        let _ = self
            .executor
            .execute_command(
                PatternList::new(cmd),
                working_dir,
                Some(shared::taxonomy_duration_vo::Timeout::new(120.0)),
            )
            .await;
        Ok(ComplianceStatus::new(true))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl RustFmtAdapter {
    pub fn new(executor: Arc<dyn ICommandExecutorProtocol>, bin_path: Option<FilePath>) -> Self {
        Self {
            executor,
            _bin_path: bin_path,
        }
    }
}
```

---

## File: crates/external-lint/src/capabilities_stdio_client.rs

```rust
// PURPOSE: StdioClient — ICommandExecutorProtocol implementation via stdio
use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use shared::common::contract_executor_protocol::ICommandExecutorProtocol;
use shared::common::taxonomy_path_vo::FilePath;
use shared::mcp_server::taxonomy_job_vo::ResponseData;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_duration_vo::Timeout;
use tokio::process::Command;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct StdioClient {
    timeout: Timeout,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl ICommandExecutorProtocol for StdioClient {
    async fn execute_command(
        &self,
        command: PatternList,
        working_dir: FilePath,
        timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData> {
        let timeout_val = match timeout {
            Some(d) => Duration::from_secs_f64(d.value()),
            None => Duration::from_secs_f64(self.timeout.value()),
        };
        let cmd_list: Vec<&str> = command.values.iter().map(|s| s.as_str()).collect();
        if cmd_list.is_empty() {
            anyhow::bail!("Empty command");
        }
        let mut cmd = Command::new(cmd_list[0]);
        if cmd_list.len() > 1 {
            cmd.args(&cmd_list[1..]);
        }
        cmd.current_dir(&working_dir.value)
            .env("PYTHONUNBUFFERED", "1");
        cmd.kill_on_drop(true);

        let result = tokio::time::timeout(timeout_val, cmd.output()).await;
        match result {
            Ok(Ok(output)) => {
                let mut meta_map = HashMap::new();
                meta_map.insert(
                    "protocol".to_string(),
                    serde_json::Value::String("Stdio".to_string()),
                );
                Ok(ResponseData {
                    value: Some(serde_json::Value::Null),
                    stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                    returncode: match output.status.code() {
                        Some(c) => c as i64,
                        None => -1,
                    },
                    metadata: meta_map,
                })
            }
            Ok(Err(e)) => anyhow::bail!("Command execution failed: {}", e),
            Err(_) => anyhow::bail!("Command timed out after {}s", timeout_val.as_secs()),
        }
    }

    async fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::new())
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl StdioClient {
    pub fn new(timeout: Timeout) -> Self {
        Self { timeout }
    }
}
```

---

## File: crates/external-lint/src/lib.rs

```rust
// PURPOSE: Module declarations for external-lint (external linter adapters)
pub use shared::common::taxonomy_adapter_error::{AdapterError, ScanError, ValidationError};

pub mod agent_external_lint_orchestrator;
pub mod capabilities_stdio_client;
pub use capabilities_stdio_client::StdioClient;
pub mod capabilities_external_lint_executor;
pub use capabilities_external_lint_executor::ExternalLintExecutor;
pub mod capabilities_js_eslint_adapter;
pub use capabilities_js_eslint_adapter::ESLintAdapter;
pub mod capabilities_js_prettier_adapter;
pub use capabilities_js_prettier_adapter::PrettierAdapter;
pub mod capabilities_js_tsc_adapter;
pub use capabilities_js_tsc_adapter::TSCAdapter;
pub mod capabilities_py_bandit_adapter;
pub use capabilities_py_bandit_adapter::BanditAdapter;
pub mod capabilities_py_mypy_adapter;
pub use capabilities_py_mypy_adapter::MyPyAdapter;
pub mod capabilities_py_ruff_adapter;
pub use capabilities_py_ruff_adapter::RuffAdapter;
pub mod capabilities_rs_audit_adapter;
pub use capabilities_rs_audit_adapter::CargoAuditAdapter;
pub mod capabilities_rs_fmt_adapter;
pub use capabilities_rs_fmt_adapter::RustFmtAdapter;
pub mod capabilities_rs_clippy_adapter;
pub use capabilities_rs_clippy_adapter::RustLinterAdapter;
pub mod root_external_lint_container;
pub use root_external_lint_container::ExternalLintContainer;
```

---

## File: crates/external-lint/src/root_external_lint_container.rs

```rust
// PURPOSE: ExternalLintContainer — root layer, wires orchestrator with utility adapters
//
// The DI container that assembles the external lint subsystem:
//   1. Creates a StdioClient (ICommandExecutorProtocol) for subprocess execution
//   2. Creates ExternalLintExecutor (IExternalLintExecutorProtocol) for command execution
//   3. Registers all 9 adapters (ruff, bandit, mypy, eslint, prettier, tsc, clippy, rustfmt, cargo-audit)
//
// Each adapter follows the same pattern: Arc<dyn ILinterAdapterProtocol> in a HashMap keyed by name.
use std::collections::HashMap;
use std::sync::Arc;

use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::external_lint::contract_external_lint_executor_protocol::IExternalLintExecutorProtocol;
use shared::taxonomy_duration_vo::Timeout;

pub struct ExternalLintContainer {
    aggregate: Arc<dyn IExternalLintAggregate>,
}

impl ExternalLintContainer {
    pub fn new() -> Self {
        let executor: Arc<
            dyn shared::common::contract_executor_protocol::ICommandExecutorProtocol,
        > = Arc::new(crate::capabilities_stdio_client::StdioClient::new(
            Timeout::new(60.0),
        ));

        let lint_executor: Arc<dyn IExternalLintExecutorProtocol> = Arc::new(
            crate::capabilities_external_lint_executor::ExternalLintExecutor::new(executor.clone()),
        );

        let mut adapters: HashMap<String, Arc<dyn ILinterAdapterProtocol>> = HashMap::new();
        adapters.insert(
            "ruff".to_string(),
            Arc::new(crate::capabilities_py_ruff_adapter::RuffAdapter::new(
                lint_executor.clone(),
                None,
            )),
        );
        adapters.insert(
            "bandit".to_string(),
            Arc::new(crate::capabilities_py_bandit_adapter::BanditAdapter::new(
                lint_executor.clone(),
                None,
            )),
        );
        adapters.insert(
            "mypy".to_string(),
            Arc::new(crate::capabilities_py_mypy_adapter::MyPyAdapter::new(
                lint_executor.clone(),
                None,
            )),
        );
        adapters.insert(
            "eslint".to_string(),
            Arc::new(crate::capabilities_js_eslint_adapter::ESLintAdapter::new(
                lint_executor.clone(),
            )),
        );
        adapters.insert(
            "prettier".to_string(),
            Arc::new(
                crate::capabilities_js_prettier_adapter::PrettierAdapter::new(
                    lint_executor.clone(),
                ),
            ),
        );
        adapters.insert(
            "tsc".to_string(),
            Arc::new(crate::capabilities_js_tsc_adapter::TSCAdapter::new(
                lint_executor.clone(),
            )),
        );
        adapters.insert(
            "clippy".to_string(),
            Arc::new(
                crate::capabilities_rs_clippy_adapter::RustLinterAdapter::new(
                    executor.clone(),
                    None,
                ),
            ),
        );
        adapters.insert(
            "rustfmt".to_string(),
            Arc::new(crate::capabilities_rs_fmt_adapter::RustFmtAdapter::new(
                executor.clone(),
                None,
            )),
        );
        adapters.insert(
            "cargo-audit".to_string(),
            Arc::new(crate::capabilities_rs_audit_adapter::CargoAuditAdapter::new()),
        );

        Self {
            aggregate: Arc::new(
                crate::agent_external_lint_orchestrator::ExternalLintOrchestrator::new(adapters),
            ),
        }
    }

    pub fn new_default() -> Self {
        Self::new()
    }

    pub fn aggregate(&self) -> Arc<dyn IExternalLintAggregate> {
        self.aggregate.clone()
    }
}

impl Default for ExternalLintContainer {
    fn default() -> Self {
        Self::new()
    }
}
```

---

## File: crates/shared/src/auto-fix/taxonomy_fix_applied_event.rs

```rust
// PURPOSE: FixApplied — domain event published when a lint fix is applied
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::Timestamp;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FixApplied {
    pub path: FilePath,
    pub adapter: AdapterName,
    pub error_code: ErrorCode,
    pub changes_count: Count,
    #[serde(default)]
    pub timestamp: Timestamp,
}

impl FixApplied {
    pub fn new(
        path: FilePath,
        adapter: AdapterName,
        error_code: ErrorCode,
        changes_count: Count,
    ) -> Self {
        Self {
            path,
            adapter,
            error_code,
            changes_count,
            timestamp: Timestamp::default(),
        }
    }
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

## File: crates/shared/src/code-analysis/contract_adapter_protocol.rs

```rust
// PURPOSE: ILinterAdapterProtocol — protocol trait for linter adapter implementations (Ruff, Mypy, Clippy, etc.)

use async_trait::async_trait;

use crate::code_analysis::taxonomy_analysis_vo::LintResultList;
use crate::code_analysis::taxonomy_operation_error::LinterOperationError;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::common::taxonomy_path_vo::FilePath;

/// Abstract interface for linter adapters.
/// Implemented by Infrastructure (e.g., RuffAdapter, MypyAdapter).
#[async_trait]
pub trait ILinterAdapterProtocol: Send + Sync {
    fn name(&self) -> AdapterName;
    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError>;
    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError>;
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

## File: crates/shared/src/code-analysis/taxonomy_operation_error.rs

```rust
// PURPOSE: LinterOperationError — structured error type for linter operation failures (scan, fix, report)
use crate::common::taxonomy_adapter_error::AdapterError;
use crate::common::taxonomy_adapter_error::ScanError;
/// linter_operation_error — Unified error type for linter adapter operations.
/* UNKNOWN: ErrorMessage */
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::LineNumber;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum LinterOperationError {
    #[error("Scan error: {0}")]
    Scan(ScanError),

    #[error("Adapter error: {0}")]
    Adapter(AdapterError),
}

impl LinterOperationError {
    pub fn message(&self) -> ErrorMessage {
        let _ = &LineNumber::default();
        ErrorMessage::new(self.to_string())
    }
}

impl From<ScanError> for LinterOperationError {
    fn from(e: ScanError) -> Self {
        LinterOperationError::Scan(e)
    }
}

impl From<AdapterError> for LinterOperationError {
    fn from(e: AdapterError) -> Self {
        LinterOperationError::Adapter(e)
    }
}
```

---

## File: crates/shared/src/common/contract_executor_protocol.rs

```rust
// PURPOSE: Port: ICommandExecutorProtocol — trait for executing shell commands and capturing response
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_duration_vo::Timeout;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_response_data_vo::ResponseData;

#[async_trait::async_trait]
pub trait ICommandExecutorProtocol: Send + Sync {
    /// Execute a command and return the response.
    async fn execute_command(
        &self,
        command: PatternList,
        working_dir: FilePath,
        timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData>;

    /// Check the health of the execution transport.
    async fn health_check(&self) -> anyhow::Result<ResponseData>;
}
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
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_job_id_vo::JobId;
use crate::common::taxonomy_layer_vo::LineContentVO;
use crate::common::taxonomy_response_data_vo::ResponseData;
use crate::common::taxonomy_severity_vo::Severity;

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

## File: crates/shared/src/common/utility_path_normalization.rs

```rust
// PURPOSE: Path normalization utilities for external tool execution (clippy, ruff, eslint, ...).
// Plain free functions — no protocol / dependency injection.
use crate::common::taxonomy_path_vo::FilePath;

/// Return `path` unchanged. External lint tools already receive absolute/normalized paths.
pub fn normalize_path(path: FilePath) -> FilePath {
    path
}

/// Resolve a capability/module `path` relative to an optional `context_path`.
/// Default behavior: the path is returned unchanged.
pub fn resolve_capabilities_path(path: FilePath, _context_path: Option<FilePath>) -> FilePath {
    path
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

## File: crates/shared/src/external-lint/contract_external_lint_executor_protocol.rs

```rust
// PURPOSE: IExternalLintExecutorProtocol — protocol for external lint command execution
// Defines the interface for executing linter commands with error mapping.

use crate::code_analysis::taxonomy_operation_error::LinterOperationError;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_response_data_vo::ResponseData;

/// Protocol for executing external linter commands.
///
/// Implementations wrap `ICommandExecutorProtocol` and add error mapping
/// for scan and adapter operations.
#[async_trait::async_trait]
pub trait IExternalLintExecutorProtocol: Send + Sync {
    /// Execute a command, mapping failures to `LinterOperationError::Scan`.
    async fn exec_cmd_scan(
        &self,
        args: Vec<String>,
        working_dir: FilePath,
        timeout_secs: f64,
        adapter_name: Option<AdapterName>,
        path: &FilePath,
    ) -> Result<ResponseData, LinterOperationError>;

    /// Execute a command, mapping failures to `LinterOperationError::Adapter`.
    async fn exec_cmd_adapter(
        &self,
        args: Vec<String>,
        working_dir: FilePath,
        timeout_secs: f64,
        adapter_name: AdapterName,
    ) -> Result<ResponseData, LinterOperationError>;

    /// Apply a JS tool's fix command.
    async fn js_apply_fix(
        &self,
        path: &FilePath,
        tool: &str,
        fix_arg: &str,
    ) -> Result<ComplianceStatus, LinterOperationError>;
}
```

---

## File: crates/shared/src/external-lint/contract_external_lint_language_detector_protocol.rs

```rust
// PURPOSE: IExternalLintLanguageDetectorProtocol — protocol for detecting languages in a project directory
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

pub struct DetectedLanguages {
    pub has_rs: bool,
    pub has_py: bool,
    pub has_js: bool,
}

#[async_trait]
pub trait IExternalLintLanguageDetectorProtocol: Send + Sync {
    /// Detect which languages are present at the given path.
    async fn detect_languages(&self, path: &FilePath) -> DetectedLanguages;
}
```

---

## File: crates/shared/src/external-lint/contract_external_lint_selector_protocol.rs

```rust
// PURPOSE: IExternalLintSelectorProtocol — protocol for selecting adapters based on detected languages
use crate::common::taxonomy_adapter_list_vo::AdapterNameList;
use async_trait::async_trait;

/// Protocol for choosing which external-lint adapters to run.
///
/// Based on booleans indicating the presence of Rust, Python, or TypeScript
/// files in the project, the selector returns the list of adapter names
/// that should be invoked during the external linting phase.
#[async_trait]
pub trait IExternalLintSelectorProtocol: Send + Sync {
    fn select_adapters(&self, has_rs: bool, has_py: bool, has_js: bool) -> AdapterNameList;
}
```

---

## File: crates/shared/src/external-lint/contract_external_lint_utility_protocol.rs

```rust
// PURPOSE: IExternalLintPathProtocol — protocol for path operations in external lint
use crate::code_analysis::taxonomy_operation_error::LinterOperationError;
use crate::common::contract_executor_protocol::ICommandExecutorProtocol;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_duration_vo::Timeout;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use crate::common::taxonomy_response_data_vo::ResponseData;
use async_trait::async_trait;

// ─── Path Operations ──────────────────────────────────────

#[async_trait]
pub trait IExternalLintPathProtocol: Send + Sync {
    fn canonicalize_path(&self, path_str: &str) -> FilePath;
    fn default_working_dir(&self, path: &FilePath) -> FilePath;
}

// ─── Language Detection ───────────────────────────────────

#[async_trait]
pub trait IExternalLintLanguageProtocol: Send + Sync {
    fn has_python_files(&self, path: &FilePath) -> bool;
    fn has_py_in_dir(&self, dir: &DirectoryPath) -> bool;
    fn is_in_path(&self, executable: &str) -> bool;
}

// ─── JS Adapter Operations ────────────────────────────────

#[async_trait]
pub trait IExternalLintJsProtocol: Send + Sync {
    fn resolve_js_cmd(
        &self,
        executable: &str,
        args: PatternList,
        working_dir: &FilePath,
    ) -> PatternList;
    fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath;
    async fn js_apply_fix(
        &self,
        executor: &dyn ICommandExecutorProtocol,
        path: &FilePath,
        tool: &str,
        fix_arg: &str,
    ) -> Result<ComplianceStatus, LinterOperationError>;
}

// ─── Cargo Adapter Operations ─────────────────────────────

#[async_trait]
pub trait IExternalLintCargoProtocol: Send + Sync {
    fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath;
    fn resolve_cargo_lock_working_dir(&self, path: &FilePath) -> FilePath;
}

// ─── Command Execution ────────────────────────────────────

#[async_trait]
pub trait IExternalLintCommandProtocol: Send + Sync {
    async fn exec_cmd_scan(
        &self,
        executor: &dyn ICommandExecutorProtocol,
        args: PatternList,
        working_dir: FilePath,
        timeout_secs: Timeout,
        adapter_name: Option<AdapterName>,
        path: &FilePath,
    ) -> Result<ResponseData, LinterOperationError>;
    async fn exec_cmd_adapter(
        &self,
        executor: &dyn ICommandExecutorProtocol,
        args: PatternList,
        working_dir: FilePath,
        timeout_secs: Timeout,
        adapter_name: AdapterName,
    ) -> Result<ResponseData, LinterOperationError>;
    async fn noop_apply_fix(&self) -> Result<ComplianceStatus, LinterOperationError>;
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

## File: crates/shared/src/external-lint/utility_external_lint.rs

```rust
// PURPOSE: utility_external_lint_helper — pure utility functions for external linter adapters
// No contract imports — only taxonomy types allowed in utility layer.

use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::common::taxonomy_path_vo::FilePath;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::code_analysis::taxonomy_operation_error::LinterOperationError;

/// Canonicalize a path string, falling back to the original on error.
pub fn canonicalize_path(path_str: &str) -> String {
    match std::fs::canonicalize(path_str) {
        Ok(p) => p.to_string_lossy().to_string(),
        Err(_) => path_str.to_string(),
    }
}

/// Create a default `"."` working directory, falling back to the given path if it fails.
pub fn default_working_dir(path: &FilePath) -> FilePath {
    FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone())
}

/// No-op apply_fix for linters that cannot auto-fix (scanners, type-checkers).
pub async fn noop_apply_fix() -> Result<ComplianceStatus, LinterOperationError> {
    Ok(ComplianceStatus::new(false))
}

/// Return true if the given path contains any Python (`.py`) files.
pub fn has_python_files(path: &FilePath) -> bool {
    let p = std::path::Path::new(&path.value);
    if !p.exists() {
        return p.extension().map(|e| e == "py").unwrap_or(false);
    }
    if p.is_file() {
        return p.extension().map(|e| e == "py").unwrap_or(false);
    }
    has_py_in_dir(p)
}

fn has_py_in_dir(dir: &std::path::Path) -> bool {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return false;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if has_py_in_dir(&path) {
                return true;
            }
        } else if path.extension().map(|e| e == "py").unwrap_or(false) {
            return true;
        }
    }
    false
}

/// Resolve the executable command for a JS tool (eslint, prettier, tsc).
pub fn resolve_js_cmd(executable: &str, args: Vec<String>, working_dir: &str) -> Vec<String> {
    let local_bin = Path::new(working_dir)
        .join("node_modules")
        .join(".bin")
        .join(executable);
    if local_bin.exists() {
        let mut cmd = vec![local_bin.to_string_lossy().to_string()];
        cmd.extend(args);
        return cmd;
    }
    let runner = if is_bun_available() { "bunx" } else { "npx" };
    let mut cmd = vec![runner.to_string(), executable.to_string()];
    cmd.extend(args);
    cmd
}

/// Walk up from the given path to find the JS project root.
pub fn resolve_js_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if let Ok(abs_path) = std::fs::canonicalize(path_str) {
        let mut current = if abs_path.is_file() {
            abs_path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from("."))
        } else {
            abs_path.clone()
        };
        for _ in 0..10 {
            if current.join("lint_arwaky.config.yaml").is_file()
                || current.join("lint_arwaky.config.python.yaml").is_file()
                || current.join("package.json").is_file()
                || current.join(".git").is_dir()
            {
                return FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default();
            }
            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => break,
            }
        }
        return FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default();
    }
    FilePath::new(".".to_string()).unwrap_or_default()
}

/// Find parent dir with Cargo.toml (for cargo fmt, cargo clippy).
pub fn resolve_cargo_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if path_str.is_empty() {
        return path.clone();
    }
    let current = Path::new(path_str);
    if current.is_dir() {
        if current.join("Cargo.toml").exists() {
            return path.clone();
        }
    } else if let Some(parent) = current.parent() {
        if parent.join("Cargo.toml").exists() {
            return FilePath::new(parent.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| path.clone());
        }
        if let Some(grandparent) = parent.parent() {
            if grandparent.join("Cargo.toml").exists() {
                return FilePath::new(grandparent.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|_| path.clone());
            }
        }
    }
    FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone())
}

/// Find parent dir with Cargo.lock (for cargo-audit).
pub fn resolve_cargo_lock_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if path_str.is_empty() {
        return path.clone();
    }
    let current = Path::new(path_str);
    if current.is_dir() {
        if current.join("Cargo.lock").exists() {
            return path.clone();
        }
    } else if let Some(parent) = current.parent() {
        if parent.join("Cargo.lock").exists() {
            return FilePath::new(parent.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| path.clone());
        }
        if let Some(grandparent) = parent.parent() {
            if grandparent.join("Cargo.lock").exists() {
                return FilePath::new(grandparent.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|_| path.clone());
            }
        }
    }
    FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone())
}

static BUN_AVAILABLE: OnceLock<bool> = OnceLock::new();

fn is_bun_available() -> bool {
    *BUN_AVAILABLE.get_or_init(|| {
        std::process::Command::new("bun")
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    })
}
```

---

## File: crates/shared/src/external-lint/utility_external_lint_io.rs

```rust
// PURPOSE: utility_external_lint_io — stateless I/O utilities for external lint adapters
use crate::common::utility_file;
use std::path::{Path, PathBuf};

/// Canonicalize a path, returning the original path on error.
pub fn canonicalize_path(path_str: &str) -> PathBuf {
    match std::fs::canonicalize(path_str) {
        Ok(p) => p,
        Err(_) => PathBuf::from(path_str),
    }
}

/// Scan directory entries, returning vector of (file_name, file_path, is_dir) tuples.
pub fn scan_directory(dir_path: &Path) -> Vec<(String, String, bool)> {
    let mut entries = Vec::new();
    if let Ok(read_dir) = dir_path.read_dir() {
        for dir_entry in read_dir.flatten() {
            if let Some(name) = dir_entry.file_name().to_str() {
                let path = dir_entry.path();
                let is_dir = utility_file::is_dir(&path);
                entries.push((name.to_string(), path.to_string_lossy().to_string(), is_dir));
            }
        }
    }
    entries
}

/// Recursively scan directory for Python files.
/// Returns true if any .py file is found.
pub fn has_python_files(dir_path: &Path) -> bool {
    if let Ok(entries) = dir_path.read_dir() {
        for dir_entry in entries.flatten() {
            let path = dir_entry.path();
            if utility_file::is_dir(&path) {
                if has_python_files(&path) {
                    return true;
                }
            } else if path.extension().map(|e| e == "py").unwrap_or(false) {
                return true;
            }
        }
    }
    false
}

/// Check if a configuration file exists at the given path.
pub fn has_config_file(dir_path: &Path) -> bool {
    utility_file::is_file_generic(dir_path.join("lint_arwaky.config.yaml"))
        || utility_file::is_file_generic(dir_path.join("lint_arwaky.config.python.yaml"))
        || utility_file::is_file_generic(dir_path.join("package.json"))
        || utility_file::is_dir(dir_path.join(".git"))
}

/// Check if Cargo.toml exists at the given path (or parent/grandparent).
pub fn has_cargo_toml(path_str: &str) -> Option<String> {
    let current = Path::new(path_str);
    if utility_file::is_dir(current) && utility_file::is_file_generic(current.join("Cargo.toml")) {
        return Some(path_str.to_string());
    }
    if let Some(parent) = current.parent() {
        if utility_file::is_file_generic(parent.join("Cargo.toml")) {
            return Some(parent.to_string_lossy().replace('\\', "/"));
        }
        if let Some(grandparent) = parent.parent() {
            if utility_file::is_file_generic(grandparent.join("Cargo.toml")) {
                return Some(grandparent.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    None
}

/// Check if Cargo.lock exists at the given path (or parent/grandparent).
pub fn has_cargo_lock(path_str: &str) -> Option<String> {
    let current = Path::new(path_str);
    if utility_file::is_dir(current) && utility_file::is_file_generic(current.join("Cargo.lock")) {
        return Some(path_str.to_string());
    }
    if let Some(parent) = current.parent() {
        if utility_file::is_file_generic(parent.join("Cargo.lock")) {
            return Some(parent.to_string_lossy().replace('\\', "/"));
        }
        if let Some(grandparent) = parent.parent() {
            if utility_file::is_file_generic(grandparent.join("Cargo.lock")) {
                return Some(grandparent.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    None
}

/// Check if an executable exists in PATH directories.
pub fn is_executable_in_path(executable: &str) -> bool {
    if let Ok(path_var) = std::env::var("PATH") {
        for path_dir in std::env::split_paths(&path_var) {
            let path = path_dir.join(executable);
            if utility_file::is_file_generic(&path) {
                return true;
            }
        }
    }
    false
}

/// Check if a local bin executable exists.
pub fn has_local_bin(working_dir: &Path, executable: &str) -> bool {
    let local_bin = working_dir
        .join("node_modules")
        .join(".bin")
        .join(executable);
    utility_file::is_file_generic(local_bin)
}
```

---

## File: crates/shared/src/mcp-server/mod.rs

```rust
// mcp-server — taxonomy and contract types
pub mod contract_mcp_server_aggregate;
pub mod taxonomy_mcp_tool_args_vo;
// Re-export from common for backward compatibility
pub use crate::common::taxonomy_action_vo;
pub use crate::common::taxonomy_job_vo;
```

---

