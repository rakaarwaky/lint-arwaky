use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;
use std::sync::Arc;

pub struct McpDesktopClientSurface {
    pub container: Option<Arc<dyn ServiceContainerAggregate>>,
}

impl Default for McpDesktopClientSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl McpDesktopClientSurface {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub fn register_all(&mut self, container: Arc<dyn ServiceContainerAggregate>) {
        self.container = Some(container);
    }
}

pub fn register_desktop_client(
    container: Arc<dyn ServiceContainerAggregate>,
) -> McpDesktopClientSurface {
    let mut surface = McpDesktopClientSurface::new();
    surface.register_all(container);
    surface
}
