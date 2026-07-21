// PURPOSE: Dependency file I/O utility — stateless helpers for reading dependency manifests
// and running external analysis tools (cargo-audit, bandit).

use std::fs;
use std::process::Command;

/// Read a file synchronously. Returns Ok(content) or Err(io::Error).
pub fn read_dependency_file(path: &std::path::Path) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

/// Execute an external command and return stdout/stderr/success status.
pub fn run_external_command(name: &str, args: &[&str]) -> (String, String, bool) {
    let output = Command::new(name).args(args).output();

    match output {
        Ok(o) => (
            String::from_utf8_lossy(&o.stdout).to_string(),
            String::from_utf8_lossy(&o.stderr).to_string(),
            o.status.success(),
        ),
        Err(e) => (
            String::new(),
            format!("Failed to execute {}: {}", name, e),
            false,
        ),
    }
}

/// Execute an external command with a working directory and return stdout/stderr/success.
pub fn run_external_command_in(
    name: &str,
    args: &[&str],
    current_dir: &str,
) -> (String, String, bool) {
    let output = Command::new(name)
        .args(args)
        .current_dir(current_dir)
        .output();

    match output {
        Ok(o) => (
            String::from_utf8_lossy(&o.stdout).to_string(),
            String::from_utf8_lossy(&o.stderr).to_string(),
            o.status.success(),
        ),
        Err(e) => (
            String::new(),
            format!("Failed to execute {}: {}", name, e),
            false,
        ),
    }
}
