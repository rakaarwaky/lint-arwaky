// Acceptance tests for the `install` command.

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

#[test]
fn frd_install_01_shows_step_progress() {
    let output = cli_bin()
        .arg("install")
        .output()
        .expect("failed to run install");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("[1/2]") || stdout.contains("Python"),
        "install must show step progress, got: {}",
        stdout
    );
}

#[test]
fn frd_install_02_exit_code_is_0_or_1() {
    let output = cli_bin()
        .arg("install")
        .output()
        .expect("failed to run install");
    let code = output.status.code().unwrap_or(-1);
    assert!(
        code == 0 || code == 1,
        "install should exit 0 or 1, got {}",
        code
    );
}
