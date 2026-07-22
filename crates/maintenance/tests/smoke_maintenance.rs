// PURPOSE: Smoke test — verify the maintenance crate boots and responds within 5 seconds.
// Layer: Smoke (must complete < 5s).

use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;
use maintenance_lint_arwaky::capabilities_maintenance_checker::MaintenanceChecker;
use maintenance_lint_arwaky::capabilities_tool_executor_adapter::ToolExecutorAdapter;

use shared::project_setup::contract_maintenance_protocol::IMaintenanceCheckerProtocol;
use shared::project_setup::contract_tool_executor_protocol::IToolExecutorProtocol;

#[tokio::test]
async fn maintenance_crate_boots_and_responds() {
    // 1. Container instantiates without panic
    let container = MaintenanceContainer::new();

    // 2. Orchestrator is accessible
    let orch = container.orchestrator();

    // 3. Doctor responds (fastest meaningful operation)
    let result = orch.doctor().await;
    assert!(!result.python_version.value().is_empty());

    // 4. Capabilities instantiate
    let checker = MaintenanceChecker::new();
    let adapter = ToolExecutorAdapter::new();

    // 5. Tool executor responds
    let output = adapter.run_tool("echo", &["smoke_ok"]).await;
    assert!(output.success);
    assert!(output.stdout.contains("smoke_ok"));

    // 6. Checker responds
    let diag = checker.diagnose_toolchain().await;
    assert!(!diag.binary_path.is_empty());
}
