// PURPOSE: ConfigCommandsSurface — CLI surface for config show
use shared::config_system::contract_orchestration_aggregate::IConfigOrchestrationAggregate;
use std::process::ExitCode;
use std::sync::Arc;

pub async fn handle_config_show(
    config_orchestrator: Arc<dyn IConfigOrchestrationAggregate>,
) -> ExitCode {
    let project_root =
        shared::source_parsing::taxonomy_path_vo::FilePath::new(".".to_string()).unwrap_or_default();

    let config_reader = config_orchestrator.config_reader();
    let config_files = config_reader.list_config_files(&project_root).await;

    if !config_files.is_empty() {
        let (lang, path_str) = &config_files[0];
        if let Some(source) = config_reader.read_config(&project_root, lang).await {
            println!("Found: {}", path_str);
            println!("{}", source.raw_content);
            return ExitCode::SUCCESS;
        }
    }

    println!("No config file found. Run `lint-arwaky init` to create one.");
    ExitCode::SUCCESS
}
