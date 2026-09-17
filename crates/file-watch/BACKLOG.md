# Feature Backlog: File Watch

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [BACKLOG.md](../../BACKLOG.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p file_watch --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p file_watch --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| FILE-01 | FR-001 | All scenarios verified | P0 | Done | `cargo test -p file_watch --lib --tests` → 0 failures at `29c71083` | @raka | None | 2026-09-17 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Start watcher on existing directory | Automated | `tests/file-watch/` | cargo test -p file_watch | `29c71083` |
| Start watcher on non-existent path | Automated | `tests/file-watch/` | cargo test -p file_watch | `29c71083` |
| Modify .rs file | Automated | `tests/file-watch/` | cargo test -p file_watch | `29c71083` |
| Modify .txt file | Automated | `tests/file-watch/` | cargo test -p file_watch | `29c71083` |
| Rapid modifications to same file | Automated | `tests/file-watch/` | cargo test -p file_watch | `29c71083` |
| File matching ignore pattern | Automated | `tests/file-watch/` | cargo test -p file_watch | `29c71083` |
| Ctrl+C during watch | Automated | `tests/file-watch/` | cargo test -p file_watch | `29c71083` |
| Multiple subscribers | Automated | `tests/file-watch/` | cargo test -p file_watch | `29c71083` |
| Broadcast channel lagged | Automated | `tests/file-watch/` | cargo test -p file_watch | `29c71083` |
| Initial lint on startup | Automated | `tests/file-watch/` | cargo test -p file_watch | `29c71083` |
| Recursive watch | Automated | `tests/file-watch/` | cargo test -p file_watch | `29c71083` |
| Non-recursive watch | Automated | `tests/file-watch/` | cargo test -p file_watch | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p file_watch --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 1 scenarios mapped; all Automated via `cargo test -p file_watch` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
