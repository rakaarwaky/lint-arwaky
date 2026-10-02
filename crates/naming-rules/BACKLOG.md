# Feature Backlog: Naming Rules

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: defined in root [ROADMAP.md](../../ROADMAP.md) — cited here, not restated
Last Updated: 2026-10-02

## Current Condition

- Done: **AES103** (test/bench file-prefix discipline) added as this crate's third seam, backed by `capabilities_test_file_prefix_checker.rs`, the `ITestFilePrefixProtocol` contract, and a `tests/`+`benches/`-aware discovery walk in the filesystem foundation — the default walk prunes both directories, so AES103 receives its own file set alongside the production source. `cargo nextest run -p naming-rules-lint-arwaky --lib --tests` → 0 failures at `HEAD` (2026-10-02). Per-scenario evidence below.
- In Progress: None
- Blocked: None
- Next Action: Re-run `cargo test -p naming-rules-lint-arwaky` after any code change to this crate

## Backlog

| ID | Priority | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|
| NAMI-01 | P0 | Done | On Track | None | — | 2026-09-29  |
| NAMI-02 | P0 | Done | On Track | None | — | 2026-09-29  |
| NAMI-03 | P0 | Done | On Track | None | — | 2026-10-02  |

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
| All seven test-type prefixes accepted in `tests/` | AES103 silent | Automated | `tests/acceptance_AES103.rs::every_seven_test_types_are_accepted_in_tests_dir` | `HEAD` |
| A support prefix (`regression_` et al.) accepted in `tests/` | AES103 silent | Automated | `tests/acceptance_AES103.rs::a_regression_guard_is_a_legal_name_even_though_it_is_not_a_category` | `HEAD` |
| A `tests/common/` support module accepted | AES103 silent | Automated | `tests/acceptance_AES103.rs::a_support_directory_is_accepted_so_shared_helpers_survive` | `HEAD` |
| An unprefixed test file fires and lists the legal set | AES103 HIGH | Automated | `tests/acceptance_AES103.rs::an_unprefixed_test_file_is_named_with_its_legal_prefixes` | `HEAD` |
| A `bench_` file inside `tests/` fires as a move | AES103 HIGH | Automated | `tests/acceptance_AES103.rs::a_benchmark_named_inside_tests_says_move_it` | `HEAD` |
| A test prefix inside `benches/` fires as a move | AES103 HIGH | Automated | `tests/acceptance_AES103.rs::a_test_named_inside_benches_says_move_it` | `HEAD` |
| A subdirectory under `tests/` fires with its depth | AES103 HIGH | Automated | `tests/acceptance_AES103.rs::a_subdirectory_under_tests_says_how_deep_and_how_to_flatten` | `HEAD` |
| Production source is never reported | AES103 silent | Automated | `tests/acceptance_AES103.rs::production_source_is_never_reported` | `HEAD` |
| A conforming suite is clean end to end | AES103 silent | Automated | `tests/e2e_naming_flow.rs::e2e_a_test_file_with_a_legal_prefix_is_clean` | `HEAD` |
| An illegal prefix fires AES103 and not AES101 | AES103 only | Automated | `tests/e2e_naming_flow.rs::e2e_an_illegal_test_prefix_fires_only_aes103` | `HEAD` |
| Nesting is reported once, not twice | AES103 once | Automated | `tests/e2e_naming_flow.rs::e2e_a_nested_test_file_fires_the_flatness_half_only` | `HEAD` |
| Rule AES103 disabled in config | AES103 silent | Automated | `tests/e2e_naming_flow.rs::e2e_aes103_disabled_in_config_skips_the_prefix_check` | `HEAD` |
| Slot reads the containing directory, not the file name | Unit | Automated | `tests/unit_naming_rules_test_prefix_checker.rs::the_containing_directory_is_the_parent_segment_not_the_file_name` | `HEAD` |
| A Windows-separator path classifies identically | Unit | Automated | `tests/unit_naming_rules_test_prefix_checker.rs::a_windows_separator_path_is_classified_the_same_way` | `HEAD` |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | `cargo nextest run -p naming-rules-lint-arwaky --lib --tests` → 0 failures at `HEAD` (2026-10-02) |
| Scenario evidence | Done | 42 scenarios mapped; all Automated via `cargo nextest run -p naming-rules-lint-arwaky --lib --tests` |
| Docs | Done | [FRD.md](FRD.md) is specification-only; status lives in this file |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-17 | Initial backlog created from FRD test-scenario mapping | @raka |
| 2026-09-29 | Split `capabilities_naming_checker` into `capabilities_naming_convention_checker` (AES101) and `capabilities_suffix_policy_checker` (AES102); removed AES403 exception | @raka |
| 2026-10-02 | Published **AES103** (test/bench file-prefix discipline). Added `capabilities_test_file_prefix_checker.rs`, the `ITestFilePrefixProtocol` seam, the shared prefix vocabulary (`taxonomy_naming_rules_constant.rs` + `utility_test_prefix.rs`), and a `tests/`+`benches/`-aware discovery walk in the filesystem foundation so the rule sees the files the default walk prunes. Renamed `utility_config_merger_tests.rs` to a `unit_` prefix | @raka |
