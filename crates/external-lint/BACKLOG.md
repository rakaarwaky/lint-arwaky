# Feature Backlog: External Lint

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-29

## Current Condition

- Done: `cargo nextest run -p external-lint-lint-arwaky --lib --tests` → 86 passed, 0 failures (2026-09-29). Protocol consolidation complete: exactly 3 protocol traits in `shared` (`IAdapterScanProtocol`, `ILinterAdapterProtocol`, `INormalizeProtocol`). Subprocess execution and JS tool fix helpers are stateless free functions in `shared::filesystem::utility_command_execution` and `shared::filesystem::utility_js_fix`. All 10 adapters updated, self-lint clean (0 violations in `external-lint/src` and new shared utility files).
- In Progress: None
- Blocked: None
- Next Action: Ready to merge.

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| EXTE-01 | FR-ExternalLint-001 | Scan Execution — sequential adapter orchestration with post-filtering | P0 | Done | `cargo test -p external-lint-lint-arwaky` → 0 failures (2026-09-29) | @raka | None | 2026-09-29 |
| EXTE-02 | FR-ExternalLint-002 | Auto-Fix — native fix command per adapter (no-op for non-fixing tools) | P0 | Done | `cargo test -p external-lint-lint-arwaky` → 0 failures (2026-09-29) | @raka | None | 2026-09-29 |
| EXTE-03 | FR-ExternalLint-003 | Normalization — tool-native codes + severity mapping into unified format | P0 | Done | `cargo test -p external-lint-lint-arwaky` → 0 failures (2026-09-29) | @raka | None | 2026-09-29 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Rust-only project | Only clippy, rustfmt, cargo-audit run | Automated | `tests/acceptance_adapters_selection.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| Python-only project | Only ruff, mypy, bandit run | Automated | `tests/acceptance_adapters_selection.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| JS-only project | Only eslint, prettier, tsc run | Automated | `tests/acceptance_adapters_selection.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| Multi-language project | All 10 adapters run | Automated | `tests/acceptance_adapters_selection.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| Empty directory | No adapters run, empty result list | Automated | `tests/acceptance_adapters_selection.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| Single .rs file path | Only Rust adapters run | Automated | `tests/acceptance_adapters_selection.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| Adapter binary not installed | Warning printed, other adapters continue | Automated | `tests/e2e_external_lint_flow.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| Adapter produces JSON output | Correctly parsed into LintResult | Automated | `tests/unit_external_lint_md_markdownlint_adapter.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| Adapter produces empty output | Empty result list | Automated | `tests/unit_external_lint_md_markdownlint_adapter.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| All adapters fail | Returns empty result list with warnings | Automated | `tests/e2e_external_lint_flow.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| One adapter fails | Other adapters still run | Automated | `tests/e2e_external_lint_flow.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| Timeout exceeded | Adapter returns error, others continue | Automated | `tests/e2e_external_lint_flow.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| Sequential execution | Adapters run one after another | Automated | `tests/integration_external_lint.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| ESLint fix | `eslint --fix` executed | Automated | `tests/integration_external_lint.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| Prettier fix | `prettier --write` executed | Automated | `tests/integration_external_lint.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| Ruff fix | `ruff check --fix` executed | Automated | `tests/integration_external_lint.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| Clippy fix | `cargo clippy --fix` executed | Automated | `tests/integration_external_lint.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| Rustfmt fix | `cargo fmt` executed | Automated | `tests/integration_external_lint.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| TSC/MyPy/Bandit/audit fix | No-op (no auto-fix capability) | Automated | `tests/integration_external_lint.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| Clippy `correctness` lint | Severity CRITICAL, code `clippy::<name>` | Automated | `tests/unit_external_lint_rs_clippy_adapter.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| Clippy `style` lint | Severity MEDIUM | Automated | `tests/unit_external_lint_rs_clippy_adapter.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| Ruff `E501` (line too long) | Severity LOW, code `ruff::E501` | Automated | `tests/unit_external_lint_py_ruff_adapter.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| Ruff `S105` (hardcoded password) | Severity CRITICAL, code `ruff::S105` | Automated | `tests/unit_external_lint_py_ruff_adapter.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| ESLint severity 2 (error) | Severity HIGH, code `eslint::<rule>` | Automated | `tests/unit_external_lint_md_markdownlint_adapter.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| markdownlint `MD041` | Severity MEDIUM, code `markdownlint::MD041` | Automated | `tests/unit_external_lint_md_markdownlint_adapter.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| Tool produces invalid JSON | Empty results, warning logged | Automated | `tests/unit_external_lint_md_markdownlint_adapter.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |
| Relative file path in tool output | Canonicalized to absolute path | Automated | `tests/unit_external_lint_md_markdownlint_adapter.rs` | cargo test -p external-lint-lint-arwaky | `2026-09-29` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p external-lint-lint-arwaky --lib --tests` → 0 failures (2026-09-29) |
| Scenario evidence | Done | 26 scenarios mapped; all Automated |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
| 2026-09-29 | Rewrote FRD: removed FR-006/007/008 (utilities) | @raka |
| 2026-09-29 | Reduced to 3 FRs / 3 protocols. Removed ILanguageDetectProtocol and IExternalLintSelectorProtocol. Language detection and adapter selection are now internal orchestrator mechanics. | @raka |
| 2026-09-29 | Extracted subprocess execution and JS tool fix utilities as stateless free functions into `shared::filesystem::utility_command_execution` and `shared::filesystem::utility_js_fix`. Removed struct definitions to comply with AES404. All 10 adapters wired, 86 unit/integration/contract/smoke tests passing, self-lint clean (0 violations). | @raka |
