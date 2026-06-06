# Product Requirements Document (PRD)

## Lint Arwaky v1.10.2

---

## 1. Product Overview

**Name**: Lint Arwaky
**Type**: CLI tool + MCP server
**Version**: 1.10.2
**License**: MIT
**Language**: Rust (2021 edition)

Lint Arwaky is an autonomous multi-language linting, type-checking, and architectural rule auditing tool. It runs as both a CLI binary (`lint-arwaky-cli`) and an MCP server (`lint-arwaky-mcp`) that exposes 5 tools over JSON-RPC 2.0.

The project audits itself: `lint-arwaky-cli check .` runs the same AES rule engine against `src-rust/` that it runs against third-party code.

---

## 2. Problem Statement

Software projects accumulate quality debt silently. Developers lack:

- A single tool that audits Rust + Python + JavaScript/TypeScript together
- Architectural enforcement that prevents cross-layer violations in multi-domain codebases
- A unified interface for both human developers (CLI) and AI agents (MCP tools)
- A self-auditing tool whose own codebase passes the rules it enforces
- Static analysis with zero bypass tolerance (`noqa`, `type: ignore`, `#[allow(...)]` are flagged)

Lint Arwaky addresses all five.

---

## 3. AI Agent Value

Lint Arwaky is designed to integrate with AI coding agents through its MCP interface, providing:

| Value Driver | Description |
| --- | --- |
| **Agent Autonomy** | Agents operate without human oversight via 5 MCP tools |
| **Multi-Agent Sync** | Jobs are tracked in a thread-safe registry accessible across agent instances |
| **Self-Healing** | The `fix` command applies safe auto-fixes; the `suggest` command guides manual fixes |
| **24/7 Quality** | The `watch` command polls and re-lints continuously during development |

---

## 4. Target Users

| User | Interface | Use Case |
| --- | --- | --- |
| **AI Agents** | MCP tools (5) | Automated code review, pre-commit checks, CI integration |
| **Prototype Developers** | MCP + CLI | Fast iterations, AI-assisted coding, quality gates |
| **Architecture Engineers** | Architecture tools | AES rule enforcement, clean code, DDD |
| **Developers** | CLI (20+ commands) | Local development, watch mode, git hooks |
| **CI/CD Pipelines** | CLI + exit codes | Quality gates, SARIF/JUnit/JSON reports |
| **Contributors** | GitHub + PRs | New adapters, CLI commands, MCP tools |

---

## 5. Functional Requirements

### 5.1 Core Linting Engine

| ID | Requirement | Status |
| --- | --- | --- |
| FR-001 | Run Clippy linting on Rust files | Done |
| FR-002 | Run MyPy type checking on Python files | Done |
| FR-003 | Run Bandit security scanning on Python files | Done |
| FR-004 | Run ESLint on JavaScript/TypeScript files | Done |
| FR-005 | Run Prettier formatting on JS/TS files | Done |
| FR-006 | Run TSC type checking on TypeScript files | Done |
| FR-007 | Run Radon-style complexity analysis on Python files | Done |
| FR-008 | Run pip-audit dependency vulnerability scan | Done |
| FR-009 | Detect oversized files (configurable threshold) | Done |
| FR-010 | Track quality trends over time | Done |
| FR-011 | Apply safe auto-fixes (Rust + Python + JS/TS) | Done |
| FR-012 | Architectural rules (AES layer rules, 31 codes: AES001–AES033 with AES028/029 reserved) | Done |
| FR-013 | AST scanning for Rust, Python, JavaScript/TypeScript | Done |

### 5.2 Report Formats

| ID | Format | Status |
| --- | --- | --- |
| FR-020 | Text (human-readable) | Done |
| FR-021 | JSON (machine-readable) | Done |
| FR-022 | SARIF 2.1.0 (GitHub Code Scanning) | Done |
| FR-023 | JUnit XML (Jenkins/CI) | Done |

### 5.3 Integration

| ID | Requirement | Status |
| --- | --- | --- |
| FR-030 | MCP server via JSON-RPC 2.0 (`mcp-sdk-rs` 0.3.4) | Done |
| FR-031 | CLI via `clap` 4.6 with subcommand groups | Done |
| FR-032 | Direct command execution via `std::process::Command` | Done |
| FR-033 | Git pre-commit hook install/uninstall | Done |
| FR-034 | File watcher (poll-based) for auto-lint on save | Done |
| FR-035 | Self-lint target (`lint-arwaky-cli check .`) | Done |
| FR-036 | Setup: `init`, `doctor`, `mcp-config`, `hermes` | Done |
| FR-037 | Modern CI/CD (OIDC, SLSA Provenance) — inherited from prior v1.9.x | Done |

### 5.4 Semantic Analysis (Enrichment)

| ID | Requirement | Status |
| --- | --- | --- |
| FR-040 | Show enclosing scope (function/class) for violations | Done |
| FR-041 | Trace call chains across project | Done |
| FR-042 | Track variable flow within scope | Done |
| FR-043 | Project-wide symbol rename | Done |
| FR-044 | Generate naming variants (snake_case, camelCase, etc.) | Done |

---

## 6. Non-Functional Requirements

| ID | Requirement | Target | Current |
| --- | --- | --- | --- |
| NFR-003 | Startup time (MCP server) | < 2 s | ~1 s |
| NFR-004 | Single-file scan time | < 5 s | ~2 s |
| NFR-005 | Full project scan | < 30 s | ~10 s |
| NFR-006 | Rust toolchain | >= 1.70 (edition 2021) | 1.70+ |
| NFR-007 | Binary size (release, stripped) | < 30 MB | ~25 MB |

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
surfaces       -> taxonomy, contract
capabilities   -> taxonomy, contract
infrastructure -> taxonomy, contract
contract       -> taxonomy
taxonomy       -> taxonomy
```

Surfaces may NOT import from `agent`, `capabilities`, or `infrastructure` directly — they interact with `agent` only through the `ServiceContainerAggregate` trait (AES023, AES022).

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

### 7.4 Agentic Engineering System (AES) v1.10.2

Severity levels and their point penalty per finding:

| Severity | Penalty | Description |
| --- | --- | --- |
| LOW | -1 | Minor style or naming issue |
| MEDIUM | -2 | Structural concern, barrel/import patterns |
| HIGH | -3 | Architecture violation, mandatory requirement |
| CRITICAL | -5 | Bypass markers, dead inheritance, layer fraud |

Total score starts at 100.0 and is deducted per finding. If any CRITICAL finding exists, the run fails regardless of score.

**AES006 Primitive Policy**: Value Object enforcement is **granular per layer**:

- `contract` and `taxonomy(entity|error|event)` → `no_primitives: true` (strict)
- `infrastructure`, `capabilities`, `surfaces` → `no_primitives: false` (adapter layers may use primitives as supporting types)
- `taxonomy(constant)` → raw primitives allowed by definition; must contain ONLY constant declarations (AES033)

**AES033 Constant Purity (v1.10.2)**: Taxonomy files ending in `_constant` must contain only `pub const` / `pub static` declarations. Any `struct`, `enum`, `fn`, or `impl` block in a `_constant` file is a violation.

See [docs/AES_RULES.md](docs/AES_RULES.md) for the full rule catalog (31 codes: AES001–AES033, AES028/029 reserved) and [docs/AESArchitecture.md](docs/AESArchitecture.md) for the layered specification with Mermaid diagrams.

---

## 8. MCP Interface (5 Tools)

| Tool | Purpose |
| --- | --- |
| `execute_command(action, args)` | Execute any CLI command |
| `list_commands(domain)` | Discover available CLI commands |
| `commands_schema(tool_name)` | Retrieve the JSON Schema for a registered tool |
| `read_skill_context(section)` | Read SKILL.md documentation by section |
| `health_check()` | Check linter adapter health and system state |

> **Note**: Job cancellation is exposed as the CLI subcommand `lint-arwaky-cli cancel <job_id>` rather than an MCP tool.

---

## 9. CLI Interface

Subcommands are defined in `src-rust/surfaces/cli_core_command.rs` and dispatched from `cli_main_entry.rs`.

| Category | Subcommands |
| --- | --- |
| Core | check, scan, fix, report, ci, version, adapters, security, cancel |
| Scans | complexity, duplicates, trends, dependencies |
| Setup | setup init, setup doctor, setup mcp-config, setup hermes |
| Dev | diff, suggest, import, export, config, install-hook, uninstall-hook, watch |
| Git | git-diff, multi-project |

---

## 10. Constraints

- Pure-Rust implementation (no embedded Python or Node.js runtime)
- No database required (file-based history only for trends)
- Static binary release via `cargo build --release`
- Platform: Linux, macOS, Windows (MSVC and GNU)

---

## 11. Dependencies (Cargo.toml)

| Crate | Version | Purpose |
| --- | --- | --- |
| serde | 1.0 | Serialization framework |
| serde_json | 1.0 | JSON support |
| serde_yaml | 0.9.34 | YAML config support |
| toml | 1.1.2 | TOML parsing (Cargo.lock) |
| regex | 1.10 | Pattern matching |
| tokio | 1.52.3 | Async runtime |
| async-trait | 0.89 | Async trait support |
| once_cell | 1.21.4 | Global statics |
| thiserror | 1.0.52 | Error derive macros |
| anyhow | 1.0.102 | Error context |
| clap | 4.6.1 | CLI parsing |
| reqwest | 0.13.4 (blocking) | HTTP client |
| chrono | 0.4.44 | Date/time |
| mcp-sdk-rs | 0.3.4 | MCP protocol |
| rand | 0.10.1 | Random number generation |
| tracing | 0.1 | Structured logging |
| tracing-subscriber | 0.3 | Log filtering |
