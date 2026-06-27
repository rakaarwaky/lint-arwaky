use shared::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use shared::config_system::taxonomy_setting_vo::AdapterStatus;
use shared::config_system::taxonomy_setting_vo::ProjectConfig;
use shared::config_system::taxonomy_validation_vo::ValidationResult;
use shared::taxonomy_adapter_name_vo::AdapterName;

pub struct ConfigRulesValidator;

impl Default for ConfigRulesValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigRulesValidator {
    pub fn new() -> Self {
        Self
    }
}

impl IConfigValidatorProtocol for ConfigRulesValidator {
    fn is_adapter_enabled(&self, config: &ProjectConfig, adapter_name: &AdapterName) -> bool {
        for adapter in &config.adapters {
            if adapter.name == *adapter_name {
                return adapter.status == AdapterStatus::Enabled;
            }
        }
        true
    }

    fn validate_thresholds(&self, config: &ProjectConfig) -> ValidationResult {
        let t = &config.thresholds;
        let mut errors = Vec::new();

        if !(0.0..=100.0).contains(&t.score.value) {
            errors.push("Score threshold must be between 0 and 100.");
        }
        if t.complexity.value <= 0 {
            errors.push("Complexity threshold must be positive.");
        }
        if t.max_file_lines.value <= 0 {
            errors.push("max_file_lines threshold must be positive.");
        }

        if errors.is_empty() {
            ValidationResult::ok()
        } else {
            ValidationResult::fail(&errors.join(" | "))
        }
    }
}
