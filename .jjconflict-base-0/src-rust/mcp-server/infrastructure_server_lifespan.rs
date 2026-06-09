//! mcp_server_lifespan — MCP server startup/shutdown lifecycle management.

use std::path::PathBuf;

use crate::mcp_server::taxonomy_server_constant::AUTO_LINT_VERSION;
use crate::mcp_server::taxonomy_server_constant::MCP_SERVER_VERSION;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use tracing::info;

/// Context object yielded by the lifespan manager.
#[derive(Debug, Clone)]
pub struct WrapperContext {
    /// Project root path.
    pub project_root: PathBuf,
    /// Server version.
    pub server_version: String,
    /// Auto-lint version.
    pub auto_lint_version: String,
}

/// MCP server lifespan: initialize context, log startup, prepare for shutdown.
pub async fn mcp_server_lifespan(project_root: DirectoryPath) -> WrapperContext {
    let root = PathBuf::from(&project_root.value);
    info!("MCP server starting up — project_root={}", root.display());

    let ctx = WrapperContext {
        project_root: root,
        server_version: MCP_SERVER_VERSION.to_string(),
        auto_lint_version: AUTO_LINT_VERSION.to_string(),
    };
    info!(
        "MCP server lifespan: initialized, version={}",
        MCP_SERVER_VERSION
    );

    ctx
}
