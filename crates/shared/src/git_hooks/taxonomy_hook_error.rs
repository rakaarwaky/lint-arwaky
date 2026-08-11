// PURPOSE: GitHookError — structured error type for git hook operation failures
use crate::domain_error_vo;

domain_error_vo!(GitHookError, "Git Hook Error");
