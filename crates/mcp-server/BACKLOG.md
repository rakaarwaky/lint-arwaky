# Mcp Server — BACKLOG

FRD: —
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-10-24

## Current Condition

- Done: `cargo test -p mcp-server-lint-arwaky --lib --tests` → 0 failures
- In Progress: None
- Blocked: None
- Next Action: —

## Backlog

| ID | Priority | State | Dependencies | Next Action | Updated |
|---:|---|---|---|---|---|
| DS-MCPS-001 | P0 | Done | None | — | 2026-10-24 |
| DS-MCPS-002 | P1 | Done | None | — | 2026-10-24 |
| DS-MCPS-003 | P1 | Done | None | — | 2026-10-24 |
| DS-MCPS-004 | P1 | Done | None | — | 2026-10-24 |
| DS-MCPS-005 | P1 | Done | None | — | 2026-10-24 |
| DS-MCPS-006 | P1 | Done | None | — | 2026-10-24 |
| DS-MCPS-007 | P1 | Done | None | — | 2026-10-24 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| DS-MCPS-001 JSON-RPC loop and tool dispatch | Automated | `tests/unit_mcp_server_action.rs` | `dispatch_routes_execute_command` | working-tree |

## Blockers

None.

## Dependencies

None.

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo nextest run -p mcp-server-lint-arwaky` → 0 failures |
| Docs | Done | [DESIGN.md](DESIGN.md) is specification-only; status lives in this file |

## Deferred

—

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-10-26 | Qualified the Backlog ids to the MCPS design-item scheme for the DESIGN/DATA template redesign | @raka |
| 2026-10-24 | Initialized BACKLOG.md for mcp-server surface crate | @raka |
