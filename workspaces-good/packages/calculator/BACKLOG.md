# BACKLOG — Calculator

## Current Condition

Calculator routes a request to the operation feature that owns it and merges
the four operation logs into the history the caller reads.

## Backlog

| ID | Priority | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|
| BLK-CALC-001 | P1 | Planned | On Track | — | Cache the merged history per session | 2026-10-03 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| A request reaches its feature | Automated | tests/e2e_calculator.py | test_e2e_calculator | 6f498ff7 |
| Merging four logs returns one history | Automated | tests/unit_calculator.py | test_unit_calculator | 6f498ff7 |

## Blockers

None.

## Dependencies

| Dependency | Why |
|------------|-----|
| Addition, division, multiplication, subtraction | Each owns one operation. |

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Build | Ready | Builds on the pinned toolchain. |
| Tests | Ready | Seven test types plus a benchmark. |
| Gates | Ready | `lint-arwaky-cli check .` reports 0 violations. |

## Deferred

Nothing deferred.

## Change Log

| Date | Change |
|------|--------|
| 2026-10-03 | Backlog opened with the contract sections. |
