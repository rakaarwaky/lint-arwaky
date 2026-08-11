// PURPOSE: WatchServiceError — structured error type for file watch service failures
use crate::domain_error_vo;

/// Domain error types for the file watch subsystem.
/// Uses the `domain_error_vo!` macro from `utility_value_object_generator`.
#[allow(clippy::needless_pass_by_value, dead_code)]
pub fn _watch_error_anchor() {}

domain_error_vo!(WatchServiceError, "Watch Error");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::taxonomy_message_vo::LintMessage;

    #[test]
    fn watch_service_error_has_expected_shape() {
        let err = WatchServiceError::new(LintMessage::new("test"));
        let _ =
            format!("{}: on {}: {}", "Watch Error", err.path.value, err.message);
    }
}

// ─── anchor: file minimum length (5 lines) ───

