// PURPOSE: Taxonomy types for the logging feature.
//
// All types here are Value Objects that cross layer boundaries.

use std::time::Instant;

/// The filter directive string for `LogVerbosity::Default` — returned by
/// `build_filter` when no `LINT_ARWAKY_LOG` env var is set.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FilterDirective {
    pub value: String,
}

/// One timing scope for a scan phase. Created by `IPhaseTimerProtocol::phase_started`,
/// consumed by `phase_finished` to emit the `phase_done` event.
#[derive(Clone, Debug)]
pub struct PhaseTimerVO {
    pub phase: &'static str,
    pub start: Instant,
}

/// The four reasons a walker can skip a directory. Reported verbatim in
/// `walker_skip` events so a developer sees exactly why a dir was not entered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkipReason {
    /// The dir name is on the default skip list (e.g. `target`, `.git`).
    DefaultSkipDir,
    /// The dir path matches a pattern in the config's `ignored_paths`.
    IgnoredPathPattern,
    /// The dir sits at the workspace root and is not a member dir.
    NonMemberAtWorkspaceRoot,
    /// The dir contains a `.git` file (nested worktree / submodule).
    NestedGitRepo,
}

impl SkipReason {
    /// The machine-readable reason string emitted in `walker_skip` events.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DefaultSkipDir => "default_skip_dir",
            Self::IgnoredPathPattern => "ignored_path_pattern",
            Self::NonMemberAtWorkspaceRoot => "non_member_at_ws_root",
            Self::NestedGitRepo => "nested_git_repo",
        }
    }
}

/// The walker action that a `report_walk` call describes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalkAction {
    /// The walker descended into `dir`.
    Enter,
    /// The walker skipped `dir` for the given reason.
    Skip(SkipReason),
}

/// The tracing target name used by scan-phase, walker, and adapter events.
/// All four capability protocols emit on this target.
pub const AUDIT_TARGET: &str = "lint_arwaky::audit";

/// The four reasons a walker can skip a directory. Reported verbatim in
/// `walker_skip` events so a developer sees exactly why a dir was not entered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkipReason {
    /// The dir name is on the default skip list (e.g. `target`, `.git`).
    DefaultSkipDir,
    /// The dir path matches a pattern in the config's `ignored_paths`.
    IgnoredPathPattern,
    /// The dir sits at the workspace root and is not a member dir.
    NonMemberAtWorkspaceRoot,
    /// The dir contains a `.git` file (nested worktree / submodule).
    NestedGitRepo,
}

impl SkipReason {
    /// The machine-readable reason string emitted in `walker_skip` events.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DefaultSkipDir => "default_skip_dir",
            Self::IgnoredPathPattern => "ignored_path_pattern",
            Self::NonMemberAtWorkspaceRoot => "non_member_at_ws_root",
            Self::NestedGitRepo => "nested_git_repo",
        }
    }
}

/// The walker action that a `report_walk` call describes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalkAction {
    /// The walker descended into `dir`.
    Enter,
    /// The walker skipped `dir` for the given reason.
    Skip(SkipReason),
}

/// The tracing target name used by scan-phase, walker, and adapter events.
/// All four capability protocols emit on this target.
pub const AUDIT_TARGET: &str = "lint_arwaky::audit";
