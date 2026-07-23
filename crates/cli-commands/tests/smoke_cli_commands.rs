// Smoke test — verify the CLI binary boots and responds to basic commands.

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
fn cli_binary_responds_to_version() {
    let output = cli_bin()
        .arg("version")
        .output()
        .expect("failed to execute CLI binary");
    assert!(
        output.status.success(),
        "version command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Lint Arwaky") || stdout.contains("lint-arwaky"),
        "unexpected version output: {}",
        stdout
    );
}

#[test]
fn cli_binary_responds_to_help() {
    let output = cli_bin()
        .arg("--help")
        .output()
        .expect("failed to execute CLI binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("check") || stdout.contains("scan"));
}

#[test]
fn cli_binary_adapters_command() {
    let output = cli_bin()
        .arg("adapters")
        .output()
        .expect("failed to execute CLI binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("External lint adapters"));
}

#[test]
fn cli_binary_doctor_command() {
    let output = cli_bin()
        .arg("doctor")
        .output()
        .expect("failed to execute CLI binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Environment Diagnostics"));
}
