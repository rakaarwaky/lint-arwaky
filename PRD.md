# Product Requirements Document (PRD)

## Lint Arwaky v1.10.6 — SIGNED OFF

---

## 1. Product Overview

**Name**: Lint Arwaky
**Type**: CLI tool + MCP server + TUI launcher
**Version**: 1.10.6
**License**: MIT
**Language**: Rust (2021 edition)

Lint Arwaky is an autonomous multi-language linting, type-checking, and architectural rule auditing tool. It runs as a CLI binary (`lint-arwaky-cli`), an MCP server (`lint-arwaky-mcp`) that exposes 5 tools over JSON-RPC 2.0, and an interactive TUI launcher (`lint-arwaky-tui`).

The project audits itself: `lint-arwaky-cli check .` runs the same AES rule engine against `src-rust/` that it runs against third-party code.

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

| Value Driver       | Description                                                                          |
| ------------------ | ------------------------------------------------------------------------------------ |
| **Agent Autonomy** | Agents operate via 5 MCP tools without human oversight                               |
| **Job Tracking**   | Jobs are tracked in a thread-safe registry (in-memory, per-process)                  |
| **Self-Healing**   | The `fix` command applies safe auto-fixes; the `suggest` command guides manual fixes |
| **24/7 Quality**   | The `watch` command polls and re-lints continuously during development               |

---

## 4. Target Users

| User                       | Interface          | Use Case                                                 |
| -------------------------- | ------------------ | -------------------------------------------------------- |
| **AI Agents**              | MCP tools (5)      | Automated code review, pre-commit checks, CI integration |
| **Prototype Developers**   | MCP + CLI          | Fast iterations, AI-assisted coding, quality gates       |
| **Architecture Engineers** | Architecture tools | AES rule enforcement, clean code, DDD                    |
| **Developers**             | CLI (20+ commands) | Local development, watch mode, git hooks                 |
| **CI/CD Pipelines**        | CLI + exit codes   | Quality gates, SARIF/JUnit/JSON reports                  |
| **Contributors**           | GitHub + PRs       | New adapters, CLI commands, MCP tools                    |

---

## 5. Feature Requirements

### 5.1 Core Platform (Foundation)

| ID     | Requirement                                                                                                                                                                                                                                             | Dependency             |
| ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------- |
| FR-001 | 6-layer AES architecture enforcement — layer hierarchy, dependency direction, sibling equivalence across taxonomy, contract, capabilities, infrastructure, agent, surfaces — see[`docs/FRD_001_6layer_architecture.md`](FRD_001_6layer_architecture.md) | —                      |
| FR-002 | Config system: multi-config support (`lint_arwaky.config.rust.yaml`, `.python.yaml`, `.javascript.yaml`), YAML reader, language detection, config-driven rules — see [`docs/FRD_002_config_yaml_parser.md`](FRD_002_config_yaml_parser.md)              | FR-001                 |
| FR-003 | Source code parsing for Rust, Python, JavaScript/TypeScript — regex-based line scanners (not true AST parsers; see[`docs/FRD_003_ast_scanning.md`](FRD_003_ast_scanning.md))                                                                            | FR-001                 |
| FR-004 | Self-lint target (`lint-arwaky-cli check .`) — project audits itself                                                                                                                                                                                    | FR-001, FR-002, FR-003 |
| FR-005 | Apply safe auto-fixes (Rust + Python + JS/TS)                                                                                                                                                                                                           | FR-003                 |
| FR-006 | Track quality trends over time                                                                                                                                                                                                                          | FR-004                 |

### 5.2 Layer Import Rules

| ID     | Requirement                                                                                                                                                                                                                                     | Dependency |
| ------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| FR-010 | **Import layer violation detector** (AES001) — cross-layer import detection                                                                                                                                                                     | FR-001     |
| FR-011 | **Mandatory import missing detector** (AES002) — required imports per layer                                                                                                                                                                     | FR-001     |
| FR-013 | **Root layer detection** (AES004) — forbidden root import patterns                                                                                                                                                                              | FR-001     |
| FR-014 | **Layer suffix mismatch detector** (AES005) — file suffix must match layer                                                                                                                                                                      | FR-001     |
| FR-015 | **Contract suffix mismatch detector** (AES006) — contract needs \_port/\_protocol/\_aggregate                                                                                                                                                   | FR-001     |
| FR-016 | **Surface layer rule checker** (AES0306) — surface must not implement domain logic; Smart surfaces parse input and delegate via `ServiceContainerAggregate`; Passive surfaces (`_component`, `_layout`, `_view`) import taxonomy only (AES0306) | FR-001     |
| FR-017 | **Surface direct import checker** (AES003) — no direct infra/cap imports                                                                                                                                                                        | FR-001     |

### 5.3 Naming & Structure Rules

| ID     | Requirement                                                                                 | Dependency |
| ------ | ------------------------------------------------------------------------------------------- | ---------- |
| FR-020 | **Naming convention checker** (AES010) — strict word snake_case                             | FR-003     |
| FR-021 | **Mandatory struct/trait definition checker** (AES011) — every file needs struct/enum/trait | FR-003     |

### 5.4 File & Content Rules

| ID     | Requirement                                                              | Dependency |
| ------ | ------------------------------------------------------------------------ | ---------- |
| FR-025 | **File size limit checker** (AES020) — max line threshold                | FR-003     |
| FR-026 | **File minimum size checker** (AES021) — min line threshold              | FR-003     |
| FR-027 | **Primitive usage checker** (AES016) — no raw primitives in domain types | FR-003     |

### 5.5 Code Quality & Bypass Detection

| ID     | Requirement                                                                      | Dependency |
| ------ | -------------------------------------------------------------------------------- | ---------- |
| FR-030 | **Bypass comment violation detector** (AES022) — no #[allow, unwrap, panic, noqa | FR-003     |
| FR-031 | **Unused mandatory import detector** (AES023) — unused imports flagged           | FR-003     |
| FR-032 | **Dead inheritance bypass detector** (AES024) — empty struct/trait               | FR-003     |
| FR-033 | **Orphan code detector** (AES030) — unreachable components                       | FR-003     |

### 5.6 Surface & Agent Rules

| ID     | Requirement                                                                        | Dependency |
| ------ | ---------------------------------------------------------------------------------- | ---------- |
| FR-035 | **Surface hierarchy violation detector** (AES0306) — utility imports smart surface | FR-001     |
| FR-036 | **Passive surface violation detector** (AES0306) — passive imports taxonomy only   | FR-001     |
| FR-037 | **Agent role violation detector** (AES0305) — behavioral mandates per agent role   | FR-001     |
| FR-038 | **Agent any-bypass detector** (AES0305) — no `any` type in orchestrators           | FR-003     |

### 5.7 Contract & Aggregate Rules

| ID     | Requirement                                                                       | Dependency |
| ------ | --------------------------------------------------------------------------------- | ---------- |
| FR-040 | **MCP schema checker** (AES025) — MCP tools need docstrings + JSON Schema         | FR-003     |
| FR-041 | **Forbidden inheritance detector** (AES013) — aggregate not inherit port/protocol | FR-003     |
| FR-042 | **Mandatory inheritance checker** (AES014) — every file implements a contract     | FR-003     |

### 5.8 Capability Dispatch & Constants

| ID     | Requirement                                                                    | Dependency |
| ------ | ------------------------------------------------------------------------------ | ---------- |
| FR-045 | **Capability method existence checker** (AES0303) — dispatch method exists     | FR-003     |
| FR-046 | **Single capability bottleneck detector** (AES0303) — balance dispatch routes  | FR-003     |
| FR-048 | **Constant purity checker** (AES015) — \_constant files: only pub const/static | FR-003     |

### 5.9 Project-Wide Analysis

| ID     | Requirement                                                               | Dependency |
| ------ | ------------------------------------------------------------------------- | ---------- |
| FR-050 | **Circular dependency cycle analyzer** (AES012) — detect circular imports | FR-003     |

### 5.10 CLI Interface

| ID     | Requirement                                                         | Dependency    |
| ------ | ------------------------------------------------------------------- | ------------- |
| FR-055 | Full architecture compliance analysis (`check [path] [--git-diff]`) | FR-001–FR-050 |
| FR-056 | External project scan (`scan [path]`) — AES + all external adapters | FR-055        |
| FR-057 | Apply safe fixes (`fix [path]`)                                     | FR-005        |
| FR-058 | Generate quality report (`report [path] --output-format <format>`)  | FR-055        |
| FR-059 | CI mode with exit codes (`ci [path] --threshold <N>`)               | FR-055        |
| FR-060 | Environment diagnostics (`setup doctor`)                            | FR-002        |
| FR-061 | Create default config (`setup init`)                                | FR-002        |
| FR-062 | MCP client config (`setup mcp-config --client <name>`)              | FR-110        |
| FR-063 | Hermes integration (`setup hermes [--remove]`)                      | FR-110        |
| FR-064 | List adapters (`adapters`)                                          | FR-055        |
| FR-065 | Show config (`config show`)                                         | FR-002        |
| FR-066 | Display version (`version`)                                         | —             |
| FR-067 | Cancel lint job (`cancel <job_id>`)                                 | FR-055        |

### 5.11 Multi-Language Linting (External Adapters)

| ID     | Requirement                                           | Dependency |
| ------ | ----------------------------------------------------- | ---------- |
| FR-070 | Run Clippy linting on Rust files                      | FR-055     |
| FR-071 | Run rustfmt formatting check on Rust files            | FR-055     |
| FR-072 | Run cargo-audit dependency vulnerability scan on Rust | FR-055     |
| FR-073 | Run Ruff linting on Python files                      | FR-055     |
| FR-074 | Run MyPy type checking on Python files                | FR-055     |
| FR-075 | Run Bandit security scanning on Python files          | FR-055     |
| FR-076 | Run Radon-style complexity analysis on Python files   | FR-055     |
| FR-077 | Run pip-audit dependency vulnerability scan on Python | FR-055     |
| FR-078 | Run ESLint on JavaScript/TypeScript files             | FR-055     |
| FR-079 | Run Prettier formatting on JS/TS files                | FR-055     |
| FR-080 | Run TSC type checking on TypeScript files             | FR-055     |

### 5.12 Analysis & Scan Subcommands

| ID     | Requirement                                    | Dependency |
| ------ | ---------------------------------------------- | ---------- |
| FR-085 | Security vulnerability scan (`security`)       | FR-075     |
| FR-086 | Cyclomatic complexity analysis (`complexity`)  | FR-076     |
| FR-087 | Code duplication detection (`duplicates`)      | FR-055     |
| FR-088 | Quality trends (`trends`)                      | FR-006     |
| FR-089 | Dependency listing (`dependencies`)            | FR-072     |
| FR-090 | Git diff lint (`git-diff`)                     | FR-055     |
| FR-091 | Multi-project aggregate lint (`multi-project`) | FR-055     |

### 5.13 Report Formats

| ID     | Format                             | Dependency |
| ------ | ---------------------------------- | ---------- |
| FR-095 | Text (human-readable)              | FR-055     |
| FR-096 | JSON (machine-readable)            | FR-055     |
| FR-097 | SARIF 2.1.0 (GitHub Code Scanning) | FR-055     |
| FR-098 | JUnit XML (Jenkins/CI)             | FR-055     |

### 5.14 MCP Integration (AI Agent Interface)

| ID     | Requirement                                      | Dependency |
| ------ | ------------------------------------------------ | ---------- |
| FR-100 | MCP server via JSON-RPC 2.0 (`mcp-sdk-rs` 0.3.4) | FR-055     |
| FR-101 | MCP tool:`execute_command(action, args)`         | FR-100     |
| FR-102 | MCP tool:`list_commands(domain)`                 | FR-100     |
| FR-103 | MCP tool:`commands_schema(tool_name)`            | FR-100     |
| FR-104 | MCP tool:`read_skill_context(section)`           | FR-100     |
| FR-105 | MCP tool:`health_check()`                        | FR-100     |
| FR-106 | CI/CD integration (OIDC, SLSA Provenance)        | FR-100     |

### 5.15 Dev & Utility Tools

| ID     | Requirement                                            | Dependency |
| ------ | ------------------------------------------------------ | ---------- |
| FR-110 | Compare violation diff between paths (`diff`)          | FR-055     |
| FR-111 | AI-powered fix suggestions (`suggest`)                 | FR-057     |
| FR-112 | Import/export configuration (`import`, `export`)       | FR-002     |
| FR-113 | File watcher for auto-lint (`watch`)                   | FR-055     |
| FR-114 | Git pre-commit hook (`install-hook`, `uninstall-hook`) | FR-055     |
| FR-115 | CLI via `clap` 4.6 subcommand groups                   | FR-001     |
| FR-116 | Direct command execution via `std::process::Command`   | FR-001     |

### 5.16 Semantic Analysis (Enrichment)

| ID     | Requirement                                            | Dependency |
| ------ | ------------------------------------------------------ | ---------- |
| FR-120 | Show enclosing scope (function/class) for violations   | FR-003     |
| FR-121 | Trace call chains across project                       | FR-003     |
| FR-122 | Track variable flow within scope                       | FR-003     |
| FR-123 | Project-wide symbol rename                             | FR-003     |
| FR-124 | Generate naming variants (snake_case, camelCase, etc.) | FR-003     |

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

### 7.1 Domain Model (6 Layers)

```
src-rust/
  agent/           -- Lifecycle, orchestration, pipeline, DI container
  capabilities/    -- Use-case logic: analysis, formatting, architecture
  contract/        -- Interfaces: traits, protocols, aggregates
  infrastructure/  -- Adapters: rust_linter, python_ruff, eslint, transports
  surfaces/        -- Interfaces: CLI (clap), MCP (mcp-sdk-rs)
  taxonomy/        -- Domain types: value objects, models, errors, constants
```

### 7.2 Dependency Rules

```
agent          -> taxonomy, contract, infrastructure, capabilities
surface        -> taxonomy, contract
capabilities   -> taxonomy, contract
infrastructure -> taxonomy, contract
contract       -> taxonomy
taxonomy       -> taxonomy
```

Surfaces must NOT import from `agent`, `capabilities`, or `infrastructure` directly — they access capabilities and infrastructure only through the `ServiceContainerAggregate` trait in the contract layer (AES001 sub-condition surface_direct). The DI container is created in `cli_main_entry.rs` (root layer, not a surface) and passed to surfaces. This enforces strict dependency inversion per ARCHITECTURE.md.

### 7.3 MCP Server Architecture

The MCP server uses `mcp-sdk-rs` 0.3.4 over JSON-RPC 2.0 on stdin/stdout. It announces `protocolVersion: 2024-11-05` and exposes the `tools` capability.

```
mcp_main_entry.rs    -- tokio main loop, reads JSON-RPC from stdin
mcp_tools_command.rs -- execute_command / list_commands / commands_schema /
                        read_skill_context / health_check
mcp_server_handler.rs / mcp_server_wrapper.rs -- Schema, validation, lifespan
mcp_command_handler.rs -- command_catalog and dispatch
```

The DI container is created once at server start; the same `Arc<dyn ServiceContainerAggregate>` is passed to every tool call.

### 7.4 Agentic Engineering System (AES) v1.10.6

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

See [docs/AES_RULES.md](docs/AES_RULES.md) for the full rule catalog (27 active codes across 4 groups) and [docs/AESArchitecture.md](docs/AESArchitecture.md) for the layered specification with Mermaid diagrams.

---

## 8. MCP Interface (5 Tools)

| Tool                            | Purpose                                        |
| ------------------------------- | ---------------------------------------------- |
| `execute_command(action, args)` | Execute any CLI command                        |
| `list_commands(domain)`         | Discover available CLI commands                |
| `commands_schema(tool_name)`    | Retrieve the JSON Schema for a registered tool |
| `read_skill_context(section)`   | Read SKILL.md documentation by section         |
| `health_check()`                | Check linter adapter health and system state   |

> **Note**: Job cancellation is exposed as the CLI subcommand `lint-arwaky-cli cancel <job_id>` rather than an MCP tool.

---

## 9. CLI Interface

Subcommands are defined in `src-rust/surfaces/cli_core_command.rs` and dispatched from `cli_main_entry.rs`.

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

## 11. Dependencies (Cargo.toml)

| Crate              | Version           | Purpose                   |
| ------------------ | ----------------- | ------------------------- |
| serde              | 1.0               | Serialization framework   |
| serde_json         | 1.0               | JSON support              |
| serde_yaml         | 0.9.34            | YAML config support       |
| toml               | 1.1.2             | TOML parsing (Cargo.lock) |
| regex              | 1.10              | Pattern matching          |
| tokio              | 1.52.3            | Async runtime             |
| async-trait        | 0.89              | Async trait support       |
| once_cell          | 1.21.4            | Global statics            |
| thiserror          | 1.0.52            | Error derive macros       |
| anyhow             | 1.0.102           | Error context             |
| clap               | 4.6.1             | CLI parsing               |
| reqwest            | 0.13.4 (blocking) | HTTP client               |
| chrono             | 0.4.44            | Date/time                 |
| mcp-sdk-rs         | 0.3.4             | MCP protocol              |
| rand               | 0.10.1            | Random number generation  |
| tracing            | 0.1               | Structured logging        |
| tracing-subscriber | 0.3               | Log filtering             |

---
