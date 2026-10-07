// PURPOSE: logging — tracing subscriber init + verbose-flag plumbing.
// Centralizes EnvFilter construction, audit-target constant, and the
// LogVerbosity type so CLI, MCP, and TUI entry points share one
// filter string instead of duplicating "warn,lint_arwaky::audit=info".

pub mod agent_logging_orchestrator;
pub mod capabilities_filter_build;
pub mod capabilities_phase_timer;
pub mod capabilities_subscriber_init;
pub mod capabilities_walker_reporter;
pub mod root_logging_container;

pub use agent_logging_orchestrator::LoggingOrchestrator;
pub use capabilities_subscriber_init::SubscriberInit;
pub use root_logging_container::LoggingContainer;

// Re-exported so CLI and MCP entry points can import from this crate
// without adding a second shared-common import.
pub use shared_logging::LogVerbosity;
pub use shared_logging::AUDIT_TARGET;
pub use shared_logging::ILoggingAggregate;
pub use shared_logging::ISubscriberInitProtocol;
