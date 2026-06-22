// PURPOSE: GitDiffDataVO — value object representing semantic diff data between two file versions
use crate::common::taxonomy_common_vo::Count;
use serde::{Deserialize, Serialize};

/// Semantic status of the diff between two file versions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GitDiffStatus {
    /// Files are byte-identical (or content-identical after normalization).
    Unchanged,
    /// Files differ in content.
    Modified,
    /// Path1 does not exist.
    MissingFirst,
    /// Path2 does not exist.
    MissingSecond,
    /// Either path is not a regular file.
    NotAFile,
}

/// One side of a two-file diff (path1 or path2 in the original HashMap key
/// "version1" / "version2"). The score is reserved for future use (currently
/// always 0.0); kept as a field so callers do not have to introduce a new VO
/// once we wire up a real similarity score.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitDiffSideVO {
    pub path: String,
    pub similarity_score: f64,
}

impl GitDiffSideVO {
    pub fn new(path: impl Into<String>, similarity_score: f64) -> Self {
        Self {
            path: path.into(),
            similarity_score,
        }
    }
}

/// Strongly-typed replacement for the previous
/// `HashMap<String, serde_json::Value>` return type. Each field has a real
/// domain meaning — there is no `serde_json::Value` in the contract surface.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitDiffDataVO {
    pub version1: GitDiffSideVO,
    pub version2: GitDiffSideVO,
    /// 0.0 when files are identical; positive number otherwise.
    /// Concrete unit (line count? byte count? semantic diff?) is left to the
    /// caller to populate; the contract only requires a non-negative number.
    pub difference: f64,
    pub status: GitDiffStatus,
}

impl GitDiffDataVO {
    pub fn unchanged(version1_path: impl Into<String>, version2_path: impl Into<String>) -> Self {
        Self {
            version1: GitDiffSideVO::new(version1_path, 1.0),
            version2: GitDiffSideVO::new(version2_path, 1.0),
            difference: 0.0,
            status: GitDiffStatus::Unchanged,
        }
    }

    pub fn modified(
        version1_path: impl Into<String>,
        version2_path: impl Into<String>,
        difference: f64,
    ) -> Self {
        Self {
            version1: GitDiffSideVO::new(version1_path, 0.0),
            version2: GitDiffSideVO::new(version2_path, 0.0),
            difference,
            status: GitDiffStatus::Modified,
        }
    }
}

/// One ignore-rule update request passed to `IHookProtocol::update_ignore_rule`.
/// Mirrors the previous `(rule: &str, remove: bool, config_path: &str)`
/// positional signature but uses VOs.
#[derive(Debug, Clone)]
pub struct HookIgnoreUpdateVO {
    pub rule: String,
    pub remove: bool,
    pub config_path: String,
}

impl HookIgnoreUpdateVO {
    pub fn new(rule: impl Into<String>, remove: bool, config_path: impl Into<String>) -> Self {
        Self {
            rule: rule.into(),
            remove,
            config_path: config_path.into(),
        }
    }
}

/// Diff statistics reported as a count, not a raw float.
#[allow(dead_code)]
pub type DiffLineCount = Count;
