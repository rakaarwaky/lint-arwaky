// Unit tests — SetupInstallerAdapter edge cases.
use project_setup_lint_arwaky::capabilities_setup_installer_adapter::SetupInstallerAdapter;
use shared::project_setup::ISetupInstallerProtocol;

#[test]
fn install_python_packages_empty_returns_ok() {
    let adapter = SetupInstallerAdapter::new();
    let result = adapter.install_python_packages(&[]);
    assert!(result.is_ok(), "Empty packages list should return Ok");
}

#[test]
fn install_npm_packages_empty_returns_ok() {
    let adapter = SetupInstallerAdapter::new();
    let result = adapter.install_npm_packages(&[], false);
    assert!(result.is_ok(), "Empty packages list should return Ok");
}

#[test]
fn install_python_packages_empty_with_sudo() {
    let adapter = SetupInstallerAdapter::new();
    let result = adapter.install_python_packages(&[]);
    assert!(result.is_ok());
}

#[test]
fn install_npm_packages_empty_with_sudo() {
    let adapter = SetupInstallerAdapter::new();
    let result = adapter.install_npm_packages(&[], true);
    assert!(result.is_ok());
}

#[test]
fn adapter_is_default_constructible() {
    let adapter = SetupInstallerAdapter::default();
    let result = adapter.install_python_packages(&[]);
    assert!(result.is_ok());
}
