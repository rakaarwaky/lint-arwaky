// AES506: Surface Orphan violation - this smart surface is not imported by entry/router
pub struct OrphanSurfaceComponent;

impl OrphanSurfaceComponent {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self) -> String {
        "rendered".to_string()
    }
}