// PURPOSE: SubscriberInit — tracing-subscriber installation capability.
// One place that builds the fmt layer, so the CLI and MCP entry points
// stay a one-line call. The TUI keeps its own file-appender subscriber;
// this capability serves the stdio entry points.

use std::sync::Once;

use shared_common::taxonomy_logging_vo::LogVerbosity;
use shared_logging::contract_logging_protocol::ISubscriberInstallProtocol;
use tracing_subscriber::EnvFilter;

// ─── Block 1: Struct Definition ───────────────────────────

/// Guards the global subscriber install: the first call installs, subsequent
/// calls are no-ops. `tracing_subscriber::fmt().init()` panics if a global
/// dispatcher is already set, so we must not re-call it.
static INIT_ONCE: Once = Once::new();

/// Installs the global tracing subscriber.
///
/// The subscriber writes to stderr, routes through an `EnvFilter` built
/// from `verbosity` (or the `LINT_ARWAKY_LOG` env var when present), and
/// optionally suppresses ANSI escapes when `with_ansi` is `false` —
/// the MCP stdio transport must never carry escape bytes.
#[derive(Default)]
pub struct SubscriberInit {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ISubscriberInstallProtocol for SubscriberInit {
    fn install(&self, verbosity: LogVerbosity, with_ansi: bool) {
        // Build the filter: LINT_ARWAKY_LOG env var overrides the flag.
        let filter = if let Ok(env_filter) = EnvFilter::try_from_env("LINT_ARWAKY_LOG") {
            env_filter
        } else {
            EnvFilter::new(verbosity.filter_directive())
        };
        INIT_ONCE.call_once(|| {
            let builder = tracing_subscriber::fmt()
                .with_env_filter(filter)
                .with_writer(std::io::stderr);
            let builder = if with_ansi {
                builder
            } else {
                builder.with_ansi(false)
            };
            builder.init();
        });
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ───────────

impl SubscriberInit {
    /// Build the `EnvFilter` for the given verbosity (without installing).
    pub fn build_filter(&self, verbosity: LogVerbosity) -> EnvFilter {
        if let Ok(env_filter) = EnvFilter::try_from_env("LINT_ARWAKY_LOG") {
            return env_filter;
        }
        EnvFilter::new(verbosity.filter_directive())
    }
}
