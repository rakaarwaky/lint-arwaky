// PURPOSE: GitHookError — structured error type for git hook operation failures
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone)]
pub struct GitHookError {
    pub path: FilePath,
    pub message: String,
}

impl GitHookError {
    pub fn new(message: crate::taxonomy_common_error::ErrorMessage) -> Self {
        Self {
            path: FilePath::default(),
            message: message.value,
        }
    }
}

impl std::fmt::Display for GitHookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Git Hook Error on {}: {}", self.path.value, self.message)
    }
}

impl std::error::Error for GitHookError {}
