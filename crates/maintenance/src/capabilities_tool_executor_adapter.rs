use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_tool_name_vo::ToolName;
use shared::maintenance::contract_maintenance_protocol::{IToolExecutorProtocol, ToolOutput};
use std::process::Command;

pub struct ToolExecutorAdapter;

impl IToolExecutorProtocol for ToolExecutorAdapter {
    fn run_tool(&self, name: &ToolName, args: &[&str]) -> ToolOutput {
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

    fn run_tool_in_dir(&self, name: &ToolName, args: &[&str], dir: &FilePath) -> ToolOutput {
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

    fn tool_exists(&self, name: &ToolName) -> bool {
        Command::new("which")
            .arg(name.value())
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn get_binary_path(&self) -> FilePath {
        let path = std::env::current_exe()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        FilePath::new(path).unwrap_or_default()
    }
}

impl ToolExecutorAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ToolExecutorAdapter {
    fn default() -> Self {
        Self::new()
    }
}
