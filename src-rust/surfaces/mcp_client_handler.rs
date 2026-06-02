/// MCP Desktop Client Surface — Internal helper for DesktopCommander integration.
use crate::taxonomy::*;
use crate::contract::*;
use crate::surfaces::mcp_execute_command::_running_jobs;

pub struct McpDesktopClientSurface {
    pub container: Option<ServiceContainerAggregate>,
}

impl McpDesktopClientSurface {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub fn register_all(&mut self, container: ServiceContainerAggregate) {
        self.container = Some(container);
    }
}

pub fn register_desktop_client(container: ServiceContainerAggregate) -> McpDesktopClientSurface {
    let mut surface = McpDesktopClientSurface::new();
    surface.register_all(container);
    surface
}
