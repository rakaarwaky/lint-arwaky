// PURPOSE: VersionCommandsSurface — version info business logic, no formatting.
// Returns the crate version at compile time.

/// Version report — formatted by CLI/MCP/TUI surfaces.
#[derive(Debug, Clone)]
pub struct VersionReport {
    pub version: String,
    pub edition: String,
}

/// Collect version info from compile-time environment.
pub fn collect_version() -> VersionReport {
    VersionReport {
        version: env!("CARGO_PKG_VERSION").to_string(),
        edition: env!("CARGO_PKG_RUST_VERSION").to_string(),
    }
}
