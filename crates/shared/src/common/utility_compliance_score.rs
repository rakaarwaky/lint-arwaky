// PURPOSE: Stateless utility functions for compliance score calculation
use crate::cli_commands::taxonomy_result_vo::LintResult;

/// Calculate compliance score from lint results.
///
/// Returns a value between 0.0 and 100.0 by summing the score impact of
/// each violation and subtracting from 100. The result is clamped to a
/// minimum of 0.0 so that a project with many violations never goes
/// negative.
#[rustfmt::skip]
pub fn compute_score
    (results: &[LintResult]) -> f64 {
    let penalty: f64 = results.iter().map(|r| r.severity.score_impact()).sum();
    f64::max(100.0 - penalty, 0.0)
}
