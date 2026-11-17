// PURPOSE: OrphanResponse — response payload for the orphan aggregate

use crate::taxonomy_orphan_rules_vo::OrphanFileListVO;
use shared_common::taxonomy_lint_vo::LintResult;
use shared_quality_rules::taxonomy_quality_rules_vo::GraphAnalysisContext;

pub enum OrphanResponse {
    GraphContext {
        context: GraphAnalysisContext,
    },
    EntryPoints {
        files: OrphanFileListVO,
    },
    Violations {
        violations: Vec<LintResult>,
    },
    ScanOutcome {
        context: GraphAnalysisContext,
        violations: Vec<LintResult>,
        /// True when the scan was cut short (e.g. by Ctrl+C) before every
        /// phase completed; the violations are the partial findings produced
        /// so far.
        partial: bool,
        /// Count of files whose content could not be read; they are treated
        /// as not-alive during the scan.
        unread_count: usize,
    },
}

impl OrphanResponse {
    /// Take the graph context. Returns an empty context if a different verb was served.
    pub fn into_graph_context(self) -> GraphAnalysisContext {
        match self {
            Self::GraphContext { context } | Self::ScanOutcome { context, .. } => context,
            _ => GraphAnalysisContext::default(),
        }
    }

    /// Take the violations. Returns an empty list if a different verb was served.
    pub fn into_violations(self) -> Vec<LintResult> {
        match self {
            Self::Violations { violations } | Self::ScanOutcome { violations, .. } => violations,
            _ => Vec::new(),
        }
    }

    /// Take both halves of a scan result. Returns an empty context and list
    /// if a different verb was served.
    pub fn into_scan_outcome(self) -> (GraphAnalysisContext, Vec<LintResult>) {
        match self {
            Self::ScanOutcome {
                context,
                violations,
                ..
            } => (context, violations),
            Self::Violations { violations } => (GraphAnalysisContext::default(), violations),
            Self::GraphContext { context } => (context, Vec::new()),
            Self::EntryPoints { .. } => (GraphAnalysisContext::default(), Vec::new()),
        }
    }

    /// Take the scan outcome with its metadata. `partial` is true when the
    /// scan was cut short (e.g. by Ctrl+C); `unread_count` reports files whose
    /// content could not be read and were treated as not-alive.
    pub fn into_detailed_scan_outcome(
        self,
    ) -> (GraphAnalysisContext, Vec<LintResult>, bool, usize) {
        match self {
            Self::ScanOutcome {
                context,
                violations,
                partial,
                unread_count,
            } => (context, violations, partial, unread_count),
            Self::Violations { violations } => {
                (GraphAnalysisContext::default(), violations, false, 0)
            }
            Self::GraphContext { context } => (context, Vec::new(), false, 0),
            Self::EntryPoints { .. } => (GraphAnalysisContext::default(), Vec::new(), false, 0),
        }
    }
}
