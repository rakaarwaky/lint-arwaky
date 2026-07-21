# Crate: import-rules (v1.10.106)

This document contains the source code for feature crate `import-rules` along with its corresponding and imported definitions from the `shared` crate.

## Problem Statement

The following issues were detected by `lint-arwaky-cli scan`:

```
============================================================
  AES Architecture Compliance Report
============================================================
  Project: /home/raka/mcp-arwaky/lint-arwaky/crates/import-rules
  Violations: 0
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
- [crates/shared/src/auto-fix/taxonomy_fix_applied_event.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/taxonomy_fix_applied_event.rs)
- [crates/shared/src/cli-commands/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/mod.rs)
- [crates/shared/src/cli-commands/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_severity_vo.rs)
- [crates/shared/src/common/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/mod.rs)
- [crates/shared/src/common/taxonomy_common_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_vo.rs)
- [crates/shared/src/common/taxonomy_definition_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_definition_vo.rs)
- [crates/shared/src/common/taxonomy_layer_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_layer_vo.rs)
- [crates/shared/src/common/taxonomy_message_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_message_vo.rs)
- [crates/shared/src/common/taxonomy_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_name_vo.rs)
- [crates/shared/src/common/taxonomy_path_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_path_vo.rs)
- [crates/shared/src/common/taxonomy_paths_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_paths_vo.rs)
- [crates/shared/src/common/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_severity_vo.rs)
- [crates/shared/src/common/taxonomy_source_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_source_vo.rs)
- [crates/shared/src/common/utility_file.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/utility_file.rs)
- [crates/shared/src/common/utility_layer_detector.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/utility_layer_detector.rs)
- [crates/shared/src/config-system/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/mod.rs)
- [crates/shared/src/config-system/taxonomy_config_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_config_vo.rs)
- [crates/shared/src/config-system/utility_config_merger.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/utility_config_merger.rs)
- [crates/shared/src/import-rules/contract_cycle_import_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_cycle_import_protocol.rs)
- [crates/shared/src/import-rules/contract_dummy_import_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_dummy_import_protocol.rs)
- [crates/shared/src/import-rules/contract_import_forbidden_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_import_forbidden_protocol.rs)
- [crates/shared/src/import-rules/contract_import_mandatory_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_import_mandatory_protocol.rs)
- [crates/shared/src/import-rules/contract_import_runner_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_import_runner_aggregate.rs)
- [crates/shared/src/import-rules/contract_unused_import_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_unused_import_protocol.rs)
- [crates/shared/src/import-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/mod.rs)
- [crates/shared/src/import-rules/taxonomy_cycle_color_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_cycle_color_vo.rs)
- [crates/shared/src/import-rules/taxonomy_dependency_edge_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_dependency_edge_vo.rs)
- [crates/shared/src/import-rules/taxonomy_import_constant.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_import_constant.rs)
- [crates/shared/src/import-rules/taxonomy_import_rule_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_import_rule_vo.rs)
- [crates/shared/src/import-rules/taxonomy_language_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_language_vo.rs)
- [crates/shared/src/import-rules/taxonomy_violation_import_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/taxonomy_violation_import_vo.rs)
- [crates/shared/src/import-rules/utility_cycle_detector.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/utility_cycle_detector.rs)
- [crates/shared/src/import-rules/utility_dummy_detector.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/utility_dummy_detector.rs)
- [crates/shared/src/import-rules/utility_file_read.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/utility_file_read.rs)
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

| Category                      | Concern        | Responsibility                                 |
| ----------------------------- | -------------- | ---------------------------------------------- |
| **Business Logic**      | Validation     | Check domain conditions or input correctness   |
|                               | Computation    | Calculate scores, totals, or derived values    |
|                               | Transformation | Map, filter, reduce, or reshape data           |
|                               | Resolution     | Apply rules and decide outcomes                |
|                               | Assessment     | Judge severity, compliance, grade, or quality  |
| **External Adaptation** | Repository     | Fetch or persist domain entities to a database |
|                               | Integration    | Communicate with third-party services or APIs  |
|                               | Provider       | Generate data from external systems            |

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

Agent may depend only on Taxonomy and Contract.

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
- Agent must not use and must be completely ignorant of Capabilities and Utility implementations.
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

| Group            | Roles                             | Dependencies                           | Rule                                            |
| ---------------- | --------------------------------- | -------------------------------------- | ----------------------------------------------- |
| Smart surfaces   | command, controller, page, router | Taxonomy, Contract Aggregate, Utility | May initiate feature behavior through aggregate |
| Utility surfaces | hook, store, action, screen       | Taxonomy only                          | Support smart surfaces but must not import them |
| Passive surfaces | component, view, layout           | Taxonomy only                          | Presentation-only, no logic or orchestration    |

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
name = "import_rules-lint-arwaky"
version = "1.10.106"
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

[dev-dependencies]
```

---

## File: crates/import-rules/FRD.md

```rust
# FRD — import-rules

## Feature Goal

The import-rules crate enforces correct structural boundaries and unidirectional dependency flows. It prevents spaghetti architecture, circular dependencies, and dead/unused imports by validating every import statement against a predefined layer-hierarchy matrix.

## Requirements & Scope

- AES201 Layer Dependency Violation (Unidirectional Flow)
  - Requirement: Restricts imports based on the layer hierarchy. Lower layers (e.g., taxonomy_, contract_) must never import higher layers (e.g., capabilities_, utility_, agent_, surface_).
  - Layer Boundary: utility_ and capabilities_ must not import each other directly; .
- AES202 Mandatory Layer Imports
  - Requirement: Verifies that specific layers contain required imports (e.g., ensuring a capability layer file correctly imports its corresponding contract trait, or that a surface entry imports its container).
- AES203 Unused Import Detection
  - Requirement: Detects and flags imported symbols that are never referenced anywhere within the file body.
- AES204 Dummy or Forbidden Imports
  - Requirement: Detects imports that point to mock, dummy, or forbidden packages/modules in production configurations.
- AES205 Circular Dependency Cycle Detection
  - Requirement: Builds a dependency graph of imports across all workspace files and detects cycles (e.g., File A imports B, B imports C, C imports A). Circular dependencies must be flagged.

## Success Indicators

- [ ] Zero dependency cycles — all import cycle loops are detected and resolved.
- [ ] Strict unidirectional flow — complete blocking of cross-layer violations (e.g., taxonomy files importing orchestration layer code).
- [ ] Cleaner namespace — prompt warning of unused symbols to maintain clean, lean namespaces.
- [ ] High performance — graph cycle detection runs within milliseconds using optimized cycle-finding algorithms (e.g., Tarjan's or simple DFS-based cycle detection).
```

---

## File: crates/import-rules/src/agent_import_orchestrator.rs

```rust
// PURPOSE: ImportOrchestrator — agent that orchestrates import rule checks
// Uses new protocol interfaces — no IAnalyzer, no IArchImportProtocol.
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::taxonomy_definition_vo::LayerMapVO;
use std::path::Path;
use std::sync::Arc;

pub fn str_or<'a>(opt: Option<&'a str>, fallback: &'a str) -> &'a str {
    opt.map_or(fallback, |s| s)
}

pub struct ImportOrchestrator {
    mandatory: Arc<dyn IImportMandatoryProtocol>,
    forbidden: Arc<dyn IImportForbiddenProtocol>,
    unused: Arc<dyn IUnusedImportProtocol>,
    cycle: Arc<dyn ICycleImportProtocol>,
    config: ArchitectureConfig,
    layer_map: LayerMapVO,
    ignored_paths: Vec<String>,
}

impl ImportOrchestrator {
    pub fn new(
        mandatory: Arc<dyn IImportMandatoryProtocol>,
        forbidden: Arc<dyn IImportForbiddenProtocol>,
        unused: Arc<dyn IUnusedImportProtocol>,
        cycle: Arc<dyn ICycleImportProtocol>,
    ) -> Self {
        let (merged_layers, _) = shared::config_system::utility_config_merger::merge_config(
            &ArchitectureConfig::default(),
        );
        let config = ArchitectureConfig::default();
        let layer_map = LayerMapVO::new(merged_layers.clone());
        let ignored_paths: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();
        Self {
            mandatory,
            forbidden,
            unused,
            cycle,
            config,
            layer_map,
            ignored_paths,
        }
    }

    pub fn with_config(config: ArchitectureConfig) -> Self {
        let (merged_layers, _) =
            shared::config_system::utility_config_merger::merge_config(&config);
        let mut config = config;
        config.layers = merged_layers.clone();
        let layer_map = LayerMapVO::new(merged_layers);
        let ignored_paths: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();
        let empty_mandatory: Arc<dyn IImportMandatoryProtocol> = Arc::new(
            crate::capabilities_import_mandatory_checker::ArchImportMandatoryChecker::new(),
        );
        let empty_forbidden: Arc<dyn IImportForbiddenProtocol> = Arc::new(
            crate::capabilities_import_forbidden_checker::ArchImportForbiddenChecker::new(),
        );
        let empty_unused: Arc<dyn IUnusedImportProtocol> =
            Arc::new(crate::capabilities_import_unused_checker::UnusedImportRuleChecker::new());
        let empty_cycle: Arc<dyn ICycleImportProtocol> =
            Arc::new(crate::capabilities_cycle_import_analyzer::DependencyCycleAnalyzer::new());
        Self {
            mandatory: empty_mandatory,
            forbidden: empty_forbidden,
            unused: empty_unused,
            cycle: empty_cycle,
            config,
            layer_map,
            ignored_paths,
        }
    }

    fn is_ignored(&self, p: &Path) -> bool {
        let s = p.to_string_lossy();
        let dir_name = p
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        shared::common::utility_file::is_path_ignored(&s, &self.ignored_paths)
            || match dir_name.strip_prefix('.') {
                Some(n) => self.ignored_paths.iter().any(|i| i.contains(n)),
                None => false,
            }
    }

    fn collect_files(&self, target: &FilePath) -> FilePathList {
        let path = Path::new(target.value());
        let mut files = Vec::new();
        if path.is_dir() {
            self.walk_dir(path, &mut files, true);
        } else if path.is_file() {
            if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                files.push(fp);
            }
        }
        FilePathList::new(files)
    }

    fn walk_dir(&self, dir: &Path, files: &mut Vec<FilePath>, is_subdir: bool) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if is_subdir && self.is_ignored(&path) {
                        continue;
                    }
                    self.walk_dir(&path, files, true);
                } else if path.is_file() {
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

#[async_trait]
impl IImportRunnerAggregate for ImportOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        if !self.config.enabled.value {
            return Vec::new();
        }
        let mut results = LintResultList::new(Vec::new());
        let files = self.collect_files(target);
        let first_component = str_or(target.value().split('/').next(), ".");
        let root_dir = match FilePath::new(first_component.to_string()) {
            Ok(p) => p,
            Err(_) => return vec![],
        };

        let (mandatory_results, forbidden_results) = tokio::join!(
            async {
                let mut r = LintResultList::new(Vec::new());
                self.mandatory
                    .run_mandatory_imports(&self.config, &self.layer_map, &files, &root_dir, &mut r)
                    .await;
                r
            },
            async {
                let mut r = LintResultList::new(Vec::new());
                self.forbidden
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
        results.values.extend(mandatory_results.values);
        results.values.extend(forbidden_results.values);

        for file in files.iter() {
            let file_path = file.value();
            if let Ok(content) = std::fs::read_to_string(file_path) {
                self.unused
                    .check_unused_imports(file_path, &content, &mut results.values);
            }
        }

        self.cycle
            .check_cycles(
                &self.config,
                &self.layer_map,
                &files,
                &root_dir,
                &mut results,
            )
            .await;
        results.values
    }

    fn name(&self) -> &str {
        "import-rules"
    }
}
```

---

## File: crates/import-rules/src/capabilities_cycle_import_analyzer.rs

```rust
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::utility_layer_detector;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::DependencyEdge;
use shared::import_rules::{
    utility_cycle_detector, utility_file_read, utility_import_module_parser,
};
use shared::taxonomy_definition_vo::LayerMapVO;
use shared::taxonomy_message_vo::LintMessage;
use std::collections::HashMap;

use async_trait::async_trait;

// PURPOSE: DependencyCycleAnalyzer — AES205: circular dependency detection
// Uses utility functions directly — no IImportParserProtocol, no IAnalyzer.

// ─── Block 1: Struct Definition ───────────────────────────

pub struct DependencyCycleAnalyzer;

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
        files: &shared::common::taxonomy_paths_vo::FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let file_strs: Vec<String> = files.values.iter().map(|f| f.to_string()).collect();
        let cycle_violations = self._scan(config, layer_map, &file_strs, &root_dir.to_string());
        results.values.extend(cycle_violations);
    }

    fn detect_cycle_edges(
        &self,
        edges: &[DependencyEdge],
    ) -> Vec<shared::taxonomy_name_vo::SymbolName> {
        utility_cycle_detector::detect_cycle_edges(edges)
    }

    fn normalize_to_layer(&self, name: &str) -> shared::taxonomy_layer_vo::LayerNameVO {
        shared::taxonomy_layer_vo::LayerNameVO::new(name.split('_').next().unwrap_or(name))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for DependencyCycleAnalyzer {
    fn default() -> Self {
        Self
    }
}

impl DependencyCycleAnalyzer {
    pub fn new() -> Self {
        Self
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
        let mut edges = Vec::new();
        let mut file_by_layer: HashMap<String, String> = HashMap::new();

        for file in files {
            let file_fp = match FilePath::new(file.clone()) {
                Ok(p) => p,
                Err(_) => continue,
            };
            let basename = file_fp.basename();
            if let Some(rule) = aes205_rule {
                if rule.exceptions.values.contains(&basename.to_string()) {
                    continue;
                }
            }
            let content = match utility_file_read::read_file(file) {
                Some(c) => c,
                None => continue,
            };

            let filename = utility_layer_detector::extract_filename(file);
            let file_layer = match utility_layer_detector::detect_layer_from_prefix(filename) {
                Some(l) => {
                    let specialized =
                        utility_layer_detector::resolve_specialized_layer(&l, file, &layer_keys);
                    match specialized.split('(').next() {
                        Some(p) => p.to_string(),
                        None => specialized,
                    }
                }
                None => continue,
            };

            let modules = utility_import_module_parser::extract_import_modules(&content);
            let mut has_cross_layer = false;
            for module in modules {
                let module_value = module.value();
                let is_crate_import = module_value.starts_with("crate::")
                    || module_value.starts_with("lint_arwaky::");
                let layer_prefixes = [
                    "taxonomy_",
                    "contract_",
                    "capabilities_",
                    "utility_",
                    "agent_",
                    "surface_",
                ];
                let layer_names = [
                    "taxonomy",
                    "contract",
                    "capabilities",
                    "utility",
                    "agent",
                    "surface",
                ];
                let is_cross_layer_crate = if is_crate_import {
                    let stripped = module_value
                        .strip_prefix("crate::")
                        .or_else(|| module_value.strip_prefix("lint_arwaky::"))
                        .unwrap_or("");
                    let first_segment = stripped.split("::").next().unwrap_or("");
                    layer_prefixes.iter().any(|p| stripped.starts_with(p))
                        || layer_names.contains(&first_segment)
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
                let module_layer_names: Vec<String> =
                    layer_map.values.keys().map(|k| k.to_string()).collect();
                if let Some(target_layer) =
                    utility_layer_detector::detect_module_layer(module_path, &module_layer_names)
                {
                    let target_layer_str = match target_layer.split('(').next() {
                        Some(p) => p.to_string(),
                        None => target_layer,
                    };
                    if target_layer_str != file_layer {
                        edges.push(DependencyEdge::new(file_layer.clone(), target_layer_str));
                        has_cross_layer = true;
                    }
                }
            }
            if has_cross_layer {
                file_by_layer
                    .entry(file_layer.clone())
                    .or_insert_with(|| file.clone());
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
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::utility_layer_detector;
use shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol;
use shared::import_rules::taxonomy_language_vo::LanguageVO;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::utility_dummy_detector;
use shared::taxonomy_layer_vo::{Identity, LayerNameVO};
use shared::taxonomy_name_vo::SymbolName;

// PURPOSE: DummyImportChecker — AES204: detect dummy imports, dummy functions, dummy trait impls
// Uses utility functions directly — no IImportParserProtocol, no IAnalyzer.

// ─── Block 1: Struct Definition ───────────────────────────

pub struct DummyImportChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IDummyImportCheckerProtocol for DummyImportChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES204")
    }

    fn check_dummy_imports(
        &self,
        file: &FilePath,
        content: &shared::common::taxonomy_source_vo::ContentString,
        violations: &mut Vec<LintResult>,
        _root_dir: &FilePath,
    ) {
        let layer_map = shared::taxonomy_definition_vo::LayerMapVO::default();
        self._check_dummy_imports(file.value(), content.value(), violations, &layer_map);
    }

    fn check_dummy_functions(
        &self,
        file: &FilePath,
        content: &shared::common::taxonomy_source_vo::ContentString,
        violations: &mut Vec<LintResult>,
        _root_dir: &FilePath,
    ) {
        let layer_map = shared::taxonomy_definition_vo::LayerMapVO::default();
        self._check_dummy_functions(file.value(), content.value(), violations, &layer_map);
    }

    fn check_dummy_impls(
        &self,
        file: &FilePath,
        content: &shared::common::taxonomy_source_vo::ContentString,
        violations: &mut Vec<LintResult>,
        _root_dir: &FilePath,
    ) {
        let layer_map = shared::taxonomy_definition_vo::LayerMapVO::default();
        self._check_dummy_impls(file.value(), content.value(), violations, &layer_map);
    }

    fn check_taxonomy_intent(
        &self,
        file: &FilePath,
        content: &shared::common::taxonomy_source_vo::ContentString,
        violations: &mut Vec<LintResult>,
        _root_dir: &FilePath,
    ) {
        let layer_map = shared::taxonomy_definition_vo::LayerMapVO::default();
        self._check_taxonomy_intent(file.value(), content.value(), violations, &layer_map);
    }

    fn check_layer_contract_intent(
        &self,
        _file: &FilePath,
        _content: &shared::common::taxonomy_source_vo::ContentString,
        _violations: &mut Vec<LintResult>,
        _root_dir: &FilePath,
    ) {
    }

    fn check_surface_logic(
        &self,
        file: &FilePath,
        content: &shared::common::taxonomy_source_vo::ContentString,
        violations: &mut Vec<LintResult>,
        _root_dir: &FilePath,
    ) {
        self._check_surface_logic(file.value(), content.value(), violations);
    }
}

impl Default for DummyImportChecker {
    fn default() -> Self {
        Self
    }
}

impl DummyImportChecker {
    pub fn new() -> Self {
        Self
    }

    fn _detect_layer(
        &self,
        file: &str,
        layer_map: &shared::taxonomy_definition_vo::LayerMapVO,
    ) -> String {
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();
        let filename: &str = utility_layer_detector::extract_filename(file);
        match utility_layer_detector::detect_layer_from_prefix(filename) {
            Some(base) => {
                utility_layer_detector::resolve_specialized_layer(&base, file, &layer_keys)
            }
            None => "any".to_string(),
        }
    }

    fn _check_dummy_imports(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        layer_map: &shared::taxonomy_definition_vo::LayerMapVO,
    ) {
        let lines: Vec<&str> = content.lines().collect();
        let lang: LanguageVO = LanguageVO::from_path(file);
        let dummy_ranges = utility_dummy_detector::dummy_function_ranges(&lines, lang);
        let dummy_impl_traits: Vec<String> =
            utility_dummy_detector::dummy_impl_traits_with_lines(&lines)
                .into_iter()
                .map(|(t, _)| t.value().to_string())
                .collect();
        let layer_name: String = self._detect_layer(file, layer_map);

        for (symbol, line_no) in utility_dummy_detector::imported_symbols(&lines, lang) {
            let symbol_str = symbol.value().to_string();
            if utility_dummy_detector::symbol_used_real(
                &lines,
                &symbol_str,
                &dummy_ranges,
                &dummy_impl_traits,
            ) {
                continue;
            }
            violations.push(LintResult::new_arch(file, line_no.value() as usize, "AES204", Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(layer_name.clone()),
                    import_type: SymbolName::new(symbol_str),
                    intent: SymbolName::new("Use imported symbols in real logic, not only in dummy functions or stubs"),
                    reason: Some(shared::taxonomy_message_vo::LintMessage::new(
                        "Imported symbols placed inside _use_ dummy functions are dead code — they exist only to suppress unused-import warnings."
                    )),
                }.to_string(),
            ));
        }
    }

    fn _check_dummy_functions(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        layer_map: &shared::taxonomy_definition_vo::LayerMapVO,
    ) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = LanguageVO::from_path(file);
        let layer_name = self._detect_layer(file, layer_map);

        for (start, end) in utility_dummy_detector::dummy_function_ranges(&lines, lang) {
            violations.push(LintResult::new_arch(
                file,
                start.value() as usize,
                "AES204",
                Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(layer_name.clone()),
                    import_type: SymbolName::new("_use_mandatory_imports"),
                    intent: SymbolName::new(
                        "Remove dummy functions that exist only to silence unused import checks",
                    ),
                    reason: Some(shared::taxonomy_message_vo::LintMessage::new(format!(
                        "Dummy function range ends at line {}",
                        end
                    ))),
                }
                .to_string(),
            ));
        }
    }

    fn _check_dummy_impls(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        layer_map: &shared::taxonomy_definition_vo::LayerMapVO,
    ) {
        let lines: Vec<&str> = content.lines().collect();
        let layer_name = self._detect_layer(file, layer_map);

        for (trait_name, start) in utility_dummy_detector::dummy_impl_traits_with_lines(&lines) {
            violations.push(LintResult::new_arch(
                file,
                start.value() as usize,
                "AES204",
                Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(layer_name.clone()),
                    import_type: SymbolName::new(trait_name.value().to_string()),
                    intent: SymbolName::new(
                        "Implement contract methods with real behavior instead of empty/todo stubs",
                    ),
                    reason: Some(shared::taxonomy_message_vo::LintMessage::new(
                        "Trait implementations with empty bodies violate the contract abstraction.",
                    )),
                }
                .to_string(),
            ));
        }
    }

    fn _check_taxonomy_intent(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        layer_map: &shared::taxonomy_definition_vo::LayerMapVO,
    ) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = LanguageVO::from_path(file);
        let _layer_name = self._detect_layer(file, layer_map);
        let dummy_ranges = utility_dummy_detector::dummy_function_ranges(&lines, lang);
        let dummy_impl_traits: Vec<String> =
            utility_dummy_detector::dummy_impl_traits_with_lines(&lines)
                .into_iter()
                .map(|(t, _)| t.value().to_string())
                .collect();
        let imported = utility_dummy_detector::imported_symbols(&lines, lang);

        let mut has_dummy_function = false;
        let mut dummy_function_line = 0;
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            let is_dummy = match lang {
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
                    match lang {
                        LanguageVO::Rust => {
                            t.contains("use shared::taxonomy_")
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
                &dummy_ranges,
                &dummy_impl_traits,
            )
        });

        if !has_real_usage {
            let has_taxonomy_import = lines.iter().any(|l| {
                let t = l.trim();
                match lang {
                    LanguageVO::Rust => {
                        t.contains("use shared::taxonomy_")
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
                        reason: Some(shared::taxonomy_message_vo::LintMessage::new(
                            "Taxonomy VOs encode domain concepts — using raw primitives defeats the purpose."
                        )),
                    }.to_string(),
                ));
            }
        }
    }

    fn _check_aggregate_intent(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = LanguageVO::from_path(file);
        let imported = utility_dummy_detector::imported_symbols(&lines, lang);
        let aggregate_types: Vec<String> = imported
            .into_iter()
            .filter(|(s, _)| s.value().ends_with("Aggregate"))
            .map(|(s, _)| s.value().to_string())
            .collect();

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            let is_phantom = match lang {
                LanguageVO::Rust => trimmed.contains("PhantomData"),
                LanguageVO::Python => trimmed.contains("TYPE_CHECKING"),
                LanguageVO::JavaScript => {
                    trimmed.contains("@ts-ignore") || trimmed.contains("@ts-expect")
                }
                LanguageVO::Unknown => false,
            };
            if is_phantom {
                for agg_type in &aggregate_types {
                    if trimmed.contains(agg_type) {
                        let real_count = lines
                            .iter()
                            .filter(|l| {
                                let t = l.trim();
                                t.contains(agg_type)
                                    && !t.contains("PhantomData")
                                    && !t.contains("fn _use_")
                                    && !t.starts_with("//")
                                    && !t.starts_with("use ")
                                    && !t.starts_with("import ")
                                    && !t.starts_with("from ")
                            })
                            .count();
                        if real_count == 0 {
                            violations.push(LintResult::new_arch(file, i + 1, "AES204", Severity::HIGH,
                                AesImportViolation::ImportIntentViolation {
                                    source_layer: LayerNameVO::new("surfaces"),
                                    import_type: SymbolName::new(agg_type.to_string()),
                                    intent: SymbolName::new("Call aggregate functions instead of using PhantomData"),
                                    reason: Some(shared::taxonomy_message_vo::LintMessage::new("Aggregate in PhantomData is never instantiated — dead code.")),
                                }.to_string(),
                            ));
                        }
                    }
                }
            }
        }
    }

    fn _check_surface_logic(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = LanguageVO::from_path(file);
        let logic_patterns = [
            "lint_path(",
            "compute_score(",
            "has_critical(",
            "walk_rs_files(",
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
                            reason: Some(shared::taxonomy_message_vo::LintMessage::new(
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
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::utility_layer_detector;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::{utility_file_read, utility_import_resolver};
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_layer_vo::{Identity, LayerNameVO};

// PURPOSE: ArchImportForbiddenChecker — AES201: enforce forbidden import rules
// Uses utility functions directly — no IImportParserProtocol, no IAnalyzer.
use async_trait::async_trait;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ArchImportForbiddenChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl IImportForbiddenProtocol for ArchImportForbiddenChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES201")
    }

    async fn check_forbidden_imports(
        &self,
        config: &shared::config_system::taxonomy_config_vo::ArchitectureConfig,
        layer_map: &shared::taxonomy_definition_vo::LayerMapVO,
        files: &FilePathList,
        _root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in &files.values {
            let f_str = f.to_string();
            let basename = f.basename();
            let mut is_exception = false;
            for r in &config.rules {
                if r.name.value.as_str() == "AES201" && r.exceptions.values.contains(&basename) {
                    is_exception = true;
                    break;
                }
            }
            if is_exception {
                continue;
            }

            let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();
            let filename = utility_layer_detector::extract_filename(&f_str);
            if let Some(base_layer) = utility_layer_detector::detect_layer_from_prefix(filename) {
                let specialized = utility_layer_detector::resolve_specialized_layer(
                    &base_layer,
                    &f_str,
                    &layer_keys,
                );
                let layer_name = LayerNameVO::new(specialized.as_str());
                if let Some(def) = layer_map.values.get(&layer_name) {
                    self._check_forbidden_imports(&f_str, &specialized, def, &mut results.values);
                }
            }
            self._check_scope_forbidden_imports(&f_str, config, &mut results.values);
        }
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

    fn _check_forbidden_imports(
        &self,
        file: &str,
        layer_name: &str,
        definition: &LayerDefinition,
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

        let content = match utility_file_read::read_file(file) {
            Some(c) => c,
            None => return,
        };
        let import_lines = utility_import_resolver::parse_import_lines_helper(&content);
        let layer_name_vo = LayerNameVO::new(layer_name);

        for (line_num, line) in &import_lines {
            if let Some(module) = utility_import_resolver::extract_module_from_line(line) {
                let segments: Vec<&str> = module
                    .value()
                    .split([':', '.', '/', '\\'])
                    .filter(|s| !s.is_empty())
                    .collect();
                for forbidden in &forbidden_list {
                    let forbidden_identity = Identity::new(forbidden);
                    let (layer, suffixes) =
                        utility_import_resolver::resolve_scope(&forbidden_identity);
                    let is_forbidden = if suffixes.is_empty() {
                        segments.iter().any(|seg| {
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

    fn _check_scope_forbidden_imports(
        &self,
        file: &str,
        config: &shared::config_system::taxonomy_config_vo::ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        let file_path = match FilePath::new(file.to_string()) {
            Ok(p) => p,
            Err(_) => return,
        };
        let basename = file_path.basename();
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }
        let stem = basename
            .rsplit('.')
            .next_back()
            .map_or(basename.as_str(), |s| s);
        let suffix = stem.rsplit('_').next().map_or("", |s| s);

        let content = match utility_file_read::read_file(file) {
            Some(c) => c,
            None => return,
        };
        let import_lines = utility_import_resolver::parse_import_lines_helper(&content);
        if import_lines.is_empty() {
            return;
        }

        for rule in &config.rules {
            if rule.exceptions.values.contains(&basename.to_string()) {
                continue;
            }
            let scope_identity = Identity::new(&rule.scope.value);
            let (rule_layer, rule_suffixes) =
                utility_import_resolver::resolve_scope(&scope_identity);
            let rule_layer_str = rule_layer.value();
            if !stem.starts_with(&format!("{}_", rule_layer_str)) {
                continue;
            }
            if !rule_suffixes.is_empty() && !rule_suffixes.iter().any(|s| s.value() == suffix) {
                continue;
            }

            for (line_num, line) in &import_lines {
                if let Some(module) = utility_import_resolver::extract_module_from_line(line) {
                    let segments: Vec<&str> = module
                        .value()
                        .split([':', '.', '/', '\\'])
                        .filter(|s| !s.is_empty())
                        .collect();
                    for forbidden in &rule.forbidden.values {
                        let forbidden_identity = Identity::new(forbidden);
                        let (forbidden_layer, forbidden_suffixes) =
                            utility_import_resolver::resolve_scope(&forbidden_identity);
                        let is_forbidden = if forbidden_suffixes.is_empty() {
                            segments.iter().any(|seg| {
                                let cleaned = Identity::new(seg.trim_end_matches(';').trim());
                                match utility_import_resolver::extract_layer_from_import(&cleaned) {
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
                                    source_layer: rule_layer.clone(),
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
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::utility_layer_detector;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::{utility_file_read, utility_import_resolver};
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO};
use shared::taxonomy_name_vo::SymbolName;

// PURPOSE: ArchImportMandatoryChecker — AES202: enforce mandatory import rules
// Uses utility functions directly — no IImportParserProtocol, no IAnalyzer.
use async_trait::async_trait;

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
        config: &shared::config_system::taxonomy_config_vo::ArchitectureConfig,
        layer_map: &shared::taxonomy_definition_vo::LayerMapVO,
        files: &FilePathList,
        _root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in &files.values {
            let f_str = f.to_string();
            let basename = f.basename();

            let mut is_exception = false;
            for r in &config.rules {
                if r.name.value.as_str() == "AES202" && r.exceptions.values.contains(&basename) {
                    is_exception = true;
                    break;
                }
            }
            if is_exception {
                continue;
            }

            let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();
            let filename = utility_layer_detector::extract_filename(&f_str);
            if let Some(base_layer) = utility_layer_detector::detect_layer_from_prefix(filename) {
                let specialized = utility_layer_detector::resolve_specialized_layer(
                    &base_layer,
                    &f_str,
                    &layer_keys,
                );
                let layer_name = LayerNameVO::new(specialized.as_str());
                if let Some(def) = layer_map.values.get(&layer_name) {
                    self._check_mandatory_imports(&f_str, def, &mut results.values);
                }
            }
            self._check_scope_mandatory_imports(&f_str, config, &mut results.values);
        }
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

    fn _check_mandatory_imports(
        &self,
        file: &str,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        if definition.mandatory.values.is_empty() {
            return;
        }
        let file_path = match FilePath::new(file.to_string()) {
            Ok(p) => p,
            Err(_) => return,
        };
        let basename = file_path.basename();
        if basename == "__init__.py" {
            return;
        }
        if definition.exceptions.values.contains(&basename.to_string()) {
            return;
        }

        let content = match utility_file_read::read_file(file) {
            Some(c) => c,
            None => return,
        };
        let file_content = FileContentVO::new(content);
        let import_lines: Vec<(
            shared::taxonomy_common_vo::LineNumber,
            shared::taxonomy_layer_vo::LineContentVO,
        )> = utility_import_resolver::parse_import_lines_helper(file_content.value());
        let stem: &str = basename
            .rsplit('.')
            .next_back()
            .map_or(basename.as_str(), |s| s);
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

    fn _check_scope_mandatory_imports(
        &self,
        file: &str,
        config: &shared::config_system::taxonomy_config_vo::ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        let file_path = match FilePath::new(file.to_string()) {
            Ok(p) => p,
            Err(_) => return,
        };
        let basename = file_path.basename();
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }
        let stem = basename
            .rsplit('.')
            .next_back()
            .map_or(basename.as_str(), |s| s);
        let suffix = stem.rsplit('_').next().map_or("", |s| s);

        let content = match utility_file_read::read_file(file) {
            Some(c) => c,
            None => return,
        };
        let import_lines = utility_import_resolver::parse_import_lines_helper(&content);

        for rule in &config.rules {
            if rule.mandatory.values.is_empty() {
                continue;
            }
            let scope_identity = Identity::new(&rule.scope.value);
            let (rule_layer, rule_suffixes) =
                utility_import_resolver::resolve_scope(&scope_identity);
            let rule_layer_str = rule_layer.value();
            if !stem.starts_with(&format!("{}_", rule_layer_str)) {
                continue;
            }
            if !rule_suffixes.is_empty() {
                let suffix_match = rule_suffixes.iter().any(|s| s.value() == suffix);
                if !suffix_match {
                    continue;
                }
            }
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
                            source_layer: rule_layer.clone(),
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
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::{
    utility_file_read, utility_import_resolver, utility_import_symbol_extractor,
};

// PURPOSE: UnusedImportRuleChecker — AES203: detect unused imports (Rust/Python/JS)
// Uses utility functions directly — no IImportParserProtocol.
use shared::cli_commands::taxonomy_result_vo::LintResult;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct UnusedImportRuleChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IUnusedImportProtocol for UnusedImportRuleChecker {
    fn find_unused_imports(&self, path: &FilePath) -> Vec<LintMessage> {
        let Some(content) = utility_file_read::read_file(path.value()) else {
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

    pub fn new_default() -> Self {
        Self::new_with_config(shared::config_system::taxonomy_config_vo::default_aes_config())
    }

    pub fn mandatory(
        &self,
    ) -> Arc<dyn shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol>
    {
        Arc::new(crate::capabilities_import_mandatory_checker::ArchImportMandatoryChecker::new())
    }

    pub fn forbidden(
        &self,
    ) -> Arc<dyn shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol>
    {
        Arc::new(crate::capabilities_import_forbidden_checker::ArchImportForbiddenChecker::new())
    }

    pub fn dummy(
        &self,
    ) -> Arc<dyn shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol>
    {
        Arc::new(crate::capabilities_dummy_import_checker::DummyImportChecker::new())
    }

    pub fn unused(
        &self,
    ) -> Arc<dyn shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol> {
        Arc::new(crate::capabilities_import_unused_checker::UnusedImportRuleChecker::new())
    }

    pub fn cycle(
        &self,
    ) -> Arc<dyn shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol> {
        Arc::new(crate::capabilities_cycle_import_analyzer::DependencyCycleAnalyzer::new())
    }

    pub fn config(&self) -> &ArchitectureConfig {
        &self.config
    }

    pub fn orchestrator(&self) -> Arc<dyn IImportRunnerAggregate> {
        Arc::new(crate::agent_import_orchestrator::ImportOrchestrator::new(
            self.mandatory(),
            self.forbidden(),
            self.unused(),
            self.cycle(),
        ))
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
pub mod taxonomy_catalog_constant;
pub mod taxonomy_cli_vo;
pub mod taxonomy_command_catalog_vo;
pub mod taxonomy_format_vo;
pub mod taxonomy_metadata_vo;
pub mod taxonomy_position_vo;
pub mod taxonomy_protocol_vo;
pub mod taxonomy_result_vo;
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
        Self {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(line as i64),
            column: ColumnNumber::new(0),
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
pub use crate::common::taxonomy_severity_vo::Severity;
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
pub mod utility_file;
pub mod utility_language_detector;
pub mod utility_layer_detector;
pub mod utility_path_normalization;
pub mod utility_process;
pub mod utility_value_object_generator;
pub use utility_signature_parser::{
    extract_python_method_signatures, extract_trait_method_signatures,
    extract_typescript_method_signatures, python_signature_uses_forbidden_primitive,
    signature_uses_forbidden_primitive, typescript_signature_uses_forbidden_primitive,
};
pub mod utility_signature_parser;
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
    pub naming: crate::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO,
    #[serde(flatten)]
    pub code_analysis: crate::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO,
    #[serde(flatten)]
    pub role: crate::role_rules::taxonomy_role_rule_vo::RoleRuleVO,
    #[serde(flatten)]
    pub orphan: crate::orphan_detector::taxonomy_orphan_rule_vo::OrphanRuleVO,
}

single_field_vo!(LayerMapVO, values: std::collections::HashMap<LayerNameVO, LayerDefinition>);
single_field_vo!(NamingConfig, word_count: Count);
```

---

## File: crates/shared/src/common/taxonomy_layer_vo.rs

```rust
// PURPOSE: FileContentVO, Identity, LayerNameVO, LineContentVO — VOs for layer identity and file content
use crate::string_value_object;

string_value_object!(FileContentVO);
string_value_object!(Identity);
string_value_object!(LayerNameVO);
string_value_object!(LineContentVO);
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
use crate::config_system::taxonomy_config_vo::default_aes_config;

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
        if pat.starts_with("*.") || (pat.starts_with('.') && pat.contains('.')) {
            let suffix = if let Some(s) = pat.strip_prefix('*') {
                s.trim_start_matches('.')
            } else {
                pat.trim_start_matches('.')
            };
            if suffix.is_empty() {
                continue;
            }
            let basename = segments.last().copied().unwrap_or_default();
            if basename.ends_with(suffix) {
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

/// Build default ignored paths from config, with a hardcoded safety net for
/// build artifacts and dependency trees that must never be linted.
pub fn default_ignored_paths() -> Vec<String> {
    let mut ignored: Vec<String> = vec![
        "target".to_string(),
        "test-workspaces".to_string(),
        ".mimocode".to_string(),
        ".agents".to_string(),
        "node_modules".to_string(),
        "build.rs".to_string(),
    ];
    let config = default_aes_config();
    for fp in config.ignored_paths.values.iter() {
        let v = fp.value.replace('/', std::path::MAIN_SEPARATOR_STR);
        if !v.is_empty() && !ignored.contains(&v) {
            ignored.push(v);
        }
    }
    ignored
}

#[cfg(unix)]
fn get_inode(meta: &std::fs::Metadata) -> u64 {
    use std::os::unix::fs::MetadataExt;
    meta.ino()
}

#[cfg(not(unix))]
fn get_inode(_meta: &std::fs::Metadata) -> u64 {
    0
}

/// Collect all lintable source files from a directory tree.
pub fn collect_all_source_files(dir: &Path) -> Vec<FilePath> {
    let mut files = Vec::new();
    if dir.exists() && dir.is_dir() {
        let ignored = default_ignored_paths();
        walk_source_files(dir, &mut files, &ignored);
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
pub fn scan_directory(path: &DirectoryPath) -> Result<FilePathList, FileSystemError> {
    let dir = std::path::Path::new(&path.value);
    if !dir.exists() || !dir.is_dir() {
        return Ok(FilePathList { values: vec![] });
    }
    let files = collect_all_source_files(dir);
    Ok(FilePathList { values: files })
}

/// Walk a directory tree collecting all source files, skipping ignored directories.
pub fn walk_source_files(dir: &Path, files: &mut Vec<FilePath>, ignored: &[String]) {
    let mut visited = HashSet::new();
    walk_source_files_inner(dir, files, ignored, &mut visited)
}

fn walk_source_files_inner(
    dir: &Path,
    files: &mut Vec<FilePath>,
    ignored: &[String],
    visited: &mut HashSet<u64>,
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
                        if let Ok(target_meta) = target.metadata() {
                            let inode = get_inode(&target_meta);
                            if !visited.insert(inode) {
                                continue;
                            }
                            if target_meta.is_dir() {
                                walk_source_files_inner(&target, files, ignored, visited);
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
                if let Ok(meta) = fs::metadata(&path) {
                    let inode = get_inode(&meta);
                    if !visited.insert(inode) {
                        continue;
                    }
                }
                walk_source_files_inner(&path, files, ignored, visited);
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
pub fn walk_rs_files(dir: &Path, cb: &mut dyn FnMut(PathBuf), ignored: &[String]) {
    let root = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    let mut visited = HashSet::new();
    walk_rs_files_inner(&root, cb, ignored, &mut visited, &root)
}

fn walk_rs_files_inner(
    dir: &Path,
    cb: &mut dyn FnMut(PathBuf),
    ignored: &[String],
    visited: &mut HashSet<u64>,
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
                        if let Ok(target_meta) = target.metadata() {
                            let inode = get_inode(&target_meta);
                            if !visited.insert(inode) {
                                continue;
                            }
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
                if let Ok(meta) = fs::metadata(&p) {
                    let inode = get_inode(&meta);
                    if !visited.insert(inode) {
                        continue;
                    }
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

/// Walk up from `start` looking for workspace root markers.
/// Returns the first directory containing Cargo.toml, crates/, packages/, or modules/.
pub fn find_workspace_root(start: &str) -> Option<std::path::PathBuf> {
    let mut dir = std::path::Path::new(start).to_path_buf();
    if !dir.is_absolute() {
        dir = std::env::current_dir().ok()?.join(&dir);
    }
    loop {
        if dir.join("Cargo.toml").exists()
            || dir.join("crates").is_dir()
            || dir.join("packages").is_dir()
            || dir.join("modules").is_dir()
        {
            return Some(dir);
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

## File: crates/shared/src/config-system/mod.rs

```rust
// config-system — taxonomy and contract types
pub mod contract_multi_project_orchestrator_aggregate;
pub mod contract_orchestration_aggregate;
pub mod contract_parser_protocol;
pub mod contract_reader_protocol;
pub mod contract_validator_protocol;
pub mod contract_workspace_detector_protocol;
pub mod taxonomy_config_error;
pub mod taxonomy_config_vo;
pub mod taxonomy_identifier_vo;
pub mod taxonomy_multi_project_summary_vo;
pub mod taxonomy_multi_project_vo;
pub mod taxonomy_multi_project_workspace_info_vo;
pub mod taxonomy_setting_vo;
pub mod taxonomy_source_vo;
pub mod taxonomy_validation_vo;
pub mod utility_config_io;
pub mod utility_config_merger;
```

---

## File: crates/shared/src/config-system/taxonomy_config_vo.rs

```rust
// PURPOSE: ArchitectureConfig, LayerDefinition, ConfigRule — configuration value objects for AES rules definition
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_definition_vo::NamingConfig;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(default)]
pub struct ArchitectureRule {
    pub name: DescriptionVO,
    pub description: DescriptionVO,
    pub rule_type: ErrorCode,
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

pub fn parse_config_yaml(yaml_str: &str) -> ArchitectureConfig {
    let raw: serde_yaml_ng::Value = serde_yaml_ng::from_str(yaml_str).unwrap_or_default();
    if let Some(arch_val) = raw.get("architecture") {
        let mut arch_json: serde_json::Value = serde_json::to_value(arch_val).unwrap_or_default();
        // Extract layers from rules (first rule containing "layers" key) if not at top-level
        if arch_json.get("layers").is_none() {
            if let Some(rules_obj) = arch_json.get_mut("rules").and_then(|r| r.as_object_mut()) {
                for (_rule_code, rule_val) in rules_obj.iter_mut() {
                    if let Some(layers) = rule_val.get_mut("layers") {
                        let layers = std::mem::take(layers);
                        arch_json["layers"] = layers;
                        break;
                    }
                }
            }
        }
        let mut json = arch_json;
        fn remove_nulls(val: &mut serde_json::Value) {
            match val {
                serde_json::Value::Object(m) => {
                    m.retain(|_, v| !v.is_null());
                    for v in m.values_mut() {
                        remove_nulls(v);
                    }
                }
                serde_json::Value::Array(arr) => {
                    for v in arr.iter_mut() {
                        remove_nulls(v);
                    }
                }
                _ => {}
            }
        }
        remove_nulls(&mut json);
        // Convert ignored_paths from array to {values: [...]} format because the Rust struct expects an object with a "values" field.
        if let Some(arr) = json.get("ignored_paths").and_then(|v| v.as_array()) {
            json["ignored_paths"] = serde_json::json!({"values": arr});
        }
        if let Some(layers_obj) = json.get_mut("layers") {
            if let Some(obj) = layers_obj.as_object_mut() {
                let mut suffix_updates: Vec<(
                    String,
                    Option<String>,
                    serde_json::Value,
                    serde_json::Value,
                )> = Vec::new();
                for (layer_name, layer) in obj.iter() {
                    if let Some(suffix_val) = layer.get("suffix") {
                        if let Some(arr) = suffix_val.as_array() {
                            let mut policy: Option<String> = None;
                            let mut allowed = serde_json::Value::Array(Vec::new());
                            let mut forbidden = serde_json::Value::Array(Vec::new());
                            for entry in arr {
                                if let Some(entry_obj) = entry.as_object() {
                                    for (pkey, plist) in entry_obj {
                                        match pkey.as_str() {
                                            "strict" | "flexible" => {
                                                policy = Some(pkey.clone());
                                                if let Some(list) = plist.as_array() {
                                                    allowed = serde_json::json!(list);
                                                }
                                            }
                                            "forbidden" => {
                                                if let Some(list) = plist.as_array() {
                                                    forbidden = serde_json::json!(list);
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            suffix_updates.push((layer_name.clone(), policy, allowed, forbidden));
                        }
                    }
                }
                for (name, policy, allowed, forbidden) in suffix_updates {
                    if let Some(layer) = obj.get_mut(&name) {
                        if let Some(layer_obj) = layer.as_object_mut() {
                            if let Some(ref p) = policy {
                                layer_obj.insert("suffix_policy".to_string(), serde_json::json!(p));
                            }
                            layer_obj.insert("allowed_suffix".to_string(), allowed);
                            if let Some(arr) = forbidden.as_array() {
                                if !arr.is_empty() {
                                    layer_obj.insert("forbidden_suffix".to_string(), forbidden);
                                }
                            }
                            layer_obj.remove("suffix");
                        }
                    }
                }
            }
        }
        if let Some(rules_obj) = json.get_mut("rules") {
            if let Some(obj) = rules_obj.as_object_mut() {
                let mut flat = serde_json::Value::Array(Vec::new());
                for (code, rule_val) in obj.iter() {
                    if let Some(rule_obj) = rule_val.as_object() {
                        let mut base = rule_obj.clone();
                        base.insert("name".to_string(), serde_json::json!(code));
                        // Expand scope array into multiple entries — one per scope element
                        // Only applies to rules WITHOUT conditions (conditions have their own scopes)
                        if let Some(scope_arr) = base.get("scope").and_then(|s| s.as_array()) {
                            if !base.contains_key("conditions") && scope_arr.len() > 1 {
                                for scope_val in scope_arr {
                                    if let Some(s) = scope_val.as_str() {
                                        let mut entry = base.clone();
                                        entry.insert("scope".to_string(), serde_json::json!(s));
                                        if let Some(arr) = flat.as_array_mut() {
                                            arr.push(serde_json::Value::Object(entry));
                                        }
                                    }
                                }
                                continue; // Already pushed per-scope entries, skip single push below
                            } else if let Some(first) = scope_arr.first().and_then(|v| v.as_str()) {
                                base.insert("scope".to_string(), serde_json::json!(first));
                            }
                        }
                        if let Some(conditions) = base.remove("conditions") {
                            if let Some(conds) = conditions.as_array() {
                                if !conds.is_empty() {
                                    for cond in conds {
                                        if let Some(cond_obj) = cond.as_object() {
                                            let mut entry = base.clone();
                                            for (k, v) in cond_obj {
                                                entry.insert(k.clone(), v.clone());
                                            }
                                            // Remove top-level scope array leftovers if condition has its own scope
                                            if let Some(arr) = flat.as_array_mut() {
                                                arr.push(serde_json::Value::Object(entry));
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if let Some(arr) = flat.as_array_mut() {
                                arr.push(serde_json::Value::Object(base));
                            }
                        }
                    }
                }
                *rules_obj = flat;
            }
        }
        let mut config = match serde_json::from_value::<ArchitectureConfig>(json) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[warn] Failed to deserialize ArchitectureConfig: {:?}", e);
                eprintln!("[warn] Falling back to default config. Check your YAML syntax and field types.");
                ArchitectureConfig::default()
            }
        };
        // Top-level ignored_paths (outside architecture section) — merge into config
        if config.ignored_paths.values.is_empty() {
            if let Some(arr) = raw.get("ignored_paths").and_then(|v| v.as_sequence()) {
                let paths: Vec<_> = arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| FilePath::new(s.to_string()).unwrap_or_default())
                    .collect();
                if !paths.is_empty() {
                    config.ignored_paths = FilePathList::new(paths);
                }
            }
        }
        config
    } else {
        let mut config = ArchitectureConfig::default();
        if let Some(arr) = raw.get("ignored_paths").and_then(|v| v.as_sequence()) {
            let paths: Vec<_> = arr
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| FilePath::new(s.to_string()).unwrap_or_default())
                .collect();
            if !paths.is_empty() {
                config.ignored_paths = FilePathList::new(paths);
            }
        }
        config
    }
}

/// All 3 config YAMLs are baked into the binary at compile time via `include_str!`.
/// Runtime project-level config files override these defaults.
/// Cached via OnceLock to avoid re-parsing on every call.
static DEFAULT_RUST_CONFIG: OnceLock<ArchitectureConfig> = OnceLock::new();
static DEFAULT_PYTHON_CONFIG: OnceLock<ArchitectureConfig> = OnceLock::new();
static DEFAULT_TS_CONFIG: OnceLock<ArchitectureConfig> = OnceLock::new();

pub fn default_aes_config() -> ArchitectureConfig {
    DEFAULT_RUST_CONFIG
        .get_or_init(|| parse_config_yaml(include_str!("../../../../lint_arwaky.config.rust.yaml")))
        .clone()
}

pub fn default_config_for_language(language: &str) -> ArchitectureConfig {
    match language {
        "rust" => default_aes_config(),
        "python" => DEFAULT_PYTHON_CONFIG
            .get_or_init(|| {
                parse_config_yaml(include_str!("../../../../lint_arwaky.config.python.yaml"))
            })
            .clone(),
        "javascript" | "typescript" => DEFAULT_TS_CONFIG
            .get_or_init(|| {
                parse_config_yaml(include_str!(
                    "../../../../lint_arwaky.config.javascript.yaml"
                ))
            })
            .clone(),
        _ => {
            eprintln!(
                "[warn] Unknown language '{}', using empty default config.",
                language
            );
            ArchitectureConfig::default()
        }
    }
}
```

---

## File: crates/shared/src/config-system/utility_config_merger.rs

```rust
// PURPOSE: Config merger utility — pure function for merging rules into layer definitions
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::config_system::taxonomy_config_vo::{ArchitectureConfig, ArchitectureRule};
use std::collections::HashMap;

/// Merge all rules into layer definitions.
///
/// Returns (merged_layers, rules_by_layer_index):
/// - `merged_layers`: layer name → merged LayerDefinition
/// - `rules_by_layer_index`: scope string → list of rules (for specialized sub-layer creation)
pub fn merge_config(
    config: &ArchitectureConfig,
) -> (
    HashMap<LayerNameVO, LayerDefinition>,
    HashMap<String, Vec<&ArchitectureRule>>,
) {
    // Step 1: Index all rules by layer scope
    let rules_by_layer = index_rules_by_scope(&config.rules);

    // Step 2: Merge global + base-layer rules into each layer definition
    let mut merged_layers: HashMap<LayerNameVO, LayerDefinition> = HashMap::new();
    for (lname, mut ldef) in config.layers.clone() {
        let lstr = lname.to_string();
        let base_name = match lstr.split('(').next() {
            Some(s) => s.to_string(),
            None => lstr.to_string(),
        };

        // Apply: global rules (key="") + base-layer rules (key=base_name)
        for key in &[String::new(), base_name.clone()] {
            if let Some(rules) = rules_by_layer.get(key.as_str()) {
                for rule in rules {
                    // Skip specialised scoped rules when processing base layers
                    if key.as_str() == base_name && rule.scope.value.contains('(') {
                        continue;
                    }
                    merge_rule_into_definition(&mut ldef, rule);
                }
            }
        }
        merged_layers.insert(lname, ldef);
    }

    // Step 3: Create specialised sub-layer entries from scoped rules
    for rule in &config.rules {
        let scope = rule.scope.to_string();
        if !scope.contains('(') {
            continue;
        }
        create_specialized_sub_layers(&mut merged_layers, &rules_by_layer, &scope);
    }

    (merged_layers, rules_by_layer)
}

/// Index rules by their scope (both base name and full scoped name).
fn index_rules_by_scope(rules: &[ArchitectureRule]) -> HashMap<String, Vec<&ArchitectureRule>> {
    let mut index: HashMap<String, Vec<&ArchitectureRule>> = HashMap::new();
    for rule in rules {
        let scope = rule.scope.to_string();
        let base_key = if scope.is_empty() {
            String::new()
        } else {
            match scope.split('(').next() {
                Some(s) => s.to_string(),
                None => scope.to_string(),
            }
        };
        index.entry(base_key).or_default().push(rule);
        if scope.contains('(') {
            index.entry(scope.clone()).or_default().push(rule);
        }
    }
    index
}

/// Merge a single rule's values into a layer definition.
fn merge_rule_into_definition(ldef: &mut LayerDefinition, rule: &ArchitectureRule) {
    if !rule.exceptions.values.is_empty() {
        for val in &rule.exceptions.values {
            if !ldef.exceptions.values.contains(val) {
                ldef.exceptions.values.push(val.clone());
            }
        }
    }
    if !rule.mandatory.values.is_empty() {
        for val in &rule.mandatory.values {
            if !ldef.mandatory.values.contains(val) {
                ldef.mandatory.values.push(val.clone());
            }
        }
    }
    if !rule.forbidden.values.is_empty() {
        for val in &rule.forbidden.values {
            if !ldef.forbidden.values.contains(val) {
                ldef.forbidden.values.push(val.clone());
            }
        }
    }
    if rule.code_analysis.min_lines.value > 0 {
        ldef.code_analysis.min_lines = rule.code_analysis.min_lines.clone();
    }
    if rule.code_analysis.max_lines.value > 0 {
        ldef.code_analysis.max_lines = rule.code_analysis.max_lines.clone();
    }
    if rule.code_analysis.mandatory_class_definition.value {
        ldef.code_analysis.mandatory_class_definition =
            rule.code_analysis.mandatory_class_definition.clone();
    }
    if !rule.code_analysis.forbidden_inheritance.values.is_empty() {
        for val in &rule.code_analysis.forbidden_inheritance.values {
            if !ldef
                .code_analysis
                .forbidden_inheritance
                .values
                .contains(val)
            {
                ldef.code_analysis
                    .forbidden_inheritance
                    .values
                    .push(val.clone());
            }
        }
    }
    if rule.orphan.check_orphan.value {
        ldef.orphan.check_orphan = BooleanVO::new(true);
    }
    if !rule.orphan.orphan_entry_points.values.is_empty() {
        for val in &rule.orphan.orphan_entry_points.values {
            if !ldef.orphan.orphan_entry_points.values.contains(val) {
                ldef.orphan.orphan_entry_points.values.push(val.clone());
            }
        }
    }
}

/// Create specialised sub-layer entries from a scoped rule.
///
/// E.g., scope "agent(container|registry)" with base layer "agent":
/// - Clones agent definition
/// - Overlays container-specific rules
/// - Inserts "agent(container)" and "agent(registry)" as new sub-layers
fn create_specialized_sub_layers(
    merged_layers: &mut HashMap<LayerNameVO, LayerDefinition>,
    rules_by_layer: &HashMap<String, Vec<&ArchitectureRule>>,
    scope: &str,
) {
    if let Some(paren_start) = scope.find('(') {
        let base_name = scope[..paren_start].trim();
        let inner = scope[paren_start + 1..].trim_end_matches(')').trim();

        // Get base definition (clone to avoid borrow conflict)
        let base_key_str = base_name.to_string();
        let base_def_opt = {
            let base_key = LayerNameVO::new(&base_key_str);
            merged_layers.get(&base_key).cloned()
        };

        if let Some(base_def) = base_def_opt {
            let suffixes: Vec<&str> = if inner.contains('|') {
                inner
                    .split('|')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect()
            } else {
                inner
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect()
            };

            for suffix in suffixes {
                let specialized_key = LayerNameVO::new(format!("{}({})", base_name, suffix));
                if merged_layers.contains_key(&specialized_key) {
                    continue;
                }
                let mut spec_def = base_def.clone();
                if let Some(rules) = rules_by_layer.get(scope) {
                    for r in rules {
                        merge_rule_into_definition(&mut spec_def, r);
                    }
                }
                merged_layers.insert(specialized_key, spec_def);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::taxonomy_common_vo::{Count, PatternList};

    fn make_config(
        layers: HashMap<LayerNameVO, LayerDefinition>,
        rules: Vec<ArchitectureRule>,
    ) -> ArchitectureConfig {
        ArchitectureConfig {
            enabled: BooleanVO::new(true),
            layers,
            rules,
            naming: crate::common::taxonomy_definition_vo::NamingConfig::new(Count::new(2)),
            ignored_paths: crate::common::taxonomy_paths_vo::FilePathList { values: vec![] },
            mandatory_class_definition: BooleanVO::new(false),
        }
    }

    #[test]
    fn merge_empty_config() {
        let config = make_config(HashMap::new(), vec![]);
        let (merged, _) = merge_config(&config);
        assert!(merged.is_empty());
    }

    #[test]
    fn merge_global_rule() {
        let mut layers = HashMap::new();
        layers.insert(LayerNameVO::new("agent"), LayerDefinition::default());
        let rule = ArchitectureRule {
            scope: LayerNameVO::new(""),
            forbidden: PatternList {
                values: vec!["capabilities".to_string()],
            },
            ..Default::default()
        };
        let config = make_config(layers, vec![rule]);
        let (merged, _) = merge_config(&config);
        assert!(merged[&LayerNameVO::new("agent")]
            .forbidden
            .values
            .contains(&"capabilities".to_string()));
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
use crate::taxonomy_layer_vo::Identity;

pub trait IDummyImportCheckerProtocol: Send + Sync {
    fn rule_name(&self) -> Identity;

    fn check_dummy_imports(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
    );
    fn check_dummy_functions(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
    );
    fn check_dummy_impls(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
    );
    fn check_taxonomy_intent(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
    );
    fn check_layer_contract_intent(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
    );
    fn check_surface_logic(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        root_dir: &FilePath,
    );
}
```

---

## File: crates/shared/src/import-rules/contract_import_forbidden_protocol.rs

```rust
// PURPOSE: IImportForbiddenProtocol — exclusive contract for forbidden import checks (AES201)
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::taxonomy_definition_vo::LayerMapVO;
use crate::taxonomy_layer_vo::{Identity, LayerNameVO};
use async_trait::async_trait;

pub struct ForbiddenRuleConfig<'a> {
    pub forbidden_list: &'a [String],
    pub source_layer: &'a LayerNameVO,
    pub allowed_values: &'a [String],
}

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
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::taxonomy_definition_vo::LayerMapVO;
use crate::taxonomy_layer_vo::Identity;
use async_trait::async_trait;

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
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
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
pub mod taxonomy_import_rule_vo;
pub mod taxonomy_language_vo;
pub mod taxonomy_violation_import_vo;
pub mod utility_cycle_detector;
pub mod utility_dummy_detector;
pub mod utility_file_read;
pub mod utility_import_module_parser;
pub mod utility_import_resolver;
pub mod utility_import_symbol_extractor;
pub mod utility_path_normalizer;

pub use taxonomy_dependency_edge_vo::DependencyEdge;
pub use taxonomy_language_vo::LanguageVO;
pub use taxonomy_violation_import_vo::AesImportViolation;
```

---

## File: crates/shared/src/import-rules/taxonomy_cycle_color_vo.rs

```rust
// PURPOSE: ColorVO — DFS 3-color cycle detection state (AES205)
use serde::{Deserialize, Serialize};

/// DFS color for cycle detection.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub enum Color {
    #[default]
    White, // unvisited
    Gray,  // currently in stack
    Black, // fully explored
}
```

---

## File: crates/shared/src/import-rules/taxonomy_dependency_edge_vo.rs

```rust
// PURPOSE: DependencyEdge — representing directed edges in dependency graph

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DependencyEdge {
    pub source: String,
    pub target: String,
}

impl DependencyEdge {
    pub fn new(source: String, target: String) -> Self {
        Self { source, target }
    }
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
```

---

## File: crates/shared/src/import-rules/taxonomy_import_rule_vo.rs

```rust
// PURPOSE: CustomMessageVO, MandatoryImportRuleVO — VOs for AES rule definitions
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_suffix_vo::SuffixVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomMessageVO {
    pub pattern: String,
    pub message: ErrorMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MandatoryImportRuleVO {
    pub suffix: SuffixVO,
    pub imports: PatternList,
}

impl CustomMessageVO {
    pub fn new(pattern: String, message: ErrorMessage) -> Self {
        Self { pattern, message }
    }
}

impl MandatoryImportRuleVO {
    pub fn new(suffix: SuffixVO, imports: PatternList) -> Self {
        Self { suffix, imports }
    }
}
```

---

## File: crates/shared/src/import-rules/taxonomy_language_vo.rs

```rust
// PURPOSE: LanguageVO — classification of programming languages for import rules
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageVO {
    Rust,
    Python,
    JavaScript,
    Unknown,
}

impl LanguageVO {
    pub fn from_path(path: &str) -> Self {
        let ext = Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default();
        match ext {
            "rs" => LanguageVO::Rust,
            "py" => LanguageVO::Python,
            "js" | "ts" | "jsx" | "tsx" => LanguageVO::JavaScript,
            _ => LanguageVO::Unknown,
        }
    }
}
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
use crate::common::taxonomy_name_vo::SymbolName;
use crate::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq)]
enum Color {
    White,
    Gray,
    Black,
}

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

    let mut color: HashMap<String, Color> = HashMap::new();
    let mut parent: HashMap<String, String> = HashMap::new();
    let mut cycle_edges_set: HashSet<(String, String)> = HashSet::new();

    for node in graph.keys() {
        color.entry(node.clone()).or_insert(Color::White);
    }

    for node in graph.keys().cloned().collect::<Vec<_>>() {
        if color[&node] == Color::White {
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
    color: &mut HashMap<String, Color>,
    parent: &mut HashMap<String, String>,
    cycle_edges: &mut HashSet<(String, String)>,
) {
    color.insert(node.to_string(), Color::Gray);

    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            if *color.get(neighbor).unwrap_or(&Color::White) == Color::Gray {
                cycle_edges.insert((node.to_string(), neighbor.clone()));
            } else if *color.get(neighbor).unwrap_or(&Color::White) == Color::White {
                parent.insert(neighbor.clone(), node.to_string());
                dfs_3color(neighbor, graph, color, parent, cycle_edges);
            }
        }
    }

    color.insert(node.to_string(), Color::Black);
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
        match parent.get(cur) {
            Some(p) => {
                cur = p;
                path.push(cur.to_string());
            }
            None => return None,
        }
    }

    path.reverse();
    Some(path)
}
```

---

## File: crates/shared/src/import-rules/utility_dummy_detector.rs

```rust
// PURPOSE: taxonomy_dummy_helper — pure utility functions for dummy function, block, and trait detection
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::import_rules::taxonomy_language_vo::LanguageVO;

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
        || symbol.ends_with("Port")
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

        if !trimmed.contains(symbol) {
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

fn python_imported_symbols(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("from ") && trimmed.contains(" import ") {
            if let Some(import_part) = trimmed.split_once(" import ").map(|(_, p)| p) {
                for name in import_part.split(',') {
                    let name: &str = name.split_whitespace().next().unwrap_or_default();
                    if !name.is_empty() && name != "*" {
                        symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
                    }
                }
            }
            continue;
        }

        if trimmed.starts_with("import ") {
            let module: &str = trimmed
                .trim_start_matches("import ")
                .split_whitespace()
                .next()
                .unwrap_or_default();
            if !module.is_empty() {
                let name: &str = match module.rsplit('.').next() {
                    Some(n) => n,
                    None => module,
                };
                symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
            }
        }
    }

    symbols
}

fn js_imported_symbols(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("import ") && trimmed.contains('{') && trimmed.contains("from") {
            if let Some(open) = trimmed.find('{') {
                if let Some(close) = trimmed.find('}') {
                    let inside = &trimmed[open + 1..close];
                    for part in inside.split(',') {
                        let name: &str = part.split_whitespace().next().unwrap_or_default();
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
                let name = import_part
                    .split_once(" from ")
                    .map(|(n, _)| n)
                    .unwrap_or_default();
                let name = name.trim();
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
                        let name = match part.trim().split(':').next() {
                            Some(n) => n.trim(),
                            None => "",
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

fn is_short_marker(inner: &str) -> bool {
    let t = ['t', 'o', 'd', 'o', '!', '('].iter().collect::<String>();
    let u = [
        'u', 'n', 'i', 'm', 'p', 'l', 'e', 'm', 'e', 'n', 't', 'e', 'd', '!', '(',
    ]
    .iter()
    .collect::<String>();
    let p = ['p', 'a', 'n', 'i', 'c', '!', '(']
        .iter()
        .collect::<String>();
    let r = [
        'u', 'n', 'r', 'e', 'a', 'c', 'h', 'a', 'b', 'l', 'e', '!', '(',
    ]
    .iter()
    .collect::<String>();
    inner.starts_with(&t) || inner.starts_with(&u) || inner.starts_with(&p) || inner.starts_with(&r)
}
```

---

## File: crates/shared/src/import-rules/utility_file_read.rs

```rust
// PURPOSE: File read utility — stateless file content reading helper
use std::path::Path;

/// Read a file to string, returning None on error.
pub fn read_file<P: AsRef<Path>>(path: P) -> Option<String> {
    std::fs::read_to_string(path).ok()
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
pub fn parse_import_lines_helper(content: &str) -> Vec<(LineNumber, LineContentVO)> {
    let mut result = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
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
                combined = combined.split_whitespace().collect::<Vec<&str>>().join(" ");
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
];

fn is_rust_trait_import(name: &str) -> bool {
    if name.starts_with('I') && name.len() > 1 && name.chars().nth(1).unwrap_or(' ').is_uppercase()
    {
        return true;
    }
    if name.ends_with("Protocol")
        || name.ends_with("Port")
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
    let mut in_cfg_test = false;
    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("#[cfg(test)]") {
            in_cfg_test = true;
            continue;
        }
        if in_cfg_test {
            if trimmed == "}" || trimmed.starts_with("}") {
                in_cfg_test = false;
            }
            continue;
        }

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

        if let Some(import_part) = trimmed.strip_prefix("import ") {
            for name in import_part.split(',') {
                let name = name.trim();
                if name.is_empty() {
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
    let mut in_cfg_test = false;
    for (i, line) in content.lines().enumerate() {
        let t = line.trim();
        if t.starts_with("#[cfg(test)]") {
            in_cfg_test = true;
            continue;
        }
        if in_cfg_test {
            if t == "}" || t.starts_with("}") {
                in_cfg_test = false;
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
                || s.ends_with("Port")
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

// ─── Private Helpers ───

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_macro_serialize_always_used() {
        let content = r#"
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(
            Identity::new("Serialize"),
            Identity::new("serde::Serialize"),
        );
        aliases.insert(
            Identity::new("Deserialize"),
            Identity::new("serde::Deserialize"),
        );

        let used = extract_used_symbols(content, &aliases);
        assert!(
            used.contains(&Identity::new("Serialize")),
            "Serialize should always be considered used"
        );
        assert!(
            used.contains(&Identity::new("Deserialize")),
            "Deserialize should always be considered used"
        );
    }

    #[test]
    fn derive_macro_async_trait_always_used() {
        let content = r#"
use async_trait::async_trait;

#[async_trait]
trait MyTrait {
    async fn do_something();
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(
            Identity::new("async_trait"),
            Identity::new("async_trait::async_trait"),
        );

        let used = extract_used_symbols(content, &aliases);
        assert!(
            used.contains(&Identity::new("async_trait")),
            "async_trait should always be considered used"
        );
    }

    #[test]
    fn derive_macro_enum_iter_always_used() {
        // EnumIter was NOT previously in is_rust_trait_import — only DERIVE_MACROS catches it
        let content = r#"
use strum::{EnumIter, Display};

#[derive(EnumIter, Display)]
enum Color {
    Red,
    Green,
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(Identity::new("EnumIter"), Identity::new("strum::EnumIter"));
        aliases.insert(Identity::new("Display"), Identity::new("strum::Display"));

        let used = extract_used_symbols(content, &aliases);
        assert!(
            used.contains(&Identity::new("EnumIter")),
            "EnumIter should always be considered used"
        );
        assert!(
            used.contains(&Identity::new("Display")),
            "Display should always be considered used"
        );
    }

    #[test]
    fn derive_macro_as_ref_str_always_used() {
        // AsRefStr was NOT previously in is_rust_trait_import — only DERIVE_MACROS catches it
        let content = r#"
use strum::AsRefStr;

#[derive(AsRefStr)]
enum Status {
    Active,
    Inactive,
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(Identity::new("AsRefStr"), Identity::new("strum::AsRefStr"));

        let used = extract_used_symbols(content, &aliases);
        assert!(
            used.contains(&Identity::new("AsRefStr")),
            "AsRefStr should always be considered used"
        );
    }

    #[test]
    fn non_derive_import_still_checked_normally() {
        // Regular imports should NOT be auto-marked as used
        let content = r#"
use std::collections::HashMap;

fn main() {
    let _x = 42;
}
"#;
        let mut aliases = HashMap::new();
        aliases.insert(
            Identity::new("HashMap"),
            Identity::new("std::collections::HashMap"),
        );

        let used = extract_used_symbols(content, &aliases);
        assert!(
            !used.contains(&Identity::new("HashMap")),
            "HashMap is genuinely unused"
        );
    }

    #[test]
    fn is_name_used_returns_true_for_derive_macros() {
        // is_name_used should short-circuit for all DERIVE_MACROS entries
        for &m in DERIVE_MACROS {
            assert!(
                is_name_used(m, "fn main() {}", 0),
                "{m} should be considered used via DERIVE_MACROS"
            );
        }
    }
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
    if normalized_file.starts_with(&normalized_root) {
        normalized_file[normalized_root.len()..]
            .trim_start_matches('/')
            .to_string()
    } else {
        normalized_file
    }
}
```

---

