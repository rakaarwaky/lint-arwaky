
use std::marker::PhantomData;

use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_git_command_protocol::{GitCommandOutput, IGitCommandProtocol};
use shared::git_hooks::utility_git_io as git_io;

// PURPOSE: GitCommandAdapter — IGitCommandProtocol implementation for running git commands

// ─── Block 1: Struct Definition ───────────────────────────

pub struct GitCommandAdapter {
    _p: PhantomData<()>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait::async_trait]
impl IGitCommandProtocol for GitCommandAdapter {
    async fn run_git(&self, args: &[&str], dir: &FilePath) -> GitCommandOutput {
        let (stdout, stderr, success) = git_io::run_git_command(args, &dir.value);
        GitCommandOutput {
            stdout,
            stderr,
            success,
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

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl GitCommandAdapter {
    pub fn new() -> Self {
        Self { _p: PhantomData }
    }
}

impl Default for GitCommandAdapter {
    fn default() -> Self {
        Self::new()
    }
}

