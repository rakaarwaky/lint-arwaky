// Unit tests for TestFilePrefixChecker — AES103 test/bench prefix discipline.
use naming_rules_lint_arwaky::capabilities_test_file_prefix_checker::TestFilePrefixChecker;
use shared_naming_rules::TestSuiteSlot;
use shared_naming_rules::utility_test_prefix::{
    leaf_directory_of, nested_under, prefix_list, prefixes_for, slot_of,
};

fn checker() -> TestFilePrefixChecker {
    TestFilePrefixChecker::new()
}

/// The filename half of `path`, the way the checker receives it from a caller.
fn basename(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

/// Run the check the way the orchestrator does.
fn check(path: &str) -> Option<String> {
    checker()
        .check_test_file_prefix_internal(path, basename(path))
        .map(|r| r.message.value().to_string())
}

// ─── Construction ──────────────────────────────────────────────────────────

#[test]
fn construction_succeeds() {
    let _ = checker();
}

// ─── Legal names, no violation ─────────────────────────────────────────────

#[test]
fn every_test_type_prefix_passes_in_tests_dir() {
    for prefix in [
        "contract_",
        "unit_",
        "integration_",
        "dogfood_",
        "smoke_",
        "e2e_",
        "acceptance_",
    ] {
        let path = format!("crates/calc/tests/{prefix}adder.rs");
        assert!(
            check(&path).is_none(),
            "'{prefix}' is one of the seven test types and must pass; got {:?}",
            check(&path)
        );
    }
}

#[test]
fn every_support_prefix_passes_in_tests_dir() {
    // These are the conventions the repository already follows — TEST.md §2.0
    // requires `regression_` — so AES103 must accept them even though they are
    // not one of the seven types.
    for prefix in ["regression_", "behavioral_", "mock_", "fixture_"] {
        let path = format!("crates/calc/tests/{prefix}helper.rs");
        assert!(
            check(&path).is_none(),
            "'{prefix}' is a legal support prefix and must pass; got {:?}",
            check(&path)
        );
    }
}

#[test]
fn bench_prefix_passes_in_benches_dir() {
    assert!(check("crates/calc/benches/bench_adder.rs").is_none());
}

#[test]
fn a_file_outside_both_directories_is_not_read_at_all() {
    // Production source carries no test-type prefix, and the slot check is what
    // keeps AES103 off it — the reason the rule takes its own file set.
    //
    // Paths reaching this rule come from the discovery walk, which roots at the
    // scan target and yields no `..` segment, so the classification reads the
    // path as written rather than canonicalizing it.
    for path in [
        "crates/calc/src/capabilities_adder_checker.rs",
        "crates/calc/src/taxonomy_domain_vo.rs",
        "crates/calc/src/unit_adder.rs",
        "workspaces-good/modules/addition/src/agent_addition_orchestrator.py",
    ] {
        assert!(
            check(path).is_none(),
            "'{path}' is production source and carries no test prefix; got {:?}",
            check(path)
        );
    }
}

#[test]
fn a_barrel_in_a_support_dir_is_skipped() {
    // The barrel re-exports the support module beside it; it names no test.
    assert!(check("crates/calc/tests/common/mod.rs").is_none());
}

#[test]
fn a_file_under_the_support_dir_is_treated_as_flat() {
    assert!(
        nested_under("crates/calc/tests/common/mock_filesystem.rs").is_none(),
        "`tests/common/` is the one sanctioned subdirectory, so a file inside it \\
         is as deep as the layout allows"
    );
    assert!(check("crates/calc/tests/common/mock_filesystem.rs").is_none());
}

// ─── Illegal names, violation ──────────────────────────────────────────────

#[test]
fn a_file_with_no_legal_prefix_produces_a_violation() {
    assert!(
        check("crates/calc/tests/helpers.rs").is_some(),
        "a file with no legal test-type prefix must fire AES103"
    );
}

#[test]
fn the_violation_lists_the_legal_prefixes_for_that_directory() {
    let message = check("crates/calc/tests/helpers.rs").expect("must fire");
    for prefix in ["contract_", "unit_", "e2e_"] {
        assert!(
            message.contains(prefix),
            "the message must list the legal set so a reader can rename without \\
             guessing; missing {prefix} in: {message}"
        );
    }
}

#[test]
fn a_bench_prefix_inside_tests_says_move_rather_than_rename() {
    // `bench_` is legal in benches/ and nowhere else, so the fix is a move.
    let message = check("crates/calc/tests/bench_adder.rs").expect("must fire");
    assert!(
        message.contains("benches/"),
        "the message must name the directory the prefix belongs to; got {message}"
    );
    assert!(
        message.contains("Move the file"),
        "a prefix from the wrong directory needs a move, not a rename; got {message}"
    );
}

#[test]
fn a_test_prefix_inside_benches_says_move_rather_than_rename() {
    let message = check("crates/calc/benches/unit_adder.rs").expect("must fire");
    assert!(
        message.contains("Move the file"),
        "a prefix from the wrong directory needs a move, not a rename; got {message}"
    );
}

#[test]
fn a_nested_file_reports_the_depth_it_sits_at() {
    let message = check("crates/calc/tests/inner/unit_adder.rs").expect("must fire");
    assert!(
        message.contains("1 level(s) below"),
        "the message must say how deep the file sits so the reader knows which \\
         level to flatten; got {message}"
    );
    // The nesting finding replaces the prefix finding rather than duplicating it.
    assert!(
        !message.contains("does not start with a legal"),
        "a nested file is one defect, not two; got {message}"
    );
}

#[test]
fn a_benches_subdirectory_is_also_reported() {
    assert!(check("crates/calc/benches/inner/bench_adder.rs").is_some());
}

// ─── The shared vocabulary ─────────────────────────────────────────────────

#[test]
fn slot_is_read_from_the_containing_directory() {
    assert_eq!(
        slot_of("crates/calc/tests/unit_adder.rs"),
        TestSuiteSlot::Tests
    );
    assert_eq!(
        slot_of("crates/calc/benches/bench_adder.rs"),
        TestSuiteSlot::Benches
    );
    assert_eq!(
        slot_of("crates/calc/src/capabilities_adder_checker.rs"),
        TestSuiteSlot::Outside
    );
}

#[test]
fn the_containing_directory_is_the_parent_segment_not_the_file_name() {
    // Taking the last segment of the whole path would return the file name and
    // classify every source file as a test.
    assert_eq!(
        leaf_directory_of("crates/calc/tests/unit_adder.rs").as_deref(),
        Some("tests")
    );
    assert_eq!(leaf_directory_of("unit_adder.rs").as_deref(), None);
}

#[test]
fn tests_dir_carries_the_seven_types_plus_the_support_set() {
    let legal = prefix_list(prefixes_for(TestSuiteSlot::Tests));
    for prefix in [
        "contract_",
        "unit_",
        "integration_",
        "dogfood_",
        "smoke_",
        "e2e_",
        "acceptance_",
        "regression_",
        "behavioral_",
        "mock_",
        "fixture_",
    ] {
        assert!(
            legal.contains(prefix),
            "'{prefix}' must be legal in tests/; got {legal}"
        );
    }
}

#[test]
fn benches_dir_carries_only_the_benchmark_prefix() {
    assert_eq!(prefixes_for(TestSuiteSlot::Benches), &["bench_"]);
}

#[test]
fn nesting_reports_the_directory_and_the_level_count() {
    assert_eq!(
        nested_under("crates/calc/tests/inner/deeper/unit_adder.rs"),
        Some(("tests", 2))
    );
    assert_eq!(nested_under("crates/calc/tests/unit_adder.rs"), None);
}

#[test]
fn a_windows_separator_path_is_classified_the_same_way() {
    assert_eq!(
        slot_of("crates\\calc\\tests\\unit_adder.rs"),
        TestSuiteSlot::Tests
    );
    assert_eq!(
        leaf_directory_of("crates\\calc\\tests\\unit_adder.rs").as_deref(),
        Some("tests")
    );
    assert_eq!(
        nested_under("crates\\calc\\tests\\inner\\unit_adder.rs"),
        Some(("tests", 1))
    );
}
