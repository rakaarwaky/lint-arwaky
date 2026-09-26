# Feature Backlog: External Lint

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [BACKLOG.md](../../BACKLOG.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p external_lint --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p external_lint --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| EXTE-01 | FR-001 | Adapter Execution — 7 scenarios verified | P0 | Done | `cargo test -p external_lint --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| EXTE-02 | FR-002 | Auto-Fix — 6 scenarios verified | P0 | Done | `cargo test -p external_lint --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| EXTE-03 | FR-003 | Normalization — 8 scenarios verified | P0 | Done | `cargo test -p external_lint --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| EXTE-04 | FR-004 | Tool Path Resolution — 5 scenarios verified | P0 | Done | `cargo test -p external_lint --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Adapter binary not installed | Warning printed, other adapters continu… | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| Adapter produces JSON output | Correctly parsed into LintResult | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| Adapter produces empty output | Empty result list | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| All adapters fail | Returns empty result list with warnings | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| One adapter fails | Other adapters still run | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| Timeout exceeded | Adapter returns error, others continue | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| Sequential execution | Adapters run one after another | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| ESLint fix | `eslint --fix` executed | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| Prettier fix | `prettier --write` executed | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| Ruff fix | `ruff check --fix` executed | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| Clippy fix | `cargo clippy --fix` executed | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| Rustfmt fix | `cargo fmt` executed | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| TSC/MyPy/Bandit/audit fix | No-op (no auto-fix capability) | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| Clippy `correctness` lint | Severity CRITICAL, code `clippy::<name>` | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| Clippy `style` lint | Severity MEDIUM | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| Ruff `E501` (line too long) | Severity LOW, code `ruff::E501` | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| Ruff `S105` (hardcoded password) | Severity CRITICAL, code `ruff::S105… | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| ESLint severity 2 (error) | Severity HIGH, code `eslint::<rule>` | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| cargo-audit critical vulnerability | Severity CRITICAL, code `cargo-au… | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| Tool produces invalid JSON | Empty results, warning logged | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| Relative file path in tool output | Canonicalized to absolute path | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| JS tool found in node_modules/.bin | Local binary used | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| JS tool not found locally | Global PATH fallback used | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| JS tool not found anywhere | Error at execution | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| Cargo.toml found in parent directory | Cargo tools use that directory | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |
| No Cargo.toml in hierarchy | Adapter skipped with warning | Automated | `tests/external-lint/` | cargo test -p external_lint | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p external_lint --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 26 scenarios mapped; all Automated via `cargo test -p external_lint` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
