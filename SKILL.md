---
name: lint-arwaky
description: Multi-language linting and AES architecture enforcement. Use this skill for scan, check, fix, ci, orphan, watch, setup, adapters, config, maintenance, MCP setup, and release workflows.
---

# Lint Arwaky Skill

See [README.md](README.md) for user-facing usage and [ARCHITECTURE.md](ARCHITECTURE.md) for the full 7-layer specification.

## AES Layers

| Layer              | Prefix            | Role                                                 |
| ------------------ | ----------------- | ---------------------------------------------------- |
| **root**           | `root_`           | Wires everything, depends on all                     |
| **surface**        | `surface_`        | UI/API entry points → agents                         |
| **agent**          | `agent_`          | Orchestrators → capabilities + infra                 |
| **capabilities**   | `capabilities_`   | Business logic → contracts                           |
| **infrastructure** | `infrastructure_` | Tech adapters → contracts                            |
| **contract**       | `contract_`       | Interfaces (ports, protocols) → taxonomy             |
| **taxonomy**       | `taxonomy_`       | Pure domain VOs, entities, errors, events, constants |

Import rule: `surface` → `agent` → `capabilities` / `infrastructure` → `contract` → `taxonomy`.
Peer layers cannot import each other. Violations are **AES201 (CRITICAL)**.

## AES Rules

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

## MCP Tools

- `health_check()` — server health and component status
- `read_skill(section)` — read skill markdown context
- `list_commands(domain)` — list CLI commands, optional domain filter
- `execute_command(action, args)` — execute a CLI action
- `command_schema(tool_name)` — retrieve JSON Schema for a tool

## CLI Commands

### Core

- `check [path]` — run full AES compliance analysis
- `scan [path]` — deep directory scan, alias for `check`
- `fix [path] [--dry-run]` — apply safe automatic fixes
- `ci [path] [--threshold N]` — CI mode; exit 1 if score < threshold
- `orphan <path>` — check if a file is dead/unreachable code

### Git & Integration

- `install-hook` — install git pre-commit hook
- `uninstall-hook` — remove installed git hook

### Project

- `watch [path]` — watch path and re-lint on changes

### Setup & Config

- `setup init` — auto-configure lint-arwaky
- `setup install [--sudo]` — install linter dependencies
- `setup mcp-config --client <claude|hermes|vscode|all>` — print MCP server config
- `config show` — show active configuration
- `adapters` — list active linter adapters

### Maintenance

- `maintenance doctor` — diagnose environment health and toolchain

### Info

- `version` — show version
