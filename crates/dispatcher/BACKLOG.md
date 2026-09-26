# Feature Backlog: Dispatcher

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [BACKLOG.md](../../BACKLOG.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p dispatcher --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p dispatcher --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| DISP-01 | FR-001 | SCEN-001 Unified Scan — 5 scenarios verified | P0 | Done | `cargo test -p dispatcher --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| DISP-02 | FR-002 | SCEN-002 CI Validation — 4 scenarios verified | P0 | Done | `cargo test -p dispatcher --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| DISP-03 | FR-003 | SCEN-003 Individual Linters — 4 scenarios verified | P0 | Done | `cargo test -p dispatcher --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| DISP-04 | FR-004 | SCEN-004 Auto-Fix — 3 scenarios verified | P0 | Done | `cargo test -p dispatcher --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| DISP-05 | FR-005 | SCEN-005 Git Diff — 3 scenarios verified | P0 | Done | `cargo test -p dispatcher --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| DISP-06 | FR-006 | SCEN-006 Hook Management — 3 scenarios verified | P0 | Done | `cargo test -p dispatcher --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Scan valid project path | Returns violations from all 6 linters | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| Scan non-existent path | Returns`Err("Error: path ... does not exist")… | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| Scan with filter "AES101" | Only AES101 violations returned | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| Scan with invalid member | Returns`Err("no workspace member matching")… | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| Scan empty project | Returns empty Vec | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| CI with high threshold (90) | Pass = true if score >= 90 | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| CI with CRITICAL violation | Pass = false (auto-fail) | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| CI with score below threshold | Pass = false, reasons contains score m… | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| CI with no violations | Pass = true, score = 100 | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| Naming scan on valid project | Returns naming violations | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| Import scan with filter | Only matching violations returned | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| Orphan scan on multi-workspace | Builds unified graph, filters per mem… | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| Role scan via subprocess | Returns violations (or empty on fail) | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| Dry-run mode | after_count = before_count, fixed_count = 0 | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| Execute with fixable violations | fixed_count > 0 | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| Execute with no fixable violations | fixable list is empty, no changes | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| Diff with 3 changed lintable files | 3 files in report, violations per… | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| Diff with non-existent base | Returns git diff error | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| Diff with filter | Only matching files included | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| Install hook | HookReport with success=true | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| Uninstall hook | HookReport with success=true | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |
| Install hook (no .git dir) | HookReport with success=false | Automated | `tests/dispatcher/` | cargo test -p dispatcher | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p dispatcher --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 22 scenarios mapped; all Automated via `cargo test -p dispatcher` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
