// PURPOSE: Module declarations for plugin-system (orchestrator, provider)
pub use shared::plugin_system::contract_plugin_commands_aggregate::PluginCommandsAggregate;
pub use shared::plugin_system::contract_plugin_manager_port::IPluginManagerPort;
pub use shared::plugin_system::taxonomy_group_vo::PluginGroup;
pub use shared::plugin_system::taxonomy_manager_error::{
    DiscoveryError, PluginError, RegistrationError,
};
pub mod agent_commands_orchestrator;
pub use agent_commands_orchestrator::PluginCommandsOrchestrator;
pub mod infrastructure_system_provider;
pub use infrastructure_system_provider::PluginSystemProvider;
pub mod root_plugin_container;
