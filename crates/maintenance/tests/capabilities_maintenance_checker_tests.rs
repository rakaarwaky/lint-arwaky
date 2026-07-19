use maintenance_lint_arwaky::capabilities_maintenance_checker::MaintenanceChecker;
use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_filesystem_maintenance_port::IFileSystemMaintenancePort;
use shared::project_setup::contract_maintenance_protocol::IMaintenanceCheckerProtocol;
use shared::project_setup::contract_tool_executor_port::IToolExecutorPort;
use shared::project_setup::taxonomy_doctor_vo::ToolOutput;
use std::sync::Arc;

struct MockToolExecutor;
#[async_trait::async_trait]
impl IToolExecutorPort for MockToolExecutor {
    async fn run_tool(&self, _name: &str, _args: &[&str]) -> ToolOutput {
        ToolOutput {
            stdout: "1.0.0".to_string(),
            stderr: String::new(),
            success: true,
        }
    }
    async fn run_tool_in_dir(&self, _name: &str, _args: &[&str], _dir: &str) -> ToolOutput {
        ToolOutput {
            stdout: String::new(),
            stderr: String::new(),
            success: false,
        }
    }
    async fn get_binary_path(&self) -> String {
        "/usr/bin/test".to_string()
    }
}

struct MockFs;
#[async_trait::async_trait]
impl IFileSystemMaintenancePort for MockFs {
    async fn file_exists(&self, _path: &str) -> bool {
        false
    }
    async fn read_file(&self, _path: &str) -> Result<String, String> {
        Err("not found".to_string())
    }
}

fn make_checker() -> MaintenanceChecker {
    MaintenanceChecker::new(Arc::new(MockToolExecutor), Arc::new(MockFs))
}

#[tokio::test]
async fn test_diagnose_toolchain_returns_cargo() {
    let checker = make_checker();
    let diag = checker.diagnose_toolchain().await;
    assert!(!diag.rust_tools.is_empty());
    let cargo = &diag.rust_tools[0];
    assert_eq!(cargo.name, "cargo");
    assert_eq!(cargo.status, "OK");
}

#[tokio::test]
async fn test_diagnose_toolchain_has_git() {
    let checker = make_checker();
    let diag = checker.diagnose_toolchain().await;
    let git = diag.vcs_tools.iter().find(|t| t.name == "git");
    assert!(git.is_some());
    assert_eq!(git.unwrap().status, "OK");
}

#[tokio::test]
async fn test_diagnose_toolchain_returns_non_empty_sections() {
    let checker = make_checker();
    let diag = checker.diagnose_toolchain().await;
    assert!(!diag.python_tools.is_empty());
    assert!(!diag.js_tools.is_empty());
}

#[tokio::test]
async fn test_dependency_report_no_lockfile() {
    let checker = make_checker();
    let path = FilePath::new("/nonexistent_path".to_string()).unwrap_or_default();
    let result = checker.run_dependency_report(&path).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_security_scan_returns_report() {
    let checker = make_checker();
    let path = FilePath::new("/nonexistent_path".to_string()).unwrap_or_default();
    let report = checker.run_security_scan(&path).await;
    assert_eq!(report.language, "Python");
    assert_eq!(report.tool_name, "bandit");
}
