// PURPOSE: dogfood test — drive the shipped lint-arwaky binary's `check`
// command against this crate's own folder, proving the self-lint gate that
// the supervisor crate is expected to pass is actually wired end to end.
//
// Mirrors `crates/maintenance/tests/dogfood_maintenance.rs`: skip when the
// binary is absent (local-only CI gate); hard-fail when it is present but
// reports a violation, because a self-lint failure is the exact defect this
// suite exists to catch.

use std::path::PathBuf;
use std::process::{Command, Output};

fn binary() -> Option<PathBuf> {
    let name = if cfg!(windows) {
        "lint-arwaky-cli.exe"
    } else {
        "lint-arwaky-cli"
    };
    std::env::var_os("PATH").and_then(|path| {
        std::env::split_paths(&path)
            .map(|dir| dir.join(name))
            .find(|candidate| candidate.is_file())
    })
}

fn run(arguments: &[&str]) -> Option<Output> {
    let binary = binary()?;
    Command::new(binary).args(arguments).output().ok()
}

fn text_of(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn dogfood_check_reports_no_violations_for_supervisor_workflow() {
    let Some(output) = run(&["check", "crates/supervisor-workflow"]) else {
        eprintln!(
            "SKIP dogfood_supervisor_cycle: no `lint-arwaky-cli` on PATH; nothing to dogfood against"
        );
        return;
    };
    let report = text_of(&output);
    assert!(
        !report.trim().is_empty(),
        "`lint-arwaky-cli check` must print a report even on a clean scan"
    );
    assert!(
        !report.contains("AES"),
        "self-lint of crates/supervisor-workflow must report 0 violations, got: {report}"
    );
}
