---
version: 1.10.72
---

# Lint Arwaky Skill

Multi-language linting and AES (Agentic Engineering System) architecture enforcement.

#### Key Features

- **Multi-Language**: Audits Rust (Clippy, Cargo Audit, rustfmt), Python (Ruff, MyPy, Bandit), and JavaScript/TypeScript (ESLint, Prettier, TSC) in a single command.
- **AES Rules**: 24 rules across 5 groups — Naming, Import, Quality, Role, Orphan.
- **Auto-Fix**: `fix` subcommand applies safe style fixes without human intervention.
- **Watch**: Real-time file watching with inotify-based change detection.

# AES Architecture

Agentic Engineering System — 7-layer strict layered architecture enforced by file prefix naming.

| Layer              | Prefix            | Dependency Direction                                 |
| ------------------ | ----------------- | ---------------------------------------------------- |
| **root**           | `root_`           | Wires everything, depends on all                     |
| **surfaces**       | `surface_`        | UI/API entry points → agents                         |
| **agent**          | `agent_`          | Orchestrators → capabilities + infra                 |
| **capabilities**   | `capabilities_`   | Business logic → contracts                           |
| **infrastructure** | `infrastructure_` | Tech adapters → contracts                            |
| **contract**       | `contract_`       | Interfaces (ports, protocols) → taxonomy             |
| **taxonomy**       | `taxonomy_`       | Pure domain VOs, entities, errors, events, constants |

Import rule: `surface` → `agent` → `capabilities` / `infrastructure` → `contract` → `taxonomy`.
Peer layers cannot import each other. Violations are **AES201 (CRITICAL)**.

# AES Rules (24 rules)

| Code   | Group   | Rule                                                               | Severity |
| ------ | ------- | ------------------------------------------------------------------ | -------- |
| AES101 | Naming  | Filename must follow `prefix_concept_suffix` pattern               | HIGH     |
| AES102 | Naming  | Suffix must match layer definition (allowed/forbidden suffixes)    | HIGH     |
| AES201 | Import  | Cross-layer imports must comply with allowed/mandatory/forbidden   | CRITICAL |
| AES202 | Import  | File is missing required imports defined by config                 | HIGH     |
| AES203 | Import  | Symbol is imported but never used in file scope                    | MEDIUM   |
| AES204 | Import  | Import matches forbidden dummy pattern                             | MEDIUM   |
| AES205 | Import  | Circular dependency between layers — must be unidirectional        | HIGH     |
| AES301 | Quality | File exceeds maximum allowed line count (default: 1000)            | LOW      |
| AES302 | Quality | File is below minimum required line count (default: 5)             | LOW      |
| AES303 | Quality | File missing struct/enum/trait definition, or definition empty     | HIGH     |
| AES304 | Quality | Forbidden bypass pattern detected (#[allow], unwrap!, panic!, etc) | CRITICAL |
| AES305 | Quality | Duplicate code blocks detected across files                        | MEDIUM   |
| AES401 | Role    | Constant purity violation or primitive usage in domain models      | HIGH     |
| AES402 | Role    | Contract trait/method uses primitive types instead of VO/constant  | HIGH     |
| AES403 | Role    | Capability has no protocol implementation                          | HIGH     |
| AES404 | Role    | Infrastructure has no port implementation                          | HIGH     |
| AES405 | Role    | Orchestrator does not call any port or protocol                    | MEDIUM   |
| AES406 | Role    | Surface contains active domain logic; file exceeds 25 functions    | MEDIUM   |
| AES501 | Orphan  | Taxonomy file has no inbound imports from any contract file        | LOW      |
| AES502 | Orphan  | Contract trait not implemented by expected layer                   | LOW      |
| AES503 | Orphan  | Capability not wired in container AND unreachable in import graph  | MEDIUM   |
| AES504 | Orphan  | Infrastructure not wired in container AND unreachable in graph     | MEDIUM   |
| AES505 | Orphan  | Agent aggregate not called by surface                              | HIGH     |
| AES506 | Orphan  | Surface not imported by entry/router; utility not imported         | HIGH     |

# MCP Tools

### `health_check()`

Server health and component status.

## `read_skill (section)`

read skilll md context

### `list_commands(domain)`

List available commands with descriptions and examples. Optional `domain` filter (e.g. `"setup"`, `"check"`).

### `execute_command(action, args)`

Execute a command action (check, scan, fix, etc.) with arguments.

# CLI Subcommands

### Core

- `check [path]`: Run full AES compliance analysis.
- `scan [path]`: Deep directory scan (alias for check).
- `fix [path] [--dry-run]`: Apply safe automatic fixes; `--dry-run` to preview.
- `ci [path] [--threshold N]`: CI mode; exit 1 if score < threshold (default 80).
- `orphan <path>`: Check if a file is dead/unreachable code

### Git & Integration

- `install-hook`: Install git pre-commit hook (`lint-arwaky check .`).
- `uninstall-hook`: Remove installed git hook.

### Project

- `watch [path]`: Watch path every 2s and re-lint on changes (Ctrl+C to stop).

### Setup & Config

- `setup init`: Auto-configure lint-arwaky.
- `setup install [--sudo]`: Install linter deps (ruff, mypy, bandit, eslint, prettier, tsc).
- `setup mcp-config --client <claude|hermes|vscode|all>`: Print MCP server config.
- `config show`: Show active configuration.
- `adapters`: List active linter adapters.

### Maintenance

- `maintenance doctor`: Diagnose environment health and toolchain.

### Info

- `version`: Show version.
