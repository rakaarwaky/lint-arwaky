// Acceptance tests — AES103 test/bench file-prefix discipline.
//
// Each case maps to a user story: an agent scaffolding a test suite needs the
// linter to name the offending file, the directory it sits in, and whether the
// fix is a rename or a move. A message that names only the file leaves the
// author guessing which of the two it is.
use naming_rules_lint_arwaky::capabilities_test_file_prefix_checker::TestFilePrefixChecker;
use shared_common::taxonomy_lint_result_vo::LintResult;

/// Run the check the way the orchestrator does: the path plus its basename.
fn audit(path: &str) -> Option<LintResult> {
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
            "an agent writing {prefix}adder.rs is following the skill and must not be \\
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
    // The reader's editor shows the basename in the file column, so the message
    // carries the directory as context rather than repeating the whole path.
    assert!(
        message.contains("helpers.rs"),
        "the message must name the offending file; got {message}"
    );
    assert!(
        message.contains("tests/"),
        "the message must name the directory the file sits in; got {message}"
    );
    assert!(
        message.contains("contract_") && message.contains("acceptance_"),
        "the message must list the legal prefix set so the author can rename without \
         guessing; got {message}"
    );
    // The support set is legal too, so the listing must not imply the seven are
    // the only options — `regression_` is a name the repository relies on.
    assert!(
        message.contains("regression_"),
        "the listing must include the support prefixes; otherwise an author holding a \
         regression guard reads the message as telling them to rename it. Got {message}"
    );
}

#[test]
fn a_benchmark_named_inside_tests_says_move_it() {
    // The prefix is legal, just in the other directory — so the fix is a move.
    let message = audit("crates/calc/tests/bench_adder.rs")
        .expect("must fire")
        .message
        .value()
        .to_string();
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
fn a_test_named_inside_benches_says_move_it() {
    let message = audit("crates/calc/benches/unit_adder.rs")
        .expect("must fire")
        .message
        .value()
        .to_string();
    assert!(
        message.contains("tests/") && message.contains("Move the file"),
        "a test prefix inside benches/ needs a move; got {message}"
    );
}

#[test]
fn a_subdirectory_under_tests_says_how_deep_and_how_to_flatten() {
    let message = audit("crates/calc/tests/inner/unit_adder.rs")
        .expect("must fire")
        .message
        .value()
        .to_string();
    assert!(
        message.contains("1 level(s) below"),
        "the message must say how deep the file sits; got {message}"
    );
    assert!(
        message.contains("must stay flat"),
        "the message must state the flat-layout rule; got {message}"
    );
}

#[test]
fn a_support_directory_is_accepted_so_shared_helpers_survive() {
    // Rust reaches shared support code through `mod common;`, so a support
    // module cannot sit flat beside the test files without colliding.
    assert!(audit("crates/calc/tests/common/mod.rs").is_none());
    assert!(audit("crates/calc/tests/common/mock_filesystem.rs").is_none());
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
