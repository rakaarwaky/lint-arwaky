# Plan: auto-fix — Architect (Merged Plan)

## Summary

The `auto-fix` crate is a well-structured, AES-compliant feature implementing mechanical corrections for lint violations (AES101 naming, AES203 unused imports, AES304 bypass comments). It follows the 7-layer architecture correctly: contracts live in `shared/auto_fix/`, capabilities implement protocols, the agent orchestrator is a thin delegation layer, and the root container wires everything via DI. All imports follow allowed dependency directions. The BA and TL reports were empty (no prior analysis available); this plan is based on full source review across all 5 source files, 8 test files, 1 benchmark, and 7 shared contracts.

**Overall assessment:** PASS with minor observations. No critical or high-severity findings. Two low-severity cleanup items and one informational note.

---

## Merged Findings

### Layer Boundaries

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 1 | 🟢 INFO | Capabilities file `capabilities_fix_processor.rs` imports `shared::quality_rules::contract_code_analysis_aggregate::ICodeAnalysisAggregate` — cross-crate contract dependency. Valid per AES rules (capabilities may import contract protocols), but couples auto-fix to quality-rules at the type level. | `capabilities_fix_processor.rs:32` | No action needed — correct per spec. Document the coupling in FRD integration points (already documented). |
| 2 | 🟢 INFO | Root container `root_auto_fix_container.rs` imports concrete capability structs directly (`use crate::agent_fix_orchestrator::FixOrchestrator`, etc.) — correct per root layer spec (root may depend on all layers for DI wiring). | `root_auto_fix_container.rs:7-9` | No action needed — correct per spec. |

### Naming

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| — | — | All 5 source files pass AES101-102. Prefix/suffix correct. | — | None |

### Orphan

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| — | — | All 5 source files pass AES501-506. Protocols implemented, aggregates wired, capabilities connected to container, agent called by aggregate trait. | — | None |

### Scalability

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 3 | 🟡 WARNING | `#[deprecated]` method `with_dry_run` remains in `LintFixProcessor` — adds dead code surface. The deprecation notice is correct ("Use new() + execute(path, dry_run)"), but if no callers remain, it should be removed. | `capabilities_fix_processor.rs:206-214` | Check callers across workspace. If zero callers, delete the method. |
| 4 | 🟡 WARNING | `execute()` method is ~120 lines — filters violations, applies fixes for 3 error codes, collects events, generates output. While within limits, it mixes filtering + fixing + reporting. | `capabilities_fix_processor.rs:62-175` | Consider extracting violation filtering into a private helper for readability, but not required for correctness. |

### Data Flow

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| — | — | Data flow is unidirectional: Surface → Agent (aggregate) → Capabilities (protocol) → FileAdapter (protocol) → filesystem. No cycles. DI wiring in root is clean. | — | None |

### FRD Compliance

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| — | — | FR-001 (unused import removal) ✅, FR-002 (bypass fix) ✅, FR-003 (symbol rename) ✅, FR-004 (dry-run) ✅, FR-005 (non-fixable reporting) ✅. All 5 FRs implemented. API contract matches FRD spec. Reason-coded outcomes (Applied/Skipped/Failed) used throughout. | — | None |

---

## File Classification

| File | Layer | Type Count | AES Compliance |
|------|-------|------------|----------------|
| `agent_fix_orchestrator.rs` | Agent | 1 struct + 1 impl + helpers | ✅ |
| `capabilities_file_adapter.rs` | Capabilities | 1 struct + 1 impl + constructor | ✅ |
| `capabilities_fix_processor.rs` | Capabilities | 1 struct + 1 impl + helpers + 3 free fns | ✅ |
| `root_auto_fix_container.rs` | Root | 1 struct + 1 impl + 2 constructors | ✅ |
| `lib.rs` | lib.rs (exception) | re-exports only | ✅ |

---

## Test Coverage

| Test File | Type | Status |
|-----------|------|--------|
| `contract_auto_fix.rs` | Contract | Present |
| `unit_auto_fix_file_adapter.rs` | Unit | Present |
| `unit_auto_fix_fix_processor.rs` | Unit | Present |
| `integration_auto_fix.rs` | Integration | Present |
| `e2e_auto_fix_flow.rs` | E2E | Present |
| `smoke_auto_fix.rs` | Smoke | Present |
| `acceptance_AES201_fix.rs` | Acceptance | Present |
| `acceptance_AES304_fix.rs` | Acceptance | Present |
| `benches/bench_auto_fix.rs` | Benchmark | Present |

**Note:** No `acceptance_AES101_fix.rs` test — AES101 symbol renaming has no dedicated acceptance test (covered partially by unit tests). Low priority.

---

## Validation

- [x] FRD compliance checked — all 5 FRs implemented, API contract matches
- [x] AES compliance checked — all 5 source files pass AES101-102, AES201-205, AES301-305, AES401-406, AES501-506
- [x] Skip Report validated — BA and TL reports were empty (no prior analysis); architect performed full analysis independently
- [x] Assumptions validated — all FRD assumptions verified in code (UTF-8, single-threaded, per-request dry_run, reason-coded outcomes)
- [x] Timestamp + Correlation ID signed — corr-20260807-auto-fix-800, 2026-08-07T06:30:00+07:00

---

## Action Items

- [ ] 🟡 Remove deprecated `with_dry_run` method if no callers remain (file: `capabilities_fix_processor.rs`)
- [ ] 🟢 Optional: Add `acceptance_AES101_fix.rs` test for symbol renaming acceptance coverage

---

## Fixed Code

No code changes required. The crate is architecturally compliant. The two 🟡 WARNING items are cleanup suggestions, not violations.
