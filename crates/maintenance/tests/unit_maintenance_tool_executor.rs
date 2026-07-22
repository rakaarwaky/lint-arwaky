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
