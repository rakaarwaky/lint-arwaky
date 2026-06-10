// PURPOSE: PluginCommandsAggregate — aggregate trait for plugin command execution
pub use crate::plugin_system::contract_manager_port::IPluginManagerPort;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait PluginCommandsAggregate: Send + Sync {
    fn root_path(&self) -> Option<&FilePath>;
    async fn adapters(&self);
    async fn plugins(&self);
}
