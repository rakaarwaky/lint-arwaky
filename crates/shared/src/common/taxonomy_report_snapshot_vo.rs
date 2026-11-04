// PURPOSE: report's snapshot types — what one scan counted, and the store of
// every target's counts. Data lives here because a `utility_` file must hold no
// type definitions (AES404).
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A scan's per-member counts, plus when it was taken.
///
/// Member paths are the report's `{top}/{member}` keys, so a member counts the
/// same whether the scan covered one member or the whole workspace.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReportSnapshot {
    /// Unix seconds of the scan that produced these counts.
    pub taken_at: i64,
    /// Violation count per member, keyed by the report's member path.
    pub members: BTreeMap<String, usize>,
}

/// Every target's last snapshot, in one file.
///
/// One entry per target, because a snapshot written for one target would
/// otherwise become the baseline for another and report a false delta.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotStore {
    pub entries: BTreeMap<String, ReportSnapshot>,
}

/// One member's row: its count now, its count before, and the change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberDelta {
    pub member: String,
    pub now: usize,
    /// `None` only when there is no baseline at all. A member missing from the
    /// baseline is `Some(0)`, so it is measured from zero rather than reported
    /// as unknown — reporting it unknown would hide a new member.
    pub before: Option<usize>,
}

impl MemberDelta {
    /// The change since the baseline, or `None` when there is no baseline.
    pub fn change(&self) -> Option<i64> {
        self.before.map(|b| self.now as i64 - b as i64)
    }
}

/// The whole comparison, plus the totals a reader checks first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportDelta {
    pub members: Vec<MemberDelta>,
    pub total_now: usize,
    pub total_before: Option<usize>,
    /// When the baseline was taken, for the "since" line.
    pub since: Option<i64>,
}

impl ReportDelta {
    pub fn total_change(&self) -> Option<i64> {
        self.total_before.map(|b| self.total_now as i64 - b as i64)
    }

    /// Violations in the baseline that are gone now.
    pub fn fixed(&self) -> usize {
        self.members
            .iter()
            .filter(|m| m.before.is_some_and(|b| b > 0) && m.now < m.before.unwrap_or(0))
            .map(|m| m.before.unwrap_or(0) - m.now)
            .sum()
    }

    /// Violations present now that were not in the baseline.
    pub fn introduced(&self) -> usize {
        self.members
            .iter()
            .filter(|m| m.before.is_none_or(|b| b < m.now))
            .map(|m| m.now - m.before.unwrap_or(0))
            .sum()
    }
}
