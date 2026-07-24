# FRD — cli-commands

## System Overview

The cli-commands crate provides the unified command-line interface that drives the entire lint-arwaky linting pipeline. Surface handlers are thin dispatchers that parse CLI args and delegate all business logic to agent/orchestration layers. Report formatting is delegated to the report-formatter crate via the report formatter aggregate.

**Exit Code Contract** (workspace standard — see root PRD):

| Code | Meaning |
| ---- | ------- |
| 0 | Ok / clean / diagnostic completed |
| 1 | Policy fail (violations, CI fail, vulns found, remaining after fix) |
| 2 | Runtime error (bad path, pipeline crash, invalid state) |
| 3 | Prerequisite missing (required external tool not installed) |

**Doctor policy (locked):** exit **0** when the diagnostic finishes (missing tools are listed in the body); exit **2** only if the doctor command itself fails.

## Functional Requirements

### FR-001: Check/Scan Command (Mutual Aliases)

- **Description**: Run full architecture compliance analysis on the target project or workspace. `check` and `scan` are 1:1 equivalent command aliases.
- **Input**: `path: Option<FilePath>`, `filter: Option<String>`, `member: Option<String>`, `format: Format`, `git_diff: bool`
- **Output**: `ExitCode` (0 = pass, 1 = violations found, 2 = error)
- **Business Rules**:
  - `check` and `scan` are 1:1 equivalent aliases that invoke the exact same parallel subprocess analysis pipeline.
  - Runs the complete 6-group analysis pipeline in parallel: code analysis (AES301-305), naming rules (AES101-102), import rules (AES201-205), external adapters (Clippy, Ruff, ESLint), role rules (AES401-406), orphan detection (AES501-506).
  - Results filtered to the target path using canonical path comparison.
  - Supports `--git-diff` for staged-only scanning via the git hooks aggregate.
  - Path validated before scanning — returns exit code 2 if path doesn't exist.
  - Auto-discovers workspace members via the config orchestrator aggregate.
  - Each workspace member gets isolated analysis with filtered results.
  - `--member <name>` targets a specific workspace member by directory name.
  - In multi-workspace text mode, prints per-member violation summaries with code breakdowns.
  - Falls back to single-scan mode if no workspaces discovered.
  - Pre-computes canonical paths once per workspace for efficient filtering.
- **Edge Cases**:
  - Path doesn't exist → error message + exit code 2.
  - No violations found → exit code 0.
  - Pipeline runtime creation fails → exit code 2.
  - `--member` with non-existent name → error message listing available members.
  - No workspace members discovered → falls back to single-scan.
  - Pipeline fails for a specific workspace → warning logged, continues with others.
  - Empty results across all workspaces → exit code 0.
- **Error Handling**: Pipeline failures printed to stderr, exit code 2 returned. Pipeline errors per workspace logged as warnings; global errors return exit code 2.

### FR-002: CI Command

- **Description**: CI-optimized analysis with configurable threshold and auto-fail on CRITICAL violations.
- **Input**: `FilePath`threshold: Threshold`
- **Output**: `ExitCode` (0 = pass, 1 = fail)
- **Business Rules**:
  - Computes architecture compliance score via the score calculation function.
  - Auto-fails on any CRITICAL violation regardless of score.
  - Compares score against threshold as float comparison (not truncated integer).
  - Prints severity breakdown: CRITICAL / HIGH / MEDIUM / LOW counts.
- **Edge Cases**:
  - Score exactly at threshold → passes.
  - CRITICAL violation present but score above threshold → still fails.
  - No violations → score 100, passes.
- **Error Handling**: None — pure computation on existing results.

### FR-003: Fix Command

- **Description**: Apply automatic safe fixes to files that violate rules.
- **Input**: `path: Option<FilePath>`, `dry_run: bool`
- **Output**: `ExitCode` (0 = all fixed, 1 = remaining violations)
- **Business Rules**:
  - Runs lint → apply auto-fixes → re-lint to measure improvement.
  - Supports `--dry-run` for preview mode (no changes applied).
  - Only auto-fixes safe, non-destructive rule violations (naming rules, unused imports, bypass comments).
  - Factory pattern allows the DI container to control fix vs dry-run.
  - Reports fixed count = before - after.
- **Edge Cases**:
  - Dry-run mode → skips second scan, prints preview.
  - No violations before fix → reports 0 fixed.
  - All violations fixed → prints "all violations resolved".
  - Fix operation itself fails → error propagated.
- **Error Handling**: Exit code 1 if any violations remain after fix.

### FR-004: Doctor Command

- **Description**: Toolchain diagnostics — check availability and version of required tools.
- **Input**: Target project context (optional path); maintenance aggregate
- **Output**: `ExitCode` — **0** when diagnostic completes; **2** if the doctor command fails internally
- **Business Rules**:
  - Checks Rust toolchain (rustc, cargo, clippy, rustfmt).
  - Checks Python toolchain (python3, ruff, mypy, bandit).
  - Checks JavaScript toolchain (node, npm, eslint, prettier, typescript).
  - Checks VCS tools (git).
  - Displays version and status (OK/MISSING) for each tool.
  - Missing tools are **reported in the body**, not as exit code 3 (exit 3 is reserved for commands that require a tool to run, e.g. security).
- **Edge Cases**:
  - All tools installed → all show OK status, exit 0.
  - Some tools missing → shows MISSING status, still exit 0.
  - Binary path available → displayed for Rust tools.
- **Error Handling**: Internal failure of doctor → exit 2.

### FR-005: Security Command

- **Description**: Vulnerability scanning via cargo-audit (Rust) or bandit (Python).
- **Input**: `maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>`, `path: Option<FilePath>`
- **Output**: `ExitCode` (0 = clean, 1 = vulnerabilities found, 3 = tool missing)
- **Business Rules**:
  - Auto-detects language from project structure.
  - Runs appropriate scanner (cargo-audit for Rust, bandit for Python).
  - Displays findings with severity, test ID, file, line, and issue description.
  - Exit code 3 when scanning tool is not installed.
- **Edge Cases**:
  - Tool not installed → exit code 3, error message.
  - No vulnerabilities → exit code 0.
  - Vulnerabilities found → exit code 1 with findings listed.
- **Error Handling**: Tool not found → exit code 3; scan failures → exit code 2.

### FR-006: Dependencies Command

- **Description**: Dependency report from Cargo.lock / pyproject.toml / package.json.
- **Input**: `maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>`, `path: Option<FilePath>`
- **Output**: `ExitCode` (0 = success, 2 = error)
- **Business Rules**:
  - Lists all dependencies with name, version, and type.
  - Auto-detects language from project structure.
  - Displays up to 30 dependencies, then "... and N more".
  - Tabular output format with aligned columns.
- **Edge Cases**:
  - More than 30 dependencies → truncated with count.
  - No dependency file found → error message.
  - Invalid dependency file → error propagated.
- **Error Handling**: `Err` from dependency report → error message + exit code 2.

### FR-007: Init Command

- **Description**: Create default lint-arwaky configuration files and distribute documentation.
- **Input**: `setup_orchestrator: Arc<dyn SetupManagementAggregate>`
- **Output**: `ExitCode` (0 = success, 1 = partial failure)
- **Business Rules**:
  - Detects languages present in the project.
  - Creates `lint_arwaky.config.<lang>.yaml` for each detected language.
  - Distributes docs from XDG config: `SKILL.md`, `ARCHITECTURE.md`, `MIGRATION_RUST.md`, `MIGRATION_PYTHON.md`, `MIGRATION_TYPESCRIPT.md`, `RULES_AES.md`.
  - Skips files that already exist.
- **Edge Cases**:
  - Config file already exists → prints "already exists", skips.
  - Doc file not in XDG config → prints "not in XDG config", skips.
  - XDG config directory cannot be determined → warning printed.
  - Write failure → error message, `all_ok` set to false.
- **Error Handling**: Per-file errors logged; overall exit code 1 if any failure.

### FR-008: Install Command

- **Description**: Install adapter dependencies for detected languages.
- **Input**: `setup_orchestrator: Arc<dyn SetupManagementAggregate>`, `sudo: bool`
- **Output**: `ExitCode` (0 = success, 1 = partial failure)
- **Business Rules**:
  - Installs Python adapters: ruff, mypy, bandit.
  - Installs JavaScript adapters: eslint, prettier, typescript.
  - Supports `--sudo` flag for npm global installs requiring elevated permissions.
  - Prints step progress: [1/2] Python, [2/2] JavaScript.
- **Edge Cases**:
  - Python install fails but JS succeeds → exit code 1.
  - Both fail → exit code 1 with suggestion to use `--sudo`.
  - Both succeed → exit code 0 with "Run `lint-arwaky doctor` to verify."
- **Error Handling**: Per-language install status reported; overall exit code 1 if any failure.

### FR-009: MCP Config Command

- **Description**: Print MCP server configuration JSON for a specified client.
- **Input**: `client: &str` (claude, cursor, windsurf, copilot, hermes, vscode, all)
- **Output**: `ExitCode` (always 0)
- **Business Rules**:
  - Generates client-specific JSON configuration for MCP server integration.
  - Binary resolution: (1) `LINT_ARWAKY_MCP_BIN` env var, (2) sibling of current executable, (3) fail closed (no bare PATH fallback).
  - Supports clients: claude-code, cursor, windsurf, copilot, hermes, vscode, all.
  - Canonicalizes binary path for safety.
- **Edge Cases**:
  - `LINT_ARWAKY_MCP_BIN` points to non-file → error.
  - Binary not found → falls back to bare string "lint-arwaky-mcp".
  - Unknown client → uses default mcpServers format.
- **Error Handling**: Binary resolution failure → fallback string; canonicalization failure → error message.

### FR-010: Config Show Command

- **Description**: Display active configuration files and their contents with secret redaction.
- **Input**: the config orchestrator aggregate
- **Output**: `ExitCode` (always 0)
- **Business Rules**:
  - Lists all config files found at project root.
  - Displays raw config content for each file.
  - Redacts sensitive values: AWS access keys (AKIA pattern), long base64 strings (40+ chars).
  - Multiple configs shown with language header.
- **Edge Cases**:
  - No config files found → prints "Run `lint-arwaky init` to create one."
  - Config read fails → warning logged, continues.
  - Multiple config files → each shown with language prefix.
- **Error Handling**: Config read errors logged as warnings.

### FR-011: Adapters Command

- **Description**: List enabled external lint adapters discovered by the external-lint layer.
- **Input**: the external lint aggregate
- **Output**: `ExitCode` (always 0)
- **Business Rules**:
  - Queries adapter names from the external lint aggregate.
  - Lists each adapter on a separate line with bullet prefix.
  - Shows "(none enabled)" when no adapters found.
- **Edge Cases**:
  - No adapters → shows "(none enabled)".
  - Multiple adapters → each listed.
- **Error Handling**: None.

### FR-012: Git Diff Command

- **Description**: Run AES analysis only on files changed since a specified git base.
- **Input**: `git_aggregate: Arc<dyn GitHooksAggregate>`, `code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>`, `base: GitBranchName`, `project_path: Option<&str>`, `filter: Option<&str>`
- **Output**: `ExitCode` (0 = clean, 1 = violations)
- **Business Rules**:
  - Gets changed files from git diff since specified base branch.
  - Filters to lintable files only.
  - Applies optional filter to changed files.
  - Runs per-file AES analysis with violation details (file:line, severity, message).
  - Shows up to 3 violations per file in summary.
- **Edge Cases**:
  - No changed files → 0 violations, exit 0.
  - File not lintable → skipped.
  - Multiple violations per file → shows top 3.
- **Error Handling**: None — analysis runs per-file independently.

### FR-013: Watch Command

- **Description**: Monitor file changes and trigger re-scans on modified files.
- **Input**: Watch aggregate, optional project path
- **Output**: `ExitCode` (0 = clean shutdown; 2 = error setting up handler)
- **Business Rules**:
  - Creates a watch configuration from the given path.
  - Sets up Ctrl+C signal handler for graceful shutdown via atomic running flag.
  - Delegates to the watch aggregate which blocks until interrupted.
- **Edge Cases**:
  - Ctrl+C handler setup fails → error message + exit code 2.
  - User presses Ctrl+C → prints "Stopping watcher...", graceful shutdown, exit 0.
- **Error Handling**: Signal handler registration failure → exit code 2.

### FR-014: Individual Linter Commands (quality, import, naming, role, orphan, external)

- **Description**: Run a single linter independently for targeted analysis.
- **Input**: Optional path, format; orphan may take member filter
- **Output**: `ExitCode` (0 = pass, 1 = violations found, 2 = error)
- **Business Rules**:
  - `quality` — Runs code-quality analysis (AES301-305).
  - `import` — Runs import-rule checks (AES201-205).
  - `naming` — Runs naming-rule checks (AES101-102).
  - `role` — Runs role-rule checks (AES401-406).
  - `orphan` — Runs orphan detection (AES501-506). Supports `--member` for workspace filtering.
  - `external` — Runs external linters (Clippy, Ruff, ESLint).
  - Each command supports `--format` (text, json, sarif, junit).
  - When scanning a specific member path, output shows detailed per-file violations.
  - When scanning a workspace root, output shows compact per-AES-code counts.
- **Edge Cases**:
  - Path doesn't exist → error message + exit code 2.
  - No violations found → exit code 0.
- **Error Handling**: Pipeline failures printed to stderr, exit code 2 returned.


## API Contract


| Operation    | Input                                             | Output    | Description                                              |
| -------------- | --------------------------------------------------- | ----------- | ---------------------------------------------------------- |
| Check        | check options                                     | Exit code | Analysis on project (1:1 equivalent alias of Scan)       |
| Scan         | scan options                                      | Exit code | Multi-workspace analysis (1:1 equivalent alias of Check) |
| Quality      | path, format                                      | Exit code | Code-quality analysis only (AES301-305)                  |
| Import       | path, format                                      | Exit code | Import-rule checks only (AES201-205)                     |
| Naming       | path, format                                      | Exit code | Naming-rule checks only (AES101-102)                     |
| Role         | path, format                                      | Exit code | Role-rule checks only (AES401-406)                       |
| Orphan       | path, member, format                              | Exit code | Orphan detection only (AES501-506)                       |
| External     | path, format                                      | Exit code | External linter checks only (Clippy, Ruff, ESLint)       |
| CI           | linter, path, threshold                           | Exit code | CI-mode threshold comparison                             |
| Fix          | path, dry-run flag, linter, factory               | Exit code | Apply automatic fixes                                    |
| Doctor       | maintenance aggregate                             | Exit code | Toolchain diagnostics                                    |
| Security     | maintenance aggregate, path                       | Exit code | Vulnerability scan                                       |
| Dependencies | maintenance aggregate, path                       | Exit code | Dependency report                                        |
| Init         | setup aggregate                                   | Exit code | Create config files                                      |
| Install      | setup aggregate, sudo flag                        | Exit code | Install adapter dependencies                             |
| MCP Config   | client name                                       | Exit code | Print MCP client config JSON                             |
| Config Show  | config orchestrator aggregate                     | Exit code | Display active config files                              |
| Adapters     | external lint aggregate                           | Exit code | List enabled adapters                                    |
| Git Diff     | git hooks aggregate, linter, branch, path, filter | Exit code | Analyze git-changed files                                |
| Watch        | watch aggregate, path                             | Exit code | File watch with auto-lint                                |

## Integration Points

- **Internal**:
  - `report-formatter` — report formatter aggregate for text/JSON/SARIF/JUnit formatting.
  - `shared` — taxonomy VOs, contract traits, utility functions.
  - `config-system` — config orchestrator aggregate for config loading and workspace discovery.
  - `code-analysis`, `naming-rules`, `import-rules`, `role-rules`, `orphan-detector`, `external-lint` — linter subsystem aggregates.
  - `auto-fix` — fix orchestrator aggregate for automatic fix application.
  - `git-hooks` — git hooks aggregate for git integration.
  - `project-setup` — maintenance commands aggregate, setup management aggregate.
  - `file-watch` — watch aggregate for file monitoring.
- **External**:
  - Async runtime for concurrent linter execution.
  - Signal handling for graceful watch shutdown.
  - Regex library for secret redaction pattern matching.

## Non-functional Requirements (Detailed)

- **Cross-platform**: File walker uses canonical paths (not inodes), works on all platforms including Windows.
- **Performance**: Ignore-aware scanning excludes common build/dependency directories. Symlink targets outside workspace root are pruned.
- **Concurrency**: Async linter groups run concurrently. Deferred container construction for lightweight commands (version, adapters).
- **Multi-workspace**: Scan auto-discovers workspace members and runs per-project analysis with isolated DI containers.
- **Security**: MCP binary resolution fails closed (no bare PATH fallback). Config-show redacts AWS keys and base64 secrets. Environment variable for MCP binary path is checked for file existence before use.

## Test Scenarios / QA Checklist

- [ ] FR-001: `check`/`scan` run full pipeline; correct exit 0/1/2
- [ ] FR-001: non-existent path → exit 2
- [ ] FR-001: `--git-diff` filters to staged/changed files
- [ ] FR-001: workspace member discovery, `--member`, fallback single-scan
- [ ] FR-002: `ci` passes when score ≥ threshold and no CRITICAL
- [ ] FR-002: `ci` fails on CRITICAL regardless of score; float threshold compare
- [ ] FR-003: `fix` applies remove/replace/rename; reports counts
- [ ] FR-003: `fix --dry-run` previews without applying changes
- [ ] FR-004: `doctor` shows tool statuses; exit 0 even when some MISSING
- [ ] FR-004: doctor internal failure → exit 2
- [ ] FR-005: `security` exit 3 when tool missing; exit 1 when vulns found
- [ ] FR-006: `dependencies` lists up to 30 then truncates; error → exit 2
- [ ] FR-007: `init` creates config for detected languages; skips existing
- [ ] FR-008: `install` installs adapters; partial failure → exit 1
- [ ] FR-009: `mcp-config` correct JSON per client; binary resolve fail-closed
- [ ] FR-010: `config-show` redacts secrets; handles missing config
- [ ] FR-011: `adapters` lists enabled adapters or none
- [ ] FR-012: `git-diff` analyzes only changed files
- [ ] FR-013: `watch` monitors files and re-scans; handler fail → exit 2
- [ ] FR-014: individual linters (quality/import/naming/role/orphan/external)

## Assumptions & Constraints

- All surface handlers follow AES406: zero business logic, only dispatch.
- Report formatting never happens in surface layer — always delegated to the report formatter aggregate.
- Exit codes follow the workspace contract: 0 ok, 1 policy fail, 2 runtime error, 3 prerequisite missing.
- Workspace structure follows `crates/`, `packages/`, `modules/` convention.
- MCP binary resolution uses fail-closed strategy (no PATH fallback).
- Config-show always redacts secrets before display.
- MCP execute surface must preserve full parity with these commands (see mcp-server FRD).

## Glossary


| Term         | Definition                                                                            |
| -------------- | --------------------------------------------------------------------------------------- |
| AES          | Architecture Enforcement Specification — the coding standard enforced by lint-arwaky |
| Pipeline     | The 6-group analysis sequence: code analysis, naming, import, external, role, orphan  |
| Surface      | Thin CLI handler layer — parses args, delegates to agents, formats output            |
| Aggregate    | Agent-layer orchestrator implementing a contract trait                                |
| DI Container | Composition root that wires capabilities to contract protocols                        |
| LintResult   | Individual violation finding with file, line, code, severity, message                 |
| ScanReport   | Aggregated results + diagnostics from a full pipeline run                             |

## Reference

- PRD: [PRD.md](../../PRD.md)
