// PURPOSE: dogfood test — drive filesystem's own CLI surface against a live run,
// skipping when the environment cannot supply one.
//
// The dogfood type is local-only by contract: it exercises the shipped binary
// rather than an in-process seam, so it depends on what is installed. Every
// assertion here is guarded, and an unavailable environment reports a skip
// instead of a failure — a dogfood test must never break CI.
//
// AES704 requires this file to exist: `dogfood_` is one of the seven test types
// every feature folder owes, and its purpose is proving the CLI works against a
// real session rather than a mock.

use std::process::Command;

/// The linter binary, resolved from `PATH` or the usual install location.
fn binary() -> Option<std::path::PathBuf> {
    let name = "filesystem-lint-arwaky-cli";
    if let Ok(path) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path) {
            let candidate = dir.join(name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

/// The dogfood run: invoke the real CLI and read its exit status.
///
/// Returns `None` when no binary is installed, which the caller reports as a
/// skip — a machine without the CLI has nothing to dogfood against.
fn run(arguments: &[&str]) -> Option<i32> {
    let binary = binary()?;
    let output = Command::new(binary).args(arguments).output().ok()?;
    Some(output.status.code().unwrap_or(-1))
}

#[test]
fn dogfood_cli_reports_a_usage_line_when_asked_for_help() {
    let Some(code) = run(&["--help"]) else {
        eprintln!(
            "SKIP dogfood_filesystem: no `filesystem-lint-arwaky-cli` on PATH; nothing to dogfood against"
        );
        return;
    };
    assert_eq!(
        code, 0,
        "`filesystem-lint-arwaky-cli --help` must exit 0; a non-zero exit means the shipped CLI \
         does not start, which is exactly what this test exists to catch"
    );
}

#[test]
fn dogfood_cli_exits_nonzero_on_a_path_that_does_not_exist() {
    let Some(code) = run(&["scan", "/nonexistent-filesystem-dogfood-target"]) else {
        eprintln!(
            "SKIP dogfood_filesystem: no `filesystem-lint-arwaky-cli` on PATH; nothing to dogfood against"
        );
        return;
    };
    assert_ne!(
        code, 0,
        "scanning a missing path must exit non-zero; exiting 0 would report a \
         clean scan for a target that does not exist"
    );
}
