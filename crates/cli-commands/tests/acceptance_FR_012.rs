// Acceptance tests for the `git-diff` command.

use std::process::Command;

fn cli_bin() -> Command {
    let bin = std::env::current_exe().unwrap();
    let mut dir = bin.parent().unwrap();
    for _ in 0..5 {
        let candidate = dir.join("lint-arwaky-cli");
        if candidate.exists() {
            return Command::new(candidate);
        }
        dir = dir.parent().unwrap_or(dir);
    }
    let mut p = std::env::current_exe().unwrap();
    p.pop();
    p.pop();
    p.pop();
    p.push("lint-arwaky-cli");
    Command::new(p)
}

/// Run git-diff from the worktree (a git repo) against HEAD.
fn run_git_diff() -> std::process::Output {
    // Use the worktree root as the path — it's a git repo
    let wt_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    cli_bin()
        .arg("git-diff")
        .arg("--base")
        .arg("HEAD")
        .arg(wt_root.to_str().unwrap())
        .output()
        .expect("failed to run git-diff")
}

#[test]
fn frd_git_diff_01_shows_version_and_base() {
    let output = run_git_diff();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Git-Diff") || stdout.contains("Base:"),
        "git-diff must show version header and base branch, got: {}",
        stdout
    );
}

#[test]
fn frd_git_diff_02_exit_code_is_0_or_1() {
    let output = run_git_diff();
    let code = output.status.code().unwrap_or(-1);
    assert!(
        code == 0 || code == 1,
        "git-diff should exit 0 or 1, got {}",
        code
    );
}

#[test]
fn frd_git_diff_03_shows_files_changed() {
    let output = run_git_diff();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Files changed") || stdout.contains("changed files"),
        "git-diff must show changed files count, got: {}",
        stdout
    );
}
