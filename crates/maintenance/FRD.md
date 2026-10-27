# FRD — maintenance (v2.1.0)

---

## Feature Overview

The maintenance crate provides operational health and upkeep commands for the
lint-arwaky system: environment diagnostics, toolchain verification, adapter
health checking, cache cleanup, tool updates, binary self-update from
GitHub releases, security scanning, dependency reporting, and project
statistics. It is the ops-focused crate — it handles environment health,
not code quality analysis.

The crate follows the AES 7-layer architecture. Each of the eight business
capabilities below corresponds to one `_protocol` trait in `shared/maintenance`
and one implementation in `capabilities_maintenance_checker`.

- **Architecture & Data Flow**

```mermaid
flowchart TD
    A["Surface"] -->|input| B["maintenance aggregate"]
    B --> C{"action"}

    C -->|"doctor / diagnose / health"| D["MaintenanceChecker"]
    C -->|"security / dependencies"| D
    C -->|"stats / clean / update"| D
    C -->|"self-update"| D

    D --> F["filesystem IO protocol"]
    F -->|subprocess| G["Tool Output"]
    G --> D
    D --> H["Maintenance Result"]

    H --> B
    B -->|output| A
```

---

## Functional Requirements

### FR-Maintenance-001: Environment Health Check (doctor + toolchain)

- **Description**: Report toolchain diagnostics — the installation status and
  version of every external tool — and build an environment health report
  from them.
- **Input**: None (operates on current working directory).
- **Output**: Toolchain diagnostics containing rust tools, python tools, js
  tools, vcs tools (each a list of tool statuses) plus the binary path, and a
  doctor result containing language versions, adapter statuses (map of tool
  name to status), and overall health status.
- **Business Rules**:

  - Rust tools: `rustc`, `cargo`, `clippy` (via `cargo clippy`), `rustfmt`
    — all required.
  - Python tools: `python3`, `ruff`, `mypy` — all optional.
  - JS tools: `node`, `eslint` — all optional.
  - VCS tools: `git` (required).
  - Tool status: `OK` (found), `WARN` (optional, not found), `FAIL`
    (required, not found).
  - Version extracted from first line of stdout. Clippy is checked via
    `cargo clippy --version` and reported as "clippy".
  - The doctor report reads the same toolchain diagnostics this capability
    produces and never re-runs the tool checks.
  - Extracts rust, python, and node version from the first tool in each
    category.
  - Builds adapter statuses map from all tool statuses.
  - Health is true only when all required tools have OK status.
  - Issues list is currently empty (issues are embedded in tool statuses).
- **Edge Cases**:

  - No tools installed → health is false, adapter statuses all show FAIL.
  - Partial toolchain (e.g., Python only) → health is false.
  - Tool installed but version command produces no output → version set to
    empty string.
  - Multiple versions installed → only the first found is reported.
- **Error Handling**: No error thrown; failed tool checks return a status
  without crashing, and all information is embedded in the result structures.

---

### FR-Maintenance-002: Project Statistics (stats)

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

### FR-Maintenance-003: Cache Cleanup (clean)

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

### FR-Maintenance-004: Tool Update (update)

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

### FR-Maintenance-005: Security Scan

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

### FR-Maintenance-006: Dependency Report

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

### FR-Maintenance-007: Adapter Health Check

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

### FR-Maintenance-008: Self-Update (binary)

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

---

## Public Interface

### Protocol API

| Method | Input | Output | Error | Event | Description |
|---|---|---|---|---|---|
| `diagnose_toolchain` | — | `ToolchainDiagnostics` | — | — | Diagnose toolchain. |
| `health_check` | — | `HealthCheckResult` | — | — | Health check. |
| `run_security_scan` | &FilePath | `SecurityScanReport` | — | — | Run security scan. |
| `run_dependency_report` | &FilePath | `DependencyReport` | `String` | — | Run dependency report. |
| `stats` | &FilePath | `MaintenanceStatsVO` | — | — | Stats. |
| `clean` | — | `` | — | — | Clean. |
| `update` | — | `` | — | — | Update. |
| `doctor` | — | `DoctorResultVO` | — | — | Doctor. |
| `self_update` | bool | `SelfUpdateResultVO` | — | — | Self update. |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
|---|---|---|---|---|---|
| `execute` | MaintenanceRequest | `MaintenanceResponse` | — | — | Single composite entry point over the feature. |

## Reference

- PRD: [PRD.md](../../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature; this file is specification only.
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)

| Roadmap | {Link to ROADMAP.md} | [{link}](ROADMAP.md) |
