// PURPOSE: WatchOrchestrator — coordinates watch → filter → lint pipeline

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use tracing::{error, info, warn};

use shared_common::ExitCode;
use shared_file_watch::contract_watch_aggregate::IWatchAggregate;
use shared_file_watch::contract_watch_protocol::{
    IChangeFilterProtocol, IChangeLintProtocol, IWatchLifecycleProtocol,
};
use shared_file_watch::taxonomy_file_watch_request::WatchRequest;
use shared_file_watch::taxonomy_file_watch_response::WatchResponse;
use shared_file_watch::taxonomy_file_watch_vo::ChangeLintSummary;
use shared_file_watch::taxonomy_file_watch_vo::WatchConfig;
use shared_quality_rules::CodeAnalysisRequest;
use shared_quality_rules::ICodeAnalysisAggregate;

// ─── Block 1: Module Declaration ───────────────────────────

pub struct WatchOrchestrator {
    lifecycle: Arc<dyn IWatchLifecycleProtocol>,
    filter: Arc<dyn IChangeFilterProtocol>,
    lint: Arc<dyn IChangeLintProtocol>,
    linter: Arc<dyn ICodeAnalysisAggregate>,
}

impl WatchOrchestrator {
    pub fn new(
        lifecycle: Arc<dyn IWatchLifecycleProtocol>,
        filter: Arc<dyn IChangeFilterProtocol>,
        lint: Arc<dyn IChangeLintProtocol>,
        linter: Arc<dyn ICodeAnalysisAggregate>,
    ) -> Self {
        Self {
            lifecycle,
            filter,
            lint,
            linter,
        }
    }

    /// Full-project analysis run once on startup, to establish a baseline.
    ///
    /// The score is returned rather than only logged: the caller decides
    /// whether the tree it is about to watch is already failing.
    fn run_initial_lint(&self) -> InitialLint {
        let results = self
            .linter
            .execute(CodeAnalysisRequest::RunAnalysis { files: vec![] })
            .into_violations();
        let score = shared_common::Score::new(
            shared_quality_rules::utility_compliance_score::compute_score(&results),
        );
        info!(
            violations = results.len(),
            score = score.value(),
            "Initial scan complete"
        );
        let critical = results
            .iter()
            .filter(|r| r.severity == shared_common::Severity::CRITICAL)
            .count();
        InitialLint {
            violations: results.len(),
            critical,
            score: score.value(),
        }
    }

    fn run_watch_loop(&self, config: &WatchConfig, running: Arc<AtomicBool>) -> ExitCode {
        info!(
            version = shared_common::RELEASE_VERSION,
            target = config.path.value(),
            "Watch mode started, press Ctrl+C to stop"
        );

        // Policy threshold a single file must clear. Matches the `ci`
        // default; a file below it fails the run.
        const WATCH_SCORE_THRESHOLD: f64 = 80.0;

        let initial = self.run_initial_lint();
        if initial.fails_policy(WATCH_SCORE_THRESHOLD) {
            error!(
                violations = initial.violations,
                critical = initial.critical,
                score = initial.score,
                "Initial scan already fails policy — watch continues, exit code will report it"
            );
        }
        // Aggregated across the whole session: the loop reports the worst thing
        // it saw, because a single clean batch after a failing one is not a
        // passing run.
        let mut worst = SessionVerdict::from(initial);

        // Start watcher (block on async call via minimal runtime)
        let rt = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(r) => r,
            Err(e) => {
                error!(error = %e, "failed to create tokio runtime");
                return ExitCode::RUNTIME_ERROR;
            }
        };
        if let Err(e) = rt.block_on(self.lifecycle.start(config)) {
            error!(error = %e, "failed to start watcher");
            return ExitCode::RUNTIME_ERROR;
        }

        // Subscribe to file-change events
        let mut rx = self.lifecycle.subscribe();

        // Sync event loop — poll every 100ms, check running flag each iteration
        while running.load(Ordering::SeqCst) {
            match rx.try_recv() {
                Ok(event) => {
                    // Batch: collect all pending events before processing
                    let mut batch = vec![event];
                    while let Ok(ev) = rx.try_recv() {
                        batch.push(ev);
                    }

                    // FR-002: deduplicate and filter to lintable files
                    let filtered = self.filter.filter_events(batch);

                    for event in filtered {
                        // FR-003: lint the changed file. A service error is a
                        // runtime error, not a clean file — it used to be
                        // discarded by `let _ =`.
                        match self.lint.lint_changed(&event) {
                            Ok(summary) => {
                                if summary.fails_policy(WATCH_SCORE_THRESHOLD) {
                                    error!(
                                        file = %summary.path,
                                        violations = summary.violations,
                                        critical = summary.critical,
                                        score = summary.score,
                                        "Changed file fails policy"
                                    );
                                }
                                worst = worst.combine(&summary);
                            }
                            Err(e) => {
                                error!(file = %event.path, error = %e, "failed to lint changed file");
                                worst.service_errors += 1;
                            }
                        }
                    }
                }
                Err(tokio::sync::broadcast::error::TryRecvError::Empty) => {}
                Err(tokio::sync::broadcast::error::TryRecvError::Lagged(_)) => continue,
                Err(tokio::sync::broadcast::error::TryRecvError::Closed) => break,
            }
            thread::sleep(Duration::from_millis(100));
        }

        // Stop watcher — log error on failure
        if let Err(e) = rt.block_on(self.lifecycle.stop()) {
            warn!(error = %e, "failed to stop watcher cleanly");
        }
        info!(
            violations = worst.violations,
            critical = worst.critical,
            service_errors = worst.service_errors,
            "Watcher stopped"
        );
        // Exit code contract: a runtime failure (a lint that could not run)
        // outranks a policy failure; a policy failure outranks success. The
        // loop previously returned `ExitCode::OK` on every path, including
        // this one, so `--watch` in a hook could never fail a commit.
        if worst.service_errors > 0 {
            ExitCode::RUNTIME_ERROR
        } else if worst.fails_policy(WATCH_SCORE_THRESHOLD) {
            ExitCode::POLICY_FAIL
        } else {
            ExitCode::OK
        }
    }
}
// ─── Block 2: Module Contract ──────────────

impl IWatchAggregate for WatchOrchestrator {
    fn execute(&self, request: WatchRequest) -> WatchResponse {
        match request {
            WatchRequest::Run { config, running } => WatchResponse::Run {
                exit_code: self.run_watch_loop(&config, running),
            },
            WatchRequest::IsLintable { path } => WatchResponse::IsLintable {
                lintable: self.filter.is_lintable(path.value()),
            },
        }
    }
}

// ─── Block 3: Module Helpers ──────

/// What the initial full-project scan found.
#[derive(Debug, Clone, Copy)]
struct InitialLint {
    violations: usize,
    critical: usize,
    score: f64,
}

/// The worst result seen during the session.
#[derive(Debug, Clone, Copy)]
struct SessionVerdict {
    violations: usize,
    critical: usize,
    service_errors: usize,
    score: f64,
}

impl SessionVerdict {
    fn combine(&self, summary: &ChangeLintSummary) -> Self {
        Self {
            violations: self.violations + summary.violations,
            critical: self.critical + summary.critical,
            service_errors: self.service_errors,
            score: self.score.min(summary.score),
        }
    }

    fn fails_policy(&self, threshold: f64) -> bool {
        self.critical > 0 || self.score < threshold
    }
}

impl From<InitialLint> for SessionVerdict {
    fn from(initial: InitialLint) -> Self {
        Self {
            violations: initial.violations,
            critical: initial.critical,
            service_errors: 0,
            score: initial.score,
        }
    }
}

impl InitialLint {
    fn fails_policy(&self, threshold: f64) -> bool {
        self.critical > 0 || self.score < threshold
    }
}
