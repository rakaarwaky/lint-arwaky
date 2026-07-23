# FRD — maintenance

## System Overview

The maintenance crate provides operational health and upkeep commands for the lint-arwaky system: environment diagnostics, toolchain verification, cache cleanup, tool updates, security scanning, dependency reporting, and project statistics. It is the ops-focused crate — it handles environment health, not code quality analysis.

```
┌──────────────────────────────────────────────────────┐
│            MaintenanceCommandsOrchestrator            │
│  ┌────────────────┐     ┌─────────────────────────┐  │
│  │  doctor()      │     │  MaintenanceChecker      │  │
│  │  stats()       │     │  (toolchain diagnose,    │  │
│  │  clean()       │     │   security scan,         │  │
│  │  update()      │     │   dependency report)     │  │
│  └────────────────┘     └─────────────────────────┘  │
│                          ┌─────────────────────────┐  │
│                          │  ToolExecutorAdapter     │  │
│                          │  (subprocess execution)  │  │
│                          └─────────────────────────┘  │
└──────────────────────────────────────────────────────┘
```

## Functional Requirements

### FR-001: Environment Health Check (doctor)
- **Description**: Verify that required linter tools are installed and configuration files exist in the project root.
- **Input**: None (operates on current working directory).
- **Output**: `DoctorResultVO` containing `python_version`, `is_installed`, `config_found`, `adapter_statuses` (HashMap), `issues` (Vec<ErrorMessage>), `healthy` (ComplianceStatus).
- **Business Rules**:
  - Checks for config files: `.lint_arwaky.json`, `lint_arwaky.config.yaml`, `pyproject.toml`.
  - Checks adapter availability for: `ruff`, `mypy`, `bandit`, `radon` via `which` command.
  - Checks `lint-arwaky` pip package installation via `pip show`.
  - If no config file found, adds "No configuration file found" issue.
  - If adapter not found, adds "Linter adapter '<name>' is not installed" issue.
  - `healthy` is true only if `issues` is empty.
- **Edge Cases**:
  - `pip show` command fails — `is_installed` set to false.
  - `which` command fails for adapter — status set to "MISSING".
  - Config files exist but are empty — still counted as found.
- **Error Handling**: No error thrown; issues collected in `DoctorResultVO.issues`.

### FR-002: Project Statistics (stats)
- **Description**: Count Python files and test files in a project, compute test-to-file ratio.
- **Input**: `FilePath` — project root path.
- **Output**: `MaintenanceStatsVO` containing `project_path`, `total_files`, `test_files`, `test_ratio`, `python_files`.
- **Business Rules**:
  - Recursively walks directory tree excluding: `target/`, `.git/`, `node_modules/`, `.venv/`.
  - Counts files with `.py` extension.
  - Test files are identified by filename prefix `test_`.
  - `test_ratio = test_files / total_files` (0.0 if no files).
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
  - Uses `std::fs::remove_dir_all` for each found cache directory.
- **Edge Cases**:
  - Cache directory doesn't exist — no-op.
  - Permission denied on cache directory — `remove_dir_all` returns Err, silently ignored.
  - CWD is inside `target/` — still searches recursively.
- **Error Handling**: `remove_dir_all` failures are silently ignored (returns Ok(())).

### FR-004: Tool Update (update)
- **Description**: Upgrade Python linter tools via pip.
- **Input**: None.
- **Output**: None (side effect: pip install --upgrade executed).
- **Business Rules**:
  - Targets: `ruff`, `mypy`, `bandit`, `radon`.
  - Each tool upgraded independently via `pip install --upgrade <tool>`.
  - Failure of one tool does not prevent others from being upgraded.
- **Edge Cases**:
  - pip not installed — `Command::new("pip")` fails, silently ignored.
  - Tool already at latest version — pip exits successfully (no-op).
  - Network unavailable — pip fails, silently ignored.
- **Error Handling**: All `Command::output()` failures silently ignored.

### FR-005: Diagnose Toolchain
- **Description**: Check installation status and version of Rust, Python, JavaScript, and VCS tools.
- **Input**: None.
- **Output**: `ToolchainDiagnostics` containing `rust_tools`, `python_tools`, `js_tools`, `vcs_tools` (each Vec<ToolStatus>), `binary_path` (String).
- **Business Rules**:
  - Rust tools: `cargo`, `clippy`, `rustfmt` — all required (`required: true`).
  - Python tools: `python3`, `ruff`, `mypy`, `bandit` — all optional.
  - JS tools: `node`, `eslint`, `prettier`, `tsc` — all optional; local `node_modules/.bin/` preferred over global.
  - VCS tools: `git` (required), `jj` (optional).
  - Tool status: `OK` (found), `WARN` (optional, not found), `FAIL` (required, not found).
  - Version extracted from first line of stdout.
- **Edge Cases**:
  - Tool installed but `--version` produces no output — version set to empty string.
  - Local `node_modules/.bin/<tool>` exists — reported as "local" version.
  - Multiple versions installed — only the first found is reported.
- **Error Handling**: Failed tool checks return status without crashing.

### FR-006: Security Scan
- **Description**: Run dependency vulnerability scanning using cargo-audit (Rust) or bandit (Python).
- **Input**: `FilePath` — project root path.
- **Output**: `SecurityScanReport` containing `language`, `tool_name`, `findings` (Vec<SecurityFinding>), `tool_installed` (bool).
- **Business Rules**:
  - If `Cargo.lock` exists → run `cargo audit --json` (Rust project).
  - Otherwise → run `bandit -r --format json <root>` (Python project).
  - Parse JSON output to extract findings with `severity`, `test_id`, `file`, `line`, `issue`.
  - `tool_installed` is always `true` in current implementation.
- **Edge Cases**:
  - Neither `Cargo.lock` nor Python files exist — runs bandit on the directory (may find nothing).
  - cargo-audit not installed — command fails, returns empty findings.
  - JSON parse failure — returns empty findings list.
  - Advisory without CVE ID — `test_id` set to "unknown".
- **Error Handling**: Parse failures result in empty findings; no error propagated.

### FR-007: Dependency Report
- **Description**: Parse project dependency files and list direct and transitive dependencies.
- **Input**: `FilePath` — project root path.
- **Output**: `Result<DependencyReport, String>` containing `language`, `dependencies` (Vec<DependencyInfo>).
- **Business Rules**:
  - Rust projects: parse `Cargo.lock` + `Cargo.toml` to classify deps as "direct" or "transitive".
  - Python projects: parse `pyproject.toml` or `requirements.txt` (fallback chain).
  - Each dependency includes `name`, `version`, `dep_type`.
- **Edge Cases**:
  - No dependency files found (no Cargo.lock, pyproject.toml, requirements.txt) → returns `Err("No dependency files found...")`.
  - Cargo.toml has no `[dependencies]` section — all Cargo.lock entries classified as transitive.
  - requirements.txt has unpinned versions — version set to empty string.
  - pyproject.toml has comments or section headers — skipped during parsing.
- **Error Handling**: File read failures propagate as `Err(String)`.

## Data Model / Entity Relationship

```
DoctorResultVO
├── python_version: DescriptionVO
├── is_installed: ComplianceStatus
├── config_found: FilePathList
├── adapter_statuses: HashMap<AdapterName, String>
├── issues: Vec<ErrorMessage>
└── healthy: ComplianceStatus

MaintenanceStatsVO
├── project_path: FilePath
├── total_files: Count
├── test_files: Count
├── test_ratio: Score
└── python_files: Count

ToolchainDiagnostics
├── rust_tools: Vec<ToolStatus>
├── python_tools: Vec<ToolStatus>
├── js_tools: Vec<ToolStatus>
├── vcs_tools: Vec<ToolStatus>
└── binary_path: String

ToolStatus
├── name: String
├── status: String ("OK" | "WARN" | "FAIL")
└── version: String

SecurityScanReport
├── language: String
├── tool_name: String
├── findings: Vec<SecurityFinding>
└── tool_installed: bool

SecurityFinding
├── severity: String
├── test_id: String
├── file: String
├── line: u64
└── issue: String

DependencyReport
├── language: String
└── dependencies: Vec<DependencyInfo>

DependencyInfo
├── name: String
├── version: String
└── dep_type: String
```

## API Contract

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| `MaintenanceCommandsOrchestrator::doctor()` | — | `DoctorResultVO` | Check environment health: tool installations, config presence. |
| `MaintenanceCommandsOrchestrator::stats()` | `&FilePath` | `MaintenanceStatsVO` | Count Python files and test files, compute ratio. |
| `MaintenanceCommandsOrchestrator::clean()` | — | `()` | Remove cache directories from project tree. |
| `MaintenanceCommandsOrchestrator::update()` | — | `()` | Upgrade Python linter tools via pip. |
| `MaintenanceCommandsOrchestrator::diagnose_toolchain()` | — | `ToolchainDiagnostics` | Check Rust/Python/JS/VCS tool installations. |
| `MaintenanceCommandsOrchestrator::run_security_scan()` | `&FilePath` | `SecurityScanReport` | Run cargo-audit or bandit for vulnerability scanning. |
| `MaintenanceCommandsOrchestrator::run_dependency_report()` | `&FilePath` | `Result<DependencyReport, String>` | Parse and list project dependencies. |
| `MaintenanceCommandsOrchestrator::cancel()` | `JobId` | `()` | Cancel a running operation (currently no-op). |
| `MaintenanceChecker::diagnose_toolchain()` | — | `ToolchainDiagnostics` | Business logic for toolchain diagnostics. |
| `MaintenanceChecker::run_security_scan()` | `&FilePath` | `SecurityScanReport` | Business logic for security scanning. |
| `MaintenanceChecker::run_dependency_report()` | `&FilePath` | `Result<DependencyReport, String>` | Business logic for dependency reporting. |
| `ToolExecutorAdapter::run_tool()` | `&str`, `&[&str]` | `ToolOutput` | Run an external tool as subprocess. |
| `ToolExecutorAdapter::tool_exists()` | `&str` | `bool` | Check if a tool is available via `which`. |

## Integration Points

- **Internal**:
  - `shared::project_setup::MaintenanceCommandsAggregate` — aggregate trait the orchestrator implements.
  - `shared::project_setup::IMaintenanceCheckerProtocol` — protocol interface for checker capabilities.
  - `shared::project_setup::IToolExecutorProtocol` — protocol interface for subprocess execution.
  - `shared::common::utility_command_runner` — shared command execution utilities.
  - `shared::maintenance::utility_dependency_io` — shared dependency file I/O utilities.
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

- [ ] `doctor()` with all tools installed — returns `healthy: true`, all statuses "OK".
- [ ] `doctor()` with missing ruff — returns issue "Linter adapter 'ruff' is not installed".
- [ ] `doctor()` with no config file — returns issue "No configuration file found".
- [ ] `stats()` on Python project — correct file count and test ratio.
- [ ] `stats()` on empty directory — returns all zeros.
- [ ] `clean()` removes `.pytest_cache` directories from project.
- [ ] `clean()` skips `target/` and `.git/` directories.
- [ ] `update()` upgrades ruff, mypy, bandit, radon independently.
- [ ] `diagnose_toolchain()` with cargo installed — reports "OK".
- [ ] `diagnose_toolchain()` with missing clippy — reports "FAIL" (required).
- [ ] `run_security_scan()` on Rust project with Cargo.lock — runs cargo-audit.
- [ ] `run_security_scan()` on Python project — runs bandit.
- [ ] `run_dependency_report()` on Rust project — parses Cargo.lock + Cargo.toml.
- [ ] `run_dependency_report()` with no dependency files — returns `Err`.
- [ ] `run_dependency_report()` on Python project with pyproject.toml — parses dependencies.
- [ ] `tool_exists()` returns true for `cargo`, false for nonexistent tool.

## Assumptions & Constraints

- The crate assumes `pip`, `cargo`, `which`, and other tools are available in the system PATH when invoked.
- Security scanning requires `cargo-audit` (Rust) or `bandit` (Python) to be installed.
- Dependency parsing is line-based (not full TOML/lockfile parsing); may miss edge cases in complex manifests.
- Cache cleanup operates on CWD; the caller must ensure the correct working directory.
- All subprocess operations use `std::process::Command` (synchronous); no Tokio runtime required for this crate's own code.

## Glossary

| Term | Definition |
|------|-----------|
| Toolchain | The set of programming language tools (compilers, linters, formatters) installed on the system. |
| Dependency Report | A listing of all project dependencies with name, version, and classification. |
| Cache Directory | Temporary build/lint output directories that can be safely deleted. |
| Security Finding | A vulnerability detected by cargo-audit or bandit in project dependencies. |

## Reference

- PRD: [PRD.md](../../PRD.md)
