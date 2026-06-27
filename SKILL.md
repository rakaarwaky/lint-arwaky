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

| Code   | Group   | Rule                                                | Severity |
| ------ | ------- | --------------------------------------------------- | -------- |
| AES101 | Naming  | File suffix must match layer convention             | MAJOR    |
| AES102 | Naming  | Filename must follow `layer_concept_suffix` pattern | MAJOR    |
| AES201 | Import  | Forbidden layer import detected                     | CRITICAL |
| AES202 | Import  | Missing mandatory import for layer                  | CRITICAL |
| AES203 | Import  | Unused import detected                              | MINOR    |
| AES204 | Import  | Dummy/todo-only import detected                     | MINOR    |
| AES205 | Import  | Missing re-export in barrel file                    | MAJOR    |
| AES301 | Quality | File exceeds max line count                         | MINOR    |
| AES302 | Quality | Function exceeds max line count                     | MINOR    |
| AES303 | Quality | Bypass (noqa/allow) suppression detected            | CRITICAL |
| AES304 | Quality | Missing mandatory definition                        | MAJOR    |
| AES305 | Quality | `todo!()` / `unimplemented!()` in non-test          | MINOR    |
| AES306 | Quality | Function exceeds max parameters                     | MINOR    |
| AES307 | Quality | Function exceeds max return types                   | MINOR    |
| AES308 | Quality | Nested function complexity too high                 | MINOR    |
| AES401 | Role    | Layer-role suffix mismatch                          | CRITICAL |
| AES402 | Role    | Bypasses contract aggregate (direct impl dep)       | CRITICAL |
| AES403 | Role    | Capability bypasses agent orchestrator              | CRITICAL |
| AES404 | Role    | Surface calls capability directly                   | CRITICAL |
| AES405 | Role    | Infrastructure implements port without aggregate    | CRITICAL |
| AES406 | Role    | Duplicate registration in container                 | MAJOR    |
| AES501 | Orphan  | Unused/Unreachable file                             | MAJOR    |
| AES502 | Orphan  | Unused contract port/protocol                       | MAJOR    |
| AES503 | Orphan  | Unused capability                                   | MAJOR    |
| AES504 | Orphan  | Dead dependency                                     | MINOR    |
| AES505 | Orphan  | Circular dependency                                 | CRITICAL |
| AES506 | Orphan  | Barrel file with all-unused exports                 | MAJOR    |

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
