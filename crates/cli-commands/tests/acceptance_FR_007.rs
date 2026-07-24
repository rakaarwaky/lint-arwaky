// Acceptance tests for setup commands (init, install, mcp-config).

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
fn frd_setup_01_mcp_config_claude() {
    let output = cli_bin()
        .arg("mcp-config")
        .arg("--client")
        .arg("claude")
        .output()
        .expect("failed to run mcp-config");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("mcpServers"),
        "claude config must have mcpServers key"
    );
    assert!(
        stdout.contains("lint-arwaky"),
        "config must reference lint-arwaky"
    );
}

#[test]
fn frd_setup_02_mcp_config_cursor() {
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
fn frd_setup_03_mcp_config_windsurf() {
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
fn frd_setup_04_binary_resolution_fail_closed() {
    let output = cli_bin()
        .arg("mcp-config")
        .arg("--client")
        .arg("claude")
        .output()
        .expect("failed to run mcp-config");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Binary:"),
        "mcp-config must display resolved binary path"
    );
}
