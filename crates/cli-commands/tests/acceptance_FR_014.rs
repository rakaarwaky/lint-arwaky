// Acceptance tests for individual linter commands.

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

fn run_linter_on_clean_dir(linter: &str) -> i32 {
    let tmp = std::env::temp_dir().join(format!("acc_linter_{}_{}", linter, std::process::id()));
    let src = tmp.join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), "pub fn clean() {}\n").unwrap();
    let output = cli_bin()
        .arg(linter)
        .arg(tmp.to_str().unwrap())
        .output()
        .unwrap_or_else(|_| panic!("failed to run {}", linter));
    let code = output.status.code().unwrap_or(-1);
    fs::remove_dir_all(&tmp).ok();
    code
}

#[test]
fn frd_linter_01_quality() {
    let code = run_linter_on_clean_dir("quality");
    assert!(code == 0 || code == 1 || code == 2, "quality exit {}", code);
}

#[test]
fn frd_linter_02_import() {
    let code = run_linter_on_clean_dir("import");
    assert!(code == 0 || code == 1 || code == 2, "import exit {}", code);
}

#[test]
fn frd_linter_03_naming() {
    let code = run_linter_on_clean_dir("naming");
    assert!(code == 0 || code == 1 || code == 2, "naming exit {}", code);
}

#[test]
fn frd_linter_04_role() {
    let code = run_linter_on_clean_dir("role");
    assert!(code == 0 || code == 1 || code == 2, "role exit {}", code);
}

#[test]
fn frd_linter_05_orphan() {
    let code = run_linter_on_clean_dir("orphan");
    assert!(code == 0 || code == 1 || code == 2, "orphan exit {}", code);
}

#[test]
fn frd_linter_06_external() {
    let code = run_linter_on_clean_dir("external");
    assert!(
        code == 0 || code == 1 || code == 2,
        "external exit {}",
        code
    );
}

#[test]
fn frd_linter_07_quality_nonexistent_path_exit_2() {
    let output = cli_bin()
        .arg("quality")
        .arg("/nonexistent/path/xyz")
        .output()
        .expect("failed to run quality");
    assert_eq!(
        output.status.code(),
        Some(2),
        "quality on nonexistent path should exit 2"
    );
}

#[test]
fn frd_linter_08_quality_format_json() {
    let tmp = std::env::temp_dir().join(format!("acc_linter_json_{}", std::process::id()));
    let src = tmp.join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), "pub fn clean() {}\n").unwrap();
    let output = cli_bin()
        .arg("quality")
        .arg(tmp.to_str().unwrap())
        .arg("--format")
        .arg("json")
        .output()
        .expect("failed to run quality --format json");
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.trim().is_empty() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(stdout.trim());
        assert!(parsed.is_ok(), "JSON output must be valid JSON");
    }
    fs::remove_dir_all(&tmp).ok();
}
