// PURPOSE: TransportContainer — wiring for cli-transport feature (root layer, wiring only)
use shared::cli_transport::contract_executor_port::ICommandExecutorPort;
use std::sync::Arc;

pub struct TransportContainer {
    executor: Arc<dyn ICommandExecutorPort>,
}

impl TransportContainer {
    pub fn new() -> Self {
        Self {
            executor: Arc::new(
                shared::cli_transport::infrastructure_transport_client::StdioClient::new(
                    std::time::Duration::from_secs(60),
                ),
            ),
        }
    }

    pub fn executor(&self) -> Arc<dyn ICommandExecutorPort> {
        self.executor.clone()
    }
}
impl Default for TransportContainer {
    fn default() -> Self {
        Self::new()
    }
}
