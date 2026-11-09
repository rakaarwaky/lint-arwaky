// PURPOSE: taxonomy_fix_delta_vo — compare the pre-fix and post-fix violation
// sets so a fix run reports what actually changed instead of a count
// subtraction.
//
// `collect_fix` used to derive its outcome from
// `results.len().saturating_sub(after.len())`. Two runs with the same count but
// a different rule set were therefore indistinguishable from a no-op: a fix
// that resolved one finding and introduced another reported "Fixed 0
// violations" and a policy failure, with nothing naming the regression.
//
// Comparison is by *identity* — (file, line, code) — because that is the tuple
// a maintainer acts on. A violation that moved to a different line is a new
// finding as far as the report is concerned, which is the honest reading: the
// fixer is expected to be deterministic.
use shared_common::taxonomy_lint_vo::LintResult;
use std::collections::HashSet;

/// What a fix run changed. Pure data; the caller decides the exit code.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FixDelta {
    /// Findings present before the run and absent after it.
    pub resolved: Vec<LintResult>,
    /// Findings present after the run that were not present before it.
    pub introduced: Vec<LintResult>,
    /// Findings that were fixable before the run and are still present.
    pub remaining_fixable: usize,
    /// Every finding still present after the run.
    pub after_count: usize,
}

impl FixDelta {
    /// Compare a real run's before/after sets.
    pub fn between(before: &[LintResult], after: &[LintResult]) -> Self {
        let before_keys: HashSet<(String, i64, String)> = before.iter().map(key).collect();
        let after_keys: HashSet<(String, i64, String)> = after.iter().map(key).collect();

        let resolved = before
            .iter()
            .filter(|r| !after_keys.contains(&key(r)))
            .cloned()
            .collect();
        let introduced = after
            .iter()
            .filter(|r| !before_keys.contains(&key(r)))
            .cloned()
            .collect();

        Self {
            resolved,
            introduced,
            // `remaining_fixable` is set by the caller, which knows which rule
            // codes the fixer is allowed to touch. A preview has no "after"
            // set, so it is the only meaningful count there.
            remaining_fixable: 0,
            after_count: after.len(),
        }
    }

    /// A dry-run preview: nothing changed, so everything fixable remains.
    pub fn preview(before: &[LintResult], fixable: &[LintResult]) -> Self {
        Self {
            resolved: Vec::new(),
            introduced: Vec::new(),
            remaining_fixable: fixable.len(),
            after_count: before.len(),
        }
    }

    /// Record how many fixable findings survived the run.
    pub fn with_remaining_fixable(mut self, remaining: usize) -> Self {
        self.remaining_fixable = remaining;
        self
    }

    /// True when nothing fixable is left and no new finding appeared.
    pub fn is_clean(&self) -> bool {
        self.introduced.is_empty() && self.remaining_fixable == 0
    }
}

/// The identity a violation is compared by.
fn key(result: &LintResult) -> (String, i64, String) {
    (
        result.file.value.clone(),
        result.line.value(),
        result.code.code().to_string(),
    )
}
