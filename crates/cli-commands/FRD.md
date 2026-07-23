# FRD — cli-commands

## Feature Goal

The cli-commands crate provides a unified command-line interface (CLI) that drives the entire lint-arwaky linting pipeline. It implements thin surface handlers that delegate business logic to agent/orchestrator layers (IAnalysisPipelineAggregate, MaintenanceCommandsAggregate, etc.). Report formatting is delegated to the `report-formatter` crate via `IReportFormatterAggregate`.

## Commands & Scope

### Analysis Commands

- **check** — Run full architecture compliance analysis on a target path. Runs all 6 linter groups in sequence: code analysis (AES301-305), naming rules (AES101-102), import rules (AES201-205), external adapters (Clippy, Ruff, ESLint), role rules (AES401-406), orphan detection (AES501-506). Supports `--git-diff` for staged-only scanning.
- **scan** — Multi-workspace discovery scan that auto-detects Cargo.toml/pyproject.toml/package.json members, creates per-project DI containers, and runs the full analysis pipeline on each. Supports `--member <name>` to target a specific workspace member.
- **ci** — CI-optimized analysis with configurable threshold and exit codes. Auto-fails on CRITICAL violations regardless of score. Compares score against threshold as float comparison (not truncated integer).

### Fix Commands

- **fix** — Apply automatic fixes to files that violate rules. Supports `--dry-run` for preview mode. Only auto-fixes safe, non-destructive rule violations.

### Maintenance Commands

- **doctor** — Toolchain diagnostics: checks availability and version of cargo, python3, node, git, and other required tools. Returns exit code 0 regardless of findings (diagnostic only).
- **security** — Vulnerability scanning via cargo-audit (Rust) or bandit (Python). Returns exit code 3 when the scanning tool is missing, exit code 1 when vulnerabilities are found, exit code 0 when clean.
- **dependencies** — Dependency report from Cargo.lock / pyproject.toml / package.json. Lists all dependencies with version and type.

### Project Setup Commands

- **init** — Create default lint-arwaky configuration file in the current project directory (XDG-compliant).
- **install** — Install adapter dependencies (Clippy, Ruff, ESLint, etc.) for the detected language. Supports `--sudo` flag.
- **install-hook** — Install git pre-commit hook that runs lint-arwaky on staged files.
- **uninstall-hook** — Remove the installed git pre-commit hook.
- **mcp-config --client <name>** — Print MCP server configuration for the specified client (e.g., `claude`, `cursor`).
- **config-show** — Display active configuration files and their contents. Sensitive values (AWS keys, long base64 strings) are redacted before display.

### Utility Commands

- **adapters** — List enabled external lint adapters (Clippy, Ruff, ESLint, etc.) discovered by the external-lint layer.
- **watch** — Monitor file changes and trigger re-scans on modified files.
- **orphan <path>** — Check if a specific file is dead/unreachable code by analyzing the workspace import graph.
- **version** — Show version and build information (Git commit hash, Rustc version).

## Architecture

### Layer Delegation Pattern

All surface handlers follow strict AES406 rules:

- **Surfaces** (`surface_*.rs`) — Thin dispatch layer. Parse args, call aggregates, format output. Zero business logic.
- **Agents/Orchestrators** (`agent_*.rs`) — Orchestrate multi-linter pipelines. Depend on contracts only. No I/O, no formatting.
- **Capabilities** (`capabilities_*.rs`) — Single-responsibility implementations (report formatters, etc.). Implement contract protocols.

### Analysis Pipeline

The core analysis pipeline is defined by `IAnalysisPipelineAggregate` trait implemented by `AnalysisPipelineOrchestrator`:

1. Collect source files (ignore-aware via `collect_all_source_files`)
2. Run code analysis (AES301-305)
3. Run naming, import, external, and role audits concurrently
4. Run orphan detection across workspace
5. Merge results, apply path filtering, format output

### Formatters

Report formatting is delegated to the `report-formatter` crate via `IReportFormatterAggregate`. The surface layer never formats output directly — it calls `self.report_formatter.format(&report, format)`. Supported formats:

- **Text** — Human-readable formatted output with severity badges
- **JSON** — Machine-readable structured output for CI/CD integration
- **SARIF 2.1.0** — Standard static analysis results format (VS Code, GitHub Code Scanning)
- **JUnit XML** — Test report format for CI/CD pipelines

## Exit Codes

| Code | Meaning                                           |
| ---- | ------------------------------------------------- |
| 0    | Success — no violations found                     |
| 1    | Violations/findings detected                      |
| 2    | System/operational error                          |
| 3    | Required tool missing (e.g., cargo-audit, bandit) |

## Non-Functional Requirements

- **Cross-platform** — File walker uses canonical paths (not inodes), works on all platforms including Windows.
- **SARIF support** — Full SARIF 2.1.0 output for IDE integration and GitHub Code Scanning.
- **Performance** — Ignore-aware scanning excludes `target/`, `node_modules/`, `.git/`, `dist/`, `build/`, `coverage/`, `.venv/`. Symlink targets outside workspace root are pruned.
- **Concurrency** — Async linter groups run concurrently via `tokio::join!`. Deferred container construction for lightweight commands (version, adapters).
- **Multi-workspace** — Scan auto-discovers workspace members and runs per-project analysis with isolated DI containers.

## Configuration Resolution Algorithm

When loading a configuration file for a given project path, lint-arwaky searches in this priority order:

1. **Project-root YAML** — Search from the project root upward (max 3 levels) for language-specific config files:
   - Rust: `lint_arwaky.config.rust.yaml` or `lint_arwaky.config.yaml`
   - Python: `lint_arwaky.config.python.yaml` or `lint_arwaky.config.yaml`
   - TypeScript: `lint_arwaky.config.javascript.yaml` or `lint_arwaky.config.yaml`

2. **Parent directory traversal** — Walk up parent directories (depth ≤ 3) looking for config files, allowing shared configs at the workspace root to apply to all members.

3. **XDG user config** — Check `<XDG_CONFIG_HOME>/lint-arwaky/<config-file>` (default: `~/.config/lint-arwaky/`).

4. **System XDG dirs** — Check each entry in `$XDG_CONFIG_DIRS` (or `/etc/xdg/lint-arwaky/` by default) for `<config-file>`. Limited to 8 directories, only absolute paths accepted.

5. **Embedded defaults** — If no config file is found at any level, use compiled-in defaults appropriate for the detected language (based on `Cargo.toml`, `pyproject.toml`, or `package.json` presence).

When multiple config files are found across levels, the deepest match wins (most specific path takes priority). The loaded config is cached by file path to avoid re-parsing.

## Dependencies

- `report-formatter` — Report formatting capabilities (text, JSON, SARIF, JUnit)
- `shared` — Taxonomy, contracts, and utility types
- `code-analysis`, `naming-rules`, `import-rules`, `role-rules`, `orphan-detector`, `external-lint` — Linter subsystems

## Success Indicators

- [ ] AES compliance — the crate passes self-lint (`cargo run --bin lint-arwaky-cli -- check`)
- [ ] Surface thinness — surface handlers contain no business logic, only dispatch
- [ ] Formatters delegated — surface uses `IReportFormatterAggregate`, no inline formatting
- [ ] Exit code correctness — all commands follow the standardized exit code convention
- [ ] Secret redaction — config-show never leaks tokens or API keys
