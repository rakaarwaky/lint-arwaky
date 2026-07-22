# Review Report: orphan-detector — Business Analyst

## Summary

The orphan-detector crate implements AES501–AES506 orphan detection across the 7-layer architecture. After stakeholder review, the FRD has been updated to align with implementation behavior and clarify scope, detection methods, and acceptance criteria. All 17 findings have been addressed.

## Decisions Made

| # | Finding | Decision |
| --- | -------- | -------- |
| 1 | Scope definition | **All source files** in scope (not just AES-named). Naming validation handled by `naming-rules` crate. |
| 2 | Valid code definition | Orphan detector concerns **reachability only**. Assumes naming is already correct. |
| 3 | AES503 capabilities scope | Capabilities must be **wired in container**. DI pattern (Arc<T>/DI containers) — no direct capability-to-capability imports. |
| 4 | Barrel file exceptions | **Add all barrel files** to skip list: `__init__.py`, `mod.rs`, `index.ts`, `index.js`. |
| 5 | AES506 detection method | **Import graph only** for all 3 surface groups. No identifier-based fallback. |
| 6 | Performance targets | **Tiered:** 1000 files <500ms, 5000 files <2s, 10000 files <5s. |
| 7 | Multi-language tests | **Add scenarios** for Python, TypeScript, Rust edge cases. |
| 8 | Test coverage | **Add more tests** for edge cases. |
| 9 | Config exceptions | **Add criteria** — configured exceptions must not produce violations. |
| 10 | Shared dependencies | **Document critical modules** in FRD. |
| 11 | Benchmarks | No changes needed. |
| 12 | Workspace assumptions | **Document in FRD** — crates/packages/modules structure. |
| 13 | AES501 taxonomy scope | Taxonomy orphan = **no contract imports it**. Contract enforces type usage downstream. |
| 14 | AES502 contract scope | Protocol → implemented by capability, called by agent via DI. Aggregate → implemented by agent, called by surface via DI. Port → **legacy, marked for removal**. |
| 15 | AES504 utility consumers | Utility must be imported by **agent, capability, or surface**. |
| 16 | AES505 detail | **Add detail** — extract aggregate traits, check surface/entry references. |
| 17 | AES506 detail | **Add detail** — 3-group classification with dependency chain. |

## Updated FRD Sections

The following sections were added or rewritten in FRD.md:

- **Scope** — All source files, naming handled by naming-rules crate
- **Workspace Assumptions** — crates/packages/modules structure
- **Barrel File Exceptions** — __init__.py, mod.rs, index.ts/index.js
- **AES501** — Taxonomy orphan = no contract imports
- **AES502** — Protocol/aggregate/port sub-rules with DI pattern
- **AES503** — Container wiring check, no direct capability imports
- **AES504** — Agent/capability/surface as consumers
- **AES505** — Aggregate trait extraction, surface/entry reference check
- **AES506** — 3-group table with import graph detection
- **Configuration** — ignored_paths, orphan_entry_points, layer exceptions
- **Performance Targets** — Tiered benchmarks
- **Shared Module Dependencies** — Critical vs optional modules table
- **Multi-Language Test Scenarios** — Python/TS/Rust/cross-language
- **Acceptance Criteria for Config Exceptions** — 4 criteria

## Remaining Action Items

- [ ] 🟡 P1 Update implementation to match new FRD (barrel file skip, surface import graph only)
- [ ] 🟡 P1 Add multi-language acceptance test scenarios
- [ ] 🟡 P2 Add config exception acceptance tests
- [ ] 🟢 P3 Add more edge case tests for AES505/AES506 detail

## Violations (if any)

No AES layer violations detected in the orphan-detector crate itself. The crate follows AES naming conventions and layer boundaries correctly:

- `agent_orphan_orchestrator.rs` — Agent layer (orchestrator role)
- `capabilities_orphan_*.rs` — Capabilities layer (analyzer role)
- `root_orphan_detector_container.rs` — Root layer (container role)
- `lib.rs` — Module declarations only
