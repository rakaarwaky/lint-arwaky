# FRD — maintenance

## System Overview

The maintenance crate provides operational health and upkeep commands for the lint-arwaky system: environment diagnostics, toolchain verification, cache cleanup, tool updates, security scanning, dependency reporting, and project statistics. It is the ops-focused crate — it handles environment health, not code quality analysis.

```
┌──────────────────────────────────────────────────────┐
│           the maintenance orchestrator                │
│  ┌────────────────┐     ┌─────────────────────────┐  │
│  │  doctor        │     │  the maintenance checker │  │
│  │  stats         │     │  (toolchain diagnose,    │  │
│  │  clean         │     │   security scan,         │  │
│  │  update        │     │   dependency report)     │  │
│  └────────────────┘     └─────────────────────────┘  │
│                          ┌─────────────────────────┐  │
│                          │  the tool executor       │  │
│                          │  (subprocess execution)  │  │
│                          └─────────────────────────┘  │
└──────────────────────────────────────────────────────┘
```

## Functional Requirements

### FR-001: Environment Health Check (doctor)

- **Description**: Verify that required linter tools are installed and configuration files exist in the project root.
- **Input**: None (operates on current working directory).
- **Output**: Doctor result containing python version, installation status, config presence, adapter statuses (map), issues list, and overall health status.
- **Business Rules**:
  - Checks for config files: `.lint_arwaky.json`, `lint_arwaky.config.yaml`, `pyproject.toml`.
  - Checks adapter availability for: `ruff`, `mypy`, `bandit`, `radon` via `which` command.
  - Checks lint-arwaky pip package installation via `pip show`.
  - If no config file found, adds "No configuration file found" issue.
  - If adapter not found, adds "Linter adapter '<name></name>' is not installed" issue.
  - Health is true only if issues list is empty.
- **Edge Cases**:
  - `pip show` command fails — installation status set to false.
  - `which` command fails for adapter — status set to "MISSING".
  - Config files exist but are empty — still counted as found.
- **Error Handling**: No error thrown; issues collected in the result's issues list.

### FR-002: Project Statistics (stats)

- **Description**: Count Python files and test files in a project, compute test-to-file ratio.
- **Input**: Project root path.
- **Output**: Maintenance stats containing project path, total files, test files, test ratio, and python files.
- **Business Rules**:
  - Recursively walks directory tree excluding: `target/`, `.git/`, `node_modules/`, `.venv/`.
  - Counts files with `.py` extension.
  - Test files are identified by filename prefix `test_`.
  - Test ratio = test files / total files (0.0 if no files).
- **Edge Cases**:
  - Empty project (no `.py` files) — all counts 0, ratio 0.0.
  - Symlinks — followed if they point to directories.
  - Permission denied on subdirectory — silently skipped.
- **Error Handling**: Walk failures are silently ignored; partial results returned.

### FR-003: Cache Cleanup (clean)

- **Description**: Remove cache directories from the project tree.
- **Input**: None (operates on current working directory).
- **Output**: None (side effect: directories deleted).
- **Business Rules**:
  - Targets: `.pytest_cache`, `.mypy_cache`, `.ruff_cache`, `__pycache__`, `.lint_arwaky_cache`.
  - Recursively searches from CWD, excluding `target/`, `.git/`, `node_modules/`.
  - Uses filesystem directory removal for each found cache directory.
- **Edge Cases**:
  - Cache directory doesn't exist — no-op.
  - Permission denied on cache directory — removal fails, silently ignored.
  - CWD is inside `target/` — still searches recursively.
- **Error Handling**: Directory removal failures are silently ignored.

### FR-004: Tool Update (update)

- **Description**: Upgrade Python linter tools via pip.
- **Input**: None.
- **Output**: None (side effect: pip install --upgrade executed).
- **Business Rules**:
  - Targets: `ruff`, `mypy`, `bandit`, `radon`.
  - Each tool upgraded independently via pip install --upgrade command.
  - Failure of one tool does not prevent others from being upgraded.
- **Edge Cases**:
  - pip not installed — command fails, silently ignored.
  - Tool already at latest version — pip exits successfully (no-op).
  - Network unavailable — pip fails, silently ignored.
- **Error Handling**: All subprocess output failures silently ignored.

### FR-005: Diagnose Toolchain

- **Description**: Check installation status and version of Rust, Python, JavaScript, and VCS tools.
- **Input**: None.
- **Output**: Toolchain diagnostics containing rust tools, python tools, js tools, vcs tools (each a list of tool statuses), and binary path (string).
- **Business Rules**:
  - Rust tools: `cargo`, `clippy`, `rustfmt` — all required.
  - Python tools: `python3`, `ruff`, `mypy`, `bandit` — all optional.
  - JS tools: `node`, `eslint`, `prettier`, `tsc` — all optional; local `node_modules/.bin/` preferred over global.
  - VCS tools: `git` (required), `jj` (optional).
  - Tool status: `OK` (found), `WARN` (optional, not found), `FAIL` (required, not found).
  - Version extracted from first line of stdout.
- **Edge Cases**:
  - Tool installed but version command produces no output — version set to empty string.
  - Local `node_modules/.bin/<tool>` exists — reported as "local" version.
  - Multiple versions installed — only the first found is reported.
- **Error Handling**: Failed tool checks return status without crashing.

### FR-006: Security Scan

- **Description**: Run dependency vulnerability scanning using cargo-audit (Rust) or bandit (Python).
- **Input**: Project root path.
- **Output**: Security scan report containing language, tool name, findings list, and tool installed status.
- **Business Rules**:
  - If `Cargo.lock` exists → run `cargo audit --json` (Rust project).
  - Otherwise → run `bandit -r --format json <root>` (Python project).
  - Parse JSON output to extract findings with severity, test id, file, line, and issue description.
  - Tool installed status is always true in current implementation.
- **Edge Cases**:
  - Neither `Cargo.lock` nor Python files exist — runs bandit on the directory (may find nothing).
  - cargo-audit not installed — command fails, returns empty findings.
  - JSON parse failure — returns empty findings list.
  - Advisory without CVE ID — test id set to "unknown".
- **Error Handling**: Parse failures result in empty findings; no error propagated.

### FR-007: Dependency Report

- **Description**: Parse project dependency files and list direct and transitive dependencies.
- **Input**: Project root path.
- **Output**: Result containing language and dependencies list.
- **Business Rules**:
  - Rust projects: parse `Cargo.lock` + `Cargo.toml` to classify deps as "direct" or "transitive".
  - Python projects: parse `pyproject.toml` or `requirements.txt` (fallback chain).
  - Each dependency includes name, version, and dependency type.
- **Edge Cases**:
  - No dependency files found (no Cargo.lock, pyproject.toml, requirements.txt) → returns error.
  - Cargo.toml has no `[dependencies]` section — all Cargo.lock entries classified as transitive.
  - requirements.txt has unpinned versions — version set to empty string.
  - pyproject.toml has comments or section headers — skipped during parsing.
- **Error Handling**: File read failures propagate as error.

## Data Model / Entity Relationship

```
Doctor Result
├── python_version: description
├── is_installed: compliance status
├── config_found: list of config file paths
├── adapter_statuses: map of adapter name to status string
├── issues: list of error messages
└── healthy: compliance status

Maintenance Stats
├── project_path: file path
├── total_files: count
├── test_files: count
├── test_ratio: score
└── python_files: count

Toolchain Diagnostics
├── rust_tools: list of tool statuses
├── python_tools: list of tool statuses
├── js_tools: list of tool statuses
├── vcs_tools: list of tool statuses
└── binary_path: string

Tool Status
├── name: string
├── status: string ("OK" | "WARN" | "FAIL")
└── version: string

Security Scan Report
├── language: string
├── tool_name: string
├── findings: list of security findings
└── tool_installed: boolean

Security Finding
├── severity: string
├── test_id: string
├── file: string
├── line: u64
└── issue: string

Dependency Report
├── language: string
└── dependencies: list of dependency info

Dependency Info
├── name: string
├── version: string
└── dep_type: string
```

## API Contract


| Function                                           | Input           | Output                | Description                                                    |
| ---------------------------------------------------- | ----------------- | ----------------------- | ---------------------------------------------------------------- |
| the maintenance orchestrator doctor                | —              | doctor result         | Check environment health: tool installations, config presence. |
| the maintenance orchestrator stats                 | project path    | maintenance stats     | Count Python files and test files, compute ratio.              |
| the maintenance orchestrator clean                 | —              | ()                    | Remove cache directories from project tree.                    |
| the maintenance orchestrator update                | —              | ()                    | Upgrade Python linter tools via pip.                           |
| the maintenance orchestrator diagnose toolchain    | —              | toolchain diagnostics | Check Rust/Python/JS/VCS tool installations.                   |
| the maintenance orchestrator run security scan     | project path    | security scan report  | Run cargo-audit or bandit for vulnerability scanning.          |
| the maintenance orchestrator run dependency report | project path    | result                | Parse and list project dependencies.                           |
| the maintenance orchestrator cancel                | job id          | ()                    | Cancel a running operation (currently no-op).                  |
| the maintenance checker diagnose toolchain         | —              | toolchain diagnostics | Business logic for toolchain diagnostics.                      |
| the maintenance checker run security scan          | project path    | security scan report  | Business logic for security scanning.                          |
| the maintenance checker run dependency report      | project path    | result                | Business logic for dependency reporting.                       |
| the tool executor run tool                         | tool name, args | tool output           | Run an external tool as subprocess.                            |
| the tool executor tool exists                      | tool name       | boolean               | Check if a tool is available via`which`.                       |

## Integration Points

- **Internal**:
  - The maintenance commands aggregate in the shared crate — aggregate trait the orchestrator implements.
  - The maintenance checker protocol in the shared crate — protocol interface for checker capabilities.
  - The tool executor protocol in the shared crate — protocol interface for subprocess execution.
  - The command runner utility in the shared crate — shared command execution utilities.
  - The dependency I/O utility in the shared crate — shared dependency file I/O utilities.
- **External**:
  - `cargo audit --json` — Rust dependency vulnerability scanning.
  - `bandit -r --format json` — Python security vulnerability scanning.
  - `pip install --upgrade` — Python tool installation/upgrade.
  - `which <tool>` — tool availability detection.
  - Filesystem I/O: `std::fs::read_dir`, `std::fs::remove_dir_all`.

## Non-functional Requirements (Detailed)

- **Performance**: Doctor check completes in <2s (4 tool checks + config scan). Stats walk scales linearly with file count. Cache cleanup is O(n) in directory tree size.
- **Memory**: Dependency report loads entire Cargo.lock/pyproject.toml into memory; suitable for projects with <10K dependencies.
- **Accuracy**: Tool availability reflects exact state of system PATH at invocation time. Dependency classification (direct vs transitive) relies on Cargo.toml `[dependencies]` section.

## Test Scenarios / QA Checklist

- [ ]  Doctor with all tools installed — returns healthy: true, all statuses "OK".
- [ ]  Doctor with missing ruff — returns issue "Linter adapter 'ruff' is not installed".
- [ ]  Doctor with no config file — returns issue "No configuration file found".
- [ ]  Stats on Python project — correct file count and test ratio.
- [ ]  Stats on empty directory — returns all zeros.
- [ ]  Clean removes `.pytest_cache` directories from project.
- [ ]  Clean skips `target/` and `.git/` directories.
- [ ]  Update upgrades ruff, mypy, bandit, radon independently.
- [ ]  Diagnose toolchain with cargo installed — reports "OK".
- [ ]  Diagnose toolchain with missing clippy — reports "FAIL" (required).
- [ ]  Security scan on Rust project with Cargo.lock — runs cargo-audit.
- [ ]  Security scan on Python project — runs bandit.
- [ ]  Dependency report on Rust project — parses Cargo.lock + Cargo.toml.
- [ ]  Dependency report with no dependency files — returns error.
- [ ]  Dependency report on Python project with pyproject.toml — parses dependencies.
- [ ]  Tool exists returns true for `cargo`, false for nonexistent tool.

## Assumptions & Constraints

- The crate assumes `pip`, `cargo`, `which`, and other tools are available in the system PATH when invoked.
- Security scanning requires `cargo-audit` (Rust) or `bandit` (Python) to be installed.
- Dependency parsing is line-based (not full TOML/lockfile parsing); may miss edge cases in complex manifests.
- Cache cleanup operates on CWD; the caller must ensure the correct working directory.
- All subprocess operations use `std::process::Command` (synchronous); no Tokio runtime required for this crate's own code.

## Glossary


| Term              | Definition                                                                                      |
| ------------------- | ------------------------------------------------------------------------------------------------- |
| Toolchain         | The set of programming language tools (compilers, linters, formatters) installed on the system. |
| Dependency Report | A listing of all project dependencies with name, version, and classification.                   |
| Cache Directory   | Temporary build/lint output directories that can be safely deleted.                             |
| Security Finding  | A vulnerability detected by cargo-audit or bandit in project dependencies.                      |

## Reference

- PRD: [PRD.md](../../PRD.md)
