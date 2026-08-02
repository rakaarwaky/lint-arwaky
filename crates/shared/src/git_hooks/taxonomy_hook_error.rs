// PURPOSE: GitHookError — structured error type for git hook operation failures
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone)]
pub struct GitHookError {
    pub path: FilePath,
    pub message: LintMessage,
}

impl GitHookError {
    pub fn new(message: LintMessage) -> Self {
        Self {
            path: FilePath::default(),
            message,
        }
    }
}

impl std::fmt::Display for GitHookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Git Hook Error on {}: {}", self.path.value, self.message)
    }
}

impl std::error::Error for GitHookError {}
