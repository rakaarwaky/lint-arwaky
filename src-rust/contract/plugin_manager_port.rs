use crate::taxonomy::AdapterMetadata;
use crate::taxonomy::AdapterName;
use crate::taxonomy::PluginError;
use super::*;

pub trait IPluginManagerPort: Send + Sync {
    fn discover_plugins(&self, group: &str) -> Result<Vec<(String, String)>, PluginError>;
    fn list_custom_adapters(&self) -> Vec<AdapterMetadata>;
    fn register_custom_adapter(&self, name: &AdapterName, class_path: &str) -> Result<(), PluginError>;
}
