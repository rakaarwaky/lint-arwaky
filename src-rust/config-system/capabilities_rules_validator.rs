// PURPOSE: Validator: Rules validation

use crate::config_system::taxonomy_setting_vo::AdapterStatus;
use crate::config_system::taxonomy_setting_vo::ProjectConfig;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;

/// Result of a validation operation.
pub struct ValidationResult {
    pub is_valid: bool,
    pub reason: Option<String>,
}

impl ValidationResult {
    pub fn ok() -> Self {
        Self {
            is_valid: true,
            reason: None,
        }
    }
    pub fn fail(reason: &str) -> Self {
        Self {
            is_valid: false,
            reason: Some(reason.to_string()),
        }
    }
}

/// Business logic for interpreting and validating project configuration.
pub struct ConfigRulesValidator {
    config: ProjectConfig,
}

impl ConfigRulesValidator {
    pub fn new(config: ProjectConfig) -> Self {
        Self { config }
    }

    /// Determines if a specific adapter should run based on configuration rules.
    pub fn is_adapter_enabled(&self, adapter_name: &AdapterName) -> bool {
        for adapter in &self.config.adapters {
            if adapter.name == *adapter_name {
                return adapter.status == AdapterStatus::Enabled;
            }
        }
        // Default policy: enabled if not explicitly mentioned
        true
    }

    /// Validates that scoring thresholds are sane.
    pub fn validate_thresholds(&self) -> ValidationResult {
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
