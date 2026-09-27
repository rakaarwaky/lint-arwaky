// PURPOSE: IConfigOrchestratorAggregate — single entry point over the config domain
use crate::config_system::taxonomy_config_request::ConfigRequest;
use crate::config_system::taxonomy_config_response::ConfigResponse;

/// Single entry point over the config feature; the agent dispatches internally.
pub trait IConfigOrchestratorAggregate: Send + Sync {
    fn execute(&self, request: ConfigRequest) -> ConfigResponse;
}
