// Acceptance tests for the `scan` command — multi-workspace discovery.

use std::fs;
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
    p.pop();
    p.pop();
    p.pop();
    p.push("lint-arwaky-cli");
    Command::new(p)
}

#[test]
fn frd_scan_01_workspace_discovery() {
    let tmp = std::env::temp_dir().join(format!("acc_scan_01_{}", std::process::id()));
    let crate_a = tmp.join("crates").join("crate-a").join("src");
    let crate_b = tmp.join("crates").join("crate-b").join("src");
    fs::create_dir_all(&crate_a).unwrap();
    fs::create_dir_all(&crate_b).unwrap();
    fs::write(
        tmp.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/*\"]\n",
    )
    .unwrap();
    fs::write(crate_a.join("lib.rs"), "pub fn a() {}\n").unwrap();
    fs::write(crate_b.join("lib.rs"), "pub fn b() {}\n").unwrap();
    let output = cli_bin()
        .arg("scan")
        .arg(tmp.to_str().unwrap())
        .output()
        .expect("failed to run scan");
    let code = output.status.code().unwrap_or(-1);
    assert!(
        code == 0 || code == 1,
        "scan should exit 0 or 1, got {}",
        code
    );
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn frd_scan_02_member_filter() {
    let tmp = std::env::temp_dir().join(format!("acc_scan_02_{}", std::process::id()));
    let crate_a = tmp.join("crates").join("crate-a").join("src");
    fs::create_dir_all(&crate_a).unwrap();
    fs::write(
        tmp.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/*\"]\n",
    )
    .unwrap();
    fs::write(crate_a.join("lib.rs"), "pub fn a() {}\n").unwrap();
    let output = cli_bin()
        .arg("scan")
        .arg(tmp.to_str().unwrap())
        .arg("--member")
        .arg("crate-a")
        .output()
        .expect("failed to run scan --member");
    let code = output.status.code().unwrap_or(-1);
    assert!(
        code == 0 || code == 1,
        "scan --member should exit 0 or 1, got {}",
        code
    );
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn frd_scan_03_nonexistent_member_exit_2() {
    let tmp = std::env::temp_dir().join(format!("acc_scan_03_{}", std::process::id()));
    let crate_a = tmp.join("crates").join("crate-a").join("src");
    fs::create_dir_all(&crate_a).unwrap();
    fs::write(
        tmp.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/*\"]\n",
    )
    .unwrap();
    fs::write(crate_a.join("lib.rs"), "pub fn a() {}\n").unwrap();
    let output = cli_bin()
        .arg("scan")
        .arg(tmp.to_str().unwrap())
        .arg("--member")
        .arg("nonexistent-crate")
        .output()
        .expect("failed to run scan --member");
    assert_eq!(
        output.status.code(),
        Some(2),
        "scan --member nonexistent should exit 2"
    );
    fs::remove_dir_all(&tmp).ok();
}
