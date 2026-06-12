// PURPOSE: PluginManagerError — structured error type for plugin management failures
use crate::common::taxonomy_adapter_name_vo::AdapterName;

define_error! {
    pub struct PluginError
    display("Plugin Error")
}

define_wrapper! {
    pub struct DiscoveryError {
        pub base: PluginError,
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, thiserror::Error)]
pub struct RegistrationError {
    #[serde(flatten)]
    pub base: PluginError,
    pub adapter_name: AdapterName,
}

impl RegistrationError {
    pub fn new(message: crate::common::taxonomy_common_error::ErrorMessage) -> Self {
        Self {
            base: PluginError::new(message),
            adapter_name: AdapterName::default(),
        }
    }
}

impl std::fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name_str = self.adapter_name.to_string();
        let target = if name_str.is_empty() {
            String::new()
        } else {
            format!(" for '{}'", name_str)
        };
        write!(f, "Registration Error{}: {}", target, self.base.message)
    }
}
