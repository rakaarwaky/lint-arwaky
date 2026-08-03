// Unit tests — ToolExecutorAdapter methods.
use shared::common::FilePath;
use shared::maintenance::IToolExecutorProtocol;

fn make_executor() -> impl IToolExecutorProtocol {
    maintenance_lint_arwaky::capabilities_tool_executor_adapter::ToolExecutorAdapter::new()
}

#[test]
fn run_tool_echo_succeeds() {
    let executor = make_executor();
    let output = executor.run_tool("echo", &["hello"]);
    assert!(output.success, "echo should succeed");
    assert!(
        output.stdout.contains("hello"),
        "stdout should contain 'hello'"
    );
}

#[test]
fn run_tool_nonexistent_fails() {
    let executor = make_executor();
    let output = executor.run_tool("nonexistent_tool_12345", &[]);
    assert!(!output.success, "Nonexistent tool should fail");
}

#[test]
fn run_tool_in_dir() {
    let executor = make_executor();
    let dir = FilePath::new("/tmp".to_string()).unwrap();
    let output = executor.run_tool_in_dir("pwd", &[], &dir);
    assert!(output.success, "pwd should succeed");
}

#[test]
fn tool_exists_echo() {
    let executor = make_executor();
    assert!(executor.tool_exists("echo"), "echo should exist");
}

#[test]
fn tool_exists_nonexistent() {
    let executor = make_executor();
    assert!(
        !executor.tool_exists("nonexistent_tool_12345"),
        "Nonexistent tool should not exist"
    );
}

#[test]
fn get_binary_path_non_empty() {
    let executor = make_executor();
    let path = executor.get_binary_path();
    assert!(!path.value.is_empty(), "Binary path should not be empty");
}

#[test]
fn executor_is_default_constructible() {
    let executor =
        maintenance_lint_arwaky::capabilities_tool_executor_adapter::ToolExecutorAdapter::default();
    let output = executor.run_tool("echo", &["test"]);
    assert!(output.success);
}
