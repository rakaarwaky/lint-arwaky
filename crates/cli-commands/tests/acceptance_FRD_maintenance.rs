// Acceptance tests for maintenance commands (doctor, security, dependencies).

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
    p.pop();
    p.pop();
    p.pop();
    p.push("lint-arwaky-cli");
    Command::new(p)
}

#[test]
fn frd_maint_01_doctor_always_exit_0() {
    let output = cli_bin()
        .arg("doctor")
        .output()
        .expect("failed to run doctor");
    assert_eq!(
        output.status.code(),
        Some(0),
        "doctor must always exit 0 (diagnostic only)"
    );
}

#[test]
fn frd_maint_02_doctor_checks_tools() {
    let output = cli_bin()
        .arg("doctor")
        .output()
        .expect("failed to run doctor");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Rust Toolchain"),
        "doctor must check Rust tools"
    );
    assert!(
        stdout.contains("Python Toolchain"),
        "doctor must check Python tools"
    );
    assert!(
        stdout.contains("JavaScript Toolchain"),
        "doctor must check JS tools"
    );
    assert!(stdout.contains("VCS"), "doctor must check VCS tools");
}

#[test]
fn frd_maint_03_dependencies_report() {
    let output = cli_bin()
        .arg("dependencies")
        .arg(".")
        .output()
        .expect("failed to run dependencies");
    let code = output.status.code().unwrap_or(-1);
    assert!(
        code == 0 || code == 2,
        "dependencies should exit 0 or 2, got {}",
        code
    );
    if code == 0 {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Dependency Report"),
            "must show dependency report header"
        );
    }
}
