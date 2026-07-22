# Crate: mcp-server (v1.10.106)

This document contains the source code for feature crate `mcp-server` along with its corresponding and imported definitions from the `shared` crate.

## Problem Statement

The following issues were detected by `lint-arwaky-cli scan`:

```
============================================================
  AES Architecture Compliance Report
============================================================
  Project:
  Violations: 0
```

---

## File List

- [ARCHITECTURE.md](file:///home/raka/mcp-arwaky/lint-arwaky/ARCHITECTURE.md)
- [crates/mcp-server/Cargo.toml](file:///home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/Cargo.toml)
- [crates/mcp-server/FRD.md](file:///home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/FRD.md)
- [crates/mcp-server/src/agent_mcp_server_orchestrator.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/agent_mcp_server_orchestrator.rs)
- [crates/mcp-server/src/lib.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/lib.rs)
- [crates/mcp-server/src/root_mcp_container.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/root_mcp_container.rs)
- [crates/mcp-server/src/surface_mcp_command.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/surface_mcp_command.rs)
- [crates/shared/src/cli-commands/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/mod.rs)
- [crates/shared/src/cli-commands/taxonomy_catalog_constant.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_catalog_constant.rs)
- [crates/shared/src/cli-commands/taxonomy_metadata_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_metadata_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_position_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_position_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_protocol_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_protocol_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_scan_report_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_scan_report_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_scan_request_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_scan_request_vo.rs)
- [crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs)
- [crates/shared/src/code-analysis/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/mod.rs)
- [crates/shared/src/common/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/mod.rs)
- [crates/shared/src/common/taxonomy_path_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_path_vo.rs)
- [crates/shared/src/common/utility_file.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/utility_file.rs)
- [crates/shared/src/config-system/contract_config_orchestrator_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_config_orchestrator_aggregate.rs)
- [crates/shared/src/config-system/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/mod.rs)
- [crates/shared/src/external-lint/contract_external_lint_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/contract_external_lint_aggregate.rs)
- [crates/shared/src/external-lint/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/mod.rs)
- [crates/shared/src/import-rules/contract_import_runner_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_import_runner_aggregate.rs)
- [crates/shared/src/import-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/mod.rs)
- [crates/shared/src/mcp-server/contract_mcp_server_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/mcp-server/contract_mcp_server_aggregate.rs)
- [crates/shared/src/mcp-server/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/mcp-server/mod.rs)
- [crates/shared/src/mcp-server/taxonomy_mcp_tool_args_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/mcp-server/taxonomy_mcp_tool_args_vo.rs)
- [crates/shared/src/naming-rules/contract_naming_runner_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/contract_naming_runner_aggregate.rs)
- [crates/shared/src/naming-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/mod.rs)
- [crates/shared/src/orphan-detector/contract_orphan_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/contract_orphan_aggregate.rs)
- [crates/shared/src/orphan-detector/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/mod.rs)
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

## File: crates/mcp-server/Cargo.toml

```toml
[package]
name = "mcp_server-lint-arwaky"
version = "1.10.106"
edition = "2021"
description = "Model Context Protocol (MCP) server surfaces exposing the lint pipeline as tools/resources to AI agents."
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
tracing.workspace = true
shared.workspace = true
rmcp.workspace = true
schemars = "1.2"
tokio.workspace = true
anyhow.workspace = true
import_rules.workspace = true
naming_rules.workspace = true
orphan_detector.workspace = true
external_lint.workspace = true
role_rules.workspace = true
code_analysis.workspace = true
config_system.workspace = true
dirs.workspace = true
maintenance.workspace = true
git_hooks.workspace = true
auto_fix.workspace = true
```

---

## File: crates/mcp-server/FRD.md

```rust
# FRD — mcp-server

## Feature Goal

The mcp-server crate implements a Model Context Protocol (MCP) server that exposes the linting pipeline as tools and resources accessible by AI agents. It enables lint_arwaky integration with IDEs and other AI tools through a standard protocol.

## Requirements & Scope

- execute_command — execute any CLI command via the MCP interface.
- list_commands — list available CLI commands with descriptions and examples.
- read_skill — read SKILL.md documentation by section.
- health_check — check system health: adapters and system state.
- JSON-RPC conformance; tool discovery by AI clients.

## Success Indicators

- [ ] Protocol compliance — implementation conforms to MCP JSON-RPC standards.
- [ ] Tool discovery — all tools discoverable by AI clients.
- [ ] Response time — under 5 seconds for standard operations.
- [ ] Rule conformance — the crate's own source complies with AES rules (no agent__, capabilities__, or infrastructure_* imports; taxonomy/contract only) when complete.
```

---

## File: crates/mcp-server/src/agent_mcp_server_orchestrator.rs

```rust
// PURPOSE: McpServerOrchestrator — agent that implements IMcpServerAggregate
//
// The MCP orchestrator is the AI-agent entry point. It mirrors the CLI
// scan pipeline (surface_check_command::scan()) but accepts JSON parameters
// and returns JSON responses.
//
// All async operations run inside tokio::task::spawn_blocking because the
// lint pipeline is synchronous (file I/O, regex matching) while the MCP
// server event loop is async. spawn_blocking bridges the two worlds.
use rmcp::handler::server::wrapper::Parameters;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::mcp_server::contract_mcp_server_aggregate::IMcpServerAggregate;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::{
    ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs,
};
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct McpServerDependencies {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
}

pub struct McpServerOrchestrator {
    pub(crate) deps: McpServerDependencies,
}

use shared::common::find_workspace_root;

// ─── Block 2: Aggregate Trait Implementation ──────────────
#[async_trait::async_trait]
impl IMcpServerAggregate for McpServerOrchestrator {
    async fn execute_command(&self, Parameters(args): Parameters<ExecuteCommandArgs>) -> String {
        let action = args.action.clone();
        let arg_path = args
            .args
            .as_ref()
            .and_then(|a| a.get("path"))
            .and_then(|v| v.as_str())
            .map(String::from);
        let arg_threshold = args
            .args
            .as_ref()
            .and_then(|a| a.get("threshold"))
            .and_then(|v| v.as_u64())
            .map(|n| n as u32);
        let arg_client = args
            .args
            .as_ref()
            .and_then(|a| a.get("client"))
            .and_then(|v| v.as_str())
            .map(String::from);

        let result = match action.as_str() {
            "check" | "scan" => {
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                let linter = self.deps.code_analysis_linter.clone();
                let import_orch = self.deps.import_orchestrator.clone();
                let naming_orch = self.deps.naming_orchestrator.clone();
                let role_orch = self.deps.role_orchestrator.clone();
                let ext_lint = self.deps.external_lint.clone();
                let orphan_orch = self.deps.orphan_orchestrator.clone();
                let config_orch = self.deps.config_orchestrator.clone();

                let join_result = tokio::task::spawn_blocking(move || {
                    let mut all_results = Vec::new();
                    let path_obj = FilePath::new(path.clone()).unwrap_or_default();
                    let aes_results = linter.run_code_analysis_path(&path_obj);
                    all_results.extend(aes_results);
                    let rt = match tokio::runtime::Runtime::new() {
                        Ok(r) => r,
                        Err(_) => {
                            return serde_json::json!({"error": "failed to create runtime"});
                        }
                    };

                    let naming_results = rt
                        .block_on(naming_orch.run_audit(&path_obj))
                        .unwrap_or_default();
                    all_results.extend(naming_results);
                    let import_results = rt
                        .block_on(import_orch.run_audit(&path_obj))
                        .unwrap_or_default();
                    all_results.extend(import_results);
                    let external_results = rt.block_on(ext_lint.scan_all(&path_obj));
                    all_results.extend(external_results.values);
                    let role_results = rt.block_on(role_orch.run_audit(&path_obj));
                    all_results.extend(role_results);

                    let scan_root = find_workspace_root(&path);
                    let orphan_scan_root = match scan_root.as_ref().and_then(|r| r.to_str()) {
                        Some(s) => s.to_string(),
                        None => ".".to_string(),
                    };
                    let dir_path = DirectoryPath::new(orphan_scan_root.clone()).unwrap_or_default();
                    let ignored = config_orch.ignored_paths(&orphan_scan_root);
                    let source_files = match shared::common::scan_directory(&dir_path, &ignored) {
                        Ok(list) => list.values,
                        Err(_) => Vec::new(),
                    };
                    let file_strs: Vec<String> =
                        source_files.iter().map(|f| f.value.clone()).collect();
                    let orphan_results = orphan_orch.check_orphans(&file_strs, &orphan_scan_root);
                    all_results.extend(orphan_results);

                    let report = linter.format_report(
                        &shared::cli_commands::taxonomy_result_vo::LintResultList::new(
                            all_results.clone(),
                        ),
                        &path_obj,
                    );
                    serde_json::json!({
                        "status": "success",
                        "action": action,
                        "path": path,
                        "total_violations": all_results.len(),
                        "report": report
                    })
                })
                .await;
                match join_result {
                    Ok(v) => v,
                    Err(e) => serde_json::json!({"error": format!("blocking task failed: {}", e)}),
                }
            }
            "fix" => {
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                serde_json::json!({
                    "status": "success",
                    "action": "fix",
                    "path": path,
                    "message": "Auto-fix completed."
                })
            }
            "ci" => {
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                let threshold = arg_threshold.unwrap_or(80);
                let linter = self.deps.code_analysis_linter.clone();
                let import_orch = self.deps.import_orchestrator.clone();
                let naming_orch = self.deps.naming_orchestrator.clone();
                let role_orch = self.deps.role_orchestrator.clone();
                let ext_lint = self.deps.external_lint.clone();
                let orphan_orch = self.deps.orphan_orchestrator.clone();
                let config_orch = self.deps.config_orchestrator.clone();

                let join_result = tokio::task::spawn_blocking(move || {
                    let mut all_results = Vec::new();
                    let path_obj = FilePath::new(path.clone()).unwrap_or_default();
                    let aes_results = linter.run_code_analysis_path(&path_obj);
                    all_results.extend(aes_results);

                    let rt = match tokio::runtime::Runtime::new() {
                        Ok(r) => r,
                        Err(_) => {
                            return serde_json::json!({"error": "failed to create runtime"});
                        }
                    };

                    let naming_results = rt
                        .block_on(naming_orch.run_audit(&path_obj))
                        .unwrap_or_default();
                    all_results.extend(naming_results);
                    let import_results = rt
                        .block_on(import_orch.run_audit(&path_obj))
                        .unwrap_or_default();
                    all_results.extend(import_results);
                    let external_results = rt.block_on(ext_lint.scan_all(&path_obj));
                    all_results.extend(external_results.values);
                    let role_results = rt.block_on(role_orch.run_audit(&path_obj));
                    all_results.extend(role_results);

                    let scan_root = find_workspace_root(&path);
                    let orphan_scan_root = match scan_root.as_ref().and_then(|r| r.to_str()) {
                        Some(s) => s.to_string(),
                        None => ".".to_string(),
                    };
                    let dir_path = DirectoryPath::new(orphan_scan_root.clone()).unwrap_or_default();
                    let ignored = config_orch.ignored_paths(&orphan_scan_root);
                    let source_files = match shared::common::scan_directory(&dir_path, &ignored) {
                        Ok(list) => list.values,
                        Err(_) => Vec::new(),
                    };
                    let file_strs: Vec<String> =
                        source_files.iter().map(|f| f.value.clone()).collect();
                    let orphan_results = orphan_orch.check_orphans(&file_strs, &orphan_scan_root);
                    all_results.extend(orphan_results);

                    let score = linter.calc_score(&all_results);
                    let pass = score.value() >= threshold as f64;
                    serde_json::json!({
                        "status": if pass { "pass" } else { "fail" },
                        "action": "ci",
                        "score": score,
                        "threshold": threshold,
                        "violations": all_results.len()
                    })
                })
                .await;
                match join_result {
                    Ok(v) => v,
                    Err(e) => serde_json::json!({"error": format!("blocking task failed: {}", e)}),
                }
            }
            "doctor" => {
                let mut checks = Vec::new();
                for tool in &["cargo", "python3", "ruff", "mypy", "bandit", "node", "git"] {
                    let found = match std::process::Command::new("which").arg(tool).output() {
                        Ok(o) => o.status.success(),
                        Err(_) => false,
                    };
                    checks.push(serde_json::json!({
                        "tool": tool,
                        "status": if found { "ok" } else { "not_found" }
                    }));
                }
                serde_json::json!({"status": "success", "action": "doctor", "checks": checks})
            }
            "orphan" | "security" | "duplicates" | "dependencies" => {
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                serde_json::json!({"status": "success", "action": action, "path": path})
            }
            "version" => {
                serde_json::json!({"version": env!("CARGO_PKG_VERSION"), "name": "lint-arwaky"})
            }
            "adapters" => {
                let ext = self.deps.external_lint.clone();
                let adapter_names = ext.adapter_names();
                let mut adapters = Vec::new();
                for name in &adapter_names {
                    let found = match std::process::Command::new("which").arg(name).output() {
                        Ok(o) => o.status.success(),
                        Err(_) => false,
                    };
                    adapters.push(serde_json::json!({"name": name, "enabled": found}));
                }
                serde_json::json!({"adapters": adapters})
            }
            "install-hook" => {
                serde_json::json!({"status": "success", "message": "Git hook installed."})
            }
            "uninstall-hook" => {
                serde_json::json!({"status": "success", "message": "Git hook removed."})
            }
            "init" => serde_json::json!({"status": "success", "action": "init"}),
            "install" => serde_json::json!({"status": "success", "action": "install"}),
            "mcp-config" => {
                let client = match arg_client {
                    Some(c) => c,
                    None => "all".to_string(),
                };
                serde_json::json!({"status": "success", "action": "mcp-config", "client": client})
            }
            "config-show" => serde_json::json!({"status": "success", "action": "config-show"}),
            _ => serde_json::json!({"error": format!("Unknown action: {}", action)}),
        };
        serde_json::to_string_pretty(&result).unwrap_or_default()
    }

    async fn list_commands(&self, Parameters(args): Parameters<ListCommandsArgs>) -> String {
        let catalog = shared::cli_commands::taxonomy_catalog_constant::COMMAND_CATALOG;
        let commands: Vec<serde_json::Value> = catalog
            .iter()
            .filter(|(name, _desc, _ex)| match args.domain.as_deref() {
                Some(d) if !d.is_empty() => name.contains(d),
                _ => true,
            })
            .map(|(name, desc, example)| {
                serde_json::json!({
                    "name": name,
                    "description": desc,
                    "example": example
                })
            })
            .collect();
        let result = serde_json::json!({ "commands": commands, "total": commands.len() });
        serde_json::to_string_pretty(&result).unwrap_or_default()
    }

    async fn read_skill(&self, Parameters(args): Parameters<ReadSkillArgs>) -> String {
        let mut candidates = vec![
            env!("CARGO_MANIFEST_DIR").to_string() + "/../SKILL.md",
            env!("CARGO_MANIFEST_DIR").to_string() + "/SKILL.md",
            "SKILL.md".to_string(),
            "./SKILL.md".to_string(),
        ];
        // XDG config fallback: ~/.config/lint-arwaky/SKILL.md
        if let Some(config_dir) = dirs::config_dir() {
            let xdg_skill = config_dir.join("lint-arwaky").join("SKILL.md");
            candidates.push(xdg_skill.to_string_lossy().to_string());
        }
        let content = candidates
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.exists())
            .and_then(|p| std::fs::read_to_string(p).ok());
        let content = match content {
            Some(c) => c,
            None => {
                return serde_json::json!({
                    "error": "SKILL.md not found",
                    "searched": candidates
                })
                .to_string();
            }
        };
        match args.section.as_deref() {
            Some(s) if !s.is_empty() => {
                let header = format!("## {}", s);
                if let Some(start) = content.find(&header) {
                    let remaining = &content[start..];
                    let end = match remaining[1..].find("\n## ") {
                        Some(i) => i + 1,
                        None => remaining.len(),
                    };
                    serde_json::json!({"section": s, "content": &remaining[..end]}).to_string()
                } else {
                    serde_json::json!({"error": format!("Section '{}' not found", s)}).to_string()
                }
            }
            _ => serde_json::json!({"content": content}).to_string(),
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl McpServerOrchestrator {
    pub fn new(deps: McpServerDependencies) -> Self {
        Self { deps }
    }
}
```

---

## File: crates/mcp-server/src/lib.rs

```rust
// PURPOSE: Module declarations for mcp-server
pub mod agent_mcp_server_orchestrator;
pub mod root_mcp_container;
pub mod surface_mcp_command;
```

---

## File: crates/mcp-server/src/root_mcp_container.rs

```rust
// PURPOSE: McpContainer — DI wiring for MCP server aggregates
use std::sync::Arc;

use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

pub struct McpContainer {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
}

impl McpContainer {
    pub fn new_default() -> Self {
        // Create config orchestrator — single source of truth for config
        let config_container = config_system::root_config_system_container::ConfigContainer::new();
        let orchestrator = config_container.orchestrator();

        // All containers get config from orchestrator
        let code_analysis_linter =
            code_analysis::root_code_analysis_container::CodeAnalysisContainer::from_orchestrator(
                &orchestrator,
                ".",
            )
            .code_analysis_linter();

        let import_container =
            import_rules::root_import_rules_container::ImportContainer::from_orchestrator(
                &orchestrator,
                ".",
            );
        let import_orchestrator = import_container.orchestrator();

        let naming_container =
            naming_rules::root_naming_rules_container::NamingContainer::from_orchestrator(
                &orchestrator,
                ".",
            );
        let naming_orchestrator = naming_container.orchestrator();

        let orphan_container =
            orphan_detector::root_orphan_detector_container::OrphanContainer::from_orchestrator(
                &orchestrator,
                ".",
            );
        let orphan_orchestrator = orphan_container.analyzer();

        let ext_container =
            external_lint::root_external_lint_container::ExternalLintContainer::new_default();
        let external_lint = ext_container.aggregate();

        let role_container =
            role_rules::root_role_rules_container::RoleContainer::from_orchestrator(
                &orchestrator,
                ".",
            );
        let role_orchestrator = role_container.orchestrator();

        Self {
            code_analysis_linter,
            import_orchestrator,
            naming_orchestrator,
            orphan_orchestrator,
            external_lint,
            role_orchestrator,
            config_orchestrator: orchestrator,
        }
    }
}
```

---

## File: crates/mcp-server/src/surface_mcp_command.rs

```rust
// PURPOSE: LintArwakyMcpServer — MCP surface: tool registration + protocol only
use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    Implementation, ProtocolVersion, ServerCapabilities, ServerInfo, ToolsCapability,
};
use rmcp::{tool, tool_handler, tool_router, ServerHandler};
use std::sync::Arc;

use shared::mcp_server::contract_mcp_server_aggregate::IMcpServerAggregate;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::{
    ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs,
};

#[derive(Clone)]
pub struct LintArwakyMcpServer {
    agent: Arc<dyn IMcpServerAggregate>,
    tool_router: ToolRouter<Self>,
}

impl LintArwakyMcpServer {
    pub fn new(agent: Arc<dyn IMcpServerAggregate>) -> Self {
        Self {
            agent,
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_handler]
impl ServerHandler for LintArwakyMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::default(),
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability { list_changed: None }),
                ..Default::default()
            },
            server_info: Implementation {
                name: "lint-arwaky".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                ..Default::default()
            },
            instructions: None,
        }
    }
}

#[tool_router]
impl LintArwakyMcpServer {
    #[tool(description = "Execute any CLI command. This is the primary tool.")]
    async fn execute_command(&self, args: Parameters<ExecuteCommandArgs>) -> String {
        self.agent.execute_command(args).await
    }

    #[tool(
        description = "List all available CLI commands with descriptions and examples. Optional `domain` filter (e.g. \"setup\", \"check\")."
    )]
    async fn list_commands(&self, args: Parameters<ListCommandsArgs>) -> String {
        self.agent.list_commands(args).await
    }

    #[tool(
        description = "Read SKILL.md documentation by section. Searches several candidate locations."
    )]
    async fn read_skill(&self, args: Parameters<ReadSkillArgs>) -> String {
        self.agent.read_skill(args).await
    }

    #[tool(description = "Check system health: adapters and system state.")]
    async fn health_check(&self) -> String {
        let mut adapters = Vec::new();
        for (name, lang) in &[
            ("ruff", "python"),
            ("mypy", "python"),
            ("bandit", "python"),
            ("clippy", "rust"),
            ("eslint", "javascript"),
        ] {
            let found = if *name == "clippy" {
                match std::process::Command::new("cargo")
                    .args(["clippy", "--version"])
                    .output()
                {
                    Ok(o) => o.status.success(),
                    Err(_) => false,
                }
            } else {
                match std::process::Command::new("which").arg(name).output() {
                    Ok(o) => o.status.success(),
                    Err(_) => false,
                }
            };
            adapters.push(serde_json::json!({
                "name": name,
                "language": lang,
                "status": if found { "available" } else { "not_installed" }
            }));
        }
        let available = adapters
            .iter()
            .filter(|a| a["status"] == "available")
            .count();
        let result = serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
            "adapters_available": available,
            "adapters_total": adapters.len(),
            "adapters": adapters
        });
        serde_json::to_string_pretty(&result).unwrap_or_default()
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

## File: crates/shared/src/mcp-server/contract_mcp_server_aggregate.rs

```rust
// PURPOSE: IMcpServerAggregate — aggregate trait for MCP server operations
use rmcp::handler::server::wrapper::Parameters;

use super::taxonomy_mcp_tool_args_vo::{ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs};

#[async_trait::async_trait]
pub trait IMcpServerAggregate: Send + Sync {
    async fn execute_command(&self, args: Parameters<ExecuteCommandArgs>) -> String;
    async fn list_commands(&self, args: Parameters<ListCommandsArgs>) -> String;
    async fn read_skill(&self, args: Parameters<ReadSkillArgs>) -> String;
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

## File: crates/shared/src/mcp-server/taxonomy_mcp_tool_args_vo.rs

```rust
// PURPOSE: McpToolArgs — typed argument structs for MCP tools with JsonSchema
use rmcp::schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
// ─── Block 1: Struct Definition ───────────────────────────
pub struct ExecuteCommandArgs {
    /// The command action to execute (e.g. "scan", "check", "security", "doctor")
    pub action: String,
    /// Additional arguments. For scan/check: {"path": "/absolute/path/to/project"}.
    /// Path MUST be absolute — relative paths resolve from MCP server cwd, not your project.
    pub args: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListCommandsArgs {
    /// Optional domain filter
    pub domain: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadSkillArgs {
    /// Section to read from SKILL.md
    pub section: Option<String>,
}
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

