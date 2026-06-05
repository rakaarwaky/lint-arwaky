---
version: 1.10.2
---
# Lint Arwaky Skill

> **GUIDE FOR AI AGENTS.**
> Humans: Use the `auto-lint` CLI directly in the terminal.

MCP Server for autonomous multi-language linting and architectural governance audits.

## Key Features

- **Multi-Linter**: Runs Ruff, MyPy, Bandit, Radon, pip-audit, ESLint, Prettier, and TSC in a single command.
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

### `comands_schema(tool_name)`

Retrieve the JSON schemas for the registered MCP tools.

### `read_skill_context(section)`

Read this SKILL.md documentation by section or in its entirety.

### `health_check()`

Check system health: adapters and system state.

## CLI Command List (auto-lint)

### Core

- `auto-lint check <path>`: Run all linters and calculate score.
- `auto-lint scan <path>`: Alias for check (CI-friendly).
- `auto-lint fix <path>`: Apply safe automatic fixes.
- `auto-lint report <path> --output-format json`: Generate detailed quality reports.
- `auto-lint ci <path>`: CI mode (exit code 1 if score < threshold).
- `auto-lint git-diff [--base HEAD]`: Show files changed since base ref.

### Multi-Project

- `auto-lint multi-project <paths...>`: Run lint across multiple projects and aggregate results.

### Scans

- `auto-lint security <path>`: Scan for vulnerabilities using Bandit.
- `auto-lint complexity <path>`: Cyclomatic complexity analysis (Radon).
- `auto-lint duplicates <path>`: Detect code duplication or SRP violations.
- `auto-lint trends <path>`: Monitor quality trends over time.
- `auto-lint dependencies <path>`: Scan for library vulnerabilities (pip-audit).

### Setup & Maintenance

- `auto-lint setup doctor`: Diagnose environment health and linter binaries.
- `auto-lint setup init`: Automatic environment configuration.
- `auto-lint setup hermes`: Auto-install into Hermes Agent.
- `auto-lint setup mcp-config`: Print MCP configuration for clients.
- `auto-lint adapters`: List all active linters.
- `auto-lint version`: Show current version (1.10.2).
- `auto-lint config show`: View active configuration (YAML).
- `auto-lint cancel <job_id>`: Cancel a running lint job.

### Dev

- `auto-lint diff <path1> <path2>`: Compare lint results between two versions.
- `auto-lint import <config_file>`: Import configurations from a JSON/YAML file.
- `auto-lint export <sarif|junit|json>`: Export lint reports in standard formats.
- `auto-lint watch <path>`: Monitor files and run lint automatically on changes.
- `auto-lint suggest <path>`: Provide improvement suggestions (can use --ai).
- `auto-lint install-hook`: Install git pre-commit hook.
- `auto-lint uninstall-hook`: Remove git pre-commit hook.
