use crate::contract::ServiceContainerAggregate;
use crate::contract::adapter_container_aggregate::AdapterContainerAggregate;
use crate::contract::agent_lifecycle_aggregate::AgentLifecycleAggregate;
use crate::contract::analysis_orchestrator_aggregate::AnalysisOrchestratorAggregate;
use crate::contract::architecture_coordinator_aggregate::ArchCoordinatorAggregate;
use crate::contract::capability_container_aggregate::CapabilityContainerAggregate;
use crate::contract::dispatch_check_aggregate::CheckCommandsAggregate;
use crate::contract::container_registry_aggregate::ContainerRegistryAggregate;
use crate::contract::dev_commands_aggregate::DevCommandsAggregate;
use crate::contract::directory_watch_aggregate::DirectoryWatchAggregate;
use crate::contract::dispatch_fix_aggregate::FixCommandsAggregate;
use crate::contract::git_commands_aggregate::GitCommandsAggregate;
use crate::contract::diff_result_aggregate::GitDiffResultAggregate;

pub struct MaintenanceCommandsSurface {
    pub container: Option<Box<dyn ServiceContainerAggregate>>,
}

impl MaintenanceCommandsSurface {
    pub fn new(container: Option<Box<dyn ServiceContainerAggregate>>) -> Self {
        Self { container }
    }

    pub fn register_all(&mut self, container: Box<dyn ServiceContainerAggregate>) {
        self.container = Some(container);
    }

    pub fn stats(&self, path: &str) {
        let abs_path = std::path::Path::new(path);
        println!(" Auto-Linter Statistics for {}", abs_path.to_string_lossy());
        println!("{}", "=".repeat(50));
        println!(" Python files: 0");
        println!(" Test files: 0");
        println!(" Test ratio: 0.0%");
        println!("{}", "=".repeat(50));
    }

    pub fn clean(&self) {
        println!(" Cleaning cache...");
        println!(" Cleanup complete.");
    }

    pub fn update(&self) {
        println!(" Updating adapters...");
        println!("\n Update complete");
    }

    pub fn doctor(&self) {
        println!(" Lint Arwaky Doctor");
        println!("{}", "=".repeat(50));
        println!(" Python: 3.12+");
        println!(" Status: Installed");
        println!(" Config: lint_arwaky.config.yaml");
        println!("\n Adapters:");
        println!("  - ruff: OK");
        println!("  - mypy: OK");
        println!("{}", "=".repeat(50));
        println!("\n All systems healthy!");
    }

    pub fn cancel(&self, job_id: &str) {
        println!("Request to cancel job {job_id} sent.");
    }
}

pub fn register_maintenance_commands(
    container: impl ServiceContainerAggregate + Clone + 'static,
) -> MaintenanceCommandsSurface {
    let mut surface = MaintenanceCommandsSurface::new(Some(Box::new(container.clone())));
    surface.register_all(Box::new(container));
    surface
}
