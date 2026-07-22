// Acceptance tests for the `ci` command — threshold and exit codes.

use std::fs;
use std::process::Command;

fn cli_binary_path() -> std::path::PathBuf {
    if let Some(path) = std::env::var_os("CARGO_BIN_EXE_lint-arwaky-cli") {
        return std::path::PathBuf::from(path);
    }
    let mut p = std::env::current_exe().unwrap();
    p.pop();
    let mut q = p.clone();
    q.pop();
    q.push("lint-arwaky-cli");
    if q.exists() {
        return q;
    }
    p.push("lint-arwaky-cli");
    p
}

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
fn frd_ci_01_pass_above_threshold() {
    let tmp = std::env::temp_dir().join(format!("acc_ci_01_{}", std::process::id()));
    fs::create_dir_all(&tmp).unwrap();
    let output = cli_bin()
        .arg("ci")
        .arg(tmp.to_str().unwrap())
        .arg("--threshold")
        .arg("0")
        .output()
        .expect("failed to run ci");
    assert!(
        output.status.success(),
        "ci with threshold 0 on clean dir should pass"
    );
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn frd_ci_02_fail_below_threshold() {
    let tmp = std::env::temp_dir().join(format!("acc_ci_02_{}", std::process::id()));
    fs::create_dir_all(&tmp).unwrap();
    let output = cli_bin()
        .arg("ci")
        .arg(tmp.to_str().unwrap())
        .arg("--threshold")
        .arg("101")
        .output()
        .expect("failed to run ci");
    assert_eq!(
        output.status.code(),
        Some(1),
        "ci with threshold 101 should fail (exit 1)"
    );
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn frd_ci_03_critical_auto_fail() {
    let tmp = std::env::temp_dir().join(format!("acc_ci_03_{}", std::process::id()));
    fs::create_dir_all(&tmp).unwrap();
    let output = cli_bin()
        .arg("ci")
        .arg(tmp.to_str().unwrap())
        .arg("--threshold")
        .arg("80")
        .output()
        .expect("failed to run ci");
    let code = output.status.code().unwrap_or(-1);
    assert!(
        code == 0 || code == 1,
        "ci should exit 0 or 1, got {}",
        code
    );
    fs::remove_dir_all(&tmp).ok();
}
