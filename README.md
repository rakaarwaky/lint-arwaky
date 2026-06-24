# Lint Arwaky

[![Rust 2021](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![crates.io](https://img.shields.io/crates/v/lint_arwaky.svg)](https://crates.io/crates/lint_arwaky)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![MCP Server](https://img.shields.io/badge/MCP-Server-blue.svg)](https://modelcontextprotocol.io/)
[![Architecture: AES](https://img.shields.io/badge/architecture-AES+Clean-green.svg)](ARCHITECTURE.md)

**Autonomous code quality and architecture enforcement for AI agents and developers — written in Rust.**

Lint Arwaky audits Rust, Python, and JavaScript/TypeScript source code in a single pass. It enforces 24 Agentic Engineering System (AES) rules across 5 groups (Naming, Import, Quality, Role, Orphan) that check layer boundaries, naming conventions, type safety, dead code, and architectural bypass attempts — all at the code level with zero bypass allowed.

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
| **Architecture Audit** | 24 AES rules enforce clean architecture layer boundaries, naming, type safety, and dead code               |
| **MCP Server**         | 5 tools for autonomous AI-agent integration over JSON-RPC 2.0                                              |
| **Zero Bypass**        | `noqa`, `type: ignore`, and `#[allow(...)]` suppressions are detected and flagged (**AES304**) |
| **CI Ready**           | SARIF 2.1.0, JUnit XML, and JSON reports with proper exit codes                                            |
| **Self-Auditing**      | The project lints itself under its own rule engine                                                         |

### Who it's for

| Persona               | Use Case                                       | Start Here                      |
| --------------------- | ---------------------------------------------- | ------------------------------- |
| **AI Agent**    | Autonomous linting, self-healing, code review  | [SKILL.md](SKILL.md)               |
| **Developer**   | Lint codebases, enforce architecture           | [Quick Start](#usage) below        |
| **DevOps / CI** | Quality gates, trend reports, dependency scans | `ci`, `check`               |
| **Contributor** | Extend adapters, add CLI commands              | [CONTRIBUTING.md](CONTRIBUTING.md) |
| **Reviewer**    | Architecture audit, code quality analysis      | `check`, `orphan`           |

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
lint-arwaky-cli version        # should print "Lint Arwaky v1.10.14 (AES Semantic Builder)"
lint-arwaky-cli maintenance doctor   # environment diagnostics
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

### Orphan & fix commands

```bash
lint-arwaky-cli orphan <path>      # Check if file is dead/unreachable code
lint-arwaky-cli fix .              # Apply safe automatic fixes (--dry-run to preview)
```

### Watch mode

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

Lint Arwaky follows its own AES (Agentic Engineering System) specification — a strict layered architecture with six layers, organized into **20 feature crates** (vertical slicing). See [ARCHITECTURE.md](ARCHITECTURE.md) for the full specification.

### Layer prefix naming

Files use the layer as a **file prefix** (not a directory): `[layer]_[concept]_[suffix].rs`

| Layer (prefix)      | Allowed suffixes                                                                                                                               |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `taxonomy_`       | `_vo`, `_entity`, `_event`, `_error`, `_constant`, `_utility`, `_helper`                                                         |
| `contract_`       | `_port`, `_protocol`, `_aggregate`                                                                                                       |
| `capabilities_`   | `_analyzer`, `_checker`, `_processor`, etc.                                                                                              |
| `infrastructure_` | `_adapter`, `_provider`, `_scanner`, etc.                                                                                                |
| `agent_`          | `_orchestrator`                                                                                                                              |
| `surface_`        | `_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_hook`, `_store`, `_action`, `_screen` |

### Feature folders (vertical slicing)

```
crates/
  cli-commands/      import-rules/         role-rules/
  config-system/     naming-rules/         external-lint/
  project-setup/     git-hooks/            orphan-detector/
  file-watch/        auto-fix/
  maintenance/       mcp-server/           file-system/
  code-analysis/     source-parsing/
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

### MCP tools

| Tool                              | Description                                                       |
| --------------------------------- | ----------------------------------------------------------------- |
| `execute_command(action, args)` | Execute any CLI command with arguments. This is the primary tool. |
| `list_commands(domain)`         | List all available CLI commands with descriptions.                |
| `command_schema(tool_name)`     | Retrieve the JSON Schema for a registered MCP tool.               |
| `read_skill(section)`           | Read SKILL.md documentation by section for AI context.            |
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

| Command                                       | Description                               |
| --------------------------------------------- | ----------------------------------------- |
| `lint-arwaky-cli check [path] [--git-diff]` | Run full architecture compliance analysis |
| `lint-arwaky-cli scan [path]`               | Alias for`check` (CI-friendly)          |
| `lint-arwaky-cli fix [path]`                | Apply safe fixes automatically            |
| `lint-arwaky-cli ci [path] --threshold <N>` | CI mode with exit codes                   |
| `lint-arwaky-cli orphan <path>`             | Check if file is dead/unreachable code    |

### Scans

| Command                                 | Description                               |
| --------------------------------------- | ----------------------------------------- |
| `lint-arwaky-cli security [path]`     | Bandit-style vulnerability scan           |
| `lint-arwaky-cli duplicates [path]`   | 5-line block duplication detection        |
| `lint-arwaky-cli dependencies [path]` | Parse and list`Cargo.toml` dependencies |

### Maintenance

| Command                                | Description                              |
| -------------------------------------- | ---------------------------------------- |
| `lint-arwaky-cli maintenance doctor` | Environment diagnostics (Rust toolchain) |

### Setup

| Command                                                                  | Description                                           |
| ------------------------------------------------------------------------ | ----------------------------------------------------- |
| `lint-arwaky-cli setup init`                                           | Create a default`lint_arwaky.config.yaml`           |
| `lint-arwaky-cli setup install [--sudo]`                               | Install linter adapter dependencies                   |
| `lint-arwaky-cli setup mcp-config --client <claude\|vscode\|hermes\|all>` | Print MCP config for AI clients                       |
| `lint-arwaky-cli setup hermes [--remove]`                              | Add/remove the`[mcp.lint-arwaky]` section in Hermes |

### Dev & Info

| Command                                     | Description                                                 |
| ------------------------------------------- | ----------------------------------------------------------- |
| `lint-arwaky-cli git-diff [--base <ref>]` | List files changed since base ref (default`HEAD`)         |
| `lint-arwaky-cli watch [path]`            | Watch path for changes and re-lint (inotify-based)          |
| `lint-arwaky-cli install-hook`            | Install`lint-arwaky-cli check .` as a git pre-commit hook |
| `lint-arwaky-cli uninstall-hook`          | Remove the installed git pre-commit hook                    |
| `lint-arwaky-cli adapters`                | List active linter adapters                                 |
| `lint-arwaky-cli config show`             | Show active configuration                                   |
| `lint-arwaky-cli version`                 | Show version                                                |

---

## TUI (Interactive File Browser)

`lint-arwaky-tui` — Ranger-style 3-panel file browser (`ratatui` + `crossterm`).
Path project dimasukkan sekali di startup, lalu navigasi folder dan jalanin perintah pada file/folder terseleksi.

```
┌─────────────────────────────────────────────────────────────────────┐
│  Path: /home/project/lint-arwaky                      [Ctrl+Q] Quit │
├──────────┬──────────────────┬──────────────────────────────────────┤
│  crates/ │  ► cli-commands/ │  File Preview / Lint Results        │
│  docs/   │    src/          │                                      │
│  shared/ │      ▼ surface_  │  AES203: OK                         │
│  ...     │        check_    │  AES204: OK                         │
│          │        scan_     │  Violations: 0                      │
│          │        tui_      │                                      │
│          │        fix_      │  [c]heck  [s]can  [f]ix  [w]atch    │
│          │      infrastruc… │  [o]rphan  [d]octor  ?:[h]elp       │
│          │    Cargo.toml    │                                      │
│          │  src/            │                                      │
│          │  tests/          │                                      │
├──────────┴──────────────────┴──────────────────────────────────────┤
│  c:check  s:scan  f:fix  t:ci  w:watch  o:orphan  d:doctor  i:init│
│  I:install  m:mcp  C:config  H:hook  U:unhook  a:adapter  v:version│
│  Status: Ready  |  Selected: crates/cli-commands/src/  |  0 viol.  │
└─────────────────────────────────────────────────────────────────────┘
```

### Panels

| Panel  | Isi                                        |
| ------ | ------------------------------------------ |
| Kiri   | Directory tree                             |
| Tengah | File list + layer AES badges (warna-coded) |
| Kanan  | File preview / hasil lint                  |

Setiap file dikasih **layer badge** warna:
`[taxonomy]` cyan, `[contract]` blue, `[capabilities]` magenta, `[infra]` yellow, `[agent]` green, `[surface]` red, `[root]` white.

### Shortcuts (always visible at bottom bar)

#### Navigation

| Key               | Action                   |
| ----------------- | ------------------------ |
| `j`/`k`       | Gerak atas/bawah         |
| `h`             | Back (parent dir)        |
| `l` / `Enter` | Buka folder / preview    |
| `gg` / `G`    | Lompat awal/akhir        |
| `/`             | Cari file                |
| Tab               | Cycle panel focus        |
| Mouse click       | Pilih item / fokus panel |
| Scroll wheel      | Scroll panel             |

#### Actions (on selected file/folder)

| Key    | Action                                          | CLI Equivalent              |
| ------ | ----------------------------------------------- | --------------------------- |
| `c`  | **check** — full AES compliance          | `check [path]`            |
| `s`  | **scan** — multi-adapter scan            | `scan [path]`             |
| `f`  | **fix** — auto-fix (toggle dry-run)      | `fix [path]`              |
| `t`  | **ci** — CI mode (input threshold)       | `ci [path] --threshold N` |
| `w`  | **watch** — real-time file watch         | `watch [path]`            |
| `o`  | **orphan** — dead code check             | `orphan [path]`           |
| `^S` | **security** — vulnerability scan        | `security [path]`         |
| `^D` | **duplicates** — duplication detection   | `duplicates [path]`       |
| `^P` | **dependencies** — library vulnerability | `dependencies [path]`     |
| `d`  | **doctor** — environment diagnosis       | `maintenance doctor`      |
| `i`  | **init** — create default config         | `setup init`              |
| `I`  | **install** — install adapter deps       | `setup install`           |
| `m`  | **mcp-config** — print MCP config        | `setup mcp-config`        |
| `C`  | **config-show** — show active config     | `config show`             |
| `H`  | **install-hook** — git hook install      | `install-hook`            |
| `U`  | **uninstall-hook** — git hook remove     | `uninstall-hook`          |
| `a`  | **adapters** — list active adapters      | `adapters`                |
| `v`  | **version** — show version               | `version`                 |
| `?`  | Help overlay                                    | —                          |
| `q`  | Quit                                            | —                          |

#### Mouse support

Semua elemen bisa diklik: file list, action buttons, panel focus. Scroll wheel untuk scroll.

### Run

```bash
cargo run --bin lint-arwaky-tui
# atau langsung:
./target/release/lint-arwaky-tui
```

---

## Project Stats (v1.10.29)

| Metric             | Value                                                                                               |
| ------------------ | --------------------------------------------------------------------------------------------------- |
| Language           | Rust 2021 edition                                                                                   |
| Crate              | `lint_arwaky` (library)                                                                           |
| Binaries           | `lint-arwaky-cli`, `lint-arwaky-mcp`, `lint-arwaky-tui`                                       |
| Source files       | 270+ (across 6 layers + 3 entry points +`lib.rs`)                                                 |
| Layers             | 7 (taxonomy, contract, capabilities, infrastructure, agent, surfaces, root)                         |
| AES rules enforced | 24 (5 groups: Naming, Import, Quality, Role, Orphan)                                                |
| Linter adapters    | 9 (Rust AST + Clippy, Python AST + Ruff + MyPy + Bandit + Metrics, JS/TS AST + ESLint/Prettier/TSC) |
| MCP tools          | 5 (execute_command, list_commands, commands_schema, read_skill_context, health_check)               |
| CLI subcommands    | 15+ across core/scans/maintenance/setup/dev                                                         |
| Report formats     | `text`, `json`, `sarif` 2.1.0, `junit` XML                                                  |
| Self-lint target   | `crates/` scanned under the same rules the project enforces                                       |
