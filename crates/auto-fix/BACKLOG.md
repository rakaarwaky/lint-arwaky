# Feature Backlog: Auto-Fix

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [BACKLOG.md](../../BACKLOG.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p auto_fix --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p auto_fix --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| AUTO-01 | FR-001 | SCEN-001 - Unused Import Removal — 6 scenarios verified | P0 | Done | `cargo test -p auto_fix --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| AUTO-02 | FR-002 | SCEN-002 - Bypass Fix — 10 scenarios verified | P0 | Done | `cargo test -p auto_fix --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| AUTO-03 | FR-003 | SCEN-003 - Symbol Renaming — 5 scenarios verified | P0 | Done | `cargo test -p auto_fix --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| AUTO-04 | FR-004 | SCEN-004 –FR-005 - Dry-Run & Non-Fixable — 5 scenarios verified | P0 | Done | `cargo test -p auto_fix --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| AUTO-05 | FR-005 | Idempotency & Error Handling — 2 scenarios verified | P0 | Done | `cargo test -p auto_fix --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Unused import at valid line | Removed, `Applied` | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| Line 0 or beyond EOF | `Skipped(line_out_of_bounds)` | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| Non-import line | `Skipped(not_an_import_line)` | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| Multi-line import block | `Skipped(multi_line_import)` | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| File does not exist | `Failed(file_not_found)` | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| JS `= require(` pattern | Detected and removed | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| `unwrap()` on target line | Replaced with `expect("safe")`, `Applied` | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| `#[allow(unused)]` line | Removed entirely, `Applied` | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| `// noqa` comment | Stripped from line, `Applied` | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| `// FIXME: refactor` comment | Stripped from line, `Applied` | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| `panic!("error")` | `Skipped(unsafe_removal)` | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| `todo!()` | `Skipped(unsafe_removal)` | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| `unimplemented!()` | `Skipped(unsafe_removal)` | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| `unwrap_or_default()` | Not modified (safe variant) | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| Missing file | `Failed(file_not_found)` | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| No bypass on target line | `Skipped(no_bypass_pattern)` | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| Symbol rename, 3 occurrences | All replaced, `Applied` + count | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| Symbol already valid snake_case | `Skipped(already_valid)` | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| Symbol not found in file | `Skipped(symbol_not_found)` | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| Missing file | `Failed(file_not_found)` | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| New name is a Rust keyword | `Skipped(keyword_conflict)` | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| Dry-run with fixable violations | Outcomes reported, no files modified | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| Dry-run with no violations | "No automatic fixes applied" | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| Non-fixable violations (AES401) | In manual report | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| AES304 `panic!` skipped | In manual report as unsafe_removal | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| Empty violation list | Empty manual report | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| Second run after fix | No further `Applied` outcomes | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |
| Write failure | `Failed(write_error)` | Automated | `tests/auto-fix/` | cargo test -p auto_fix | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p auto_fix --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 28 scenarios mapped; all Automated via `cargo test -p auto_fix` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
