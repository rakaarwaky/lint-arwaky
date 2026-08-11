# FRD — dispatcher (v2.0.0)

---

## System Overview

The dispatcher crate centralizes all business logic for Smart surfaces (CLI, MCP, TUI, API). Smart surfaces are thin wrappers that parse input, call dispatcher functions, and format output. Dispatcher owns the business logic; surfaces own the rendering.

### Architecture & Data Flow

```mermaid
flowchart TD
    A["Smart Surface\n(CLI / MCP / TUI / API)"] -->|"call action"| D["dispatcher\n(Utility Surface)"]

    D -->|"scan / lint"| SCAN["scan actions\n(check, naming, import,\nquality, orphan, role,\nexternal, ci)"]
    D -->|"fix / watch"| FIX["fix & watch actions\n(fix, watch)"]
    D -->|"config / setup / git"| CFG["config & setup actions\n(config, setup, maintenance,\nplugin, git)"]

    SCAN -->|"role / external\nvia subprocess"| S["lint-arwaky-cli\n(--format json)"]
    SCAN -->|"naming / import / quality\norphan via direct call"| B1["naming / import /\nquality / orphan\naggregates"]
    CFG -->|"read_config / adapter_names\ninit / security / hooks"| B2["config / maintenance /\nexternal / setup / git\naggregates"]
    FIX -->|"lint → fix → re-lint"| B6["auto_fix_aggregate"]

    S -->|"ViolationItem[]"| OUT["ViolationItem\n(shared taxonomy)"]
    B1 -->|"LintResult"| OUT
    B2 -->|"ConfigResult\n/ SetupReport / HookReport"| OUT
    B6 -->|"FixReport"| OUT

    OUT -->|"Vec<ViolationItem>\nor Report"| A
```

---

## Functional Requirements

### FR-001: Unified Scan

**What it produces**: Aggregated `Vec<ViolationItem>` from all 6 linters executed as subprocesses.

| Output          | Description                                                              |
| --------------- | ------------------------------------------------------------------------ |
| Violation items | Combined violations from quality, role, import, naming, orphan, external |
| Error message   | User-facing error if path not found or member invalid                    |

**Input**: `ScanOptions` — optional path, filter, member, filesystem aggregate, multi-project orchestrator.

**Business Rules**:

- Runs 6 linters sequentially via `std::process::Command` (self-invocation pattern).
- Each linter invoked with `--format json`, stdout parsed into `ViolationItem`.
- Validates member against discovered workspaces if `multi_project_orchestrator` provided.
- Normalizes relative paths to absolute via filesystem aggregate `canonicalize`.
- Filters violations by target directory (only files within scanned root).
- Optional `filter` parameter: retains only violations whose code contains the filter string.
- AES205 cycle violations are always retained if the file is within the same parent workspace.

**Edge Cases**:

- Path does not exist: returns `Err("Error: path '...' does not exist")`.
- Invalid member name: returns `Err("[error] no workspace member matching '...'")`.
- Linter subprocess fails: silently skipped (no violation produced).
- No violations found: returns empty `Vec`.

**Error Handling**: `Result<Vec<ViolationItem>, String>` — user-facing error messages.

---

### FR-002: CI Threshold Validation

**What it produces**: `CiReport` with pass/fail decision based on score and critical violations.

| Output           | Description                                         |
| ---------------- | --------------------------------------------------- |
| Score            | Computed quality score (0–100)                     |
| Pass/fail        | Whether all CI checks passed                        |
| Reasons          | List of failure reasons (critical, below threshold) |
| Severity counts  | Critical, high, medium, low violation counts        |
| Total violations | Total number of violations found                    |

**Input**: All aggregate dependencies (code analysis, import, naming, config, orphan, filesystem) + optional path + threshold.

**Business Rules**:

- Builds file index once via `filesystem.build_file_index_with_ignored()`.
- Runs 4 rule categories sequentially: quality → import → naming → orphan.
- Passes pre-fetched `file_list()` to each rule checker (zero-I/O pattern).
- Computes score via `ICodeAnalysisAggregate::calc_score()`.
- Auto-fail if any CRITICAL violation detected.
- Auto-fail if score below threshold.

**Edge Cases**:

- Path does not exist: returns `Err`.
- No violations: score = 100, pass = true.
- CRITICAL violation present: always fail regardless of score.

**Error Handling**: `Result<CiReport, String>`.

---

### FR-003: Individual Linter Scanning

**What it produces**: `Vec<ViolationItem>` from a single linter category.

| Output          | Description                         |
| --------------- | ----------------------------------- |
| Violation items | Violations from one specific linter |
| Error message   | User-facing error if path not found |

**Input**: Optional path, linter orchestrator aggregate, filter string, filesystem aggregate.

**Business Rules**:

- Each function follows the same pattern: resolve path → validate → build file index → run audit → convert to ViolationItem → apply filter.
- **Naming**: `naming_orchestrator.run_audit_with_entries(file_list())`
- **Import**: `import_orchestrator.run_audit_with_entries(file_list())`
- **Quality**: `code_analysis_linter.run_analysis_with_entries(file_list())`
- **Orphan**: `orphan_orchestrator.check_orphans_with_entries(file_list(), context)` — supports workspace discovery, member filtering, and unified cross-member orphan graph building.
- **Role** / **External**: Uses subprocess self-invocation (known gap — no direct aggregate variant for single-path scan).
- **External Direct**: Direct call without subprocess, used by CLI `external` subcommand to avoid recursive spawning. Detects languages from file extensions, loads config adapter entries, and passes an `ExternalLintContext` to the orchestrator.

**Edge Cases**:

- Path does not exist: returns `Err`.
- Orphan with multi-workspace: builds unified filesystem across all members, runs unified orphan graph, then filters results per workspace member.
- Role/external subprocess fails: returns empty violations from failed parse.

**Error Handling**: `Result<Vec<ViolationItem>, String>`.

---

### FR-004: Auto-Fix

**What it produces**: `FixReport` with before/after violation counts and fix details.

| Output       | Description                                        |
| ------------ | -------------------------------------------------- |
| Before count | Number of violations before fix                    |
| After count  | Number of violations after fix (0 in dry-run)      |
| Fixed count  | Number of violations resolved                      |
| Fixable list | Violations matching fixable rules (AES101/203/304) |
| Success flag | Whether all violations resolved                    |

**Input**: Optional path, dry_run flag, code analysis aggregate, fix orchestrator factory closure or direct orchestrator instance.

**Business Rules**:

- Runs initial lint to get baseline violation count.
- Filters fixable violations (AES101, AES203, AES304 only).
- In dry-run mode: preview only, no modifications, after_count = before_count.
- In execute mode: runs fix, re-lints, computes delta.
- Two entry points: `collect_fix` (factory closure) and `collect_fix_direct` (pre-built orchestrator for TUI).

**Edge Cases**:

- No fixable violations: `fixable` is empty, fix still runs (no-op).
- Fix orchestrator failure: returns error with fix output.

**Error Handling**: `Result<FixReport, String>`.

---

### FR-005: Git Diff Integration and Hook Management

**What it produces**: `GitDiffReport` with changed files and their violations, or `HookReport` for hook install/uninstall.

| Output           | Description                                 |
| ---------------- | ------------------------------------------- |
| Changed files    | `Vec<FilePath>` of lintable changed files |
| Results          | `Vec<LintResult>` per changed file        |
| Total violations | Count of all violations in changed files    |
| Hook action      | Hook install/uninstall success + message    |

**Input**: For git diff: code analysis aggregate, git base branch name, optional project path, optional filter. For hooks: git hooks aggregate, optional executable path.

**Business Rules**:

- Runs `git diff --name-only <base>...HEAD` via `std::process::Command`.
- Filters to lintable files via `is_lintable()`.
- Runs code analysis on each changed file individually.
- Optional filter restricts to files containing the filter string.
- Hook management: `collect_install_hook` installs a pre-commit hook via `GitHooksAggregate`. `collect_uninstall_hook` removes it.

**Edge Cases**:

- Git diff fails: returns `Err("[error] git diff failed: ...")`.
- No changed files: empty report.
- Non-lintable files changed: excluded from results.
- Hook install fails: returns `HookReport` with success=false and error message.

**Error Handling**: `Result<GitDiffReport, String>` for git diff; `Result<HookReport, String>` for hooks.

---

### FR-006: Configuration Display

**What it produces**: `ConfigShowReport` with redacted config content per language.

| Output   | Description                                 |
| -------- | ------------------------------------------- |
| Entries  | Config file content per language (redacted) |
| Warnings | Errors encountered during config reading    |

**Input**: Config orchestrator aggregate.

**Business Rules**:

- Iterates known languages: Rust, Python, TypeScript.
- Reads config via `orchestrator.read_config()` (sync).
- Redacts AWS keys (`AKIA...`) and long base64-like tokens (>40 chars).
- Returns empty entry for missing configs.

**Edge Cases**:

- Config read error: pushes warning, continues to next language.
- No config files found: returns empty entries with no warnings.

**Error Handling**: Returns `ConfigShowReport` (never errors, warnings embedded).

---

### FR-007: Project Setup

**What it produces**: Setup items, install report, MCP config snippet.

| Output         | Description                                  |
| -------------- | -------------------------------------------- |
| Init items     | List of setup steps with success/failure     |
| Install report | Python and JS adapter installation status    |
| MCP config     | JSON config snippet for specified MCP client |

**Input**: Setup management aggregate, filesystem IO protocol, optional sudo flag, client name.

**Business Rules**:

- `collect_init`: Detects languages, writes config template, distributes docs from XDG config dir to project, copies `.agents/` directory from XDG config.
- `collect_install`: Installs Python and JS adapters via aggregate.
- `collect_mcp_config`: Generates JSON config for the specified MCP client.
- MCP binary resolution: env var `LINT_ARWAKY_MCP_BIN` → sibling of current exe → error (no PATH fallback).

**Edge Cases**:

- XDG config dir not found: warns and skips doc distribution.
- Config write failure: reports error, continues.
- MCP binary not found: error with resolution hint.

**Error Handling**: Embedded in report items (never returns `Err` for init).

---

### FR-008: Maintenance Operations

**What it produces**: Toolchain diagnostics, security scan report, dependency report, adapter health check.

| Output                | Description                        |
| --------------------- | ---------------------------------- |
| Toolchain diagnostics | Tool availability and version info |
| Security scan         | Vulnerability scan results         |
| Dependency report     | Dependency list                    |
| Adapter health check  | 9-adapter availability status      |

**Input**: Maintenance commands aggregate, optional path.

**Business Rules**:

- All operations delegate to `MaintenanceCommandsAggregate` — no direct subprocess calls.
- `collect_doctor`: Toolchain diagnostics via `diagnose_toolchain()`.
- `collect_health_check`: 9-adapter availability via `health_check()`.
- `collect_security`: Security scan at given path via `run_security_scan()`.
- `collect_dependencies`: Dependency report at given path via `run_dependency_report()`.

**Error Handling**: `Result<T, String>` for security and dependencies; direct return for doctor and health check.

---

### FR-009: Plugin Management

**What it produces**: `AdapterNameList` of available external lint adapters, or detailed adapter list with binary availability.

| Output        | Description                      |
| ------------- | -------------------------------- |
| Adapter names | List of registered adapter names |
| Adapter detail | Name, label, and installed flag |

**Input**: External lint aggregate, filesystem aggregate.

**Business Rules**:

- `collect_adapters`: Delegates to `external_lint.adapter_names()`.
- `collect_adapters_detailed`: Scans filesystem for known adapter binaries, returns `Vec<AdapterDetail>` with name, label, and installed status. Includes built-in AST scanners (always available) and external adapters (checked via filesystem).

**Error Handling**: Direct return (infallible).

---

### FR-010: File Watching

**What it produces**: Blocking watch session with Ctrl+C signal handling.

| Output        | Description                   |
| ------------- | ----------------------------- |
| Watch session | Blocks until interrupted      |
| Stop callback | Invoked on Ctrl+C for cleanup |

**Input**: Watch aggregate, optional path, `on_stop` callback.

**Business Rules**:

- Creates `WatchConfig` from resolved path.
- Sets up `ctrlc` handler with atomic running flag.
- Delegates to `watch_aggregate.run(config, running)`.
- `on_stop` callback provided by CLI surface for stop message.

**Edge Cases**:

- Ctrl+C handler setup fails: returns `Err`.
- Watch session fails: returns `Err("watch session failed")`.

**Error Handling**: `Result<(), String>`.

---

### FR-011: Violation Output Component

**What it produces**: `ViolationItem` — the shared violation data type used by all surface actions.

| Output        | Description                                                           |
| ------------- | --------------------------------------------------------------------- |
| ViolationItem | Normalized violation with code, file, line, column, message, severity |

**Input**: Re-exported from `shared::common::ViolationItem`.

**Business Rules**:

- This module re-exports the shared `ViolationItem` type for use by all dispatcher actions.
- The actual implementation (`from_lint_result`, `from_json_obj`, `severity_level`) lives in the shared taxonomy layer.
- All dispatcher actions convert their results to `ViolationItem` before returning to surfaces.

---

### FR-012: Version Info

**What it produces**: `VersionReport` with compile-time version and edition info.

| Output  | Description                                |
| ------- | ------------------------------------------ |
| version | Crate version from `CARGO_PKG_VERSION`    |
| edition | Rust edition from `CARGO_PKG_RUST_VERSION` |

**Input**: None (reads compile-time environment variables).

**Business Rules**:

- Returns `VersionReport` with `version` and `edition` fields populated from `env!()` macros.
- Used by MCP server and CLI to report the tool version.

**Edge Cases**: None (infallible).

**Error Handling**: None (infallible).

---

## Non-functional Requirements

- **Performance**: Scan of 1,000 files completes in < 5s (subprocess overhead included).
- **Memory**: Stateless functions — no persistent state between calls.
- **Sync**: All functions are synchronous. No tokio runtime required.
- **DI**: All aggregate dependencies injected via `Arc<dyn Trait>`. No concrete type imports from lower layers.
- **No Formatting**: Dispatcher returns data only. CLI/MCP/TUI format output themselves.

---

## Test Scenarios

### FR-001: Unified Scan

| # | Scenario                  | Expected                                         |
| - | ------------------------- | ------------------------------------------------ |
| 1 | Scan valid project path   | Returns violations from all 6 linters            |
| 2 | Scan non-existent path    | Returns`Err("Error: path ... does not exist")` |
| 3 | Scan with filter "AES101" | Only AES101 violations returned                  |
| 4 | Scan with invalid member  | Returns`Err("no workspace member matching")`   |
| 5 | Scan empty project        | Returns empty Vec                                |

### FR-002: CI Validation

| # | Scenario                      | Expected                                 |
| - | ----------------------------- | ---------------------------------------- |
| 1 | CI with high threshold (90)   | Pass = true if score >= 90               |
| 2 | CI with CRITICAL violation    | Pass = false (auto-fail)                 |
| 3 | CI with score below threshold | Pass = false, reasons contains score msg |
| 4 | CI with no violations         | Pass = true, score = 100                 |

### FR-003: Individual Linters

| # | Scenario                       | Expected                              |
| - | ------------------------------ | ------------------------------------- |
| 1 | Naming scan on valid project   | Returns naming violations             |
| 2 | Import scan with filter        | Only matching violations returned     |
| 3 | Orphan scan on multi-workspace | Builds unified graph, filters per member |
| 4 | Role scan via subprocess       | Returns violations (or empty on fail) |

### FR-004: Auto-Fix

| # | Scenario                           | Expected                                    |
| - | ---------------------------------- | ------------------------------------------- |
| 1 | Dry-run mode                       | after_count = before_count, fixed_count = 0 |
| 2 | Execute with fixable violations    | fixed_count > 0                             |
| 3 | Execute with no fixable violations | fixable list is empty, no changes           |

### FR-005: Git Diff

| # | Scenario                           | Expected                               |
| - | ---------------------------------- | -------------------------------------- |
| 1 | Diff with 3 changed lintable files | 3 files in report, violations per file |
| 2 | Diff with non-existent base        | Returns git diff error                 |
| 3 | Diff with filter                   | Only matching files included           |

### FR-005: Hook Management

| # | Scenario                    | Expected                            |
| - | --------------------------- | ----------------------------------- |
| 1 | Install hook                | HookReport with success=true       |
| 2 | Uninstall hook              | HookReport with success=true       |
| 3 | Install hook (no .git dir) | HookReport with success=false      |

---

## Glossary

| Term                              | Definition                                                   |
| --------------------------------- | ------------------------------------------------------------ |
| **Utility Surface**         | Crate that centralizes business logic for Smart surfaces     |
| **Smart Surface**           | Thin UI wrapper (CLI, MCP, TUI) that calls dispatcher        |
| **ViolationItem**           | Shared data type for lint violations across all actions      |
| **ScanOptions**             | Input VO for unified scan (path, filter, member, aggregates) |
| **CiReport**                | CI evaluation result with score, threshold, pass/fail        |
| **FixReport**               | Auto-fix outcome with before/after counts                    |
| **GitDiffReport**           | Git-diff lint result with changed files and violations       |
| **HookReport**              | Git hook install/uninstall result with success flag          |
| **Self-invocation pattern** | Subprocess spawning the same binary for linter execution     |

---

## Reference

- PRD: [PRD.md](../../PRD.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
