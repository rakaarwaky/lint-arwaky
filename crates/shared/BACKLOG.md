# Shared — BACKLOG

FRD: —
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-10-05

## Current Condition

- Done: `cargo nextest run --workspace --lib --tests` → 0 failures
- In Progress: None
- Blocked: None
- Next Action: —

## Backlog

| ID | Priority | State | Dependencies | Next Action | Updated |
|---:|---|---|---|---|---|
| DT-SHAR-001 | P0 | Done | None | — | 2026-09-29 |
| DT-SHAR-002 | P0 | Done | None | — | 2026-09-29 |
| DT-SHAR-003 | P0 | Done | None | — | 2026-09-29 |
| DT-SHAR-004 | P0 | Done | None | — | 2026-09-29 |
| DT-SHAR-005 | P0 | Done | None | — | 2026-09-29 |
| DT-SHAR-006 | P1 | Done | None | — | 2026-09-29 |
| DT-SHAR-007 | P1 | Done | None | — | 2026-09-29 |
| DT-SHAR-008 | P1 | Done | None | — | 2026-09-29 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| SHAR-02 AES205 edge-set ordering | Gap | `tests/unit_shared_common_report.rs` | `report_delta_is_deterministic` | working-tree |

## Blockers

None.

## Dependencies

None.

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo nextest run --workspace` → 0 failures |
| Docs | Done | [DATA.md](DATA.md) is specification-only; status lives in this file |
| SHAR-02 | In Progress | AES205 edge walk behind `ReportDelta` is non-deterministic; pinned in place |

## Deferred

### SHAR-02: AES205 edge-ordering (Open — in progress)

The kernel's shared cycle-detection helper walks its edge set without a stable
order, so the AES205 (circular import) findings flip-flop by a couple of
violations between runs on identical files. `report` shows this directly: a
progress table moves without any code changing.

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-10-26 | Qualified the Backlog ids to the SHAR data-item scheme for the DESIGN/DATA template redesign; SHAR-02 moved under Deferred | @raka |
| 2026-10-05 | Gained `ReportSnapshot`, `SnapshotStore`, `MemberDelta`, and `ReportDelta` for the `report` command's per-member counts and their comparison. Opened SHAR-02 for the AES205 edge-set ordering the report's delta depends on | @raka |
| 2026-10-01 | Evidence command retargeted: `shared-lint-arwaky` was split into 20 packages, so the suite is verified workspace-wide | @raka |
| 2026-09-29 | Created backlog entry for shared kernel folder | @raka |
