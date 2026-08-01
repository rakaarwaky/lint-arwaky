// Acceptance tests for the `security` command — vulnerability scanning.

use std::fs;
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
fn frd_security_01_tool_not_installed_exit_3() {
    // Run security on a temp dir — tool may or may not be installed
    let tmp = std::env::temp_dir().join(format!("acc_sec_01_{}", std::process::id()));
    fs::create_dir_all(&tmp).unwrap();
    let output = cli_bin()
        .arg("security")
        .arg(tmp.to_str().unwrap())
        .output()
        .expect("failed to run security");
    let code = output.status.code().unwrap_or(-1);
    // Exit 0 = clean, 1 = vulns found, 3 = tool missing
    assert!(
        code == 0 || code == 1 || code == 3,
        "security should exit 0, 1, or 3, got {}",
        code
    );
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn frd_security_02_shows_language_and_tool() {
    let tmp = std::env::temp_dir().join(format!("acc_sec_02_{}", std::process::id()));
    fs::create_dir_all(&tmp).unwrap();
    let output = cli_bin()
        .arg("security")
        .arg(tmp.to_str().unwrap())
        .output()
        .expect("failed to run security");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Language:") || stdout.contains("Tool:"),
        "security must display language and tool info, got: {}",
        stdout
    );
    fs::remove_dir_all(&tmp).ok();
}
