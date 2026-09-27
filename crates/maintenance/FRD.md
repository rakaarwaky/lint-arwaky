# FRD — maintenance (v2.1.0)

---

## Reference

- PRD: [PRD.md](../../PRD.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature; this file is specification only.

## System Overview

The maintenance crate provides operational health and upkeep commands for the
lint-arwaky system: environment diagnostics, toolchain verification, adapter
health checking, cache cleanup, tool updates, binary self-update from
GitHub releases, security scanning, dependency reporting, and project
statistics. It is the ops-focused crate — it handles environment health,
not code quality analysis.

The crate follows the AES 7-layer architecture: the maintenance checker
(capabilities) implements the maintenance checker protocol, the maintenance
orchestrator (agent) delegates to the protocol, and the maintenance container
(root) wires dependencies.

### Architecture & Data Flow

```mermaid
flowchart TD
    A["Surface"] -->|input| B["maintenance aggregate"]
    B --> C{"action"}

    C -->|"doctor / diagnose / health"| D["maintenance checker"]
    C -->|"security / dependencies"| D
    C -->|"stats / clean / update"| D
    C -->|"self-update"| D

    D --> F["filesystem / tool executor"]
    F -->|subprocess| G["Tool Output"]
    G --> D
    D --> H["Maintenance Result"]

    H --> B
    B -->|output| A

```

---

## Functional Requirements

### FR-MAINTENANCE-001: Environment Health Check (doctor)

- **Description**: Build a health status report by combining toolchain
  diagnostics with adapter status information.
- **Input**: None (operates on current working directory).
- **Output**: Doctor result containing language versions, adapter statuses
  (map of tool name to status), and overall health status.
- **Business Rules**:

  - Calls `diagnose_toolchain` to obtain per-tool status (OK/WARN/FAIL)
    and version strings for Rust, Python, JS, and VCS tools.
  - Extracts rust, python, and node version from the first tool in each
    category.
  - Builds adapter statuses map from all tool statuses.
  - Health is true only when all required tools have OK status.
  - Issues list is currently empty (issues are embedded in tool statuses).
- **Edge Cases**:

  - No tools installed → health is false, adapter statuses all show FAIL.
  - Partial toolchain (e.g., Python only) → health is false.
- **Error Handling**: No error thrown; all information is embedded in the
  result structure.

---

### FR-MAINTENANCE-002: Project Statistics (stats)

- **Description**: Count source files and test files in the top-level
  directory of a project, compute test-to-file ratio.
- **Input**: Project root path.
- **Output**: Maintenance stats containing project path, total file count,
  test file count, test ratio, and per-language file counts (Rust, Python,
  JS/TS).
- **Business Rules**:

  - Reads entries in the top-level project directory (non-recursive).
  - Counts files by extension: `.rs` (Rust), `.py` (Python),
    `.ts`, `.js`, `.jsx`, `.tsx` (JS/TS).
  - Identifies test files by name containing "test" or "spec".
  - Test ratio = test files / source files (0.0 if no source files).
  - Source count is the sum of Rust + Python + JS/TS files.
- **Edge Cases**:

  - Empty directory (no files) → all counts 0, ratio 0.0.
  - Non-source files (e.g., `.md`, `.yaml`) → counted in total but not
    in per-language breakdowns.
- **Error Handling**: Directory read failure → returns default stats with
  zero counts.

---

### FR-MAINTENANCE-003: Cache Cleanup (clean)

- **Description**: Remove known cache directories from the project tree.
- **Input**: None (operates on current working directory).
- **Output**: None (side effect: directories deleted).
- **Business Rules**:

  - Targets: `.pytest_cache`, `__pycache__`, `node_modules/.cache`,
    `target`.
  - Checks each target in the current directory and removes it if present.
- **Edge Cases**:

  - Cache directory doesn't exist → no-op.
  - Permission denied on cache directory → removal fails silently.
- **Error Handling**: Directory removal failures are silently ignored.

---

### FR-MAINTENANCE-004: Tool Update (update)

- **Description**: Upgrade Python linter tools to their latest versions.
- **Input**: None.
- **Output**: None (side effect: tools upgraded).
- **Business Rules**:

  - Python tools: `ruff`, `mypy`, `bandit` — upgraded via
    `pip install --upgrade`.
  - Each tool upgraded in a single pip invocation.
  - JS/TS tools and Rust tools are **not upgraded** by this command.
- **Edge Cases**:

  - pip not installed → command fails, warning printed.
  - Tool already at latest version → package manager exits successfully.
  - Network unavailable → package manager fails, warning printed.
- **Error Handling**: Failure logged as warning; no crash.

---

### FR-MAINTENANCE-005: Diagnose Toolchain

- **Description**: Check installation status and version of Rust, Python,
  JavaScript, and VCS tools.
- **Input**: None.
- **Output**: Toolchain diagnostics containing rust tools, python tools,
  js tools, vcs tools (each a list of tool statuses), and binary path.
- **Business Rules**:

  - Rust tools: `rustc`, `cargo`, `clippy` (via `cargo clippy`), `rustfmt`
    — all required.
  - Python tools: `python3`, `ruff`, `mypy` — all optional.
  - JS tools: `node`, `eslint` — all optional.
  - VCS tools: `git` (required).
  - Tool status: `OK` (found), `WARN` (optional, not found), `FAIL`
    (required, not found).
  - Version extracted from first line of stdout.
  - Clippy is checked via `cargo clippy --version` and reported as "clippy".
- **Edge Cases**:

  - Tool installed but version command produces no output → version set to
    empty string.
  - Multiple versions installed → only the first found is reported.
- **Error Handling**: Failed tool checks return status without crashing.

---

### FR-MAINTENANCE-006: Security Scan

- **Description**: Run dependency vulnerability scanning using cargo-audit
  for Rust projects.
- **Input**: Project root path.
- **Output**: Security scan report containing language, tool name, findings
  list, and tool installed status.
- **Business Rules**:

  - Checks if `Cargo.lock` exists at the project root.
  - If present, runs `cargo audit --json` and parses JSON output for
    vulnerability advisories.
  - Extracts findings with severity, advisory ID, package name, and
    issue description.
  - If `Cargo.lock` does not exist, returns empty findings with
    `tool_installed: false`.
- **Edge Cases**:

  - `Cargo.lock` missing → returns empty findings with tool_installed false.
  - cargo-audit not installed → returns empty findings with warning.
  - JSON parse failure → returns empty findings list with warning.
  - Advisory without CVE ID → test id set to "unknown".
- **Error Handling**: Parse failures result in empty findings with warning;
  no crash. Tool not installed → `tool_installed: false`.

---

### FR-MAINTENANCE-007: Dependency Report

- **Description**: Parse Rust project dependency files and list direct and
  transitive dependencies.
- **Input**: Project root path.
- **Output**: Result containing language ("Rust") and dependencies list.
- **Business Rules**:

  - Checks if `Cargo.lock` exists at the project root.
  - Parses `Cargo.lock` line by line, extracting package name and version
    from `[[package]]` entries.
  - All entries classified as "transitive" (Cargo.lock does not distinguish
    direct vs transitive without Cargo.toml cross-reference).
  - Each dependency includes name, version, and dependency type.
- **Edge Cases**:

  - No `Cargo.lock` found → returns error.
  - Empty `Cargo.lock` → returns empty dependency list.
  - Incomplete package entry at end of file → still captured.
- **Error Handling**: File read failures propagate as error.

---

### FR-MAINTENANCE-008: Adapter Health Check

- **Description**: Check availability of all 9 linter adapters and return
  their installation status.
- **Input**: None.
- **Output**: Health check result containing a list of adapter statuses
  (name, language, available flag).
- **Business Rules**:

  - Checks all 9 adapters via version command:

    | Adapter     | Language |
    | ------------- | ---------- |
    | clippy      | Rust     |
    | rustfmt     | Rust     |
    | cargo-audit | Rust     |
    | ruff        | Python   |
    | mypy        | Python   |
    | bandit      | Python   |
    | eslint      | JS/TS    |
    | prettier    | JS/TS    |
    | tsc         | JS/TS    |
  - Each adapter checked via its version command (e.g., `ruff --version`).
  - Available is `true` if version command succeeds, `false` otherwise.
- **Edge Cases**:

  - Adapter not installed → available set to `false`.
  - Version command produces no output but exits successfully → available
    set to `true`.
- **Error Handling**: No error thrown; unavailable adapters reported in the
  result structure.

---

### FR-MAINTENANCE-009: Self-Update (binary)

- **Description**: Query the latest release from GitHub and install the
  `lint-arwaky-cli` binary when a newer version is available.
- **Input**: None (operates on current executable path).
- **Output**: `SelfUpdateResultVO` — current version, latest tag, upgrade
  status, human-readable message.
- **Business Rules**:
  - Fetches the latest release from `https://api.github.com/repos/rakaarwaky/lint-arwaky/releases/latest`
    using `curl` via the filesystem IO protocol.
  - Compares `CARGO_PKG_VERSION` (normalised by stripping the `v` prefix)
    against the release tag. Only installs when the release tag is strictly
    greater.
  - When `check_only` is `true`, reports the result without downloading.
  - When `check_only` is `false`, downloads the `lint-arwaky-cli` asset and
    the published `lint-arwaky-cli.sha256` checksum to a temporary file in the
    same directory as the running binary, verifies the binary's SHA-256 hash
    against the published value, then moves the verified binary over the
    existing executable and marks it executable with `chmod +x`.
- **Edge Cases**:
  - Network unavailable → graceful error, no crash, `latest_version` empty.
  - Release tag missing or malformed → graceful error.
  - Current executable cannot be resolved → error.
  - Download fails (missing asset, 404) → error logged, no binary replaced.
  - Published checksum missing or SHA-256 mismatch → error logged, no binary
    replaced; the downloaded asset is cleaned up.
  - `chmod` fails → error logged, no binary replaced.
  - Local version is ahead of the released version → no update triggered.
- **Error Handling**: All failures return `SelfUpdateResultVO::error()` with
  a descriptive message; the process exits with `RUNTIME_ERROR`.

---

## API Contract

### Protocol API

| Method | Input | Output | Error | Event | Description |
| --- | --- | --- | --- | --- | --- |
| `execute` | `MaintenanceRequest` | `MaintenanceResponse` | None — errors are carried within each response variant | — | Single composite entry point on the maintenance protocol trait, dispatching every request variant to the checker capabilities. |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
| --- | --- | --- | --- | --- | --- |
| `execute` | `MaintenanceRequest` | `MaintenanceResponse` | None | — | Dispatch a typed maintenance request to the matching capability. |
| `doctor` | — | `DoctorResultVO` | None | — | Build a health status report combining toolchain diagnostics with adapter availability. |
| `diagnose_toolchain` | — | `ToolchainDiagnostics` | None | — | Report per-tool status and version strings for required and optional tools. |
| `health_check` | — | `HealthCheckResult` | None | — | Return availability for all nine linter adapters. |
| `stats` | `FilePath` | `MaintenanceStatsVO` | Directory read failure → default stats with zero counts | — | Count source and test files in the top-level project directory and report the test ratio. |
| `clean` | — | — | Removal failures are silently ignored | — | Remove known cache directories from the current working directory. |
| `update` | — | — | Failure logged as warning; no crash | — | Upgrade Python linter tools via pip. |
| `self_update` | `check_only: bool` | `SelfUpdateResultVO` | All failures return an error variant without replacing the binary | — | Query GitHub for the latest release and install the binary when the release tag is strictly newer. |
| `run_security_scan` | `FilePath` | `SecurityScanReport` | `tool_installed: false` when `cargo-audit` is absent; parse failure → empty findings with warning | — | Run a vulnerability scan on the Rust project using `cargo audit --json`. |
| `run_dependency_report` | `FilePath` | `Result<DependencyReportVO, MROError>` | File read failure propagates as an error | — | Parse `Cargo.lock` and classify every package as transitive. |
| `cancel` | `JobId` | — | No-op | — | Placeholder for future long-running job cancellation. |

---

## Integration Points

| System | Direction | Purpose | Failure mode |
| --- | --- | --- | --- |
| Maintenance checker protocol | in | Protocol interface for maintenance checker capabilities | Missing protocol impl → construction error at startup |
| Maintenance commands aggregate | in | Aggregate trait the orchestrator implements | Missing impl → construction error at startup |
| Tool executor protocol | in | Protocol interface wrapping `std::process::Command` for subprocess execution | Spawn failure → error logged; non-zero exit → captured in result |
| Filesystem aggregate | in | Read, write, delete, directory listing, and external command execution | File I/O failure → warning logged; no crash |
| `std::process::Command` | in | Synchronous subprocess execution for tool checks and installs | Spawn failure → treated as tool not found; non-zero exit → logged as failure |
| `std::fs` | in | Filesystem I/O for cache cleanup operations | Removal failure → silently ignored |
| `cargo audit` | in | Rust dependency vulnerability scanning via JSON output | Tool not installed → empty findings with `tool_installed: false`; JSON parse failure → empty findings with warning |
| `pip install --upgrade` | in | Python tool upgrade for ruff, mypy, bandit | pip not installed → warning printed; network unavailable → warning printed |
| `curl` | in | GitHub releases API query and release asset download for self-update | Network unavailable → `latest_version` empty, graceful error |
| `mv`, `chmod`, `sha256sum` | in | Atomic binary replacement, executable-bit setting, and download integrity verification (via filesystem I/O protocol) | `mv` failure → error logged, no binary replaced; `chmod` failure → error logged; SHA-256 mismatch → downloaded asset cleaned up |
| `which <tool>` | in | Tool availability detection on Unix-like systems | `which` absent → fallback to direct spawn attempt |

---

## Non-functional Requirements

| Metric | Target | Measurement method |
| --- | --- | --- |
| Doctor check latency | < 2s for 10 tool checks plus 3 language version checks | Time a full `doctor` run on a fully provisioned machine |
| Stats scan cost | O(n) in the top-level file count | Time `stats` on directories of increasing file count |
| Cache cleanup cost | Fixed list of directory checks | Time `clean` and assert the cost is independent of project size |
| Memory: dependency report | Whole lockfile held in memory; suitable for projects with < 10K dependencies | Measure peak memory on a lockfile at the stated bound |
| Tool availability accuracy | Reflects the exact state of the system PATH at invocation time | Remove a tool from PATH and assert its status changes in the same run |
| Subprocess model | Synchronous via `std::process::Command`; no async runtime dependency | Confirm the crate declares no async runtime dependency |

---

## Test Scenarios

- A fully provisioned toolchain reports `healthy: true` with all statuses `OK`.
- A missing required tool (e.g. rustc) reports `healthy: false`.
- A missing optional tool (e.g. ruff) shows status `WARN` in the adapter status map.
- Installed language runtimes report their versions in the doctor result.
- A missing language runtime reports version `NOT FOUND`.
- A directory with mixed file types reports per-language counts and the overall total.
- A Python project with test files computes the correct test ratio.
- A directory with no source files reports all zero counts and a 0.0 test ratio.
- An empty directory reports all zero counts and a 0.0 test ratio.
- A project containing cache directories such as `.pytest_cache` and `__pycache__` has them removed by the clean command.
- A project with a `target/` directory has it removed by the clean command.
- A project with no cache directories produces a no-op clean result.
- A Python tool update runs the upgrade command for each tool via the Python package manager.
- An unavailable Python package manager produces a warning and no crash during tool update.
- An installed tool chain (cargo and rustc) reports status `OK` in diagnostics.
- A missing required tool (e.g. clippy) reports status `FAIL` in diagnostics.
- A missing optional tool (e.g. mypy) reports status `WARN` in diagnostics.
- A missing optional tool (e.g. eslint) reports status `WARN` in diagnostics.
- A Rust project with a lock file triggers the cargo-audit scan.
- A project without a lock file returns `tool_installed: false` and empty findings.
- A machine without cargo-audit installed returns `tool_installed: false` and empty findings.
- A scan with no vulnerabilities returns an empty findings list and a success status.
- A dependency report on a Rust project parses every package entry in the lock file.
- A dependency report on a project without a lock file returns an error.
- A dependency report on an empty lock file returns an empty dependency list.
- All nine adapters installed reports `available: true` for each.
- A machine with a missing adapter (e.g. ruff) reports `available: false` for it.
- A machine with no adapters installed reports `available: false` for all nine.
- A GitHub reachable release at the same version reports `already_up_to_date: true` and `upgraded: false`.
- A GitHub reachable release at a newer version sets `latest_version` and reports `upgraded: true` (when not in check-only mode).
- A check-only self-update performs no download and reports the current release tag.
- An unavailable network leaves `latest_version` empty and a status beginning with `Error:`.
- A local version ahead of the release reports `already_up_to_date: true`.
### SCEN-001 — Doctor



FRD Ref: FR-001

| # | Scenario | Expected |
| - | - | - |
| 1 | All required tools OK | healthy: true, all statuses "OK" |
| 2 | Missing rustc (required) | healthy: false |
| 3 | Missing ruff (optional) | Status "WARN" in adapter_statuses |
| 4 | Language runtimes installed | Versions reported (rustc, python3, node) |
| 5 | Language runtime missing | Version "NOT FOUND" |

### SCEN-002 — Stats



FRD Ref: FR-002

| # | Scenario | Expected |
| - | - | - |
| 1 | Directory with mixed files | Per-language counts + overall totals |
| 2 | Python project with test files | Correct test ratio |
| 3 | Directory with no source files | All zeros, ratio 0.0 |
| 4 | Empty directory | All zeros, ratio 0.0 |

### SCEN-003 — Clean



FRD Ref: FR-003

| # | Scenario | Expected |
| - | - | - |
| 1 | Project with .pytest_cache, __pycache__ | Directories removed |
| 2 | Project with target/ | Directory removed |
| 3 | No cache directories | No-op |

### SCEN-004 — Update



FRD Ref: FR-004

| # | Scenario | Expected |
| - | - | - |
| 1 | Python tools upgrade | pip install --upgrade per tool |
| 2 | pip not installed | Warning, no crash |

### SCEN-005 — Diagnose



FRD Ref: FR-005

| # | Scenario | Expected |
| - | - | - |
| 1 | cargo + rustc installed | Status "OK" |
| 2 | Missing clippy (required) | Status "FAIL" |
| 3 | Missing mypy (optional) | Status "WARN" |
| 4 | Missing eslint (optional) | Status "WARN" |

### SCEN-006 — Security



FRD Ref: FR-006

| # | Scenario | Expected |
| - | - | - |
| 1 | Rust project with Cargo.lock | Runs cargo-audit |
| 2 | No Cargo.lock | tool_installed: false, empty findings |
| 3 | cargo-audit not installed | tool_installed: false, empty findings |
| 4 | No vulnerabilities | Empty findings, success |

### SCEN-007 — Dependencies



FRD Ref: FR-007

| # | Scenario | Expected |
| - | - | - |
| 1 | Rust project with Cargo.lock | Parses all packages |
| 2 | No Cargo.lock | Returns error |
| 3 | Empty Cargo.lock | Empty dependency list |

### SCEN-008 — Adapter Health Check



FRD Ref: FR-008

| # | Scenario | Expected |
| - | - | - |
| 1 | All 9 adapters installed | All available: true |
| 2 | Missing ruff | ruff available: false |
| 3 | No adapters installed | All available: false |

### SCEN-009 — Self-Update


FRD Ref: FR-009

| # | Scenario | Expected |
| - | - | - |
| 1 | GitHub reachable, same version | already_up_to_date=true, upgraded=false |
| 2 | GitHub reachable, newer version | latest_version set, upgraded=true (when not check-only) |
| 3 | check_only=true | No download performed; status reports current tag |
| 4 | Network unavailable | latest_version empty, status starts with "Error:" |
| 5 | Local version ahead of release | already_up_to_date=true (local > released) |


---

## Assumptions & Constraints

- The crate assumes `pip`, `cargo`, `npm`, `which`, `curl`, `mv`, and
  `chmod` are available in the system PATH when invoked.
- Security scanning requires `cargo-audit` to be installed for Rust projects.
- Dependency parsing is line-based (not full TOML/lockfile parsing); may
  miss edge cases in complex manifests.
- Cache cleanup operates on CWD; the caller must ensure the correct working
  directory.
- All subprocess operations use `std::process::Command` (synchronous).
  No async runtime dependency.
- The maintenance crate performs its own file walking for ops purposes
  (stats, clean). This is distinct from source code analysis walking
  handled by the filesystem crate.
- Security scanning and dependency reporting currently cover Rust projects only.
  Python and JS/TS support is planned; see BACKLOG.md for status.

---

## Glossary

- **AES**: Agentic Engineering System — the 7-layer coding convention.
- **Toolchain**: The set of programming language tools (compilers, linters, formatters) installed on the system.
- **Dependency Report**: A listing of all project dependencies with name, version, and classification.
- **Cache Directory**: Temporary build or lint output directories that can be safely deleted.
- **Self-Update**: The process of downloading the latest release binary from GitHub and replacing the running executable after verifying its SHA-256 hash.
- **Security Finding**: A vulnerability detected by `cargo audit` in the Rust project's dependency tree.

---
