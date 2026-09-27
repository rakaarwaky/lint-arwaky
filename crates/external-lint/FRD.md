# FRD — external-lint (v1.12.0)

---

## Reference

- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature; this file is specification only.
- PRD: [PRD.md](../../PRD.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)

## System Overview

The external-lint crate is an aggregate bridge to external, industry-standard linters and formatters. It coordinates and executes Cargo Clippy, Rustfmt, cargo-audit, Ruff, Mypy, Bandit, ESLint, Prettier, and TSC on Rust, Python, and JS/TS files. It normalizes their JSON/text reports into the unified lint-arwaky violation format using **tool-native rule codes** (e.g., `clippy::needless_return`, `ruff::E501`) and integrates them into the compliance report.

The crate also provides **auto-fix** capabilities — each adapter exposes an `apply_fix` method that runs the tool's native fix command (e.g., `cargo clippy --fix`, `ruff check --fix`, `eslint --fix`).

All adapters execute **sequentially** (no threads, no async runtime). Each adapter runs its external tool as a subprocess, captures output, and normalizes results. The entry point is the DI container, which wires all adapters and exposes the aggregate trait.

### Architecture & Data Flow

```mermaid
flowchart TD
    A["Surface / Dispatcher"] -->|input| B["ExternalLintContainer"]
    B --> C["ExternalLintOrchestrator\n(IExternalLintAggregate)"]

    C -->|"select_adapters()"| D["ExternalLintSelector\n(IExternalLintSelectorProtocol)"]
    D -->|"adapter names"| C

    C -->|"scan(path)"| E["9 Adapters\n(ILinterAdapterProtocol)"]
    C -->|"context.ignored_paths\npost-filter"| F["Lint Results"]

    E -->|"Rust adapters\n(ICommandExecutorProtocol\ndirect, 120-180s timeout)"| R["clippy\nrustfmt\ncargo-audit"]
    E -->|"Python adapters\n(StdioClient 60s timeout)"| P["ruff\nmypy\nbandit"]
    E -->|"JS adapters\n(StdioClient 60s timeout)"| J["eslint\nprettier\ntsc"]

    R -->|"subprocess\n(std::process::Command)"| G["result normalization\n(tool-native codes\n+ severity mapping)"]
    P -->|"subprocess"| G
    J -->|"subprocess"| G

    G --> F
    F --> B
    B -->|output| A
```

---

## Functional Requirements

### FR-EXTERNALLINT-001: Detect Project Languages

- **Description**: Determine which languages (Rust, Python, JS/TS) are present in the project using a lightweight extension walk via the filesystem aggregate's `discover_files()`.
- **Input**: Filesystem aggregate reference.
- **Output**: Three booleans: `has_rust`, `has_python`, `has_js`.
- **Business Rules**:

  - Language detection based on file extension:
    - Rust: `.rs`
    - Python: `.py`
    - JS/TS: `.js`, `.jsx`, `.ts`, `.tsx`
  - Symlink behavior follows filesystem crate convention: follow if target is within workspace root, skip otherwise.
- **Edge Cases**:

  - Empty project → all booleans false, no adapters selected.
  - Unknown extensions → ignored.
- **Error Handling**: Filesystem crate handles walk errors internally. Returns partial detection results.

---

### FR-EXTERNALLINT-002: Select Adapters by Language

- **Description**: Based on detected languages, select the appropriate set of linter adapters to run.
- **Input**: Booleans `has_rust`, `has_python`, `has_js`.
- **Output**: Ordered list of adapter names.
- **Business Rules**:

  - Rust adapters: `clippy`, `rustfmt`, `cargo-audit`.
  - Python adapters: `ruff`, `mypy`, `bandit`.
  - JS/TS adapters: `eslint`, `prettier`, `tsc`.
  - Adapters are appended in language-group order (Rust → Python → JS).
  - Hardcoded defaults via `with_defaults()` constructor.
- **Edge Cases**:

  - No languages detected → empty adapter list, no scans run.
  - All languages detected → up to 9 adapters selected.
- **Error Handling**: No error; empty list for no matches.

---

### FR-EXTERNALLINT-003: Execute Scan Across Adapters

- **Description**: Run all selected adapters one after another in adapter-list order, aggregating results. The orchestrator optionally filters adapters by configuration entries and post-filters results by ignored paths.
- **Input**: Target path, optional context (config entries, ignored paths).
- **Output**: Aggregated lint results from all adapters.
- **Business Rules**:

  - Iterates the adapter list in order (Rust → Python → JS groups).
  - Each adapter receives the same target path.
  - Optionally filters adapter list by `context.config_entries` if present.
  - Results are collected into a single `Vec` as they arrive.
  - After collection, filters results against `context.ignored_paths` via the filesystem aggregate's `should_ignore()`.
  - No threads — execution is strictly sequential.
  - Each adapter's scan method invokes subprocess and normalizes output.
- **Edge Cases**:

  - All adapters return empty results → returns empty result list.
  - One adapter fails (panic or error) → remaining adapters still run, failure logged as warning.
  - Adapter binary not installed → warning printed, results for that adapter are empty ("No such file or directory" / "os error 2" detection).
  - Adapter timeout exceeded → error logged, other adapters continue.
- **Error Handling**: Per-adapter errors are caught at the loop boundary. Missing tool detection via OS error string matching. A failing adapter does not stop subsequent adapters.

---

### FR-EXTERNALLINT-004: Apply Auto-Fix via Adapters

- **Description**: Run an external linter tool's native fix command for a specific file, returning whether the fix succeeded.
- **Input**: Tool name, file path, fix argument (e.g., `--fix`, `--write`).
- **Output**: Compliance status indicating success or failure.
- **Business Rules**:

  - JS adapters resolve the working directory and absolute file path, then execute the tool via `js_apply_fix`.
  - Fix-capable adapters and their fix commands:
    - ESLint: `npx eslint <file> --fix`
    - Prettier: `npx prettier <file> --write`
    - Ruff: `ruff check <file> --fix --exit-zero`
    - Clippy: `cargo clippy --fix --allow-dirty --allow-staged`
    - Rustfmt: `cargo fmt` (without `--check`)
  - Non-fixing adapters (MyPy, Bandit, TSC, cargo-audit) return a no-op result.
- **Edge Cases**:

  - Fix command fails → returns failure status, does not crash.
  - Tool not installed → returns failure status.
- **Error Handling**: Subprocess failure mapped to adapter error. Caller decides next step.

---

### FR-EXTERNALLINT-005: Normalize External Tool Output

- **Description**: Each adapter normalizes its external tool's stdout/JSON output into `LintResult` structs compatible with the unified lint-arwaky format. Rule codes use **tool-native identifiers** (e.g., `clippy::needless_return`, `ruff::E501`).
- **Input**: Raw output from external linter subprocess (JSON or text).
- **Output**: Normalized lint results.
- **Business Rules**:

  - **Rule codes**: Use tool-native identifiers prefixed with tool name:
    - Clippy: `clippy::<lint_name>` (e.g., `clippy::needless_return`)
    - Rustfmt: `rustfmt::unformatted`
    - cargo-audit: `cargo-audit::<RUSTSEC-ID>` (e.g., `cargo-audit::RUSTSEC-2021-0124`)
    - Ruff: `ruff::<code>` (e.g., `ruff::E501`, `ruff::F401`)
    - Mypy: `mypy::<error-code>` (e.g., `mypy::arg-type`)
    - Bandit: `bandit::<test-id>` (e.g., `bandit::B101`)
    - ESLint: `eslint::<rule-id>` (e.g., `eslint::no-unused-vars`)
    - Prettier: `prettier::diff`
    - tsc: `tsc::<error-code>` (e.g., `tsc::TS2345`)
  - File paths are canonicalized to absolute paths.
  - Line numbers extracted from tool-specific JSON fields.
  - Severity mapping per tool (see table below).
- **Severity Mapping**:

  | Tool            | Tool Severity/Category          | lint-arwaky Severity |
  | ----------------- | --------------------------------- | ---------------------- |
  | **Clippy**      | `correctness`                   | CRITICAL             |
  |                 | `suspicious`                    | HIGH                 |
  |                 | `perf`                          | HIGH                 |
  |                 | `style`                         | MEDIUM               |
  |                 | `complexity`                    | MEDIUM               |
  |                 | `pedantic`                      | LOW                  |
  |                 | `nursery`                       | LOW                  |
  |                 | `restriction`                   | LOW                  |
  | **Rustfmt**     | diff found                      | MEDIUM               |
  | **cargo-audit** | `Critical`                      | CRITICAL             |
  |                 | `High`                          | HIGH                 |
  |                 | `Medium`                        | MEDIUM               |
  |                 | `Low` / `Unknown`               | LOW                  |
  | **Ruff**        | `E999` (syntax error)           | CRITICAL             |
  |                 | `S*` (security, all S-codes)    | CRITICAL             |
  |                 | `F8xx` (undefined name)         | HIGH                 |
  |                 | `B0xx` (bugbear)                | HIGH                 |
  |                 | `F401` (unused import)          | MEDIUM               |
  |                 | `E1xx` (indentation)            | LOW                  |
  |                 | `E5xx` (line length)            | LOW                  |
  |                 | `W2xx` (whitespace)             | LOW                  |
  |                 | default                         | MEDIUM               |
  | **Mypy**        | `error`                         | HIGH                 |
  |                 | `warning`                       | MEDIUM               |
  |                 | `note`                          | LOW                  |
  |                 | `syntax`/`parse` error in message (beyond the table) | CRITICAL |
  | **Bandit**      | HIGH confidence + HIGH severity | CRITICAL             |
  |                 | HIGH severity                   | HIGH                 |
  |                 | MEDIUM severity                 | MEDIUM               |
  |                 | LOW severity                    | LOW                  |
  | **ESLint**      | severity 2 (error)              | HIGH                 |
  |                 | severity 1 (warning)            | MEDIUM               |
  | **Prettier**    | diff found                      | MEDIUM               |
  | **tsc**         | error                           | HIGH                 |

  > Note: Ruff severity is determined by the rule **code**, not the tool's severity field.
- **Edge Cases**:

  - Tool produces invalid JSON → adapter returns empty results with error logged.
  - Tool output contains zero violations → empty result list (not an error).
  - File path in tool output is relative → canonicalized to absolute path.
  - Unknown tool severity/category → defaults to MEDIUM.
  - Mypy extension: any error message containing `syntax` or `parse` is escalated to CRITICAL (a parse failure blocks all type checking), beyond the severity table above.
- **Error Handling**: Parse failures return empty results with warning. No crash on malformed output.

---

### FR-EXTERNALLINT-006: Execute Subprocess Commands

- **Description**: Run external linter tools as subprocesses with timeout, stdout/stderr capture, and error mapping.
- **Input**: Command args, working directory (optional), timeout, adapter name.
- **Output**: Subprocess result containing stdout, stderr, and return code.
- **Business Rules**:

  - Uses `std::process::Command` (blocking, thread-safe).
  - Sets `PYTHONUNBUFFERED=1` environment variable for all subprocesses.
  - Default timeout: 60 seconds per adapter for Python and JS tools.
  - Rust adapters (Clippy, Rustfmt, cargo-audit) bypass the standard executor and use the command executor directly with longer timeouts: 180s for Clippy, 120s for Rustfmt and cargo-audit.
  - Working directory set to the resolved project root for each adapter.
  - Timeout exceeded → process killed, error returned.
  - Command not found → error returned.
  - Working directory is optional — if `None`, adapter is skipped with warning.
- **Edge Cases**:

  - Subprocess hangs beyond timeout → process terminated.
  - Working directory doesn't exist → command fails with OS error.
- **Error Handling**: Missing binary mapped to "tool not found" warning. Timeout mapped to error. Other OS errors mapped to generic adapter failure. All errors are per-adapter.

---

### FR-EXTERNALLINT-007: Resolve JS Tool Paths

- **Description**: For JS/TS tools, prefer local `node_modules/.bin/` binaries over global installations.
- **Input**: Tool name, arguments, working directory.
- **Output**: Resolved command with full path.
- **Business Rules**:

  - Check `node_modules/.bin/<tool>` in working directory first.
  - If local binary exists, use its absolute path.
  - If not, fall back to global PATH resolution.
  - Working directory resolved by walking up to 10 parent directories looking for config files (`.eslintrc.*`, `prettier.config.*`, `tsconfig.json`, `package.json`).
  - Nearest config file wins.
- **Edge Cases**:

  - Local `node_modules/.bin/` doesn't exist → falls back to global.
  - Multiple config files in parent hierarchy → nearest one wins.
  - No config file found in 10 levels → use original working directory.
- **Error Handling**: Missing tools result in error at execution time.

---

### FR-EXTERNALLINT-008: Resolve Cargo Working Directory

- **Description**: For Rust tools (clippy, rustfmt, cargo-audit), find the directory containing `Cargo.toml` or `Cargo.lock`.
- **Input**: Target path.
- **Output**: Resolved working directory, or none if not found.
- **Business Rules**:

  - Walk up directory tree looking for `Cargo.toml` (for clippy/rustfmt) or `Cargo.lock` (for cargo-audit).
  - If found → return the directory.
  - If not found → return none. Caller skips adapter with warning.
- **Edge Cases**:

  - Monorepo with multiple `Cargo.toml` → nearest ancestor wins.
  - Path is a file → check parent directory first.
- **Error Handling**: None return causes caller to skip adapter with warning.

---

## API Contract

### Protocol API

| Method | Input | Output | Error | Event | Description |
| --- | --- | --- | --- | --- | --- |
| `execute` | `ExternalLintRequest` | `ExternalLintResponse` | Adapter failures carried as warnings inside the response | — | Single composite entry point on the external lint protocol trait, dispatching every request variant to the orchestrator's adapters. |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
| --- | --- | --- | --- | --- | --- |
| `execute` | `ExternalLintRequest` | `ExternalLintResponse` | Adapter failures carried as warnings | — | Dispatch a typed external lint request to the matching capability. |
| `scan_all` | `FilePath` | `LintResultList` | Missing tools warn; scan continues with available adapters | — | Select adapters for the detected languages and run them all sequentially. |
| `scan_all_with_context` | `FilePath`, `ExternalLintContext` | `LintResultList` | Config-restricted adapter names warn; scan continues | — | Run the scan with an explicit language, config, and ignored-path context. |
| `adapter_names` | — | `AdapterNameList` | None | — | Return the names of the adapters available for selection. |

---

## Integration Points

| System | Direction | Purpose | Failure mode |
| --- | --- | --- | --- |
| Linter adapter protocol | in | Interface every linter adapter implements (scan + apply fix) | Missing protocol impl → construction error |
| External lint aggregate | in | Aggregate trait consumed by the dispatcher | Aggregate unavailable → no external findings |
| External lint selector protocol | in | Selects adapters by detected language | Selection failure → empty adapter list |
| Command executor protocol | in | Subprocess execution with timeout and error mapping | Spawn failure → adapter skipped with warning; timeout → adapter error |
| External lint executor protocol | in | Executor with error mapping for the Python and JS adapters | Parse failure → empty results with warning |
| `filesystem` crate | in | Language detection via file extension walk, JS tool path resolution, Cargo working directory resolution, and ignored-path filtering | Tool not found locally → falls back to global PATH; no Cargo manifest in hierarchy → adapter skipped with warning |
| `cargo clippy` | in | Rust idiom, performance, and style linting with auto-fix | Binary not installed → warning printed, other adapters continue |
| `rustfmt --check` / `cargo fmt` | in | Rust formatting verification with auto-fix | Binary not installed → warning printed, other adapters continue |
| `cargo audit --json` | in | Rust dependency vulnerability auditing | Binary not installed → warning printed, other adapters continue |
| `ruff check` | in | Python linting with auto-fix | Binary not installed → warning printed, other adapters continue |
| `mypy` | in | Python static type checking | Binary not installed → warning printed, other adapters continue |
| `bandit -r` | in | Python security vulnerability scanning | Binary not installed → warning printed, other adapters continue |
| `eslint` | in | JavaScript/TypeScript linting with auto-fix | Binary not installed → warning printed, other adapters continue |
| `prettier --check` / `prettier --write` | in | JavaScript/TypeScript formatting verification with auto-fix | Binary not installed → warning printed, other adapters continue |
| `tsc --noEmit` | in | TypeScript type checking | Binary not installed → warning printed, other adapters continue |

---

## Non-functional Requirements

| Metric | Target | Measurement method |
| --- | --- | --- |
| Scan execution model | Adapters run sequentially; total scan time is the sum of adapter times | Time a full multi-language scan and compare to the sum of individual adapter times |
| Language detection cost | O(n) in file count | Measure detection on directories of increasing file count |
| Memory per adapter | Results collected in a Vec; no thread overhead | Peak memory per adapter measured in isolation |
| JSON parsing | Full tool output loaded into memory | Confirm no streaming or truncation on large JSON outputs |
| Unknown severity | Defaults to MEDIUM | Run each tool and verify unknown severities map to MEDIUM |
| Rule code fidelity | Tool-native rule codes preserve full diagnostic information | Run each tool and assert the rule code is unchanged from its native output |
| Subprocess timeout | Default 60s for Python/JS adapters; 120–180s for Rust adapters | Inject a slow adapter and confirm it is killed at the configured limit |

---

## Test Scenarios

- A Rust-only project runs only the clippy, rustfmt, and cargo-audit adapters.
- A Python-only project runs only the ruff, mypy, and bandit adapters.
- A JavaScript-only project runs only the eslint, prettier, and tsc adapters.
- A multi-language project runs all 9 adapters.
- An empty directory runs no adapters and returns an empty result list.
- A single Rust source file path runs only the Rust adapters.
- An adapter whose binary is not installed prints a warning and the other adapters continue.
- An adapter producing JSON output is parsed into the unified lint result.
- An adapter producing empty output yields an empty result list.
- A run where all adapters fail returns an empty result list with warnings.
- A run where one adapter fails still runs the other adapters.
- An adapter exceeding its timeout returns an error and the other adapters continue.
- Adapters execute sequentially, one after another.
- An ESLint fix executes the tool's native fix command.
- A Prettier fix executes the tool's write command.
- A Ruff fix executes the tool's check-and-fix command.
- A Clippy fix executes the tool's fix command.
- A Rustfmt fix executes the tool's format command.
- A fix request for tsc, mypy, bandit, or cargo-audit is a no-op since those tools have no auto-fix capability.
- A Clippy `correctness` lint maps to CRITICAL severity with a `clippy::` prefixed rule name.
- A Clippy `style` lint maps to MEDIUM severity.
- A Ruff `E501` finding (line too long) maps to LOW severity with code `ruff::E501`.
- A Ruff `S105` finding (hardcoded password) maps to CRITICAL severity with code `ruff::S105`.
- An ESLint severity 2 (error) maps to HIGH severity with a `eslint::` prefixed rule name.
- A cargo-audit critical vulnerability maps to CRITICAL severity with a `cargo-audit::RUSTSEC-*` code.
- A tool producing invalid JSON yields empty results and logs a warning.
- A relative file path in tool output is canonicalized to an absolute path.
- A JavaScript tool found in the local binary directory uses the local binary.
- A JavaScript tool not found locally falls back to the global PATH.
- A JavaScript tool not found anywhere raises an error at execution time.
- A Cargo manifest found in a parent directory makes the Cargo tools use that directory.
- No Cargo manifest in the directory hierarchy skips the adapter with a warning.
### Language Detection & Adapter Selection

| # | Scenario | Expected |
| - | - | - |
| 1 | Rust-only project | Only clippy, rustfmt, cargo-audit run |
| 2 | Python-only project | Only ruff, mypy, bandit run |
| 3 | JS-only project | Only eslint, prettier, tsc run |
| 4 | Multi-language project | All 9 adapters run |
| 5 | Empty directory | No adapters run, empty result list |
| 6 | Single .rs file path | Only Rust adapters run |

### Adapter Execution

| # | Scenario | Expected |
| - | - | - |
| 1 | Adapter binary not installed | Warning printed, other adapters continue |
| 2 | Adapter produces JSON output | Correctly parsed into LintResult |
| 3 | Adapter produces empty output | Empty result list |
| 4 | All adapters fail | Returns empty result list with warnings |
| 5 | One adapter fails | Other adapters still run |
| 6 | Timeout exceeded | Adapter returns error, others continue |
| 7 | Sequential execution | Adapters run one after another |

### Auto-Fix

| # | Scenario | Expected |
| - | - | - |
| 1 | ESLint fix | `eslint --fix` executed |
| 2 | Prettier fix | `prettier --write` executed |
| 3 | Ruff fix | `ruff check --fix` executed |
| 4 | Clippy fix | `cargo clippy --fix` executed |
| 5 | Rustfmt fix | `cargo fmt` executed |
| 6 | TSC/MyPy/Bandit/audit fix | No-op (no auto-fix capability) |

### Normalization

| # | Scenario | Expected |
| - | - | - |
| 1 | Clippy `correctness` lint | Severity CRITICAL, code `clippy::<name>` |
| 2 | Clippy `style` lint | Severity MEDIUM |
| 3 | Ruff `E501` (line too long) | Severity LOW, code `ruff::E501` |
| 4 | Ruff `S105` (hardcoded password) | Severity CRITICAL, code `ruff::S105` |
| 5 | ESLint severity 2 (error) | Severity HIGH, code `eslint::<rule>` |
| 6 | cargo-audit critical vulnerability | Severity CRITICAL, code `cargo-audit::RUSTSEC-*` |
| 7 | Tool produces invalid JSON | Empty results, warning logged |
| 8 | Relative file path in tool output | Canonicalized to absolute path |

### Tool Path Resolution

| # | Scenario | Expected |
| - | - | - |
| 1 | JS tool found in node_modules/.bin | Local binary used |
| 2 | JS tool not found locally | Global PATH fallback used |
| 3 | JS tool not found anywhere | Error at execution |
| 4 | Cargo.toml found in parent directory | Cargo tools use that directory |
| 5 | No Cargo.toml in hierarchy | Adapter skipped with warning |


---

## Assumptions & Constraints

- External linter tools must be installed in the system PATH or in `node_modules/.bin/` for their respective adapters to produce results.
- Missing tools produce warnings, not errors — the scan continues with available adapters.
- Subprocess timeout defaults to 60 seconds for Python/JS adapters; Rust adapters use 120-180 seconds.
- The crate assumes the project root contains appropriate config files for each language's tools.
- JSON parsing of tool output is lenient; malformed output results in empty results rather than crashes.
- Language detection uses the filesystem crate's file extension walk.
- Execution is sequential (no threads). No async runtime dependency.
- Rule codes use tool-native identifiers. No new naming scheme is imposed.

---

## Glossary

- **AES**: Agentic Engineering System — the 7-layer coding convention.
- **Adapter**: A wrapper around an external linter tool that normalizes its output to the unified lint result format.
- **Language Detection**: Lightweight filesystem scan to determine which programming languages are present.
- **Canonicalize**: Resolve a relative file path to its absolute path.
- **Normalization**: Convert a tool-specific severity, message, line, and code format to the unified lint result format.
- **Tool-native code**: Rule identifier using the original tool's naming (e.g., `clippy::needless_return`, `ruff::E501`).
- **Subprocess**: External process spawned via `std::process::Command` to run a linter tool.
- **Auto-fix**: Running an external tool's native fix command to automatically correct violations.

---
