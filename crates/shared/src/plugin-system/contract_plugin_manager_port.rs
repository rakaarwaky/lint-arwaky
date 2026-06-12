// PURPOSE: IPluginManagerPort — port trait for plugin lifecycle management
/* UNKNOWN: AdapterClassMap */
use crate::config_system::taxonomy_adapter_vo::AdapterClassMap;
use crate::plugin_system::taxonomy_group_vo::PluginGroup;
/* UNKNOWN: PluginError */ use crate::plugin_system::taxonomy_manager_error::PluginError;
/* UNKNOWN: AdapterMetadataList */
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_source_vo::ContentString;
use crate::config_system::taxonomy_adapter_vo::AdapterMetadataList;

pub trait IPluginManagerPort: Send + Sync {
    fn discover_plugins(&self, group: &PluginGroup) -> Result<AdapterClassMap, PluginError>;
    fn list_custom_adapters(&self) -> AdapterMetadataList;
    fn register_custom_adapter(
        &self,
        name: &AdapterName,
        class_path: &ContentString,
    ) -> Result<(), PluginError>;
}
