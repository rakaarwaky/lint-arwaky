# Feature Backlog: Naming Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-09-29

## Current Condition

- Done: `cargo test -p naming-rules-lint-arwaky` → 65 passed, 0 failed at `1b580e3d` (2026-09-29). Split of `capabilities_naming_checker` into `capabilities_naming_convention_checker` (AES101) and `capabilities_suffix_policy_checker` (AES102) completed; AES403 exception removed. Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p naming-rules-lint-arwaky` after any code change to this crate

## Backlog

| ID | FRD Ref | Work Item | Priority | State | Actual Condition | Owner | Dependencies | Updated |
|---|---|---|---:|---|---|---|---|---|
| NAMI-01 | FR-NAMINGRULES-001 | AES101 naming convention — 17 scenarios verified | P0 | Done | `cargo test -p naming-rules-lint-arwaky` → 65 passed, 0 failed at `1b580e3d` (2026-09-29). Capability: `capabilities_naming_convention_checker` (AES101 stem validation) | @raka | None | 2026-09-29 |
| NAMI-02 | FR-NAMINGRULES-002 | AES102 suffix/prefix validation — 12 scenarios verified | P0 | Done | `cargo test -p naming-rules-lint-arwaky` → 65 passed, 0 failed at `1b580e3d` (2026-09-29). Capability: `capabilities_suffix_policy_checker` (AES102 suffix enforcement); AES403 exception removed | @raka | None | 2026-09-29 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| Valid snake_case file, 3+ words, recognized layer prefix | Automated | `tests/acceptance_AES101.rs` | three_words_clean_underscore_passes | `1b580e3d` |
| File with uppercase characters in stem | Automated | `tests/acceptance_AES101.rs` | uppercase_single_word_produces_violation | `1b580e3d` |
| File with only 2 words | Automated | `tests/acceptance_AES101.rs` | two_words_produces_violation_when_min_is_3 | `1b580e3d` |
| File with hyphens in stem | Automated | `tests/acceptance_AES101.rs` | hyphen_separator_produces_violation | `1b580e3d` |
| File with dots in stem | Automated | `tests/acceptance_AES101.rs` | dots_in_name_produces_violation | `1b580e3d` |
| Barrel file or module entry point | Automated | `tests/acceptance_AES101.rs` | mod_rs_is_skipped | `1b580e3d` |
| File in the rule exception list | Automated | `tests/acceptance_AES101.rs` | excepted_filename_passes | `1b580e3d` |
| Valid file below the configured `min_words` | Automated | `tests/acceptance_AES101.rs` | config_min_words_5_three_word_file_violates | `1b580e3d` |
| File with an unrecognized prefix | Automated | `tests/acceptance_AES101.rs` | unknown_prefix_file_with_valid_snake_case_passes | `1b580e3d` |
| File with digits in a segment | Automated | `tests/acceptance_AES101.rs` | digits_in_segment_no_violation | `1b580e3d` |
| Prefix and suffix both in the taxonomy strict allow-list | Automated | `tests/acceptance_AES102.rs` | correct_suffix_for_layer_passes | `1b580e3d` |
| Taxonomy prefix with a contract-layer suffix | Automated | `tests/acceptance_AES102.rs` | cross_layer_taxonomy_with_protocol_suffix | `1b580e3d` |
| Contract prefix with a taxonomy-layer suffix | Automated | `tests/acceptance_AES102.rs` | cross_layer_suffix_violation_detected | `1b580e3d` |
| Agent layer with a suffix outside its strict allow-list | Automated | `tests/acceptance_AES102.rs` | wrong_suffix_for_layer_produces_violation | `1b580e3d` |
| Utility layer with a flexible, non-forbidden suffix | Automated | `tests/acceptance_AES102.rs` | flexible_policy_allows_unknown_suffix | `1b580e3d` |
| Utility layer with a forbidden suffix | Automated | `tests/acceptance_AES102.rs` | forbidden_suffix_produces_violation | `1b580e3d` |
| Capabilities layer with a forbidden suffix | Automated | `tests/acceptance_AES102.rs` | forbidden_suffix_produces_violation | `1b580e3d` |
| Capabilities layer with a flexible, non-forbidden suffix | Automated | `tests/acceptance_AES102.rs` | valid_file_with_correct_suffix_passes | `1b580e3d` |
| Surface layer with a suffix in its strict allow-list | Automated | `tests/acceptance_AES102.rs` | correct_suffix_for_layer_passes | `1b580e3d` |
| Surface layer with a suffix outside its strict allow-list | Automated | `tests/acceptance_AES102.rs` | wrong_suffix_for_layer_produces_violation | `1b580e3d` |
| Root layer with a suffix in its strict allow-list | Automated | `tests/acceptance_AES102.rs` | valid_file_with_correct_suffix_passes | `1b580e3d` |
| Root layer with a suffix outside its strict allow-list | Automated | `tests/acceptance_AES102.rs` | wrong_suffix_for_layer_produces_violation | `1b580e3d` |
| The build script | Automated | `tests/acceptance_AES102.rs` | aes102_barrel_file_skipped | `1b580e3d` |
| File in the exception list for its layer | Automated | `tests/acceptance_AES102.rs` | excepted_file_bypasses_suffix_check | `1b580e3d` |
| File with no suffix beyond the prefix | Automated | `tests/acceptance_AES102.rs` | single_word_no_suffix_fails_strict_policy | `1b580e3d` |
| Rule AES101 disabled in config | Automated | `tests/e2e_naming_flow.rs` | e2e_aes101_disabled_skips_convention_check | `1b580e3d` |
| Rule AES102 disabled in config | Automated | `tests/e2e_naming_flow.rs` | e2e_suffix_violations_found | `1b580e3d` |
| File in the exceptions list | Automated | `tests/acceptance_AES101.rs` | excepted_filename_passes | `1b580e3d` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo test -p naming-rules-lint-arwaky` → 65 passed, 0 failed at `1b580e3d` (2026-09-29) |
| Scenario evidence | Done | 28 scenarios mapped; all Automated via `cargo test -p naming-rules-lint-arwaky` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
| 2026-09-29 | Split `capabilities_naming_checker` into `capabilities_naming_convention_checker` (AES101) and `capabilities_suffix_policy_checker` (AES102); removed AES403 exception | @raka |
