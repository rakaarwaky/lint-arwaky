# TUI — BACKLOG

FRD: —
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-29

## Current Condition

- Done: `cargo test -p tui --lib --tests` → 0 failures
- In Progress: None
- Blocked: None
- Next Action: —

## Backlog

| ID | Priority | State| Dependencies | Next Action | Updated |
|---:|---|---|---|---|---|
| TUIC-01 | P2 | Done | None | — | 2026-09-29  |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| TUI renders scan progress without blocking main thread | Gap | `tests/unit_tui_progress_callback.rs` | `progress_callback_updates_progress_bar` | 2026-09-29 |

## Blockers

None.

## Dependencies

None.

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p tui --lib --tests` → 0 failures |
| Docs | Done | [DESIGN.md](DESIGN.md) defines the TUI surface contract; status lives in this file |

## Deferred

—

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-29 | Created backlog entry for TUI surface crate | @raka |
