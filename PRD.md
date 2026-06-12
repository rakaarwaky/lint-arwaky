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

Requirements are organized by **dependency order** (Level 0 → Level 6). Each level builds upon the previous levels.

**Layer convention:** Layer determined by file prefix (`taxonomy_`, `contract_`, `capabilities_`, `infrastructure_`, `agent_`, `surface_`), NOT by folder.

### 5.1 Level 0: `shared` — Foundation (Zero Dependencies)

**Depends on:** Nothing

| ID     | Requirement                                                 | AES Codes |
| ------ | ----------------------------------------------------------- | --------- |
| FR-001 | All `taxonomy_*` VOs, entities, events, errors, constants | —        |
| FR-002 | All `contract_*` ports, protocols, aggregates             | —        |
| FR-003 | No dependencies on any feature crate                        | —        |

### 5.2 Level 1: `source-parsing` — Source Code Parsing

**Depends on:** `shared`

Empirical parsing component responsible for extracting the structural features of source code across Rust, Python, and JavaScript/TypeScript. Its sole responsibility is to scan raw files and standardize metadata (imports, symbols, inheritance, control flow, paths) into clean, language-agnostic Value Objects defined in `shared`. It enforces a strict separation of concerns: it behaves as an empirical data gatherer and does not perform validation or rule evaluation, leaving all architectural policy enforcement to upper-level rules modules.

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

### 5.3 Level 2: Core Infrastructure Features

**Depends on:** `shared`, `source-parsing`

#### 5.3.1 `file-system` — File System Abstraction

Infrastructure abstraction component responsible for physical disk I/O and file system queries. It acts as the project's single gateway to the operating system's file system, offering features like directory traversal, globbing, reading/writing files, and path validation. It maintains a strict boundary: it has no knowledge of code syntax or parsing rules (which belongs to `source-parsing`), nor does it enforce configuration or architectural validation policies. It treats files purely as raw data streams and directories as structural paths.

| ID     | Requirement                                                                                                         | AES Codes |
| ------ | ------------------------------------------------------------------------------------------------------------------- | --------- |
| FR-028 | Directory Recursive Walking — recursively walk a directory and gather file paths while filtering out ignored files | —        |
| FR-029 | Glob Pattern Matching — locate files matching a specific search pattern query                                      | —        |
| FR-125 | File Read/Write Operations — read string content from files and write string content to target files               | —        |
| FR-126 | Path Existence and Type Checks — check if paths exist and determine if they are files or directories               | —        |
| FR-127 | Path Resolution and Manipulation — get parent paths, working directory, relative path mapping, and path joining    | —        |

#### 5.3.2 `file-watch` — File Watching

| ID     | Requirement                            | AES Codes |
| ------ | -------------------------------------- | --------- |
| FR-113 | File watcher for auto-lint (`watch`) | —        |

#### 5.3.3 `metrics-service` — Metrics Provider

| ID     | Requirement                 | AES Codes |
| ------ | --------------------------- | --------- |
| FR-088 | Quality trends (`trends`) | —        |

#### 5.3.4 `multi-project` — Multi-Project Governance

| ID     | Requirement                                      | AES Codes |
| ------ | ------------------------------------------------ | --------- |
| FR-091 | Multi-project aggregate lint (`multi-project`) | —        |

#### 5.3.5 `code-analysis` — Code Quality & Auto-Fix

| ID     | Requirement                                                           | AES Codes |
| ------ | --------------------------------------------------------------------- | --------- |
| FR-025 | File size limit checker — max line threshold                         | AES020    |
| FR-026 | File minimum size checker — min line threshold                       | AES021    |
| FR-030 | Bypass comment violation detector — no #[allow], unwrap, panic, noqa | AES022    |
| FR-031 | Unused mandatory import detector — unused imports flagged            | AES023    |
| FR-032 | Dead inheritance bypass detector — empty struct/trait                | AES024    |
| FR-045 | Capability method existence checker — dispatch method exists         | AES0303   |
| FR-046 | Single capability bottleneck detector — balance dispatch routes      | AES0303   |

### 5.4 Level 3: Middle Features

**Depends on:** `shared`, `source-parsing`, Level 2 features

#### 5.4.1 `lifecycle-state` — Agent Lifecycle Management

**Depends on:** `shared`, `pipeline-jobs`

| ID     | Requirement                    | AES Codes |
| ------ | ------------------------------ | --------- |
| FR-006 | Track quality trends over time | —        |

#### 5.4.2 `import-rules` — Import Compliance

**Depends on:** `shared`, `source-parsing`, `file-system`, `output-report`

| ID     | Requirement                                                                    | AES Codes |
| ------ | ------------------------------------------------------------------------------ | --------- |
| FR-010 | Import layer violation detector — cross-layer import detection                | AES001    |
| FR-011 | Mandatory import missing detector — required imports per layer                | AES002    |
| FR-013 | Root layer detection — forbidden root import patterns                         | AES004    |
| FR-014 | Layer suffix mismatch detector — file suffix must match layer                 | AES005    |
| FR-015 | Contract suffix mismatch detector — contract needs _port/_protocol/_aggregate | AES006    |
| FR-017 | Surface direct import checker — no direct infra/cap imports                   | AES003    |

#### 5.4.3 `output-report` — Output Formatting & Report Generation

**Depends on:** `shared`, `source-parsing`, `code-analysis`, `pipeline-jobs`

| ID     | Requirement                                                          | AES Codes |
| ------ | -------------------------------------------------------------------- | --------- |
| FR-058 | Generate quality report (`report [path] --output-format <format>`) | —        |
| FR-095 | Text (human-readable)                                                | —        |
| FR-096 | JSON (machine-readable)                                              | —        |
| FR-097 | SARIF 2.1.0 (GitHub Code Scanning)                                   | —        |
| FR-098 | JUnit XML (Jenkins/CI)                                               | —        |

#### 5.4.4 `pipeline-jobs` — Jobs, Dispatcher, Execution

**Depends on:** `shared`, `source-parsing`, `multi-project`

| ID     | Requirement                           | AES Codes |
| ------ | ------------------------------------- | --------- |
| FR-067 | Cancel lint job (`cancel <job_id>`) | —        |

#### 5.4.5 `config-system` — Config Loading & Parsing

**Depends on:** `shared`, `source-parsing`, `import-rules`, `pipeline-jobs`

| ID     | Requirement                                                        | AES Codes |
| ------ | ------------------------------------------------------------------ | --------- |
| FR-002 | Multi-config YAML support, language detection, config-driven rules | —        |
| FR-048 | Constant purity checker — _constant files: only pub const/static  | AES015    |
| FR-040 | MCP schema checker — MCP tools need docstrings + JSON Schema      | AES025    |

#### 5.4.6 `naming-rules` — Naming Convention

**Depends on:** `shared`, `source-parsing`, `import-rules`, `output-report`

| ID     | Requirement                                                                     | AES Codes |
| ------ | ------------------------------------------------------------------------------- | --------- |
| FR-020 | Naming convention checker — strict word snake_case                             | AES010    |
| FR-021 | Mandatory struct/trait definition checker — every file needs struct/enum/trait | AES011    |

#### 5.4.7 `git-hooks` — Git Hooks Management

**Depends on:** `shared`, `source-parsing`, `output-report`, `pipeline-jobs`

| ID     | Requirement                                                | AES Codes |
| ------ | ---------------------------------------------------------- | --------- |
| FR-114 | Git pre-commit hook (`install-hook`, `uninstall-hook`) | —        |

#### 5.4.8 `role-rules` — Role Violations

**Depends on:** `shared`, `source-parsing`, `import-rules`, `output-report`

| ID     | Requirement                                                           | AES Codes |
| ------ | --------------------------------------------------------------------- | --------- |
| FR-035 | Surface hierarchy violation detector — utility imports smart surface | AES0306   |
| FR-036 | Passive surface violation detector — passive imports taxonomy only   | AES0306   |
| FR-037 | Agent role violation detector — behavioral mandates per agent role   | AES0305   |
| FR-038 | Agent any-bypass detector — no `any` type in orchestrators         | AES0305   |
| FR-027 | Primitive usage checker — no raw primitives in domain types          | AES016    |

### 5.5 Level 4: Upper Features

**Depends on:** `shared`, `source-parsing`, Level 3 features

#### 5.5.1 `auto-fix` — Auto-Fix Processor

**Depends on:** `shared`, `source-parsing`, `code-analysis`, `output-report`

| ID     | Requirement                                   | AES Codes |
| ------ | --------------------------------------------- | --------- |
| FR-005 | Apply safe auto-fixes (Rust + Python + JS/TS) | AES0303   |

#### 5.5.2 `language-adapters` — External Linter Adapters

**Depends on:** `shared`, `source-parsing`, `code-analysis`, `metrics-service`, `output-report`, `pipeline-jobs`

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

#### 5.5.3 `plugin-system` — Plugin Discovery & Management

**Depends on:** `shared`, `source-parsing`, `pipeline-jobs`

| ID     | Requirement                           | AES Codes |
| ------ | ------------------------------------- | --------- |
| FR-089 | Dependency listing (`dependencies`) | —        |

#### 5.5.4 `orphan-detector` — Orphan Code Detection

**Depends on:** `shared`, `source-parsing`, `code-analysis`, `output-report`

| ID     | Requirement                                    | AES Codes |
| ------ | ---------------------------------------------- | --------- |
| FR-033 | Orphan code detector — unreachable components | AES030    |

### 5.6 Level 5: Surface Features

**Depends on:** `shared`, `source-parsing`, Level 4 features

#### 5.6.1 `cli-commands` — CLI Surfaces + Transport (Root Layer)

**Depends on:** `shared`, `source-parsing`, `auto-fix`, `code-analysis`, `output-report`, `pipeline-jobs`

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

#### 5.6.2 `project-setup` — Project Init, Doctor, MCP Config

**Depends on:** `shared`, `source-parsing`, `cli-commands`, `pipeline-jobs`

| ID     | Requirement                                              | AES Codes |
| ------ | -------------------------------------------------------- | --------- |
| FR-060 | Environment diagnostics (`setup doctor`)               | —        |
| FR-061 | Create default config (`setup init`)                   | —        |
| FR-062 | MCP client config (`setup mcp-config --client <name>`) | —        |
| FR-063 | Hermes integration (`setup hermes [--remove]`)         | —        |

### 5.7 Level 6: Top-Level

**Depends on:** `shared`, `source-parsing`, Level 5 features

#### 5.7.1 `mcp-server` — MCP JSON-RPC 2.0 Server (Root Layer)

**Depends on:** `shared`, `source-parsing`, `cli-commands`, `code-analysis`, `language-adapters`, `output-report`, `pipeline-jobs`

| ID     | Requirement                                        | AES Codes |
| ------ | -------------------------------------------------- | --------- |
| FR-100 | MCP server via JSON-RPC 2.0 (`mcp-sdk-rs` 0.3.4) | FR-055    |
| FR-101 | MCP tool:`execute_command(action, args)`         | FR-100    |
| FR-102 | MCP tool:`list_commands(domain)`                 | FR-100    |
| FR-103 | MCP tool:`commands_schema(tool_name)`            | FR-100    |
| FR-104 | MCP tool:`read_skill_context(section)`           | FR-100    |
| FR-105 | MCP tool:`health_check()`                        | FR-100    |
| FR-106 | CI/CD integration (OIDC, SLSA Provenance)          | FR-100    |

### 5.8 Cross-Cutting Requirements

These requirements span multiple crates and are not tied to a single level.

#### 5.8.1 Analysis & Scan Subcommands

| ID     | Requirement                                     | AES Codes |
| ------ | ----------------------------------------------- | --------- |
| FR-085 | Security vulnerability scan (`security`)      | FR-075    |
| FR-086 | Cyclomatic complexity analysis (`complexity`) | FR-076    |
| FR-087 | Code duplication detection (`duplicates`)     | FR-055    |

#### 5.8.2 Semantic Analysis (Enrichment)

| ID     | Requirement                                            | AES Codes |
| ------ | ------------------------------------------------------ | --------- |
| FR-120 | Show enclosing scope (function/class) for violations   | FR-003    |
| FR-121 | Trace call chains across project                       | FR-003    |
| FR-122 | Track variable flow within scope                       | FR-003    |
| FR-123 | Project-wide symbol rename                             | FR-003    |
| FR-124 | Generate naming variants (snake_case, camelCase, etc.) | FR-003    |

#### 5.8.3 Circular Dependency Detection

| ID     | Requirement                                                   | AES Codes |
| ------ | ------------------------------------------------------------- | --------- |
| FR-050 | Circular dependency cycle analyzer — detect circular imports | AES012    |

#### 5.8.4 Forbidden Inheritance & Mandatory Contract Implementation

| ID     | Requirement                                                           | AES Codes |
| ------ | --------------------------------------------------------------------- | --------- |
| FR-041 | Forbidden inheritance detector — aggregate not inherit port/protocol | AES013    |
| FR-042 | Mandatory inheritance checker — every file implements a contract     | AES014    |

#### 5.8.5 Surface Layer Rules (Code Structure)

| ID     | Requirement                                                                   | AES Codes |
| ------ | ----------------------------------------------------------------------------- | --------- |
| FR-016 | Surface layer rule checker — surfaces delegate via ServiceContainerAggregate | AES0306   |

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
  composition_root.rs   -- Root composition (root layer)
  cli_main_entry.rs     -- CLI binary entry (root_entry)
  mcp_main_entry.rs     -- MCP binary entry (root_entry)
  tui_main_entry.rs     -- TUI binary entry (root_entry)

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
| `file-watch`        | **MISSING** (needs `file_watch_container.rs`)        | File watching (infrastructure + contracts)                                                                   |
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

LEVEL 5: Depends on Level 4
  cli-commands-lint-arwaky        ← shared, source-parsing, auto-fix, code-analysis, output-report, pipeline-jobs
  project-setup-lint-arwaky       ← shared, source-parsing, cli-commands, pipeline-jobs

LEVEL 6: Top-level
  mcp-server-lint-arwaky          ← shared, source-parsing, cli-commands, code-analysis, language-adapters, output-report, pipeline-jobs

DELETE: di-containers-lint-arwaky (God Container — will be replaced by per-feature containers)
```

### 7.3.1 Build Constraints

- Each feature crate compiles independently after its dependencies are satisfied
- `shared-lint-arwaky` is the foundation — must compile first
- `source-parsing-lint-arwaky` depends only on `shared-lint-arwaky`
- All other feature crates depend on `shared-lint-arwaky` + `source-parsing-lint-arwaky`
- Feature crates may depend on other feature crates (see dependency graph above)
- `di-containers-lint-arwaky` is deprecated — will be replaced by per-feature containers

### 7.4 Dependency Rules

```
agent          -> taxonomy, contract
surface        -> taxonomy, contract
capabilities   -> taxonomy, contract
infrastructure -> taxonomy, contract
contract       -> taxonomy
taxonomy       -> taxonomy
```

Surfaces must NOT import from `agent`, `capabilities`, or `infrastructure` directly — they access capabilities and infrastructure only through the **feature crate's container** or the `ServiceContainerAggregate` trait in the contract layer (AES001 sub-condition `surface_direct`). The `CompositionRoot` (in `composition_root.rs`, root layer) composes all feature containers and implements `ServiceContainerAggregate` for backward compatibility with existing surface commands.

### 7.5 MCP Server Architecture

The MCP server uses `mcp-sdk-rs` 0.3.4 over JSON-RPC 2.0 on stdin/stdout. It announces `protocolVersion: 2024-11-05` and exposes the `tools` capability.

```
mcp_main_entry.rs    -- tokio main loop, reads JSON-RPC from stdin
mcp_container.rs     -- wires MCP server dependencies
mcp_tools_command.rs -- execute_command / list_commands / commands_schema /
                        read_skill_context / health_check
mcp_server_handler.rs / mcp_server_wrapper.rs -- Schema, validation, lifespan
```

The DI container is created once at server start; the same `Arc<dyn ServiceContainerAggregate>` is passed to every tool call.

### 7.6 Agentic Engineering System (AES) v1.10.11

Severity levels and their point penalty per finding:

| Severity | Penalty | Description                                   |
| -------- | ------- | --------------------------------------------- |
| LOW      | -1      | Minor style or naming issue                   |
| MEDIUM   | -2      | Structural concern, import patterns           |
| HIGH     | -3      | Architecture violation, mandatory requirement |
| CRITICAL | -5      | Bypass markers, dead inheritance, layer fraud |

Total score starts at 100.0 and is deducted per finding. If any CRITICAL finding exists, the run fails regardless of score.

**AES016 Primitive Policy**: Value Object enforcement is **granular per layer**:

- `contract` and `taxonomy(entity|error|event)` → `no_primitives: true` (strict)
- `infrastructure`, `capabilities`, `surfaces` → `no_primitives: false` (adapter layers may use primitives as supporting types)
- `taxonomy(constant)` → raw primitives allowed by definition; must contain ONLY constant declarations (AES0301)

**AES015 Constant Purity (v2.0)**: Taxonomy files ending in `_constant` must contain only `pub const` / `pub static` declarations. Any `struct`, `enum`, `fn`, or `impl` block in a `_constant` file is a violation.

See [RULES_AES.md](RULES_AES.md) for the full rule catalog (27 active codes across 4 groups) and [ARCHITECTURE.md](ARCHITECTURE.md) for the layered specification with Mermaid diagrams.

---

## 8. MCP Interface (5 Tools)

| Tool                              | Purpose                                        |
| --------------------------------- | ---------------------------------------------- |
| `execute_command(action, args)` | Execute any CLI command                        |
| `list_commands(domain)`         | Discover available CLI commands                |
| `commands_schema(tool_name)`    | Retrieve the JSON Schema for a registered tool |
| `read_skill_context(section)`   | Read SKILL.md documentation by section         |
| `health_check()`                | Check linter adapter health and system state   |

---

## 9. CLI Interface

Subcommands are defined in `crates/cli-commands/src/` surfaces and dispatched from `cli_main_entry.rs`.

| Category | Subcommands                                                                |
| -------- | -------------------------------------------------------------------------- |
| Core     | check, scan, fix, report, ci, version, adapters, security, cancel          |
| Scans    | complexity, duplicates, trends, dependencies                               |
| Setup    | setup init, setup doctor, setup mcp-config, setup hermes                   |
| Dev      | diff, suggest, import, export, config, install-hook, uninstall-hook, watch |
| Git      | git-diff, multi-project                                                    |

---

## 10. Constraints

- Pure-Rust implementation (no embedded Python or Node.js runtime)
- No database required (file-based history only for trends)
- Static binary release via `cargo build --release`
- Platform: Linux

---

## 11. Workspace Crates (Cargo.toml members)

| Crate             | Package Name                      | Path                           |
| ----------------- | --------------------------------- | ------------------------------ |
| shared            | `shared-lint-arwaky`            | `crates/shared`              |
| import-rules      | `import_rules-lint-arwaky`      | `crates/import-rules`        |
| naming-rules      | `naming_rules-lint-arwaky`      | `crates/naming-rules`        |
| role-rules        | `role_rules-lint-arwaky`        | `crates/role-rules`          |
| orphan-detector   | `orphan_detector-lint-arwaky`   | `crates/orphan-detector`     |
| code-analysis     | `code_analysis-lint-arwaky`     | `crates/code-analysis`       |
| auto-fix          | `auto_fix-lint-arwaky`          | `crates/auto-fix`            |
| config-system     | `config_system-lint-arwaky`     | `crates/config-system`       |
| pipeline-jobs     | `pipeline_jobs-lint-arwaky`     | `crates/pipeline-jobs`       |
| source-parsing    | `source_parsing-lint-arwaky`    | `crates/source-parsing`      |
| language-adapters | `language_adapters-lint-arwaky` | `crates/language-adapters`   |
| file-system       | `file_system-lint-arwaky`       | `crates/file-system`         |
| file-watch        | `file_watch-lint-arwaky`        | `crates/file-watch`          |
| git-hooks         | `git_hooks-lint-arwaky`         | `crates/git-hooks`           |
| multi-project     | `multi_project-lint-arwaky`     | `crates/multi-project`       |
| project-setup     | `project_setup-lint-arwaky`     | `crates/project-setup`       |
| plugin-system     | `plugin_system-lint-arwaky`     | `crates/plugin-system`       |
| output-report     | `output_report-lint-arwaky`     | `crates/output-report`       |
| lifecycle-state   | `lifecycle_state-lint-arwaky`   | `crates/lifecycle-state`     |
| metrics-service   | `metrics_service-lint-arwaky`   | `crates/metrics-service`     |
| cli-commands      | `cli_commands-lint-arwaky`      | `crates/cli-commands`        |
| mcp-server        | `mcp_server-lint-arwaky`        | `crates/mcp-server`          |
| composition_root  | (local module)                    | `crates/composition_root.rs` |
| cli_main_entry    | (binary)                          | `crates/cli_main_entry.rs`   |
| mcp_main_entry    | (binary)                          | `crates/mcp_main_entry.rs`   |
| tui_main_entry    | (binary)                          | `crates/tui_main_entry.rs`   |

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
