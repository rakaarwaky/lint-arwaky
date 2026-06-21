---
version: 1.10.14
---

# Lint Arwaky Skill

Multi-language linting and AES (Agentic Engineering System) architecture enforcement.

#### Key Features

- **Multi-Language**: Audits Rust (Clippy, Cargo Audit, rustfmt), Python (Ruff, MyPy, Bandit), and JavaScript/TypeScript (ESLint, Prettier, TSC) in a single command.
- **AES Rules**: 24 rules across 5 groups — Naming, Import, Quality, Role, Orphan.
- **Auto-Fix**: `fix` subcommand applies safe style fixes without human intervention.
- **Watch**: Real-time file watching with inotify-based change detection.

# MCP Tools

### `health_check()`

Server health and component status.

## `read_skill (section)`

read skilll md context

### `list_commands(domain)`

List available commands with descriptions and examples. Optional `domain` filter (e.g. `"setup"`, `"check"`).

### `command_schema(tool_name)`

Get argument schema for a specific command.

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
