// PURPOSE: Module declarations and re-exports for plugin-system (orchestrator, aggregate, provider, VOs, errors)
pub mod agent_commands_orchestrator;
pub use agent_commands_orchestrator::PluginCommandsOrchestrator;
pub mod contract_commands_aggregate;
pub use contract_commands_aggregate::PluginCommandsAggregate;
pub mod contract_manager_port;
pub use contract_manager_port::IPluginManagerPort;
pub mod infrastructure_system_provider;
pub use infrastructure_system_provider::PluginSystemProvider;
pub mod taxonomy_group_vo;
pub use taxonomy_group_vo::PluginGroup;
pub mod taxonomy_manager_error;
pub use taxonomy_manager_error::{DiscoveryError, PluginError, RegistrationError};
pub mod root_plugin_container;
