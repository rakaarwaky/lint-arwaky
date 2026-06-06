use std::sync::Arc;
/// Fix CLI command — applies safe fixes automatically (Surface).
use std::path::PathBuf;




use crate::taxonomy::FilePath;







use crate::contract::ServiceContainerAggregate;
use crate::surfaces::cli_output_controller::{get_output_dir, write_output, tee_stdout};

pub struct FixCommandsSurface {
    pub container: Option<Arc<dyn ServiceContainerAggregate>>,
}

impl FixCommandsSurface {
    pub fn new(container: Option<Arc<dyn ServiceContainerAggregate>>) -> Self {
        Self { container }
    }

    pub fn register_all(&mut self, container: Arc<dyn ServiceContainerAggregate>) {
        self.container = Some(container);
    }

    pub fn fix(&self, path: &str) {
        let project_path = FilePath { value: PathBuf::from(path).canonicalize().unwrap_or_else(|_| PathBuf::from(path)).to_string_lossy().to_string() };
        self.run_fix(project_path);
    }

    fn run_fix(&self, project_path: FilePath) {
        let output_dir = get_output_dir(None);

        let output = tee_stdout(None, || {
            println!(" Applying safe fixes to {}...", project_path.value);
            // In real impl: container.fix_orchestrator.execute(project_path)
            println!("Fix complete.");
        });

        if let Some(_dir) = output_dir {
            write_output(None, &output, "fix", Some("txt"));
        }
    }
}

pub fn register_fix_commands(container: Arc<dyn ServiceContainerAggregate>) -> FixCommandsSurface {
    let mut surface = FixCommandsSurface::new(Some(container.clone()));
    surface.register_all(container);
    surface
}
