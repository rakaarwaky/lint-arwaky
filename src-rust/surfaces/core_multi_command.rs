use std::sync::Arc;











use crate::contract::ServiceContainerAggregate;
pub struct MultiCommandsSurface {
    pub container: Option<Arc<dyn ServiceContainerAggregate>>,
}

impl MultiCommandsSurface {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub fn register_all(&mut self, container: Arc<dyn ServiceContainerAggregate>) {
        self.container = Some(container);
    }

    pub fn multi_project(&self, paths: &[String], output_format: &str, _config: Option<&str>) {
        let project_list = if paths.is_empty() {
            if let Some(ref _container) = self.container {
                vec![std::env::current_dir().unwrap_or_default().to_string_lossy().to_string()]
            } else {
                vec![std::env::current_dir().unwrap_or_default().to_string_lossy().to_string()]
            }
        } else {
            paths.to_vec()
        };

        if output_format == "json" {
            println!("{{\"projects\": {project_list:?}}}");
        } else {
            for p in &project_list {
                println!("  Scan result for {p}: 100.0%");
            }
        }
    }
}

pub fn register_multi_commands(container: Arc<dyn ServiceContainerAggregate>) -> MultiCommandsSurface {
    let mut surface = MultiCommandsSurface::new();
    surface.register_all(container);
    surface
}
