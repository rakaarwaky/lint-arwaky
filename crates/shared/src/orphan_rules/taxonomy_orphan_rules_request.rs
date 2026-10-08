// PURPOSE: OrphanRequest — request payload for the orphan aggregate

use crate::taxonomy_orphan_rules_vo::OrphanFileListVO;
use shared_common::taxonomy_common_vo::PatternList;
use shared_common::taxonomy_path_vo::FilePath;
use shared_filesystem::taxonomy_filesystem_vo::FileEntry;
use shared_quality_rules::taxonomy_quality_rules_vo::GraphAnalysisContext;

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
    pub fn scan(root_dir: &FilePath, ignored: &PatternList) -> Self {
        Self::Scan {
            root_dir: root_dir.clone(),
            ignored: ignored.clone(),
        }
    }
}
