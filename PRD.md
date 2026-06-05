# Product Requirements Document (PRD)

## Lint Arwaky MCP Server v1.10.2

---

## 1. Product Overview

**Name**: Lint Arwaky
**Type**: MCP Server + CLI Tool
**Version**: 1.10.2
**License**: MIT
**Language**: Rust 1.80+ (2024 edition)

Lint Arwaky is an autonomous multi-language linting, type-checking, and
architectural rule auditing tool written in Rust. It runs as both an MCP server and CLI tool.

Uses `mcp-sdk-rs` for the MCP server interface.
Connects to system for secure command execution.

---

## 2. Problem Statement

Software projects accumulate quality debt silently. Developers lack:

- Automated pre-commit quality gates that run without configuration
- Architectural enforcement that prevents cross-layer violations
- Unified interface across multiple linters (Ruff, MyPy, Bandit, ESLint, Clippy...)
- Both human-accessible CLI and AI-agent-accessible MCP tools from one codebase
- Easy setup for community/open-source distribution via Cargo or pre-compiled binaries

---

## 3. AI Agent Value

Lint Arwaky is designed to integrate with AI coding agents through its MCP interface, providing:

| Value Driver               | Description                            |
| -------------------------- | -------------------------------------- |
| **Agent Autonomy**   | Agent operates without human oversight |
| **Multi-Agent Sync** | 2+ agents share job registry           |
| **Self-Healing**     | Agent auto-fixes detected issues       |
| **24/7 Quality**     | Agent enforces rules continuously      |

---

## 4. Target Users

| User                             | Interface             | Use Case                                                 |
| -------------------------------- | --------------------- | -------------------------------------------------------- |
| **AI Agents**              | MCP tools (5 tools)   | Automated code review, pre-commit checks, CI integration |
| **Prototype Developers**   | MCP + CLI             | Fast iterations, AI-assisted coding, quality gates       |
| **Architecture Engineers** | Architecture tools    | Architectural rule enforcement, clean code, DDD          |
| Developers                       | CLI (24+ commands)    | Local development, watch mode, git hooks                 |
| CI/CD Pipelines                  | CLI + exit codes      | Quality gates, SARIF/JUnit reports                       |
| Community                        | cargo install + setup | Easy install, works immediately                          |
| Contributors                     | GitHub + PRs          | Adapters, CLI commands, MCP tools                        |

---

## 5. Functional Requirements

### 5.1 Core Linting Engine

| ID     | Requirement                                    | Status |
| ------ | ---------------------------------------------- | ------ |
| FR-001 | Run Ruff linting on Python files               | Done   |
| FR-002 | Run MyPy type checking on Python files         | Done   |
| FR-003 | Run Bandit security scanning on Python files   | Done   |
| FR-004 | Run ESLint on JavaScript/TypeScript files      | Done   |
| FR-005 | Run Prettier formatting on JS/TS/YAML files    | Done   |
| FR-006 | Run TSC type checking on TypeScript files      | Done   |
| FR-007 | Run Radon complexity analysis on Python files  | Done   |
| FR-008 | Run Cargo Clippy static analysis on Rust files | Done   |
| FR-009 | Detect oversized files (>500 lines)            | Done   |
| FR-010 | Track quality trends over time                 | Done   |
| FR-011 | Apply safe auto-fixes (Ruff, ESLint, Prettier) | Done   |
| FR-012 | Architectural rules (AES layer rules)          | Done   |

### 5.2 Report Formats

| ID     | Format                             | Status |
| ------ | ---------------------------------- | ------ |
| FR-020 | Text (human-readable)              | Done   |
| FR-021 | JSON (machine-readable)            | Done   |
| FR-022 | SARIF 2.1.0 (GitHub Code Scanning) | Done   |
| FR-023 | JUnit XML (Jenkins/CI)             | Done   |

### 5.3 Integration

| ID     | Requirement                                | Status |
| ------ | ------------------------------------------ | ------ |
| FR-030 | MCP server via `mcp-sdk-rs`              | Done   |
| FR-031 | CLI via clap (Rust)                        | Done   |
| FR-032 | Direct command execution                   | Done   |
| FR-033 | Git pre-commit hook install/uninstall      | Done   |
| FR-034 | File watcher for auto-lint on save         | Done   |
| FR-035 | Automatic local execution                  | Done   |
| FR-036 | Community setup (setup init/hermes/doctor) | Done   |
| FR-037 | `cargo install` support                  | Done   |
| FR-038 | installer script                           | Done   |
| FR-039 | Modern CI/CD                               | Done   |

### 5.4 Semantic Analysis (Enrichment)

| ID     | Requirement                                            | Status |
| ------ | ------------------------------------------------------ | ------ |
| FR-040 | Show enclosing scope (function/class) for violations   | Done   |
| FR-041 | Trace call chains across project                       | Done   |
| FR-042 | Track variable flow within scope                       | Done   |
| FR-043 | Project-wide symbol rename                             | Done   |
| FR-044 | Generate naming variants (snake_case, camelCase, etc.) | Done   |

---

## 6. Non-Functional Requirements

| ID      | Requirement               | Target  | Current |
| ------- | ------------------------- | ------- | ------- |
| NFR-003 | Startup time (MCP server) | < 0.5s  | ~0.1s   |
| NFR-004 | Single-file scan time     | < 5s    | ~1.5s   |
| NFR-005 | Full project scan         | < 30s   | ~8s     |
| NFR-006 | Rust version              | >= 1.80 | 1.80+   |
| NFR-007 | Max directory depth       | <= 10   | 10      |

---

## 7. Architecture

### 7.1 Domain Model (6 Domains)

```
src-rust/
  agent/           -- Lifecycle, orchestration, pipeline, DI container
  capabilities/    -- Thinking logic: analysis, formatting, architecture
  contract/        -- Interfaces, traits, protocols
  infrastructure/  -- Adapters: ruff, mypy, eslint, clippy, transports
  surfaces/        -- Interfaces: CLI (clap), MCP (mcp-sdk-rs)
  taxonomy/        -- Shared types: value objects, models, errors
```

### 7.2 Dependency Rules

```
agent          -> taxonomy, contract, infrastructure, capabilities  
surfaces       -> taxonomy, contract (agent via contract traits only)
capabilities   -> taxonomy, contract   
infrastructure -> taxonomy, contract  
contract       -> taxonomy, contract            
taxonomy       -> taxonomy                                    
```

### 7.3 Agentic Engineering System (AES) v1.9.4

Severity levels and their point penalty per finding:

| Severity | Penalty | Description                                   |
| -------- | ------- | --------------------------------------------- |
| LOW      | -1      | Minor style or naming issue                   |
| MEDIUM   | -2      | Structural concern, barrel/import patterns    |
| HIGH     | -3      | Architecture violation, mandatory requirement |
| CRITICAL | -5      | Bypass markers, dead inheritance, layer fraud |

Total score starts at 100.0 and is deducted per finding. If any CRITICAL finding exists, compliance fails regardless of score.

**AES006 Primitive Policy Default**

- `contract` and `taxonomy(entity|error|event)` → `no_primitives: true` 
- `infrastructure`, `capabilities`, `surfaces` → `no_primitives: false`

See separate [AES Rules Document](docs/AES_RULES.md) for full rule definitions, codes, and violation messages.

---

## 8. MCP Interface (5 Tools)

| Tool                              | Purpose                          |
| --------------------------------- | -------------------------------- |
| `execute_command(action, args)` | Execute any CLI command          |
| `list_commands(domain)`         | Discover available CLI commands  |
| `commands_schema(tool_name)`    | Retrieve JSON schemas for tools  |
| `read_skill_context(section)`   | Read SKILL.md documentation      |
| `health_check()`                | Check adapters and system health |

---

## 9. CLI Interface (24 Commands)

| Category    | Commands                                                                          |
| ----------- | --------------------------------------------------------------------------------- |
| Core        | check, scan, fix, report, ci, version, adapters, security, cancel                 |
| Analysis    | complexity, duplicates, trends, dependencies, batch                               |
| Dev         | diff, suggest, ignore, config, export, import, init, install-hook, uninstall-hook |
| Setup       | setup init, setup hermes, setup doctor, setup mcp-config                          |
| Maintenance | stats, clean, update, doctor                                                      |
| Other       | watch, plugins, multi-project                                                     |

---

---

## 11. Dependencies

| Package    | Type | Purpose                   |
| ---------- | ---- | ------------------------- |
| serde      | Core | Serialization             |
| serde_json | Core | JSON manipulation         |
| tokio      | Core | Async runtime             |
| mcp-sdk-rs | Core | MCP server implementation |
| clap       | Core | CLI argument parsing      |
| reqwest    | Core | HTTP requests             |
| serde_yaml | Core | YAML configuration        |
| toml       | Core | TOML configuration        |

---

## 12. Constraints

- Directly executes commands on the system
- No database required (file-based history only)
- Platform: Linux
