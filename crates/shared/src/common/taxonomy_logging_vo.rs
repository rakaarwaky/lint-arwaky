// PURPOSE: LogVerbosity — tracing filter level selected by the CLI -v flag.
// Pure value object in shared-common so every entry point (CLI, MCP, TUI)
// and every capability that consumes it can depend on shared-common without
// a circular import.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LogVerbosity {
    /// `warn` general, `lint_arwaky::audit` at `info` — the default.
    #[default]
    Default,
    /// `info` general, `lint_arwaky::audit` at `info` — `la -v`.
    Info,
    /// `debug` general, `lint_arwaky::audit` at `debug` — `la -vv`.
    Debug,
}

impl LogVerbosity {
    /// Build the `EnvFilter` directive string for this level.
    pub fn filter_directive(&self) -> &'static str {
        match self {
            Self::Default => "warn,lint_arwaky::audit=info",
            Self::Info => "info,lint_arwaky::audit=info",
            Self::Debug => "debug,lint_arwaky::audit=debug",
        }
    }

    /// `true` when the level enables per-stage scan tracing.
    pub fn trace_scans(&self) -> bool {
        matches!(self, Self::Info | Self::Debug)
    }
}
