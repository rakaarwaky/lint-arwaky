// PURPOSE: CRITICAL severity detection — pure helper

use shared::cli_commands::LintResult;
use shared::common::Severity;

/// Returns true if any result has CRITICAL severity.
pub fn contains_critical_severity(results: &[LintResult]) -> bool {
    results.iter().any(|r| r.severity == Severity::CRITICAL)
}
