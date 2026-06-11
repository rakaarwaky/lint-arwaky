use crate::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use crate::config_system::taxonomy_setting_vo::AdapterStatus;
use crate::config_system::taxonomy_setting_vo::ProjectConfig;
use crate::config_system::taxonomy_validation_vo::ValidationResult;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;

/// Business logic for interpreting and validating project configuration.
pub struct ConfigRulesValidator {
    config: ProjectConfig,
}

impl ConfigRulesValidator {
    pub fn new(config: ProjectConfig) -> Self {
        Self { config }
    }
}

impl IConfigValidatorProtocol for ConfigRulesValidator {
    /// Determines if a specific adapter should run based on configuration rules.
    fn is_adapter_enabled(&self, adapter_name: &AdapterName) -> bool {
        for adapter in &self.config.adapters {
            if adapter.name == *adapter_name {
                return adapter.status == AdapterStatus::Enabled;
            }
        }
        // Default policy: enabled if not explicitly mentioned
        true
    }

    /// Validates that scoring thresholds are sane.
    fn validate_thresholds(&self) -> ValidationResult {
        let t = &self.config.thresholds;

        // Score must be 0-100
        if !(0.0..=100.0).contains(&t.score.value) {
            return ValidationResult::fail("Score threshold must be between 0 and 100.");
        }

        // Complexity and line limits must be positive
        if t.complexity.value <= 0 {
            return ValidationResult::fail("Complexity threshold must be positive.");
        }

        if t.max_file_lines.value <= 0 {
            return ValidationResult::fail("max_file_lines threshold must be positive.");
        }

        ValidationResult::ok()
    }
}
