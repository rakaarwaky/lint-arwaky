# Lint Arwaky

[![Rust 2021](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![crates.io](https://img.shields.io/crates/v/lint_arwaky.svg)](https://crates.io/crates/lint_arwaky)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![MCP Server](https://img.shields.io/badge/MCP-Server-blue.svg)](https://modelcontextprotocol.io/)
[![Architecture: AES](https://img.shields.io/badge/architecture-AES+Clean-green.svg)](ARCHITECTURE.md)

**Autonomous code quality and architecture enforcement for AI agents and developers — written in Rust.**

Lint Arwaky audits Rust, Python, and JavaScript/TypeScript source code in a single pass. It enforces 27 Agentic Engineering System (AES) rules across 4 groups (Layer & Import, Naming & Structure, File & Content, Role Violations) that check layer boundaries, naming conventions, type safety, dead code, and architectural bypass attempts — all at the code level with zero bypass allowed.

The project is its own first customer: running `lint-arwaky-cli check .` on the repository audits itself under the same AES ruleset.

---

## Table of Contents

- [Overview &amp; Value Proposition](#overview--value-proposition)
- [Install](#install)
- [Usage](#usage)
- [Architecture Overview](#architecture-overview)
- [MCP Server Configuration](#mcp-server-configuration)
- [Supported AES Rules](#supported-aes-rules)
- [CLI Commands Reference](#cli-commands-reference)

---

## Overview & Value Proposition

### What it does

| Feature                      | Description                                                                                                |
| ---------------------------- | ---------------------------------------------------------------------------------------------------------- |
| **Multi-Language**     | Rust (Clippy + AST), Python (Ruff, MyPy, Bandit, Radon), JavaScript/TypeScript (ESLint, Prettier, TSC)     |
| **Architecture Audit** | 27 AES rules enforce clean architecture layer boundaries, naming, type safety, and dead code               |
| **MCP Server**         | 5 tools for autonomous AI-agent integration over JSON-RPC 2.0                                              |
| **Zero Bypass**        | `noqa`, `type: ignore`, and `#[allow(...)]` suppressions are detected and flagged (**AES014**) |
| **CI Ready**           | SARIF 2.1.0, JUnit XML, and JSON reports with proper exit codes                                            |
| **Self-Auditing**      | The project lints itself under its own rule engine                                                         |

### Who it's for

| Persona               | Use Case                                       | Start Here                      |
| --------------------- | ---------------------------------------------- | ------------------------------- |
| **AI Agent**    | Autonomous linting, self-healing, code review  | [SKILL.md](SKILL.md)               |
| **Developer**   | Lint codebases, enforce architecture           | [Quick Start](#usage) below        |
| **DevOps / CI** | Quality gates, trend reports, dependency scans | `ci`, `report`              |
| **Contributor** | Extend adapters, add CLI commands              | [CONTRIBUTING.md](CONTRIBUTING.md) |
| **Reviewer**    | Security audit, complexity analysis            | `security`, `complexity`    |

---

## Install

### Pre-built binaries

```bash
# Linux / macOS
curl -sSL https://raw.githubusercontent.com/rakaarwaky/lint-arwaky/main/install.remote.sh | bash
```

### From source (requires Rust 1.70+)

```bash
git clone https://github.com/rakaarwaky/lint-arwaky.git
cd lint-arwaky
cargo build --release
# Binaries: target/release/lint-arwaky-cli, target/release/lint-arwaky-mcp, target/release/lint-arwaky-tui
```

### Verify installation

```bash
lint-arwaky-cli version        # should print "Lint Arwaky v1.10.11 (AES Semantic Builder)"
lint-arwaky-cli setup doctor   # environment diagnostics
```

---

## Usage

### Lint a codebase

```bash
# Full self-lint: AES architecture rules over crates/
lint-arwaky-cli check .

# Git diff mode: only audit files changed since a base ref
lint-arwaky-cli check . --git-diff

# CI-optimized with exit codes (1 if score < threshold)
lint-arwaky-cli ci . --threshold 80
```

### Per-domain scans

```bash
lint-arwaky-cli security .      # Bandit-style vulnerability scan
lint-arwaky-cli complexity .    # Cyclomatic complexity hotspots
lint-arwaky-cli duplicates .    # 5-line block duplication detection
lint-arwaky-cli trends .        # Quality score over time
lint-arwaky-cli dependencies .  # Cargo.toml dependency listing
```

### Reports

```bash
lint-arwaky-cli report . --output-format json
lint-arwaky-cli export sarif
lint-arwaky-cli export junit
```

### Self-lint (this project audits itself)

```bash
cargo run --bin lint-arwaky-cli -- check .
# Scans crates/ under the same AES rules the project enforces on others.
```

### Lint other repos

```bash
# Scan external projects with all adapters + AES architecture rules
lint-arwaky-cli scan /path/to/some-project/
```

---

## Architecture Overview

Lint Arwaky follows its own AES (Agentic Engineering System) specification — a strict layered architecture with six layers, organized into **26 feature folders** (vertical slicing). See [ARCHITECTURE.md](ARCHITECTURE.md) for the full specification.

### Layer prefix naming

Files use the layer as a **file prefix** (not a directory): `[layer]_[concept]_[suffix].rs`

| Layer (prefix)      | Allowed suffixes                                            |
| ------------------- | ----------------------------------------------------------- |
| `taxonomy_`       | `_vo`, `_entity`, `_event`, `_error`, `_constant` |
| `contract_`       | `_port`, `_protocol`, `_aggregate`                    |
| `capabilities_`   | `_analyzer`, `_checker`, `_processor`, etc.           |
| `infrastructure_` | `_adapter`, `_provider`, `_scanner`, etc.             |
| `agent_`          | `_orchestrator`                                           |
| `surface_`        | `_command`, `_handler`, `_controller`                 |

### Feature folders (vertical slicing)

```
crates/
  cli-commands/      import-rules/         role-rules/
  config-system/     naming-rules/         orphan-detector/
  pipeline-jobs/     file-watch/           lifecycle-state/
  project-setup/     git-hooks/            language-adapters/
  plugin-system/     multi-project/        auto-fix/
  output-report/     mcp-server/           file-system/
  code-analysis/     source-parsing/       metrics-service/
  shared/
```

Import flow: `surface_` → `agent_` → `capabilities_` / `infrastructure_` → `contract_` → `taxonomy_`.

### Adapters

| Adapter                       | What it checks                    | Layer          |
| ----------------------------- | --------------------------------- | -------------- |
| `ast_rust_scanner`          | Rust AST parsing                  | infrastructure |
| `ast_py_scanner`            | Python AST parsing                | infrastructure |
| `ast_js_scanner`            | JavaScript/TypeScript AST parsing | infrastructure |
| `rust_linter_adapter`       | Clippy                            | infrastructure |
| `python_ruff_adapter`       | Ruff                              | infrastructure |
| `python_mypy_adapter`       | MyPy                              | infrastructure |
| `python_bandit_adapter`     | Bandit                            | infrastructure |
| `python_metrics_adapter`    | Radon-style complexity            | infrastructure |
| `javascript_linter_adapter` | ESLint / Prettier / TSC           | infrastructure |

The architecture compliance analyzer (`arch_compliance_analyzer.rs`) carries the highest effective weight — structural violations are the highest priority.

---

## MCP Server Configuration

### Entry point

The MCP server is bootstrapped by `crates/root_mcp_main_entry.rs`:

### MCP tools (5 tools)

| Tool                              | Description                                                       |
| --------------------------------- | ----------------------------------------------------------------- |
| `execute_command(action, args)` | Execute any CLI command with arguments. This is the primary tool. |
| `list_commands(domain)`         | List all available CLI commands with descriptions.                |
| `commands_schema(tool_name)`    | Retrieve the JSON Schema for a registered MCP tool.               |
| `read_skill_context(section)`   | Read SKILL.md documentation by section for AI context.            |
| `health_check()`                | Verify linter adapter health and system state.                    |

### Configure

Add to `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "lint-arwaky": {
      "command": "lint-arwaky-mcp",
      "args": []
    }
  }
}
```

Or print the config from the CLI:

```bash
lint-arwaky-cli setup mcp-config --client claude
lint-arwaky-cli setup mcp-config --client vscode
lint-arwaky-cli setup mcp-config --client hermes
```

---

## CLI Commands Reference

The CLI is implemented in `crates/cli-commands/src/surface_core_command.rs` (with subcommands split across `surface_check_command.rs`, `surface_dev_command.rs`, `surface_setup_command.rs`, etc.). All commands are defined in `crates/shared/src/cli-commands/taxonomy_catalog_constant.rs` (`COMMAND_CATALOG`).

### Core

| Command                                                                   | Description                                          |
| ------------------------------------------------------------------------- | ---------------------------------------------------- |
| `lint-arwaky-cli check [path] [--git-diff]`                             | Run full architecture compliance analysis            |
| `lint-arwaky-cli scan [path]`                                           | Alias for `check` (CI-friendly)                    |
| `lint-arwaky-cli fix [path]`                                            | Apply safe fixes automatically                       |
| `lint-arwaky-cli report [path] --output-format <text\|json\|sarif\|junit>` | Generate quality report                              |
| `lint-arwaky-cli ci [path] --threshold <N>`                             | CI mode with exit codes                              |
| `lint-arwaky-cli git-diff [--base <ref>]`                               | List files changed since base ref (default `HEAD`) |
| `lint-arwaky-cli multi-project <paths...>`                              | Aggregate lint results across projects               |

### Scans

| Command                                 | Description                                  |
| --------------------------------------- | -------------------------------------------- |
| `lint-arwaky-cli security [path]`     | Bandit-style vulnerability scan              |
| `lint-arwaky-cli complexity [path]`   | Cyclomatic complexity hotspots (top 5 files) |
| `lint-arwaky-cli duplicates [path]`   | 5-line block duplication detection           |
| `lint-arwaky-cli trends [path]`       | Quality score trends over time               |
| `lint-arwaky-cli dependencies [path]` | Parse and list `Cargo.toml` dependencies   |

### Setup

| Command                                                                  | Description                                            |
| ------------------------------------------------------------------------ | ------------------------------------------------------ |
| `lint-arwaky-cli setup init`                                           | Create a default `lint_arwaky.config.yaml`           |
| `lint-arwaky-cli setup doctor`                                         | Environment diagnostics (Rust toolchain, binary path)  |
| `lint-arwaky-cli setup mcp-config --client <claude\|vscode\|hermes\|all>` | Print MCP config for AI clients                        |
| `lint-arwaky-cli setup hermes [--remove]`                              | Add/remove the `[mcp.lint-arwaky]` section in Hermes |

### Dev & Maintenance

| Command                                       | Description                                                  |
| --------------------------------------------- | ------------------------------------------------------------ |
| `lint-arwaky-cli diff <path1> <path2>`      | Compare violation counts and scores between two paths        |
| `lint-arwaky-cli import <config_file>`      | Import configuration from JSON/YAML file                     |
| `lint-arwaky-cli export <sarif\|junit\|json>` | Export reports in standard formats                           |
| `lint-arwaky-cli watch [path]`              | Poll the path every 2s and re-run lint                       |
| `lint-arwaky-cli suggest [path] [--ai]`     | Print top suggestions by file                                |
| `lint-arwaky-cli install-hook`              | Install `lint-arwaky-cli check .` as a git pre-commit hook |
| `lint-arwaky-cli uninstall-hook`            | Remove the installed git pre-commit hook                     |
| `lint-arwaky-cli adapters`                  | List active linter adapters                                  |
| `lint-arwaky-cli config show`               | Show active configuration                                    |
| `lint-arwaky-cli cancel <job_id>`           | Request cancellation of a running lint job                   |
| `lint-arwaky-cli version`                   | Show version (`1.10.11`)                                    |

---

## Project Stats (v1.10.11)

| Metric             | Value                                                                                               |
| ------------------ | --------------------------------------------------------------------------------------------------- |
| Language           | Rust 2021 edition                                                                                   |
| Crate              | `lint_arwaky` (library)                                                                           |
| Binaries           | `lint-arwaky-cli`, `lint-arwaky-mcp`, `lint-arwaky-tui`                                       |
| Source files       | 270+ (across 6 layers + 3 entry points +`lib.rs`)                                                 |
| Layers             | 6 (taxonomy, contract, capabilities, infrastructure, agent, surfaces)                               |
| AES rules enforced | 27 (4 groups: Layer & Import, Naming & Structure, File & Content, Role Violations)                  |
| Linter adapters    | 9 (Rust AST + Clippy, Python AST + Ruff + MyPy + Bandit + Metrics, JS/TS AST + ESLint/Prettier/TSC) |
| MCP tools          | 5 (execute_command, list_commands, commands_schema, read_skill_context, health_check)               |
| CLI subcommands    | 20+ across core/scans/setup/dev                                                                     |
| Report formats     | `text`, `json`, `sarif` 2.1.0, `junit` XML                                                  |
| Self-lint target   | `crates/` scanned under the same rules the project enforces                                     |
