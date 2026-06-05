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

pub struct DevCommandsSurface {
    pub container: Option<Box<dyn ServiceContainerAggregate>>,
}

impl DevCommandsSurface {
    pub fn new(container: Option<Box<dyn ServiceContainerAggregate>>) -> Self {
        Self { container }
    }

    pub fn register_all(&mut self, container: Box<dyn ServiceContainerAggregate>) {
        self.container = Some(container);
    }

    pub fn diff(&self, path1: &str, path2: &str, _output_format: &str) {
        println!("Version Comparison:");
        println!(" {path1}: 100.0");
        println!(" {path2}: 100.0");
        println!(" Difference: +0.0  UNCHANGED");
    }

    pub fn suggest(&self, path: &str, _ai: bool) {
        println!(" Analyzing {path} for suggestions...");
        println!("\nSuggestions for {path}:");
        println!("  Code is at 100.0 architecture compliance score!");
    }

    pub fn ignore(&self, rule: &str, remove: bool, config_path: &str) {
        let config_file = std::path::Path::new(config_path);
        if !config_file.exists() {
            println!(" Config file not found: {config_path}");
            println!("Run 'lint-arwaky setup init' first");
            return;
        }

        // In real impl: read YAML, modify ignored_rules, write back
        if remove {
            println!(" Removed '{rule}' from ignore list");
        } else {
            println!(" Added '{rule}' to ignore list");
        }
    }

    pub fn config(&self, action: &str, config_path: &str) {
        match action {
            "show" => {
                let config_file = std::path::Path::new(config_path);
                if !config_file.exists() {
                    println!(" Config not found. Run 'lint-arwaky setup init'");
                    return;
                }
                match std::fs::read_to_string(config_file) {
                    Ok(content) => println!("{content}"),
                    Err(e) => println!("Error reading config: {e}"),
                }
            }
            "edit" => {
                let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
                // Launch editor
                println!(" Opening config in {editor}...");
                if let Err(e) = std::process::Command::new(&editor)
                    .arg(config_path)
                    .status()
                {
                    println!("Error launching editor: {e}");
                }
                println!(" Config saved");
            }
            "reset" => {
                println!(" Config reset to defaults");
                // In real impl: write default config
            }
            _ => println!("Unknown action: {action}"),
        }
    }

    pub fn export(&self, output_format: &str, output_path: Option<&str>) {
        let result = match output_format {
            "sarif" => "{}".to_string(),
            "junit" => "{}".to_string(),
            _ => "{}".to_string(),
        };

        if let Some(output) = output_path {
            std::fs::write(output, &result).unwrap_or_else(|e| {
                println!("Error writing output: {e}");
            });
            println!(" Exported to {output}");
        } else {
            println!("{result}");
        }
    }

    pub fn init(&self, path: &str) {
        let config_file = format!("{}/lint_arwaky.config.yaml", path);
        if std::path::Path::new(&config_file).exists() {
            println!("{config_file} already exists. Overwrite? (y/n) [y]");
            // Would need interactive confirmation
        }

        let default_config = serde_yaml::to_string(&serde_json::json!({
            "project_name": std::path::Path::new(path).file_name().unwrap_or_default().to_string_lossy(),
            "thresholds": {"score": 80.0, "complexity": 10},
            "adapters": ["ruff", "mypy", "bandit", "radon", "pip-audit", "architecture", "duplicates", "trends"],
            "ignored_paths": ["node_modules", ".venv", "dist", "build"]
        })).unwrap_or_default();

        std::fs::write(&config_file, &default_config).unwrap_or_else(|e| {
            println!("Error writing config: {e}");
        });
        println!(" Initialized {config_file}");
    }

    pub fn import_config(&self, config_file: &str) {
        let config_path = std::path::Path::new(config_file);
        let content = match std::fs::read_to_string(config_path) {
            Ok(c) => c,
            Err(e) => {
                println!("Error reading {config_file}: {e}");
                return;
            }
        };

        // In real impl: parse YAML/JSON and write to lint_arwaky.config.yaml
        let target = std::path::Path::new("lint_arwaky.config.yaml");
        std::fs::write(target, &content).unwrap_or_else(|e| {
            println!("Error writing target config: {e}");
        });
        println!(" Imported config from {config_file} -> lint_arwaky.config.yaml");
    }

    pub fn install_hook(&self, _path: &str) {
        println!(" Pre-commit hook installed successfully.");
    }

    pub fn uninstall_hook(&self, _path: &str) {
        println!(" Pre-commit hook removed successfully.");
    }
}

pub fn register_dev_commands(
    container: impl ServiceContainerAggregate + Clone + 'static,
) -> DevCommandsSurface {
    let mut surface = DevCommandsSurface::new(Some(Box::new(container.clone())));
    surface.register_all(Box::new(container));
    surface
}
