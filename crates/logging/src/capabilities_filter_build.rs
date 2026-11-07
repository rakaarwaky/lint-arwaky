// PURPOSE: FilterBuild — mapping verbosity to EnvFilter directives.
// Pure config logic; no global state, no I/O. Testable in isolation.
// The actual subscriber install is handled by `SubscriberInit`.

use shared_logging::contract_logging_protocol::IFilterBuildProtocol;
use shared_logging::taxonomy_logging_vo::LogVerbosity;
use tracing_subscriber::EnvFilter;

// ─── Block 1: Module Declaration ───────────────────────────

#[derive(Default)]
pub struct FilterBuilder {}


impl Default for FilterBuilder {
    fn default() -> Self {{
        Self::new()
    }
}

impl FilterBuilder {
    pub fn new() -> Self {{
        Self {}
    }
}
// ─── Block 2: Module Contract ───────────────

impl IFilterBuildProtocol for FilterBuilder {
    fn build_filter(&self, verbosity: LogVerbosity) -> EnvFilter {
        if let Ok(env_filter) = EnvFilter::try_from_env("LINT_ARWAKY_LOG") {
            return env_filter;
        }
        EnvFilter::new(verbosity.filter_directive())
    }
}

// ─── Block 3: Module Helpers ───────────

