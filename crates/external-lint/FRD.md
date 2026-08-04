# FRD — external-lint (v1.1.0)

---

## System Overview

The external-lint crate is an aggregate bridge to external, industry-standard linters and formatters. It coordinates and executes Cargo Clippy, Rustfmt, cargo-audit, Ruff, Mypy, Bandit, ESLint, Prettier, and tsc on Rust, Python, and JS/TS files. It normalizes their JSON/text reports into the unified lint-arwaky violation format using **tool-native rule codes** (e.g., `clippy::needless_return`, `ruff::E501`) and integrates them into the compliance report.

The crate detects which languages are present via the filesystem crate's lightweight extension walk and only runs relevant adapters. All adapters execute **sequentially** (no threads, no async runtime). Each adapter runs its external tool as a subprocess, captures output, and normalizes results.

### Architecture & Data Flow

```mermaid
flowchart TD
    A["Surface"] -->|input| B["external_lint_aggregate"]
    B --> C["orchestrator"]

    C -->|"discover_files()"| D["filesystem_aggregate\n(external crate)"]
    D -->|"(has_rust, has_python, has_js)"| C

    C -->|"Rust adapters"| R["clippy\nrustfmt\ncargo-audit"]
    C -->|"Python adapters"| P["ruff\nmypy\nbandit"]
    C -->|"JS/TS adapters"| J["eslint\nprettier\ntsc"]

    R -->|"subprocess\n(std::process::Command)"| H["result normalization\n(tool-native codes\n+ severity mapping)"]
    P -->|"subprocess"| H
    J -->|"subprocess"| H

    H --> L["Lint Results"]
    L --> C
    C --> B
    B -->|output| A

    style A fill:#e1f5fe,stroke:#0288d1
    style D fill:#fff3e0,stroke:#e65100
    style R fill:#fff3e0,stroke:#e65100
    style P fill:#fff3e0,stroke:#e65100
    style J fill:#fff3e0,stroke:#e65100
    style H fill:#fce4ec,stroke:#c62828
    style L fill:#f3e5f5,stroke:#7b1fa2
```

## Functional Requirements

### FR-001: Detect Project Languages

- **Description**: Determine which languages (Rust, Python, JS/TS) are present in the project using a lightweight extension walk via the filesystem aggregate's `discover_files()`.
- **Input**: Filesystem aggregate reference.
- **Output**: Three booleans: `has_rust`, `has_python`, `has_js`.
- **Business Rules**:

  - Calls `discover_files()` then inspects file extensions locally (extension check only, no file reading or parsing).
  - Language detection based on file extension:
    - Rust: `.rs`
    - Python: `.py`
    - JS/TS: `.js`, `.jsx`, `.ts`, `.tsx`
  - Symlink behavior follows filesystem crate convention: follow if target is within workspace root, skip otherwise.
- **Edge Cases**:

  - Empty project → all booleans false, no adapters selected.
  - Single file → extension checked via filesystem crate.
  - Unknown extensions → ignored.
- **Error Handling**: Filesystem crate handles walk errors internally. Returns partial detection results.

---

### FR-002: Select Adapters by Language

- **Description**: Based on detected languages, select the appropriate set of linter adapters to run.
- **Input**: Booleans `has_rust`, `has_python`, `has_js`. Architecture configuration (adapter enabled/disabled, weights).
- **Output**: Adapter list — ordered list of adapter descriptors (name, weight, timeout).
- **Business Rules**:

  - Rust adapters: `clippy`, `rustfmt`, `cargo-audit`.
  - Python adapters: `ruff`, `mypy`, `bandit`.
  - JS/TS adapters: `eslint`, `prettier`, `tsc`.
  - Adapters are appended in language-group order (Rust → Python → JS).
  - Adapters with `enabled: false` in config are excluded.
  - Missing adapters (not registered) are silently skipped.
  - Each adapter carries its configured `weight` (default 1.0) and `timeout` (default 60s).
- **Edge Cases**:

  - No languages detected → empty adapter list, no scans run.
  - All languages detected → up to 9 adapters selected.
  - Adapter disabled in config → excluded from list.
- **Error Handling**: No error; empty list for no matches.

---

### FR-003: Execute Adapters Sequentially

- **Description**: Run all selected adapters one after another in adapter-list order, aggregating results.
- **Input**: Adapter list, target path.
- **Output**: Aggregated lint results from all adapters.
- **Business Rules**:

  - Iterates the adapter list in order (Rust → Python → JS groups).
  - Each adapter receives the same target path.
  - Results are appended into a single `Vec` as they arrive.
  - No threads — execution is strictly sequential.
  - Each adapter's scan method invokes subprocess (FR-005) and normalizes output (FR-004).
- **Edge Cases**:

  - All adapters return empty results → returns empty result list.
  - One adapter fails (panic or error) → remaining adapters still run, failure logged as warning.
  - Adapter binary not installed → warning printed, results for that adapter are empty.
  - Adapter timeout exceeded → error logged, other adapters continue.
- **Error Handling**: Per-adapter errors are caught at the loop boundary. "No such file or directory" or "os error 2" → warning about missing tool. Other errors → generic adapter failure warning. A failing adapter does not stop subsequent adapters.

### FR-004: Normalize External Tool Output

- **Description**: Each adapter normalizes its external tool's stdout/JSON output into `LintResult` structs compatible with the unified lint-arwaky format. Rule codes use **tool-native identifiers** (e.g., `clippy::needless_return`, `ruff::E501`).
- **Input**: Raw output from external linter subprocess (JSON or text).
- **Output**: Normalized lint results.
- **Business Rules**:

  - Each adapter implements the linter adapter protocol's scan method.
  - **Rule codes**: Use tool-native identifiers prefixed with tool name:
    - Clippy: `clippy::<lint_name>` (e.g., `clippy::needless_return`, `clippy::unwrap_used`)
    - Rustfmt: `rustfmt::diff`
    - cargo-audit: `cargo-audit::<RUSTSEC-ID>` (e.g., `cargo-audit::RUSTSEC-2021-0124`)
    - Ruff: `ruff::<code>` (e.g., `ruff::E501`, `ruff::F401`)
    - Mypy: `mypy::<error-code>` (e.g., `mypy::arg-type`, `mypy::assignment`)
    - Bandit: `bandit::<test-id>` (e.g., `bandit::B101`, `bandit::B602`)
    - ESLint: `eslint::<rule-id>` (e.g., `eslint::no-unused-vars`, `eslint::@typescript-eslint/no-explicit-any`)
    - Prettier: `prettier::diff`
    - tsc: `tsc::<error-code>` (e.g., `tsc::TS2345`, `tsc::TS2322`)
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
  |                 | `S1xx` (security)               | CRITICAL             |
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
  | **Bandit**      | HIGH confidence + HIGH severity | CRITICAL             |
  |                 | HIGH severity                   | HIGH                 |
  |                 | MEDIUM severity                 | MEDIUM               |
  |                 | LOW severity                    | LOW                  |
  | **ESLint**      | severity 2 (error)              | HIGH                 |
  |                 | severity 1 (warning)            | MEDIUM               |
  | **Prettier**    | diff found                      | MEDIUM               |
  | **tsc**         | error                           | HIGH                 |
- **Edge Cases**:

  - Tool produces invalid JSON → adapter returns empty results with error logged.
  - Tool output contains zero violations → empty result list (not an error).
  - File path in tool output is relative → canonicalized to absolute path.
  - Unknown tool severity/category → defaults to MEDIUM.
- **Error Handling**: Parse failures return empty results with warning. No crash on malformed output.

---

### FR-005: Execute Subprocess Commands

- **Description**: Run external linter tools as subprocesses with timeout, stdout/stderr capture, and error mapping.
- **Input**: Command args, working directory (optional), timeout, adapter name.
- **Output**: Subprocess result containing stdout, stderr, and return code.
- **Business Rules**:

  - Uses `std::process::Command` (blocking, thread-safe).
  - Default timeout: 60 seconds per adapter (configurable per adapter in YAML).
  - Working directory set to the resolved project root for each adapter.
  - Timeout exceeded → process killed (`child.kill()`), error returned.
  - Command not found → `OperationError::ToolNotFound` returned.
  - Working directory is optional:
    - `Some(path)` → set as working directory.
    - `None` → skip adapter with warning (no valid working directory found).
- **Edge Cases**:

  - Subprocess hangs beyond timeout → process terminated by `child.kill()`.
  - Working directory doesn't exist → command fails with OS error, mapped to `OperationError`.
  - Adapter name is None for scan operations → error message omits adapter name.
- **Error Handling**: `OperationError::ToolNotFound` for missing binaries. `OperationError::Timeout` for exceeded timeout. `OperationError::IoError` for other OS errors. All errors are per-adapter — one adapter failure does not affect others.

---

### FR-006: Resolve JS Tool Paths

- **Description**: For JS/TS tools, prefer local `node_modules/.bin/` binaries over global installations.
- **Input**: Tool name, arguments, working directory.
- **Output**: Resolved command with full path.
- **Business Rules**:

  - Check `node_modules/.bin/<tool>` in working directory first.
  - If local binary exists, use its absolute path.
  - If not, check global PATH via `which` / executable path check.
  - If neither found, use bare tool name (will fail at execution time with `ToolNotFound`).
  - Working directory resolved by walking up to 10 parent directories looking for config files (`.eslintrc.*`, `prettier.config.*`, `tsconfig.json`, `package.json`).
  - Nearest config file wins.
- **Edge Cases**:

  - Local `node_modules/.bin/` doesn't exist → falls back to global.
  - Multiple config files in parent hierarchy → nearest one wins.
  - Tool name contains path separators → treated as literal path.
  - No config file found in 10 levels → use original working directory.
- **Error Handling**: Missing tools result in `ToolNotFound` error at execution time.

---

### FR-007: Resolve Cargo Working Directory

- **Description**: For Rust tools (clippy, rustfmt, cargo-audit), find the directory containing `Cargo.toml` or `Cargo.lock`.
- **Input**: Target path.
- **Output**: Resolved working directory, or none if not found.
- **Business Rules**:

  - Walk up directory tree looking for `Cargo.toml` (for clippy/rustfmt) or `Cargo.lock` (for audit).
  - If found → return `Some(directory)`.
  - If not found → return `None`. Caller skips adapter with warning.
  - Empty path input → return `None`.
- **Edge Cases**:

  - Monorepo with multiple `Cargo.toml` → nearest ancestor wins.
  - Path is a file → check parent directory first.
  - No `Cargo.toml` in entire hierarchy → `None`, adapter skipped.
- **Error Handling**: `None` return causes caller to skip adapter with warning. No sentinel path.

---

## API Contract


| Operation                       | Input                                    | Output                    | Purpose                                                                    |
| --------------------------------- | ------------------------------------------ | --------------------------- | ---------------------------------------------------------------------------- |
| Full external lint scan         | Filesystem aggregate, target path        | Lint results              | Detect languages, select adapters, run all sequentially, aggregate results |
| Detect project languages        | Filesystem aggregate                     | Language booleans         | Determine which languages (Rust, Python, JS/TS) are present                |
| Select adapters                 | Language booleans, configuration         | Adapter list              | Select adapters based on detected languages and config                     |
| Execute subprocess              | Command args, working directory, timeout | Subprocess result         | Run external tool as subprocess with timeout and error mapping             |
| Resolve JS tool path            | Tool name, arguments, working dir        | Resolved command          | Resolve JS tool with local node_modules/.bin fallback                      |
| Resolve JS working directory    | Target path                              | Working directory         | Find nearest directory with JS config files                                |
| Resolve Cargo working directory | Target path                              | Working directory or none | Find directory containing Cargo.toml, or skip adapter                      |

---

## Integration Points

- **Internal**:

  - Linter adapter protocol — interface for all linter adapters.
  - External lint aggregate — aggregate trait for the orchestrator.
  - Command executor protocol — protocol for subprocess execution.
  - File handler utility — file system utilities.
- **External**:

  - **`filesystem` crate** — provides `discover_files()` for lightweight language detection (extension-only walk, no file reading or parsing).
  - `cargo clippy` — Rust idiom, performance, and style linting.
  - `rustfmt --check` — Rust formatting verification.
  - `cargo audit --json` — Rust dependency vulnerability auditing.
  - `ruff check` — Python linting (replacement for flake8/autoflake/isort).
  - `mypy` — Python static type checking.
  - `bandit -r` — Python security vulnerability scanning.
  - `eslint` — JavaScript/TypeScript linting.
  - `prettier --check` — JavaScript/TypeScript formatting verification.
  - `tsc --noEmit` — TypeScript type checking.

---

## Non-functional Requirements

- **Performance**: All adapters run sequentially; total scan time is the sum of adapter times (typically < 60s per adapter timeout bound for medium projects). Language detection is O(n) in file count.
- **Memory**: Each adapter's results are collected in Vec. JSON parsing loads full tool output into memory. No thread overhead.
- **Accuracy**: Severity mapping is tool-specific (see FR-004 table). Unknown tool severities default to MEDIUM. Tool-native rule codes preserve full diagnostic information from the original tool.
- **Concurrency**: Sequential execution. No threads, no async runtime dependency.
- **Configurability**: All adapters configurable via YAML — enabled/disabled, weight, timeout. See Appendix A.

---

## Test Scenarios / QA Checklist

### Language Detection & Adapter Selection


| # | Scenario                        | Expected                              | Rule       |
| --- | --------------------------------- | --------------------------------------- | ------------ |
| 1 | Rust-only project (.rs files)   | Only clippy, rustfmt, cargo-audit run | FR-001/002 |
| 2 | Python-only project (.py files) | Only ruff, mypy, bandit run           | FR-001/002 |
| 3 | JS-only project (.ts files)     | Only eslint, prettier, tsc run        | FR-001/002 |
| 4 | Multi-language project          | All 9 adapters run                    | FR-001/002 |
| 5 | Empty directory                 | No adapters run, empty result list    | FR-001/002 |
| 6 | Single .rs file path            | Only Rust adapters run                | FR-001/002 |
| 7 | Adapter disabled in config      | Excluded from adapter list            | FR-002     |

### Adapter Execution


| # | Scenario                                      | Expected                                          | Rule   |
| --- | ----------------------------------------------- | --------------------------------------------------- | -------- |
| 1 | Adapter binary not installed                  | Warning printed, other adapters continue          | FR-003 |
| 2 | Adapter produces JSON output                  | Correctly parsed into LintResult                  | FR-004 |
| 3 | Adapter produces empty output (no violations) | Empty result list                                 | FR-004 |
| 4 | All adapters fail                             | Returns empty result list with warnings           | FR-003 |
| 5 | One adapter fails (panic or error)             | Other adapters still run and results collected | FR-003 |
| 6 | Timeout exceeded                              | Adapter returns error, other adapters continue    | FR-005 |
| 7 | Sequential execution                         | Adapters run one after another (verify with timing) | FR-003 |

### Normalization


| # | Scenario                           | Expected                                        | Rule   |
| --- | ------------------------------------ | ------------------------------------------------- | -------- |
| 1 | Clippy`correctness` lint           | Severity CRITICAL, code`clippy::<name>`         | FR-004 |
| 2 | Clippy`style` lint                 | Severity MEDIUM                                 | FR-004 |
| 3 | Ruff`E501` (line too long)         | Severity LOW, code`ruff::E501`                  | FR-004 |
| 4 | Ruff`S105` (hardcoded password)    | Severity CRITICAL, code`ruff::S105`             | FR-004 |
| 5 | ESLint severity 2 (error)          | Severity HIGH, code`eslint::<rule>`             | FR-004 |
| 6 | cargo-audit critical vulnerability | Severity CRITICAL, code`cargo-audit::RUSTSEC-*` | FR-004 |
| 7 | Tool produces invalid JSON         | Empty results, warning logged                   | FR-004 |
| 8 | Relative file path in tool output  | Canonicalized to absolute path                  | FR-004 |

### Tool Path Resolution


| # | Scenario                             | Expected                                   | Rule   |
| --- | -------------------------------------- | -------------------------------------------- | -------- |
| 1 | JS tool found in node_modules/.bin   | Local binary used                          | FR-006 |
| 2 | JS tool not found locally            | Global PATH fallback used                  | FR-006 |
| 3 | JS tool not found anywhere           | ToolNotFound error at execution            | FR-006 |
| 4 | Cargo.toml found in parent directory | Cargo tools use that directory             | FR-007 |
| 5 | No Cargo.toml in hierarchy           | Option::None, adapter skipped with warning | FR-007 |

---

## Assumptions & Constraints

- External linter tools must be installed in the system PATH or in `node_modules/.bin/` for their respective adapters to produce results.
- Missing tools produce warnings, not errors — the scan continues with available adapters.
- Subprocess timeout defaults to 60 seconds per adapter; configurable per adapter via YAML.
- The crate assumes the project root contains appropriate config files for each language's tools (e.g., `.eslintrc`, `Cargo.toml`, `pyproject.toml`).
- JSON parsing of tool output is lenient; malformed output results in empty results rather than crashes.
- Language detection uses the filesystem crate's `discover_files()` method. The filesystem crate must be initialized before external-lint runs.
- Execution is sequential (no threads). No async runtime dependency.
- Rule codes use tool-native identifiers. No new naming scheme is imposed.
- Symlink behavior follows filesystem crate convention: follow if within workspace, skip otherwise.

---

## Glossary


| Term                   | Definition                                                                                              |
| ------------------------ | --------------------------------------------------------------------------------------------------------- |
| **AES**                | Agentic Engineering System — the 7-layer coding convention                                             |
| **Adapter**            | A wrapper around an external linter tool that normalizes its output to the unified LintResult format    |
| **Language Detection** | Lightweight filesystem scan (via filesystem crate) to determine which programming languages are present |
| **Canonicalize**       | Resolve a relative file path to its absolute path                                                       |
| **Normalization**      | Convert tool-specific severity/message/line/code format to the unified LintResult format                |
| **Tool-native code**   | Rule identifier using the original tool's naming (e.g.,`clippy::needless_return`, `ruff::E501`)         |
| **Subprocess**         | External process spawned via`std::process::Command` to run a linter tool                                |
| **Weight**             | Configurable multiplier per adapter for compliance score calculation                                    |
| **Filesystem crate**   | External crate providing language detection via lightweight filesystem scan                             |

---

## Appendix A: YAML Configuration Schema

### External Lint Configuration

```yaml
external_lint:
  enabled: true
  default_timeout: 60          # seconds, per adapter
  adapters:
    - name: "clippy"
      enabled: true
      weight: 1.0
    - name: "rustfmt"
      enabled: true
      weight: 1.0
    - name: "cargo-audit"
      enabled: true
      weight: 1.0
    - name: "ruff"
      enabled: true
      weight: 1.0
    - name: "mypy"
      enabled: true
      weight: 1.0
    - name: "bandit"
      enabled: true
      weight: 1.0
    - name: "eslint"
      enabled: true
      weight: 1.0
    - name: "tsc"
      enabled: true
      weight: 1.0
    - name: "prettier"
      enabled: true
      weight: 1.0
```

### Schema Definition

```yaml
external_lint:
  enabled: <bool>                    # Master switch (default: true)
  default_timeout: <integer>         # Default timeout in seconds (default: 60)
  adapters:
    - name: "<string>"               # Adapter name (clippy, rustfmt, cargo-audit, ruff, mypy, bandit, eslint, prettier, tsc)
      enabled: <bool>                # Enable/disable this adapter (default: true)
      weight: <float>                # Compliance score weight (default: 1.0)
      timeout: <integer>             # Override timeout in seconds (optional, uses default_timeout if absent)
```

---

## Reference

- PRD: [PRD.md](../../PRD.md)
- Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
