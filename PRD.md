# Product Requirements Document (PRD)

## Lint Arwaky — SIGNED OFF

---

## 1. Product Overview

**Name**: Lint Arwaky
**Type**: CLI tool + MCP server
**Version**: 1.10.14
**License**: MIT
**Language**: Rust (2021 edition)

Lint Arwaky is an autonomous multi-language linting, type-checking, and architectural rule auditing tool. It runs as a CLI binary (`lint-arwaky-cli`) and an MCP server (`lint-arwaky-mcp`) that exposes 5 tools over JSON-RPC 2.0.

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

| Value Driver             | Description                                                              |
| ------------------------ | ------------------------------------------------------------------------ |
| **Agent Autonomy** | Agents operate via MCP tools without human oversight                     |
| **Self-Healing**   | The `fix` command applies safe auto-fixes                              |
| **24/7 Quality**   | The `watch` command polls and re-lints continuously during development |

---

## 4. Target Users

| User                             | Interface        | Use Case                                                 |
| -------------------------------- | ---------------- | -------------------------------------------------------- |
| **AI Agents**              | MCP tools (5)    | Automated code review, pre-commit checks, CI integration |
| **Developers**             | CLI + MCP        | Local development, watch mode, git hooks                 |
| **Architecture Engineers** | AES rules        | Layer boundary enforcement, clean code                   |
| **CI/CD Pipelines**        | CLI + exit codes | Quality gates with exit codes                            |

---

## 5. Feature Requirements

**Vertical Slicing & Layer Boundary Rules:**

1. Layer determined by **file prefix** (`taxonomy_`, `contract_`, etc.), NOT by folder.
2. `infrastructure_` and `capabilities_` must not import each other directly (enforced by AES201).
3. Communication via `contract_` ports/protocols or `agent_` orchestrator.

### 5.1 `shared` — 

Taxonomy types and contract traits. Zero dependency on other workspace crates.

| ID     | Requirement                                                                 |
| ------ | --------------------------------------------------------------------------- |
| FR-001 | All `taxonomy_*` VOs, entities, events, errors, constants across features |
| FR-002 | All `contract_*` ports, protocols, and aggregates across features         |

### 5.2 `source-parsing` — Source Code Parsing

| ID     | Requirement                                                                                    |
| ------ | ---------------------------------------------------------------------------------------------- |
| FR-003 | Multi-Language Scanners — regex-based scanners for Rust, Python, JavaScript/TypeScript        |
| FR-004 | Import & Export Extraction — extract import statements and resolve symbol exports             |
| FR-007 | Symbol & Definition Mapping — index raw symbols, class/struct definitions, functions, methods |
| FR-129 | Path Normalization — normalize file paths and relative imports                                |

### 5.3 `file-system` — File System Abstraction

| ID     | Requirement                                                                 |
| ------ | --------------------------------------------------------------------------- |
| FR-028 | Directory Recursive Walking — walk directories filtering out ignored files |
| FR-029 | Glob Pattern Matching — locate files matching a search pattern             |
| FR-125 | File Read/Write Operations                                                  |
| FR-126 | Path Existence and Type Checks                                              |

### 5.4 `file-watch` — File Watching

| ID      | Requirement                                                                                   |
| ------- | --------------------------------------------------------------------------------------------- |
| FR-113a | Directory Snapshotting — snapshot project files and modification timestamps                  |
| FR-113b | File Modification Detection — detect new or modified files                                   |
| FR-113c | Ignore Patterns Filtering — filter `.git`, `node_modules`, `__pycache__` from watching |
| FR-113d | Event Trigger Dispatching — expose changed files for incremental linting                     |

### 5.5 `multi-project` — Multi-Project Governance

| ID      | Requirement                                                                                         |
| ------- | --------------------------------------------------------------------------------------------------- |
| FR-091a | Multi-Project Auto-Discovery — detect nested sub-projects by scanning config files                 |
| FR-091b | Workspace Compliance Aggregation — run lint across all projects and consolidate into single report |

### 5.6 `code-analysis` — Code Quality

| ID      | Requirement                                                                            | AES Code |
| ------- | -------------------------------------------------------------------------------------- | -------- |
| FR-025a | Maximum File Line Count Validation                                                     | AES301   |
| FR-025b | Minimum File Line Count Validation                                                     | AES302   |
| FR-030a | Attribute Bypass Detection —`#[allow(...)]`                                         | AES304   |
| FR-030b | Fatal Panic and Unwrap Detection —`panic!`, `unwrap()`, `expect()`              | AES304   |
| FR-030c | Comment-Based Linter Bypass Detection —`noqa`, `type: ignore`, `eslint-disable` | AES304   |
| FR-031a | Mandatory Definition Check — file must have a struct/enum/trait/class definition      | AES303   |
| FR-032a | Empty Struct and Trait Check — dead inheritance (empty impl blocks)                   | AES303   |
| FR-306  | Duplicate Code Detection                                                               | AES305   |

### 5.7 `import-rules` — Import Compliance

| ID      | Requirement                                                            | AES Code |
| ------- | ---------------------------------------------------------------------- | -------- |
| FR-010a | Layer Dependency Violation Scan — enforce unidirectional import flows | AES201   |
| FR-011a | Mandatory Imports Verification — check required imports per layer     | AES202   |
| FR-023  | Unused Import Check — symbol imported but never used                  | AES203   |
| FR-024  | Dummy Import Check — import matches forbidden dummy pattern           | AES204   |
| FR-050  | Circular dependency cycle analyzer — detect circular imports          | AES205   |

### 5.8 `config-system` — Config Loading

| ID     | Requirement                                                        |
| ------ | ------------------------------------------------------------------ |
| FR-002 | Multi-config YAML support, language detection, config-driven rules |

### 5.9 `naming-rules` — Naming Convention

| ID     | Requirement                                                                 | AES Code |
| ------ | --------------------------------------------------------------------------- | -------- |
| FR-020 | Naming convention checker — snake_case, lowercase, underscore, min 2 words | AES101   |
| FR-022 | Suffix/Prefix rules — suffix must match layer definition                   | AES102   |

### 5.10 `role-rules` — Role Violations

| ID      | Requirement                                                                       | AES Code |
| ------- | --------------------------------------------------------------------------------- | -------- |
| FR-034  | Taxonomy constant purity —`_constant` files: only `pub const`/`pub static` | AES401   |
| FR-027  | Primitive usage — no raw primitives in taxonomy domain types                     | AES401   |
| FR-035  | Contract primitive checker — contract uses VO/constants, not primitives          | AES402   |
| FR-037  | Capability role — capability must implement a protocol                           | AES403   |
| FR-045a | Capability Dispatch Method Check — capability routing bottleneck detection       | AES403   |
| FR-046a | Capability Load Balancing Check — capability dispatch load analysis              | AES403   |
| FR-404  | Infrastructure role — infra must implement contract, no direct surface import    | AES404   |
| FR-038  | Agent role — no `any` type, no low-level infra imports in orchestrators        | AES405   |
| FR-039  | Surface role — passive surface must not contain business logic                   | AES406   |

### 5.11 `git-hooks` — Git Hooks

| ID     | Requirement                                                |
| ------ | ---------------------------------------------------------- |
| FR-114 | Git pre-commit hook (`install-hook`, `uninstall-hook`) |

### 5.12 `auto-fix` — Auto-Fix Processor

| ID     | Requirement                                   |
| ------ | --------------------------------------------- |
| FR-005 | Apply safe auto-fixes (Rust + Python + JS/TS) |

### 5.13 `external-lint` — External Linter Adapters

| ID     | Requirement                                  |
| ------ | -------------------------------------------- |
| FR-070 | Run Clippy linting on Rust files             |
| FR-071 | Run rustfmt formatting check on Rust files   |
| FR-072 | Run cargo-audit dependency scan on Rust      |
| FR-073 | Run Ruff linting on Python files             |
| FR-074 | Run MyPy type checking on Python files       |
| FR-075 | Run Bandit security scanning on Python files |
| FR-078 | Run ESLint on JavaScript/TypeScript files    |
| FR-079 | Run Prettier formatting on JS/TS files       |
| FR-080 | Run TSC type checking on TypeScript files    |

### 5.14 `orphan-detector` — Orphan Code Detection

| ID     | Requirement                                         | AES Code    |
| ------ | --------------------------------------------------- | ----------- |
| FR-033 | Orphan code detector — unreachable/dead components | AES501–506 |

### 5.15 `project-setup` — Setup

| ID     | Requirement                                              |
| ------ | -------------------------------------------------------- |
| FR-060 | Environment diagnostics (`setup doctor`)               |
| FR-061 | Create default config (`setup init`)                   |
| FR-062 | MCP client config (`setup mcp-config --client <name>`) |

### 5.16 `cli-commands` — CLI Surface

| ID     | Requirement                                              |
| ------ | -------------------------------------------------------- |
| FR-055 | Full architecture compliance analysis (`check [path]`) |
| FR-056 | External project scan (`scan [path]`)                  |
| FR-057 | Apply safe fixes (`fix [path] [--dry-run]`)            |
| FR-059 | CI mode with exit codes (`ci [path] --threshold <N>`)  |
| FR-064 | List adapters (`adapters`)                             |
| FR-065 | Show config (`config show`)                            |
| FR-066 | Display version (`version`)                            |
| FR-091 | Multi-project lint (`multi-project <paths...>`)        |
| FR-092 | Orphan file check (`orphan <path>`)                    |
| FR-115 | CLI via `clap` 4.6 subcommand groups                   |

### 5.17 `mcp-server` — MCP Server

| ID     | Requirement                                        |
| ------ | -------------------------------------------------- |
| FR-100 | MCP server via JSON-RPC 2.0 (`mcp-sdk-rs` 0.3.4) |
| FR-101 | MCP tool:`execute_command(action, args)`         |
| FR-102 | MCP tool:`list_commands(domain)`                 |
| FR-103 | MCP tool:`command_schema(tool_name)`             |
| FR-104 | MCP tool:`read_skill(section)`                   |
| FR-105 | MCP tool:`health_check()`                        |

### 5.18 `maintenance` — Maintenance

| ID     | Requirement          |
| ------ | -------------------- |
| FR-130 | Python cache cleanup |

---

## 6. Architecture — AES Layered

6 layers determined by **file prefix** (NOT folder):

| Layer          | Prefix              | Allowed Suffixes                                                                                                                               |
| -------------- | ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| Taxonomy       | `taxonomy_`       | `_vo`, `_entity`, `_event`, `_error`, `_constant`, `_utility`, `_helper`                                                         |
| Contract       | `contract_`       | `_port`, `_protocol`, `_aggregate`                                                                                                       |
| Capabilities   | `capabilities_`   | `_checker`, `_analyzer`, `_processor`, etc.                                                                                              |
| Infrastructure | `infrastructure_` | `_adapter`, `_provider`, `_scanner`, etc.                                                                                                |
| Agent          | `agent_`          | `_orchestrator` (only)                                                                                                                       |
| Surface        | `surface_`        | `_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_hook`, `_store`, `_action`, `_screen` |
| Root           | `root_`           | `_container`, `_entry`                                                                                                                     |

---

## 7. AES Rules — 24 Codes Across 5 Groups

| Group   | Codes       | Count |
| ------- | ----------- | ----- |
| Naming  | AES101–102 | 2     |
| Import  | AES201–205 | 5     |
| Quality | AES301–305 | 5     |
| Role    | AES401–406 | 6     |
| Orphan  | AES501–506 | 6     |

---

## 8. CLI Interface

| Category       | Subcommands                                                                      |
| -------------- | -------------------------------------------------------------------------------- |
| Core           | check, scan, fix, ci, orphan                                                     |
| Git            | install-hook, uninstall-hook                                                     |
| Project        | watch                                                                            |
| Setup & Config | setup init, setup doctor, setup install, setup mcp-config, config show, adapters |
| Info           | version                                                                          |

---

## 9. MCP Interface (5 Tools)

| Tool                              | Purpose                                      |
| --------------------------------- | -------------------------------------------- |
| `execute_command(action, args)` | Execute any CLI command                      |
| `list_commands(domain)`         | Discover available CLI commands              |
| `command_schema(tool_name)`     | Retrieve JSON Schema for a registered tool   |
| `read_skill(section)`           | Read SKILL.md documentation by section       |
| `health_check()`                | Check linter adapter health and system state |

---

## 10. Constraints

- Pure-Rust implementation (no embedded Python or Node.js runtime)
- No database required
- Static binary release via `cargo build --release`
- Platform: Linux
