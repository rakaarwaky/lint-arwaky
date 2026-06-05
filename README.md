# Lint Arwaky

[![Rust 1.80+](https://img.shields.io/badge/rust-1.80+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![MCP Server](https://img.shields.io/badge/MCP-Server-blue.svg)](https://modelcontextprotocol.io/)
[![Architecture: AES](https://img.shields.io/badge/architecture-AES+Clean-green.svg)](lint_arwaky.config.rust.yaml)

**Autonomous code quality and architecture enforcement for AI agents and developers.**

Lint Arwaky runs Python checks (Ruff, MyPy, Bandit, Radon), JavaScript/TypeScript checks (ESLint, Prettier, TSC), and Rust checks (Cargo Clippy) in a single pass, then overlays 31 Agentic Engineering System (AES) rules that check layer boundaries, naming conventions, type safety, and dead code — all enforced at the code level with zero bypass allowed.

---

## Table of Contents

- [Overview & Value Proposition](#overview--value-proposition)
- [Install](#install)
- [Usage](#usage)
- [Architecture Overview](#architecture-overview)
- [MCP Server Configuration](#mcp-server-configuration)
- [Supported AES Rules](#supported-aes-rules)
- [CLI Commands Reference](#cli-commands-reference)

---

## Overview & Value Proposition

### What it does

| Feature | Description |
|---|---|
| **Multi-Linter** | Runs Ruff/MyPy/Bandit/Radon (Python), ESLint/Prettier/TSC (JS/TS), and Cargo Clippy (Rust) |
| **Architecture Audit** | 31 AES rules enforce clean architecture layer boundaries |
| **Auto-Fix** | Safe fixes applied automatically without human intervention |
| **AI Ready** | MCP server with 5 tools for autonomous agent integration |
| **Zero Bypass** | No noqa, no type: ignore, no clippy allow — violations fixed in code |
| **CI Ready** | SARIF, JUnit, JSON reports with exit codes |

### Who it's for

| Persona | Use Case | Start Here |
|---|---|---|
| **AI Agent** | Autonomous linting, self-healing, code review | SKILL.md |
| **Developer** | Lint codebases, enforce architecture | Quick Start below |
| **DevOps / CI** | Quality gates, trend reports, dependency scans | `ci`, `report` |
| **Contributor** | Extend adapters, add CLI commands | Contributing sections |
| **Reviewer** | Security audit, complexity analysis | `security`, `complexity` |

---

## Install

### Cargo (compile from source)

```bash
git clone https://github.com/rakaarwaky/lint-arwaky.git
cd lint-arwaky
cargo build --release

# The compiled binary is available at target/release/lint-arwaky
```

### Installer scripts

```bash
# Linux / macOS
curl -sSL https://raw.githubusercontent.com/rakaarwaky/lint-arwaky/main/install.sh | bash

# Windows PowerShell
Invoke-WebRequest -Uri https://raw.githubusercontent.com/rakaarwaky/lint-arwaky/main/install.ps1 | Invoke-Expression
```

### Verify installation

```bash
lint-arwaky version
lint-arwaky setup doctor
```

---

## Usage

### Lint a codebase

```bash
# Full check: runs Python, JavaScript, and Rust linters + architecture
lint-arwaky check ./src-rust/

# Git diff mode: only lint changed files
lint-arwaky check ./src-rust/ --git-diff

# CI-optimized with exit codes
lint-arwaky ci ./src-rust/
```

### Auto-fix

```bash
# Apply safe fixes automatically
lint-arwaky fix ./src-rust/
```

### Security scan

```bash
lint-arwaky security ./src-rust/
```

### Reports

```bash
lint-arwaky report ./src-rust/ --output-format json
lint-arwaky export sarif -o report.sarif
```

### Self-lint (this project audits itself)

```bash
cd lint-arwaky
lint-arwaky check ./src-rust/
# Result: 100/100 score, 0 violations across all adapters
```

### Lint other repos

```bash
# Point at any multi-language project
lint-arwaky check /path/to/some-project/src/
lint-arwaky security /path/to/some-project/
lint-arwaky report /path/to/some-project/ --output-format sarif
```

---

## Architecture Overview

Lint Arwaky follows its own AES (Agentic Engineering System) specification — a strict layered Clean Architecture with six layers (+ root entry points):

```
┌─────────────────────────────────────────────┐
│  SURFACES  (entry points)                   │  _handler, _command, _controller
│  cli_check_command, mcp_server_handler      │
├─────────────────────────────────────────────┤
│  AGENT     (orchestration & wiring)         │  _orchestrator, _container, _coordinator, _registry, _manager
│  DI, pipeline execution, job management     │
├─────────────────────────────────────────────┤
│  CAPABILITIES (use-case logic)              │  _analyzer, _checker, _processor, _handler, _evaluator, _formatter, _resolver, _validator
│  32 modules: import, naming, security, MCP  │
├─────────────────────────────────────────────┤
│  CONTRACT    (ports/protocols/aggregates)   │  _port, _protocol, _aggregate
│  81 interface definitions                   │
├─────────────────────────────────────────────┤
│  INFRASTRUCTURE (adapters)                  │  _adapter, _provider, _client, _scanner, _wrapper, _lifespan, _schemas, _validator
│  ruff, mypy, bandit, radon wrappers         │
├─────────────────────────────────────────────┤
│  TAXONOMY    (value objects & entities)     │  _vo, _entity, _error, _event, _constant
│  75 domain types                            │
└─────────────────────────────────────────────┘
```

### Layer responsibilities

| Layer | Suffix | Purpose |
|---|---|---|
| `root` | `_entry` | System entry points (CLI server bootstraps) |
| `taxonomy` | `_vo`, `_entity`, `_error`, `_event`, `_constant` | Domain types, value objects, structured errors, compile-time constants |
| `contract` | `_port`, `_protocol`, `_aggregate` | Interface definitions, dependency injection contracts |
| `capabilities` | 8 allowed (`_analyzer`, `_checker`, `_processor`, `_handler`, `_evaluator`, `_formatter`, `_resolver`, `_validator`) | Use-case implementations, the workhorse logic |
| `infrastructure` | 8 allowed (`_adapter`, `_provider`, `_scanner`, `_wrapper`, `_client`, `_lifespan`, `_schemas`, `_validator`) | External tool adapters, linter wrappers |
| `surfaces` | `_handler`, `_command`, `_controller` | CLI and MCP server command handlers |
| `agent` | `_container`, `_orchestrator`, `_coordinator`, `_registry`, `_manager` | DI wiring, pipeline coordination, job management |

### Dependency direction

Dependencies flow inward only:
```
surfaces → agent → capabilities → contract → taxonomy
                              → infrastructure → contract → taxonomy
```

No layer may import from an outer layer. Taxonomy is the bottom-most foundation with zero dependencies on any other layer.

### Linter adapters

| Adapter | Target Language | What it checks | Weight |
|---|---|---|---|
| `ruff` | Python | Style, unused imports, code errors, complexity | 1.0 |
| `mypy` | Python | Type checking, annotation compliance | 1.0 |
| `bandit` | Python | Security vulnerabilities, unsafe patterns | 1.0 |
| `radon` | Python | Cyclomatic/complexity metrics | 1.0 |
| `eslint` | JS/TS | JS/TS linting and styling rules | 1.0 |
| `prettier` | JS/TS/Other | Code formatting compliance | 1.0 |
| `tsc` | TypeScript | TS compilation and type check | 1.0 |
| `clippy` | Rust | Cargo clippy checks and warnings | 1.0 |
| `architecture` | All | AES rules: layer boundaries, naming, type safety, orphans, dead code | 3.0 |

Architecture checker has 3x weight — structural violations are the highest priority.

---

## MCP Server Configuration

### Entry point

The MCP server is bootstrapped by `src-rust/mcp_main_entry.rs`.

The `Cargo.toml` registers two binary entry points and a library:
```toml
[lib]
name = "lint_arwaky"
path = "src-rust/lib.rs"

[[bin]]
name = "lint-arwaky"
path = "src-rust/cli_main_entry.rs"

[[bin]]
name = "lint-arwaky-mcp"
path = "src-rust/mcp_main_entry.rs"
```

### MCP tools (5 tools)

| Tool | Description |
|---|---|
| `execute_command` | Execute any lint-arwaky CLI command with arguments |
| `list_commands` | List all available CLI commands with descriptions |
| `commands_schema` | Retrieve JSON schemas for registered MCP tools |
| `read_skill_context` | Read SKILL.md documentation sections for AI context |
| `health_check` | Verify linter adapter health (ruff, mypy, bandit, radon) |

### Configure in Claude Desktop

```bash
lint-arwaky setup mcp-config --client claude
```

### Configure in VS Code (MCP extension)

```bash
lint-arwaky setup mcp-config --client vscode
```

### Configure in Hermes Agent

```bash
cargo install --path .
lint-arwaky setup hermes
```

### Manual MCP config

```json
{
  "mcpServers": {
    "lint-arwaky": {
      "command": "lint-arwaky"
    }
  }
}
```

### Architecture

The MCP implementation uses a 3-layer structure:

```
Surface (FastMCP server)
  └─ McpServerHandlerSurface: creates FastMCP server
     └─ McpToolsRegistrySurface: registers 4 tool groups
        ├─ mcp_execute_command → execute_command tool
        ├─ mcp_command_handler → list_commands, commands_schema, read_skill_context
        ├─ mcp_health_handler → health_check
        └─ mcp_desktop_client_handler → desktop client operations
```

The `McpServerWrapper` (infrastructure layer) wraps FastMCP with:
- Explicit JSON Schema tool declarations
- Input validation with bounds checking
- Structured error responses (never leaks stack traces)
- Resource support for rule definitions
- Version compatibility negotiation
- Lifespan context manager for startup/shutdown

---

## Supported AES Rules

Lint Arwaky enforces 31 architecture rules across three categories:

### Global rules (applied everywhere)

| Code | Name | Description |
|---|---|---|
| AES001 | import-layer-violation | Cross-layer dependency violations |
| AES003 | naming-convention | File names must follow `word1_word2_word3.rs` pattern (or appropriate language convention) |
| AES004 | file-too-large | Files exceeding 500 lines (SRP violation) |
| AES005 | file-too-short | Files under 10 lines (dead code clutter) |
| AES006 | primitive-usage | Raw types (str, int, bool) used where Value Objects are required in domain contract & taxonomy layers |
| AES009 | mandatory-class-definition | Files must contain a struct/class definition |
| AES014 | bypass-comment-violation | `# noqa`, `// eslint-disable`, or clippy/compiler allows detected — forbidden |
| AES015 | unused-mandatory-import | Required symbols imported/used but never used (bypass fraud) |
| AES016 | dead-inheritance-bypass | Empty implementations/classes inheriting from contracts (compliance fraud) |
| AES024 | agent-any-bypass | `Any` or dynamically typed bypasses bypassing type safety |

### Structural rules (per-layer validation)

| Code | Name | Description |
|---|---|---|
| AES002 | mandatory-import-missing | Layer must import required dependencies (e.g., taxonomy/contract) |
| AES006 | primitive-usage (domain) | Primitive usage in entity/error/event/taxonomy domain types |
| AES007 | contract-barrel-violation | Contract imports must be from the layer's barrel, not internal modules |
| AES008 | contract-suffix-mismatch | Contract file missing `_port`, `_protocol`, or `_aggregate` suffix |
| AES010 | forbidden-suffix | File suffix is explicitly forbidden for this layer |
| AES011 | suffix-mismatch | File suffix does not match allowed patterns for layer |
| AES012 | barrel-completeness | `__init__.py` / `mod.rs` missing export list |
| AES013 | internal-all-forbidden | Public export list detected in non-barrel file |
| AES020 | circular-import-violation | Circular import between modules breaks DI wiring |
| AES033 | constant-integrity-violation | `_constant` file contains forbidden constructs (struct, enum, fn, impl) |

### Cross-layer rules (inter-module relationships)

| Code | Name | Description |
|---|---|---|
| AES017 | orphan-code-detection | File is unreachable from surface entry points (dead code) |
| AES018 | surface-hierarchy-violation | Surface file not imported from its layer barrel |
| AES019 | passive-surface-violation | Passive surface (component/view) contains complex domain logic |
| AES021 | agent-role-violation | Agent file violates behavioral mandate for its role |
| AES022 | surface-role-violation | Complex domain logic or state in passive surface |
| AES023 | surface-dependency-violation | Surface imports from forbidden layers (infrastructure, capabilities) |
| AES025 | mcp-tool-schema-violation | MCP tool missing docstring, untyped params, or invalid JSON Schema |
| AES026 | forbidden-inheritance | Class inherits from forbidden port/protocol/aggregate (use composition) |
| AES027 | mandatory-inheritance | File imports contract but no class inherits from it |
| AES030 | capability-method-not-found | Capability method referenced in dispatch catalog does not exist |
| AES031 | single-capability-bottleneck | All dispatch routes go to a single capability class |
| AES032 | missing-vo-construction | Capability method call missing required Value Object parameter |

### Agent role mandates

| Role | Suffix | Rules |
|---|---|---|
| Container | `_container` | No domain logic, must implement ServiceContainerAggregate, lazy/eager init only |
| Orchestrator | `_orchestrator` | Stateless execution, single execution goal, no Any type |
| Coordinator | `_coordinator` | High-level policy only, coordinates multiple orchestrators |
| Registry | `_registry` | CRUD only, no decision logic, thread/async safe |
| Manager | `_manager` | No domain data storage, owns health transitions, lifecycle tracking only |

### Full rule reference

For the complete rule specification with WHY and FIX sections, see [AES_RULES.md](./AES_RULES.md).

---

## CLI Commands Reference

### Core commands

| Command | Description |
|---|---|
| `lint-arwaky check <path>` | Full audit: Python/JS/Rust linters + AES architecture |
| `lint-arwaky scan <path>` | Alias for check (CI-friendly) |
| `lint-arwaky fix <path>` | Apply safe fixes automatically |
| `lint-arwaky report <path>` | Generate quality report (text/json/sarif/junit) |
| `lint-arwaky ci <path>` | CI-optimized run with exit codes |
| `lint-arwaky self-lint` | Run audit on lint-arwaky itself |

### Scans

| Command | Description |
|---|---|
| `lint-arwaky security <path>` | Bandit (Python) / other security scans |
| `lint-arwaky complexity <path>` | Cyclomatic complexity analysis |
| `lint-arwaky duplicates <path>` | Code duplication detection |
| `lint-arwaky trends <path>` | Quality trends over time |
| `lint-arwaky dependencies <path>` | Dependency vulnerability scan |

### Setup

| Command | Description |
|---|---|
| `lint-arwaky setup init` | Auto-configure for your system |
| `lint-arwaky setup hermes` | Install into Hermes Agent |
| `lint-arwaky setup doctor` | Diagnose configuration issues |
| `lint-arwaky setup mcp-config` | Print MCP config for AI clients |

### Dev & Maintenance

| Command | Description |
|---|---|
| `lint-arwaky diff <path1> <path2>` | Compare lint results between versions |
| `lint-arwaky config show/edit/reset` | Manage configuration |
| `lint-arwaky export sarif/junit/json` | Export reports |
| `lint-arwaky version` | Show version |
| `lint-arwaky install-hook` | Install git pre-commit hook |
| `lint-arwaky uninstall-hook` | Remove git pre-commit hook |

---

## Configuration

Configuration files: `lint_arwaky.config.rust.yaml`, `lint_arwaky.config.python.yaml`, `lint_arwaky.config.javascript.yaml`

Key sections:

| Section | Purpose |
|---|---|
| `thresholds` | Score target (100.0), max complexity (10) |
| `adapters` | Enabled linters with weights |
| `ignored_rules` | Specific rule codes to skip |
| `ignored_paths` | Directories/files to exclude from analysis |
| `architecture.enabled` | Turn AES rules on/off |
| `architecture.layers` | Layer definitions with paths and allowed suffixes |
| `architecture.rules` | Global/internal/external rule specifications |

---

## Project Stats (v1.10.2)

| Metric | Value |
|---|---|
| Rust version | 1.80+ |
| Target languages | JavaScript, Python, Rust |
| Source files | 280+ (.rs) |
| Layers | 6 (+ root) |
| AES rules enforced | 31 |
| Linter adapters | 8 + architecture |
| MCP tools | 5 |
| CLI commands | 24+ |
| Self-lint score | 100/100 |
