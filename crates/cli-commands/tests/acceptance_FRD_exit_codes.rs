// Acceptance tests for standardized exit codes across all commands.

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
fn frd_exit_01_success_no_violations() {
    let tmp = std::env::temp_dir().join(format!("acc_exit_01_{}", std::process::id()));
    fs::create_dir_all(&tmp).unwrap();
    let output = cli_bin()
        .arg("check")
        .arg(tmp.to_str().unwrap())
        .output()
        .expect("failed to run check");
    assert_eq!(output.status.code(), Some(0), "clean dir should exit 0");
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn frd_exit_02_system_error_nonexistent_path() {
    let output = cli_bin()
        .arg("check")
        .arg("/nonexistent/path/xyz")
        .output()
        .expect("failed to run check");
    assert_eq!(
        output.status.code(),
        Some(2),
        "nonexistent path should exit 2"
    );
}

#[test]
fn frd_exit_03_tool_missing_security() {
    let tmp = std::env::temp_dir().join(format!("acc_exit_03_{}", std::process::id()));
    fs::create_dir_all(&tmp).unwrap();
    let output = cli_bin()
        .arg("security")
        .arg(tmp.to_str().unwrap())
        .output()
        .expect("failed to run security");
    let code = output.status.code().unwrap_or(-1);
    assert!(
        code == 0 || code == 1 || code == 3,
        "security should exit 0, 1, or 3, got {}",
        code
    );
    fs::remove_dir_all(&tmp).ok();
}
