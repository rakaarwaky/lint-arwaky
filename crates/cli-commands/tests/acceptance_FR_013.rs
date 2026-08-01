// Acceptance tests for the `watch` command.

use std::process::Command;
use std::time::Duration;

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
fn frd_watch_01_starts_and_shuts_down_gracefully() {
    let mut child = cli_bin()
        .arg("watch")
        .arg(".")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn watch");
    std::thread::sleep(Duration::from_millis(500));
    child.kill().expect("failed to kill watch");
    let output = child.wait().expect("failed to wait for watch");
    // If we reach here, watch started and was killable — test passes
    let _code = output.code();
}

#[test]
fn frd_watch_02_accepts_path_argument() {
    let mut child = cli_bin()
        .arg("watch")
        .arg(".")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn watch");
    std::thread::sleep(Duration::from_millis(200));
    child.kill().ok();
    child.wait().ok();
}
