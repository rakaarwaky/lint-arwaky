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

| Feature                      | Description                                                                                            |
| ---------------------------- | ------------------------------------------------------------------------------------------------------ |
| **Multi-Language**     | Rust (Clippy + AST), Python (Ruff, MyPy, Bandit, Radon), JavaScript/TypeScript (ESLint, Prettier, TSC) |
| **Architecture Audit** | 24 AES rules enforce clean architecture layer boundaries, naming, type safety, and dead code           |
| **MCP Server**         | 5 tools for autonomous AI-agent integration over JSON-RPC 2.0                                          |
| **Zero Bypass**        | `noqa`, `type: ignore`, and `#[allow(...)]` suppressions are detected and flagged               |
| **CI Ready**           | SARIF 2.1.0, JUnit XML, and JSON reports with proper exit codes                                        |
| **Self-Auditing**      | The project lints itself under its own rule engine                                                     |

### Who it's for

| Persona               | Use Case                                       | Start Here                        |
| --------------------- | ---------------------------------------------- | --------------------------------- |
| **AI Agent**    | Autonomous linting, self-healing, code review  | [SKILL.md](SKILL.md)               |
| **Developer**   | Lint codebases, enforce architecture           | [Quick Start](#usage) below        |
| **DevOps / CI** | Quality gates, trend reports, dependency scans | `ci`, `check`                 |
| **Contributor** | Extend adapters, add CLI commands              | [CONTRIBUTING.md](CONTRIBUTING.md) |
| **Reviewer**    | Architecture audit, code quality analysis      | `check`, `orphan`             |

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
lint-arwaky-cli version        # should print "Lint Arwaky v1.10.74 (AES Semantic Builder)"
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

Lint Arwaky follows its own AES (Agentic Engineering System) specification — a strict layered architecture with seven layers, organized into **feature crates** (vertical slicing). See [ARCHITECTURE.md](ARCHITECTURE.md) for the full specification, layer hierarchy, and naming conventions.

## MCP Server Configuration

See [SKILL.md](SKILL.md) for the MCP tool reference and [DEPLOY.md](DEPLOY.md) for client setup.

## CLI Commands Reference

See [SKILL.md](SKILL.md) for the complete command catalog.

---

## TUI (Interactive File Browser)

`lint-arwaky-tui` — Ranger-style 3-panel file browser (`ratatui` + `crossterm`).
Path project is entered once at startup, then navigate folders and run commands on selected files/folders.

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

| Panel  | Content                                    |
| ------ | ------------------------------------------ |
| Left   | Directory tree                             |
| Center | File list + layer AES badges (color-coded) |
| Right  | File preview / lint results                |

Each file is given a **layer badge** color:
`[taxonomy]` cyan, `[contract]` blue, `[capabilities]` magenta, `[infra]` yellow, `[agent]` green, `[surface]` red, `[root]` white.

### Shortcuts (always visible at bottom bar)

#### Navigation

| Key               | Action                    |
| ----------------- | ------------------------- |
| `j`/`k`       | Move up/down              |
| `h`             | Back (parent dir)         |
| `l` / `Enter` | Open folder / preview     |
| `gg` / `G`    | Jump to start/end         |
| `/`             | Search file               |
| Tab               | Cycle panel focus         |
| Mouse click       | Select item / focus panel |
| Scroll wheel      | Scroll panel              |

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
| `d`  | **doctor** — environment diagnosis       | `doctor`                  |
| `i`  | **init** — create default config         | `init`                    |
| `I`  | **install** — install adapter deps       | `install`                 |
| `m`  | **mcp-config** — print MCP config        | `mcp-config`              |
| `C`  | **config-show** — show active config     | `config-show`             |
| `H`  | **install-hook** — git hook install      | `install-hook`            |
| `U`  | **uninstall-hook** — git hook remove     | `uninstall-hook`          |
| `a`  | **adapters** — list active adapters      | `adapters`                |
| `v`  | **version** — show version               | `version`                 |
| `?`  | Help overlay                                    | —                          |
| `q`  | Quit                                            | —                          |

#### Mouse support

All elements are clickable: file list, action buttons, panel focus. Scroll wheel to scroll.

### Run

```bash
cargo run --bin lint-arwaky-tui
# or directly:
./target/release/lint-arwaky-tui
```
