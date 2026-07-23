// Acceptance tests for output format support (text, json, sarif, junit).

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
fn frd_fmt_01_text_format() {
    let tmp = std::env::temp_dir().join(format!("acc_fmt_01_{}", std::process::id()));
    fs::create_dir_all(&tmp).unwrap();
    let output = cli_bin()
        .arg("check")
        .arg(tmp.to_str().unwrap())
        .arg("--format")
        .arg("text")
        .output()
        .expect("failed to run check --format text");
    assert!(output.status.success() || output.status.code() == Some(1));
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn frd_fmt_02_json_format_valid() {
    let tmp = std::env::temp_dir().join(format!("acc_fmt_02_{}", std::process::id()));
    fs::create_dir_all(&tmp).unwrap();
    let output = cli_bin()
        .arg("check")
        .arg(tmp.to_str().unwrap())
        .arg("--format")
        .arg("json")
        .output()
        .expect("failed to run check --format json");
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.trim().is_empty() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(stdout.trim());
        assert!(parsed.is_ok(), "JSON output must be valid JSON");
    }
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn frd_fmt_03_sarif_format_valid() {
    let tmp = std::env::temp_dir().join(format!("acc_fmt_03_{}", std::process::id()));
    fs::create_dir_all(&tmp).unwrap();
    let output = cli_bin()
        .arg("check")
        .arg(tmp.to_str().unwrap())
        .arg("--format")
        .arg("sarif")
        .output()
        .expect("failed to run check --format sarif");
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.trim().is_empty() {
        let parsed: serde_json::Value =
            serde_json::from_str(stdout.trim()).expect("SARIF must be valid JSON");
        assert_eq!(parsed["version"], "2.1.0", "SARIF version must be 2.1.0");
        assert!(parsed["runs"].is_array(), "SARIF must have runs array");
    }
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn frd_fmt_04_junit_format_valid() {
    let tmp = std::env::temp_dir().join(format!("acc_fmt_04_{}", std::process::id()));
    fs::create_dir_all(&tmp).unwrap();
    let output = cli_bin()
        .arg("check")
        .arg(tmp.to_str().unwrap())
        .arg("--format")
        .arg("junit")
        .output()
        .expect("failed to run check --format junit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.trim().is_empty() {
        assert!(
            stdout.contains("<?xml") || stdout.contains("<testsuites"),
            "JUnit output must contain XML declaration or testsuites element"
        );
    }
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn frd_fmt_05_invalid_format_rejected() {
    let output = cli_bin()
        .arg("check")
        .arg(".")
        .arg("--format")
        .arg("invalid_format")
        .output()
        .expect("failed to run check --format invalid");
    assert!(
        !output.status.success(),
        "invalid format should be rejected"
    );
}
