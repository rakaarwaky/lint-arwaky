// PURPOSE: Stateless tool-execution utility — wraps subprocess calls so unit
// tests can exercise executor logic without touching the filesystem IO layer.
// Not a business capability; no corresponding FR.

use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_tool_name_vo::ToolName;
use crate::maintenance::taxonomy_maintenance_vo::ToolOutput;
use std::process::Command;

/// Run a tool from PATH and return its output.
pub fn run_tool(name: &ToolName, args: &[&str]) -> ToolOutput {
    match Command::new(name.value()).args(args).output() {
        Ok(o) => ToolOutput {
            stdout: String::from_utf8_lossy(&o.stdout).to_string(),
            stderr: String::from_utf8_lossy(&o.stderr).to_string(),
            success: o.status.success(),
        },
        Err(_) => ToolOutput {
            stdout: String::new(),
            stderr: format!("Failed to execute {}", name.value()),
            success: false,
        },
    }
}

/// Run a tool in a specific directory and return its output.
pub fn run_tool_in_dir(name: &ToolName, args: &[&str], dir: &FilePath) -> ToolOutput {
    match Command::new(name.value())
        .args(args)
        .current_dir(&dir.value)
        .output()
    {
        Ok(o) => ToolOutput {
            stdout: String::from_utf8_lossy(&o.stdout).to_string(),
            stderr: String::from_utf8_lossy(&o.stderr).to_string(),
            success: o.status.success(),
        },
        Err(_) => ToolOutput {
            stdout: String::new(),
            stderr: format!("Failed to execute {} in {}", name.value(), dir.value),
            success: false,
        },
    }
}

/// Check whether a tool exists on PATH (via `which`).
pub fn tool_exists(name: &ToolName) -> bool {
    Command::new("which")
        .arg(name.value())
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Return the path of the running binary.
pub fn get_binary_path() -> FilePath {
    let path = std::env::current_exe()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    FilePath::new(path).unwrap_or_default()
}
