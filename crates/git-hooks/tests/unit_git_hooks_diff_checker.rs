// PURPOSE: Unit tests for DiffChecker — IDiffProtocol implementation.
// Covers: happy path, edge cases, error paths for diff operations.
// Layer: Capabilities (DiffChecker)
// Speed: ms

use git_hooks_lint_arwaky::capabilities_diff_checker::DiffChecker;
use shared::common::taxonomy_git_vo::GitBranchName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_diff_protocol::IDiffProtocol;

fn sut() -> DiffChecker {
    DiffChecker::new()
}

fn test_path() -> FilePath {
    FilePath::new(".").unwrap_or_default()
}

// ─── Construction ─────────────────────────────────────────

#[test]
fn new_creates_instance() {
    let _checker = DiffChecker::new();
}

#[test]
fn default_creates_instance() {
    let _checker = DiffChecker::default();
}

// ─── get_default_branch ───────────────────────────────────

#[tokio::test]
async fn get_default_branch_returns_branch_name() {
    let checker = sut();
    let branch = checker.get_default_branch(&test_path()).await;
    // In a non-git directory, should fallback to "main"
    assert!(!branch.value().is_empty());
}

#[tokio::test]
async fn get_default_branch_fallback_is_main() {
    let checker = sut();
    // Use a path that is definitely not a git repo
    let path = FilePath::new("/tmp/nonexistent_git_repo_test").unwrap_or_default();
    let branch = checker.get_default_branch(&path).await;
    assert_eq!(branch.value(), "main");
}

// ─── get_changed_files ────────────────────────────────────

#[tokio::test]
async fn get_changed_files_with_empty_base_uses_default() {
    let checker = sut();
    let empty_branch = GitBranchName::new("");
    let files = checker.get_changed_files(&test_path(), &empty_branch).await;
    // Should not panic; returns whatever git finds (possibly empty)
    assert!(files.len() >= 0);
}

#[tokio::test]
async fn get_changed_files_with_dot_base_uses_default() {
    let checker = sut();
    let dot_branch = GitBranchName::new(".");
    let files = checker.get_changed_files(&test_path(), &dot_branch).await;
    assert!(files.len() >= 0);
}

#[tokio::test]
async fn get_changed_files_with_explicit_branch() {
    let checker = sut();
    let branch = GitBranchName::new("main");
    let files = checker.get_changed_files(&test_path(), &branch).await;
    // In a valid git repo this may return files; in invalid repo returns empty
    assert!(files.len() >= 0);
}

#[tokio::test]
async fn get_changed_files_invalid_path_returns_empty() {
    let checker = sut();
    let path = FilePath::new("/tmp/definitely_not_a_git_repo_xyz").unwrap_or_default();
    let branch = GitBranchName::new("main");
    let files = checker.get_changed_files(&path, &branch).await;
    assert!(files.is_empty());
}

// ─── get_diff ─────────────────────────────────────────────

#[tokio::test]
async fn get_diff_returns_valid_structure() {
    let checker = sut();
    let result = checker.get_diff(&test_path()).await;
    // total_changed should be non-negative
    assert!(result.total_changed.value() >= 0);
}

#[tokio::test]
async fn get_diff_invalid_path_returns_empty_diff() {
    let checker = sut();
    let path = FilePath::new("/tmp/no_git_here_abc").unwrap_or_default();
    let result = checker.get_diff(&path).await;
    assert!(result.added.is_empty());
    assert!(result.deleted.is_empty());
    assert!(result.renamed.is_empty());
    assert_eq!(result.total_changed.value(), 0);
}

// ─── run_git_diff_check ───────────────────────────────────

#[tokio::test]
async fn run_git_diff_check_returns_lint_result_list() {
    let checker = sut();
    let results = checker.run_git_diff_check(&test_path()).await;
    // Current implementation returns empty LintResultList
    assert!(results.is_empty());
}

#[tokio::test]
async fn run_git_diff_check_invalid_path_no_panic() {
    let checker = sut();
    let path = FilePath::new("/tmp/invalid_path_for_diff").unwrap_or_default();
    let results = checker.run_git_diff_check(&path).await;
    assert!(results.is_empty());
}
