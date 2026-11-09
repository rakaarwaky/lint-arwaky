// BE-5-02 — behavioural tests for `git` and `fix`.
//
// Both verbs had zero behavioural coverage: `collect_git_diff` and
// `handle_fix` appeared in the test suite only as `let _ = fn;` compile-time
// references. The git ref validator in particular had no test at all, and it is
// the only guard between the MCP `base` argument and a `git` argument list.
use dispatcher_lint_arwaky::surface_fix_action::FixReport;
use dispatcher_lint_arwaky::surface_git_action::validate_git_base_ref;
use shared_common::ExitCode;

#[test]
fn git_base_ref_accepts_ordinary_refs() {
    for reference in ["develop", "main", "origin/main", "release/1.2.3"] {
        assert!(
            validate_git_base_ref(reference).is_ok(),
            "{reference} must be accepted"
        );
    }
    // The allow-list is deliberately conservative: `git diff` also accepts
    // `~`, `^` and `@{}`, but rejecting them costs the caller one flag and
    // closes a class of refs that are easy to get wrong.
    for reference in ["HEAD~1", "main^", "main@{1}"] {
        assert!(
            validate_git_base_ref(reference).is_err(),
            "{reference} must be rejected: not in the allow-list"
        );
    }
}

#[test]
fn git_base_ref_rejects_option_injection() {
    // The whole point of the check: a ref that starts with '-' would be parsed
    // by git as an option, e.g. `--upload-pack=…` turns a diff into code
    // execution.
    for reference in ["--upload-pack=/bin/sh", "-c", "-"] {
        let error =
            validate_git_base_ref(reference).expect_err("an option-shaped ref must be rejected");
        assert!(
            error.contains("'-'"),
            "the error must name the reason for {reference}: {error}"
        );
    }
}

#[test]
fn git_base_ref_rejects_shell_metacharacters() {
    for reference in [
        "main; rm -rf /",
        "main$(whoami)",
        "main`id`",
        "main|cat",
        "main&bg",
    ] {
        assert!(
            validate_git_base_ref(reference).is_err(),
            "{reference} must be rejected"
        );
    }
}

#[test]
fn git_base_ref_rejects_empty() {
    let error = validate_git_base_ref("").expect_err("empty ref must be rejected");
    assert!(error.contains("empty"), "{error}");
}

/// The fix verdict is the single exit-code decision shared by the CLI, the MCP
/// action and the TUI. These cases pin the mapping; they do not need a running
/// fixer because `verdict` is a pure function of the report.
fn report(dry_run: bool, remaining_fixable: usize, introduced: usize) -> FixReport {
    let mut report = FixReport {
        project_path: ".".to_string(),
        dry_run,
        before_count: 0,
        after_count: 0,
        fixed_count: 0,
        output: String::new(),
        success: false,
        has_failed: false,
        fixable: Vec::new(),
        resolved: Vec::new(),
        introduced: Vec::new(),
        remaining_fixable,
    };
    report.success = introduced == 0 && remaining_fixable == 0;
    report
}

#[test]
fn fix_verdict_is_ok_when_everything_was_fixed() {
    let report = report(false, 0, 0);
    assert_eq!(report.verdict(), ExitCode::OK);
}

#[test]
fn fix_verdict_fails_policy_when_fixable_violations_remain() {
    // The CLI and MCP used to disagree here: the CLI answered 1, MCP answered
    // 0, so an agent gating on exit_code concluded the tree was clean.
    let report = report(false, 3, 0);
    assert_eq!(report.verdict(), ExitCode::POLICY_FAIL);
}

#[test]
fn fix_verdict_fails_policy_when_the_run_introduced_a_violation() {
    // A count-based delta called this a no-op: one resolved, one introduced,
    // same total.
    let report = report(false, 0, 1);
    assert_eq!(report.verdict(), ExitCode::POLICY_FAIL);
}

#[test]
fn fix_verdict_is_ok_for_a_preview() {
    // A dry run never writes, so findings that remain are not a failure.
    let report = report(true, 5, 0);
    assert_eq!(report.verdict(), ExitCode::OK);
}

#[test]
fn fix_verdict_runtime_error_outranks_policy() {
    let mut report = report(false, 4, 2);
    report.has_failed = true;
    assert_eq!(
        report.verdict(),
        ExitCode::RUNTIME_ERROR,
        "a Failed(reason) from the fixer is a runtime error, not a policy failure"
    );
}

// ─── BE-3-03: fix discovery must honour ignored paths ──────────────────────

/// Build a real filesystem aggregate over a scratch tree.
fn fs_aggregate()
-> std::sync::Arc<dyn shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate> {
    filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator()
}

#[test]
fn build_fix_entries_skips_an_ignored_directory() {
    // The old walker used `std::fs::read_dir` on a hard-coded skip list and
    // never consulted `ignored_paths`, so a fix run rewrote code in a tree the
    // operator had explicitly excluded — including vendored or generated code.
    let dir = tempfile::tempdir().expect("temp dir");
    let src = dir.path().join("src");
    let fixtures = dir.path().join("fixtures");
    std::fs::create_dir_all(&src).expect("src");
    std::fs::create_dir_all(&fixtures).expect("fixtures");
    std::fs::write(src.join("lib.rs"), "pub fn good_name() {}\n").expect("src file");
    std::fs::write(fixtures.join("bad.rs"), "pub fn BadName() {}\n").expect("fixture file");

    let root = shared_common::FilePath::new(dir.path().to_string_lossy()).expect("root");
    let ignored = vec!["fixtures".to_string()];

    let entries = dispatcher_lint_arwaky::surface_fix_action::build_fix_entries(
        &fs_aggregate(),
        &root,
        &ignored,
    );
    let paths: Vec<String> = entries
        .iter()
        .map(|e| e.path.to_string_lossy().to_string())
        .collect();

    assert!(
        paths.iter().any(|p| p.ends_with("src/lib.rs")),
        "source files must be discovered: {paths:?}"
    );
    assert!(
        paths.iter().all(|p| !p.contains("fixtures")),
        "an ignored directory must never be linted or fixed: {paths:?}"
    );
}

#[test]
fn build_fix_entries_returns_nothing_for_a_missing_root() {
    let root = shared_common::FilePath::new("/nonexistent/fix-root-xyz").expect("root");
    let entries =
        dispatcher_lint_arwaky::surface_fix_action::build_fix_entries(&fs_aggregate(), &root, &[]);
    assert!(entries.is_empty());
}
