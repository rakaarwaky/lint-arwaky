// E2E tests — fix command lifecycle.

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
fn e2e_fix_dry_run_no_modification() {
    let tmp = std::env::temp_dir().join(format!("e2e_fix_dry_{}", std::process::id()));
    let src = tmp.join("src");
    fs::create_dir_all(&src).unwrap();
    let content = "pub fn hello() { println!(\"hi\"); }\n";
    let file_path = src.join("main.rs");
    fs::write(&file_path, content).unwrap();
    let output = cli_bin()
        .arg("fix")
        .arg(tmp.to_str().unwrap())
        .arg("--dry-run")
        .output()
        .expect("failed to run fix");
    let after = fs::read_to_string(&file_path).unwrap();
    assert_eq!(after, content, "dry-run must not modify files");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("DRY-RUN") || stdout.contains("dry-run"));
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn e2e_fix_clean_directory() {
    let tmp = std::env::temp_dir().join(format!("e2e_fix_clean_{}", std::process::id()));
    fs::create_dir_all(&tmp).unwrap();
    let output = cli_bin()
        .arg("fix")
        .arg(tmp.to_str().unwrap())
        .output()
        .expect("failed to run fix");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("0 violations") || stdout.contains("Fix complete"),
        "fix on clean dir should report no violations: {}",
        stdout
    );
    fs::remove_dir_all(&tmp).ok();
}
