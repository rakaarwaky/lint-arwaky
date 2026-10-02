// PURPOSE: dogfood test — drive file_watch's own CLI surface against a live run,
// skipping when the environment cannot supply one.
//
// The dogfood type is local-only by contract: it exercises the shipped binary
// rather than an in-process seam, so it depends on what is installed. A missing
// binary is a skip; a binary that is present but cannot *launch* is a failure,
// because that is precisely the defect these tests exist to catch.
//
// AES704 requires this file to exist: `dogfood_` is one of the seven test types
// every feature folder owes, and its purpose is proving the CLI works against a
// real session rather than a mock.

use std::path::PathBuf;
use std::process::{Command, Output};

/// The shipped linter binary, resolved from `PATH`.
///
/// This is the one binary every layer's CLI is exercised through
/// (`lint-arwaky-cli`), matching the install path in `AGENTS.md`. On Windows the
/// resolved name carries `.exe`, so a candidate without it is never found and
/// the tests would skip on a machine that has the CLI installed.
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

/// Invoke the real CLI and hand back the raw output.
///
/// Returns `None` only when no binary is installed. A launch failure is a panic
/// rather than a silent skip: the binary exists, so something is wrong with it,
/// and reporting "not installed" would misdescribe the cause.
fn run(arguments: &[&str]) -> Option<Output> {
    let binary = binary()?;
    match Command::new(binary).args(arguments).output() {
        Ok(output) => Some(output),
        Err(error) => panic!("`lint-arwaky-cli {arguments:?}` failed to launch: {error}"),
    }
}

/// The combined stdout+stderr of a CLI run.
fn text_of(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn dogfood_cli_prints_usage_when_asked_for_help() {
    let Some(output) = run(&["--help"]) else {
        eprintln!(
            "SKIP dogfood_file_watch: no `lint-arwaky-cli` on PATH; nothing to dogfood against"
        );
        return;
    };
    assert_eq!(
        output.status.code(),
        Some(0),
        "`lint-arwaky-cli --help` must exit 0; a non-zero exit means the shipped CLI \
         does not start, which is exactly what this test exists to catch"
    );
    assert!(
        !text_of(&output).trim().is_empty(),
        "`lint-arwaky-cli --help` must print usage; exiting 0 with no output means the \
         command dispatched nothing and the CLI is unusable from a terminal"
    );
}

#[test]
fn dogfood_cli_exits_nonzero_on_a_path_that_does_not_exist() {
    let Some(output) = run(&["scan", "/nonexistent-file-watch-dogfood-target"]) else {
        eprintln!(
            "SKIP dogfood_file_watch: no `lint-arwaky-cli` on PATH; nothing to dogfood against"
        );
        return;
    };
    assert_ne!(
        output.status.code(),
        Some(0),
        "scanning a missing path must exit non-zero; exiting 0 would report a \
         clean scan for a target that does not exist"
    );
}
