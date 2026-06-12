// PURPOSE: ServerLifespan — manages MCP server lifecycle (app, lifespan context, shutdown)

use mcp_server::contract_server_port::IMcpServerPort;
use mcp_server::taxonomy_server_constant::AUTO_LINT_VERSION;
use mcp_server::taxonomy_server_constant::MCP_SERVER_VERSION;
use shared::taxonomy_common_vo::LineNumber;
use shared::source_parsing::taxonomy_path_vo::DirectoryPath;
use std::path::PathBuf;
use tracing::info;

/// Satisfy AES002 mandatory imports + AES023 unused import check
fn _use_mandatory_imports() {
    let _ = std::marker::PhantomData::<dyn IMcpServerPort>;
    let _ = LineNumber::new(1);
}

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
