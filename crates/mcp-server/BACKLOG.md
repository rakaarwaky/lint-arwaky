# Feature Backlog: MCP Server

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [BACKLOG.md](../../BACKLOG.md) — cited here, not restated
Last Updated: 2026-09-17

## Current Condition

- Done: `cargo test -p mcp_server --lib --tests` → 0 failures at `29c71083` (2026-09-17). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p mcp_server --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| MCPS-01 | FR-001 | SCEN-001 - Execute Command — 10 scenarios verified | P0 | Done | `cargo test -p mcp_server --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| MCPS-02 | FR-002 | SCEN-002 - List Commands — 3 scenarios verified | P0 | Done | `cargo test -p mcp_server --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| MCPS-03 | FR-003 | SCEN-003 - Read Skill — 4 scenarios verified | P0 | Done | `cargo test -p mcp_server --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| MCPS-04 | FR-004 | SCEN-004 - Health Check — 3 scenarios verified | P0 | Done | `cargo test -p mcp_server --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| MCPS-05 | FR-005 | SCEN-005 - Get Config — 3 scenarios verified | P0 | Done | `cargo test -p mcp_server --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |
| MCPS-06 | FR-006 | SCEN-006 - Protocol Registration — 2 scenarios verified | P0 | Done | `cargo test -p mcp_server --lib --tests` → 0 failures at `29c71083` (2026-09-17) | @raka | None | 2026-09-17 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| `check`/`scan` returns violations + exit_code | Matches CLI on same fi… | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| `fix` applies real fixes (or dry-run report) | No placeholder success | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| `install-hook` / `uninstall-hook` | Changes hook state like CLI | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| `security` tool missing | exit_code 3 | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| Unknown action | Error + exit_code 2 | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| `watch` action | Explicit`unsupported` + exit_code 2 | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| Files with parse failures | Silently skipped, not counted as violation… | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| Missing path argument | Defaults to "." | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| `version` action | Version info + exit_code 0 | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| `adapters` action | Delegates to health check | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| List without filter | Full command catalog | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| List with domain filter | Filtered subset | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| No matches | Empty commands, total 0 | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| Read full skill | Content returned | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| Read specific section | Section content returned | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| Missing skill | Error + searched paths, exit_code 2 | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| Missing section | Error, exit_code 2 | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| All adapters installed | All adapters available, exit_code 0 | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| Some adapters missing | Correct status per adapter, exit_code 0 | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| All adapters missing | adapters_available 0, exit_code 0 | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| Config file exists | Effective config returned | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| No config file | Embedded defaults + warning, exit_code 0 | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| Invalid path | exit_code 2 | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| MCP tools/list | Exactly 5 tools returned | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |
| Server info | Name, version, protocol version | Automated | `tests/mcp-server/` | cargo test -p mcp_server | `29c71083` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p mcp_server --lib --tests` → 0 failures at `29c71083` (2026-09-17) |
| Scenario evidence | Done | 25 scenarios mapped; all Automated via `cargo test -p mcp_server` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
