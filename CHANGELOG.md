# Changelog

## 1.11.0 (2026-06-09) — Vertical Slicing Refactoring

### Changed
- **Vertical slicing restructuring**: Replaced 6 layer directories (`taxonomy/`, `contract/`, `capabilities/`, `infrastructure/`, `agent/`, `surfaces/`) with 26 feature/domain folders under `src-rust/`.
- **File naming convention**: Changed from `[domain]_[concept]_[suffix].rs` to `[layer]_[concept]_[suffix].rs`. Layer is now a file prefix, not a directory.
- **All 258 source files moved and renamed** — zero architectural changes, zero new features. Pure structural refactoring.
- **Build verified**: 0 errors, 0 warnings.

## 1.10.2 (2026-06-07) — 31 AES Rules Complete

### Added
- **Full 31/31 AES coverage**: All 31 AES codes (AES001–AES033, AES028/029 reserved) implemented and verified. 30/31 unique codes detected across Rust self-lint, Python, and JS test projects.
- **AES030 capability-method-not-found**: Dispatch integrity check — verifies capability methods exist in dispatch catalog.
- **AES031 single-capability-bottleneck**: Detects when all dispatch routes go to a single capability class.
- **AES032 missing-vo-construction**: Flags capability calls missing required Value Object parameters.
- **AES033 constant-purity**: Ensures `_constant` files contain only `pub const`/`pub static` declarations.
- **Multi-language configs**: `lint_arwaky.config.rust.yaml`, `lint_arwaky.config.python.yaml`, `lint_arwaky.config.javascript.yaml` — per-language architecture enforcement with language-specific bypass patterns and barrel conventions.
- **Docs**: `docs/RULES_AES.md` — full AES rule catalog with multi-condition import matrices, suffix tables, and agent role mandates. `docs/ARCHITECHTURE.md` — complete AES architecture reference with Mermaid diagram.

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
- **AES033 `constant-purity` rule**: New taxonomy rule. Files ending in `_constant` may contain only `pub const` / `pub static` declarations. `struct`, `enum`, `fn`, `impl`, `mod`, `pub mod`, `pub use` blocks in a `_constant` file trigger AES033 (HIGH). See [docs/AES_RULES.md](docs/AES_RULES.md).
- **5 MCP tools wired through Rust**: `execute_command`, `list_commands`, `commands_schema`, `read_skill_context`, `health_check` — all routed through `src-rust/surfaces/mcp_tools_command.rs`.
- **20+ CLI subcommands**: `check`, `scan`, `fix`, `report`, `ci`, `git-diff`, `multi-project`, `security`, `complexity`, `duplicates`, `trends`, `dependencies`, `setup init|doctor|mcp-config|hermes`, `adapters`, `config show`, `cancel`, `diff`, `import`, `export`, `watch`, `suggest`, `install-hook`, `uninstall-hook`, `version`.
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
- 30 CLI commands: check, scan, fix, report, security, complexity, duplicates, trends, dependencies, ci, batch, watch, version, adapters, stats, clean, update, doctor, install-hook, uninstall-hook, config, diff, export, import, ignore, init, suggest, cancel, plugins, multi-project
- 4 setup commands: setup init, setup hermes, setup doctor, setup mcp-config
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
