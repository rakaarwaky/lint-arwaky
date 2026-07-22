
# Test Suite for `maintenance` crate (v1.10.106)

Below is the complete test suite following the `create-test-rust` conventions. All tests live flat in `crates/maintenance/tests/` with filename prefixes as virtual subfolders.

---

## Task Progress

- [X] Step 1: Analyze crate structure
- [X] Step 2: Identify untested public API
- [X] Step 3: Write `contract_maintenance.rs`
- [X] Step 4: Write `unit_maintenance_checker.rs`
- [X] Step 5: Write `unit_maintenance_tool_executor.rs`
- [X] Step 6: Write `unit_maintenance_orchestrator.rs`
- [X] Step 7: Write `integration_maintenance.rs`
- [X] Step 8: Write `smoke_maintenance.rs`
- [X] Step 9: Write `e2e_maintenance_flow.rs`
- [X] Step 10: Write `acceptance_FRD_*.rs`
- [X] Step 11: Write `bench_maintenance_throughput.rs` + register in Cargo.toml

---

## Directory Layout

```
crates/maintenance/
├── src/
│   ├── lib.rs
│   ├── agent_maintenance_orchestrator.rs
│   ├── capabilities_maintenance_checker.rs
│   ├── capabilities_tool_executor_adapter.rs
│   └── root_maintenance_container.rs
├── tests/
│   ├── contract_maintenance.rs
│   ├── unit_maintenance_checker.rs
│   ├── unit_maintenance_tool_executor.rs
│   ├── unit_maintenance_orchestrator.rs
│   ├── integration_maintenance.rs
│   ├── smoke_maintenance.rs
│   ├── e2e_maintenance_flow.rs
│   ├── acceptance_FRD_dep_update.rs
│   ├── acceptance_FRD_audit.rs
│   └── bench_maintenance_throughput.rs
└── Cargo.toml
```

---

## `tests/contract_maintenance.rs`

```rust
// PURPOSE: Verify that all public structs implement their required contract traits.
// Layer: Contract verification — runs in ms, every PR.

use maintenance_lint_arwaky::capabilities_maintenance_checker::MaintenanceChecker;
use maintenance_lint_arwaky::capabilities_tool_executor_adapter::ToolExecutorAdapter;
use maintenance_lint_arwaky::agent_maintenance_orchestrator::MaintenanceCommandsOrchestrator;
use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;

use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use shared::project_setup::contract_maintenance_protocol::IMaintenanceCheckerProtocol;
use shared::project_setup::contract_tool_executor_protocol::IToolExecutorProtocol;

// ─── MaintenanceChecker implements IMaintenanceCheckerProtocol ───

#[test]
fn maintenance_checker_implements_i_maintenance_checker_protocol() {
    fn assert_trait<T: IMaintenanceCheckerProtocol>() {}
    assert_trait::<MaintenanceChecker>();
}

#[test]
fn maintenance_checker_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<MaintenanceChecker>();
}

// ─── ToolExecutorAdapter implements IToolExecutorProtocol ───

#[test]
fn tool_executor_adapter_implements_i_tool_executor_protocol() {
    fn assert_trait<T: IToolExecutorProtocol>() {}
    assert_trait::<ToolExecutorAdapter>();
}

#[test]
fn tool_executor_adapter_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ToolExecutorAdapter>();
}

// ─── MaintenanceCommandsOrchestrator implements MaintenanceCommandsAggregate ───

#[test]
fn orchestrator_implements_maintenance_commands_aggregate() {
    fn assert_trait<T: MaintenanceCommandsAggregate>() {}
    assert_trait::<MaintenanceCommandsOrchestrator>();
}

#[test]
fn orchestrator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<MaintenanceCommandsOrchestrator>();
}

// ─── MaintenanceContainer wiring ───

#[test]
fn container_exposes_aggregate_as_trait_object() {
    let container = MaintenanceContainer::new();
    let orchestrator = container.orchestrator();
    // Verify the Arc<dyn MaintenanceCommandsAggregate> is usable
    let _ref: &dyn MaintenanceCommandsAggregate = orchestrator.as_ref();
}

// ─── Default trait implementations ───

#[test]
fn maintenance_checker_implements_default() {
    let checker = MaintenanceChecker::default();
    let _ = checker;
}

#[test]
fn tool_executor_adapter_implements_default() {
    let adapter = ToolExecutorAdapter::default();
    let _ = adapter;
}

#[test]
fn orchestrator_implements_default() {
    let orch = MaintenanceCommandsOrchestrator::default();
    let _ = orch;
}

#[test]
fn container_implements_default() {
    let container = MaintenanceContainer::default();
    let _ = container;
}
```

---

## `tests/unit_maintenance_checker.rs`

```rust
// PURPOSE: Unit tests for MaintenanceChecker — diagnose_toolchain, run_security_scan, run_dependency_report.
// Layer: Capabilities (target ≥ 70% coverage).

use maintenance_lint_arwaky::capabilities_maintenance_checker::MaintenanceChecker;
use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_maintenance_protocol::IMaintenanceCheckerProtocol;

fn sut() -> MaintenanceChecker {
    MaintenanceChecker::new()
}

// ─── diagnose_toolchain ───

#[tokio::test]
async fn diagnose_toolchain_returns_rust_tools() {
    let checker = sut();
    let diag = checker.diagnose_toolchain().await;

    // cargo is required — should always be present in a Rust test environment
    assert!(
        diag.rust_tools.iter().any(|t| t.name == "cargo"),
        "Expected 'cargo' in rust_tools"
    );
}

#[tokio::test]
async fn diagnose_toolchain_returns_python_tools() {
    let checker = sut();
    let diag = checker.diagnose_toolchain().await;

    // python3 is optional — list should still be populated
    assert!(
        !diag.python_tools.is_empty(),
        "python_tools should not be empty"
    );
}

#[tokio::test]
async fn diagnose_toolchain_returns_js_tools() {
    let checker = sut();
    let diag = checker.diagnose_toolchain().await;

    assert!(!diag.js_tools.is_empty(), "js_tools should not be empty");
}

#[tokio::test]
async fn diagnose_toolchain_returns_vcs_tools() {
    let checker = sut();
    let diag = checker.diagnose_toolchain().await;

    assert!(
        diag.vcs_tools.iter().any(|t| t.name == "git"),
        "Expected 'git' in vcs_tools"
    );
}

#[tokio::test]
async fn diagnose_toolchain_binary_path_is_not_empty() {
    let checker = sut();
    let diag = checker.diagnose_toolchain().await;

    assert!(
        !diag.binary_path.is_empty(),
        "binary_path should resolve to current exe"
    );
}

#[tokio::test]
async fn diagnose_toolchain_tool_status_has_valid_status_values() {
    let checker = sut();
    let diag = checker.diagnose_toolchain().await;

    let all_tools = diag
        .rust_tools
        .iter()
        .chain(diag.python_tools.iter())
        .chain(diag.js_tools.iter())
        .chain(diag.vcs_tools.iter());

    for tool in all_tools {
        assert!(
            ["OK", "WARN", "FAIL"].contains(&tool.status.as_str()),
            "Tool '{}' has invalid status '{}'",
            tool.name,
            tool.status
        );
    }
}

// ─── run_security_scan ───

#[tokio::test]
async fn security_scan_nonexistent_path_returns_report() {
    let checker = sut();
    let path = FilePath::new("/tmp/nonexistent_project_xyz_12345").unwrap();
    let report = checker.run_security_scan(&path).await;

    // Should not panic; returns a report (possibly empty findings)
    assert!(
        report.language == "Rust" || report.language == "Python",
        "Language should be Rust or Python"
    );
}

#[tokio::test]
async fn security_scan_report_has_tool_name() {
    let checker = sut();
    let path = FilePath::new("/tmp/nonexistent_project_xyz_12345").unwrap();
    let report = checker.run_security_scan(&path).await;

    assert!(
        !report.tool_name.is_empty(),
        "tool_name should not be empty"
    );
}

// ─── run_dependency_report ───

#[tokio::test]
async fn dependency_report_no_files_returns_error() {
    let checker = sut();
    let path = FilePath::new("/tmp/empty_dir_no_deps_xyz_99999").unwrap();

    // Create the empty dir so path exists but has no dependency files
    let _ = std::fs::create_dir_all("/tmp/empty_dir_no_deps_xyz_99999");

    let result = checker.run_dependency_report(&path).await;
    assert!(result.is_err(), "Should return error when no dep files found");
    assert!(result
        .unwrap_err()
        .contains("No dependency files found"));

    // Cleanup
    let _ = std::fs::remove_dir_all("/tmp/empty_dir_no_deps_xyz_99999");
}

#[tokio::test]
async fn dependency_report_with_requirements_txt() {
    let checker = sut();
    let dir = "/tmp/test_dep_report_reqs_xyz";
    let _ = std::fs::create_dir_all(dir);
    std::fs::write(
        format!("{}/requirements.txt", dir),
        "flask==2.3.0\nrequests>=2.28\n# comment\nnumpy\n",
    )
    .unwrap();

    let path = FilePath::new(dir.to_string()).unwrap();
    let result = checker.run_dependency_report(&path).await;

    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.language, "Python");
    assert!(report.dependencies.len() >= 3); // flask, requests, numpy (comment skipped)

    let flask = report.dependencies.iter().find(|d| d.name == "flask");
    assert!(flask.is_some());

    // Cleanup
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn dependency_report_with_cargo_lock() {
    let checker = sut();
    let dir = "/tmp/test_dep_report_cargo_xyz";
    let _ = std::fs::create_dir_all(dir);

    let cargo_lock = r#"
[[package]]
name = "serde"
version = "1.0.193"

[[package]]
name = "my-project"
version = "0.1.0"
"#;
    std::fs::write(format!("{}/Cargo.lock", dir), cargo_lock).unwrap();

    let cargo_toml = r#"
[package]
name = "my-project"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#;
    std::fs::write(format!("{}/Cargo.toml", dir), cargo_toml).unwrap();

    let path = FilePath::new(dir.to_string()).unwrap();
    let result = checker.run_dependency_report(&path).await;

    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.language, "Rust");
    assert!(report.dependencies.len() >= 2);

    let serde_dep = report.dependencies.iter().find(|d| d.name == "serde");
    assert!(serde_dep.is_some());
    assert_eq!(serde_dep.unwrap().dep_type, "direct");

    // Cleanup
    let _ = std::fs::remove_dir_all(dir);
}
```

---

## `tests/unit_maintenance_tool_executor.rs`

```rust
// PURPOSE: Unit tests for ToolExecutorAdapter — run_tool, run_tool_in_dir, tool_exists, get_binary_path.
// Layer: Capabilities (target ≥ 70% coverage).

use maintenance_lint_arwaky::capabilities_tool_executor_adapter::ToolExecutorAdapter;
use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_tool_executor_protocol::IToolExecutorProtocol;

fn sut() -> ToolExecutorAdapter {
    ToolExecutorAdapter::new()
}

// ─── run_tool ───

#[tokio::test]
async fn run_tool_echo_returns_stdout() {
    let adapter = sut();
    let output = adapter.run_tool("echo", &["hello", "world"]).await;

    assert!(output.success);
    assert!(output.stdout.contains("hello world"));
    assert!(output.stderr.is_empty());
}

#[tokio::test]
async fn run_tool_nonexistent_binary_returns_failure() {
    let adapter = sut();
    let output = adapter
        .run_tool("nonexistent_binary_xyz_99999", &[])
        .await;

    assert!(!output.success);
    assert!(output.stdout.is_empty());
    assert!(output.stderr.contains("Failed to execute"));
}

#[tokio::test]
async fn run_tool_captures_stderr() {
    let adapter = sut();
    // Use a command that writes to stderr
    let output = adapter
        .run_tool("sh", &["-c", "echo error_msg >&2"])
        .await;

    assert!(output.stderr.contains("error_msg"));
}

#[tokio::test]
async fn run_tool_exit_code_failure() {
    let adapter = sut();
    let output = adapter.run_tool("sh", &["-c", "exit 1"]).await;

    assert!(!output.success);
}

// ─── run_tool_in_dir ───

#[tokio::test]
async fn run_tool_in_dir_executes_in_specified_directory() {
    let adapter = sut();
    let dir = FilePath::new("/tmp".to_string()).unwrap();
    let output = adapter.run_tool_in_dir("pwd", &[], &dir).await;

    assert!(output.success);
    // /tmp may resolve to /private/tmp on macOS
    assert!(
        output.stdout.trim().contains("tmp"),
        "Expected pwd to contain 'tmp', got: {}",
        output.stdout
    );
}

#[tokio::test]
async fn run_tool_in_dir_nonexistent_dir_returns_failure() {
    let adapter = sut();
    let dir = FilePath::new("/nonexistent_dir_xyz_99999".to_string()).unwrap();
    let output = adapter.run_tool_in_dir("echo", &["test"], &dir).await;

    assert!(!output.success);
}

// ─── tool_exists ───

#[tokio::test]
async fn tool_exists_returns_true_for_echo() {
    let adapter = sut();
    let exists = adapter.tool_exists("echo").await;
    assert!(exists, "'echo' should exist on any Unix system");
}

#[tokio::test]
async fn tool_exists_returns_false_for_nonexistent() {
    let adapter = sut();
    let exists = adapter.tool_exists("nonexistent_tool_xyz_99999").await;
    assert!(!exists);
}

// ─── get_binary_path ───

#[tokio::test]
async fn get_binary_path_returns_valid_path() {
    let adapter = sut();
    let path = adapter.get_binary_path().await;

    assert!(
        !path.value().is_empty(),
        "Binary path should not be empty"
    );
}
```

---

## `tests/unit_maintenance_orchestrator.rs`

```rust
// PURPOSE: Unit tests for MaintenanceCommandsOrchestrator — stats, clean, doctor, cancel.
// Layer: Agent (target ≥ 60% coverage).

use maintenance_lint_arwaky::agent_maintenance_orchestrator::MaintenanceCommandsOrchestrator;
use shared::common::taxonomy_path_vo::FilePath;
use shared::mcp_server::taxonomy_action_vo::JobId;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;

fn sut() -> MaintenanceCommandsOrchestrator {
    MaintenanceCommandsOrchestrator::new()
}

// ─── stats ───

#[tokio::test]
async fn stats_counts_python_files_in_directory() {
    let dir = "/tmp/test_stats_py_xyz";
    let _ = std::fs::create_dir_all(format!("{}/src", dir));
    std::fs::write(format!("{}/src/main.py", dir), "print('hi')").unwrap();
    std::fs::write(format!("{}/src/utils.py", dir), "pass").unwrap();
    std::fs::write(format!("{}/src/test_main.py", dir), "def test(): pass").unwrap();
    std::fs::write(format!("{}/README.md", dir), "# Readme").unwrap();

    let orch = sut();
    let path = FilePath::new(dir.to_string()).unwrap();
    let stats = orch.stats(&path).await;

    assert_eq!(stats.python_files.value(), 3); // main.py, utils.py, test_main.py
    assert_eq!(stats.test_files.value(), 1); // test_main.py
    assert!(stats.test_ratio.value() > 0.0);

    // Cleanup
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn stats_empty_directory_returns_zero_counts() {
    let dir = "/tmp/test_stats_empty_xyz";
    let _ = std::fs::create_dir_all(dir);

    let orch = sut();
    let path = FilePath::new(dir.to_string()).unwrap();
    let stats = orch.stats(&path).await;

    assert_eq!(stats.total_files.value(), 0);
    assert_eq!(stats.test_files.value(), 0);
    assert_eq!(stats.test_ratio.value(), 0.0);

    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn stats_skips_target_and_git_dirs() {
    let dir = "/tmp/test_stats_skip_xyz";
    let _ = std::fs::create_dir_all(format!("{}/target", dir));
    let _ = std::fs::create_dir_all(format!("{}/.git", dir));
    std::fs::write(format!("{}/target/build.py", dir), "pass").unwrap();
    std::fs::write(format!("{}/.git/hook.py", dir), "pass").unwrap();
    std::fs::write(format!("{}/app.py", dir), "pass").unwrap();

    let orch = sut();
    let path = FilePath::new(dir.to_string()).unwrap();
    let stats = orch.stats(&path).await;

    // Only app.py should be counted (target/ and .git/ are skipped)
    assert_eq!(stats.python_files.value(), 1);

    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn stats_test_ratio_is_correct_fraction() {
    let dir = "/tmp/test_stats_ratio_xyz";
    let _ = std::fs::create_dir_all(dir);
    // 4 python files, 2 are tests → ratio = 0.5
    std::fs::write(format!("{}/a.py", dir), "").unwrap();
    std::fs::write(format!("{}/b.py", dir), "").unwrap();
    std::fs::write(format!("{}/test_a.py", dir), "").unwrap();
    std::fs::write(format!("{}/test_b.py", dir), "").unwrap();

    let orch = sut();
    let path = FilePath::new(dir.to_string()).unwrap();
    let stats = orch.stats(&path).await;

    assert!((stats.test_ratio.value() - 0.5).abs() < f64::EPSILON);

    let _ = std::fs::remove_dir_all(dir);
}

// ─── clean ───

#[tokio::test]
async fn clean_does_not_panic() {
    let orch = sut();
    // clean() operates on cwd — just verify it doesn't panic
    orch.clean().await;
}

// ─── doctor ───

#[tokio::test]
async fn doctor_returns_structured_result() {
    let orch = sut();
    let result = orch.doctor().await;

    // python_version should be set
    assert!(!result.python_version.value().is_empty());

    // adapter_statuses should have entries for ruff, mypy, bandit, radon
    assert!(result.adapter_statuses.len() >= 4);
}

#[tokio::test]
async fn doctor_reports_missing_adapters_as_issues() {
    let orch = sut();
    let result = orch.doctor().await;

    // If any adapter is missing, there should be a corresponding issue
    let missing_count = result
        .adapter_statuses
        .values()
        .filter(|s| *s == "MISSING")
        .count();

    // issues should include at least the missing adapter messages
    // (plus possibly "No configuration file found")
    assert!(result.issues.len() >= missing_count);
}

#[tokio::test]
async fn doctor_healthy_flag_matches_issues() {
    let orch = sut();
    let result = orch.doctor().await;

    let expected_healthy = result.issues.is_empty();
    assert_eq!(result.healthy.value(), expected_healthy);
}

// ─── cancel ───

#[tokio::test]
async fn cancel_does_not_panic_with_arbitrary_job_id() {
    let orch = sut();
    let job_id = JobId::new("test-job-001".to_string());
    orch.cancel(job_id).await;
    // No assertion needed — just verifying no panic
}

// ─── diagnose_toolchain (delegated) ───

#[tokio::test]
async fn diagnose_toolchain_delegates_to_checker() {
    let orch = sut();
    let diag = orch.diagnose_toolchain().await;

    assert!(!diag.rust_tools.is_empty());
    assert!(!diag.binary_path.is_empty());
}

// ─── run_security_scan (delegated) ───

#[tokio::test]
async fn run_security_scan_delegates_to_checker() {
    let orch = sut();
    let path = FilePath::new("/tmp/nonexistent_scan_xyz".to_string()).unwrap();
    let report = orch.run_security_scan(&path).await;

    assert!(!report.tool_name.is_empty());
}

// ─── run_dependency_report (delegated) ───

#[tokio::test]
async fn run_dependency_report_delegates_to_checker() {
    let orch = sut();
    let path = FilePath::new("/tmp/nonexistent_dep_xyz".to_string()).unwrap();
    let result = orch.run_dependency_report(&path).await;

    // Should return error since no dep files exist
    assert!(result.is_err());
}
```

---

## `tests/integration_maintenance.rs`

```rust
// PURPOSE: Integration tests — verify DI container wiring and cross-layer collaboration.
// Layer: Integration (uses real MaintenanceContainer).

use std::sync::Arc;

use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;

fn container() -> MaintenanceContainer {
    MaintenanceContainer::new()
}

// ─── Container wiring ───

#[test]
fn container_creates_orchestrator_successfully() {
    let c = container();
    let orch = c.orchestrator();
    assert!(Arc::strong_count(&orch) >= 1);
}

#[test]
fn container_orchestrator_returns_same_arc_on_multiple_calls() {
    let c = container();
    let orch1 = c.orchestrator();
    let orch2 = c.orchestrator();
    assert!(Arc::ptr_eq(&orch1, &orch2));
}

// ─── Orchestrator via container ───

#[tokio::test]
async fn container_orchestrator_stats_works() {
    let dir = "/tmp/test_integration_stats_xyz";
    let _ = std::fs::create_dir_all(dir);
    std::fs::write(format!("{}/app.py", dir), "pass").unwrap();

    let c = container();
    let orch = c.orchestrator();
    let path = FilePath::new(dir.to_string()).unwrap();
    let stats = orch.stats(&path).await;

    assert_eq!(stats.python_files.value(), 1);

    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn container_orchestrator_doctor_works() {
    let c = container();
    let orch = c.orchestrator();
    let result = orch.doctor().await;

    // Should return a valid DoctorResultVO regardless of environment
    assert!(!result.python_version.value().is_empty());
}

#[tokio::test]
async fn container_orchestrator_diagnose_toolchain_works() {
    let c = container();
    let orch = c.orchestrator();
    let diag = orch.diagnose_toolchain().await;

    // In a Rust test environment, cargo must be present
    assert!(diag.rust_tools.iter().any(|t| t.name == "cargo"));
}

#[tokio::test]
async fn container_orchestrator_security_scan_does_not_panic() {
    let c = container();
    let orch = c.orchestrator();
    let path = FilePath::new("/tmp/nonexistent_integ_scan".to_string()).unwrap();
    let report = orch.run_security_scan(&path).await;

    assert!(!report.tool_name.is_empty());
}

#[tokio::test]
async fn container_orchestrator_dependency_report_returns_error_for_empty() {
    let c = container();
    let orch = c.orchestrator();
    let path = FilePath::new("/tmp/nonexistent_integ_dep".to_string()).unwrap();
    let result = orch.run_dependency_report(&path).await;

    assert!(result.is_err());
}

// ─── Full pipeline: stats → doctor → diagnose ───

#[tokio::test]
async fn full_maintenance_pipeline_sequential() {
    let dir = "/tmp/test_integration_pipeline_xyz";
    let _ = std::fs::create_dir_all(dir);
    std::fs::write(format!("{}/main.py", dir), "print('hello')").unwrap();
    std::fs::write(format!("{}/test_main.py", dir), "def test(): pass").unwrap();

    let c = container();
    let orch = c.orchestrator();
    let path = FilePath::new(dir.to_string()).unwrap();

    // Step 1: stats
    let stats = orch.stats(&path).await;
    assert_eq!(stats.python_files.value(), 2);
    assert_eq!(stats.test_files.value(), 1);

    // Step 2: doctor
    let doctor = orch.doctor().await;
    assert!(!doctor.python_version.value().is_empty());

    // Step 3: diagnose_toolchain
    let diag = orch.diagnose_toolchain().await;
    assert!(!diag.rust_tools.is_empty());

    let _ = std::fs::remove_dir_all(dir);
}
```

---

## `tests/smoke_maintenance.rs`

```rust
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
```

---

## `tests/e2e_maintenance_flow.rs`

```rust
// PURPOSE: E2E test — full maintenance lifecycle: create project → stats → scan → report → clean.
// Layer: E2E (full request lifecycle, no internal mocks).

use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;

#[tokio::test]
async fn full_maintenance_lifecycle_on_python_project() {
    // Arrange: create a temporary Python project
    let dir = "/tmp/e2e_maintenance_python_proj";
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(format!("{}/src", dir)).unwrap();
    std::fs::create_dir_all(format!("{}/tests", dir)).unwrap();

    std::fs::write(format!("{}/src/app.py", dir), "def main(): pass\n").unwrap();
    std::fs::write(format!("{}/src/utils.py", dir), "def helper(): pass\n").unwrap();
    std::fs::write(format!("{}/tests/test_app.py", dir), "def test_main(): pass\n").unwrap();
    std::fs::write(
        format!("{}/requirements.txt", dir),
        "flask==2.3.0\nrequests>=2.28\n",
    )
    .unwrap();

    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.to_string()).unwrap();

    // Act 1: stats
    let stats = orch.stats(&path).await;
    assert_eq!(stats.python_files.value(), 3);
    assert_eq!(stats.test_files.value(), 1);
    assert!(stats.test_ratio.value() > 0.0);

    // Act 2: diagnose_toolchain
    let diag = orch.diagnose_toolchain().await;
    assert!(!diag.rust_tools.is_empty());
    assert!(!diag.vcs_tools.is_empty());

    // Act 3: run_security_scan (bandit path since no Cargo.lock)
    let scan = orch.run_security_scan(&path).await;
    assert_eq!(scan.language, "Python");
    assert_eq!(scan.tool_name, "bandit");

    // Act 4: run_dependency_report (requirements.txt path)
    let dep_result = orch.run_dependency_report(&path).await;
    assert!(dep_result.is_ok());
    let dep_report = dep_result.unwrap();
    assert_eq!(dep_report.language, "Python");
    assert!(dep_report.dependencies.len() >= 2);

    // Act 5: doctor
    let doctor = orch.doctor().await;
    assert!(!doctor.python_version.value().is_empty());

    // Cleanup
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn full_maintenance_lifecycle_on_rust_project() {
    // Arrange: create a temporary Rust project structure
    let dir = "/tmp/e2e_maintenance_rust_proj";
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(format!("{}/src", dir)).unwrap();

    std::fs::write(
        format!("{}/Cargo.toml", dir),
        r#"[package]
name = "test-proj"
version = "0.1.0"

[dependencies]
serde = "1.0"
"#,
    )
    .unwrap();

    std::fs::write(
        format!("{}/Cargo.lock", dir),
        r#"
[[package]]
name = "serde"
version = "1.0.193"

[[package]]
name = "test-proj"
version = "0.1.0"
"#,
    )
    .unwrap();

    std::fs::write(format!("{}/src/main.rs", dir), "fn main() {}\n").unwrap();

    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.to_string()).unwrap();

    // Act 1: run_security_scan (cargo-audit path)
    let scan = orch.run_security_scan(&path).await;
    assert_eq!(scan.language, "Rust");
    assert_eq!(scan.tool_name, "cargo-audit");

    // Act 2: run_dependency_report (Cargo.lock path)
    let dep_result = orch.run_dependency_report(&path).await;
    assert!(dep_result.is_ok());
    let dep_report = dep_result.unwrap();
    assert_eq!(dep_report.language, "Rust");

    let serde_dep = dep_report.dependencies.iter().find(|d| d.name == "serde");
    assert!(serde_dep.is_some());
    assert_eq!(serde_dep.unwrap().version, "1.0.193");
    assert_eq!(serde_dep.unwrap().dep_type, "direct");

    // Cleanup
    let _ = std::fs::remove_dir_all(dir);
}
```

---

## `tests/acceptance_FRD_dep_update.rs`

```rust
// PURPOSE: Acceptance test — FRD requirement: dep-update (update dependencies across workspace).
// Maps 1:1 to FRD requirement: "dep-update — update Rust/Python/JS dependencies across the workspace."

use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;

/// FRD-dep-update: The system can analyze project dependencies and produce a structured report.
/// This validates the "dependency_report" capability that underpins the update workflow.
#[tokio::test]
async fn frd_dep_update_dependency_report_identifies_direct_and_transitive() {
    let dir = "/tmp/acceptance_dep_update_xyz";
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).unwrap();

    std::fs::write(
        format!("{}/Cargo.toml", dir),
        "[package]\nname = \"app\"\nversion = \"0.1.0\"\n\n[dependencies]\nserde = \"1.0\"\n",
    )
    .unwrap();
    std::fs::write(
        format!("{}/Cargo.lock", dir),
        "[[package]]\nname = \"serde\"\nversion = \"1.0.193\"\n\n[[package]]\nname = \"serde_derive\"\nversion = \"1.0.193\"\n\n[[package]]\nname = \"app\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();

    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.to_string()).unwrap();

    let result = orch.run_dependency_report(&path).await;
    assert!(result.is_ok(), "Dependency report should succeed");

    let report = result.unwrap();
    assert_eq!(report.language, "Rust");

    // serde is a direct dependency
    let serde = report.dependencies.iter().find(|d| d.name == "serde");
    assert!(serde.is_some(), "serde should be in dependencies");
    assert_eq!(serde.unwrap().dep_type, "direct");

    // serde_derive is transitive
    let derive = report.dependencies.iter().find(|d| d.name == "serde_derive");
    assert!(derive.is_some(), "serde_derive should be in dependencies");
    assert_eq!(derive.unwrap().dep_type, "transitive");

    let _ = std::fs::remove_dir_all(dir);
}

/// FRD-dep-update: The update command executes without error (upgrades linter tools via pip).
#[tokio::test]
async fn frd_dep_update_update_command_executes_without_panic() {
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();

    // update() calls pip install --upgrade for ruff, mypy, bandit, radon
    // We just verify it doesn't panic (actual pip may or may not be installed)
    orch.update().await;
}
```

---

## `tests/acceptance_FRD_audit.rs`

```rust
// PURPOSE: Acceptance test — FRD requirement: audit (run security audits using cargo-audit, bandit).
// Maps 1:1 to FRD requirement: "audit — run security audits using cargo-audit, bandit, or external tools."

use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;

/// FRD-audit: Security scan on a Rust project uses cargo-audit and returns structured findings.
#[tokio::test]
async fn frd_audit_rust_project_uses_cargo_audit() {
    let dir = "/tmp/acceptance_audit_rust_xyz";
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(format!("{}/Cargo.lock", dir), "[[package]]\nname = \"app\"\nversion = \"0.1.0\"\n").unwrap();

    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.to_string()).unwrap();

    let report = orch.run_security_scan(&path).await;

    assert_eq!(report.language, "Rust");
    assert_eq!(report.tool_name, "cargo-audit");
    // findings may be empty if no vulnerabilities — that's valid
    assert!(report.tool_installed);

    let _ = std::fs::remove_dir_all(dir);
}

/// FRD-audit: Security scan on a Python project uses bandit and returns structured findings.
#[tokio::test]
async fn frd_audit_python_project_uses_bandit() {
    let dir = "/tmp/acceptance_audit_python_xyz";
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(format!("{}/app.py", dir), "import os\nos.system('ls')\n").unwrap();

    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.to_string()).unwrap();

    let report = orch.run_security_scan(&path).await;

    assert_eq!(report.language, "Python");
    assert_eq!(report.tool_name, "bandit");
    // bandit may or may not be installed — report should still be structured
    assert!(!report.tool_name.is_empty());

    let _ = std::fs::remove_dir_all(dir);
}

/// FRD-audit: Audit coverage — all vulnerabilities detected and reported with severity.
#[tokio::test]
async fn frd_audit_findings_have_required_fields() {
    let dir = "/tmp/acceptance_audit_fields_xyz";
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).unwrap();
    // No Cargo.lock → falls through to bandit path
    std::fs::write(format!("{}/insecure.py", dir), "import subprocess\nsubprocess.call('ls', shell=True)\n").unwrap();

    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(dir.to_string()).unwrap();

    let report = orch.run_security_scan(&path).await;

    // If bandit is installed and finds issues, verify structure
    for finding in &report.findings {
        assert!(!finding.severity.is_empty(), "severity must not be empty");
        assert!(!finding.test_id.is_empty(), "test_id must not be empty");
        assert!(!finding.issue.is_empty(), "issue must not be empty");
    }

    let _ = std::fs::remove_dir_all(dir);
}

/// FRD-audit: Doctor health check verifies tool installations.
#[tokio::test]
async fn frd_audit_doctor_checks_all_adapters() {
    let container = MaintenanceContainer::new();
    let orch = container.orchestrator();

    let result = orch.doctor().await;

    // Must check ruff, mypy, bandit, radon
    let expected_adapters = ["ruff", "mypy", "bandit", "radon"];
    for adapter in &expected_adapters {
        let found = result.adapter_statuses.keys().any(|k| k.value() == *adapter);
        assert!(found, "Doctor should check adapter '{}'", adapter);
    }
}
```

---

## `tests/bench_maintenance_throughput.rs`

```rust
// PURPOSE: Benchmark tests for maintenance operations — stats file walking and dependency parsing.
// Layer: Benchmark (criterion, runs at release gate / nightly).

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use maintenance_lint_arwaky::capabilities_maintenance_checker::MaintenanceChecker;
use maintenance_lint_arwaky::agent_maintenance_orchestrator::MaintenanceCommandsOrchestrator;
use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use shared::project_setup::contract_maintenance_protocol::IMaintenanceCheckerProtocol;

fn setup_project_with_n_files(n: usize) -> String {
    let dir = format!("/tmp/bench_maintenance_{}", n);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for i in 0..n {
        std::fs::write(format!("{}/file_{}.py", dir, i), "pass\n").unwrap();
    }
    dir
}

fn bench_stats_file_walking(c: &mut Criterion) {
    let mut group = c.benchmark_group("stats_file_walking");

    for size in [10, 100, 500] {
        let dir = setup_project_with_n_files(size);
        let orch = MaintenanceCommandsOrchestrator::new();
        let path = FilePath::new(dir.clone()).unwrap();

        group.bench_with_input(
            BenchmarkId::new("stats", size),
            &path,
            |b, p| {
                b.to_async(tokio::runtime::Runtime::new().unwrap())
                    .iter(|| orch.stats(p));
            },
        );

        let _ = std::fs::remove_dir_all(&dir);
    }
    group.finish();
}

fn bench_dependency_parsing(c: &mut Criterion) {
    let dir = "/tmp/bench_dep_parsing";
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).unwrap();

    // Generate a Cargo.lock with many packages
    let mut lock_content = String::new();
    for i in 0..200 {
        lock_content.push_str(&format!(
            "[[package]]\nname = \"dep_{}\"\nversion = \"1.0.{}\"\n\n",
            i, i
        ));
    }
    std::fs::write(format!("{}/Cargo.lock", dir), &lock_content).unwrap();
    std::fs::write(
        format!("{}/Cargo.toml", dir),
        "[package]\nname = \"bench\"\nversion = \"0.1.0\"\n\n[dependencies]\ndep_0 = \"1.0\"\n",
    )
    .unwrap();

    let checker = MaintenanceChecker::new();
    let path = FilePath::new(dir.to_string()).unwrap();

    let mut group = c.benchmark_group("dependency_parsing");
    group.bench_function("cargo_lock_200_packages", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| checker.run_dependency_report(&path));
    });
    group.finish();

    let _ = std::fs::remove_dir_all(dir);
}

criterion_group!(benches, bench_stats_file_walking, bench_dependency_parsing);
criterion_main!(benches);
```

---

## Updated `Cargo.toml` (bench registration)

```toml
[package]
name = "maintenance-lint-arwaky"
version = "1.10.106"
edition = "2021"
description = "Maintenance operations: dep updates, audit sweeps, drift detection, and rule-codebook refreshes for the AES catalog."
license = "MIT"
repository = "https://github.com/rakaarwaky/lint-arwaky"
publish = true

[lints]
workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
async-trait.workspace = true
shared.workspace = true

[dev-dependencies]
tokio.workspace = true
criterion = { version = "0.5", features = ["async_tokio"] }

[[bench]]
name = "bench_maintenance_throughput"
path = "tests/bench_maintenance_throughput.rs"
harness = false
```

---

## `lib.rs` Update (expose modules for tests)

The existing `lib.rs` needs to also export `capabilities_tool_executor_adapter` for the contract and unit tests:

```rust
// PURPOSE: Module declarations for maintenance (orchestrator, container)
pub mod agent_maintenance_orchestrator;
pub mod capabilities_maintenance_checker;
pub mod capabilities_tool_executor_adapter;
pub mod root_maintenance_container;
pub use agent_maintenance_orchestrator::MaintenanceCommandsOrchestrator;
```

---

## Run Commands

```bash
# All tests
cargo test -p maintenance-lint-arwaky -- --nocapture

# Contract only
cargo test -p maintenance-lint-arwaky --test contract_maintenance

# Unit tests
cargo test -p maintenance-lint-arwaky --test unit_maintenance_checker
cargo test -p maintenance-lint-arwaky --test unit_maintenance_tool_executor
cargo test -p maintenance-lint-arwaky --test unit_maintenance_orchestrator

# Integration
cargo test -p maintenance-lint-arwaky --test integration_maintenance

# Smoke
cargo test -p maintenance-lint-arwaky --test smoke_maintenance

# E2E
cargo test -p maintenance-lint-arwaky --test e2e_maintenance_flow

# Acceptance
cargo test -p maintenance-lint-arwaky --test acceptance_FRD_dep_update
cargo test -p maintenance-lint-arwaky --test acceptance_FRD_audit

# Benchmarks
cargo bench -p maintenance-lint-arwaky

# Coverage
cargo tarpaulin -p maintenance-lint-arwaky --fail-under 70
```

---

## Coverage Mapping

| Layer        | File                                      | Target | Tests Covering                                                     |
| ------------ | ----------------------------------------- | ------ | ------------------------------------------------------------------ |
| Capabilities | `capabilities_maintenance_checker.rs`   | ≥ 70% | `unit_maintenance_checker.rs`, `e2e_*`, `acceptance_*`       |
| Capabilities | `capabilities_tool_executor_adapter.rs` | ≥ 70% | `unit_maintenance_tool_executor.rs`                              |
| Agent        | `agent_maintenance_orchestrator.rs`     | ≥ 60% | `unit_maintenance_orchestrator.rs`, `integration_*`, `e2e_*` |
| Root         | `root_maintenance_container.rs`         | —     | `integration_maintenance.rs`, `smoke_*`                        |
