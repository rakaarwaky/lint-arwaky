/// plugin_system_provider — Entry point discovery and loading for custom adapters.
use crate::contract::IPluginManagerPort;
use crate::taxonomy::{AdapterMetadata, AdapterMetadataList, AdapterName, PluginGroup, AdapterClassMap, PluginError};
use std::collections::HashMap;

pub struct PluginSystemProvider {
    custom_adapters: HashMap<String, String>,
}

impl PluginSystemProvider {
    pub fn new() -> Self {
        Self { custom_adapters: HashMap::new() }
    }

    pub fn register_custom_adapter(&mut self, name: AdapterName, class_path: &str) {
        self.custom_adapters.insert(name.value.clone(), class_path.to_string());
    }

    pub fn unregister_custom_adapter(&mut self, name: &AdapterName) -> Option<String> {
        self.custom_adapters.remove(&name.value)
    }

    pub fn get_custom_adapter(&self, name: &AdapterName) -> Option<&String> {
        self.custom_adapters.get(&name.value)
    }

    pub fn list_custom_adapters(&self) -> AdapterMetadataList {
        let items: Vec<AdapterMetadata> = self.custom_adapters.iter().map(|(name, path)| {
            AdapterMetadata::new(AdapterName::new(name.clone()).unwrap(), path.clone(), String::new())
        }).collect();
        AdapterMetadataList::new(items)
    }
}

#[async_trait::async_trait]
impl IPluginManagerPort for PluginSystemProvider {
    async fn discover_plugins(&self, group: PluginGroup) -> Result<AdapterClassMap, PluginError> {
        unimplemented!()
    }
    async fn list_custom_adapters(&self) -> AdapterMetadataList {
        unimplemented!()
    }
    async fn register_custom_adapter(&self, name: AdapterName, adapter_class: String) -> Option<PluginError> {
        unimplemented!()
    }
}

