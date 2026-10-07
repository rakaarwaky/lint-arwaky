// PURPOSE: FilterBuild — mapping verbosity to EnvFilter directives.
// Pure config logic; no global state, no I/O. Testable in isolation.
// The actual subscriber install is handled by `SubscriberInit`.

use shared_logging::taxonomy_logging_vo::LogVerbosity;
use shared_logging::contract_logging_protocol::IFilterBuildProtocol;
use tracing_subscriber::EnvFilter;

// ─── Block 1: Struct Definition ───────────────────────────

#[derive(Default)]
pub struct FilterBuilder {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IFilterBuildProtocol for FilterBuilder {
    fn build_filter(&self, verbosity: LogVerbosity) -> EnvFilter {
        if let Ok(env_filter) = EnvFilter::try_from_env("LINT_ARWAKY_LOG") {
            return env_filter;
        }
        EnvFilter::new(verbosity.filter_directive())
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ───────────

impl FilterBuilder {
    pub fn new() -> Self {
        Self {}
    }
}
