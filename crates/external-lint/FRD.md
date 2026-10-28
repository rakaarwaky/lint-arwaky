# FRD — external-lint (v1.14.0)

---

## Feature Overview

The external-lint crate is an aggregate bridge to external, industry-standard linters and formatters. It coordinates and executes Cargo Clippy, Rustfmt, cargo-audit, Ruff, Mypy, Bandit, ESLint, Prettier, TSC, and markdownlint-cli on Rust, Python, JS/TS, and Markdown files. It normalizes their JSON/text reports into the unified lint-arwaky violation format using **tool-native rule codes** (e.g., `clippy::needless_return`, `ruff::E501`, `markdownlint::MD041`) and integrates them into the compliance report.

The crate also provides **auto-fix** capabilities — each adapter exposes an `apply_fix` method that runs the tool's native fix command (e.g., `cargo clippy --fix`, `ruff check --fix`, `eslint --fix`).

All adapters execute **sequentially** (no threads, no async runtime). Each adapter runs its external tool as a subprocess, captures output, and normalizes results. The entry point is the DI container, which wires all adapters and exposes the aggregate trait.

The crate does **not** own language detection. Project-level language detection belongs to the `filesystem` aggregate, which serves it as `FilesystemRequest::DetectProjectLanguages` (see [crates/filesystem/FRD.md](../filesystem/FRD.md)). When a caller supplies no pre-computed context, the orchestrator asks the filesystem aggregate which language groups are present and uses the returned flags. It never walks the tree itself.

- **Architecture & Data Flow**

```mermaid
flowchart TD
    A["Surface / Dispatcher"] -->|input| B["ExternalLintContainer"]
    B --> C["ExternalLintOrchestrator\n(IExternalLintAggregate)"]

    C -->|"DetectProjectLanguages\n(no caller context)"| FS["filesystem aggregate"]
    FS -->|"has_rust / has_python\nhas_js / has_markdown"| C

    C -->|"select_adapters()"| D["ExternalLintSelector\n(IExternalLintSelectorProtocol)"]
    D -->|"adapter names"| C

    C -->|"scan(path)"| E["9 Adapters\n(ILinterAdapterProtocol)"]
    C -->|"context.ignored_paths\npost-filter"| F["Lint Results"]

    E -->|"Rust adapters\n(ICommandExecutorProtocol\ndirect, 120-180s timeout)"| R["clippy\nrustfmt\ncargo-audit"]
    E -->|"Python adapters\n(StdioClient 60s timeout)"| P["ruff\nmypy\nbandit"]
    E -->|"JS adapters\n(StdioClient 60s timeout)"| J["eslint\nprettier\ntsc"]
    E -->|"Markdown adapter\n(StdioClient 60s timeout)"| M["markdownlint-cli"]

    R -->|"subprocess\n(std::process::Command)"| G["result normalization\n(tool-native codes\n+ severity mapping)"]
    P -->|"subprocess"| G
    J -->|"subprocess"| G
    M -->|"subprocess"| G

    G --> F
    F --> B
    B -->|output| A
```

---

## Functional Requirements

### FR-ExternalLint-001: Select and Execute Adapters by Language

| Aspect         | Detail |
|----------------|--------|
| Description    | Based on the language flags for a project, select the appropriate set of linter adapters and run them in adapter-list order, aggregating the results. |
| Business Rules | Append adapters in language-group order (Rust → Python → JS → Markdown): Rust `clippy`, `rustfmt`, `cargo-audit`; Python `ruff`, `mypy`, `bandit`; JS/TS `eslint`, `prettier`, `tsc`; Markdown `markdownlint`. Hardcoded defaults via `with_defaults()`. When no caller-supplied context is present, language flags are requested from the `filesystem` aggregate via `DetectProjectLanguages` — this crate performs no extension walk of its own. Iterate the adapter list in that order, each adapter receiving the the same target path. Optionally filter the adapter list by `context.config_entries`, then post-filter results against `context.ignored_paths` via the filesystem aggregate's `should_ignore()`. No threads — execution is strictly sequential; each adapter's scan method invokes subprocess and normalizes output. |
| Edge Cases     | No languages detected → empty adapter list, no scans run. All languages detected → up to 10 adapters selected. All adapters return empty results → empty result list. One adapter fails (panic or error) → remaining adapters still run, failure logged as a warning. Adapter binary not installed → warning printed, results for that adapter empty ("No such file or directory" / "os error 2" detection). Adapter timeout exceeded → error logged, other adapters continue. |
| Capability     | capabilities_external_lint_selector |
| Protocol       | IExternalLintSelectorProtocol |
| Method         | select_adapters |
| Input          | Target path, the booleans `has_rust`, `has_python`, `has_js`, `has_markdown`, and an optional context (config entries, ignored paths). The target path flows through the `ExternalLintRequest` request variants carried by the aggregate entry point (`execute`). |
| Output         | Ordered list of adapter names, and the aggregated lint results from all adapters. |
| Error          | No error on selection; empty list for no matches. Per-adapter errors are caught at the loop boundary; missing tool detection via OS error string matching. A failing adapter does not stop subsequent adapters. |
| Event          | — |
| Test           | `select_adapters(true, false, false, false)` yields `clippy`, `rustfmt`, `cargo-audit` in that order; empty flags yield an empty list. |
| Metric         | 0 unrequested adapters invoked; 0 threads spawned per scan. |

### FR-ExternalLint-002: Apply Auto-Fix via Adapters

| Aspect         | Detail |
|----------------|--------|
| Description    | Run an external linter tool's native fix command for a specific file, returning whether the fix succeeded. |
| Business Rules | JS adapters resolve the working directory and absolute file path, then execute the tool via `js_apply_fix`. Fix-capable adapters and their fix commands: ESLint `npx eslint <file> --fix`; Prettier `npx prettier <file> --write`; Ruff `ruff check <file> --fix --exit-zero`; Clippy `cargo clippy --fix --allow-dirty --allow-staged`; Rustfmt `cargo fmt` (without `--check`). Non-fixing adapters (Mypy, Bandit, TSC, cargo-audit) return a no-op result. |
| Edge Cases     | Fix command fails → failure status returned, no crash. Tool not installed → failure status. |
| Capability     | capabilities_js_tool_resolver (executed through capabilities_command_executor) |
| Protocol       | IJsToolResolutionProtocol |
| Method         | js_apply_fix |
| Input          | Tool name, file path, fix argument (e.g., `--fix`, `--write`). |
| Output         | Compliance status indicating success or failure. |
| Error          | Subprocess failure mapped to an adapter error; caller decides next step. |
| Event          | — |
| Test           | `js_apply_fix` against a fix-capable tool reports success after the tool rewrites the file; against Mypy it reports the no-op status. |
| Metric         | 1 subprocess spawn per invoked fix. |

### FR-ExternalLint-003: Normalize External Tool Output

| Aspect         | Detail |
|----------------|--------|
| Description    | Each adapter normalizes its external tool's stdout/JSON output into `LintResult` structs compatible with the unified lint-arwaky format. Rule codes use **tool-native identifiers** (e.g., `clippy::needless_return`, `ruff::E501`). |
| Business Rules | **Rule codes** — tool-native identifiers prefixed with the tool name: Clippy `clippy::<lint_name>` (e.g. `clippy::needless_return`); Rustfmt `rustfmt::unformatted`; cargo-audit `cargo-audit::<RUSTSEC-ID>` (e.g. `cargo-audit::RUSTSEC-2021-0124`); Ruff `ruff::<code>` (e.g. `ruff::E501`, `ruff::F401`); Mypy `mypy::<error-code>` (e.g. `mypy::arg-type`); Bandit `bandit::<test-id>` (e.g. `bandit::B101`); ESLint `eslint::<rule-id>` (e.g. `eslint::no-unused-vars`); Prettier `prettier::diff`; tsc `tsc::<error-code>` (e.g. `tsc::TS2345`); markdownlint `markdownlint::<MD-id>` (e.g. `markdownlint::MD041`). File paths canonicalized to absolute; line numbers extracted from tool-specific JSON fields; severity mapped per tool: Clippy `correctness`→CRITICAL, `suspicious`/`perf`→HIGH, `style`/`complexity`→MEDIUM, `pedantic`/`nursery`/`restriction`→LOW; Rustfmt diff→MEDIUM; cargo-audit `Critical`→CRITICAL, `High`→HIGH, `Medium`→MEDIUM, `Low`/`Unknown`→LOW; Ruff `E999` and all `S*`→CRITICAL, `F8xx`/`B0xx`→HIGH, `F401`→MEDIUM, `E1xx`/`E5xx`/`W2xx`→LOW, default→MEDIUM; Mypy `error`→HIGH, `warning`→MEDIUM, `note`→LOW; Bandit HIGH confidence + HIGH severity→CRITICAL, HIGH→HIGH, MEDIUM→MEDIUM, LOW→LOW; ESLint severity 2→HIGH, 1→MEDIUM; Prettier diff→MEDIUM; tsc error→HIGH; markdownlint any rule violation→MEDIUM. Ruff severity is determined by the rule **code**, not the tool's severity field. |
| Edge Cases     | Tool produces invalid JSON → empty results with error logged. Tool output contains zero violations → empty result list, not an error. File path in tool output is relative → canonicalized to absolute. Unknown tool severity/category → defaults to MEDIUM. Mypy extension: any error message containing `syntax` or `parse` is escalated to CRITICAL (a parse failure blocks all type checking), beyond the severity table above. |
| Capability     | capabilities_output_normalizer |
| Protocol       | INormalizeProtocol |
| Method         | normalize |
| Input          | Raw output from an external linter subprocess (JSON or text). |
| Output         | Normalized lint results. |
| Error          | Parse failures return empty results with a warning; no crash on malformed output. |
| Event          | — |
| Test           | A ruff JSON report with `E501` normalizes to `ruff::E501` LOW; a clippy `correctness` diagnostic normalizes to CRITICAL. |
| Metric         | 0 panics on malformed tool output; 100% of results carry tool-native rule codes. |

### FR-ExternalLint-004: Subprocess Execution

| Aspect         | Detail |
|----------------|--------|
| Description    | Spawn external linter tools as blocking subprocesses, capturing stdout/stderr, enforcing per-adapter timeouts, and mapping spawn/timeout errors. |
| Business Rules | Python/JS adapters run with a 60-second timeout; Rust adapter Clippy with 180 seconds; Rust adapters Rustfmt and cargo-audit with 120 seconds. `PYTHONUNBUFFERED=1` is set on all subprocesses. A spawn failure or timeout is reported as an adapter error; the scan continues with the next adapter. **No retry.** An adapter gets exactly one attempt per scan, for both spawn failures and timeouts, per the Integration resilience decision in `PRD.md` (PR-LA-006): a retried tool run executes against a different tool state and can mask the result the first run would have reported, so a skip-and-log is preferred over a second attempt. The contrasting multi-strategy fallback chain in `crates/git-hooks/FRD.md` is not an inconsistency — every strategy there queries the same trusted local repository, so a fallback still yields a correct file list, whereas no second strategy exists here that would yield a correct lint result. **No overall budget.** Each ceiling applies to one adapter; they sum rather than cap, so a scan with every language present can run up to 180 + 120 + 120 + (7 × 60) ≈ 840 s (~14 minutes) before returning. Callers that need a bound — notably the MCP surface, which has no timeout of its own — must impose it themselves. |
| Edge Cases     | Executable absent from PATH → the adapter is reported unavailable, scan continues. Timeout exceeded → error logged, other adapters continue, no retry. Every adapter present and slow → the call blocks for the summed worst case; no layer above interrupts it. |
| Capability     | capabilities_stdio_client (error mapping in capabilities_command_executor) |
| Protocol       | ICommandExecutorProtocol |
| Method         | exec_cmd_scan / exec_cmd_adapter |
| Input          | Tool name, argument list, target path, and timeout in seconds. |
| Output         | Captured stdout/stderr text plus a completion status (ok, timed out, spawn failure). |
| Error          | Spawn and timeout errors are surfaced as `LinterOperationError`. |
| Event          | — |
| Test           | A tool exceeding its ceiling returns the timed-out status while the remaining adapters still run; a PATH-missing tool returns the unavailable status. |
| Metric         | 0 retries per adapter; total worst-case scan wall time ≤ ~840 s. |

### FR-ExternalLint-005: JS Tool Path Resolution

| Aspect         | Detail |
|----------------|--------|
| Description    | Resolve the binary path for a JS/TS tool, preferring a local `node_modules/.bin/<tool>` installation over a global PATH lookup, and locate the working directory by walking up to 10 parent directories. |
| Business Rules | 1. Check `node_modules/.bin/<tool>` in the resolved working directory. 2. If the local binary exists, use its absolute path. 3. Otherwise fall back to global PATH resolution. 4. Resolve the working directory by walking up to 10 parent directories looking for config files (`.eslintrc.*`, `prettier.config.*`, `tsconfig.json`, `package.json`). |
| Edge Cases     | No local binary and no global PATH entry → tool is reported unavailable. Working directory not found within 10 levels → the scan is skipped for that file. |
| Capability     | capabilities_js_tool_resolver |
| Protocol       | IJsToolResolutionProtocol |
| Method         | resolve_js_cmd / resolve_js_working_dir |
| Input          | Tool name and a starting path. |
| Output         | Absolute path to the resolved binary and the working directory to run it from. |
| Error          | Resolution failures are reported as adapter errors; other adapters continue. |
| Event          | — |
| Test           | A file under `node_modules/.bin`-bearing project resolves to the local binary; a bare file with no config within 10 levels reports the tool unavailable. |
| Metric         | 1 lookup per tool invocation; 0 global installs required for the local-preference path. |

### FR-ExternalLint-006: Rust Working Directory Resolution

| Aspect         | Detail |
|----------------|--------|
| Description    | Find the directory containing `Cargo.toml` or `Cargo.lock` for Rust tool execution, walking up the directory tree. |
| Business Rules | 1. Walk up the directory tree looking for `Cargo.toml` (for Clippy/Rustfmt) or `Cargo.lock` (for cargo-audit). 2. Return the directory if found; the caller skips the adapter with a warning if not found. 3. No directory is created or modified — resolution is read-only. |
| Edge Cases     | No `Cargo.toml` or `Cargo.lock` anywhere in the parent chain → the adapter is skipped with a warning. |
| Capability     | capabilities_cargo_dir_resolver |
| Protocol       | ICargoDirProtocol |
| Method         | resolve_cargo_working_dir |
| Input          | Starting path. |
| Output         | Absolute path to the resolved cargo working directory, or none. |
| Error          | Resolution failure is reported as an adapter error; other adapters continue. |
| Event          | — |
| Test           | A path nested in a cargo project resolves to the directory carrying `Cargo.toml`; a path with no cargo tree reports none. |
| Metric         | 1 upward walk per invocation; 0 files created or modified. |

## Public Interface

| Aspect     | Detail |
|------------|--------|
| Agent      | agent_external_lint_orchestrator (ExternalLintOrchestrator) |
| Aggregate  | IExternalLintAggregate — single composite entry point over the feature |
| Method     | `execute` |
| Request    | ExternalLintRequest: `ScanAll { path }`, `ScanAllWithContext { path, context }`, `AdapterNames` |
| Response   | ExternalLintResponse: `Scan { violations }` or `AdapterNames { names }` |
| Protocol API | `scan` (&FilePath → `LintResultList`); `fix` (&FilePath → `ComplianceStatus`); `exec_cmd_scan` (Vec\<String\>, FilePath, f64, Option\<AdapterName\>, &FilePath → `ResponseData`); `exec_cmd_adapter` (Vec\<String\>, FilePath, f64, AdapterName → `ResponseData`); `js_apply_fix` (&FilePath, &str, &str → `ComplianceStatus`); `select_adapters` (bool, bool, bool, bool → `AdapterNameList`) — all errors `LinterOperationError` where applicable |

### Protocol API

| Method | Input | Output | Error | Event | Description |
|---|---|---|---|---|---|
| `scan` | &FilePath | `LintResultList` | `LinterOperationError` | — | Scan. |
| `fix` | &FilePath | `ComplianceStatus` | `LinterOperationError` | — | Apply fix. |
| `exec_cmd_scan` | Vec<String>, FilePath, f64, Option<AdapterName>, &FilePath | `ResponseData` | `LinterOperationError` | — | Exec cmd scan. |
| `exec_cmd_adapter` | Vec<String>, FilePath, f64, AdapterName | `ResponseData` | `LinterOperationError` | — | Exec cmd adapter. |
| `js_apply_fix` | &FilePath, &str, &str | `ComplianceStatus` | `LinterOperationError` | — | Js apply fix. |
| `select_adapters` | bool, bool, bool, bool | `AdapterNameList` | — | — | Select adapters. |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
|---|---|---|---|---|---|
| `execute` | ExternalLintRequest | `ExternalLintResponse` | — | — | Single composite entry point over the feature. |

## Reference

| Artifact     | Purpose                                                | Link |
|--------------|--------------------------------------------------------|------|
| PRD          | Business goals and scope for external lint adapters    | [PRD.md](../../PRD.md) |
| Backlog      | Real condition for this feature; specification only here | [BACKLOG.md](BACKLOG.md) |
| Roadmap      | Timeline and dependencies                              | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra               | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
