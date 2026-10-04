# Cli Commands — BACKLOG

FRD: —
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-10-05

## Current Condition

- Done: `cargo test -p cli_commands --lib --tests` → 0 failures
- In Progress: None
- Blocked: None
- Next Action: —

## Backlog

| ID | Priority | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|
| CLIC-01 | P0 | Done | On Track | None | — | 2026-09-29 |
| CLIC-02 | P1 | Open | Defect | None | Stabilize the AES205 count the report command shows as a delta | 2026-10-05 |

CLIC-02: `report` compares this run's per-member counts against the last run's.
AES205 (circular import) findings flip-flop by a couple of violations between
runs on identical files, so a progress report can move up or down without any
code changing. The delta is the visible symptom of a non-deterministic scan.

## Scenario Evidence

—

## Blockers

None.

## Dependencies

None.

## Release Readiness

Done. CLIC-02 is a scan-side defect, not a ship blocker: the report stays
useful while the AES205 count is pinned down.

## Deferred

—

## Change Log

| Date | Change |
|---|---|
| 2026-10-05 | Shipped the `report` command, the folder-tree scan output with per-violation WHY/FIX, path semantics (a path narrows the report, the scan walks the member), and the JSON/SARIF/JUnit fields matching the text report. Opened CLIC-02 for the AES205 count non-determinism that moves the report's delta |
| 2026-09-29 | Created backlog entry for CLI commands surface crate |
