use crate::utility_config_parser::parse_config_yaml;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use std::sync::OnceLock;

static DEFAULT_RUST_CONFIG: OnceLock<ArchitectureConfig> = OnceLock::new();
static DEFAULT_PYTHON_CONFIG: OnceLock<ArchitectureConfig> = OnceLock::new();
static DEFAULT_TS_CONFIG: OnceLock<ArchitectureConfig> = OnceLock::new();

pub fn default_aes_config() -> ArchitectureConfig {
    DEFAULT_RUST_CONFIG
        .get_or_init(|| {
            parse_config_yaml(include_str!(
                "../../shared/config/lint_arwaky.config.rust.yaml"
            ))
        })
        .clone()
}

pub fn default_config_for_language(language: &str) -> ArchitectureConfig {
    match language {
        "rust" => default_aes_config(),
        "python" => DEFAULT_PYTHON_CONFIG
            .get_or_init(|| {
                parse_config_yaml(include_str!(
                    "../../shared/config/lint_arwaky.config.python.yaml"
                ))
            })
            .clone(),
        "javascript" | "typescript" => DEFAULT_TS_CONFIG
            .get_or_init(|| {
                parse_config_yaml(include_str!(
                    "../../shared/config/lint_arwaky.config.javascript.yaml"
                ))
            })
            .clone(),
        _ => {
            tracing::warn!(language = language, "unknown language, using empty default config");
            ArchitectureConfig::default()
        }
    }
}
