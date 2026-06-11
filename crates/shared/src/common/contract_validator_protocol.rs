// PURPOSE: IConfigValidatorProtocol — protocol for project config and scoring threshold validation

use crate::config_system::taxonomy_validation_vo::ValidationResult;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;

pub trait IConfigValidatorProtocol: Send + Sync {
    /// Determines if a specific adapter should run based on configuration rules.
    fn is_adapter_enabled(&self, adapter_name: &AdapterName) -> bool;

    /// Validates that scoring thresholds are sane.
    fn validate_thresholds(&self) -> ValidationResult;
}
