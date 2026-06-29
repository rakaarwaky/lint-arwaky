# Changelog

## 1.10.79 (2026-06-29)

### Fixed

- **serde_yml → serde_yaml_ng migration**: Replaced deprecated `serde_yml` with `serde_yaml_ng` to fix deserialization warnings and ensure forward compatibility.
- **TUI preview scroll**: Fixed scroll behavior in TUI preview panel that was broken after recent refactor.
- **MCP clippy detection**: Improved detection of `cargo clippy` command in MCP server to correctly identify and route clippy-specific invocations.
- **Orphan cross-workspace detection**: Orphan detector now collects source files without the ignore list for cross-workspace orphan detection, preventing false negatives on files in ignored paths.
- **Ignored tests & mock stubs**: Fixed tests that were silently skipped due to incorrect mock stubs; all TUI executor tests now run correctly.
- **Gate checker paths**: `gates.sh` now uses `check .` instead of `scan crates` to correctly target the self-lint root.
- **AES coverage gate**: Split AES coverage assertion per test-workspace instead of a single combined check, improving debuggability.
- **TEST.md threshold**: Restored unique AES code threshold to 24 per language and merged security scripts.
- **Rust formatting**: Fixed `rustfmt` violation in `lint_executor_tests.rs` (struct literal wrapping in `Ok()` block).

### Performance

- **Parallel import rule checks**: Mandatory, forbidden, and intent import checks now run concurrently via `tokio::join!` instead of sequential execution.
- **File cache for ImportParserAdapter**: Added shared file content cache to avoid redundant I/O across import parsing calls.
- **Early-exit bypass checker**: Bypass detection now exits early on first match instead of scanning all patterns.
- **Shared cache for orphan-detector**: Orphan detection reuses the file cache from import-rules, reducing total file reads during full scans.

### Refactored

- **AES102 violation messages**: Updated naming rule AES102 violation output with `used_suffix` field and generic FIX guidance for clearer remediation instructions.
- **Hooks simplified**: Removed commit message lint gate from pre-commit hook — commit message validation is now handled separately.

### Test

- **Full gate verification**: All quality gates (format, clippy, tests, AES coverage, hook integrity) now verified passing in CI.
- **2528 violations detected**: Test workspaces produce 2528 intentional violations across 24 unique AES codes per language, meeting TEST.md criteria.
- **0 self-lint violations**: Lint Arwaky passes its own checks on its own codebase.

### Changed

- **Build script**: Updated `install.local.sh` with improved build instructions and usage guidance.
- **Documentation**: Fixed section headings in TEST.md for clearer pass/fail criteria.
- **Security scripts**: Merged `hook-integrity-check.sh` and `install-hook-protection.sh` into unified `security.sh`.

---

## 1.10.76 (2026-06-28)

### Added

- **Hook integrity protection**: `scripts/hook-integrity-check.sh` + systemd service prevents `--no-verify` bypass.
- **Test coverage**: Orphan violation files added to test-workspaces for AES501-AES506 (6 new intentional violations).
- **Concurrent linters (#107 P2 #17)**: Async linter groups now run via `tokio::join!` for parallel execution.

### Fixed

- **Default factory (#107 P1 #6)**: `scan()` builds default factory from existing orchestrators when none configured.
- **Git-diff base param (#107 P1 #9)**: `IDiffProtocol::get_changed_files` now accepts and uses `base` parameter.
- **Path canonicalization (#107 P1 #13)**: `check_orphan_single_file` canonicalizes paths before comparison.
- **Orphan detection hoisted (#107 P1 #7)**: Cross-workspace orphan detection runs once instead of N× per member.
- **CI severity counting (#107 P2 #20)**: Single-pass severity counting replaces 4× iteration.
- **Symlink protection (#107 P2 #27)**: File walker now detects symlinks and tracks inodes to prevent loops.
- **Workspace discovery (#107 P2 #18)**: `discover_workspaces` no longer called twice on member miss.
- **`format!("{:?}").to_uppercase()` (#107 P2 #21)**: Replaced with direct match.
- **JUnit pre-allocation (#107 P2 Miss #3)**: `String::with_capacity` prevents reallocation.
- **Redundant FilePath (#107 P2 Miss #2)**: Reused existing `path_obj` instead of constructing new one.
- **Ignore list (#107 P1 Miss #1)**: `collect_all_source_files` now passes `default_ignored_paths()` instead of `&[]`.
- **File collector (#107 P2 #28)**: Added `std::collections::HashSet` import + inode-based symlink loop detection.
- **Commit gate threshold**: Pre-commit hook now requires >=24 unique AES codes per language.

### Performance

- **Concurrent async linters**: All 4 async groups run in parallel via `tokio::join!`.
- **Single-pass CI counting**: One loop instead of 4× `.filter().count()`.
- **Pre-allocated JUnit buffer**: Estimated capacity prevents reallocation.

## 1.10.75 (2026-06-28)

### Fixed

- **TUI scan freeze (#108)**: Scan now runs in a background thread with channel-based progress reporting. TUI remains responsive during scan — keyboard navigation, scrolling, and panel focus all work. Status bar shows live scan progress (phase, file count, violations found).
- **Orphan detector false positives (#104-105)**: Fixed graph resolver — plain `pub mod foo;` declarations now parsed, `mod.rs` mapped by parent directory name, `#[path]` links only referenced file (not entire directory), path-key mismatch eliminated, entry-point matching by basename only.
- **Orphan detector correctness (#104)**: Agent orphan logic fixed (flags only when ALL aggregates uncalled, not ANY), generic impls `impl<T> Trait for Struct` now supported, infrastructure reports AES504 (not AES503), polyglot file collection includes `.py`/`.ts`/`.js`.
- **Import rules bugs (#101)**: Fixed `rsplit('_').next_back()` suffix extraction, aggregate intent filter now excludes `use`/`pub use`/`pub(crate) use` lines, `pub use` and `pub(crate) use` handled in import parser and unused checker, cycle detection rewritten with 3-color DFS algorithm.
- **Code analysis patches (#103)**: Removed fake async/Tokio overhead, pre-read files to avoid double I/O, skip unreadable files, support `[lints.clippy]` sections, brace-depth tracking for static Lazy, hoisted `.to_lowercase()` in bypass checker, u32 string interning in duplication analyzer, `main.rs` skipped from mandatory definition check, tuple struct exclusion, multi-segment bare patterns in file collector.
- **Naming rules consistency (#90-99)**: Inconsistent stem extraction between checkers fixed, AES101 regex check no longer skipped when layer definition missing, path-based ignore patterns now work for nested paths.
- **Config system (#84-89)**: Silent failures in deserialization fixed, workspace detection bounded to 2 levels, TOCTOU race conditions eliminated, config loading and validation logic corrected.
- **CLI exit codes (#107)**: scan/fix/git-diff/init commands now return correct ExitCode on violations/errors, CI threshold magic override removed.

### Performance

- **Regex caching**: All hot-path regexes now use `OnceLock` for one-time compilation (import-rules, naming-rules, orphan-detector, role-rules).
- **Combined regex alternation (#102)**: Per-alias `Regex::new()` replaced with single combined alternation regex in unused symbol detection.
- **String interning (#103)**: Duplication analyzer uses u32 string interning for sliding-window keys, reducing HashMap overhead.
- **Pre-computed lookup maps (#105)**: Workspace crate resolution uses pre-built `crate_name -> src_dir` HashMap instead of `read_dir` in hot loop.
- **Single-pass file reading (#103)**: Graph resolver merges multiple file loops into single pass.
- **Borrow over clone**: Eliminated unnecessary `Vec<String>` clones, `to_string()` calls, and `entry.path().is_dir()` extra syscalls.
- **Path normalization**: Single-pass slash collapse replaces multi-step string operations.

### Changed

- **Skill updates**: `rust-engineer` v1.6.0 and `rust-patterns` updated with 25+ patterns learned from issues #90-#108 (OnceLock, TOCTOU, basename matching, quantifier logic, brace-depth tracking, etc.).

## 1.10.72 (2026-06-27)

### Added

- **Ratatui TUI**: Implemented an interactive terminal user interface (`lint-arwaky-tui`) featuring a file browser, preview panel, real config display (`c` key), and diagnostic logs (`d` key). Full scan is wired to execute checks using all 6 underlying rule orchestrators.
- **TUI Scrolling**: Added preview panel scroll support via PageUp/PageDown keybindings and mouse scroll.
- **MCP Command Execution**: Added a unified command execution interface on the MCP server (powered by the new `rmcp` SDK and `McpContainer`) to run lint scans, CI checks, and maintenance diagnostics.
- **Multi-Format Scan Report**: Expanded the `scan` command to support multiple report formats (JSON, JUnit, SARIF, text) and clean/aggregated outputs for shared workspaces.
- **Scan --member Filter**: Added a `--member` flag to target specific workspace members during a workspace scan.
- **New Lint Rules & Enhancements**:
  - Python and TypeScript contract primitive checks (`AES402`).
  - Detection for workspace-level `clippy::allow` bypass comments in `Cargo.toml`.
  - Added option to toggle/flag orphan checking inside configuration YAML files.
  - Ignored vendor minified assets (e.g. `.min.js`, `.min.css`) in `lint_arwaky.config.rust.yaml` to avoid AES304 violations on third-party libraries.
  - Implemented `watch` subcommand using Linux `inotify` debounce rather than raw polling loops.

### Fixed

- **AES Dummy Checker**: Fixed `function_body_is_dummy` logic to correctly evaluate single-line bodies without false positives.
- **Orphan & Cycle Detectors**: Enhanced cross-layer cycle detection (`AES205`) for `crate::` imports, self-import checks, and entry point detection for `*_entry` files.
- **Import Mapping Guards**: Excluded Rust trait imports from import alias mapping to avoid false positives.
- **Graceful Tool Handling**: Improved resilience when running external linters (e.g. handling empty Ruff outputs gracefully).
- **DI & Decoupling**: Replaced risky `unwrap` and `expect` calls with robust pattern matching throughout the capability and service container layers.
- **Ignore Filter Suffix Globs**: Replaced naive substring match with a deep, segment-equality matching algorithm that properly handles paths like `test-workspaces/**`.

### Changed

- **Binary Renamed**: Renamed the CLI entry command to `lint-arwaky-cli`.
- **SDK Migration**: Migrated the MCP server from legacy infrastructure to the official `rmcp` SDK.
- **Shared Workspace Restructuring**: Moved internal test modules to external `tests/` directories, unified common taxonomy imports, and centralized dependency injection through `CliContainer` and `McpContainer`.
- **Configuration Parsing**: Replaced `serde_yaml` with `serde_yml`.
- Replaced `language-adapters` with the `external-lint` crate structure.
- Hardened capabilities and infrastructure layers by enforcing strict port/protocol contracts.

### Removed

- **Deleted dead crates**: Removed `pipeline-jobs`, `output-report`, `plugin-system`, `lifecycle-state` — unused or replaced by other crates.
- **Removed `pub use` re-exports** from `crates/lib.rs` for deleted crates.
- **Removed unused CLI commands**: `suggest`, `multi-project`, `cancel`, `import`, `export`, `diff`, `report`, `complexity` — stubs or redundant with existing commands.
- **Removed `setup doctor`**: Moved to `maintenance doctor` (top-level subcommand).

### Documentation

- **Updated all docs**: `AGENTS.md`, `ARCHITECTURE.md`, `DEPLOY.md`, `CONTRIBUTING.md`, `PRD.md`, `RULES_AES.md`, `SKILL.md` — aligned crate lists, AES codes, supported commands, and removed references to deleted crates.
- **`SKILL.md`**: Trimmed to only document CLI commands that are actually implemented and supported.

## 1.10.9 (2026-06-11) — Published to crates.io + Linux-Only Installer

### Added

- **Published to crates.io**: `lint_arwaky v1.10.9` now available via `cargo install lint_arwaky`.
- **crates.io badge**: Added to `README.md`.

### Changed

- **`install.remote.sh` rewritten (Linux-only)**: Replaced Python-based `auto-linter` install flow with Rust/Cargo. Script now exits immediately on non-Linux OS. Primary install via `cargo install lint_arwaky`; fallback to pre-built GitHub Release binaries (Linux x86_64 only).
- **`mcp.local.json`**: Updated to use `cargo run --manifest-path ... --release --bin lint-arwaky-mcp` for local development.
- **`release.yml` fixed**: Smoke-test command corrected from `check . --format json` (invalid flag) to `report . --output-format json`. Updated `actions/attest-build-provenance` from `@v1` to `@v2`.
- **All docs updated to v1.10.9**: `README.md`, `DEPLOY.md`, `CONTRIBUTING.md`, `PRD.md`, `TEST.md`. Broken `docs/` folder links replaced with root-level `RULES_AES.md` and `ARCHITECTURE.md`.

---

## 1.11.0 (2026-06-09) — AES Renumbering + Barrel Removal

### Changed

- **AES renumbering (v2.0)**: All 27 active codes reorganized into 4 groups (Layer & Import AES001–AES006, Naming & Structure AES010–AES016, File & Content AES020–AES024, Role Violations AES030–AES038). Old-to-new mapping documented in `RULES_AES.md`.
- **Vertical slicing restructuring**: Replaced 6 layer directories (`taxonomy/`, `contract/`, `capabilities/`, `infrastructure/`, `agent/`, `surfaces/`) with 26 feature/domain folders under `src-rust/`.
- **File naming convention**: Changed from `[domain]_[concept]_[suffix].rs` to `[layer]_[concept]_[suffix].rs`. Layer is now a file prefix, not a directory.
- **All 258 source files moved and renamed** — zero architectural changes, zero new features. Pure structural refactoring.
- **Build verified**: 0 errors, 0 warnings.

### Removed

- **Barrel re-export rules (AES007/AES012/AES013)**: Removed. Layer violations are detected by filename prefix, not barrel structure.
- **MCP Schema rule (AES025)**: Removed as unnecessary.
- **All `pub use` from non-barrel files**: Cleaned up. Direct imports used instead.

## 1.10.2 (2026-06-07) — 31 AES Rules Complete

### Added

- **Full 31/31 AES coverage**: All 31 AES codes (AES001–AES033, AES028/029 reserved) implemented and verified. 30/31 unique codes detected across Rust self-lint, Python, and JS test projects.
- **AES030 capability-method-not-found**: Dispatch integrity check — verifies capability methods exist in dispatch catalog.
- **AES031 single-capability-bottleneck**: Detects when all dispatch routes go to a single capability class.
- **AES032 missing-vo-construction**: Flags capability calls missing required Value Object parameters.
- **AES033 constant-purity**: Ensures `_constant` files contain only `pub const`/`pub static` declarations.
- **Multi-language configs**: `lint_arwaky.config.rust.yaml`, `lint_arwaky.config.python.yaml`, `lint_arwaky.config.javascript.yaml` — per-language architecture enforcement with language-specific bypass patterns and barrel conventions.
- **Docs**: `RULES_AES.md` — full AES rule catalog with multi-condition import matrices, suffix tables, and agent role mandates. `ARCHITECTURE.md` — complete AES architecture reference with Mermaid diagram.

### Changed

- **Architecture docs**: Suffix tables expanded to match full config lists across all languages.
- **Self-lint verified**: 153 violations detected on own codebase (15 AES codes), 0 CRITICAL.

---

## 1.10.1 (2026-06-06) — Rust Reference Implementation

### Changed

- **Full language migration**: Lint Arwaky is now implemented in Rust (edition 2021). The Python v1.9.x codebase is preserved for reference; the active project lives in `src-rust/` and produces two binaries: `lint-arwaky-cli` and `lint-arwaky-mcp`.
- **MCP stack**: Replaced the Python `mcp.server.fastmcp.FastMCP` framework with `mcp-sdk-rs` 0.3.4 speaking JSON-RPC 2.0 over stdin/stdout. Protocol version announced: `2024-11-05`.
- **CLI stack**: Replaced Click with `clap` 4.6.1 derive macros. Subcommand groups are still defined in `src-rust/surfaces/cli_core_command.rs`.
- **Naming convention**: 3-word snake_case filenames with a layer-role suffix (e.g., `architecture_compliance_analyzer.rs`, `lint_score_constant.rs`). The same AES003 rule applies to both `.py` and `.rs` files.

### Added

- **AES033 `constant-purity` rule**: New taxonomy rule. Files ending in `_constant` may contain only `pub const` / `pub static` declarations. `struct`, `enum`, `fn`, `impl`, `mod`, `pub mod`, `pub use` blocks in a `_constant` file trigger AES033 (HIGH). See [RULES_AES.md](RULES_AES.md).
- **5 MCP tools wired through Rust**: `execute_command`, `list_commands`, `commands_schema`, `read_skill_context`, `health_check` — all routed through `src-rust/surfaces/mcp_tools_command.rs`.
- **20+ CLI subcommands**: `check`, `scan`, `fix`, `ci`, `git-diff`, `security`, `duplicates`, `dependencies`, `maintenance doctor`, `setup init|install|mcp-config|hermes`, `adapters`, `config show`, `watch`, `install-hook`, `uninstall-hook`, `version`, `vscode-graph`, `orphan`.
- **Report formats**: `text`, `json`, SARIF 2.1.0 (with `$schema` and `version: 2.1.0`), JUnit XML — all delegated to `ReportFormatterProcessor` (capability layer).
- **AST scanners**: New infrastructure adapters for Rust (`ast_rust_scanner`), Python (`ast_py_scanner`), and JavaScript/TypeScript (`ast_js_scanner`) source parsing.
- **Self-lint target**: `lint-arwaky-cli check .` scans `src-rust/` under the same AES rule engine that is exposed to third-party projects.

### Fixed

- **DI wiring**: `DependencyInjectionContainer::new()` lazy-creates every adapter and exposes them through `Arc<dyn ServiceContainerAggregate>` so surface code can call into Agent via the trait only (AES023).
- **Severity model**: `Severity::score_impact()` now centralizes penalty math; CRITICAL findings fail the run regardless of total score.

---

## 1.9.4 (2026-05-20) — Pragmatic Primitive Policy Refactoring

### Added

- Flexible Primitive Types Allowance: Disabled `no_primitives` check (`no_primitives: false`) for `infrastructure`, `capabilities`, and `surfaces` layers to eliminate unnecessary boxing overhead and simplify third-party integrations (e.g., FastMCP, Click).

### Changed

- Strict Value Object Policy: Kept `no_primitives: true` active strictly for core `contract` and `taxonomy` domain layers to guarantee zero technical leakage.
- Restored original clean files for `mcp_server_lifespan.py`, `mcp_server_validator.py`, `mcp_server_schemas.py`, and `mcp_server_resources.py` and discarded temporary utility bypasses.

## 1.9.0 (2026-05-09) — Score 100/100 & Zero Lint

### Fixed

- All 46 B101 asserts: Replaced with proper `if/raise RuntimeError` guard pattern.
- 11 B404/B603 subprocess: Added `# nosec` on trusted command paths verified via `shutil.which`.
- 10 B110/B112 try/except: Added `logger.debug/warning` before `pass/continue`.
- 1 B108 tempfile: `# nosec` on legitimate temp path reference.
- 8 radon complexity hotspots: Extract method pattern refactoring in 5 files.
- Ruff E402: Import ordering in `maintenance_commands_orchestrator`.
- Ruff F401/F821: Unused imports and undefined variables across 4 files.
- Mypy relative imports: Replaced `taxonomy.X` with direct imports.
- CLI crash: Added missing `http_provider` DI parameter.
- Architecture violations: `sys.modules` hack removed, 5 singletons → lazy factories, 17 `asyncio.run()` → `run_async()` bridge.

### Added

- `contract.async_bridge_aggregate.run_async()` utility for safe event loop handling.
- Proper logging in all bare exception handlers.

### Changed

- Score range restored: negative scores now pass through (core feature).
- UV tool reinstall now required after source changes (`uv tool install --reinstall .`).

### Added

- CLI `import` command: Import config from JSON/YAML file.
- Report delegate: SARIF and JUnit output now delegate to `ReportFormatterProcessor` (capability layer) instead of inline implementation.
- Config import method: `DevCommandsSurface.import_config()`.

### Fixed

- MCP tool name: `get_system_health` → `health_check` to match SKILL.md spec.
- CLI command name: `multi_project` → `multi-project` (hyphen, not underscore).
- SARIF output: Was emitting `model_dump()` JSON instead of SARIF format — now correctly delegates to `ReportFormatterProcessor.to_sarif()`.

### Changed

- Version sync: All documents synced to 1.8.0.

## 1.7.0 (2026-05-06) — Architecture Hardening

### Fixed

- All adapters (ruff, mypy, bandit, radon): `PatternList(value=cmd)` → `PatternList(values=cmd)`. Pydantic field name mismatch caused empty command lists. All external linter adapters now produce violations correctly.
- Path normalization: Phantom root replacement now only applies when path doesn't exist. Prevents double-path concatenation on already-valid absolute paths.
- Radon adapter: Fixed `'str' object has no attribute 'value'` error in scan path handling.

### Added

- Architectural Enforcement: Implemented strict hardening rules via `lint-arwaky.config.python.yaml`.
- Naming Conventions: Enforced 3-word underscore-separated filenames (`word1_word2_word3.py`).
- Class Mandate: Mandatory class definitions for all logic files (except `__init__.py`).
- Layer Hardening: Introduced mandatory contract inheritance and explicit layer-to-contract mappings.
- Suffix Enforcement: Strict suffix checks for all layers (e.g., `_vo`, `_port`, `_orchestrator`).
- Integrity Rules: Forbidden primitives in core layers; must use Value Objects (VOs).
- Agent Specialization: Formalized roles for Container, Manager, Orchestrator, Registry, and Coordinator.

### Changed

- Thresholds: Increased default quality score threshold to 100.0.
- File Limits: Reduced maximum allowed line count per file from 500 to 300.
- Rules: Unified architectural rules into `global`, `internal`, and `external` categories.

## 1.6.9 (2026-04-30)

### Changed

- Architecture: Transitioned from 5-domain to 6-domain architecture by introducing the `contract` layer.
- Configuration: Replaced `layer_map` and `governance_rules` with `layers` and `rules` in `lint-arwaky.config.yaml`.
- Naming: Renamed "Governance" adapter to "Architecture" adapter.

## 1.6.3 (2026-04-30)

### Fixed

- PyPI Publishing: Reverted to token-based publishing to ensure compatibility with existing repository secrets.

## 1.6.2 (2026-04-30)

### Fixed

- Package Metadata: Fixed a critical issue where `pyproject.toml` was truncated, causing PyPI upload failures.

## 1.6.1 (2026-04-30)

### Improved

- Version Alignment: Synced versioning across all metadata files (pyproject.toml, SKILL.md, PRD.md).

## 1.6.0 (2026-04-30) — CI/CD Modernization

### CI/CD & Automation

- Modernized GitHub Workflows — Implemented OIDC (Trusted Publishing) for PyPI security.
- Build Provenance — Added SLSA attestations for supply chain security.
- Dogfooding CI — Refactored CI pipeline to use `auto-lint ci` for self-validation.
- Enhanced Caching — Optimized pip caching in GitHub Actions.

## 1.5.0 (2026-04-13) — Stable Release

### Quality

- 1518 tests passing, 0 failing — all 8 failing tests fixed
- 0 skipped tests — 11 phantom-feature tests removed, 1 flaky test fixed
- 0 warnings — all RuntimeWarning, ResourceWarning eliminated

### Test Fixes (8 failing → 0)

- Wrong class names corrected: `JavaScriptScopeDetector` → `show_enclosing_scope`, `LintingGovernanceAdapter` → `GovernanceAdapter`, `RadonAdapter` → `ComplexityAdapter`, `DependencyVulnAdapter` → `DependencyAdapter`, `DataFlowAnalyzer` → `find_flow`, `ScopeBoundaryAnalyzer` → `show_enclosing_scope`
- `analysis_use_case.execute` mock changed from `MagicMock(return_value=...)` to async function — fixes silent TypeError that prevented code after `await` from executing
- subprocess.run patch targets corrected for CliRunner tests

### Warning Fixes (7 → 0)

- `run_with_retry` in `tracking_job_registry.py` — added `inspect.isawaitable()` check before await
- `test_infrastructure_full.py` — `mock_response.raise_for_status` changed from AsyncMock to MagicMock (sync method)
- `test_git_hooks_manager.py` — unclosed file handle fixed with `with open()`
- `test_final_100_percent.py` — unclosed file handle fixed with `with open()`
- `test_linting_governance_adapter.py` — 5× NamedTemporaryFile leak fixed with `f.close()`
- `test_protocols.py` — 2× unclosed socket warnings suppressed
- `@pytest.mark.filterwarnings` added to 5 tests with cross-test coroutine leak

### Skipped Tests (12 → 0)

- 11 `@pytest.mark.skip("Phantom feature removed")` tests deleted from `test_adapters_python.py`
- 1 `@pytest.mark.skip("Flaky test")` in `test_config_json_provider.py` fixed with `monkeypatch.delenv()` and correct depth

## 1.1.0 (2026-04-13)

### New Features

- Full system health check — `health_check` now reports on components: agent lifecycle, job registry, and filesystem
- Semantic analyzers wired — `SemanticScopeAnalyzer` and `CallChainAnalyzer` integrated into DI container (`container.semantic_analyzers`)
- Multi-project orchestration — moved to agent domain, uses taxonomy VOs (`ProjectResult`, `AggregatedResults`)
- Git diff coordination — surfaces call `container.get_git_diff()` instead of importing infrastructure directly
- Plugin discovery coordination — surfaces call `container.get_discovered_plugins()` and `container.get_custom_adapters()`
- Local transport — Now uses direct execution via StdioClient
- SKILL.md path fixed — `read_skill_context` resolves correct path for MCP server
- PHANTOM_ROOT test fix — conftest.py force-override environment variables for consistent test results
- VS Code mypy settings — `.vscode/settings.json` for proper src/ layout resolution

### Critical Fixes

- Architecture leaks eliminated — 0 cross-layer violations (surfaces↛infra, capabilities↛infra, infra↛agent, capabilities↛agent)
- Dead code wired and functional — lifecycle, pipeline, multi-project, path normalization, stdio transport all operational
- MCP import chain repaired — `_running_jobs` moved to canonical source (`mcp_execute_command.py`)
- Mypy type errors fixed — null-safety for `normalize_path()`, proper `entry_points()` handling, correct return types
- Unused imports removed — 4 Ruff F401 violations cleaned up
- Build artifacts removed — `src/lint-arwaky.egg-info/` deleted, added to `.gitignore`
- Entry point fixed — `auto-lint` now uses `main()` wrapper for proper pip installation

### Cleanup

- `pyre-check` from core dependencies (moved to optional)
- Duplicate wiring container (`wiring_dependency_container.py`)
- Orphaned infrastructure modules (`multi_project.py`, `multi_project_aggregator.py`)

## 1.0.0 (2026-04-12)

### Added

- 6-domain architecture: agent, capabilities, contract, infrastructure, surfaces, taxonomy
- Full value object (VO) system — no bare primitives for typed concepts
- 11 lint adapters: ruff, mypy, bandit, radon, pip-audit, duplicates, trends, eslint, prettier, tsc, architecture
- 5 MCP tools: execute_command, list_commands, read_skill_context, check_status, health_check
- 15+ CLI commands: check, scan, fix, security, duplicates, dependencies, ci, watch, version, adapters, config show, maintenance doctor, setup init/install/mcp-config/hermes, install-hook, uninstall-hook, git-diff, orphan, vscode-graph
- Governance scoring with configurable thresholds
- SARIF and JUnit report formats
- Direct execution transport via Stdio
- Agent pipeline orchestration: receive -> think -> act -> respond
- Job tracking with exponential backoff retry
- Lifecycle state management with health reporting
- Config validation provider with .env + YAML support
- MCP server via FastMCP (`mcp.server.fastmcp.FastMCP`)
- CLI via Click with command groups
- Git pre-commit hook install/uninstall
- File watcher for auto-lint on save
- `.env` and `.env.example` for configuration
- `install.sh` — curl-friendly installer script

### Architecture

- Uses `mcp.server.fastmcp.FastMCP` for MCP server
- Decorator-based tool registration via `@mcp.tool()`
- Tool registry split into modules: mcp_execute_command, mcp_command_catalog, mcp_job_management, mcp_health_check
- DI container in `agent/dependency_injection_container.py`
- Standard execution adapter with retry logic

### Dependencies

- mcp[cli], fastmcp, pydantic, ruff, mypy, click, watchdog, httpx, pyyaml, python-dotenv (core)
- pyre-check (optional)
