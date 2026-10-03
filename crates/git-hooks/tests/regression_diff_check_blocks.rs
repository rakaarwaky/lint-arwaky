// Regression tests — issue #582: the pre-commit diff check must run real AES
// analysis over changed files. A file with a real violation produces a
// non-empty result list (which blocks the commit), and an analysis failure is
// reported distinctly from violations via its own code.

use git_hooks_lint_arwaky::capabilities_diff_checker::ANALYSIS_FAILURE_CODE;
use git_hooks_lint_arwaky::root_git_hooks_container::GitContainer;
use shared_common::FilePath;
use shared_git_hooks::GitHooksRequest;
use shared_git_hooks::contract_git_hooks_aggregate::IGitHooksAggregate;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use tempfile::TempDir;

// ─── Helpers ──────────────────────────────────────────────

fn git(dir: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .output()
        .expect("git must be available");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Fresh repository with one baseline commit, so untracked/modified files are
/// reported by the diff checker's `git ls-files` fallback chain.
fn make_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    git(dir, &["init", "-b", "main", "."]);
    git(
        dir,
        &[
            "-c",
            "user.email=test@example.com",
            "-c",
            "user.name=test",
            "commit",
            "--allow-empty",
            "-m",
            "baseline",
        ],
    );
    tmp
}

fn run_check(tmp: &TempDir) -> shared_cli_commands::LintResultList {
    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let linter = quality_rules::root_quality_rules_container::CodeAnalysisContainer::new()
        .code_analysis_linter();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let container = GitContainer::new(fp.clone(), fc.orchestrator(), fc.io(), linter);
    let aggregate: Arc<dyn IGitHooksAggregate> = container.aggregate();
    aggregate
        .execute(GitHooksRequest::run_check(&fp))
        .into_results()
}

// ─── Acceptance: a real violation blocks the hook ─────────

#[test]
fn changed_file_with_aes_violation_produces_blocking_results() {
    let tmp = make_repo();
    std::fs::create_dir_all(tmp.path().join("src")).unwrap();
    // AES304 bypass pattern (`unwrap`) in an untracked changed file.
    std::fs::write(
        tmp.path().join("src/bad.rs"),
        "pub fn f(v: Option<i32>) -> i32 {\n    v.unwrap()\n}\n",
    )
    .unwrap();

    let results = run_check(&tmp);
    assert!(
        !results.is_empty(),
        "issue #582: diff check must report violations, not an empty list"
    );
    assert!(
        results.iter().any(|r| r.file.value.contains("bad.rs")),
        "violation should name the changed file, got: {:?}",
        results.iter().map(|r| &r.file.value).collect::<Vec<_>>()
    );
}

#[test]
fn clean_changed_file_produces_no_violations() {
    let tmp = make_repo();
    std::fs::create_dir_all(tmp.path().join("src")).unwrap();
    std::fs::write(
        tmp.path().join("src/good.rs"),
        "pub fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n",
    )
    .unwrap();

    let results = run_check(&tmp);
    let violations: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() != ANALYSIS_FAILURE_CODE)
        .collect();
    assert!(
        violations.is_empty(),
        "clean file should produce no violations, got: {violations:?}"
    );
}

// ─── Acceptance: analysis failure reported distinctly ─────

#[test]
fn unreadable_changed_file_reports_analysis_failure_not_silence() {
    let tmp = make_repo();
    std::fs::create_dir_all(tmp.path().join("src")).unwrap();
    // Dangling symlink: git lists it as untracked, reading it fails.
    std::os::unix::fs::symlink("../nonexistent.rs", tmp.path().join("src/broken.rs")).unwrap();

    let results = run_check(&tmp);
    assert!(
        results
            .iter()
            .any(|r| r.code.code() == ANALYSIS_FAILURE_CODE),
        "a failed analysis must be reported with {ANALYSIS_FAILURE_CODE}, got: {:?}",
        results
            .iter()
            .map(|r| (r.code.code().to_string(), r.message.value().to_string()))
            .collect::<Vec<_>>()
    );
}
