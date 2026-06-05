# Lint Arwaky

[![Python 3.12+](https://img.shields.io/badge/python-3.12+-blue.svg)](https://www.python.org/downloads/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![MCP Server](https://img.shields.io/badge/MCP-Server-blue.svg)](https://modelcontextprotocol.io/)
[![Architecture: AES](https://img.shields.io/badge/architecture-AES+Clean-green.svg)](lint-arwaky.config.python.yaml)

**Autonomous code quality and architecture enforcement for AI agents and developers.**

Lint Arwaky runs Ruff, MyPy, Bandit, and Radon in a single pass, then overlays 25+ Agentic Engineering System (AES) rules that check layer boundaries, naming conventions, type safety, and dead code — all enforced at the code level with zero bypass allowed.

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
| **Multi-Linter** | Ruff + MyPy + Bandit + Radon + pip-audit in one command |
| **Architecture Audit** | 28 AES rules enforce clean architecture layer boundaries |
| **Auto-Fix** | Safe fixes applied automatically without human intervention |
| **AI Ready** | MCP server with 5 tools for autonomous agent integration |
| **Zero Bypass** | No noqa, no type: ignore — violations are fixed in code |
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

### pip

```bash
pip install lint-arwaky
```

### uv (recommended)

```bash
# Install as a globally available tool
uv tool install lint-arwaky

# Or zero-install via uvx
uvx auto-lint check ./src/
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
auto-lint version
auto-lint setup doctor
```

---

## Usage

### Lint a codebase

```bash
# Full check: ruff + mypy + bandit + radon + architecture
auto-lint check ./src/

# Git diff mode: only lint changed files
auto-lint check ./src/ --git-diff

# CI-optimized with exit codes
auto-lint ci ./src/
```

### Auto-fix

```bash
# Apply safe fixes automatically
auto-lint fix ./src/
```

### Security scan

```bash
auto-lint security ./src/
```

### Reports

```bash
auto-lint report ./src/ --output-format json
auto-lint export sarif -o report.sarif
```

### Self-lint (this project audits itself)

```bash
cd lint-arwaky
auto-lint check ./src/
# Result: 100/100 score, 0 violations across all adapters
```

### Lint other repos

```bash
# Point at any Python project
auto-lint check /path/to/some-project/src/
auto-lint security /path/to/some-project/
auto-lint report /path/to/some-project/ --output-format sarif
```

---

## Architecture Overview

Lint Arwaky follows its own AES (Agentic Engineering System) specification — a strict layered Clean Architecture with six layers:

```
┌─────────────────────────────────────────────┐
│  SURFACES  (entry points)                   │  _handler, _command, _controller
│  cli_main, mcp_server                       │
├─────────────────────────────────────────────┤
│  AGENT     (orchestration & wiring)         │  _orchestrator, _container, _coordinator, _registry
│  DI, pipeline execution, job management     │
├─────────────────────────────────────────────┤
│  CAPABILITIES (use-case logic)              │  _analyzer, _checker, _processor, _handler
│  32 modules: import, naming, security, MCP  │
├─────────────────────────────────────────────┤
│  CONTRACT    (ports/protocols/aggregates)   │  _port, _protocol, _aggregate
│  130+ interface definitions                │
├─────────────────────────────────────────────┤
│  INFRASTRUCTURE (adapters)                  │  _adapter, _provider, _client, _scanner
│  ruff, mypy, bandit, radon wrappers         │
├─────────────────────────────────────────────┤
│  TAXONOMY    (value objects & entities)     │  _vo, _entity, _error, _event
│  44 domain types                            │
└─────────────────────────────────────────────┘
```

### Layer responsibilities

| Layer | Suffix | Purpose |
|---|---|---|
| `root` | `_entry` | System entry points (CLI server bootstraps) |
| `taxonomy` | `_vo`, `_entity`, `_error`, `_event` | Domain types, value objects, structured errors |
| `contract` | `_port`, `_protocol`, `_aggregate` | Interface definitions, dependency injection contracts |
| `capabilities` | 27 allowed (analyzer, checker, etc.) | Use-case implementations, the workhorse logic |
| `infrastructure` | 34 allowed (adapter, provider, etc.) | External tool adapters, linter wrappers |
| `surfaces` | `_handler`, `_command`, `_controller` | CLI and MCP server command handlers |
| `agent` | `_container`, `_orchestrator`, etc. | DI wiring, pipeline coordination, job management |

### Dependency direction

Dependencies flow inward only:
```
surfaces → agent → capabilities → contract → taxonomy
                              → infrastructure → contract → taxonomy
```

No layer may import from an outer layer. Taxonomy is the bottom-most foundation with zero dependencies on any other layer.

### Linter adapters

| Adapter | What it checks | Weight |
|---|---|---|
| `ruff` | Style, unused imports, code errors, complexity | 1.0 |
| `mypy` | Type checking, annotation compliance | 1.0 |
| `bandit` | Security vulnerabilities, unsafe patterns | 1.0 |
| `radon` | Cyclomatic/complexity metrics | 1.0 |
| `architecture` | AES rules: layer boundaries, naming, type safety, orphans, dead code | 3.0 |

Architecture checker has 3x weight — structural violations are the highest priority.

---

## MCP Server Configuration

### Entry point

The MCP server is bootstrapped by `src/mcp_main_entry.py`:

```python
# 1. Get the DI container
container = get_container()

# 2. Create the MCP surface
surface = McpServerHandlerSurface()

# 3. Run the FastMCP server
surface.run_server(container)
```

The `pyproject.toml` registers the entry point as:
```toml
[project.scripts]
lint-arwaky = "lint-arwaky:mcp_main"
```

### MCP tools (5 tools)

| Tool | Description |
|---|---|
| `execute_command` | Execute any auto-lint CLI command with arguments |
| `list_commands` | List all available CLI commands with descriptions |
| `read_skill_context` | Read SKILL.md documentation sections for AI context |
| `check_status` | Check status of background lint jobs |
| `health_check` | Verify linter adapter health (ruff, mypy, bandit, radon) |

### Configure in Claude Desktop

```bash
auto-lint setup mcp-config --client claude
```

### Configure in VS Code (MCP extension)

```bash
auto-lint setup mcp-config --client vscode
```

### Configure in Hermes Agent

```bash
pip install lint-arwaky
auto-lint setup hermes
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
     └─ McpToolsRegistrySurface: registers 5 tool groups
        ├─ mcp_execute_command → execute_command tool
        ├─ mcp_command_handler → list_commands, read_skill_context
        ├─ mcp_job_commands_handler → check_status, cancel_job
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

Lint Arwaky enforces 28 architecture rules across three categories:

### Global rules (applied everywhere)

| Code | Name | Description |
|---|---|---|
| AES001 | import-layer-violation | Cross-layer dependency violations |
| AES003 | naming-convention | File names must follow `word1_word2_word3.py` pattern |
| AES004 | file-too-large | Files exceeding 500 lines (SRP violation) |
| AES005 | file-too-short | Files under 10 lines (dead code clutter) |
| AES006 | primitive-usage | Raw types (str, int, bool) used where Value Objects are required in domain contract & taxonomy layers |
| AES009 | mandatory-class-definition | Files must contain a class definition |
| AES014 | bypass-comment-violation | `noqa` or `type: ignore` comments detected — forbidden |
| AES015 | unused-mandatory-import | Required symbols imported but never used (bypass fraud) |
| AES016 | dead-inheritance-bypass | Empty classes inheriting from contracts (compliance fraud) |
| AES024 | agent-any-bypass | `Any` type annotation bypassing type safety |

### Internal rules (per-layer structure)

| Code | Layer | Description |
|---|---|---|
| AES006 | entity/error/event/taxonomy | Primitive usage in domain types |
| AES008 | contract | Missing _port, _protocol, or _aggregate suffix |
| AES011 | all | File suffix mismatch for layer |
| AES012 | all non-root | Missing __all__ exports in __init__.py |
| AES013 | all non-root | __all__ in non-barrel files |
| AES017 | taxonomy/agent | Orphan code unreachable from entry points |
| AES018 | surfaces | Smart surface hierarchy violations |
| AES019 | surfaces | Passive surface (component/view) depends on logic |
| AES021 | agent | Agent role violations: wrong behavior for container/orchestrator/coordinator/registry/manager |
| AES022 | surfaces | Complex domain logic in passive surface layer |

### External rules (cross-layer relationships)

| Code | Scope | Description |
|---|---|---|
| AES001 | all layers | Unauthorized imports between layers |
| AES017 | taxonomy, agent | Orphan code — no consumers |
| AES018 | surfaces | Utility surface depends on smart surface |
| AES019 | surfaces | Passive surface depends on non-taxonomy |
| AES021 | agent | Governance component unreachable from surfaces |
| AES022 | surfaces | Domain logic detected in surface layer |
| AES024 | agent | Any type in orchestrator DI |

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
| `auto-lint check <path>` | Full audit: ruff + mypy + bandit + radon + AES architecture |
| `auto-lint scan <path>` | Alias for check (CI-friendly) |
| `auto-lint fix <path>` | Apply safe fixes automatically |
| `auto-lint report <path>` | Generate quality report (text/json/sarif/junit) |
| `auto-lint ci <path>` | CI-optimized run with exit codes |
| `auto-lint self-lint` | Run audit on lint-arwaky itself |

### Scans

| Command | Description |
|---|---|
| `auto-lint security <path>` | Bandit security vulnerability scan |
| `auto-lint complexity <path>` | Cyclomatic complexity analysis (radon) |
| `auto-lint duplicates <path>` | Code duplication detection |
| `auto-lint trends <path>` | Quality trends over time |
| `auto-lint dependencies <path>` | pip-audit dependency vulnerability scan |

### Setup

| Command | Description |
|---|---|
| `auto-lint setup init` | Auto-configure for your system |
| `auto-lint setup hermes` | Install into Hermes Agent |
| `auto-lint setup doctor` | Diagnose configuration issues |
| `auto-lint setup mcp-config` | Print MCP config for AI clients |

### Dev & Maintenance

| Command | Description |
|---|---|
| `auto-lint diff <path1> <path2>` | Compare lint results between versions |
| `auto-lint config show/edit/reset` | Manage configuration |
| `auto-lint export sarif/junit/json` | Export reports |
| `auto-lint version` | Show version |
| `auto-lint install-hook` | Install git pre-commit hook |
| `auto-lint uninstall-hook` | Remove git pre-commit hook |

---

## Configuration

Configuration file: `lint-arwaky.config.python.yaml`

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

## Project Stats (v1.9.3)

| Metric | Value |
|---|---|
| Python version | 3.12+ |
| Source files | 270+ |
| Layers | 6 (+ root) |
| AES rules enforced | 28 |
| Linter adapters | 4 + architecture |
| MCP tools | 5 |
| CLI commands | 30+ |
| Self-lint score | 100/100 |
