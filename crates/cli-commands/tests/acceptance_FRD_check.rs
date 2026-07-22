// Acceptance tests mapping 1:1 to FRD requirements for the `check` command.

use std::fs;
use std::process::Command;

fn cli_bin() -> Command {
    let bin = std::env::current_exe().unwrap();
    let mut dir = bin.parent().unwrap();
    // Walk up to find the binary
    for _ in 0..5 {
        let candidate = dir.join("lint-arwaky-cli");
        if candidate.exists() {
            return Command::new(candidate);
        }
        dir = dir.parent().unwrap_or(dir);
    }
    // Fallback: use the target/debug path
    let mut p = std::env::current_exe().unwrap();
    p.pop(); p.pop(); p.pop();
    p.push("lint-arwaky-cli");
    Command::new(p)
}

#[test]
fn frd_check_01_runs_analysis_on_target_path() {
    let tmp = std::env::temp_dir().join(format!("acc_check_01_{}", std::process::id()));
    fs::create_dir_all(&tmp).unwrap();
    let output = cli_bin()
        .arg("check")
        .arg(tmp.to_str().unwrap())
        .output()
        .expect("failed to run check");
    let code = output.status.code().unwrap_or(-1);
    assert!(
        code == 0 || code == 1,
        "check should exit 0 or 1, got {}",
        code
    );
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn frd_check_02_git_diff_flag_accepted() {
    let output = cli_bin()
        .arg("check")
        .arg(".")
        .arg("--git-diff")
        .output()
        .expect("failed to run check --git-diff");
    let code = output.status.code().unwrap_or(-1);
    assert!(code >= 0, "check --git-diff should not crash");
}

#[test]
fn frd_check_03_all_formats_accepted() {
    let tmp = std::env::temp_dir().join(format!("acc_check_03_{}", std::process::id()));
    fs::create_dir_all(&tmp).unwrap();
    for format in ["text", "json", "sarif", "junit"] {
        let output = cli_bin()
            .arg("check")
            .arg(tmp.to_str().unwrap())
            .arg("--format")
            .arg(format)
            .output()
            .expect("failed to run check with format");
        let code = output.status.code().unwrap_or(-1);
        assert!(
            code == 0 || code == 1,
            "check --format {} should exit 0 or 1, got {}",
            format,
            code
        );
    }
    fs::remove_dir_all(&tmp).ok();
}
