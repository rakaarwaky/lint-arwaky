/// plugin_system_provider — Entry point discovery and loading for custom adapters.
use crate::contract::plugin_manager_port::IPluginManagerPort;
use crate::taxonomy::{
    AdapterClassMap, AdapterMetadata, AdapterMetadataList, AdapterName, ContentString,
    ErrorMessage, PluginError, PluginGroup,
};
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
        Self {
            custom_adapters: Mutex::new(HashMap::new()),
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
    fn discover_plugins(&self, _group: &PluginGroup) -> Result<AdapterClassMap, PluginError> {
        Ok(AdapterClassMap {
            values: std::collections::HashMap::new(),
        })
    }

    fn list_custom_adapters(&self) -> AdapterMetadataList {
        if let Ok(map) = self.custom_adapters.lock() {
            AdapterMetadataList {
                values: map
                    .iter()
                    .map(|(name, path)| AdapterMetadata {
                        name: AdapterName::raw(name.clone()),
                        class_path: path.clone(),
                        description: String::new(),
                    })
                    .collect(),
            }
        } else {
            AdapterMetadataList { values: vec![] }
        }
    }

    fn register_custom_adapter(
        &self,
        name: &AdapterName,
        class_path: &ContentString,
    ) -> Result<(), PluginError> {
        if let Ok(mut map) = self.custom_adapters.lock() {
            map.insert(name.value.clone(), class_path.value.clone());
            Ok(())
        } else {
            Err(PluginError::new(ErrorMessage::new("Lock poisoned")))
        }
    }
}
