# Mcp Server — BACKLOG

FRD: —
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-11-23

## Progress Overview

- Done: All 7 backlog rows reconciled to Done at HEAD; gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23). Per-row source and test evidence below.
- Done: `cargo test -p mcp-server-lint-arwaky --lib --tests` → 0 failures
- In Progress: None
- Blocked: None
- Next Action: —

## Backlog Status

### DS-MCPS-001

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Outcome Vocabulary; source: `crates/mcp-server/src/surface_mcp_action_command.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-MCPS-002

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Kind; source: `crates/mcp-server/src/lib.rs`, `crates/mcp-server/src/surface_mcp_tool_command.rs`; tests: `crates/mcp-server/tests/acceptance_mcp_server.rs`, `crates/mcp-server/tests/contract_mcp_server.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-MCPS-003

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Request Shape; source: `crates/mcp-server/src/surface_mcp_action_view.rs`, `crates/mcp-server/src/surface_mcp_tool_command.rs`; tests: `crates/dispatcher/tests/mock_common.rs`, `crates/mcp-server/tests/unit_mcp_server_action.rs`, `crates/mcp-server/tests/contract_mcp_server.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-MCPS-004

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Invariants; source: `crates/mcp-server/src/lib.rs`, `crates/mcp-server/src/surface_mcp_tool_command.rs`; tests: `crates/mcp-server/tests/contract_mcp_server.rs`, `crates/mcp-server/tests/unit_mcp_server_tool.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-MCPS-005

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Change Checklist; source: `crates/mcp-server/src/surface_mcp_tool_command.rs`, `crates/mcp-server/src/surface_mcp_action_command.rs`; tests: `crates/mcp-server/tests/unit_mcp_server_action.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-MCPS-006

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | States; source: `crates/mcp-server/src/lib.rs`, `crates/mcp-server/src/surface_mcp_action_view.rs`; tests: `crates/mcp-server/tests/acceptance_mcp_server.rs`, `crates/mcp-server/tests/unit_mcp_server_action.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

### DS-MCPS-007

| Aspect | Detail |
| -------------- | --------------------------------------------- |
| State | Done |
| Done | Error States; source: `crates/mcp-server/src/surface_mcp_action_command.rs`, `crates/mcp-server/src/surface_mcp_tool_command.rs`; tests: `crates/mcp-server/tests/unit_mcp_server_action.rs`, `crates/mcp-server/tests/unit_mcp_server_tool.rs`, `crates/mcp-server/tests/unit_mcp_server_catalog_parity.rs`; Gate: `CARGO_INCREMENTAL=0 cargo nextest run --workspace --lib --tests` → Tests job passed at `e6cb89989215f435141aa68992a71dce44bc243c` (CI run `45261257668`, 2026-11-23).
| In Progress | None |
| Risk | None |
| Updated | 2026-11-23 |

## Reference

| Artifact | Purpose | Link |
| ---------- | --------- | ------ |
| PRD.md | Product requirements | [PRD.md](../../PRD.md) |
| FRD.md | Feature requirements | [FRD.md](FRD.md) |
| DESIGN.md | Surface design | [DESIGN.md](DESIGN.md) |
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md) |
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
