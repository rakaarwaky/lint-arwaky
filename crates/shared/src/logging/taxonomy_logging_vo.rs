// PURPOSE: Taxonomy types for the logging feature.
//
// Pure value objects with no I/O, no upward imports, no primitives in
// public/domain fields. Uses std::time::Instant (pure std module).

use std::time::Instant;

/// The filter directive string for `LogVerbosity::Default`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FilterDirective {
    pub value: String,
}

/// The tracing filter level selected by the CLI `-v` / `--verbose` flag.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LogVerbosity {
    #[default]
    Default,
    Info,
    Debug,
}

impl LogVerbosity {
    pub fn filter_directive(&self) -> &'static str {
        match self {
            Self::Default => "warn,lint_arwaky::audit=info",
            Self::Info => "info,lint_arwaky::audit=info",
            Self::Debug => "debug,lint_arwaky::audit=debug",
        }
    }

    pub fn trace_scans(&self) -> bool {
        matches!(self, Self::Info | Self::Debug)
    }
}

/// One timing scope for a scan phase.
#[derive(Clone, Debug)]
pub struct PhaseTimerVO {
    pub phase: &'static str,
    pub start: Instant,
}

/// The four reasons a walker can skip a directory.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkipReason {
    DefaultSkipDir,
    IgnoredPathPattern,
    NonMemberAtWorkspaceRoot,
    NestedGitRepo,
}

impl SkipReason {
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
    Enter,
    Skip(SkipReason),
}

/// The tracing target name used by scan-phase, walker, and adapter events.
pub const AUDIT_TARGET: &str = "lint_arwaky::audit";

/// A non-negative count of items (e.g. files discovered).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Count {
    pub value: usize,
}

/// A duration in milliseconds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DurationMs {
    pub value: u64,
}

/// An optional count of items (e.g. results within a phase).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OptionalCount(pub Option<usize>);
