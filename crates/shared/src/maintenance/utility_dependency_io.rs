// PURPOSE: Dependency file I/O utility — stateless helpers for reading dependency manifests
// and running external analysis tools (cargo-audit, bandit).

use crate::common::utility_command_runner;


/// Read a file synchronously. Returns Ok(content) or Err(io::Error).
pub fn read_dependency_file(path: &std::path::Path) -> Result<String, std::io::Error> {
    crate::filesystem::utility_filesystem_io::read_file(path)
}

/// Execute an external command and return stdout/stderr/success status.
pub fn run_external_command(name: &str, args: &[&str]) -> (String, String, bool) {
    utility_command_runner::run_command(name, args)
}

/// Execute an external command with a working directory and return stdout/stderr/success.
pub fn run_external_command_in(
    name: &str,
    args: &[&str],
    current_dir: &str,
) -> (String, String, bool) {
    utility_command_runner::run_command_in_dir(name, args, Some(current_dir))
}
