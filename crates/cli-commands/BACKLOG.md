# Cli Commands — BACKLOG

FRD: —
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-10-05

## Current Condition

- Done: `cargo test -p cli_commands --lib --tests` → 0 failures
- In Progress: None
- Blocked: None
- Next Action: —

## Backlog

| ID | Priority | State | Dependencies | Next Action | Updated |
|---:|---|---|---|---|---|
| DS-CLIC-001 | P0 | Done | None | — | 2026-09-29 |
| DS-CLIC-002 | P0 | Done | None | — | 2026-09-29 |
| DS-CLIC-003 | P0 | Done | None | — | 2026-09-29 |
| DS-CLIC-004 | P1 | Open | None | Stabilize the AES205 count the report command shows as a delta | 2026-10-05 |
| DS-CLIC-005 | P0 | Done | None | — | 2026-09-29 |
| DS-CLIC-006 | P1 | Done | None | — | 2026-09-29 |
| DS-CLIC-007 | P1 | Done | None | — | 2026-09-29 |
| DS-CLIC-008 | P1 | Done | None | — | 2026-09-29 |
| DS-CLIC-009 | P1 | Done | None | — | 2026-09-29 |
| DS-CLIC-010 | P1 | Done | None | — | 2026-09-29 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| DS-CLIC-004 AES205 count non-determinism | Gap | `tests/unit_cli_commands_report.rs` | `report_delta_is_deterministic` | working-tree |

## Blockers

None.

## Dependencies

None.

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo nextest run -p cli-commands-lint-arwaky` → 0 failures |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |
| DS-CLIC-004 | In Progress | AES205 count non-determinism; report stays useful while pinned down |

## Deferred

—

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-10-26 | Qualified the Backlog ids to the CLIC design-item scheme for the DESIGN/DATA template redesign | @raka |
| 2026-10-05 | Shipped the `report` command, the folder-tree scan output with per-violation WHY/FIX, path semantics (a path narrows the report, the scan walks the member), and the JSON/SARIF/JUnit fields matching the text report. Opened DS-CLIC-004 for the AES205 count non-determinism that moves the report's delta | @raka |
| 2026-09-29 | Created backlog entry for CLI commands surface crate | @raka |
