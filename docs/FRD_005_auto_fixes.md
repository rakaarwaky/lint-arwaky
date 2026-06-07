# FRD — Apply Safe Auto-Fixes (Rust + Python + JS/TS)

> **PRD Reference**: [FR-005](PRD.md) — Apply safe auto-fixes
> **Dependency**: FR-003 (AST scanning)
> **Status**: ⚠️ PARTIAL — Naming renamer implemented; `apply_fix` on adapters + orchestrator are stubs

## 1. Problem Statement

Before auto-fixes:

| Issue | Description |
|-------|-------------|
| **No automated remediation** | All violations fixed manually |
| **No fix orchestration** | No unified fix pipeline |
| **No dry-run** | No preview of changes |
| **No audit trail** | No record of what was fixed |

## 2. Solution Overview

Unified fix interface: `apply_fix()` on each linter adapter + `LintFixOrchestrator` for coordination.

| Layer | Role |
|-------|------|
| **Taxonomy** | `FixResultVO`, `FixAppliedEvent` |
| **Contract** | `LintFixOrchestratorAggregate`, `ILinterAdapterPort::apply_fix()` |
| **Capabilities** | `NamingRenamerProcessor` — project-wide rename (working) |
| **Infrastructure** | All `*_adapter.rs` expose `apply_fix()` — currently stubs |
| **Agent** | `LintFixOrchestrator` — coordinates execution (stub) |
| **Surfaces** | `cli_fix_command.rs` — CLI entry (falls back to check) |

### Fixability

| Rule | Auto-Fixable | Status |
|------|-------------|--------|
| AES003 (naming) | ✅ Safe | `NamingRenamerProcessor` working |
| AES014 (bypass) | ✅ Safe | Stub |
| AES015 (unused) | ✅ Safe | Stub |
| AES011 (suffix) | ⚠️ Partial | Stub |
| AES004/005 (size) | ❌ Manual | — |
| AES006 (primitives) | ❌ Manual | — |
| AES001 (imports) | ❌ Manual | — |

## 3. Key Files

| File | Layer | Role |
|------|-------|------|
| `taxonomy/fix_result_vo.rs` | Taxonomy | Fix result |
| `taxonomy/fix_applied_event.rs` | Taxonomy | Audit event |
| `contract/lint_fix_aggregate.rs` | Contract | `LintFixOrchestratorAggregate` |
| `contract/linter_adapter_port.rs` | Contract | `apply_fix()`, `preview_fix()` |
| `capabilities/naming_renamer_processor.rs` | Capabilities | Symbol rename (working) |
| `infrastructure/rust_linter_adapter.rs` | Infrastructure | `apply_fix` stub |
| `infrastructure/python_ruff_adapter.rs` | Infrastructure | `apply_fix` stub |
| `infrastructure/javascript_linter_adapter.rs` | Infrastructure | `apply_fix` stub |
| `agent/lint_fix_orchestrator.rs` | Agent | Orchestrator stub |
| `surfaces/cli_fix_command.rs` | Surfaces | CLI entry |

## 4. Acceptance Criteria

| # | Kriteria | Status |
|---|----------|--------|
| AC001 | `fix .` runs lint + auto-fix pipeline | ⚠️ Stub |
| AC002 | Naming violations fixed via `NamingRenamerProcessor` | ✅ Working |
| AC003 | Bypass comments removed automatically | ⚠️ Stub |
| AC004 | Unused imports removed automatically | ⚠️ Stub |
| AC005 | `apply_fix` on all adapters (Rust/Python/JS) | ⚠️ Stub |
| AC006 | Dry-run `--dry-run` previews changes | ❌ Missing |
| AC007 | `FixAppliedEvent` for audit trail | ❌ Missing |
| AC008 | Manual-only violations reported as manual steps | ⚠️ Stub |
| AC009 | `cargo check --bin lint-arwaky-cli` passes | ✅ |
| AC010 | `cargo test` passes | ✅ |
