# Feature Backlog: Structure Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-29

## Current Condition

- Done: `cargo test -p structure-rules-lint-arwaky --lib --tests` → 0 failures. Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p structure-rules-lint-arwaky --lib --tests` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| STRUC-01 | FR-STR-001 | AES701 shared folder purity — forbidden files + kernel docs | P0 | Done | `cargo test -p structure-rules-lint-arwaky --lib --tests` → 0 failures at `d3366d3b` (2026-09-29) | @raka | None | 2026-09-29 |
| STRUC-02 | FR-STR-002 | AES702 feature folder health + docs + forbidden files | P0 | Done | `cargo test -p structure-rules-lint-arwaky --lib --tests` → 0 failures at `d3366d3b` (2026-09-29) | @raka | None | 2026-09-29 |
| STRUC-03 | FR-STR-003 | AES703 surface folder purity + DESIGN.md | P0 | Done | `cargo test -p structure-rules-lint-arwaky --lib --tests` → 0 failures at `d3366d3b` (2026-09-29) | @raka | None | 2026-09-29 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Shared holds a capability file | AES701 CRITICAL | Automated | `tests/structure_rules_contract_tests.rs::aes701_fires_when_shared_holds_a_capability_file` | `d3366d3b` |
| Shared holds an agent file | AES701 CRITICAL | Automated | `tests/structure_rules_contract_tests.rs::aes701_fires_when_shared_holds_a_capability_file` (same test, second file) | `d3366d3b` |
| Shared holds only permitted layers | AES701 silent | Automated | `tests/structure_rules_contract_tests.rs::aes701_stays_silent_when_shared_holds_only_permitted_layers` | `d3366d3b` |
| Shared carries doc pair | AES701 CRITICAL | Automated | `tests/structure_rules_contract_tests.rs::aes702_fires_when_a_kernel_folder_carries_a_doc_pair` | `d3366d3b` |
| Feature has capabilities but no orchestrator | AES702 CRITICAL | Automated | `tests/structure_rules_contract_tests.rs::aes702_fires_when_capabilities_lack_an_orchestrator` | `d3366d3b` |
| Feature has orchestrator but no capabilities | AES702 CRITICAL | Automated | `tests/structure_rules_contract_tests.rs::aes702_fires_when_orchestrator_lacks_capabilities` | `d3366d3b` |
| Feature lacks doc pair | AES702 CRITICAL | Automated | `tests/structure_rules_contract_tests.rs::aes702_fires_when_a_feature_folder_lacks_its_doc_pair` | `d3366d3b` |
| Feature has both source and doc pair | AES702 silent | Automated | `tests/structure_rules_contract_tests.rs::aes702_stays_silent_when_the_doc_pair_is_present` | `d3366d3b` |
| Feature holds forbidden taxonomy file | AES702 CRITICAL | Automated | `tests/structure_rules_contract_tests.rs::aes702_fires_when_a_feature_folder_holds_forbidden_files` | `d3366d3b` |
| Feature holds forbidden utility file | AES702 CRITICAL | Automated | `tests/structure_rules_contract_tests.rs::aes702_fires_when_a_feature_folder_holds_forbidden_files` (same test) | `d3366d3b` |
| Feature holds only capabilities and agents | AES702 silent | Automated | `tests/structure_rules_contract_tests.rs::aes702_stays_silent_when_only_capabilities_and_agents_are_present` | `d3366d3b` |
| Doc pair without orchestrator (reverse) | AES702 CRITICAL | Automated | `tests/structure_rules_contract_tests.rs::aes702_fires_when_a_doc_pair_folder_has_no_orchestrator` | `d3366d3b` |
| Doc pair with orchestrator present | AES702 silent | Automated | `tests/structure_rules_contract_tests.rs::aes702_silent_when_doc_pair_folder_has_orchestrator` | `d3366d3b` |
| Utility-only folder (not a feature) | AES702 silent | Automated | `tests/structure_rules_contract_tests.rs::aes702_stays_silent_for_a_folder_carrying_no_feature_files` | `d3366d3b` |
| Surface holds a capability file | AES703 CRITICAL | Automated | `tests/structure_rules_contract_tests.rs::aes703_fires_when_a_surface_folder_holds_a_capability` | `d3366d3b` |
| Surface holds utilities + DESIGN.md | AES703 silent | Automated | `tests/structure_rules_contract_tests.rs::aes703_stays_silent_when_surface_folder_keeps_utilities` | `d3366d3b` |
| Surface lacks DESIGN.md | AES703 CRITICAL | Automated | `tests/structure_rules_contract_tests.rs::aes703_fires_when_a_surface_folder_lacks_design_md` | `d3366d3b` |
| Surface present with DESIGN.md | AES703 silent | Automated | `tests/structure_rules_contract_tests.rs::aes703_stays_silent_when_design_md_is_present` | `d3366d3b` |
| Feature folder with one surface is not surface-dominated | AES703 silent | Automated | `tests/structure_rules_contract_tests.rs::aes703_does_not_fire_on_a_feature_folder_that_holds_a_surface` | `d3366d3b` |
| Conforming workspace reports no structure violations | No violation | Self-lint | `lint-arwaky-cli check .` | `d3366d3b` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | 26 tests, 0 failures at `d3366d3b` (2026-09-29) |
| Scenario evidence | Done | 18 scenarios mapped; all Automated |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-29 | Consolidated AES704 into AES702 and AES705 into AES703; added feature forbidden-files check; rewrote FRD with 3 FRs | @raka |
| 2026-09-29 | Aligned BACKLOG.md and FRD.md to HOW-TO templates (AES605 H2 contracts) | @raka |
