// PURPOSE: NamingResponse — response payload for the naming aggregate

use crate::taxonomy_naming_audit_stats_vo::NamingAuditStats;
use shared_common::taxonomy_violation_item_vo::ViolationItem;

pub enum NamingResponse {
    /// Violations found by the audit, with the per-rule counts of the run.
    ///
    /// The counts travel with the findings because an empty finding list used
    /// to mean two different things — "nothing is wrong" and "nothing was
    /// checked" — and only the counts tell them apart.
    Audit {
        violations: Vec<ViolationItem>,
        stats: NamingAuditStats,
    },
    /// Adapter name of the orchestrator.
    Name { name: String },
}

impl NamingResponse {
    /// Take the audit violations, or an empty list if a different verb was served.
    pub fn into_violations(self) -> Vec<ViolationItem> {
        match self {
            Self::Audit { violations, .. } => violations,
            Self::Name { .. } => Vec::new(),
        }
    }

    /// Take the audit findings together with the per-rule counts.
    ///
    /// `None` when a different verb was served.
    pub fn into_audit(self) -> Option<(Vec<ViolationItem>, NamingAuditStats)> {
        match self {
            Self::Audit { violations, stats } => Some((violations, stats)),
            Self::Name { .. } => None,
        }
    }

    /// Take the adapter name, or an empty string if a different verb was served.
    pub fn into_name(self) -> String {
        match self {
            Self::Name { name } => name,
            Self::Audit { .. } => String::new(),
        }
    }
}
