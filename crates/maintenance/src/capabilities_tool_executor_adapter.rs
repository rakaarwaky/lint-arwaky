use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_tool_executor_protocol::{IToolExecutorProtocol, ToolOutput};

// PURPOSE: ToolExecutorAdapter — IToolExecutorProtocol implementation for running external tools
use std::process::Command;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ToolExecutorAdapter;

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait::async_trait]
impl IToolExecutorProtocol for ToolExecutorAdapter {
    async fn run_tool(&self, name: &str, args: &[&str]) -> ToolOutput {
        let output = Command::new(name).args(args).output();
        match output {
            Ok(o) => ToolOutput {
                stdout: String::from_utf8_lossy(&o.stdout).to_string(),
                stderr: String::from_utf8_lossy(&o.stderr).to_string(),
                success: o.status.success(),
            },
            Err(_) => ToolOutput {
                stdout: String::new(),
                stderr: format!("Failed to execute {}", name),
                success: false,
            },
        }
    }

    async fn run_tool_in_dir(&self, name: &str, args: &[&str], dir: &FilePath) -> ToolOutput {
        let output = Command::new(name)
            .args(args)
            .current_dir(dir.value())
            .output();
        match output {
            Ok(o) => ToolOutput {
                stdout: String::from_utf8_lossy(&o.stdout).to_string(),
                stderr: String::from_utf8_lossy(&o.stderr).to_string(),
                success: o.status.success(),
            },
            Err(_) => ToolOutput {
                stdout: String::new(),
                stderr: format!("Failed to execute {} in {}", name, dir),
                success: false,
            },
        }
    }

    async fn tool_exists(&self, name: &str) -> bool {
        Command::new("which")
            .arg(name)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    async fn get_binary_path(&self) -> FilePath {
        let path = std::env::current_exe()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        FilePath::new(path).unwrap_or_default()
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for ToolExecutorAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolExecutorAdapter {
    pub fn new() -> Self {
        Self
    }
}
