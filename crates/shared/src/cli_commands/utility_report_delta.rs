// PURPOSE: utility_report_delta — compare two scans' counts. Pure arithmetic,
// no I/O, so the arithmetic can be tested without a snapshot file.
use crate::taxonomy_report_snapshot_vo::{MemberDelta, ReportDelta, ReportSnapshot};

/// Compare `now` against `previous`.
///
/// A baseline with no members is treated as no baseline: a store written by a
/// run that found nothing would otherwise report "everything appeared".
pub fn compare(now: &ReportSnapshot, previous: Option<&ReportSnapshot>) -> ReportDelta {
    let previous = previous.filter(|p| !p.members.is_empty());

    let mut members: Vec<MemberDelta> = now
        .members
        .iter()
        .map(|(member, count)| MemberDelta {
            member: member.clone(),
            now: *count,
            // A member missing from the baseline counts as having been at zero,
            // so its change is its whole count. Reporting it as unknown would
            // hide a member that appeared in the workspace.
            before: previous.map(|p| p.members.get(member).copied().unwrap_or(0)),
        })
        .collect();

    // A member that cleared entirely is still worth reporting: dropping it
    // would hide the improvement that just happened.
    if let Some(p) = previous {
        for (member, count) in &p.members {
            if !now.members.contains_key(member) {
                members.push(MemberDelta {
                    member: member.clone(),
                    now: 0,
                    before: Some(*count),
                });
            }
        }
    }

    // Heaviest member first, so the work in front of the reader is at the top.
    members.sort_by(|a, b| b.now.cmp(&a.now).then_with(|| a.member.cmp(&b.member)));

    ReportDelta {
        total_now: now.members.values().sum(),
        total_before: previous.map(|p| p.members.values().sum()),
        since: previous.map(|p| p.taken_at),
        members,
    }
}
