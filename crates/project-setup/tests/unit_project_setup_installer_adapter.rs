// Unit tests — SetupInstallerAdapter edge cases.
use project_setup_lint_arwaky::capabilities_setup_installer_adapter::SetupInstallerAdapter;
use shared_common::taxonomy_common_vo::PatternList;
use shared_project_setup::IAdapterInstallationProtocol;

#[test]
fn install_python_packages_empty_returns_ok() {
    let adapter = SetupInstallerAdapter::new();
    let result = adapter.install_python_packages(&PatternList::default());
    assert!(result.is_ok(), "Empty packages list should return Ok");
}

#[test]
fn install_npm_packages_empty_returns_ok() {
    let adapter = SetupInstallerAdapter::new();
    let result = adapter.install_npm_packages(&PatternList::default(), false);
    assert!(result.is_ok(), "Empty packages list should return Ok");
}

#[test]
fn install_python_packages_empty_with_sudo() {
    let adapter = SetupInstallerAdapter::new();
    let result = adapter.install_python_packages(&PatternList::default());
    assert!(result.is_ok());
}

#[test]
fn install_npm_packages_empty_with_sudo() {
    let adapter = SetupInstallerAdapter::new();
    let result = adapter.install_npm_packages(&PatternList::default(), true);
    assert!(result.is_ok());
}

#[test]
fn failed_pip_retry_reports_retry_exit_code_not_first() {
    // Exercises the PEP 668 path: first `pip install --user` fails (exit 1),
    // the `--break-system-packages` retry also fails (no matching
    // distribution for a bogus package). The error must carry the RETRY's
    // exit code, not the first attempt's.
    let adapter = SetupInstallerAdapter::new();
    let res = adapter
        .install_python_packages(&PatternList::new(vec!["definetly-not-a-real-package-xyz"]));
    let Err(e) = res else {
        return; // pip unexpectedly succeeded; nothing to assert
    };
    let msg = format!("{e}");
    assert!(
        msg.contains("--break-system-packages") && msg.contains("exited with status"),
        "error must reference the retry attempt and its exit status, got: {msg}"
    );
}

#[test]
fn adapter_is_default_constructible() {
    let adapter = SetupInstallerAdapter::new();
    let result = adapter.install_python_packages(&PatternList::default());
    assert!(result.is_ok());
}
