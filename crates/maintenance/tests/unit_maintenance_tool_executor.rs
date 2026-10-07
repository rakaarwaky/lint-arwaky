// Unit tests — tool-executor utility functions.
use shared_common::FilePath;
use shared_common::taxonomy_adapter_name_vo::ToolName;
use shared_maintenance::utility_tool_executor::{
    get_binary_path, run_tool, run_tool_in_dir, tool_exists,
};

#[test]
fn run_tool_echo_succeeds() {
    let output = run_tool(&ToolName::new("echo"), &["hello"]);
    assert!(output.success, "echo should succeed");
    assert!(
        output.stdout.contains("hello"),
        "stdout should contain 'hello'"
    );
}

#[test]
fn run_tool_nonexistent_fails() {
    let output = run_tool(&ToolName::new("nonexistent_tool_12345"), &[]);
    assert!(!output.success, "Nonexistent tool should fail");
}

#[test]
fn run_tool_in_dir_succeeds() {
    let dir = FilePath::new("/tmp".to_string()).unwrap();
    let output = run_tool_in_dir(&ToolName::new("pwd"), &[], &dir);
    assert!(output.success, "pwd should succeed");
}

#[test]
fn tool_exists_echo() {
    assert!(tool_exists(&ToolName::new("echo")), "echo should exist");
}

#[test]
fn tool_exists_nonexistent() {
    assert!(
        !tool_exists(&ToolName::new("nonexistent_tool_12345")),
        "Nonexistent tool should not exist"
    );
}

#[test]
fn get_binary_path_non_empty() {
    let path = get_binary_path();
    assert!(!path.value.is_empty(), "Binary path should not be empty");
}

#[test]
fn run_tool_direct_call() {
    let output = run_tool(&ToolName::new("echo"), &["test"]);
    assert!(output.success);
}
