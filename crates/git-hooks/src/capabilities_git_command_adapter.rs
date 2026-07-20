// PURPOSE: GitCommandAdapter — IGitCommandProtocol implementation for running git commands
use std::process::Command;

use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_git_command_protocol::{GitCommandOutput, IGitCommandProtocol};

// Block 1: struct Definition
pub struct GitCommandAdapter;

// Block 2: impl Port for Struct (Public Contract)
#[async_trait::async_trait]
impl IGitCommandProtocol for GitCommandAdapter {
    async fn run_git(&self, args: &[&str], dir: &FilePath) -> GitCommandOutput {
        let output = Command::new("git")
            .args(args)
            .current_dir(dir.value())
            .output();
        match output {
            Ok(o) => GitCommandOutput {
                stdout: String::from_utf8_lossy(&o.stdout).to_string(),
                stderr: String::from_utf8_lossy(&o.stderr).to_string(),
                success: o.status.success(),
            },
            Err(_) => GitCommandOutput {
                stdout: String::new(),
                stderr: "Failed to execute git".to_string(),
                success: false,
            },
        }
    }

    async fn symbolic_ref(&self, dir: &FilePath) -> Option<String> {
        let output = self
            .run_git(&["symbolic-ref", "refs/remotes/origin/HEAD"], dir)
            .await;
        if output.success {
            let ref_str = output.stdout.trim().to_string();
            ref_str.rsplit('/').next().map(|s| s.to_string())
        } else {
            None
        }
    }

    async fn diff_name_only(&self, range: &str, dir: &FilePath) -> Vec<String> {
        let output = self.run_git(&["diff", "--name-only", range], dir).await;
        if output.success {
            output
                .stdout
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect()
        } else {
            Vec::new()
        }
    }

    async fn ls_files_modified(&self, dir: &FilePath) -> Vec<String> {
        let output = self
            .run_git(
                &["ls-files", "--modified", "--others", "--exclude-standard"],
                dir,
            )
            .await;
        if output.success {
            output
                .stdout
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect()
        } else {
            Vec::new()
        }
    }
}

// Block 3: constructors
impl GitCommandAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GitCommandAdapter {
    fn default() -> Self {
        Self::new()
    }
}
