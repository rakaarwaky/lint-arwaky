// PURPOSE: Watch feature value objects — watch config, filesystem events, git diff results.
use serde::{Deserialize, Serialize};

use shared_common::taxonomy_common_vo::Count;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_path_vo::FilePathList;
use shared_common::taxonomy_path_vo::RenamedFileList;

// ─── Watch configuration ──────────────────────────────────────────────

pub struct WatchConfig {
    pub path: FilePath,
    pub recursive: bool,
    pub debounce_ms: u64,
    pub ignore_patterns: Vec<String>,
}

impl WatchConfig {
    pub fn from_path(path: String) -> Self {
        Self {
            path: FilePath::new(path).unwrap_or_default(),
            recursive: true,
            debounce_ms: 200,
            ignore_patterns: shared_common::DEFAULT_IGNORED_PATHS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

// ─── Filesystem change events ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WatchEventKind {
    Created,
    Modified,
    Removed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WatchEvent {
    pub path: String,
    pub kind: WatchEventKind,
    pub timestamp_ms: u64,
}

impl WatchEvent {
    pub fn new(path: String, kind: WatchEventKind) -> Self {
        let timestamp_ms = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        {
            Ok(d) => d.as_millis() as u64,
            Err(_) => 0,
        };
        Self {
            path,
            kind,
            timestamp_ms,
        }
    }
}

// ─── Git diff results ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitDiffResultVO {
    pub added: FilePathList,
    pub modified: FilePathList,
    pub deleted: FilePathList,
    pub renamed: RenamedFileList,
    pub lintable_files: FilePathList,
    pub all_files: FilePathList,
    pub total_changed: Count,
}

impl GitDiffResultVO {
    pub fn new(
        added: FilePathList,
        modified: FilePathList,
        deleted: FilePathList,
        renamed: RenamedFileList,
        lintable_files: FilePathList,
        all_files: FilePathList,
        total_changed: Count,
    ) -> Self {
        Self {
            added,
            modified,
            deleted,
            renamed,
            lintable_files,
            all_files,
            total_changed,
        }
    }
}
