# Feature Backlog: File Watch

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-29

## Current Condition

- Done: `cargo test -p file_watch --lib --tests` → 0 failures
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p file_watch --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| FILE-01 | FR-FileWatch-001 | Watch lifecycle — start, subscribe, stop, is_available | P0 | Done | `cargo test -p file_watch --lib --tests` → 0 failures | @raka | None | 2026-09-29 |
| FILE-02 | FR-FileWatch-002 | Filter and deduplicate events — dedup by path, filter lintable extensions | P0 | Done | `cargo test -p file_watch --lib --tests` → 0 failures | @raka | None | 2026-09-29 |
| FILE-03 | FR-FileWatch-003 | Run lint on changed files via injected aggregate | P0 | Done | `cargo test -p file_watch --lib --tests` → 0 failures | @raka | None | 2026-09-29 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Start watcher on existing directory | Automated | `tests/file-watch/` | cargo test -p file_watch | `2026-09-29` |
| Start watcher on non-existent path | Automated | `tests/file-watch/` | cargo test -p file_watch | `2026-09-29` |
| Modify .rs file | Automated | `tests/file-watch/` | cargo test -p file_watch | `2026-09-29` |
| Modify .txt file | Automated | `tests/file-watch/` | cargo test -p file_watch | `2026-09-29` |
| Rapid modifications to same file | Automated | `tests/file-watch/` | cargo test -p file_watch | `2026-09-29` |
| File matching ignore pattern | Automated | `tests/file-watch/` | cargo test -p file_watch | `2026-09-29` |
| Ctrl+C during watch | Automated | `tests/file-watch/` | cargo test -p file_watch | `2026-09-29` |
| Multiple subscribers | Automated | `tests/file-watch/` | cargo test -p file_watch | `2026-09-29` |
| Broadcast channel lagged | Automated | `tests/file-watch/` | cargo test -p file_watch | `2026-09-29` |
| Initial lint on startup | Automated | `tests/file-watch/` | cargo test -p file_watch | `2026-09-29` |
| Recursive watch | Automated | `tests/file-watch/` | cargo test -p file_watch | `2026-09-29` |
| Non-recursive watch | Automated | `tests/file-watch/` | cargo test -p file_watch | `2026-09-29` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p file_watch --lib --tests` → 0 failures |
| Scenario evidence | Done | 12 scenarios mapped; all Automated via `cargo test -p file_watch` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-29 | Restructured FRD from 6 mixed utility/capability FRs to 3 clean business-capability FRs; merged IWatchStartBroadcastShutdown into IWatchLifecycleProtocol, merged ILintableFilter+IEventDedup into IChangeFilterProtocol, promoted ChangeLintHandler to standalone capability | @raka |
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
