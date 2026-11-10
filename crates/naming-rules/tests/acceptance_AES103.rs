// Acceptance tests — AES103 test/bench file-prefix discipline.
//
// Each case maps to a user story: an agent scaffolding a test suite needs the
// linter to name the offending file, the directory it sits in, and whether the
// fix is a rename or a move. A message that names only the file leaves the
// author guessing which of the two it is.
use naming_rules_lint_arwaky::capabilities_test_file_prefix_checker::TestFilePrefixChecker;
use shared_common::taxonomy_violation_item_vo::ViolationItem;

/// Run the check the way the orchestrator does: the path plus its basename.
fn audit(path: &str) -> Option<ViolationItem> {
    let filename = path.rsplit('/').next().unwrap_or(path);
    TestFilePrefixChecker::new().check_test_file_prefix_internal(path, filename)
}

#[test]
fn every_seven_test_types_are_accepted_in_tests_dir() {
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
            audit(&path).is_none(),
            "an agent writing {prefix}adder.rs is following the naming convention and must not be \\
             reported; got {:?}",
            audit(&path).map(|r| r.message.value().to_string())
        );
    }
}

#[test]
fn a_benchmark_is_accepted_in_benches_dir() {
    assert!(audit("crates/calc/benches/bench_adder.rs").is_none());
}

#[test]
fn an_unprefixed_test_file_is_named_with_its_legal_prefixes() {
    let result = audit("crates/calc/tests/helpers.rs").expect("must fire");
    let message = result.message.value();
    let fix = result.fix.as_str();
    // The reader's editor shows the basename in the file column, so the
    // message names the offending file and the fix names the directory and
    // the legal values: the message states the problem, the fix the action.
    assert!(
        message.contains("helpers.rs"),
        "the message must name the offending file; got {message}"
    );
    assert!(
        fix.contains("tests/"),
        "the fix must name the directory the file sits in; got {fix}"
    );
    assert!(
        fix.contains("contract_") && fix.contains("acceptance_"),
        "the fix must list the legal prefix set so the author can rename without \
         guessing; got {fix}"
    );
    // The support set is legal too, so the listing must not imply the seven are
    // the only options — `regression_` is a name the repository relies on.
    assert!(
        fix.contains("regression_"),
        "the listing must include the support prefixes; otherwise an author holding a \
         regression guard reads the message as telling them to rename it. Got {fix}"
    );
}

#[test]
fn a_benchmark_named_inside_tests_says_move_it() {
    // The prefix is legal, just in the other directory — so the fix is a move.
    let result = audit("crates/calc/tests/bench_adder.rs").expect("must fire");
    let message = result.message.value();
    let fix = result.fix.as_str();
    assert!(
        message.contains("benches/"),
        "the message must name the directory the prefix belongs to; got {message}"
    );
    assert!(
        fix.contains("Move the file"),
        "a prefix from the wrong directory needs a move, not a rename; got {fix}"
    );
}

#[test]
fn a_test_named_inside_benches_says_move_it() {
    let result = audit("crates/calc/benches/unit_adder.rs").expect("must fire");
    let message = result.message.value();
    let fix = result.fix.as_str();
    assert!(
        message.contains("tests/"),
        "the message must name the directory the prefix belongs to; got {message}"
    );
    assert!(
        fix.contains("Move the file"),
        "a test prefix inside benches/ needs a move; got {fix}"
    );
}

#[test]
fn a_subdirectory_under_tests_says_how_deep_and_where_the_file_belongs() {
    let result = audit("crates/calc/tests/inner/unit_adder.rs").expect("must fire");
    let message = result.message.value();
    let fix = result.fix.as_str();
    assert!(
        message.contains("1 level inside"),
        "the message must say how deep the file sits; got {message}"
    );
    assert!(
        fix.contains("Move the file to 'tests/'"),
        "the fix must say where the file belongs; got {fix}"
    );
}

#[test]
fn a_support_prefixed_helper_is_accepted_flat() {
    // No support directory under `tests/` is sanctioned, but a support prefix
    // is legal: a shared helper sits directly under `tests/` and carries one,
    // so AES103 accepts it flat.
    assert!(audit("crates/calc/tests/mock_common.rs").is_none());
    assert!(audit("crates/calc/tests/mock_filesystem.rs").is_none());
}

#[test]
fn a_regression_guard_is_a_legal_name_even_though_it_is_not_a_category() {
    // TEST.md §2.0 requires one regression file per defect fix, so the prefix
    // must be legal. Whether the folder still owes the seven types is AES704's
    // question, not this one's.
    assert!(
        audit("crates/calc/tests/regression_scan_modes.rs").is_none(),
        "a regression guard must pass AES103; the repository's own convention \
         depends on it"
    );
}

#[test]
fn production_source_is_never_reported() {
    for path in [
        "crates/calc/src/capabilities_adder_checker.rs",
        "crates/calc/src/taxonomy_domain_vo.rs",
        "crates/calc/src/agent_calc_orchestrator.rs",
    ] {
        assert!(
            audit(path).is_none(),
            "'{path}' is production source and carries no test-type prefix; got {:?}",
            audit(path).map(|r| r.message.value().to_string())
        );
    }
}
