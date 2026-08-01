// Acceptance tests for the `adapters` command.

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
fn frd_adapters_01_shows_header() {
    let output = cli_bin()
        .arg("adapters")
        .output()
        .expect("failed to run adapters");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("External lint adapters"),
        "must show header"
    );
}

#[test]
fn frd_adapters_02_shows_none_or_list() {
    let output = cli_bin()
        .arg("adapters")
        .output()
        .expect("failed to run adapters");
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Either shows "(none enabled)" or a bullet list
    assert!(
        stdout.contains("(none enabled)") || stdout.contains("  - "),
        "must show either none enabled or a bullet list"
    );
}
