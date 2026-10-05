// Contract test: one version string, whatever crate prints it.
//
// The tool used to read `env!("CARGO_PKG_VERSION")`, which resolves to the
// version of whichever crate held the printing code. That made the reported
// version depend on the code path: `la version` came from `dispatcher`, the
// scan header from `shared/cli_commands`, SARIF from `report-formatter`, and
// they disagreed. A release that moved only the root `Cargo.toml` shipped a
// binary still announcing the previous version.
//
// `RELEASE_VERSION` is generated from the workspace `VERSION` file, so it must
// equal the root package version no matter which crate reads it. This test
// pins that against `Cargo.toml` directly — reading the file rather than
// trusting a constant, so a partial bump fails here instead of shipping.

use std::path::Path;

fn repo_root() -> std::path::PathBuf {
    // crates/dispatcher sits two levels below the workspace root.
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root resolves")
}

/// The root package's `version`, read straight out of `Cargo.toml`.
fn root_package_version() -> String {
    let manifest = std::fs::read_to_string(repo_root().join("Cargo.toml"))
        .expect("root Cargo.toml is readable");
    let mut in_package = false;
    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_package = trimmed == "[package]";
            continue;
        }
        if in_package && let Some(value) = trimmed.strip_prefix("version = ") {
            return value.trim().trim_matches('"').to_string();
        }
    }
    panic!("root [package] has no version line");
}

#[test]
fn release_version_matches_the_root_package() {
    assert_eq!(
        shared_common::RELEASE_VERSION,
        root_package_version(),
        "VERSION file and root Cargo.toml must agree"
    );
}

#[test]
fn release_version_looks_like_a_release() {
    let version = shared_common::RELEASE_VERSION;
    assert!(!version.is_empty(), "version must not be empty");
    assert!(
        version.starts_with(|c: char| c.is_ascii_digit()),
        "version must start with a digit, got {version:?}"
    );
    assert!(
        !version.starts_with('v'),
        "VERSION file holds a bare number; tags carry the v"
    );
}

#[test]
fn version_action_reports_the_release_version() {
    let report = dispatcher_lint_arwaky::surface_version_action::collect_version();
    assert_eq!(report.version, shared_common::RELEASE_VERSION);
}
