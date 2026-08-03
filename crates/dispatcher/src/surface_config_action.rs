// PURPOSE: ConfigCommandsSurface — config show business logic, no formatting.
// Adapted: sync — iterates known languages using read_config (sync) instead of
// list_config_files (async). No tokio runtime needed.
use shared::common::FilePath;
use shared::config_system::{ConfigLanguage, IConfigOrchestratorAggregate};
use std::sync::Arc;

/// One discovered config file (content already redacted).
#[derive(Debug, Clone)]
pub struct ConfigShowEntry {
    pub language: String,
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, Default)]
pub struct ConfigShowReport {
    pub entries: Vec<ConfigShowEntry>,
    pub warnings: Vec<String>,
}

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

pub fn collect_config_show(orchestrator: Arc<dyn IConfigOrchestratorAggregate>) -> ConfigShowReport {
    let project_root = FilePath::new(".".to_string()).unwrap_or_default();

    // Iterate known languages using sync read_config instead of async list_config_files
    let languages = [
        ConfigLanguage::Rust,
        ConfigLanguage::Python,
        ConfigLanguage::TypeScript,
    ];

    let mut report = ConfigShowReport::default();
    for lang in &languages {
        match orchestrator.read_config(&project_root, *lang) {
            Ok(Some(source)) => {
                report.entries.push(ConfigShowEntry {
                    language: lang.as_str().to_string(),
                    path: source.path.value.clone(),
                    content: redact_secrets(&source.raw_content),
                });
            }
            Ok(None) => {}
            Err(e) => {
                report.warnings.push(format!(
                    "Warning: Failed to read config for {}: {}",
                    lang.as_str(),
                    e
                ));
            }
        }
    }
    report
}
