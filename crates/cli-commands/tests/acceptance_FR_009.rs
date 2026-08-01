// Acceptance tests for the `mcp-config` command.

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
fn frd_mcp_01_claude_config() {
    let output = cli_bin()
        .arg("mcp-config")
        .arg("--client")
        .arg("claude")
        .output()
        .expect("failed to run mcp-config");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("mcpServers"), "must have mcpServers key");
    assert!(stdout.contains("lint-arwaky"), "must reference lint-arwaky");
}

#[test]
fn frd_mcp_02_cursor_config() {
    let output = cli_bin()
        .arg("mcp-config")
        .arg("--client")
        .arg("cursor")
        .output()
        .expect("failed to run mcp-config");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("mcpServers"));
}

#[test]
fn frd_mcp_03_windsurf_config() {
    let output = cli_bin()
        .arg("mcp-config")
        .arg("--client")
        .arg("windsurf")
        .output()
        .expect("failed to run mcp-config");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("config:lint-arwaky"),
        "windsurf uses config: prefix"
    );
}

#[test]
fn frd_mcp_04_copilot_config() {
    let output = cli_bin()
        .arg("mcp-config")
        .arg("--client")
        .arg("copilot")
        .output()
        .expect("failed to run mcp-config");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("server"), "copilot uses server key");
}

#[test]
fn frd_mcp_05_binary_resolution_displayed() {
    let output = cli_bin()
        .arg("mcp-config")
        .arg("--client")
        .arg("claude")
        .output()
        .expect("failed to run mcp-config");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Binary:"),
        "must display resolved binary path"
    );
}

#[test]
fn frd_mcp_06_unknown_client_falls_back() {
    let output = cli_bin()
        .arg("mcp-config")
        .arg("--client")
        .arg("unknown-client")
        .output()
        .expect("failed to run mcp-config");
    assert!(
        output.status.success(),
        "unknown client should still succeed with default format"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("mcpServers"), "must produce valid mcpServers JSON");
}
