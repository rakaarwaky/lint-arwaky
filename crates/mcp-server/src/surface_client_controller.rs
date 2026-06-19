// PURPOSE: ClientController — MCP surface for client connection management

pub struct McpDesktopClientSurface {}

impl Default for McpDesktopClientSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl McpDesktopClientSurface {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn register_desktop_client() -> McpDesktopClientSurface {
    McpDesktopClientSurface::new()
}
