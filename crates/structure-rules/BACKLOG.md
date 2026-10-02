# Feature Backlog: Structure Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-10-02

## Current Condition

- Done: AES704 (test-suite coverage) added; the two contract files were renamed to `contract_`/`unit_` prefixes so AES103 passes, and `unit_`, `integration_`, `dogfood_`, `smoke_`, `e2e_`, `acceptance_`, and `benches/bench_structure_rules.rs` were added to satisfy AES704. `cargo nextest run -p structure-rules-lint-arwaky --lib --tests` → 0 failures. Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p structure-rules-lint-arwaky --lib --tests` after any code change to this crate

## Backlog

| ID | Priority | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|
| STRUC-01 | P0 | Done | On Track | None | — | 2026-09-29  |
| STRUC-02 | P0 | Done | On Track | None | — | 2026-09-29  |
| STRUC-03 | P0 | Done | On Track | None | — | 2026-09-29  |
| STRUC-04 | P0 | Done | On Track | None | — | 2026-10-02  |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Shared holds a capability file | AES701 CRITICAL | Automated | `tests/contract_structure_rules.rs::aes701_fires_when_shared_holds_a_capability_file` | `d3366d3b` |
| Shared holds an agent file | AES701 CRITICAL | Automated | `tests/contract_structure_rules.rs::aes701_fires_when_shared_holds_a_capability_file` (same test, second file) | `d3366d3b` |
| Shared holds only permitted layers | AES701 silent | Automated | `tests/contract_structure_rules.rs::aes701_stays_silent_when_shared_holds_only_permitted_layers` | `d3366d3b` |
| Shared carries doc pair | AES701 CRITICAL | Automated | `tests/contract_structure_rules.rs::aes702_fires_when_a_kernel_folder_carries_a_doc_pair` | `d3366d3b` |
| Feature has capabilities but no orchestrator | AES702 CRITICAL | Automated | `tests/contract_structure_rules.rs::aes702_fires_when_capabilities_lack_an_orchestrator` | `d3366d3b` |
| Feature has orchestrator but no capabilities | AES702 CRITICAL | Automated | `tests/contract_structure_rules.rs::aes702_fires_when_orchestrator_lacks_capabilities` | `d3366d3b` |
| Feature lacks doc pair | AES702 CRITICAL | Automated | `tests/contract_structure_rules.rs::aes702_fires_when_a_feature_folder_lacks_its_doc_pair` | `d3366d3b` |
| Feature has both source and doc pair | AES702 silent | Automated | `tests/contract_structure_rules.rs::aes702_stays_silent_when_the_doc_pair_is_present` | `d3366d3b` |
| Feature holds forbidden taxonomy file | AES702 CRITICAL | Automated | `tests/contract_structure_rules.rs::aes702_fires_when_a_feature_folder_holds_forbidden_files` | `d3366d3b` |
| Feature holds forbidden utility file | AES702 CRITICAL | Automated | `tests/contract_structure_rules.rs::aes702_fires_when_a_feature_folder_holds_forbidden_files` (same test) | `d3366d3b` |
| Feature holds only capabilities and agents | AES702 silent | Automated | `tests/contract_structure_rules.rs::aes702_stays_silent_when_only_capabilities_and_agents_are_present` | `d3366d3b` |
| Doc pair without orchestrator (reverse) | AES702 CRITICAL | Automated | `tests/contract_structure_rules.rs::aes702_fires_when_a_doc_pair_folder_has_no_orchestrator` | `d3366d3b` |
| Doc pair with orchestrator present | AES702 silent | Automated | `tests/contract_structure_rules.rs::aes702_silent_when_doc_pair_folder_has_orchestrator` | `d3366d3b` |
| Utility-only folder (not a feature) | AES702 silent | Automated | `tests/contract_structure_rules.rs::aes702_stays_silent_for_a_folder_carrying_no_feature_files` | `d3366d3b` |
| Surface holds a capability file | AES703 CRITICAL | Automated | `tests/contract_structure_rules.rs::aes703_fires_when_a_surface_folder_holds_a_capability` | `d3366d3b` |
| Surface holds utilities + DESIGN.md | AES703 silent | Automated | `tests/contract_structure_rules.rs::aes703_stays_silent_when_surface_folder_keeps_utilities` | `d3366d3b` |
| Surface lacks DESIGN.md | AES703 CRITICAL | Automated | `tests/contract_structure_rules.rs::aes703_fires_when_a_surface_folder_lacks_design_md` | `d3366d3b` |
| Surface present with DESIGN.md | AES703 silent | Automated | `tests/contract_structure_rules.rs::aes703_stays_silent_when_design_md_is_present` | `d3366d3b` |
| Feature folder with one surface is not surface-dominated | AES703 silent | Automated | `tests/contract_structure_rules.rs::aes703_does_not_fire_on_a_feature_folder_that_holds_a_surface` | `d3366d3b` |
| Conforming workspace reports no structure violations | No violation | Self-lint | `lint-arwaky-cli check .` | `d3366d3b` |
| Feature folder owns source but has no `tests/` | AES704 MEDIUM | Automated | `tests/e2e_structure_rules_flow.rs::e2e_each_missing_test_category_is_named_and_located` | `HEAD` |
| Feature folder owns source but has no `benches/` | AES704 MEDIUM | Automated | `tests/e2e_structure_rules_flow.rs::e2e_a_missing_bench_directory_is_reported_once_not_per_category` | `HEAD` |
| `tests/` missing exactly one of the seven categories | AES704 MEDIUM | Automated | `tests/e2e_structure_rules_flow.rs::e2e_each_missing_test_category_is_named_and_located` | `HEAD` |
| A support prefix satisfies no category | AES704 MEDIUM | Automated | `tests/e2e_structure_rules_flow.rs::e2e_a_support_prefix_does_not_satisfy_a_required_category` | `HEAD` |
| Folder owning no feature source owes no suite | AES704 silent | Automated | `tests/e2e_structure_rules_flow.rs::e2e_a_folder_owning_no_feature_source_owes_no_test_suite` | `HEAD` |
| Feature folder carries all eight categories | AES704 silent | Automated | `tests/e2e_structure_rules_flow.rs::e2e_a_conforming_workspace_produces_no_findings_end_to_end` | `HEAD` |
| One request reaches all four auditor seams | AES701–AES704 | Automated | `tests/integration_structure_rules.rs::container_reaches_every_auditor_seam` | `HEAD` |
| Repeated requests agree and stay deduplicated | AES701–AES704 | Automated | `tests/acceptance_FR_STRUCT_004.rs::repeated_requests_agree_on_the_finding_list`, `::repeated_findings_are_collapsed` | `HEAD` |
| Structure audit answers a request under 5 s | Smoke | Automated | `tests/smoke_structure_rules.rs::structure_rules_boots_and_answers_a_request_within_the_smoke_budget` | `HEAD` |
| Per-folder audit cost over a 60-folder workspace | Perf | Automated | `benches/bench_structure_rules.rs::bench_structure_audit` | `HEAD` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | 41 tests, 0 failures at `HEAD` (2026-10-02) |
| Scenario evidence | Done | 30 scenarios mapped; all Automated |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-29 | Consolidated AES704 into AES702 (feature_missing_doc_pair sub-check) and AES705 into AES703 (surface_missing_design_md sub-check); added feature forbidden-files check; rewrote FRD with 3 FRs | @raka |
| 2026-09-29 | Aligned BACKLOG.md and FRD.md to HOW-TO templates (AES605 H2 contracts) | @raka |
| 2026-10-02 | Published **AES704** (test-suite category coverage): a feature folder owning source owes one file per test type in `tests/` plus one `bench_` file. Added `capabilities_test_suite_coverage_auditor.rs`, the `IStructureTestSuiteProtocol` seam, and the four test files plus a criterion bench that AES704 required. Renamed the two pre-existing contract files to `contract_`/`unit_` prefixes so AES103 passes | @raka |
