# FRD — external-lint

## System Overview

The external-lint crate is an aggregate bridge to external, industry-standard linters and formatters. It coordinates and executes Cargo Clippy, Rustfmt, cargo-audit, Ruff, Mypy, Bandit, ESLint, Prettier, and tsc on Rust, Python, and JS/TS files. It normalizes their JSON/text reports into the unified lint-arwaky violation format and integrates them into the compliance report. The crate detects which languages are present in the project and only runs relevant adapters.

```
┌─────────────────────────────────────────────────────────────┐
│                the external lint orchestrator                │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │  language     │  │  the adapter │  │  the result      │  │
│  │  detection    │──▶  selector   │──▶  aggregation     │  │
│  │  (FS scan)    │  │  (per lang)  │  │  (join_all)      │  │
│  └──────────────┘  └──────────────┘  └──────────────────┘  │
│                          │                                   │
│         ┌────────────────┼────────────────┐                 │
│         ▼                ▼                ▼                  │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐            │
│  │ Rust       │  │ Python     │  │ JS/TS      │            │
│  │ Adapters:  │  │ Adapters:  │  │ Adapters:  │            │
│  │ clippy     │  │ ruff       │  │ eslint     │            │
│  │ rustfmt    │  │ mypy       │  │ prettier   │            │
│  │ cargo-audit│  │ bandit     │  │ tsc        │            │
│  └────────────┘  └────────────┘  └────────────┘            │
│         │                │                │                  │
│         └────────────────┼────────────────┘                 │
│                          ▼                                   │
│                 ┌──────────────────┐                         │
│                 │ the external lint │                         │
│                 │ executor          │                         │
│                 │ (subprocess I/O)  │                        │
│                 └──────────────────┘                         │
└─────────────────────────────────────────────────────────────┘
```

## Functional Requirements

### FR-001: Detect Project Languages
- **Description**: Recursively scan the target path to determine which languages (Rust, Python, JS/TS) are present.
- **Input**: Target path (file or directory).
- **Output**: Three booleans: has Rust, has Python, has JS.
- **Business Rules**:
  - Single file: check extension directly (`.rs`, `.py`, `.js`, `.ts`, `.jsx`, `.tsx`).
  - Directory: recursively scan, skipping `node_modules/`, `target/`, `.git/`, `.jj/`.
  - Early termination: stop scanning once all three languages are found.
  - Unknown extensions are ignored.
- **Edge Cases**:
  - Empty directory — all booleans false, no adapters selected.
  - Single file path — extension checked without directory scan.
  - Symlinks — followed (standard `read_dir` behavior).
- **Error Handling**: Read failures on individual directories are silently ignored; partial scan results returned.

### FR-002: Select Adapters by Language
- **Description**: Based on detected languages, select the appropriate set of linter adapters to run.
- **Input**: Booleans for has Rust, has Python, has JS.
- **Output**: Adapter name list — ordered list of adapter names.
- **Business Rules**:
  - Rust adapters: `clippy`, `rustfmt`, `cargo-audit`.
  - Python adapters: `ruff`, `mypy`, `bandit`.
  - JS/TS adapters: `eslint`, `prettier`, `tsc`.
  - Adapters are appended in language-group order (Rust → Python → JS).
  - Missing adapters (not in map) are silently skipped.
- **Edge Cases**:
  - No languages detected — empty adapter list, no scans run.
  - All languages detected — all 9 adapters selected.
- **Error Handling**: No error; empty list for no matches.

### FR-003: Execute Adapters Concurrently
- **Description**: Run all selected adapters in parallel, collecting results.
- **Input**: Target path to scan.
- **Output**: Aggregated results from all adapters.
- **Business Rules**:
  - All adapters run concurrently (up to 9 parallel futures).
  - Each adapter receives the same target path.
  - Results are flattened into a single result list.
  - Total capacity pre-computed for list allocation.
- **Edge Cases**:
  - All adapters return empty results — returns empty result list.
  - One adapter crashes — other results still collected.
  - Adapter binary not installed — warning printed, results for that adapter are empty.
- **Error Handling**: Per-adapter errors are caught; "No such file or directory" or "os error 2" → warning about missing tool. Other errors → generic adapter failure warning.

### FR-004: Normalize External Tool Output
- **Description**: Each adapter normalizes its external tool's stdout/JSON output into lint result structs compatible with the unified lint-arwaky format.
- **Input**: Raw output from external linter subprocess (JSON or text).
- **Output**: Result list with normalized violations.
- **Business Rules**:
  - Each adapter implements the linter adapter protocol's scan method.
  - Severity levels are mapped from tool-specific to lint-arwaky severity (CRITICAL, HIGH, MEDIUM, LOW).
  - File paths are canonicalized to absolute paths.
  - Line numbers extracted from tool-specific JSON fields.
- **Edge Cases**:
  - Tool produces invalid JSON — adapter returns empty results with error logged.
  - Tool output contains zero violations — empty result list (not an error).
  - File path in tool output is relative — canonicalized to absolute path.
- **Error Handling**: Parse failures return adapter error or scan error.

### FR-005: Execute Subprocess Commands
- **Description**: Run external linter tools as subprocesses with timeout, stdout/stderr capture, and error mapping.
- **Input**: Command args, working directory, timeout, adapter name.
- **Output**: Result containing stdout, stderr, and return code, or an operation error.
- **Business Rules**:
  - Default timeout: 60 seconds per adapter.
  - Working directory set to the project root for each adapter.
  - Timeout exceeded → process killed, error returned.
  - Command not found → scan error or adapter error returned.
- **Edge Cases**:
  - Subprocess hangs beyond timeout — process terminated by OS.
  - Working directory doesn't exist — command fails with OS error.
  - Adapter name is None for scan operations — error message omits adapter name.
- **Error Handling**: Scan error for scan operations, adapter error for adapter-specific failures.

### FR-006: Resolve JS Tool Paths
- **Description**: For JS/TS tools, prefer local `node_modules/.bin/` binaries over global installations.
- **Input**: Tool name, arguments, working directory.
- **Output**: Resolved command with full path.
- **Business Rules**:
  - Check `node_modules/.bin/<tool>` in working directory first.
  - If local binary exists, use its absolute path.
  - If not, check global PATH via executable path check.
  - If neither found, use bare tool name (will fail at execution time).
  - Working directory resolved by walking up to 10 parent directories looking for config files (`.eslintrc`, `prettier.config`, `tsconfig.json`, etc.).
- **Edge Cases**:
  - Local `node_modules/.bin/` doesn't exist — falls back to global.
  - Multiple config files in parent hierarchy — nearest one wins.
  - Tool name contains path separators — treated as literal path.
- **Error Handling**: Missing tools result in command failure at execution time.

### FR-007: Resolve Cargo Working Directory
- **Description**: For Rust tools (clippy, rustfmt, cargo-audit), find the directory containing `Cargo.toml` or `Cargo.lock`.
- **Input**: Target path.
- **Output**: Resolved working directory.
- **Business Rules**:
  - Walk up directory tree looking for `Cargo.toml` (for clippy/rustfmt) or `Cargo.lock` (for audit).
  - If no `Cargo.toml/lock` found, return a nonexistent sentinel path.
  - Empty path input returns the input path unchanged.
- **Edge Cases**:
  - Monorepo with multiple `Cargo.toml` — nearest ancestor wins.
  - Path is a file — check parent directory first.
- **Error Handling**: Sentinel path causes cargo commands to fail with "manifest not found" (expected behavior).

## Data Model / Entity Relationship

```
External Lint Container (root layer)
├── executor: subprocess executor
├── lint executor: the external lint executor
├── adapters: map of adapter name to linter adapter
│   ├── "clippy"      → Rust linter adapter
│   ├── "rustfmt"     → Rust formatter adapter
│   ├── "cargo-audit" → cargo audit adapter
│   ├── "ruff"        → Ruff adapter
│   ├── "mypy"        → Mypy adapter
│   ├── "bandit"      → Bandit adapter
│   ├── "eslint"      → ESLint adapter
│   ├── "prettier"    → Prettier adapter
│   └── "tsc"         → TypeScript compiler adapter
└── aggregate: the external lint orchestrator

Lint Result List
└── values: list of lint results

Lint Result
├── file: file path
├── line: line count
├── message: string
├── code: string
├── severity: string
└── source: optional string

Response Data
├── stdout: string
├── stderr: string
└── returncode: integer

Linter Operation Error
├── Scan(scan error)
└── Adapter(adapter error)
```

## API Contract

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| the external lint orchestrator scan all | target path | lint result list | Detect languages, select adapters, run all concurrently, aggregate results. |
| the external lint orchestrator adapter names | — | list of strings | List all registered adapter names. |
| the external lint executor exec cmd scan | args, dir, timeout, adapter, path | result | Execute a scan subprocess with error mapping. |
| the external lint executor exec cmd adapter | args, dir, timeout, adapter | result | Execute an adapter subprocess with error mapping. |
| the external lint executor js apply fix | path, tool, fix arg | result | Run JS tool with fix flag. |
| the external lint selector select adapters | has Rust, has Python, has JS | adapter name list | Select adapters based on detected languages. |
| the external lint utility has python files | target path | boolean | Check if path contains Python files. |
| the external lint utility resolve js cmd | executable, args, working dir | pattern list | Resolve JS tool command with local/global fallback. |
| the external lint utility resolve js working dir | target path | file path | Find nearest directory with JS config files. |
| the external lint utility resolve cargo working dir | target path | file path | Find directory containing Cargo.toml. |
| the external lint container aggregate | — | the external lint aggregate | Access the assembled orchestrator. |

## Integration Points

- **Internal**:
  - The linter adapter protocol in the shared crate — protocol interface for all linter adapters.
  - The external lint aggregate in the shared crate — aggregate trait for the orchestrator.
  - The command executor protocol in the shared crate — protocol for subprocess execution.
  - The file handler utility in the shared crate — file system utilities for language detection.
- **External**:
  - `cargo clippy` — Rust idiom, performance, and style linting.
  - `rustfmt --check` — Rust formatting verification.
  - `cargo audit --json` — Rust dependency vulnerability auditing.
  - `ruff check` — Python linting (replacement for flake8/autoflake/isort).
  - `mypy` — Python static type checking.
  - `bandit -r` — Python security vulnerability scanning.
  - `eslint` — JavaScript/TypeScript linting.
  - `prettier --check` — JavaScript/TypeScript formatting verification.
  - `tsc --noEmit` — TypeScript type checking.

## Non-functional Requirements (Detailed)

- **Performance**: All adapters run concurrently; total scan time is bounded by the slowest adapter (typically <30s for medium projects). Language detection scan is O(n) in file count.
- **Memory**: Each adapter's results are collected in list with pre-computed capacity. JSON parsing loads full tool output into memory.
- **Accuracy**: Severity mapping is tool-specific; some tools may not have exact equivalents for lint-arwaky severity levels.

## Test Scenarios / QA Checklist

- [ ] Scan Rust-only project — only clippy, rustfmt, cargo-audit run.
- [ ] Scan Python-only project — only ruff, mypy, bandit run.
- [ ] Scan JS-only project — only eslint, prettier, tsc run.
- [ ] Scan multi-language project — all 9 adapters run.
- [ ] Scan empty directory — no adapters run, empty result list.
- [ ] Adapter binary not installed — warning printed, other adapters continue.
- [ ] Adapter produces JSON output — correctly parsed into lint result.
- [ ] Adapter produces empty output (no violations) — empty result list.
- [ ] All adapters fail — returns empty result list with warnings.
- [ ] Single file path (not directory) — extension checked, only relevant adapters run.
- [ ] JS tool found in node_modules/.bin — local binary used.
- [ ] JS tool not found locally — global PATH fallback used.
- [ ] Cargo.toml found in parent directory — cargo tools use that directory.
- [ ] Timeout exceeded — adapter returns error, other adapters continue.
- [ ] Concurrent execution — all adapters run in parallel (verify with timing).

## Assumptions & Constraints

- External linter tools must be installed in the system PATH or in `node_modules/.bin/` for their respective adapters to produce results.
- Missing tools produce warnings, not errors — the scan continues with available adapters.
- Subprocess timeout defaults to 60 seconds per adapter; configurable per adapter.
- The crate assumes the project root contains appropriate config files for each language's tools (e.g., `.eslintrc`, `Cargo.toml`, `pyproject.toml`).
- JSON parsing of tool output is lenient; malformed output results in empty results rather than crashes.

## Glossary

| Term | Definition |
|------|-----------|
| Adapter | A wrapper around an external linter tool that normalizes its output to the unified format. |
| Language Detection | Filesystem scan to determine which programming languages are present in the project. |
| Canonicalize | Resolve a relative file path to its absolute path. |
| Normalization | Convert tool-specific severity/message/line format to the unified LintResult format. |

## Reference

- PRD: [PRD.md](../../PRD.md)
