// PURPOSE: logging capability contracts — four capability seams plus the
// aggregate entry the orchestrator exposes to the surface.

use crate::taxonomy_logging_vo::{FilterDirective, PhaseTimerVO, SkipReason};
use shared_common::taxonomy_logging_vo::LogVerbosity;

/// FR-Logging-001: Build the filter directives.
///
/// Maps a `LogVerbosity` to the `EnvFilter` that decides which events reach
/// the terminal. An explicitly-set `LINT_ARWAKY_LOG` env var overrides the
/// flag (operator override — useful in CI where the flag is not passed).
pub trait IFilterBuildProtocol: Send + Sync {
    fn build_filter(&self, verbosity: LogVerbosity) -> tracing_subscriber::EnvFilter;
}

/// FR-Logging-002: Install the global subscriber.
///
/// Installs the process-wide `tracing_subscriber` fmt layer, writing to
/// stderr, with the filter from FR-Logging-001. Set-once: a second call
/// is a silent no-op.
pub trait ISubscriberInstallProtocol: Send + Sync {
    fn install(&self, verbosity: LogVerbosity, with_ansi: bool);
}

/// FR-Logging-003: Time a scan phase.
///
/// `phase_started` opens a timing scope; `phase_finished` closes it and
/// emits `{phase, elapsed_ms, count?}` on the audit target. The timer is
/// a scope guard — the end marker fires even if the phase panics.
pub trait IPhaseTimerProtocol: Send + Sync {
    fn phase_started(&self, phase: &'static str) -> PhaseTimerVO;
    fn phase_finished(&self, timer: &PhaseTimerVO, count: Option<usize>);
}

/// FR-Logging-004: Report walker progress.
///
/// Emits one event per directory the walker enters or skips, plus the
/// walk total. Skip reasons are reported verbatim so a developer watching
/// `scan -v` sees exactly why a dir was not entered.
pub trait IWalkerReportProtocol: Send + Sync {
    fn walker_enter(&self, dir: &str);
    fn walker_skip(&self, dir: &str, reason: SkipReason);
    fn files_discovered(&self, count: usize, elapsed_ms: u64);
}
