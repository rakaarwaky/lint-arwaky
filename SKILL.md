---
version: 1.10.2
---
# Lint Arwaky Skill

> **GUIDE FOR AI AGENTS.**
> Humans: invoke the `lint-arwaky-cli` binary directly in the terminal.

Rust MCP server for autonomous multi-language linting and architectural governance audits.

## Key Features

- **Multi-Language**: Audits Rust (Clippy + AST), Python (Ruff, MyPy, Bandit, Radon-style metrics), and JavaScript/TypeScript (ESLint, Prettier, TSC) in a single command.
- **Architecture Audit**: Enforces 31 Agentic Engineering System (AES) rules (codes AES001–AES033, with AES028 and AES029 reserved) — e.g., "Surfaces are prohibited from importing Infrastructure" (AES001, AES023).
- **Auto-Fix**: The `fix` subcommand applies safe style fixes without human intervention.
- **Reporting**: Quality score = `100 - sum(penalty)` (no lower bound). Output formats: `text`, `json`, SARIF 2.1.0, JUnit XML.
- **Self-Auditing**: The project scans itself under `lint-arwaky-cli check .` using the same rules it exposes to others.

## Agent Workflow (Recommended)

1. `list_commands(domain="core")` — Discover available subcommands.
2. `execute_command("check", {"path": "src-rust/"})` — Run a quality audit.
3. `execute_command("report", {"path": "src-rust/", "format": "json"})` — Retrieve structured data.
4. `execute_command("fix", {"path": "src-rust/"})` — Apply safe fixes.
5. `health_check()` — Confirm linter adapters are reachable.

## MCP Tools (5 tools)

### `execute_command(action, args)`

Primary dispatch tool. Execute any CLI subcommand. Examples of valid `action` values: `check`, `scan`, `fix`, `report`, `security`, `complexity`, `dependencies`, `setup`, `doctor`, `git-diff`, `multi-project`, `version`.

```json
{
  "action": "check",
  "args": { "path": "src-rust/", "git_diff": false }
}
```

### `list_commands(domain)`

List all available CLI subcommands grouped by domain. Returns rows from `COMMAND_CATALOG` in `src-rust/taxonomy/command_catalog_constant.rs`.

### `commands_schema(tool_name)`

Retrieve the JSON Schema for a registered MCP tool — useful for typed argument construction.

### `read_skill_context(section)`

Read this SKILL.md by section heading, or the entire document when `section` is empty/missing.

### `health_check()`

Check linter adapter liveness and system state. Reports which of the 9 adapters are reachable and the `cargo` toolchain version.

## CLI Subcommands (lint-arwaky-cli)

### Core

- `lint-arwaky-cli check .`: Self-lint this project under AES rules (no path needed).
- `lint-arwaky-cli scan <project-path>`: Scan external/test project with all adapters + AES.
- `lint-arwaky-cli fix [path]`: Apply safe automatic fixes.
- `lint-arwaky-cli report [path] --output-format <text|json|sarif|junit>`: Generate quality report.
- `lint-arwaky-cli ci [path] --threshold <N>`: CI mode; exit 1 if score < threshold (default 80).
- `lint-arwaky-cli git-diff [--base <ref>]`: List files changed since base ref (default `HEAD`).
- `lint-arwaky-cli multi-project <paths...>`: Aggregate lint results across multiple projects.

### Scans

- `lint-arwaky-cli security [path]`: Bandit-style vulnerability scan.
- `lint-arwaky-cli complexity [path]`: Top 5 cyclomatic-complexity hotspots.
- `lint-arwaky-cli duplicates [path]`: 5-line block duplication detection.
- `lint-arwaky-cli trends [path]`: Quality score over time.
- `lint-arwaky-cli dependencies [path]`: Parse and list `Cargo.toml` dependencies.

### Setup & Maintenance

- `lint-arwaky-cli setup init`: Create a default `lint_arwaky.config.yaml`.
- `lint-arwaky-cli setup doctor`: Diagnose environment health and `cargo` toolchain.
- `lint-arwaky-cli setup mcp-config --client <claude|vscode|hermes|all>`: Print MCP configuration.
- `lint-arwaky-cli setup hermes [--remove]`: Add or remove the `[mcp.lint-arwaky]` section in Hermes.
- `lint-arwaky-cli adapters`: List active linter adapters.
- `lint-arwaky-cli config show`: Show active configuration.
- `lint-arwaky-cli cancel <job_id>`: Request cancellation of a running lint job.
- `lint-arwaky-cli version`: Show current version (1.10.2).

### Dev

- `lint-arwaky-cli diff <path1> <path2>`: Compare violation counts and scores between two paths.
- `lint-arwaky-cli import <config_file>`: Import configuration from a JSON/YAML file.
- `lint-arwaky-cli export <sarif|junit|json>`: Export reports in standard formats.
- `lint-arwaky-cli watch [path]`: Poll the path every 2s and re-run lint (Ctrl+C to stop).
- `lint-arwaky-cli suggest [path] [--ai]`: Print top suggestions by file.
- `lint-arwaky-cli install-hook`: Install `lint-arwaky-cli check .` as a git pre-commit hook.
- `lint-arwaky-cli uninstall-hook`: Remove the installed git pre-commit hook.

## Build & Run

```bash
# Build
cargo build --release

# Run CLI
./target/release/lint-arwaky-cli check .

# Run MCP server (speaks JSON-RPC 2.0 over stdin/stdout)
./target/release/lint-arwaky-mcp
```
