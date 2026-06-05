---
version: 1.10.2
---
# Lint Arwaky Skill

> **GUIDE FOR AI AGENTS.**
> Humans: Use the `lint-arwaky` CLI directly in the terminal.

MCP Server for autonomous multi-language linting and architectural governance audits.

## Key Features

- **Multi-Linter**: Runs Ruff, MyPy, Bandit, Radon (Python); ESLint, Prettier, TSC (JS/TS); and Cargo Clippy (Rust) in a single command.
- **Architecture Audit**: Enforces architectural rules (e.g., "Surfaces are prohibited from importing Infrastructure").
- **Auto-Fix**: Automatically fixes code style issues (linting) without intervention.
- **Reporting**: Generates quality scores (100 - sum of violation penalties, no lower bound) and reports in JSON/SARIF/JUnit formats.
- **Hot Reload**: Supports live server code updates during development.

## Agent Workflow (Recommended)

1. `list_commands()` — Discover available commands.
2. `execute_command("check", {"path": "src/"})` — Run a quality audit.
3. `execute_command("fix", {"path": "src/"})` — Fix issues automatically.
4. `execute_command("report", {"path": "src/", "output-format": "json"})` — Retrieve detailed data.

## MCP Tools (5 tools)

### `execute_command(action, args)`

Execute any CLI command. This is the primary tool.
Example actions: check, fix, report, security, complexity, dependencies, setup, doctor.

### `list_commands(domain)`

Lists all available CLI commands along with examples.

### `commands_schema(tool_name)`

Retrieve the JSON schemas for the registered MCP tools.

### `read_skill_context(section)`

Read this SKILL.md documentation by section or in its entirety.

### `health_check()`

Check system health: adapters and system state.

## CLI Command List (lint-arwaky)

### Core

- `lint-arwaky check <path>`: Run all linters and calculate score.
- `lint-arwaky scan <path>`: Alias for check (CI-friendly).
- `lint-arwaky fix <path>`: Apply safe automatic fixes.
- `lint-arwaky report <path> --output-format json`: Generate detailed quality reports.
- `lint-arwaky ci <path>`: CI mode (exit code 1 if score < threshold).
- `lint-arwaky git-diff [--base HEAD]`: Show files changed since base ref.

### Multi-Project

- `lint-arwaky multi-project <paths...>`: Run lint across multiple projects and aggregate results.

### Scans

- `lint-arwaky security <path>`: Scan for vulnerabilities using Bandit/other scanners.
- `lint-arwaky complexity <path>`: Cyclomatic complexity analysis.
- `lint-arwaky duplicates <path>`: Detect code duplication or SRP violations.
- `lint-arwaky trends <path>`: Monitor quality trends over time.
- `lint-arwaky dependencies <path>`: Scan for library vulnerabilities.

### Setup & Maintenance

- `lint-arwaky setup doctor`: Diagnose environment health and linter binaries.
- `lint-arwaky setup init`: Automatic environment configuration.
- `lint-arwaky setup hermes`: Auto-install into Hermes Agent.
- `lint-arwaky setup mcp-config`: Print MCP configuration for clients.
- `lint-arwaky adapters`: List all active linters.
- `lint-arwaky version`: Show current version (1.10.2).
- `lint-arwaky config show`: View active configuration (YAML).
- `lint-arwaky cancel <job_id>`: Cancel a running lint job.

### Dev

- `lint-arwaky diff <path1> <path2>`: Compare lint results between two versions.
- `lint-arwaky import <config_file>`: Import configurations from a JSON/YAML file.
- `lint-arwaky export <sarif|junit|json>`: Export lint reports in standard formats.
- `lint-arwaky watch <path>`: Monitor files and run lint automatically on changes.
- `lint-arwaky suggest <path>`: Provide improvement suggestions (can use --ai).
- `lint-arwaky install-hook`: Install git pre-commit hook.
- `lint-arwaky uninstall-hook`: Remove git pre-commit hook.
