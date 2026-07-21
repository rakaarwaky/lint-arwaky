// PURPOSE: Process execution utility — stateless command runner helpers
use crate::common::utility_command_runner;

/// Execute a command and return stdout/stderr/success status.
pub fn run_command(name: &str, args: &[&str]) -> (String, String, bool) {
    utility_command_runner::run_command(name, args)
}
