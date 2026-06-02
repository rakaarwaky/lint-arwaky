use crate::surfaces::mcp_server_handler::McpServerHandlerSurface;
use crate::agent::dependency_injection_container::DependencyInjectionContainer;
use crate::taxonomy::file_path_vo::DirectoryPath;
use std::sync::Arc;

pub async fn run_server() {
    let container = Arc::new(DependencyInjectionContainer::new(DirectoryPath::new(".")));
    let surface = McpServerHandlerSurface::new();
    surface.run_server(container).await;
}
