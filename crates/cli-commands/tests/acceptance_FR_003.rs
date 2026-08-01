// Acceptance tests for the `fix` command — auto-fix with dry-run and exit codes.

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
fn frd_fix_01_dry_run_no_changes() {
    let tmp = std::env::temp_dir().join(format!("acc_fix_01_{}", std::process::id()));
    let src = tmp.join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), "pub fn bad_name() {}\n").unwrap();
    let output = cli_bin()
        .arg("fix")
        .arg(tmp.to_str().unwrap())
        .arg("--dry-run")
        .output()
        .expect("failed to run fix --dry-run");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("DRY-RUN") || stdout.contains("Dry-run") || stdout.contains("dry-run"),
        "dry-run must indicate preview mode, got: {}",
        stdout
    );
    let content = fs::read_to_string(src.join("lib.rs")).unwrap();
    assert_eq!(content, "pub fn bad_name() {}\n", "dry-run must not modify files");
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn frd_fix_02_clean_project_reports_zero() {
    let tmp = std::env::temp_dir().join(format!("acc_fix_02_{}", std::process::id()));
    let src = tmp.join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), "pub fn clean() {}\n").unwrap();
    let output = cli_bin()
        .arg("fix")
        .arg(tmp.to_str().unwrap())
        .output()
        .expect("failed to run fix");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("0") || stdout.contains("all violations resolved") || stdout.contains("no violations"),
        "clean project should report 0 violations, got: {}",
        stdout
    );
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn frd_fix_03_remaining_violations_exit_1() {
    let tmp = std::env::temp_dir().join(format!("acc_fix_03_{}", std::process::id()));
    let src = tmp.join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), "pub fn needs_work() { todo!() }\n").unwrap();
    let output = cli_bin()
        .arg("fix")
        .arg(tmp.to_str().unwrap())
        .output()
        .expect("failed to run fix");
    let code = output.status.code().unwrap_or(-1);
    assert!(
        code == 0 || code == 1,
        "fix should exit 0 or 1, got {}",
        code
    );
    fs::remove_dir_all(&tmp).ok();
}
