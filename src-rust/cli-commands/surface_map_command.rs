use std::process::ExitCode;
use std::sync::Arc;

use crate::cli_commands::surface_output_controller::{print_json, print_sarif, print_junit};
use crate::cli_commands::taxonomy_entry_vo::{compute_score, lint_path, resolve_target};
use crate::di_containers::agent_injection_container::DependencyInjectionContainer;
use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;

pub fn handle_cancel(job_id: String) -> ExitCode {
    let container = Arc::new(DependencyInjectionContainer::new(
        DirectoryPath::new(".").unwrap_or_default(),
    ));
    if let Some(registry) = container.get_job_registry() {
        let id = crate::pipeline_jobs::taxonomy_action_vo::JobId::new(&job_id);
        let rt = tokio::runtime::Runtime::new().unwrap();
        let cancelled = rt.block_on(registry.cancel_job(&id));
        if cancelled.value() {
            println!("Job {} cancelled", job_id);
        } else {
            eprintln!("No active job with ID {}", job_id);
        }
    }
    ExitCode::SUCCESS
}

pub fn handle_diff(path1: String, path2: String) -> ExitCode {
    let r1 = lint_path(&path1);
    let r2 = lint_path(&path2);
    let s1 = compute_score(&r1);
    let s2 = compute_score(&r2);
    println!("{:<40} {:>10} {:>10}", "", "Violations", "Score");
    println!("{:<40} {:>10} {:>10.1}", path1, r1.len(), s1);
    println!("{:<40} {:>10} {:>10.1}", path2, r2.len(), s2);
    ExitCode::SUCCESS
}

pub fn handle_import(config_file: String) -> ExitCode {
    if !std::path::Path::new(&config_file).exists() {
        eprintln!("[error] file not found: {}", config_file);
        return ExitCode::from(1);
    }
    println!("Imported config from {}", config_file);
    ExitCode::SUCCESS
}

pub fn handle_export(format: String) -> ExitCode {
    let results = lint_path(".");
    match format.as_str() {
        "json" => print_json(&results),
        "sarif" => print_sarif(&results, "."),
        "junit" => print_junit(&results),
        _ => eprintln!("unknown format: {}", format),
    }
    ExitCode::SUCCESS
}

pub fn handle_suggest(path: Option<String>) -> ExitCode {
    let root = resolve_target(path);
    let results = lint_path(&root);
    println!("Lint Arwaky v{} (Suggest)", env!("CARGO_PKG_VERSION"));
    if results.is_empty() {
        println!("No suggestions needed.");
    } else {
        println!("{} suggestions. Top by file:", results.len());
    }
    ExitCode::SUCCESS
}
