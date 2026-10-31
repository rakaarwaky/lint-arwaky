# FRD — external-lint

## Feature Overview

The external-lint feature sits in the feature layer, invoked by the surface CLI and dispatcher. It coordinates external linter and formatter adapters across supported language groups, querying the filesystem aggregate for project language presence when caller context is absent. Each adapter invokes external tool processes sequentially and normalizes tool diagnostic output into unified violation records.

## Functional Requirements

### FR-ExternalLint-001: Select and Execute Adapters by Language

**Description:** The feature shall select the set of external linter adapters that matches the detected project languages and execute each selected adapter sequentially in language-group order, aggregating normalized violations across every invoked adapter.

#### Business Logic
- Adapters shall be selected in deterministic language-group order: Rust (`clippy`, `rustfmt`, `cargo-audit`), Python (`ruff`, `mypy`, `bandit`), JS/TS (`eslint`, `prettier`, `tsc`), and Markdown (`markdownlint`).
- When caller context does not supply language flags, the feature shall query the filesystem aggregate for project language presence without scanning directory trees directly.
- The adapter list shall be executed sequentially without threading, passing the target path to each selected adapter.
- Results shall be filtered against configured rules and ignored path patterns before returning to the caller.

#### Edge Cases
- If no languages are detected, the feature shall return an empty adapter list and run zero scans.
- If all languages are detected, the feature shall select up to ten adapters.
- If an adapter fails during execution, the feature shall log a warning and continue executing subsequent adapters.
- If an adapter binary is missing from the environment, the feature shall log a warning, treat that adapter's results as empty, and continue the scan.

#### Technical Specification
| Aspect | Detail |
|--------|--------|
| **Input** | Target file path, boolean flags (`has_rust`, `has_python`, `has_js`, `has_markdown`), and optional audit context with configuration entries and ignored path patterns. Source: caller scan request. |
| **Output** | Ordered list of selected adapter names and aggregated normalized lint results. Destination: caller. |
| **Protocol** | `IExternalLintSelectorProtocol` |
| **Method** | `select_adapters` |
| **Event** | None |
| **Error** | Missing tool binary surfaces as unavailable adapter; adapter execution failure surfaces as a warning without aborting the scan. |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| Project language flags indicate only Rust is present | `select_adapters` receives `has_rust = true` and all other flags `false` | Adapters `clippy`, `rustfmt`, and `cargo-audit` are selected in that exact order |
| No languages are detected in the project | `select_adapters` receives all flags `false` | An empty adapter list is returned and zero adapter subprocesses are spawned |
| An external tool fails during sequential execution | The scan pipeline executes the selected adapters | The failing adapter logs a warning and remaining adapters complete execution |
| An adapter binary is missing from the environment | The scan executes that adapter | The adapter reports unavailable and subsequent adapters still execute |

#### Metric
| Target Metric | Value |
|---------------|-------|
| Unrequested Adapter Invocations | 0 |
| Concurrency Threads Spawned | 0 |
| Adapter Selection Accuracy | 100% |

### FR-ExternalLint-002: Apply Auto-Fix via Adapters

**Description:** The feature shall run the native fix command of an external linter tool against a specific file and report whether the fix succeeded, using a no-op result for tools that provide no fix command.

#### Business Logic
- Fix-capable adapters shall execute their tool's native rewrite command: ESLint `npx eslint <file> --fix`, Prettier `npx prettier <file> --write`, Ruff `ruff check <file> --fix --exit-zero`, Clippy `cargo clippy --fix --allow-dirty --allow-staged`, and Rustfmt `cargo fmt` without `--check`.
- JS tool fixes shall resolve both the working directory and the absolute file path before executing the fix command.
- Non-fixing adapters (Mypy, Bandit, TSC, and cargo-audit) shall return a no-op status without invoking a subprocess.
- A fix invocation shall report a compliance status indicating success, failure, or no-op to the caller.

#### Edge Cases
- If the fix command exits with a failure status, the feature shall return a failure status without crashing.
- If the target tool is not installed, the feature shall return a failure status.
- If the target tool is non-fixing, the feature shall return a no-op status without running a subprocess.

#### Technical Specification
| Aspect | Detail |
|--------|--------|
| **Input** | Tool name, target file path, and fix argument (`--fix`, `--write`). Source: caller fix request. |
| **Output** | Compliance status indicating success, failure, or no-op. Destination: caller. |
| **Protocol** | `IJsToolResolutionProtocol` |
| **Method** | `js_apply_fix` |
| **Event** | None |
| **Error** | Subprocess failure mapped to an adapter error without crashing; caller decides next step. |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A fix-capable tool rewrites the target file | The fix command is executed | The file is rewritten and compliance status reports success |
| A non-fixing tool such as Mypy is requested | The fix command is executed | Compliance status reports no-op and zero subprocesses run |
| The external fix command exits with an error | The fix command is executed | Compliance status reports failure and the process does not terminate abnormally |

#### Metric
| Target Metric | Value |
|---------------|-------|
| Subprocess Spawns per Invoked Fix | 1 |
| Crash Rate on Failed Fix | 0% |
| Non-Fixing Subprocess Invocations | 0 |

### FR-ExternalLint-003: Normalize External Tool Output

**Description:** The feature shall normalize each external tool's stdout or JSON report into unified violation records that carry tool-native rule codes and mapped severities.

#### Business Logic
- Rule codes shall use tool-native identifiers prefixed with the tool name: Clippy `clippy::<lint_name>`, Rustfmt `rustfmt::unformatted`, cargo-audit `cargo-audit::<RUSTSEC-ID>`, Ruff `ruff::<code>`, Mypy `mypy::<error-code>`, Bandit `bandit::<test-id>`, ESLint `eslint::<rule-id>`, Prettier `prettier::diff`, tsc `tsc::<error-code>`, and markdownlint `markdownlint::<MD-id>`.
- File paths in tool reports shall be canonicalized to absolute paths, and line numbers shall be extracted from tool-specific diagnostic fields.
- Diagnostic severity shall follow the per-tool mapping: Clippy `correctness`→CRITICAL, `suspicious`/`perf`→HIGH, `style`/`complexity`→MEDIUM, `pedantic`/`nursery`/`restriction`→LOW; Rustfmt diff→MEDIUM.
- cargo-audit `Critical`→CRITICAL, `High`→HIGH, `Medium`→MEDIUM, `Low`/`Unknown`→LOW; Ruff `E999` and all `S*`→CRITICAL, `F8xx`/`B0xx`→HIGH, `F401`→MEDIUM, `E1xx`/`E5xx`/`W2xx`→LOW, default→MEDIUM.
- Mypy `error`→HIGH, `warning`→MEDIUM, `note`→LOW; Bandit HIGH confidence with HIGH severity→CRITICAL, HIGH→HIGH, MEDIUM→MEDIUM, LOW→LOW; ESLint severity 2→HIGH, 1→MEDIUM; Prettier diff→MEDIUM; tsc error→HIGH; markdownlint rule violation→MEDIUM.
- Ruff severity shall be determined by the rule code rather than the tool's own severity field.

#### Edge Cases
- If a tool produces invalid JSON or unparseable output, the feature shall return empty results and log a warning.
- If a tool produces zero violations, the feature shall return an empty result list rather than an error.
- If a tool outputs a relative file path, the feature shall canonicalize it to an absolute path.
- If a tool diagnostic carries an unknown severity or category, the feature shall default to MEDIUM.
- If a Mypy error message contains `syntax` or `parse`, the feature shall escalate severity to CRITICAL.

#### Technical Specification
| Aspect | Detail |
|--------|--------|
| **Input** | Raw stdout or JSON output from an external linter subprocess. Source: adapter command execution. |
| **Output** | Normalized `LintResult` violations with tool-native rule codes and mapped severities. Destination: caller. |
| **Protocol** | `INormalizeProtocol` |
| **Method** | `normalize` |
| **Event** | None |
| **Error** | Parse failures return empty results with a warning; unparseable content causes no panic. |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A Ruff JSON report contains rule `E501` | Normalization is executed | Normalized violation carries rule code `ruff::E501` with severity LOW |
| A Clippy diagnostic carries category `correctness` | Normalization is executed | Normalized violation carries severity CRITICAL |
| A tool produces malformed JSON text | Normalization is executed | Empty results are returned and a warning is logged without panic |
| A tool report contains a relative file path | Normalization is executed | Violation file path is canonicalized to an absolute path |
| A Mypy error message contains `syntax` | Normalization is executed | Violation severity is escalated to CRITICAL |

#### Metric
| Target Metric | Value |
|---------------|-------|
| Panics on Malformed Tool Output | 0 |
| Results Carrying Tool-Native Rule Codes | 100% |
| Unknown-Severity Default to MEDIUM | 100% |

### FR-ExternalLint-004: Subprocess Execution

**Description:** The feature shall execute external linter tools as blocking subprocesses, capture their stdout and stderr, enforce a per-adapter timeout, and map spawn and timeout failures into adapter errors.

#### Business Logic
- Python and JS adapters shall execute with a 60-second timeout; Clippy with a 180-second timeout; Rustfmt and cargo-audit with a 120-second timeout.
- `PYTHONUNBUFFERED=1` shall be set on all subprocess environments.
- Subprocess spawn failures and timeouts shall be reported as adapter errors, allowing the scan to continue with the next adapter.
- An adapter shall receive exactly one attempt per scan with no retry for spawn failures or timeouts.
- No overall scan ceiling shall be enforced across adapters; per-adapter timeouts sum sequentially to a worst-case wall time of approximately 840 seconds.
- Callers requiring an aggregate time bound shall enforce the ceiling at the outer invocation layer.

#### Edge Cases
- If an executable is absent from PATH, the feature shall report the adapter as unavailable and proceed with remaining adapters.
- If an adapter exceeds its timeout ceiling, the feature shall log an error, avoid retry, and proceed with remaining adapters.
- If all adapters run slowly, execution shall block for the summed worst case without interruption from intermediate layers.

#### Technical Specification
| Aspect | Detail |
|--------|--------|
| **Input** | Tool name, argument list, target working directory, and timeout in seconds. Source: linter adapter. |
| **Output** | Captured stdout and stderr text with execution completion status. Destination: linter adapter. |
| **Protocol** | `ICommandExecutorProtocol` |
| **Method** | `exec_cmd_scan` / `exec_cmd_adapter` |
| **Event** | None |
| **Error** | Spawn and timeout errors surface as `LinterOperationError` variants. |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| An external tool exceeds its allotted timeout ceiling | The tool process executes | The adapter receives a timed-out status and subsequent adapters still execute |
| An external tool binary is absent from PATH | The tool process is spawned | The adapter receives an unavailable status and subsequent adapters still execute |
| An external tool executes successfully | The tool process finishes | Captured stdout and stderr text are returned with an ok completion status |

#### Metric
| Target Metric | Value |
|---------------|-------|
| Retries per Adapter Execution | 0 |
| Total Scan Worst-Case Wall Time | ≤ 840s |
| Spawn Failure Handling Rate | 100% |

### FR-ExternalLint-005: JS Tool Path Resolution

**Description:** The feature shall resolve the binary path for a JS or TS tool, preferring a local installation under `node_modules/.bin` over a global PATH lookup, and shall locate the working directory by walking up to ten parent directories for a tool configuration file.

#### Business Logic
- The feature shall check `node_modules/.bin/<tool>` in the resolved working directory; if the local binary exists, its absolute path shall be used.
- If no local binary exists, the feature shall fall back to global PATH resolution.
- The feature shall resolve the working directory by walking up to ten parent directories searching for `.eslintrc.*`, `prettier.config.*`, `tsconfig.json`, or `package.json`.

#### Edge Cases
- If neither a local binary nor a global PATH entry exists, the feature shall report the tool as unavailable.
- If no configuration file is located within ten parent directory levels, the feature shall skip the scan for that target file.

#### Technical Specification
| Aspect | Detail |
|--------|--------|
| **Input** | Tool name and target starting path. Source: JS adapter. |
| **Output** | Absolute path to the resolved binary and the directory to execute from. Destination: JS adapter. |
| **Protocol** | `IJsToolResolutionProtocol` |
| **Method** | `resolve_js_cmd` / `resolve_js_working_dir` |
| **Event** | None |
| **Error** | Tool resolution failure surfaces as an adapter error; remaining adapters continue execution. |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| Target file resides in a project with `node_modules/.bin/<tool>` | Binary resolution is executed | The returned command path points to the local binary |
| Target file resides in a project without local `node_modules/.bin` | Binary resolution is executed | The returned command path falls back to the global PATH executable |
| Neither local binary nor global PATH entry exists | Binary resolution is executed | The tool is reported as unavailable |
| No configuration file exists within ten parent directory levels | Working directory resolution is executed | Scan for the target file is skipped with a warning |

#### Metric
| Target Metric | Value |
|---------------|-------|
| Directory Lookups per Tool Invocation | 1 |
| Global Installations Required for Local Binaries | 0 |
| Resolution Fallback Success Rate | 100% |

### FR-ExternalLint-006: Rust Working Directory Resolution

**Description:** The feature shall locate the directory containing `Cargo.toml` or `Cargo.lock` for a Rust tool execution by walking up the directory tree.

#### Business Logic
- The feature shall walk up the directory tree looking for `Cargo.toml` for Clippy and Rustfmt, or `Cargo.lock` for cargo-audit.
- When the manifest file is found, the feature shall return the containing directory; if not found, the adapter shall be skipped with a warning.
- Working directory resolution shall be strictly read-only, never creating or modifying any file or directory.

#### Edge Cases
- If neither `Cargo.toml` nor `Cargo.lock` appears in the parent directory chain, the feature shall skip the adapter with a warning.

#### Technical Specification
| Aspect | Detail |
|--------|--------|
| **Input** | Starting target path. Source: Rust adapter. |
| **Output** | Absolute path to the resolved cargo working directory, or none if not found. Destination: Rust adapter. |
| **Protocol** | `ICargoDirProtocol` |
| **Method** | `resolve_cargo_working_dir` |
| **Event** | None |
| **Error** | Missing cargo manifest surfaces as an adapter error; remaining adapters continue execution. |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| Target path is nested inside a cargo project directory | Cargo working directory resolution is executed | The directory containing `Cargo.toml` is returned |
| Target path has no `Cargo.toml` or `Cargo.lock` in its parent chain | Cargo working directory resolution is executed | None is returned and the adapter logs a skip warning |
| Cargo working directory resolution is performed | The directory tree is inspected | Zero files or directories are created or modified |

#### Metric
| Target Metric | Value |
|---------------|-------|
| Upward Directory Walks per Invocation | 1 |
| Files Created or Modified | 0 |
| Resolution Determinism | 100% |

## Public Interface

| Aspect | Detail |
|--------|--------|
| Agent | `ExternalLintOrchestrator` |
| Aggregate | `IExternalLintAggregate` |
| Method | `execute` |
| Request | `ExternalLintRequest` (`ScanAll`, `ScanAllWithContext`, `AdapterNames`) |
| Response | `ExternalLintResponse` (`Scan`, `AdapterNames`) |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| PRD | Business goals, user stories, and scope for external lint adapters | [PRD.md](../../PRD.md) |
| Backlog | Task tracking, epics, and implementation status for this feature | [BACKLOG.md](./BACKLOG.md) |
| Roadmap | Phasing, timeline, and dependencies across project releases | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and vertical slicing structure | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
