use std::env;
use std::process::ExitCode;

use clap::Parser;
use lint_arwaky::cli_commands::surface_analysis_command;
use lint_arwaky::cli_commands::surface_check_command;
use lint_arwaky::cli_commands::surface_config_command;
use lint_arwaky::cli_commands::surface_core_command::{Cli, Commands};
use lint_arwaky::cli_commands::surface_dev_command;
use lint_arwaky::cli_commands::surface_fix_command;
use lint_arwaky::cli_commands::surface_git_command;
use lint_arwaky::cli_commands::surface_bootstrap_command;
use lint_arwaky::cli_commands::surface_maintenance_command;
use lint_arwaky::cli_commands::surface_map_command;
use lint_arwaky::cli_commands::surface_multi_command;
use lint_arwaky::cli_commands::surface_plugin_command;
use lint_arwaky::cli_commands::surface_report_command;
use lint_arwaky::cli_commands::surface_setup_command;
use lint_arwaky::cli_commands::surface_watch_command;
use lint_arwaky::cli_commands::taxonomy_command_target_vo;

pub struct CliMainEntry {}

fn main() -> ExitCode {
    let raw_args: Vec<String> = env::args().collect();
    if raw_args.len() <= 1 {
        return run_default_check(".");
    }

    let cli = match Cli::try_parse_from(&raw_args) {
        Ok(c) => c,
        Err(e) => e.exit(),
    };

    match cli.command {
        Commands::Check { path, git_diff } => surface_check_command::handle_check(path, git_diff),
        Commands::Scan { path } => surface_check_command::handle_scan(path),
        Commands::Fix { path, dry_run } => surface_fix_command::handle_fix(path, dry_run),
        Commands::Report {
            path,
            output_format,
        } => surface_report_command::handle_report(path, output_format),
        Commands::Ci { path, threshold } => surface_dev_command::handle_ci(path, threshold),
        Commands::Version => {
            let verbose = raw_args.iter().any(|a| a == "--verbose" || a == "-v");
            surface_bootstrap_command::handle_version(verbose)
        }
        Commands::Adapters => surface_plugin_command::handle_adapters(),
        Commands::Config { command } => surface_config_command::handle_config(command),
        Commands::GitDiff { base } => surface_git_command::handle_git_diff(base),
        Commands::MultiProject { paths } => surface_multi_command::handle_multi_project(paths),
        Commands::Security { path } => surface_maintenance_command::handle_security(path),
        Commands::Complexity { path } => surface_analysis_command::handle_complexity(path),
        Commands::Duplicates { path } => surface_analysis_command::handle_duplicates(path),
        Commands::Trends { path } => surface_analysis_command::handle_trends(path),
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
    use lint_arwaky::output_report::capabilities_reporting_formatter::ReportFormatterProcessor;
    let results = taxonomy_command_target_vo::lint_path(project_root);
    let formatter = ReportFormatterProcessor::new();
    let report = formatter.format_text(&results, project_root);
    println!("Lint Arwaky v{} (AES Self-Lint)", env!("CARGO_PKG_VERSION"));
    println!("Scanning: {}", project_root);
    println!();
    println!("{}", report);
    if taxonomy_command_target_vo::has_critical(&results) {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
