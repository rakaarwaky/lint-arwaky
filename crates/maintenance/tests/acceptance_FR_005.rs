// PURPOSE: Acceptance test — FR-001 Doctor + FR-005 Diagnose Toolchain.
// Maps 1:1 to FRD requirements for environment health and toolchain verification.

use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;

/// FR-001: Doctor checks all 9 adapters and reports their status.
#[tokio::test]
async fn frd_001_doctor_checks_all_9_adapters() {
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let result = orch.doctor().await;
    let expected = [
        "clippy",
        "rustfmt",
        "cargo-audit",
        "ruff",
        "mypy",
        "bandit",
        "eslint",
        "prettier",
        "tsc",
    ];
    for adapter in &expected {
        let found = result
            .adapter_statuses
            .keys()
            .any(|k| k.value() == *adapter);
        assert!(found, "Doctor should check adapter '{}'", adapter);
    }
}

/// FR-001: Doctor reports language runtime versions.
#[tokio::test]
async fn frd_001_doctor_reports_language_versions() {
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let result = orch.doctor().await;
    // rustc should be available in Rust test environment
    assert!(
        !result.rust_version.value().is_empty(),
        "rust_version should be set"
    );
}

/// FR-001: Doctor reports healthy=false when adapters are missing.
#[tokio::test]
async fn frd_001_doctor_healthy_when_all_present() {
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let result = orch.doctor().await;
    let expected_healthy = result.issues.is_empty();
    assert_eq!(result.healthy.value(), expected_healthy);
}

/// FR-005: Diagnose toolchain returns rustc as required tool.
#[tokio::test]
async fn frd_005_diagnose_toolchain_includes_rustc() {
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let diag = orch.diagnose_toolchain().await;
    let rustc = diag.rust_tools.iter().find(|t| t.name == "rustc");
    assert!(rustc.is_some(), "rustc should be in rust_tools");
    assert_eq!(
        rustc.unwrap().status,
        "OK",
        "rustc should be OK in Rust env"
    );
}

/// FR-005: Diagnose toolchain reports cargo as required.
#[tokio::test]
async fn frd_005_diagnose_toolchain_cargo_is_ok() {
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let diag = orch.diagnose_toolchain().await;
    let cargo = diag.rust_tools.iter().find(|t| t.name == "cargo");
    assert!(cargo.is_some());
    assert_eq!(cargo.unwrap().status, "OK");
}

/// FR-005: Diagnose toolchain reports git as required VCS tool.
#[tokio::test]
async fn frd_005_diagnose_toolchain_git_is_ok() {
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let diag = orch.diagnose_toolchain().await;
    let git = diag.vcs_tools.iter().find(|t| t.name == "git");
    assert!(git.is_some());
    assert_eq!(git.unwrap().status, "OK");
}

/// FR-005: Diagnose toolchain returns valid status for all tools.
#[tokio::test]
async fn frd_005_diagnose_all_statuses_valid() {
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let diag = orch.diagnose_toolchain().await;
    let all = diag
        .rust_tools
        .iter()
        .chain(diag.python_tools.iter())
        .chain(diag.js_tools.iter())
        .chain(diag.vcs_tools.iter());
    for tool in all {
        assert!(
            ["OK", "WARN", "FAIL"].contains(&tool.status.as_str()),
            "Tool '{}' has invalid status '{}'",
            tool.name,
            tool.status
        );
    }
}
