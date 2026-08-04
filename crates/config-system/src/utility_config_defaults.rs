use crate::utility_config_parser::parse_config_yaml;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use std::sync::OnceLock;

static DEFAULT_CONFIG: OnceLock<ArchitectureConfig> = OnceLock::new();

pub fn default_aes_config() -> ArchitectureConfig {
    DEFAULT_CONFIG
        .get_or_init(|| {
            parse_config_yaml(include_str!(
                "../../shared/config/lint_arwaky.config.yaml"
            ))
        })
        .clone()
}

pub fn default_config_for_language(language: &str) -> ArchitectureConfig {
    match language {
        "rust" | "python" | "javascript" | "typescript" => default_aes_config(),
        _ => {
            tracing::warn!(
                language = language,
                "unknown language, using empty default config"
            );
            ArchitectureConfig::default()
        }
    }
}
