# Review Report: orphan-detector — Business Analyst

## Summary

The orphan-detector crate implements AES501–AES506 orphan detection across the 7-layer architecture. Requirements are well-structured with clear per-layer scope, but several gaps exist: missing acceptance criteria for edge cases, incomplete traceability for the graph resolver's multi-language support, and ambiguous severity assignments. The implementation is solid with good test coverage (20 test files), but the FRD lacks measurable thresholds for "zero false warnings" and performance targets are vague.

## Findings by Category

### Requirements Clarity & Completeness

| # | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 1 | 🟡 WARNING | FRD states "100% detection of unused or unreachable architectural files" but does not define what constitutes "architectural" vs. non-architectural files. | FRD.md:25 | Define scope explicitly: only files matching AES naming conventions (`taxonomy_*`, `contract_*`, etc.) are in scope. |
| 2 | 🟡 WARNING | "Zero false warnings on valid code" is stated but no formal definition of "valid code" provided. | FRD.md:26 | Add acceptance criteria: "A file is valid if it is transitively reachable from an entry point AND matches its layer's naming convention." |
| 3 | 🟡 WARNING | AES503 requirement says capabilities "must be instantiated or imported by orchestrators or other capability files" — but the implementation also checks container wiring. | FRD.md:14 vs. capabilities_orphan_capabilities_analyzer.rs:34-68 | Update FRD to include container wiring as a valid reachability path. |
| 4 | 🟢 INFO | FRD does not mention the `__init__.py` / `mod.rs` / `lib.rs` exception handling that exists in code. | agent_orphan_orchestrator.rs:230-236 | Document barrel file exceptions in FRD scope section. |
| 5 | 🟡 WARNING | AES506 requirement mentions "routing system" but implementation checks entry/router files by filename pattern, not actual routing registration. | FRD.md:20 vs. capabilities_orphan_surfaces_analyzer.rs:119-136 | Clarify that "registered in routing system" means "imported by entry/router-named files." |

### Testability & Acceptance Criteria

| # | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 6 | 🟡 WARNING | Performance target "less than a second" is not quantified for specific project sizes. | FRD.md:28 | Define benchmarks: e.g., "<500ms for 1000 files, <2s for 5000 files." The bench file exists but no baseline is documented. |
| 7 | 🟡 WARNING | No acceptance criteria for multi-language edge cases (Python `__init__.py`, TS `index.ts`). | FRD.md | Add test cases for: Python package with nested `__init__.py`, TypeScript barrel exports, Rust `mod.rs` re-exports. |
| 8 | 🟢 INFO | Acceptance tests exist for all 6 AES codes (acceptance_AES501.rs through AES506.rs). | tests/ | Good coverage — no action needed. |
| 9 | 🟡 WARNING | No acceptance criteria for config-driven exceptions (`ignored_paths`, `orphan_entry_points`). | FRD.md:21 | Add test: "Configured exceptions must not produce violations." |

### Scope & Dependencies

| # | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 10 | 🟡 WARNING | Dependency on `shared` crate is heavy — 15+ shared modules imported across analyzers. | Cargo.toml:18, src/*.rs | Document which shared modules are critical vs. optional. Consider if some shared utilities could be local to orphan-detector. |
| 11 | 🟢 INFO | `criterion` benchmark dependency is properly configured. | Cargo.toml:23-28 | Good practice — benchmark infrastructure is in place. |
| 12 | 🟡 WARNING | FRD does not mention workspace structure assumptions (crates/packages/modules directories). | FRD.md | Add scope: "Assumes workspace follows AES convention with `crates/`, `packages/`, `modules/` directories." |

### Traceability (FRD ↔ Code)

| # | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |
| 13 | 🟡 WARNING | AES501 (Taxonomy Orphan) — FRD says "must be reachable from contracts, capabilities, or orchestrators" but implementation checks inbound links from ANY non-taxonomy file. | FRD.md:10 vs. capabilities_orphan_taxonomy_analyzer.rs:54-59 | Align FRD: taxonomy is orphan if not imported by ANY file outside taxonomy layer. |
| 14 | 🟡 WARNING | AES502 (Contract Orphan) — FRD says "must have at least one active implementation" but implementation also checks if port/protocol is called by orchestrator/container. | FRD.md:12 vs. capabilities_orphan_contract_analyzer.rs:93-143 | FRD should distinguish: (a) trait not implemented, (b) port/protocol not called, (c) aggregate not used by surface. |
| 15 | 🟡 WARNING | AES504 (Utility Orphan) — FRD says "must be wired into root containers or imported by capabilities/agents" but implementation defines `CONSUMER_LAYERS` as `["capabilities", "agent", "surface", "root"]`. | FRD.md:16 vs. capabilities_orphan_utility_analyzer.rs:11 | FRD should include "surface" as a valid consumer of utility files. |
| 16 | 🟢 INFO | AES505 (Agent Orphan) — FRD and implementation align: agent is orphan if aggregate traits not called by surface/entry. | FRD.md:18 vs. capabilities_orphan_agent_analyzer.rs:39-79 | Good traceability. |
| 17 | 🟢 INFO | AES506 (Surface Orphan) — FRD and implementation align with 3-category classification (smart/utility/passive). | FRD.md:20 vs. capabilities_orphan_surfaces_analyzer.rs:164-171 | Good traceability. |

## Violations (if any)

No AES layer violations detected in the orphan-detector crate itself. The crate follows AES naming conventions and layer boundaries correctly:

- `agent_orphan_orchestrator.rs` — Agent layer (orchestrator role)
- `capabilities_orphan_*.rs` — Capabilities layer (analyzer role)
- `root_orphan_detector_container.rs` — Root layer (container role)
- `lib.rs` — Module declarations only

## Action Items

- [ ] 🟡 P1 Update FRD.md to align with actual implementation behavior (findings #3, #5, #13, #14, #15)
- [ ] 🟡 P1 Add measurable performance benchmarks to FRD success indicators (finding #6)
- [ ] 🟡 P2 Document barrel file exceptions (`__init__.py`, `mod.rs`, `lib.rs`) in FRD scope (finding #4)
- [ ] 🟡 P2 Add acceptance criteria for config-driven exceptions (finding #9)
- [ ] 🟡 P2 Document workspace structure assumptions in FRD (finding #12)
- [ ] 🟢 P3 Add multi-language edge case test scenarios to FRD (finding #7)

## Gap Analysis Table

| Current State | Issue | Recommendation | Priority |
| ------------- | ----- | -------------- | -------- |
| FRD defines 6 orphan rules (AES501-506) | Implementation has additional logic not in FRD (container wiring, barrel file exceptions, multi-language support) | Expand FRD to cover all implementation paths | P1 |
| Performance target is "less than a second" | No quantified benchmarks for different project sizes | Add benchmark baselines: 1000 files <500ms, 5000 files <2s | P1 |
| Success indicator: "100% detection" | No formal definition of detection scope | Define scope: files matching AES naming conventions only | P1 |
| Success indicator: "zero false warnings" | No formal definition of "valid code" | Define validity: transitively reachable from entry points + naming convention match | P2 |
| 20 test files covering unit/integration/acceptance/e2e/bench | No documented test data or fixtures | Document test workspace structure and expected violation counts | P2 |
| Config supports `ignored_paths` and `orphan_entry_points` | No acceptance criteria for config behavior | Add test: "Configured exceptions produce zero violations" | P2 |
