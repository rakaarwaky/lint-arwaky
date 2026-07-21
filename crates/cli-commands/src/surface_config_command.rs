// PURPOSE: ConfigCommandsSurface — CLI surface for config show
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use std::process::ExitCode;
use std::sync::Arc;

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
                        println!("{}", source.raw_content);
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
