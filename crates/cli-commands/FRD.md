# FRD — cli-commands

## System Overview

```
┌──────────────────────────────────────────────────┐
│              Surface Layer                        │
│  check/scan/ci command surface handler            │
│  fix command surface handler                      │
│  doctor/security command surface handler          │
│  init/install/mcp command surface handler         │
│  config-show command surface handler              │
│  adapters command surface handler                 │
│  git-diff command surface handler                 │
│  watch command surface handler                    │
│  CI & path utility handlers                       │
├──────────────────────────────────────────────────┤
│              Agent Layer                          │
│  analysis pipeline orchestrator                   │
│  (analysis pipeline aggregate interface)          │
├──────────────────────────────────────────────────┤
│              Utility Layer                        │
│  output format helpers (SARIF/JUnit)              │
│  path resolver (workspace root, language)         │
├──────────────────────────────────────────────────┤
│              Root Container                       │
│  CLI container (DI wiring)                        │
└──────────────────────────────────────────────────┘
```

The cli-commands crate provides the unified command-line interface that drives the entire lint-arwaky linting pipeline. Surface handlers are thin dispatchers that parse CLI args and delegate all business logic to agent/orchestration layers. Report formatting is delegated to the report-formatter crate via the report formatter aggregate.

## Functional Requirements

### FR-001: Check Command (Self-Lint)
- **Description**: Run full architecture compliance analysis on the current lint-arwaky project itself.
- **Input**: `path: Option<FilePath>`, `filter: Option<String>`, `format: Format`, `git_diff: bool`
- **Output**: `ExitCode` (0 = pass, 1 = violations found, 2 = error)
- **Business Rules**:
  - Runs the complete 6-group analysis pipeline sequentially: code analysis (AES301-305), naming rules (AES101-102), import rules (AES201-205), external adapters (Clippy, Ruff, ESLint), role rules (AES401-406), orphan detection (AES501-506).
  - Results filtered to the target path using canonical path comparison.
  - Supports `--git-diff` for staged-only scanning via the git hooks aggregate.
  - Path validated before scanning — returns exit code 2 if path doesn't exist.
- **Edge Cases**:
  - Path doesn't exist → error message + exit code 2.
  - No violations found → exit code 0.
  - Pipeline runtime creation fails → exit code 2.
  - Non-existent path provided → early return with error.
- **Error Handling**: Pipeline failures printed to stderr, exit code 2 returned.

### FR-002: Scan Command (Multi-Workspace)
- **Description**: Multi-workspace discovery scan that auto-detects workspace members and runs analysis on each.
- **Input**: `path: Option<FilePath>`, `filter: Option<String>`, `member: Option<String>`, `format: Format`
- **Output**: `ExitCode` (0 = clean, 1 = violations, 2 = error)
- **Business Rules**:
  - Auto-discovers workspace members via the config orchestrator aggregate.
  - Each workspace member gets isolated analysis with filtered results.
  - `--member <name>` targets a specific workspace member by directory name.
  - In multi-workspace text mode, prints per-member violation summaries with code breakdowns.
  - Falls back to single-scan mode if no workspaces discovered.
  - Pre-computes canonical paths once per workspace for efficient filtering.
- **Edge Cases**:
  - `--member` with non-existent name → error message listing available members.
  - No workspace members discovered → falls back to single-scan.
  - Pipeline fails for a specific workspace → warning logged, continues with others.
  - Empty results across all workspaces → exit code 0.
- **Error Handling**: Pipeline errors per workspace logged as warnings; global errors return exit code 2.

### FR-003: CI Command
- **Description**: CI-optimized analysis with configurable threshold and auto-fail on CRITICAL violations.
- **Input**: `code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>`, `path: Option<FilePath>`, `threshold: Threshold`
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

### FR-004: Fix Command
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

### FR-005: Doctor Command
- **Description**: Toolchain diagnostics — check availability and version of required tools.
- **Input**: `maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>`
- **Output**: `ExitCode` (always 0 — diagnostic only)
- **Business Rules**:
  - Checks Rust toolchain (rustc, cargo, clippy, rustfmt).
  - Checks Python toolchain (python3, ruff, mypy, bandit).
  - Checks JavaScript toolchain (node, npm, eslint, prettier, typescript).
  - Checks VCS tools (git).
  - Displays version and status (OK/MISSING) for each tool.
  - Returns exit code 0 regardless of findings.
- **Edge Cases**:
  - All tools installed → all show OK status.
  - Some tools missing → shows MISSING status, still exit 0.
  - Binary path available → displayed for Rust tools.
- **Error Handling**: None — diagnostic only, always exit 0.

### FR-006: Security Command
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

### FR-007: Dependencies Command
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

### FR-008: Init Command
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

### FR-009: Install Command
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

### FR-010: MCP Config Command
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

### FR-011: Config Show Command
- **Description**: Display active configuration files and their contents with secret redaction.
- **Input**: `orchestrator: Arc<dyn IConfigOrchestratorAggregate>`
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

### FR-012: Adapters Command
- **Description**: List enabled external lint adapters discovered by the external-lint layer.
- **Input**: `external_lint: Arc<dyn IExternalLintAggregate>`
- **Output**: `ExitCode` (always 0)
- **Business Rules**:
  - Queries `adapter_names()` from the external lint aggregate.
  - Lists each adapter on a separate line with bullet prefix.
  - Shows "(none enabled)" when no adapters found.
- **Edge Cases**:
  - No adapters → shows "(none enabled)".
  - Multiple adapters → each listed.
- **Error Handling**: None.

### FR-013: Git Diff Command
- **Description**: Run AES analysis only on files changed since a specified git base.
- **Input**: `git_aggregate: Arc<dyn GitHooksAggregate>`, `code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>`, `base: GitBranchName`, `project_path: Option<&str>`, `filter: Option<&str>`
- **Output**: `ExitCode` (0 = clean, 1 = violations)
- **Business Rules**:
  - Gets changed files from git diff since specified base branch.
  - Filters to lintable files only (via `is_lintable()`).
  - Applies optional filter to changed files.
  - Runs per-file AES analysis with violation details (file:line, severity, message).
  - Shows up to 3 violations per file in summary.
- **Edge Cases**:
  - No changed files → 0 violations, exit 0.
  - File not lintable → skipped.
  - Multiple violations per file → shows top 3.
- **Error Handling**: None — analysis runs per-file independently.

### FR-014: Watch Command
- **Description**: Monitor file changes and trigger re-scans on modified files.
- **Input**: `watch_aggregate: Arc<dyn IWatchAggregate>`, `path: Option<FilePath>`
- **Output**: `ExitCode` (2 = error setting up handler)
- **Business Rules**:
  - Creates a watch configuration from the given path.
  - Sets up Ctrl+C signal handler for graceful shutdown via atomic running flag.
  - Delegates to the watch aggregate run method which blocks until interrupted.
- **Edge Cases**:
  - Ctrl+C handler setup fails → error message + exit code 2.
  - User presses Ctrl+C → prints "Stopping watcher...", graceful shutdown.
- **Error Handler**: Signal handler registration failure → exit code 2.

### FR-015: Analysis Pipeline Orchestration
- **Description**: Coordinate all 6 linter groups to produce a unified ScanReport.
- **Input**: `ScanRequest` (target path, mode, filter, member, format)
- **Output**: `Result<ScanReport, PipelineError>`
- **Business Rules**:
  - Runs linter groups in fixed order: (1) Code analysis, (2-5) Naming, Import, External, Role concurrently, (6) Orphan detection.
  - Each linter group produces lint results merged into single report.
  - Pipeline diagnostics track per-group violation counts and failures.
  - Audit failures (naming, import) reported as warnings, not fatal.
  - Orphan detection uses workspace-wide import graph for reachability analysis.
  - Multi-workspace mode: discovers members, runs per-member, aggregates results with path filtering.
  - Pre-computes canonical paths once per workspace for efficient filtering.
- **Edge Cases**:
  - Naming audit fails → warning diagnostic, continues with other groups.
  - Import audit fails → warning diagnostic, continues.
  - No workspace members → falls back to single-scan mode.
  - Invalid target path → pipeline error with invalid path variant.
- **Error Handling**: Pipeline error variants include invalid path and linter-specific errors propagated.

## Data Model / Entity Relationship

```
ScanRequest
├── target: ScanTarget (FilePath)
├── mode: ScanMode (Scan, Check, Ci)
├── filter: Option<String>
├── member: Option<String>
└── format: Format (Text, Json, Sarif, Junit)

ScanReport
├── results: Vec<LintResult>
├── diagnostics: Vec<PipelineDiagnostic>
└── score: Option<Score>

LintResult
├── file: FilePath
├── line: LineNumber
├── code: LintCode
├── severity: Severity (CRITICAL, HIGH, MEDIUM, LOW, INFO)
├── message: ErrorMessage
└── fixable: bool

PipelineDiagnostic
├── source: String
├── message: String
└── severity: DiagnosticSeverity

Format (enum)
├── Text
├── Json
├── Sarif
└── Junit

ExitCode conventions
├── 0: Success — no violations
├── 1: Violations found
├── 2: System/operational error
└── 3: Required tool missing
```

## API Contract

| Function | Input | Output | Description |
|---|---|---|---|
| `handle_check(opts)` | `CheckOptions` | `ExitCode` | Self-lint analysis on current project |
| `handle_scan(opts)` | `ScanOptions` | `ExitCode` | Multi-workspace analysis with discovery |
| `handle_ci(linter, path, threshold)` | `ICodeAnalysisAggregate, Option<FilePath>, Threshold` | `ExitCode` | CI-mode threshold comparison |
| `handle_fix(path, dry_run, linter, factory)` | `Option<FilePath>, bool, ICodeAnalysisAggregate, factory` | `ExitCode` | Apply automatic fixes |
| `handle_doctor(maintenance)` | `MaintenanceCommandsAggregate` | `ExitCode` | Toolchain diagnostics |
| `handle_security(maintenance, path)` | `MaintenanceCommandsAggregate, Option<FilePath>` | `ExitCode` | Vulnerability scan |
| `handle_dependencies(maintenance, path)` | `MaintenanceCommandsAggregate, Option<FilePath>` | `ExitCode` | Dependency report |
| `handle_init(setup)` | `SetupManagementAggregate` | `ExitCode` | Create config files |
| `handle_install(setup, sudo)` | `SetupManagementAggregate, bool` | `ExitCode` | Install adapter dependencies |
| `handle_mcp_config(client)` | `&str` | `ExitCode` | Print MCP client config JSON |
| `handle_config_show(orchestrator)` | `IConfigOrchestratorAggregate` | `ExitCode` | Display active config files |
| `handle_adapters(external_lint)` | `IExternalLintAggregate` | `ExitCode` | List enabled adapters |
| `handle_git_diff(git, linter, base, path, filter)` | `GitHooksAggregate, ICodeAnalysisAggregate, GitBranchName, Option<&str>, Option<&str>` | `ExitCode` | Analyze git-changed files |
| `handle_watch(watch, path)` | `IWatchAggregate, Option<FilePath>` | `ExitCode` | File watch with auto-lint |
| `run_ci_analysis(linter, path, threshold)` | `ICodeAnalysisAggregate, Option<FilePath>, Threshold` | `ExitCode` | CI pipeline implementation |
| `find_workspace_root(path)` | `&str` | `Option<PathBuf>` | Walk up to find workspace root |
| `detect_language_from_path(path)` | `&str` | `ConfigLanguage` | Detect language from filesystem markers |

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
  - `tokio` — async runtime for concurrent linter execution.
  - `ctrlc` — signal handling for graceful watch shutdown.
  - `regex` — secret redaction pattern matching.

## Non-functional Requirements (Detailed)

- **Cross-platform**: File walker uses canonical paths (not inodes), works on all platforms including Windows.
- **Performance**: Ignore-aware scanning excludes common build/dependency directories. Symlink targets outside workspace root are pruned.
- **Concurrency**: Async linter groups run concurrently. Deferred container construction for lightweight commands (version, adapters).
- **Multi-workspace**: Scan auto-discovers workspace members and runs per-project analysis with isolated DI containers.
- **Security**: MCP binary resolution fails closed (no bare PATH fallback). Config-show redacts AWS keys and base64 secrets. Environment variable for MCP binary path is checked for file existence before use.

## Test Scenarios / QA Checklist

- [ ] FR-001: `check` runs full pipeline and returns correct exit code
- [ ] FR-001: `check` with non-existent path returns exit code 2
- [ ] FR-001: `check --git-diff` filters to staged files only
- [ ] FR-002: `scan` discovers workspace members and runs per-member analysis
- [ ] FR-002: `scan --member <name>` targets specific workspace member
- [ ] FR-002: `scan` with non-existent member name prints available members
- [ ] FR-002: `scan` falls back to single-scan when no workspaces found
- [ ] FR-003: `ci` passes when score >= threshold and no CRITICAL
- [ ] FR-003: `ci` fails on CRITICAL violation regardless of score
- [ ] FR-003: `ci` compares score as float (not truncated integer)
- [ ] FR-004: `fix` applies fixes and reports fixed count
- [ ] FR-004: `fix --dry-run` previews without applying changes
- [ ] FR-005: `doctor` shows all tool statuses and returns exit 0
- [ ] FR-006: `security` returns exit 3 when tool missing
- [ ] FR-006: `security` returns exit 1 when vulnerabilities found
- [ ] FR-007: `dependencies` lists up to 30 deps then truncates
- [ ] FR-008: `init` creates config files for detected languages
- [ ] FR-008: `init` skips existing files
- [ ] FR-009: `install` installs Python and JS adapters
- [ ] FR-010: `mcp-config` generates correct JSON for each client
- [ ] FR-010: `mcp-config` resolves binary via sibling lookup
- [ ] FR-011: `config-show` displays config content with secrets redacted
- [ ] FR-011: `config-show` shows "no config found" when none exists
- [ ] FR-012: `adapters` lists enabled adapters or "(none enabled)"
- [ ] FR-013: `git-diff` analyzes only changed files
- [ ] FR-014: `watch` monitors files and re-scans on changes
- [ ] FR-015: Pipeline runs all 6 linter groups and merges results
- [ ] FR-015: Pipeline handles naming/import audit failures as warnings

## Assumptions & Constraints

- All surface handlers follow AES406: zero business logic, only dispatch.
- Report formatting never happens in surface layer — always delegated to the report formatter aggregate.
- Exit code conventions are standardized: 0=success, 1=violations, 2=error, 3=tool missing.
- Workspace structure follows `crates/`, `packages/`, `modules/` convention.
- MCP binary resolution uses fail-closed strategy (no PATH fallback).
- Config-show always redacts secrets before display.

## Glossary

| Term | Definition |
|---|---|
| AES | Architecture Enforcement Specification — the coding standard enforced by lint-arwaky |
| Pipeline | The 6-group analysis sequence: code analysis, naming, import, external, role, orphan |
| Surface | Thin CLI handler layer — parses args, delegates to agents, formats output |
| Aggregate | Agent-layer orchestrator implementing a contract trait |
| DI Container | Composition root that wires capabilities to contract protocols |
| LintResult | Individual violation finding with file, line, code, severity, message |
| ScanReport | Aggregated results + diagnostics from a full pipeline run |

## Reference

- PRD: [PRD.md](../../PRD.md)
