// PURPOSE: OrphanRequest/OrphanResponse — request/response VOs for the orphan aggregate
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;
use crate::orphan_rules::taxonomy_orphan_contract_vo::OrphanFileListVO;
use crate::quality_rules::taxonomy_analysis_vo::GraphAnalysisContext;

/// Consumer verb carried by the orphan aggregate's single entry point.
pub enum OrphanRequest {
    /// Build the import/reachability graph for a set of files.
    BuildGraphContext {
        files: OrphanFileListVO,
        root_dir: FilePath,
    },
    /// Resolve which files are reachable entry points.
    IdentifyEntryPoints { files: OrphanFileListVO },
    /// Run orphan detection, building the graph context inline.
    Check {
        files: OrphanFileListVO,
        root_dir: FilePath,
    },
    /// Run orphan detection against an already-built graph context.
    CheckWithContext {
        files: OrphanFileListVO,
        root_dir: FilePath,
        context: GraphAnalysisContext,
    },
    /// Run orphan detection on pre-parsed file entries from the filesystem crate.
    CheckWithEntries {
        files: Vec<FileEntry>,
        context: GraphAnalysisContext,
    },
    /// Build the graph context and run detection in one pass.
    Scan {
        root_dir: FilePath,
        ignored: PatternList,
    },
}

impl OrphanRequest {
    pub fn build_graph_context(files: &OrphanFileListVO, root_dir: &FilePath) -> Self {
        Self::BuildGraphContext {
            files: files.clone(),
            root_dir: root_dir.clone(),
        }
    }
    pub fn identify_entry_points(files: &OrphanFileListVO) -> Self {
        Self::IdentifyEntryPoints {
            files: files.clone(),
        }
    }
    pub fn check(files: &OrphanFileListVO, root_dir: &FilePath) -> Self {
        Self::Check {
            files: files.clone(),
            root_dir: root_dir.clone(),
        }
    }
    pub fn check_with_context(
        files: &OrphanFileListVO,
        root_dir: &FilePath,
        context: &GraphAnalysisContext,
    ) -> Self {
        Self::CheckWithContext {
            files: files.clone(),
            root_dir: root_dir.clone(),
            context: context.clone(),
        }
    }
    pub fn check_with_entries(files: &[FileEntry], context: &GraphAnalysisContext) -> Self {
        Self::CheckWithEntries {
            files: files.to_vec(),
            context: context.clone(),
        }
    }
    pub fn scan(root_dir: &FilePath, ignored: &PatternList) -> Self {
        Self::Scan {
            root_dir: root_dir.clone(),
            ignored: ignored.clone(),
        }
    }
}

/// Result of an orphan aggregate request.
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
