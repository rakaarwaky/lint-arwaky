// PURPOSE: IGitHooksAggregate — single entry point over the git-hooks domain
use crate::git_hooks::taxonomy_git_hooks_request_vo::{GitHooksRequest, GitHooksResponse};

/// Single entry point over git-hooks; the agent dispatches internally.
pub trait IGitHooksAggregate: Send + Sync {
    fn execute(&self, request: GitHooksRequest) -> GitHooksResponse;
}
