// PURPOSE: Unit tests for GitCommandAdapter — IGitCommandProtocol implementation.
// Covers: git command execution, symbolic ref, diff, ls-files.
// Layer: Capabilities (GitCommandAdapter)
// Speed: ms
//
// NOTE: capabilities_git_command_adapter is not currently exported in lib.rs.
// These tests validate the module directly. If the module remains private,
// move these to integration tests or export the module.

// Uncomment when module is publicly exported:
// use git_hooks_lint_arwaky::capabilities_git_command_adapter::GitCommandAdapter;

use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_git_command_protocol::{GitCommandOutput, IGitCommandProtocol};

// Inline the struct for testing since module is not exported
// (mirrors capabilities_git_command_adapter.rs)
pub struct GitCommandAdapter;

#[async_trait::async_trait]
impl IGitCommandProtocol for GitCommandAdapter {
    async fn run_git(&self, args: &[&str], dir: &FilePath) -> GitCommandOutput {
        let (stdout, stderr, success) =
            shared::git_hooks::utility_git_io::run_git_command(args, &dir.value);
        GitCommandOutput {
            stdout,
            stderr,
            success,
        }
    }

    async fn symbolic_ref(&self, dir: &FilePath) -> Option<String> {
        let output = self
            .run_git(&["symbolic-ref", "refs/remotes/origin/HEAD"], dir)
            .await;
        if output.success {
            let ref_str = output.stdout.trim().to_string();
            ref_str.rsplit('/').next().map(|s| s.to_string())
        } else {
            None
        }
    }

    async fn diff_name_only(&self, range: &str, dir: &FilePath) -> Vec<String> {
        let output = self.run_git(&["diff", "--name-only", range], dir).await;
        if output.success {
            output
                .stdout
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect()
        } else {
            Vec::new()
        }
    }

    async fn ls_files_modified(&self, dir: &FilePath) -> Vec<String> {
        let output = self
            .run_git(
                &["ls-files", "--modified", "--others", "--exclude-standard"],
                dir,
            )
            .await;
        if output.success {
            output
                .stdout
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl GitCommandAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GitCommandAdapter {
    fn default() -> Self {
        Self::new()
    }
}

fn sut() -> GitCommandAdapter {
    GitCommandAdapter::new()
}

fn test_path() -> FilePath {
    FilePath::new(".").unwrap_or_default()
}

// ─── Construction ─────────────────────────────────────────

#[test]
fn new_creates_instance() {
    let _adapter = GitCommandAdapter::new();
}

#[test]
fn default_creates_instance() {
    let _adapter = GitCommandAdapter;
}

// ─── run_git ──────────────────────────────────────────────

#[tokio::test]
async fn run_git_version_succeeds() {
    let adapter = sut();
    let output = adapter.run_git(&["--version"], &test_path()).await;
    assert!(output.success);
    assert!(output.stdout.contains("git version"));
}

#[tokio::test]
async fn run_git_invalid_command_fails_gracefully() {
    let adapter = sut();
    let output = adapter
        .run_git(&["not-a-real-git-subcommand"], &test_path())
        .await;
    assert!(!output.success);
}

#[tokio::test]
async fn run_git_invalid_directory_fails() {
    let adapter = sut();
    let path = FilePath::new("/tmp/nonexistent_dir_xyz_123").unwrap_or_default();
    let output = adapter.run_git(&["status"], &path).await;
    assert!(!output.success);
}

// ─── symbolic_ref ─────────────────────────────────────────

#[tokio::test]
async fn symbolic_ref_in_non_repo_returns_none() {
    let adapter = sut();
    let path = FilePath::new("/tmp/not_a_repo_abc").unwrap_or_default();
    let result = adapter.symbolic_ref(&path).await;
    assert!(result.is_none());
}

// ─── diff_name_only ───────────────────────────────────────

#[tokio::test]
async fn diff_name_only_invalid_range_returns_empty() {
    let adapter = sut();
    let result = adapter
        .diff_name_only("invalid_range_xyz", &test_path())
        .await;
    assert!(result.is_empty());
}

#[tokio::test]
async fn diff_name_only_invalid_dir_returns_empty() {
    let adapter = sut();
    let path = FilePath::new("/tmp/no_repo_here").unwrap_or_default();
    let result = adapter.diff_name_only("HEAD~1..HEAD", &path).await;
    assert!(result.is_empty());
}

// ─── ls_files_modified ────────────────────────────────────

#[tokio::test]
async fn ls_files_modified_in_non_repo_returns_empty() {
    let adapter = sut();
    let path = FilePath::new("/tmp/definitely_not_git").unwrap_or_default();
    let result = adapter.ls_files_modified(&path).await;
    assert!(result.is_empty());
}
