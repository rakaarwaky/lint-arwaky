// PURPOSE: IConfigValidatorProtocol — protocol for project config and scoring threshold validation

use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::config_system::taxonomy_setting_vo::ProjectConfig;
use crate::config_system::taxonomy_validation_vo::ValidationResult;

pub trait IConfigValidatorProtocol: Send + Sync {
    /// Determines if a specific adapter should run based on configuration rules.
    fn is_adapter_enabled(&self, config: &ProjectConfig, adapter_name: &AdapterName) -> bool;

    /// Validates that scoring thresholds are sane.
    fn validate_thresholds(&self, config: &ProjectConfig) -> ValidationResult;
}
