// PURPOSE: Process execution utility — stateless command runner helpers
use std::process::Command;

/// Execute a command and return stdout/stderr/success status.
pub fn run_command(name: &str, args: &[&str]) -> (String, String, bool) {
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
