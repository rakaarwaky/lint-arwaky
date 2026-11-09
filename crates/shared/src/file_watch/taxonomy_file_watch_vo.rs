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

/// User-facing lifecycle summaries emitted by the watch runner. The CLI owns
/// rendering; the watcher reports structured events instead of relying on logs.
#[derive(Debug, Clone, PartialEq)]
pub enum WatchOutputEvent {
    Started { path: String },
    InitialScanComplete { violations: usize, score: f64 },
    FileLinted { path: String, succeeded: bool },
    Stopped,
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

/// What one changed-file lint found.
///
/// `lint_changed` used to return `Result<(), WatchServiceError>`, so the
/// orchestrator could not tell a clean file from a file with a CRITICAL
/// violation and reported `ExitCode::OK` unconditionally — a pre-commit hook
/// running `--watch` could never fail.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ChangeLintSummary {
    /// Path that was linted.
    pub path: String,
    /// Violations in that file.
    pub violations: usize,
    /// CRITICAL violations in that file.
    pub critical: usize,
    /// Compliance score for that file (0–100).
    pub score: f64,
}

impl ChangeLintSummary {
    pub fn new(path: impl Into<String>, violations: usize, critical: usize, score: f64) -> Self {
        Self {
            path: path.into(),
            violations,
            critical,
            score,
        }
    }

    /// Whether this file fails the policy gate on its own.
    ///
    /// Any CRITICAL is an unconditional fail — the same rule `ci` applies — and
    /// anything else fails below the default score threshold.
    pub fn fails_policy(&self, threshold: f64) -> bool {
        self.critical > 0 || self.score < threshold
    }
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
