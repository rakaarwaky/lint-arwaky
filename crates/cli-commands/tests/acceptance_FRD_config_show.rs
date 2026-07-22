// Acceptance tests for config-show command and secret redaction.

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
fn frd_config_01_shows_config() {
    let output = cli_bin()
        .arg("config-show")
        .output()
        .expect("failed to run config-show");
    let code = output.status.code().unwrap_or(-1);
    assert!(code == 0, "config-show should exit 0, got {}", code);
}

#[test]
fn frd_config_02_no_aws_key_leak() {
    let output = cli_bin()
        .arg("config-show")
        .output()
        .expect("failed to run config-show");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("AKIA"), "config-show must redact AWS keys");
}

#[test]
fn frd_config_03_no_base64_leak() {
    let output = cli_bin()
        .arg("config-show")
        .output()
        .expect("failed to run config-show");
    let stdout = String::from_utf8_lossy(&output.stdout);
    for word in stdout.split_whitespace() {
        if word.len() >= 40
            && word
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '/' | '+' | '='))
        {
            panic!("config-show leaked a potential secret: {}...", &word[..20]);
        }
    }
}
