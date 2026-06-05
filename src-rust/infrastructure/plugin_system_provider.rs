/// plugin_system_provider — Entry point discovery and loading for custom adapters.
use crate::contract::plugin_manager_port::IPluginManagerPort;
use crate::taxonomy::{AdapterMetadata, AdapterName, PluginError, ErrorMessage};
use std::collections::HashMap;
use std::sync::Mutex;

pub struct PluginSystemProvider {
    custom_adapters: Mutex<HashMap<String, String>>,
}

impl Default for PluginSystemProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginSystemProvider {
    pub fn new() -> Self {
        Self { custom_adapters: Mutex::new(HashMap::new()) }
    }

    pub fn register_custom_adapter_old(&self, name: AdapterName, class_path: &str) {
        if let Ok(mut map) = self.custom_adapters.lock() {
            map.insert(name.value.clone(), class_path.to_string());
        }
    }

    pub fn unregister_custom_adapter(&self, name: &AdapterName) -> Option<String> {
        if let Ok(mut map) = self.custom_adapters.lock() {
            map.remove(&name.value)
        } else {
            None
        }
    }

    pub fn get_custom_adapter(&self, name: &AdapterName) -> Option<String> {
        if let Ok(map) = self.custom_adapters.lock() {
            map.get(&name.value).cloned()
        } else {
            None
        }
    }
}

impl IPluginManagerPort for PluginSystemProvider {
    fn discover_plugins(&self, _group: &str) -> Result<Vec<(String, String)>, PluginError> {
        Ok(Vec::new())
    }

    fn list_custom_adapters(&self) -> Vec<AdapterMetadata> {
        if let Ok(map) = self.custom_adapters.lock() {
            map.iter().map(|(name, path)| {
                AdapterMetadata {
                    name: AdapterName::raw(name.clone()),
                    class_path: path.clone(),
                    description: String::new(),
                }
            }).collect()
        } else {
            Vec::new()
        }
    }

    fn register_custom_adapter(&self, name: &AdapterName, class_path: &str) -> Result<(), PluginError> {
        if let Ok(mut map) = self.custom_adapters.lock() {
            map.insert(name.value.clone(), class_path.to_string());
            Ok(())
        } else {
            Err(PluginError::new(ErrorMessage::new("Lock poisoned")))
        }
    }
}
