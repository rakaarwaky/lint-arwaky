# Shared — BACKLOG

FRD: —
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-10-05

## Current Condition

- Done: `cargo nextest run --workspace --lib --tests` → 0 failures
- In Progress: None
- Blocked: None
- Next Action: —

## Backlog

| ID | Priority | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|
| SHAR-01 | P0 | Done | On Track | None | — | 2026-09-29 |
| SHAR-02 | P1 | Open | Defect | None | Stabilize the AES205 cycle findings at the kernel's edge set | 2026-10-05 |

SHAR-02: the kernel's shared cycle-detection helper walks its edge set without
a stable order, so the AES205 (circular import) findings flip-flop by a
couple of violations between runs on identical files. `report` shows this
directly: a progress table moves without any code changing.

## Scenario Evidence

—

## Blockers

None.

## Dependencies

None.

## Release Readiness

Done. SHAR-02 is a scan-side defect, not a ship blocker: the kernel's new
snapshot types are deterministic; only the AES205 edge walk behind them is not.

## Deferred

—

## Change Log

| Date | Change |
|---|---|
| 2026-10-05 | Gained `ReportSnapshot`, `SnapshotStore`, `MemberDelta`, and `ReportDelta` for the `report` command's per-member counts and their comparison. Opened SHAR-02 for the AES205 edge-set ordering the report's delta depends on |
| 2026-10-01 | Evidence command retargeted: `shared-lint-arwaky` was split into 20 packages, so the suite is verified workspace-wide |
| 2026-09-29 | Created backlog entry for shared kernel folder |
