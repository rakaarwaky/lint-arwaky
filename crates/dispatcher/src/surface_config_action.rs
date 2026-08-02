// PURPOSE: ConfigCommandsSurface — CLI surface for config show
// Adapted: sync — iterates known languages using read_config (sync) instead of
// list_config_files (async). No tokio runtime needed.
use shared::common::{ExitCode, FilePath};
use shared::config_system::{
    taxonomy_config_vo::ArchitectureConfig, ConfigLanguage, IConfigOrchestratorAggregate,
};
use std::sync::Arc;

/// Redact sensitive values from config content.
fn redact_secrets(content: &str) -> String {
    let mut result = content.to_string();

    if result.contains("AKIA") {
        let re = regex::Regex::new(r"AKIA[0-9A-Z]{16}").ok();
        if let Some(re) = re {
            result = re.replace_all(&result, "[REDACTED-AWS-KEY]").to_string();
        }
    }

    if result.len() > 100 {
        let words: Vec<String> = result.split_whitespace().map(|s| s.to_string()).collect();
        for word in &words {
            if word.len() >= 40
                && word
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '/' | '+' | '='))
            {
                result = result.replacen(word, "[REDACTED]", 1);
            }
        }
    }

    result
}

pub fn handle_config_show(orchestrator: Arc<dyn IConfigOrchestratorAggregate>) -> ExitCode {
    let project_root = FilePath::new(".".to_string()).unwrap_or_default();

    // Iterate known languages using sync read_config instead of async list_config_files
    let languages = [
        ConfigLanguage::Rust,
        ConfigLanguage::Python,
        ConfigLanguage::TypeScript,
    ];

    let mut found_any = false;
    for lang in &languages {
        match orchestrator.read_config(&project_root, *lang) {
            Ok(Some(source)) => {
                found_any = true;
                let path_str = source.path.value.as_str();
                println!("── [{}] {} ──", lang.as_str(), path_str);
                let safe_content = redact_secrets(&source.raw_content);
                println!("{safe_content}");
            }
            Ok(None) => {}
            Err(e) => {
                eprintln!(
                    "Warning: Failed to read config for {}: {}",
                    lang.as_str(),
                    e
                );
            }
        }
    }

    if !found_any {
        println!("No config file found. Run `lint-arwaky init` to create one.");
    }
    ExitCode::OK
}

// Config parsing wrappers — used by MCP agent instead of importing config_system directly.
pub fn parse_config_yaml(yaml_str: &str) -> ArchitectureConfig {
    config_system::utility_config_parser::parse_config_yaml(yaml_str)
}

pub fn parse_adapter_names_from_yaml(yaml_str: &str) -> Vec<String> {
    config_system::utility_config_parser::parse_adapter_names_from_yaml(yaml_str)
}

pub fn parse_score_threshold(yaml_str: &str) -> Option<f64> {
    config_system::utility_config_parser::parse_score_threshold(yaml_str)
}
