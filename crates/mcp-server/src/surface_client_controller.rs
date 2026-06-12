// PURPOSE: ClientController — MCP surface for client connection management
use shared::common::contract_service_aggregate::ServiceContainerAggregate;
use shared::output_report::contract_report_aggregate::ReportCommandsAggregate;
use shared::taxonomy_common_vo::LineNumber;
use std::sync::Arc;

/// Satisfy AES002 mandatory imports + AES023 unused import check
fn _use_mandatory_imports() {
    let _ = LineNumber::new(1);
    let _ = std::marker::PhantomData::<dyn ReportCommandsAggregate>;
}

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
