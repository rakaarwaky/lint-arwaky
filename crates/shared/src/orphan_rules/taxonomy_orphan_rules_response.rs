// PURPOSE: OrphanResponse — response payload for the orphan aggregate

use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::orphan_rules::taxonomy_orphan_rules_vo::OrphanFileListVO;
use crate::quality_rules::taxonomy_quality_rules_vo::GraphAnalysisContext;

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

    pub fn into_entry_points(self) -> OrphanFileListVO {
        match self {
            Self::EntryPoints { files } => files,
            _ => OrphanFileListVO::new(Vec::<String>::new()),
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
            } => (context, violations),
            Self::Violations { violations } => (GraphAnalysisContext::default(), violations),
            Self::GraphContext { context } => (context, Vec::new()),
            Self::EntryPoints { .. } => (GraphAnalysisContext::default(), Vec::new()),
        }
    }
}
