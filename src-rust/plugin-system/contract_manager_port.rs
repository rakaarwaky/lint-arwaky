/* UNKNOWN: AdapterClassMap */
use crate::plugin_system::taxonomy_group_vo::PluginGroup;
/* UNKNOWN: PluginError */ use crate::plugin_system::taxonomy_manager_error::PluginError;
use crate::shared_common::taxonomy_adapter_vo::AdapterClassMap;
/* UNKNOWN: AdapterMetadataList */
use crate::shared_common::taxonomy_adapter_vo::AdapterMetadataList;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_source_vo::ContentString;

pub trait IPluginManagerPort: Send + Sync {
    fn discover_plugins(&self, group: &PluginGroup) -> Result<AdapterClassMap, PluginError>;
    fn list_custom_adapters(&self) -> AdapterMetadataList;
    fn register_custom_adapter(
        &self,
        name: &AdapterName,
        class_path: &ContentString,
    ) -> Result<(), PluginError>;
}
