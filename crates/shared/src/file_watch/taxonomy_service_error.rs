// PURPOSE: WatchServiceError — structured error type for file watch service failures
use crate::domain_error_vo;

domain_error_vo!(WatchServiceError, "Watch Error");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::taxonomy_message_vo::LintMessage;

    #[test]
    fn watch_service_error_has_expected_shape() {
        let err = WatchServiceError::new(LintMessage::new("test"));
        let _ = format!("{} on {}: {}", "Watch Error", err.path.value, err.message);
    }
}
