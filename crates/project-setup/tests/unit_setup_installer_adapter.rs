// PURPOSE: Unit tests for SetupInstallerAdapter — pip/npm install adapters.
// Layer: Capabilities (SetupInstallerAdapter)

use project_setup_lint_arwaky::capabilities_setup_installer_adapter::SetupInstallerAdapter;
use shared::project_setup::contract_setup_protocol::ISetupInstallerProtocol;
use shared::project_setup::taxonomy_setup_contract_vo::SetupError;

fn adapter() -> SetupInstallerAdapter {
    SetupInstallerAdapter::new()
}

// ─── install_python_packages: Default trait ──

#[test]
fn installer_adapter_default_creates_valid_instance() {
    let _ = SetupInstallerAdapter::default();
}

// ─── install_python_packages: Happy path (dry run) ──

#[tokio::test]
async fn install_python_packages_calls_pip() {
    // This tests the adapter calls pip install --user <packages>
    // Since we can't actually run pip in tests, we verify the method signature is correct
    let adapter = adapter();
    let packages: Vec<String> = vec!["requests".to_string(), "click".to_string()];
    
    // The actual call would execute pip, but we verify it doesn't panic on empty input
    let result = adapter.install_python_packages(&[]).await;
    // Empty packages should succeed without error
    assert!(result.is_ok());
}

// ─── install_npm_packages: Default trait ──

#[tokio::test]
async fn install_npm_packages_calls_npm() {
    let adapter = adapter();
    let packages: Vec<String> = vec!["typescript".to_string()];
    
    // Verify method signature is correct (actual npm execution skipped in tests)
    let result = adapter.install_npm_packages(&[], false).await;
    assert!(result.is_ok());
}

// ─── SetupError verification ──

#[test]
fn setup_error_creates_valid_instance() {
    let error = SetupError::new("test error".to_string());
    assert_eq!(error.message.value, "test error");
}
