use std::path::{Path, PathBuf};

use crate::infrastructure::mcp_server_constants::{AUTO_LINT_VERSION, MCP_SERVER_VERSION};
use crate::taxonomy::{DirectoryPath, FilePath};
use tracing::info;

/// Context object yielded by the lifespan manager.
#[derive(Debug, Clone)]
pub struct WrapperContext {
    /// Container provided by surface (decoupled from ServiceContainerAggregate).
    pub container: crate::agent::dependency_injection_container::Container,
    /// Project root path.
    pub project_root: PathBuf,
    /// Server version.
    pub server_version: String,
    /// Auto-lint version.
    pub auto_lint_version: String,
}

/// MCP server lifespan:
/// - Container provided by surface
/// - Version context
/// - Cleanup on shutdown
pub async fn mcp_server_lifespan(
    container: crate::agent::dependency_injection_container::Container,
    project_root: DirectoryPath,
) -> WrapperContext {
    let root = Path::new(&project_root.value).resolve();
    info!("MCP server starting up — project_root={}", root.display());

    let ctx = WrapperContext {
        container,
        project_root: root,
        server_version: MCP_SERVER_VERSION.to_string(),
        auto_lint_version: AUTO_LINT_VERSION.to_string(),
    };
    info!(
        "MCP server lifespan: container initialized, version={}",
        MCP_SERVER_VERSION
    );

    ctx
}
