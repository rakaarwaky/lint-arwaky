// PURPOSE: PluginContainer — wiring for plugin-system feature (root layer, wiring only)
use shared::plugin_system::contract_plugin_manager_port::IPluginManagerPort;
use std::sync::Arc;

pub struct PluginContainer {
    manager: Arc<dyn IPluginManagerPort>,
}

impl PluginContainer {
    pub fn new() -> Self {
        Self {
            manager: Arc::new(crate::infrastructure_system_provider::PluginSystemProvider::new()),
        }
    }

    pub fn manager(&self) -> Arc<dyn IPluginManagerPort> {
        self.manager.clone()
    }
}
impl Default for PluginContainer {
    fn default() -> Self {
        Self::new()
    }
}
