// Acceptance tests for the `dependencies` command.

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
fn frd_deps_01_shows_report_header() {
    let output = cli_bin()
        .arg("dependencies")
        .arg(".")
        .output()
        .expect("failed to run dependencies");
    let code = output.status.code().unwrap_or(-1);
    if code == 0 {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Dependency Report"),
            "dependencies must show report header, got: {}",
            stdout
        );
    }
    assert!(code == 0 || code == 2, "should exit 0 or 2, got {}", code);
}

#[test]
fn frd_deps_02_no_dependency_file_exit_2() {
    let tmp = std::env::temp_dir().join(format!("acc_deps_02_{}", std::process::id()));
    fs::create_dir_all(&tmp).unwrap();
    let output = cli_bin()
        .arg("dependencies")
        .arg(tmp.to_str().unwrap())
        .output()
        .expect("failed to run dependencies");
    let code = output.status.code().unwrap_or(-1);
    assert!(code == 2, "no dependency file should exit 2, got {}", code);
    fs::remove_dir_all(&tmp).ok();
}
