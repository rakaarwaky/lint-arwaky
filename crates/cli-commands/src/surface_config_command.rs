// PURPOSE: ConfigCommandsSurface — CLI surface for config show
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use std::process::ExitCode;
use std::sync::Arc;

/// Redact sensitive values from config content.
///
/// Scans for common secret patterns (AWS keys, base64-encoded secrets) and
/// replaces them with [REDACTED] placeholders. Uses simple string matching
/// without regex to avoid adding new dependencies.
fn redact_secrets(content: &str) -> String {
    let mut result = content.to_string();

    // Redact AWS access key IDs (AKIA followed by 12+ alphanumeric chars)
    if result.contains("AKIA") {
        // Simple heuristic: replace AKIA + 16 alphanumeric chars with [REDACTED]
        let re = regex::Regex::new(r"AKIA[0-9A-Z]{16}").ok();
        if let Some(re) = re {
            result = re.replace_all(&result, "[REDACTED-AWS-KEY]").to_string();
        }
    }

    // Redact very long base64-like strings (40+ chars of base64 alphabet)
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

pub async fn handle_config_show(
    _orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    config_reader: Arc<dyn IConfigReaderProtocol>,
) -> ExitCode {
    let project_root = FilePath::new(".".to_string()).unwrap_or_default();

    match config_reader.list_config_files(&project_root).await {
        Ok(config_files) if !config_files.is_empty() => {
            for (lang, path) in &config_files {
                match config_reader.read_config(&project_root, *lang).await {
                    Ok(Some(source)) => {
                        let path_str = path.value.as_str();
                        if config_files.len() > 1 {
                            println!("── [{}] {} ──", lang.as_str(), path_str);
                        } else {
                            println!("Found: {}", path_str);
                        }
                        // P5.2: redact secrets before displaying config content
                        let safe_content = redact_secrets(&source.raw_content);
                        println!("{safe_content}");
                    }
                    Ok(None) => {
                        // Should not happen since list_config_files found it
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to read config: {}", e);
                    }
                }
            }
        }
        Ok(_) => {
            println!("No config file found. Run `lint-arwaky init` to create one.");
        }
        Err(e) => {
            eprintln!("Failed to list config files: {}", e);
        }
    }
    ExitCode::SUCCESS
}
