# Product Requirements Document (PRD)

## Lint Arwaky v1.10.11 — SIGNED OFF

---

## 1. Product Overview

**Name**: Lint Arwaky
**Type**: CLI tool + MCP server + TUI launcher
**Version**: 1.10.11
**License**: MIT
**Language**: Rust (2021 edition)

Lint Arwaky is an autonomous multi-language linting, type-checking, and architectural rule auditing tool. It runs as a CLI binary (`lint-arwaky-cli`), an MCP server (`lint-arwaky-mcp`) that exposes 5 tools over JSON-RPC 2.0, and an interactive TUI launcher (`lint-arwaky-tui`).

The project audits itself: `lint-arwaky-cli check .` runs the same AES rule engine against its own codebase that it runs against third-party code.

---

## 2. Problem Statement

Software projects accumulate quality debt silently. Developers lack:

- A single tool that audits Rust + Python + JavaScript/TypeScript together
- Architectural enforcement that prevents cross-layer violations in multi-domain codebases
- A unified interface for both human developers (CLI) and AI agents (MCP tools)
- A self-auditing tool whose own codebase passes the rules it enforces
- Static analysis with zero bypass tolerance (`noqa`, `type: ignore`, `#[allow(...)]` are flagged)

---

## 3. AI Agent Value

Lint Arwaky is designed to integrate with AI coding agents through its MCP interface, providing:

| Value Driver             | Description                                                                              |
| ------------------------ | ---------------------------------------------------------------------------------------- |
| **Agent Autonomy** | Agents operate via 5 MCP tools without human oversight                                   |
| **Job Tracking**   | Jobs are tracked in a thread-safe registry (in-memory, per-process)                      |
| **Self-Healing**   | The `fix` command applies safe auto-fixes; the `suggest` command guides manual fixes |
| **24/7 Quality**   | The `watch` command polls and re-lints continuously during development                 |

---

## 4. Target Users

| User                             | Interface          | Use Case                                                 |
| -------------------------------- | ------------------ | -------------------------------------------------------- |
| **AI Agents**              | MCP tools (5)      | Automated code review, pre-commit checks, CI integration |
| **Prototype Developers**   | MCP + CLI          | Fast iterations, AI-assisted coding, quality gates       |
| **Architecture Engineers** | Architecture tools | AES rule enforcement, clean code, DDD                    |
| **Developers**             | CLI (20+ commands) | Local development, watch mode, git hooks                 |
| **CI/CD Pipelines**        | CLI + exit codes   | Quality gates, SARIF/JUnit/JSON reports                  |
| **Contributors**           | GitHub + PRs       | New adapters, CLI commands, MCP tools                    |

---

## 5. Feature Requirements

Feature requirements are organized by **feature crates** (Cargo workspace members) representing self-contained vertical slices of functionality.

**Vertical Slicing & Layer Boundary Rules**:

1. **No Crate-Level Layer Restriction**: A feature crate is not restricted to a single layer. Instead, a single feature folder/crate can contain files from multiple layers internally as needed:
   * `capabilities_` (business/domain logic of the feature)
   * `infrastructure_` (external tool integrations, OS, file system, or protocol libraries)
   * `agent_` (orchestration and coordination logic within the feature)
   * `root_` (the crate's local root container, wiring implementations)
2. **Layer Prefix Convention**: The layer of a file is strictly determined by its **file prefix** (`taxonomy_`, `contract_`, `capabilities_`, `infrastructure_`, `agent_`, `surface_`, `root_`), NOT by its folder location or crate.
3. **No Direct Inter-Layer Imports**: `infrastructure_` and `capabilities_` layers must not import each other directly (enforced by import rules). Instead, they communicate via ports/protocols (`contract_`) or are coordinated by an `agent_` orchestrator.

### 5.1 `shared` — Foundation

**Depends on:** Nothing

Foundational domain component containing the core taxonomy and contract definitions of the entire system. It operates under a zero-dependency constraint (meaning it cannot import from any other workspace crate). It holds strictly declarative, primitive-level data and abstractions—such as domain Value Objects, common error entities, shared constants, generic helper utilities, and abstract contract ports—which serve as the universal vocabulary and interfaces implemented by feature crates.

| ID     | Requirement                                                                                         | AES Codes |
| ------ | --------------------------------------------------------------------------------------------------- | --------- |
| FR-001 | All `taxonomy_*` VOs, entities, events, errors, constants, utilities, and helpers across features | —        |
| FR-002 | All `contract_*` ports, protocols, and aggregates across features                                 | —        |

### 5.2 `source-parsing` — Source Code Parsing

**Depends on:** `shared`

Self-contained feature component responsible for extracting the structural features of source code across Rust, Python, and JavaScript/TypeScript. It encapsulates multi-language structural scanning (infrastructure) and AST/metadata parsing routines (capabilities) to standardize code metadata into clean, language-agnostic Value Objects defined in `shared`. It enforces a strict separation of concerns: it behaves as an empirical data gatherer and does not perform validation or rule evaluation, leaving all architectural policy enforcement to upper-level rules modules.

| ID      | Requirement                                                                                                                                                    | AES Codes |
| ------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------- |
| FR-003  | Multi-Language Scanners — regex-based line and structure scanners for Rust, Python, and JavaScript/TypeScript                                                 | —        |
| FR-004  | Import & Export Extraction — extract import statements, check unused imports, and resolve symbol exports                                                      | —        |
| FR-007  | Symbol & Definition Mapping — discover and index raw symbols, class/struct definitions, functions, and methods                                                | —        |
| FR-008a | Rust Trait Implementation Resolution — resolve `impl TraitName for StructName` associations to analyze implemented traits                                   | —        |
| FR-008b | JS/TS Class Inheritance Resolution — resolve class `extends` base maps to trace single inheritance paths                                                    | —        |
| FR-008c | Python Class Inheritance Resolution — resolve class base class lists to trace multiple class inheritance paths                                                | —        |
| FR-008d | Python Interface Resolution — resolve class inheritance of `ABC` (Abstract Base Classes) and `Protocol` implementations to support interface verification | —        |
| FR-009  | Complexity & Flow Analysis — extract and expose raw control flow tokens and variable assignment structures within files for downstream analysis               | —        |
| FR-128  | Barrel & Package Resolution — identify index/package entrypoints to trace delegated exports                                                                   | —        |
| FR-129  | Path Normalization — normalize file paths and relative imports into absolute package paths                                                                    | —        |
| FR-120  | Show enclosing scope (function/class) for violations                                                                                                           | FR-003    |
| FR-121  | Trace call chains across project                                                                                                                               | FR-003    |
| FR-122  | Track variable flow within scope                                                                                                                               | FR-003    |

### 5.3 `file-system` — File System Abstraction

Self-contained feature component responsible for physical disk I/O and file system queries. It acts as the project's single gateway to the operating system's file system, offering features like directory traversal, globbing, reading/writing files, and path validation. In accordance with the cross-layer feature architecture, it encapsulates both OS-specific infrastructure adapters and any required path-validation domain capabilities. It maintains a strict boundary: it has no knowledge of code syntax or parsing rules (which belongs to `source-parsing`), nor does it enforce configuration or architectural validation policies. It treats files purely as raw data streams and directories as structural paths.

| ID     | Requirement                                                                                                         | AES Codes |
| ------ | ------------------------------------------------------------------------------------------------------------------- | --------- |
| FR-028 | Directory Recursive Walking — recursively walk a directory and gather file paths while filtering out ignored files | —        |
| FR-029 | Glob Pattern Matching — locate files matching a specific search pattern query                                      | —        |
| FR-125 | File Read/Write Operations — read string content from files and write string content to target files               | —        |
| FR-126 | Path Existence and Type Checks — check if paths exist and determine if they are files or directories               | —        |
| FR-127 | Path Resolution and Manipulation — get parent paths, working directory, relative path mapping, and path joining    | —        |

### 5.4 `file-watch` — File Watching

Self-contained feature component responsible for real-time file system change detection. In accordance with the cross-layer feature architecture, it encapsulates both notify-based filesystem polling (infrastructure) and snapshot state comparison (capabilities). It maintains a stateful snapshot of file system modification times to compute diffs. It maintains a strict boundary: it has no knowledge of file parsing or linting rules, nor does it initiate the lint process itself; it only raises events containing a list of modified file paths for upper-level orchestrators to consume.

| ID      | Requirement                                                                                                                                                        | AES Codes |
| ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------- |
| FR-113a | Directory Snapshotting — take initial recursive snapshot of project files and their modification timestamps                                                       | —        |
| FR-113b | File Modification Detection — scan watched directories to compare current timestamps against the snapshot and detect new or modified files                        | —        |
| FR-113c | Ignore Patterns Filtering — filter out standard ignored directories (e.g.,`.git`, `node_modules`, `__pycache__`) from file watching to optimize performance | —        |
| FR-113d | Event Trigger Dispatching — expose list of changed files to trigger immediate, incremental linting runs                                                           | —        |

### 5.5 `metrics-service` — Metrics Provider

**Depends on:** `shared`, `source-parsing`, `code-analysis`

Self-contained feature component responsible for tracking and calculating long-term codebase quality trends and scoring metrics. In accordance with the cross-layer feature architecture, it encapsulates quality score mathematical engines (capabilities) and history serialization adapters (infrastructure) to store data locally. It maintains a strict boundary: it does not perform raw source code parsing or evaluate compliance rules itself (which belongs to `source-parsing` and rules modules), nor does it format final reports for CLI consumption (which belongs to `output-report` and `cli-commands`). It consumes static violations lists, computes point deductions, and manages local history files purely as structured metrics.

| ID      | Requirement                                                                                                                    | AES Codes |
| ------- | ------------------------------------------------------------------------------------------------------------------------------ | --------- |
| FR-088a | Score Calculation Engine — compute project compliance score on a 0-100 scale by deducting penalty points from rule violations | —        |
| FR-088b | Metrics History Persistence — store historical score trend data locally to a `.lint-trends.json` file                       | —        |
| FR-088c | Trend Report Visualization — output comparative score trend changes over time to stdout                                       | —        |

### 5.6 `multi-project` — Multi-Project Governance

**Depends on:** `shared`, `source-parsing`, `file-system`

Self-contained feature component responsible for managing and aggregating lint compliance across complex workspaces containing nested, multiple sub-projects. In accordance with the cross-layer feature architecture, it encapsulates recursive sub-project configuration discovery (infrastructure) and rules inheritance/consolidation logic (capabilities). It maintains a strict boundary: it does not run rules or modify files directly, nor does it compile AST/tokens for source modules. It acts purely as a coordinator that inventories workspace paths, resolves inherited rule configurations, and compiles project metadata into a unified workspace registry.

| ID      | Requirement                                                                                                                               | AES Codes |
| ------- | ----------------------------------------------------------------------------------------------------------------------------------------- | --------- |
| FR-091a | Multi-Project Auto-Discovery — recursively detect nested sub-projects by scanning configuration files                                    | —        |
| FR-091b | Workspace Compliance Aggregation — run lint checking across all projects in parallel and consolidate them into a single aggregate report | —        |

### 5.7 `code-analysis` — Code Quality & Auto-Fix

**Depends on:** `shared`, `source-parsing`, `file-system`

Self-contained feature component responsible for running file-level structural audits, quality checkers, and bypass comments detectors. In accordance with the cross-layer feature architecture, it encapsulates specialized rule validators (capabilities) and local process coordinators (agents) that execute multiple audits over files. It maintains a strict boundary: it has no direct interaction with the OS file system or physical I/O (which belongs to `file-system`), nor does it handle raw file parsing or syntax scanning (which belongs to `source-parsing`). It processes language-agnostic source metadata Value Objects and returns structural compliance violations list.

| ID      | Requirement                                                                                                                               | AES Codes |
| ------- | ----------------------------------------------------------------------------------------------------------------------------------------- | --------- |
| FR-025a | Maximum File Line Count Validation — verify that no source code file exceeds the maximum configured lines limit                          | AES020    |
| FR-025b | Minimum File Line Count Validation — verify that no source code file falls below the minimum required lines threshold                    | AES021    |
| FR-030a | Attribute Bypass Detection — scan for forbidden compilation bypass attributes such as `#[allow(...)]` or compiler warnings suppressors | AES022    |
| FR-030b | Fatal Panic and Unwrap Detection — check for raw, direct code panic or unwrap calls bypassing safe error handling architectures          | AES022    |
| FR-030c | Comment-Based Linter Bypass Detection — scan for inline linter suppression tags like `noqa` or `type: ignore`                        | AES022    |
| FR-031a | Unused Mandatory Imports Check — identify and flag imported dependency modules that are not actively referenced in the file              | AES023    |
| FR-032a | Empty Struct and Trait Bypass Check — flag defined traits and structures that contain no fields or methods (dead inheritance bypass)     | AES024    |
| FR-045a | Capability Dispatch Method Check — verify that dynamic dispatch routes correspond to existing target capability methods                  | AES0303   |
| FR-046a | Capability Load Balancing Check — audit and balance dispatch routes to ensure no single capability becomes a logic bottleneck            | AES0303   |
| FR-050  | Circular dependency cycle analyzer — detect circular imports                                                                             | AES015    |
| FR-042  | Mandatory inheritance checker — every file implements a contract                                                                         | AES014    |

### 5.8 `lifecycle-state` — Agent Lifecycle Management

**Depends on:** `shared`, `pipeline-jobs`

Self-contained feature component responsible for tracking, transitioning, and broadcasting the running lifecycle states of the linter agent. In accordance with the cross-layer feature architecture, it encapsulates state-transition state machines (capabilities) and state event handlers (agent). It maintains a strict boundary: it has no knowledge of code files, file paths, parsed source metadata, or linter compliance rules. It focuses purely on process flow transitions (e.g. from initialization to checking to completion)

| ID      | Requirement                                                                                                                         | AES Codes |
| ------- | ----------------------------------------------------------------------------------------------------------------------------------- | --------- |
| FR-006a | Agent State Transition Tracking — model and transition the execution stages of the linter agent (INIT, RUNNING, COMPLETED, FAILED) | —        |
| FR-006b | Lifecycle Event Hooking — provide event callbacks/hooks when state transitions occur for TUI or pipeline tracking                  | —        |

### 5.9 `import-rules` — Import Compliance

**Depends on:** `shared`, `source-parsing`, `file-system`, `output-report`

Self-contained feature component responsible for validating code imports and module dependency structures against strict architectural layer boundaries. In accordance with the cross-layer feature architecture, it encapsulates compliance check engines (capabilities) and configuration mapping adapters. It maintains a strict boundary: it has no direct interaction with the physical filesystem (which belongs to `file-system`), nor does it parse code files to build ASTs (which belongs to `source-parsing`). It consumes language-agnostic code metadata (specifically extracted import statements) and reports layer compliance violations.

| ID      | Requirement                                                                                                                             | AES Codes |
| ------- | --------------------------------------------------------------------------------------------------------------------------------------- | --------- |
| FR-010a | Layer Dependency Violation Scan — verify that lower layers do not import from upper layers and enforce unidirectional import flows     | AES001    |
| FR-011a | Mandatory Crate/Layer Imports Verification — check that files in specific layers import their mandatory contracts or taxonomies        | AES002    |
| FR-013a | Forbidden Root Imports Audit — identify and flag imports of root components/containers by other internal library layers                | AES004    |
| FR-014a | File Layer Name Suffix Compliance — verify that all logic files carry a suffix corresponding exactly to their layer prefix designation | AES005    |
| FR-015a | Contract Name Suffix Compliance — verify that contract files only use `_port`, `_protocol`, or `_aggregate` suffixes             | AES006    |
| FR-017a | Surface Direct Imports Enforcement — verify that surface command/handler files do not import capabilities or infrastructure directly   | AES003    |

### 5.10 `output-report` — Output Formatting & Report Generation

**Depends on:** `shared`, `source-parsing`, `code-analysis`, `pipeline-jobs`

Self-contained feature component responsible for formatting compliance findings and generating linting reports. In accordance with the cross-layer feature architecture, it encapsulates report formatting engines (capabilities) and physical output writers (infrastructure) that write reports to files or standard output streams. It maintains a strict boundary: it has no knowledge of how rules are parsed, calculated, or enforced. It consumes a list of compliance violations (Value Objects) and translates them into structured reports.

| ID      | Requirement                                                                                                            | AES Codes |
| ------- | ---------------------------------------------------------------------------------------------------------------------- | --------- |
| FR-058a | Report File Output Writer — generate and write compliance reports to specified local filesystem output paths          | —        |
| FR-095a | Plain Text Formatter — format findings into human-readable text console outputs with styled error reporting           | —        |
| FR-096a | JSON Formatter — serialize findings list into a machine-readable JSON structure for integrations                      | —        |
| FR-097a | SARIF Formatter — output reports conforming to the Static Analysis Results Interchange Format (SARIF) v2.1.0 standard | —        |
| FR-098a | JUnit Formatter — format output into JUnit XML schema to support CI build quality gate pipelines                      | —        |

### 5.11 `pipeline-jobs` — Jobs, Dispatcher, Execution

**Depends on:** `shared`, `source-parsing`, `multi-project`

Self-contained feature component responsible for scheduling and executing concurrent asynchronous linting runs.

| ID     | Requirement                           | AES Codes |
| ------ | ------------------------------------- | --------- |
| FR-067 | Cancel lint job (`cancel <job_id>`) | —        |

### 5.12 `config-system` — Config Loading & Parsing

**Depends on:** `shared`, `source-parsing`, `import-rules`, `pipeline-jobs`

Self-contained feature component responsible for loading and parsing project linter profiles (YAML configs).

| ID     | Requirement                                                        | AES Codes |
| ------ | ------------------------------------------------------------------ | --------- |
| FR-002 | Multi-config YAML support, language detection, config-driven rules | —        |
| FR-048 | Constant purity checker — _constant files: only pub const/static  | AES015    |
| FR-040 | MCP schema checker — MCP tools need docstrings + JSON Schema      | AES025    |

### 5.13 `naming-rules` — Naming Convention

**Depends on:** `shared`, `source-parsing`, `import-rules`, `output-report`

Self-contained feature component responsible for checking and enforcing naming convention rules for logic filenames and code definitions. In accordance with the cross-layer feature architecture, it encapsulates convention checker engines (capabilities) and coordinates naming compliance audits. It maintains a strict boundary: it has no direct interaction with file modification, AST parsing, or compiler tasks. It consumes token mappings (Value Objects) and reports prefix/suffix naming violations.

| ID     | Requirement                                                                                                  | AES Codes |
| ------ | ------------------------------------------------------------------------------------------------------------ | --------- |
| FR-020 | Naming convention checker — strict word snake_case                                                          | AES010    |
| FR-021 | Mandatory struct/trait definition checker — every file needs struct/enum/trait                              | AES011    |
| FR-022 | Suffix/Prefix rules — suffix must match layer definition rules (e.g., _vo for taxonomy, _port for contract) | AES012    |
| FR-124 | Generate naming variants (snake_case, camelCase, etc.)                                                       | FR-003    |

### 5.14 `git-hooks` — Git Hooks Management

**Depends on:** `shared`, `source-parsing`, `output-report`, `pipeline-jobs`

Self-contained feature component responsible for configuring and deploying automated hooks into Git.

| ID     | Requirement                                                | AES Codes |
| ------ | ---------------------------------------------------------- | --------- |
| FR-114 | Git pre-commit hook (`install-hook`, `uninstall-hook`) | —        |

### 5.15 `role-rules` — Role Violations

**Depends on:** `shared`, `source-parsing`, `import-rules`, `output-report`

Self-contained feature component responsible for verifying architectural pattern assignments and detecting role-boundary violations.

| ID     | Requirement                                                                   | AES Codes |
| ------ | ----------------------------------------------------------------------------- | --------- |
| FR-035 | Surface hierarchy violation detector — utility imports smart surface         | AES0306   |
| FR-036 | Passive surface violation detector — passive imports taxonomy only           | AES0306   |
| FR-037 | Agent role violation detector — behavioral mandates per agent role           | AES0305   |
| FR-038 | Agent any-bypass detector — no `any` type in orchestrators                 | AES0305   |
| FR-027 | Primitive usage checker — no raw primitives in domain types                  | AES016    |
| FR-041 | Forbidden inheritance detector — aggregate not inherit port/protocol         | AES013    |
| FR-016 | Surface layer rule checker — surfaces delegate via ServiceContainerAggregate | AES0306   |

### 5.16 `auto-fix` — Auto-Fix Processor

**Depends on:** `shared`, `source-parsing`, `code-analysis`, `output-report`

Self-contained feature component responsible for generating code modifications to resolve linter violations.

| ID     | Requirement                                   | AES Codes |
| ------ | --------------------------------------------- | --------- |
| FR-005 | Apply safe auto-fixes (Rust + Python + JS/TS) | AES0303   |
| FR-123 | Project-wide symbol rename                    | FR-003    |

### 5.17 `language-adapters` — External Linter Adapters

**Depends on:** `shared`, `source-parsing`, `code-analysis`, `metrics-service`, `output-report`, `pipeline-jobs`

Self-contained feature component responsible for adapting and executing external code tools (Ruff, Clippy, ESLint).

| ID     | Requirement                                           | AES Codes |
| ------ | ----------------------------------------------------- | --------- |
| FR-070 | Run Clippy linting on Rust files                      | —        |
| FR-071 | Run rustfmt formatting check on Rust files            | —        |
| FR-072 | Run cargo-audit dependency vulnerability scan on Rust | —        |
| FR-073 | Run Ruff linting on Python files                      | —        |
| FR-074 | Run MyPy type checking on Python files                | —        |
| FR-075 | Run Bandit security scanning on Python files          | —        |
| FR-076 | Run Radon-style complexity analysis on Python files   | —        |
| FR-077 | Run pip-audit dependency vulnerability scan on Python | —        |
| FR-078 | Run ESLint on JavaScript/TypeScript files             | —        |
| FR-079 | Run Prettier formatting on JS/TS files                | —        |
| FR-080 | Run TSC type checking on TypeScript files             | —        |

### 5.18 `plugin-system` — Plugin Discovery & Management

**Depends on:** `shared`, `source-parsing`, `pipeline-jobs`

Self-contained feature component responsible for registering and invoking custom external rules extensions.

| ID     | Requirement                           | AES Codes |
| ------ | ------------------------------------- | --------- |
| FR-089 | Dependency listing (`dependencies`) | —        |

### 5.19 `orphan-detector` — Orphan Code Detection

**Depends on:** `shared`, `source-parsing`, `code-analysis`, `output-report`

Self-contained feature component responsible for performing project dependency tracing to find unused components.

| ID     | Requirement                                    | AES Codes |
| ------ | ---------------------------------------------- | --------- |
| FR-033 | Orphan code detector — unreachable components | AES030    |

### 5.20 `project-setup` — Project Init, Doctor, MCP Config

**Depends on:** `shared`, `source-parsing`, `pipeline-jobs`

Self-contained feature component responsible for initializing workspace profiles, creating configs, and running doctor diagnostics.

| ID     | Requirement                                              | AES Codes |
| ------ | -------------------------------------------------------- | --------- |
| FR-060 | Environment diagnostics (`setup doctor`)               | —        |
| FR-061 | Create default config (`setup init`)                   | —        |
| FR-062 | MCP client config (`setup mcp-config --client <name>`) | —        |
| FR-063 | Hermes integration (`setup hermes [--remove]`)         | —        |

### 5.21 `cli-commands` — CLI Surfaces + Transport

**Depends on:** `shared`, `source-parsing`, `auto-fix`, `code-analysis`, `output-report`, `pipeline-jobs`, `project-setup`

Self-contained feature component responsible for implementing and presenting CLI surfaces (`clap` commands) and parsing/dispatching transport requests.

| ID     | Requirement                                                            | AES Codes      |
| ------ | ---------------------------------------------------------------------- | -------------- |
| FR-055 | Full architecture compliance analysis (`check [path] [--git-diff]`)  | FR-001–FR-050 |
| FR-056 | External project scan (`scan [path]`) — AES + all external adapters | FR-055         |
| FR-057 | Apply safe fixes (`fix [path]`)                                      | FR-005         |
| FR-059 | CI mode with exit codes (`ci [path] --threshold <N>`)                | FR-055         |
| FR-064 | List adapters (`adapters`)                                           | FR-055         |
| FR-065 | Show config (`config show`)                                          | FR-002         |
| FR-066 | Display version (`version`)                                          | —             |
| FR-090 | Git diff lint (`git-diff`)                                           | FR-055         |
| FR-110 | Compare violation diff between paths (`diff`)                        | FR-055         |
| FR-111 | AI-powered fix suggestions (`suggest`)                               | FR-057         |
| FR-112 | Import/export configuration (`import`, `export`)                   | FR-002         |
| FR-115 | CLI via `clap` 4.6 subcommand groups                                 | FR-001         |
| FR-116 | Direct command execution via `std::process::Command`                 | FR-001         |
| FR-085 | Security vulnerability scan (`security`)                             | FR-075         |
| FR-086 | Cyclomatic complexity analysis (`complexity`)                        | FR-076         |
| FR-087 | Code duplication detection (`duplicates`)                            | FR-055         |

### 5.22 `mcp-server` — MCP JSON-RPC 2.0 Server

**Depends on:** `shared`, `source-parsing`, `cli-commands`, `code-analysis`, `language-adapters`, `output-report`, `pipeline-jobs`

Self-contained feature component responsible for exposing linter capabilities as MCP JSON-RPC 2.0 services.

| ID     | Requirement                                        | AES Codes |
| ------ | -------------------------------------------------- | --------- |
| FR-100 | MCP server via JSON-RPC 2.0 (`mcp-sdk-rs` 0.3.4) | FR-055    |
| FR-101 | MCP tool:`execute_command(action, args)`         | FR-100    |
| FR-102 | MCP tool:`list_commands(domain)`                 | FR-100    |
| FR-103 | MCP tool:`commands_schema(tool_name)`            | FR-100    |
| FR-104 | MCP tool:`read_skill_context(section)`           | FR-100    |
| FR-105 | MCP tool:`health_check()`                        | FR-100    |
| FR-106 | CI/CD integration (OIDC, SLSA Provenance)          | FR-100    |

---

## 6. Non-Functional Requirements

| ID      | Requirement                     | Target                 |
| ------- | ------------------------------- | ---------------------- |
| NFR-003 | Startup time (MCP server)       | < 2 s                  |
| NFR-004 | Single-file scan time           | < 5 s                  |
| NFR-005 | Full project scan               | < 30 s                 |
| NFR-006 | Rust toolchain                  | >= 1.70 (edition 2021) |
| NFR-007 | Binary size (release, stripped) | < 30 MB                |

---

## 7. Architecture

### 7.1 Domain Model (7 Layers by File Prefix)

```
crates/
  shared/               -- Foundation: ALL taxonomy_* + contract_* (NO deps on feature crates)
  import-rules/         -- AES001, AES002 import compliance
  naming-rules/         -- AES010, AES011 naming convention
  role-rules/           -- AES0305, AES0306 role violations
  orphan-detector/      -- AES030 orphan code detection
  code-analysis/        -- AES022, AES023, AES024, AES0303 quality & auto-fix
  auto-fix/             -- AES0303 auto-fix processor
  config-system/        -- Config loading & parsing
  pipeline-jobs/        -- Jobs, dispatcher, execution
  source-parsing/       -- Source code parsing (scanners, parsers)
  language-adapters/    -- Python, JS, Rust linter adapters
  file-system/          -- File system abstraction
  file-watch/           -- File watching
  git-hooks/            -- Git hooks management
  multi-project/        -- Multi-project governance
  project-setup/        -- Project init, doctor, mcp-config
  plugin-system/        -- Plugin discovery & management
  output-report/        -- Output formatting & report generation
  lifecycle-state/      -- Agent lifecycle management
  metrics-service/      -- Metrics provider
  cli-commands/         -- CLI surfaces (_command) + transport
  mcp-server/           -- MCP JSON-RPC 2.0 server
  root_compsotion_container.rs -- Root composition (root layer)
  root_cli_main_entry.rs       -- CLI binary entry (root_entry)
  root_mcp_main_entry.rs       -- MCP binary entry (root_entry)
  root_tui_main_entry.rs       -- TUI binary entry (root_entry)

Layer prefixes (determined by FILE NAME, not folder):
  taxonomy_       → _vo, _entity, _event, _error, _constant
  contract_       → _port, _protocol, _aggregate
  capabilities_   → _checker, _analyzer, _processor, etc.
  infrastructure_ → _adapter, _provider, _scanner, etc.
  agent_          → _orchestrator (ONLY)
  surface_        → _command, _handler, _controller
  root_           → _container, _entry
```

### 7.2 Feature Crate → Container Mapping (Vertical Slicing)

Each feature crate contains **multiple layers internally** (taxonomy, contract, capabilities, infrastructure, agent, surface, root) as needed. The layer is determined by **file prefix**, not by crate. Every crate owns a `root_container.rs` that wires its internal implementations.

| Crate                 | Container File(s)                                            | Purpose                                                                                                      |
| --------------------- | ------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------ |
| `shared`            | — (no container; re-exports `common` module)              | All `taxonomy_*` VOs, entities, events, errors, constants; all `contract_*` ports, protocols, aggregates |
| `import-rules`      | `import_container.rs`                                      | AES001/AES002 import checkers (capabilities + contracts + orchestrators)                                     |
| `naming-rules`      | `naming_container.rs`                                      | AES010/AES011 naming checkers (capabilities + contracts + orchestrators)                                     |
| `role-rules`        | `role_container.rs`, `agent_role_container.rs`           | AES0305/AES0306 role auditors (capabilities + contracts + orchestrators)                                     |
| `code-analysis`     | `analysis_container.rs`, `contract_checker_container.rs` | AES022/AES023/AES024/AES0303 quality & auto-fix (capabilities + contracts + orchestrators)                   |
| `auto-fix`          | `auto_fix_container.rs`                                    | AES0303 auto-fix processor (capabilities + contracts)                                                        |
| `orphan-detector`   | `orphan_container.rs`                                      | AES030 orphan detection (capabilities + contracts + orchestrators)                                           |
| `config-system`     | `config_container.rs`                                      | Config loading, parsing, validation (infrastructure + contracts)                                             |
| `source-parsing`    | `source_parsing_container.rs`                              | Source code parsing (infrastructure + contracts)                                                             |
| `language-adapters` | `language_container.rs`                                    | External linter adapters (infrastructure + contracts + surfaces)                                             |
| `file-system`       | `file_container.rs`                                        | File system abstraction (infrastructure + contracts)                                                         |
| `file-watch`        | `file_watch_container.rs`                                  | File watching (infrastructure + contracts)                                                                   |
| `git-hooks`         | `git_container.rs`                                         | Git hooks management (infrastructure + contracts + agent)                                                    |
| `multi-project`     | `multi_project_container.rs`                               | Multi-project governance (agent + contracts)                                                                 |
| `project-setup`     | `setup_container.rs`                                       | Project init, doctor, mcp-config (agent + contracts + surfaces)                                              |
| `plugin-system`     | `plugin_container.rs`                                      | Plugin discovery & management (infrastructure + contracts)                                                   |
| `output-report`     | `output_container.rs`                                      | Output formatting & report generation (agent + contracts)                                                    |
| `lifecycle-state`   | `lifecycle_container.rs`                                   | Agent lifecycle management (agent + contracts)                                                               |
| `metrics-service`   | `metrics_container.rs`                                     | Metrics provider (infrastructure + contracts)                                                                |
| `pipeline-jobs`     | `pipeline_container.rs`, `agent_job_container.rs`        | Jobs, dispatcher, execution (agent + contracts)                                                              |
| `cli-commands`      | `transport_container.rs`                                   | CLI surfaces + command transport (surfaces + root + contracts)                                               |
| `mcp-server`        | `mcp_container.rs`                                         | MCP JSON-RPC 2.0 server (surfaces + root + contracts)                                                        |

**Rule**: Containers are at `root_` layer (`root_container.rs`). They wire `capabilities_*`, `infrastructure_*`, and `agent_*` impls behind `contract_*` traits. Surface crates access features **only** through container methods or `ServiceContainerAggregate` trait.

### 7.3 Dependency Graph (Build Order) — UPDATED

Based on actual Cargo.toml dependencies (verified):

```
LEVEL 0: Foundation (zero deps)
  shared-lint-arwaky              ← taxonomy types, contract traits

LEVEL 1: Depends only on shared
  source-parsing-lint-arwaky      ← shared

LEVEL 2: Depends on shared + source-parsing
  file-system-lint-arwaky         ← shared, source-parsing
  file-watch-lint-arwaky          ← shared, source-parsing
  metrics-service-lint-arwaky     ← shared, source-parsing
  multi-project-lint-arwaky       ← shared, source-parsing
  code-analysis-lint-arwaky       ← shared, source-parsing

LEVEL 3: Depends on Level 2
  lifecycle-state-lint-arwaky     ← shared, pipeline-jobs
  import-rules-lint-arwaky        ← shared, source-parsing, file-system, output-report
  output-report-lint-arwaky       ← shared, source-parsing, code-analysis, pipeline-jobs
  pipeline-jobs-lint-arwaky       ← shared, source-parsing, multi-project
  config-system-lint-arwaky       ← shared, source-parsing, import-rules, pipeline-jobs
  naming-rules-lint-arwaky        ← shared, source-parsing, import-rules, output-report
  git-hooks-lint-arwaky           ← shared, source-parsing, output-report, pipeline-jobs
  role-rules-lint-arwaky          ← shared, source-parsing, import-rules, output-report

LEVEL 4: Depends on Level 3
  auto-fix-lint-arwaky            ← shared, source-parsing, code-analysis, output-report
  language-adapters-lint-arwaky   ← shared, source-parsing, code-analysis, metrics-service, output-report, pipeline-jobs
  plugin-system-lint-arwaky       ← shared, source-parsing, pipeline-jobs
  orphan-detector-lint-arwaky     ← shared, source-parsing, code-analysis, output-report
  project-setup-lint-arwaky       ← shared, source-parsing, pipeline-jobs

LEVEL 5: Depends on Level 4
  cli-commands-lint-arwaky        ← shared, source-parsing, auto-fix, code-analysis, output-report, pipeline-jobs, project-setup

LEVEL 6: Top-level
  mcp-server-lint-arwaky          ← shared, source-parsing, cli-commands, code-analysis, language-adapters, output-report, pipeline-jobs
```

### 7.3.1 Build Constraints

- Each feature crate compiles independently after its dependencies are satisfied
- `shared-lint-arwaky` is the foundation — must compile first
- `source-parsing-lint-arwaky` depends only on `shared-lint-arwaky`
- All other feature crates depend on `shared-lint-arwaky` + `source-parsing-lint-arwaky`
- Feature crates may depend on other feature crates (see dependency graph above)

### 7.4 Dependency Rules

```
agent          -> taxonomy, contract
surface        -> taxonomy, contract
capabilities   -> taxonomy, contract
infrastructure -> taxonomy, contract
contract       -> taxonomy
taxonomy       -> taxonomy
```

Surfaces must NOT import from `agent`, `capabilities`, or `infrastructure` directly — they access capabilities and infrastructure only through the **feature crate's container** or the `ServiceContainerAggregate` trait in the contract layer (AES001 sub-condition `surface_direct`). The `CompositionRoot` (in `root_compsotion_container.rs`, root layer) composes all feature containers and implements `ServiceContainerAggregate` for backward compatibility with existing surface commands.

---

## 8. CLI Interface

Subcommands are defined in `crates/cli-commands/src/` surfaces and dispatched from `root_cli_main_entry.rs`.

| Category | Subcommands                                                                |
| -------- | -------------------------------------------------------------------------- |
| Core     | check, scan, fix, report, ci, version, adapters, security, cancel          |
| Scans    | complexity, duplicates, trends, dependencies                               |
| Setup    | setup init, setup doctor, setup mcp-config, setup hermes                   |
| Dev      | diff, suggest, import, export, config, install-hook, uninstall-hook, watch |
| Git      | git-diff, multi-project                                                    |

---

## 9. MCP Interface (5 Tools)

| Tool                              | Purpose                                        |
| --------------------------------- | ---------------------------------------------- |
| `execute_command(action, args)` | Execute any CLI command                        |
| `list_commands(domain)`         | Discover available CLI commands                |
| `commands_schema(tool_name)`    | Retrieve the JSON Schema for a registered tool |
| `read_skill_context(section)`   | Read SKILL.md documentation by section         |
| `health_check()`                | Check linter adapter health and system state   |

---

## 10. Constraints

- Pure-Rust implementation (no embedded Python or Node.js runtime)
- No database required (file-based history only for trends)
- Static binary release via `cargo build --release`
- Platform: Linux

---

## 11. Workspace Crates (Cargo.toml members)

| Crate                     | Package Name                      | Path                                    | Internal Dependencies                                                                                                            | External Dependencies                                                                                    |
| ------------------------- | --------------------------------- | --------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| shared                    | `shared-lint-arwaky`            | `crates/shared`                       | None                                                                                                                             | `anyhow`, `async-trait`, `chrono`, `serde`, `serde_json`, `serde_yaml`, `thiserror`        |
| source-parsing            | `source_parsing-lint-arwaky`    | `crates/source-parsing`               | `shared`                                                                                                                       | `regex`, `serde`, `serde_json`                                                                     |
| file-system               | `file_system-lint-arwaky`       | `crates/file-system`                  | `shared`, `source-parsing`                                                                                                   | `async-trait`, `serde`, `serde_json`                                                               |
| file-watch                | `file_watch-lint-arwaky`        | `crates/file-watch`                   | `shared`, `source-parsing`                                                                                                   | `serde`, `serde_json`                                                                                |
| metrics-service           | `metrics_service-lint-arwaky`   | `crates/metrics-service`              | `shared`, `source-parsing`                                                                                                   | `serde`, `serde_json`                                                                                |
| multi-project             | `multi_project-lint-arwaky`     | `crates/multi-project`                | `shared`, `source-parsing`                                                                                                   | `async-trait`, `serde`, `serde_json`                                                               |
| code-analysis             | `code_analysis-lint-arwaky`     | `crates/code-analysis`                | `shared`, `source-parsing`, `file-system`                                                                                  | `async-trait`, `once_cell`, `regex`, `serde`, `serde_json`                                     |
| pipeline-jobs             | `pipeline_jobs-lint-arwaky`     | `crates/pipeline-jobs`                | `shared`, `source-parsing`, `multi-project`                                                                                | `async-trait`, `serde`, `serde_json`, `tokio`                                                    |
| lifecycle-state           | `lifecycle_state-lint-arwaky`   | `crates/lifecycle-state`              | `shared`, `pipeline-jobs`                                                                                                    | `async-trait`, `serde`, `serde_json`                                                               |
| output-report             | `output_report-lint-arwaky`     | `crates/output-report`                | `shared`, `source-parsing`, `code-analysis`, `pipeline-jobs`                                                             | `async-trait`, `serde`, `serde_json`                                                               |
| project-setup             | `project_setup-lint-arwaky`     | `crates/project-setup`                | `shared`, `source-parsing`, `pipeline-jobs`                                                                                | `async-trait`, `serde`, `serde_json`                                                               |
| plugin-system             | `plugin_system-lint-arwaky`     | `crates/plugin-system`                | `shared`, `source-parsing`, `pipeline-jobs`                                                                                | `async-trait`, `serde`, `serde_json`                                                               |
| git-hooks                 | `git_hooks-lint-arwaky`         | `crates/git-hooks`                    | `shared`, `source-parsing`, `output-report`, `pipeline-jobs`                                                             | `async-trait`, `serde`, `serde_json`                                                               |
| import-rules              | `import_rules-lint-arwaky`      | `crates/import-rules`                 | `shared`, `source-parsing`, `file-system`, `output-report`                                                               | `async-trait`, `serde`, `serde_json`                                                               |
| naming-rules              | `naming_rules-lint-arwaky`      | `crates/naming-rules`                 | `shared`, `source-parsing`, `import-rules`, `output-report`                                                              | `async-trait`, `regex`, `serde`, `serde_json`                                                    |
| role-rules                | `role_rules-lint-arwaky`        | `crates/role-rules`                   | `shared`, `source-parsing`, `import-rules`, `output-report`                                                              | `async-trait`, `once_cell`, `regex`, `serde`, `serde_json`                                     |
| orphan-detector           | `orphan_detector-lint-arwaky`   | `crates/orphan-detector`              | `shared`, `source-parsing`, `code-analysis`, `output-report`                                                             | `regex`, `serde`, `serde_json`                                                                     |
| auto-fix                  | `auto_fix-lint-arwaky`          | `crates/auto-fix`                     | `shared`, `source-parsing`, `code-analysis`, `output-report`                                                             | `serde`, `serde_json`                                                                                |
| config-system             | `config_system-lint-arwaky`     | `crates/config-system`                | `shared`, `source-parsing`, `import-rules`, `pipeline-jobs`                                                              | `async-trait`, `serde`, `serde_json`                                                               |
| language-adapters         | `language_adapters-lint-arwaky` | `crates/language-adapters`            | `shared`, `source-parsing`, `code-analysis`, `metrics-service`, `output-report`, `pipeline-jobs`                     | `async-trait`, `once_cell`, `regex`, `serde`, `serde_json`, `tracing`                        |
| cli-commands              | `cli_commands-lint-arwaky`      | `crates/cli-commands`                 | `shared`, `source-parsing`, `auto-fix`, `code-analysis`, `output-report`, `pipeline-jobs`, `project-setup`         | `async-trait`, `clap`, `console`, `dialoguer`, `futures`, `serde`, `serde_json`, `tokio` |
| mcp-server                | `mcp_server-lint-arwaky`        | `crates/mcp-server`                   | `shared`, `source-parsing`, `cli-commands`, `code-analysis`, `language-adapters`, `output-report`, `pipeline-jobs` | `once_cell`, `regex`, `serde`, `serde_json`, `tracing`                                         |
| root_compsotion_container | (local module)                    | `crates/root_compsotion_container.rs` | All workspace member crates                                                                                                      | `ctrlc`, `futures`, `tokio`                                                                        |
| root_cli_main_entry       | (binary)                          | `crates/root_cli_main_entry.rs`       | `cli-commands`, `root_compsotion_container`                                                                                  | `clap`, `ctrlc`, `tokio`                                                                           |
| root_mcp_main_entry       | (binary)                          | `crates/root_mcp_main_entry.rs`       | `mcp-server`, `root_compsotion_container`                                                                                    | `tokio`                                                                                                |
| root_tui_main_entry       | (binary)                          | `crates/root_tui_main_entry.rs`       | `cli-commands`                                                                                                                 | None                                                                                                     |

**Removed / Legacy**:

- `di-containers` → replaced by `CompositionRoot` + per-feature containers
- `legacy-di-container(shoudberemovelater)` → to be deleted
- `cli-transport` → merged into `cli-commands` (transport_container.rs)

---

## 12. Dependencies (Cargo.toml — external)

| Crate              | Version           | Purpose                       |
| ------------------ | ----------------- | ----------------------------- |
| serde              | 1.0               | Serialization framework       |
| serde_json         | 1.0               | JSON support                  |
| serde_yaml         | 0.9.34            | YAML config support           |
| toml               | 1.1.2             | TOML parsing (Cargo.lock)     |
| regex              | 1.10              | Pattern matching              |
| tokio              | 1.52.3 (full)     | Async runtime                 |
| async-trait        | 0.1.89            | Async trait support           |
| once_cell          | 1.21.4            | Global statics                |
| thiserror          | 1.0.52            | Error derive macros           |
| anyhow             | 1.0.102           | Error context                 |
| clap               | 4.6.1 (derive)    | CLI parsing                   |
| reqwest            | 0.13.4 (blocking) | HTTP client                   |
| chrono             | 0.4.44            | Date/time                     |
| mcp-sdk-rs         | 0.3.4             | MCP protocol (JSON-RPC 2.0)   |
| rand               | 0.10.1            | Random number generation      |
| tracing            | 0.1               | Structured logging            |
| tracing-subscriber | 0.3 (env-filter)  | Log filtering                 |
| rustsec            | 0.33              | Cargo audit / advisory DB     |
| dirs               | 6.0               | Platform-specific directories |
| dialoguer          | 0.11              | Interactive CLI prompts       |
| console            | 0.15              | Terminal styling & colors     |

---
