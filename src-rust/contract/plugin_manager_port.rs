use crate::taxonomy::AdapterClassMap;
use crate::taxonomy::AdapterMetadataList;
use crate::taxonomy::AdapterName;
use crate::taxonomy::ContentString;
use crate::taxonomy::PluginError;
use crate::taxonomy::PluginGroup;

pub trait IPluginManagerPort: Send + Sync {
    fn discover_plugins(&self, group: &PluginGroup) -> Result<AdapterClassMap, PluginError>;
    fn list_custom_adapters(&self) -> AdapterMetadataList;
    fn register_custom_adapter(
        &self,
        name: &AdapterName,
        class_path: &ContentString,
    ) -> Result<(), PluginError>;
}
