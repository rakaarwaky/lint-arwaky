// PURPOSE: FilesystemResponse — response payload for the filesystem aggregate

use crate::taxonomy_filesystem_vo::FileEntry;
use crate::taxonomy_filesystem_vo::GraphAnalysisContext;
use crate::taxonomy_filesystem_vo::ImportEntry;
use shared_common::taxonomy_common_vo::FileContentPair;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_source_vo::ContentString;
use std::collections::HashMap;
use std::path::PathBuf;

pub enum FilesystemResponse {
    Files {
        entries: Vec<FileEntry>,
    },
    Content {
        value: ContentString,
    },
    ContentOpt {
        value: Option<String>,
    },
    Has {
        exists: bool,
    },
    Entries {
        pairs: Vec<FileContentPair>,
    },
    Paths {
        paths: Vec<String>,
    },
    SourcePaths {
        paths: Vec<FilePath>,
    },
    Identifiers {
        ids: Vec<String>,
    },
    TraitsMap {
        map: HashMap<String, Vec<String>>,
    },
    Root {
        path: Option<PathBuf>,
    },
    Imports {
        entries: Vec<ImportEntry>,
    },
    GraphContext {
        context: GraphAnalysisContext,
    },
    /// Language presence flags for a project root.
    ProjectLanguages {
        languages: crate::taxonomy_filesystem_vo::ProjectLanguagesVO,
    },
    /// Result of a filesystem mutation — `true` on success, `false` on failure.
    OpOk {
        ok: bool,
    },
    /// Result of a path existence check.
    PathExists {
        exists: bool,
    },
}

impl FilesystemResponse {
    pub fn into_file_list(self) -> Vec<FileEntry> {
        match self {
            Self::Files { entries } => entries,
            _ => Vec::new(),
        }
    }

    pub fn into_content(self) -> ContentString {
        match self {
            Self::Content { value } => value,
            _ => ContentString::default(),
        }
    }

    pub fn into_content_opt(self) -> Option<String> {
        match self {
            Self::ContentOpt { value } => value,
            _ => None,
        }
    }

    pub fn into_exists(self) -> bool {
        match self {
            Self::Has { exists } => exists,
            _ => false,
        }
    }

    pub fn into_pairs(self) -> Vec<FileContentPair> {
        match self {
            Self::Entries { pairs } => pairs,
            _ => Vec::new(),
        }
    }

    pub fn into_paths(self) -> Vec<String> {
        match self {
            Self::Paths { paths } => paths,
            Self::SourcePaths { paths } => {
                paths.into_iter().map(|p| p.value().to_string()).collect()
            }
            _ => Vec::new(),
        }
    }

    pub fn into_source_paths(self) -> Vec<FilePath> {
        match self {
            Self::SourcePaths { paths } => paths,
            _ => Vec::new(),
        }
    }

    pub fn into_identifiers(self) -> Vec<String> {
        match self {
            Self::Identifiers { ids } => ids,
            _ => Vec::new(),
        }
    }

    pub fn into_traits_map(self) -> HashMap<String, Vec<String>> {
        match self {
            Self::TraitsMap { map } => map,
            _ => HashMap::new(),
        }
    }

    pub fn into_root(self) -> Option<PathBuf> {
        match self {
            Self::Root { path } => path,
            _ => None,
        }
    }

    pub fn into_imports(self) -> Vec<ImportEntry> {
        match self {
            Self::Imports { entries } => entries,
            _ => Vec::new(),
        }
    }

    pub fn into_graph_context(self) -> GraphAnalysisContext {
        match self {
            Self::GraphContext { context } => context,
            _ => GraphAnalysisContext::default(),
        }
    }

    pub fn into_project_languages(self) -> crate::taxonomy_filesystem_vo::ProjectLanguagesVO {
        match self {
            Self::ProjectLanguages { languages } => languages,
            _ => Default::default(),
        }
    }

    /// Whether a filesystem mutation succeeded.
    pub fn into_op_ok(self) -> bool {
        match self {
            Self::OpOk { ok } => ok,
            _ => false,
        }
    }

    /// Whether a path existence check passed.
    pub fn into_path_exists(self) -> bool {
        match self {
            Self::PathExists { exists } => exists,
            _ => false,
        }
    }
}
