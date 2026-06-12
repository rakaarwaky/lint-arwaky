// PURPOSE: CommandsOrchestrator — orchestrates plugin management commands (list, enable, disable, install)

use shared::plugin_system::contract_plugin_commands_aggregate::PluginCommandsAggregate;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use std::collections::HashMap;

use async_trait::async_trait;

pub struct PluginCommandsOrchestrator {
    root_path: Option<FilePath>,
}

#[async_trait]
impl PluginCommandsAggregate for PluginCommandsOrchestrator {
    fn root_path(&self) -> Option<&FilePath> {
        self.root_path.as_ref()
    }
    async fn adapters(&self) {
        let names = self.get_adapter_names();
        println!("Adapters: {:?}", names);
    }
    async fn plugins(&self) {
        let info = self.get_discovered_plugins_info();
        println!("Plugins: {:?}", info);
    }
}

impl Default for PluginCommandsOrchestrator {
    fn default() -> Self {
        Self::new(None)
    }
}

impl PluginCommandsOrchestrator {
    pub fn new(root_path: Option<FilePath>) -> Self {
        Self { root_path }
    }

    pub fn get_adapter_names(&self) -> Vec<String> {
        let mut adapters = Vec::new();
        if let Some(root) = &self.root_path {
            let adapters_dir =
                std::path::Path::new(&root.value).join("crates").join("language-adapters").join("src");
            if let Ok(entries) = std::fs::read_dir(&adapters_dir) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("infrastructure_") && name.ends_with("_adapter.rs") {
                        let adapter_name = name
                            .strip_prefix("infrastructure_").unwrap_or(&name)
                            .strip_suffix("_adapter.rs").unwrap_or(&name)
                            .to_string();
                        adapters.push(adapter_name);
                    }
                }
            }
        }
        adapters
    }

    pub fn get_discovered_plugins_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();
        if let Some(root) = &self.root_path {
            let crates_dir = std::path::Path::new(&root.value).join("crates");
            if let Ok(entries) = std::fs::read_dir(&crates_dir) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let toml_path = entry.path().join("Cargo.toml");
                        if toml_path.exists() {
                            info.insert(name, "active".to_string());
                        }
                    }
                }
            }
        }
        info
    }
}
