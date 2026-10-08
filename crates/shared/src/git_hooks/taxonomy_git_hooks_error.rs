// PURPOSE: GitHookError — structured error type for git hook operation failures
use shared_common::domain_error_vo;

domain_error_vo!(GitHookError, "Git Hook Error", "GIT_HOOK", 3001u16);

#[cfg(test)]
mod tests {
    use super::*;
    use shared_common::taxonomy_message_vo::LintMessage;

    #[test]
    fn git_hook_error_has_expected_shape() {
        let err = GitHookError::new(LintMessage::new("test"));
        let _ = format!(
            "{} on {}: {}",
            "Git Hook Error", err.path.value, err.message
        );
    }
}
