// PURPOSE: dogfood tests — CLI against the real binary. Runs
// `lint-arwaky-cli version` and asserts exit 0. Skipped if the
// binary is not on PATH.
#[test]
fn dogfood_cli_version_exits_zero() {
    let binary = std::env::var("LINT_ARWAKY_CLI")
        .ok()
        .map(std::path::PathBuf::from)
        .filter(|p| p.exists())
        .unwrap_or_else(|| std::path::PathBuf::from("lint-arwaky-cli"));
    if binary.exists() {
        let status = std::process::Command::new(&binary)
            .arg("version")
            .status()
            .expect("failed to spawn");
        assert!(status.success());
    } else {
        eprintln!("SKIP: {} not found", binary.display());
    }
}
