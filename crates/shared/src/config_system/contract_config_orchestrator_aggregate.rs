// PURPOSE: IConfigOrchestratorAggregate — single entry point over the config domain
use crate::taxonomy_config_system_request::ConfigRequest;
use crate::taxonomy_config_system_response::ConfigResponse;

/// Single entry point over the config feature; the agent dispatches internally.
pub trait IConfigOrchestratorAggregate: Send + Sync {
    fn execute(&self, request: ConfigRequest) -> ConfigResponse;
}
