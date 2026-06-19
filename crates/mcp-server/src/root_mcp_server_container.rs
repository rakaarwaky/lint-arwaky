// PURPOSE: McpContainer — wiring for mcp-server feature (root layer, wiring only)
// Wiring: IMcpSchemaCheckerProtocol → McpSchemaChecker (capabilities layer)
use crate::capabilities_schema_checker::McpSchemaChecker;
use crate::infrastructure_job_registry::InMemoryJobRegistry;
use shared::mcp_server::contract_registry_port::IJobRegistryPort;
use shared::mcp_server::contract_server_port::IMcpServerPort;
use std::sync::Arc;

pub struct McpContainer {
    server: Arc<dyn IMcpServerPort>,
    job_registry: Arc<dyn IJobRegistryPort>,
    _schema_checker: McpSchemaChecker,
}

impl McpContainer {
    pub fn new() -> Self {
        Self {
            server: Arc::new(crate::infrastructure_server_wrapper::McpServerWrapper::new(
                ".",
                "lint-arwaky",
            )),
            job_registry: Arc::new(InMemoryJobRegistry::new()),
            _schema_checker: McpSchemaChecker::new(),
        }
    }

    pub fn server(&self) -> Arc<dyn IMcpServerPort> {
        self.server.clone()
    }

    pub fn job_registry(&self) -> Arc<dyn IJobRegistryPort> {
        self.job_registry.clone()
    }
}
impl Default for McpContainer {
    fn default() -> Self {
        Self::new()
    }
}
