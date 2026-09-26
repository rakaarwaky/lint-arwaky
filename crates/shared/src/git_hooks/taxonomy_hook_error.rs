// PURPOSE: GitHookError — structured error type for git hook operation failures
use crate::domain_error_vo;

/// Domain error types for the git hooks subsystem.
/// Uses the `domain_error_vo!` macro from `utility_value_object_generator`.
pub fn _hook_error_anchor() {}

domain_error_vo!(GitHookError, "Git Hook Error");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::taxonomy_message_vo::LintMessage;

    #[test]
    fn git_hook_error_has_expected_shape() {
        let err = GitHookError::new(LintMessage::new("test"));
        let _ = format!(
            "{} on {}: {}",
            "Git Hook Error", err.path.value, err.message
        );
    }
}
