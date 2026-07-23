// E2E tests — full check/scan lifecycle through the real CLI binary.

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
fn e2e_check_clean_directory_exit_0() {
    let tmp = std::env::temp_dir().join(format!("e2e_clean_{}", std::process::id()));
    fs::create_dir_all(&tmp).unwrap();
    let output = cli_bin()
        .arg("check")
        .arg(tmp.to_str().unwrap())
        .output()
        .expect("failed to run check");
    assert!(output.status.success(), "check on clean dir should exit 0");
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn e2e_check_nonexistent_path_exit_2() {
    let output = cli_bin()
        .arg("check")
        .arg("/nonexistent/path/xyz123")
        .output()
        .expect("failed to run check");
    assert_eq!(
        output.status.code(),
        Some(2),
        "check on nonexistent path should exit 2"
    );
}

#[test]
fn e2e_scan_rust_file_with_violations() {
    let tmp = std::env::temp_dir().join(format!("e2e_violations_{}", std::process::id()));
    let src = tmp.join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("BadName.rs"), "pub fn hello() {}\n").unwrap();
    let output = cli_bin()
        .arg("scan")
        .arg(tmp.to_str().unwrap())
        .output()
        .expect("failed to run scan");
    let code = output.status.code().unwrap_or(-1);
    assert!(
        code == 0 || code == 1,
        "scan should exit 0 or 1, got: {}",
        code
    );
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn e2e_scan_json_format() {
    let tmp = std::env::temp_dir().join(format!("e2e_json_{}", std::process::id()));
    fs::create_dir_all(&tmp).unwrap();
    let output = cli_bin()
        .arg("scan")
        .arg(tmp.to_str().unwrap())
        .arg("--format")
        .arg("json")
        .output()
        .expect("failed to run scan");
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.trim().is_empty() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(stdout.trim());
        assert!(
            parsed.is_ok(),
            "JSON output should be parseable: {}",
            stdout
        );
    }
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn e2e_scan_sarif_format() {
    let tmp = std::env::temp_dir().join(format!("e2e_sarif_{}", std::process::id()));
    fs::create_dir_all(&tmp).unwrap();
    let output = cli_bin()
        .arg("scan")
        .arg(tmp.to_str().unwrap())
        .arg("--format")
        .arg("sarif")
        .output()
        .expect("failed to run scan");
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.trim().is_empty() {
        let parsed: serde_json::Value =
            serde_json::from_str(stdout.trim()).expect("SARIF output should be valid JSON");
        assert_eq!(parsed["version"], "2.1.0");
    }
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn e2e_scan_with_filter() {
    let tmp = std::env::temp_dir().join(format!("e2e_filter_{}", std::process::id()));
    fs::create_dir_all(&tmp).unwrap();
    let output = cli_bin()
        .arg("scan")
        .arg(tmp.to_str().unwrap())
        .arg("--filter")
        .arg("AES999")
        .output()
        .expect("failed to run scan");
    assert!(output.status.success());
    fs::remove_dir_all(&tmp).ok();
}
