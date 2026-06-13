// PURPOSE: main entry point for lint-arwaky-cli — parses args, initializes DI, dispatches commands
use std::collections::HashMap;
use std::env;
use std::process::ExitCode;
use std::sync::Arc;

use clap::Parser;
use cli_commands::surface_bootstrap_command;
use cli_commands::surface_check_command;
use cli_commands::surface_config_command;
use cli_commands::surface_core_command::{Cli, Commands};
use cli_commands::surface_dev_command;
use cli_commands::surface_fix_command;
use cli_commands::surface_git_command;
use cli_commands::surface_maintenance_command;
use cli_commands::surface_map_command;
use cli_commands::surface_multi_command;
use cli_commands::surface_plugin_command;
use cli_commands::surface_report_command;
use cli_commands::surface_setup_command;
use cli_commands::surface_watch_command;
use code_analysis::agent_checking_orchestrator::init_global_checker;
use code_analysis::{has_critical, lint_path, CodeMetricAnalyzer, ProjectTargetResolver};
use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;

pub struct CliMainEntry {}

fn main() -> ExitCode {
    // Inline CLI composition — create exactly what CLI needs
    let source_parsing_container = source_parsing::root_source_parsing_container::SourceParsingContainer::new();
    let path_norm = source_parsing_container.path_normalization();

    let arch_linter = code_analysis::root_code_analysis_container::AnalysisContainer::new().architecture_linter();

    let auto_fix_container = auto_fix::root_auto_fix_container::AutoFixContainer::new(arch_linter.clone());

    let import_container = import_rules::root_import_rules_container::ImportContainer::new();
    let analyzer = import_container.analyzer();
    let checker_container = code_analysis::root_code_analysis_container::CheckerContainer::new(analyzer);
    init_global_checker(Arc::new(checker_container));

    let executor = Arc::new(cli_commands::infrastructure_transport_client::StdioClient::new(
        std::time::Duration::from_secs(60),
    ));

    let mut linter_adapters: HashMap<String, Arc<dyn shared::code_analysis::contract_adapter_port::ILinterAdapterPort>> = HashMap::new();
    linter_adapters.insert("ruff".to_string(), Arc::new(language_adapters::infrastructure_py_ruff_adapter::RuffAdapter::new(executor.clone(), path_norm.clone(), None)));
    linter_adapters.insert("bandit".to_string(), Arc::new(language_adapters::infrastructure_py_bandit_adapter::BanditAdapter::new(executor.clone(), path_norm.clone(), None)));
    linter_adapters.insert("mypy".to_string(), Arc::new(language_adapters::infrastructure_py_mypy_adapter::MyPyAdapter::new(executor.clone(), path_norm.clone(), None)));
    linter_adapters.insert("eslint".to_string(), Arc::new(language_adapters::infrastructure_js_linter_adapter::ESLintAdapter::new(executor.clone(), path_norm.clone())));
    linter_adapters.insert("prettier".to_string(), Arc::new(language_adapters::infrastructure_js_linter_adapter::PrettierAdapter::new(executor.clone(), path_norm.clone())));
    linter_adapters.insert("tsc".to_string(), Arc::new(language_adapters::infrastructure_js_linter_adapter::TSCAdapter::new(executor.clone(), path_norm.clone())));
    linter_adapters.insert("clippy".to_string(), Arc::new(language_adapters::infrastructure_rs_clippy_adapter::RustLinterAdapter::new(executor.clone(), path_norm.clone(), None)));

    let fix_container = auto_fix_container.clone();
    let fix_orchestrator_factory: Arc<dyn Fn(bool) -> Arc<dyn shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate> + Send + Sync> =
        Arc::new(move |dry_run| fix_container.orchestrator(dry_run));

    let raw_args: Vec<String> = env::args().collect();
    if raw_args.len() <= 1 {
        return run_default_check(".");
    }

    let cli = match Cli::try_parse_from(&raw_args) {
        Ok(c) => c,
        Err(e) => e.exit(),
    };

    let filter = cli.filter.clone();
    match cli.command {
        Commands::Check { path, git_diff } => {
            surface_check_command::handle_check(path, git_diff, filter)
        }
        Commands::Scan { path } => {
            surface_check_command::handle_scan(path, linter_adapters, arch_linter, filter)
        }
        Commands::Fix { path, dry_run } => {
            surface_fix_command::handle_fix(path, dry_run, fix_orchestrator_factory)
        }
        Commands::Report { path, output_format } => surface_report_command::handle_report(path, output_format),
        Commands::Ci { path, threshold } => surface_dev_command::handle_ci(path, threshold),
        Commands::Version => {
            let verbose = raw_args.iter().any(|a| a == "--verbose" || a == "-v");
            surface_bootstrap_command::handle_version(verbose)
        }
        Commands::Adapters => surface_plugin_command::handle_adapters(linter_adapters),
        Commands::Config { command } => surface_config_command::handle_config(command),
        Commands::GitDiff { base } => surface_git_command::handle_git_diff(base),
        Commands::MultiProject { paths } => surface_multi_command::handle_multi_project(paths),
        Commands::Security { path } => surface_maintenance_command::handle_security(path),
        Commands::Complexity { path } => {
            let resolver = ProjectTargetResolver::new();
            let analyzer = CodeMetricAnalyzer::new(Arc::new(resolver));
            analyzer.handle_complexity(path)
        }
        Commands::Duplicates { path } => {
            let resolver = ProjectTargetResolver::new();
            let analyzer = CodeMetricAnalyzer::new(Arc::new(resolver));
            analyzer.handle_duplicates(path)
        }
        Commands::Trends { path } => {
            let resolver = ProjectTargetResolver::new();
            let analyzer = CodeMetricAnalyzer::new(Arc::new(resolver));
            analyzer.handle_trends(path)
        }
        Commands::Dependencies { path } => surface_maintenance_command::handle_dependencies(path),
        Commands::Setup { command } => surface_setup_command::handle_setup(command),
        Commands::Cancel { job_id } => surface_map_command::handle_cancel(job_id),
        Commands::Diff { path1, path2 } => surface_map_command::handle_diff(path1, path2),
        Commands::Import { config_file } => surface_map_command::handle_import(config_file),
        Commands::Export { format } => surface_map_command::handle_export(format),
        Commands::Watch { path } => surface_watch_command::handle_watch(path),
        Commands::Suggest { path, ai: _ } => surface_map_command::handle_suggest(path),
        Commands::InstallHook => surface_git_command::handle_install_hook(),
        Commands::UninstallHook => surface_git_command::handle_uninstall_hook(),
    }
}

fn run_default_check(project_root: &str) -> ExitCode {
    use output_report::capabilities_reporting_formatter::ReportFormatterProcessor;
    let results = lint_path(project_root);
    let formatter = ReportFormatterProcessor::new();
    let report = formatter.format_text(&results, project_root);
    println!("Lint Arwaky v{} (AES Self-Lint)", env!("CARGO_PKG_VERSION"));
    println!("Scanning: {}", project_root);
    println!();
    println!("{}", report);
    if has_critical(&results) {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
