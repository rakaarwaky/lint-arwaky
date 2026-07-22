// Smoke test — verify the CLI binary boots and responds to basic commands.

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
    // Try relative to current_exe: target/debug/ or target/debug/deps/
    let mut p = std::env::current_exe().unwrap();
    p.pop();
    // Try target/debug/lint-arwaky-cli (pop test bin name + deps, push bin)
    let mut q = p.clone();
    q.pop();
    q.push("lint-arwaky-cli");
    if q.exists() {
        return q;
    }
    p.push("lint-arwaky-cli");
    p
}

#[test]
fn cli_binary_responds_to_version() {
    let output = Command::new(cli_bin())
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
        stdout.contains("1.10.106") || stdout.contains("lint-arwaky"),
        "unexpected version output: {}",
        stdout
    );
}

#[test]
fn cli_binary_responds_to_help() {
    let output = Command::new(cli_bin())
        .arg("--help")
        .output()
        .expect("failed to execute CLI binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("check") || stdout.contains("scan"));
}

#[test]
fn cli_binary_adapters_command() {
    let output = Command::new(cli_bin())
        .arg("adapters")
        .output()
        .expect("failed to execute CLI binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("External lint adapters"));
}

#[test]
fn cli_binary_doctor_command() {
    let output = Command::new(cli_bin())
        .arg("doctor")
        .output()
        .expect("failed to execute CLI binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Environment Diagnostics"));
}
