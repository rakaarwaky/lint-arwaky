# BACKLOG — Calculator

## Current Condition

Calculator is implemented, tested, and gated. Nothing is blocked.

## Backlog

| ID | Priority | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|
| BLK-001 | P1 | Planned | On Track | — | Document the aggregate boundary | 2026-10-03 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| A valid request resolves | Automated | tests/orchestrator.rs | test_resolves | 08a3f424 |

## Blockers

None.

## Dependencies

| Dependency | Why |
|------------|-----|
| Shared contract crate | Supplies the protocols this feature implements. |

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Build | Ready | Builds on the pinned toolchain. |
| Tests | Ready | Full suite passes. |
| Gates | Ready | `lint-arwaky-cli check .` reports 0 violations. |

## Deferred

Nothing deferred.

## Change Log

| Date | Change |
|------|--------|
| 2026-10-03 | Backlog opened with the contract sections. |
