use maintenance_lint_arwaky::capabilities_maintenance_checker::MaintenanceChecker;
use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_filesystem_maintenance_port::{
    FileEntry, IFileSystemMaintenancePort,
};
use shared::project_setup::contract_maintenance_protocol::IMaintenanceCheckerProtocol;
use shared::project_setup::contract_tool_executor_port::{IToolExecutorPort, ToolOutput};
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
    async fn run_tool_in_dir(&self, _name: &str, _args: &[&str], _dir: &FilePath) -> ToolOutput {
        ToolOutput {
            stdout: String::new(),
            stderr: String::new(),
            success: false,
        }
    }
    async fn tool_exists(&self, _name: &str) -> bool {
        false
    }
    async fn get_binary_path(&self) -> FilePath {
        FilePath::new("/usr/bin/test".to_string()).unwrap_or_default()
    }
}

struct MockFs;
#[async_trait::async_trait]
impl IFileSystemMaintenancePort for MockFs {
    async fn file_exists(&self, _path: &FilePath) -> bool {
        false
    }
    async fn read_file(&self, _path: &FilePath) -> Result<String, String> {
        Err("not found".to_string())
    }
    async fn write_file(&self, _path: &FilePath, _content: &str) -> Result<(), String> {
        Err("not supported".to_string())
    }
    async fn create_dir_all(&self, _path: &FilePath) -> Result<(), String> {
        Err("not supported".to_string())
    }
    async fn path_exists(&self, _path: &FilePath) -> bool {
        false
    }
    async fn walk_py_files(&self, _dir: &FilePath) -> Vec<FilePath> {
        Vec::new()
    }
    async fn find_cache_dirs(&self, _dir: &FilePath, _cache_names: &[&str]) -> Vec<FilePath> {
        Vec::new()
    }
    async fn remove_dir_all(&self, _path: &FilePath) -> Result<(), String> {
        Err("not supported".to_string())
    }
    async fn list_dir(&self, _dir: &FilePath) -> Vec<FileEntry> {
        Vec::new()
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
