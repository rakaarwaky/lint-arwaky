// PURPOSE: Stateless utility functions for command execution
use std::process::Command;

/// Execute a command and return `(stdout, stderr, success)`.
pub fn run_command(name: &str, args: &[&str]) -> (String, String, bool) {
    run_command_in_dir(name, args, None)
}

/// Execute a command in an optional working directory.
pub fn run_command_in_dir(
    name: &str,
    args: &[&str],
    current_dir: Option<&str>,
) -> (String, String, bool) {
    let mut command = Command::new(name);
    command.args(args);

    if let Some(dir) = current_dir {
        command.current_dir(dir);
    }

    match command.output() {
        Ok(output) => (
            String::from_utf8_lossy(&output.stdout).to_string(),
            String::from_utf8_lossy(&output.stderr).to_string(),
            output.status.success(),
        ),
        Err(error) => (
            String::new(),
            format!("Failed to execute {name}: {error}"),
            false,
        ),
    }
}
