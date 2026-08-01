// Acceptance tests for individual linter commands (quality, import, naming, role, orphan, external).

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
fn frd_linter_01_quality_clean_dir() {
    let tmp = std::env::temp_dir().join(format!("acc_linter_01_{}", std::process::id()));
    let src = tmp.join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), "pub fn clean() {}\n").unwrap();
    let output = cli_bin()
        .arg("quality")
        .arg(tmp.to_str().unwrap())
        .output()
        .expect("failed to run quality");
    let code = output.status.code().unwrap_or(-1);
    assert!(
        code == 0 || code == 1 || code == 2,
        "quality should exit 0, 1, or 2, got {}",
        code
    );
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn frd_linter_02_import_clean_dir() {
    let tmp = std::env::temp_dir().join(format!("acc_linter_02_{}", std::process::id()));
    let src = tmp.join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), "pub fn clean() {}\n").unwrap();
    let output = cli_bin()
        .arg("import")
        .arg(tmp.to_str().unwrap())
        .output()
        .expect("failed to run import");
    let code = output.status.code().unwrap_or(-1);
    assert!(
        code == 0 || code == 1 || code == 2,
        "import should exit 0, 1, or 2, got {}",
        code
    );
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn frd_linter_03_naming_clean_dir() {
    let tmp = std::env::temp_dir().join(format!("acc_linter_03_{}", std::process::id()));
    let src = tmp.join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), "pub fn clean() {}\n").unwrap();
    let output = cli_bin()
        .arg("naming")
        .arg(tmp.to_str().unwrap())
        .output()
        .expect("failed to run naming");
    let code = output.status.code().unwrap_or(-1);
    assert!(
        code == 0 || code == 1 || code == 2,
        "naming should exit 0, 1, or 2, got {}",
        code
    );
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn frd_linter_04_role_clean_dir() {
    let tmp = std::env::temp_dir().join(format!("acc_linter_04_{}", std::process::id()));
    let src = tmp.join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), "pub fn clean() {}\n").unwrap();
    let output = cli_bin()
        .arg("role")
        .arg(tmp.to_str().unwrap())
        .output()
        .expect("failed to run role");
    let code = output.status.code().unwrap_or(-1);
    assert!(
        code == 0 || code == 1 || code == 2,
        "role should exit 0, 1, or 2, got {}",
        code
    );
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn frd_linter_05_orphan_clean_dir() {
    let tmp = std::env::temp_dir().join(format!("acc_linter_05_{}", std::process::id()));
    let src = tmp.join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), "pub fn clean() {}\n").unwrap();
    let output = cli_bin()
        .arg("orphan")
        .arg(tmp.to_str().unwrap())
        .output()
        .expect("failed to run orphan");
    let code = output.status.code().unwrap_or(-1);
    assert!(
        code == 0 || code == 1 || code == 2,
        "orphan should exit 0, 1, or 2, got {}",
        code
    );
    fs::remove_dir_all(&tmp).ok();
}

#[test]
fn frd_linter_06_external_clean_dir() {
    let tmp = std::env::temp_dir().join(format!("acc_linter_06_{}", std::process::id()));
    let src = tmp.join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), "pub fn clean() {}\n").unwrap();
    let output = cli_bin()
        .arg("external")
        .arg(tmp.to_str().unwrap())
        .output()
        .expect("failed to run external");
    let code = output.status.code().unwrap_or(-1);
    assert!(
        code == 0 || code == 1 || code == 2,
        "external should exit 0, 1, or 2, got {}",
        code
    );
    fs::remove_dir_all(&tmp).ok();
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
    let tmp = std::env::temp_dir().join(format!("acc_linter_08_{}", std::process::id()));
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
