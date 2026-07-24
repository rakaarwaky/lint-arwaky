// PURPOSE: main entry point for lint-arwaky-cli — parses args, initializes DI, dispatches commands
use std::env;
use std::process::ExitCode;

use cli_commands::surface_check_command;
use cli_commands::surface_ci_command;
use cli_commands::surface_fix_action;
use cli_commands::surface_plugin_command;
use cli_commands::surface_watch_command;
use cli_commands::CliContainer;
use shared::cli_commands::taxonomy_cli_vo::{Cli, Commands};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_threshold_vo::Threshold;

pub struct CliMainEntry {}

fn main() -> ExitCode {
    let raw_args: Vec<String> = env::args().collect();
    if raw_args.len() <= 1 {
        let container = CliContainer::new_default();
        return surface_check_command::handle_default_check(
            ".",
            container.code_analysis_linter.clone(),
        );
    }

    let cli = match <Cli as clap::Parser>::try_parse_from(&raw_args) {
        Ok(c) => c,
        Err(e) => e.exit(),
    };

    // P3.3: handle lightweight commands before constructing full container
    match &cli.command {
        Commands::Version => {
            let verbose = raw_args.iter().any(|a| a == "--verbose" || a == "-v");
            let ver = env!("CARGO_PKG_VERSION");
            if verbose {
                println!("Lint Arwaky v{}", ver);
                let commit = match std::process::Command::new("git")
                    .args(["rev-parse", "HEAD"])
                    .output()
                    .ok()
                    .filter(|o| o.status.success())
                    .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                {
                    Some(c) => c,
                    None => "unknown".to_string(),
                };
                println!("  Commit:    {}", commit);
                println!("  Rustc:     {}", default_rustc_version());
                println!("  License:   MIT");
            } else {
                println!("Lint Arwaky v{ver} (AES Semantic Builder)");
            }
            return ExitCode::SUCCESS;
        }
        Commands::Adapters => {
            let external_lint_container =
                external_lint::root_external_lint_container::ExternalLintContainer::new_default();
            return surface_plugin_command::handle_adapters(external_lint_container.aggregate());
        }
        _ => {}
    }

    let container = CliContainer::new_default();
    let filter = cli.filter.clone();

    match cli.command {
        Commands::Scan {
            path,
            member,
            format,
        } => surface_check_command::handle_scan(surface_check_command::ScanOptions {
            path: path.map(|p| FilePath::new(p).unwrap_or_default()),
            report_formatter: container.report_formatter.clone(),
            multi_project_orchestrator: Some(container.multi_project_orchestrator.clone()),
            filter,
            member,
            format,
        }),
        Commands::Fix { path, dry_run } => surface_fix_action::handle_fix(
            path.map(|p| FilePath::new(p).unwrap_or_default()),
            dry_run,
            container.code_analysis_linter.clone(),
            container.fix_orchestrator_factory(),
        ),
        Commands::Ci { path, threshold } => surface_ci_command::handle_ci(
            container.code_analysis_linter.clone(),
            container.import_orchestrator.clone(),
            container.naming_orchestrator.clone(),
            container.role_orchestrator.clone(),
            container.orphan_orchestrator.clone(),
            path.map(|p| FilePath::new(p).unwrap_or_default()),
            Threshold::new(threshold),
        ),
        Commands::Doctor => {
            let maintenance_container =
                maintenance::root_maintenance_container::MaintenanceContainer::new();
            let orchestrator = maintenance_container.orchestrator();
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(r) => r,
                Err(_) => return ExitCode::from(2),
            };
            rt.block_on(cli_commands::surface_maintenance_command::handle_doctor(
                orchestrator,
            ))
        }
        Commands::Orphan { path, member, format } => {
            let path_obj = std::path::Path::new(&path);
            if path_obj.is_file() {
                // Single file mode
                let surface = cli_commands::surface_check_command::CheckCommandsSurface::new(
                    container.report_formatter.clone(),
                    None,
                );
                surface.check_orphan_single_file(&path);
                ExitCode::SUCCESS
            } else {
                // Directory mode — scan all files
                cli_commands::surface_orphan
                    Some(FilePath::new(path).unwrap_or_default()),
                    member,
                    format,
                    container.orphan_orchestrator.clone(),
                    container.multi_project_orchestrator.clone(),
                    container.report_formatter.clone(),
                )
            }
        }
        Commands::ScanQuality { path, format } => {
            cli_commands::surface_quality_command::handle_scan_quality(
                path.map(|p| FilePath::new(p).unwrap_or_default()),
                format,
                container.code_analysis_linter.clone(),
                container.report_formatter.clone(),
            )
        }
        Commands::ScanImport { path, format } => cli_commands::surface_import_command::handle_scan_import(
            path.map(|p| FilePath::new(p).unwrap_or_default()),
            format,
            container.import_orchestrator.clone(),
            container.report_formatter.clone(),
        ),
        Commands::ScanNaming { path, format } => cli_commands::surface_naming_command::handle_scan_naming(
            path.map(|p| FilePath::new(p).unwrap_or_default()),
            format,
            container.naming_orchestrator.clone(),
            container.report_formatter.clone(),
        ),
        Commands::ScanRole { path, format } => cli_commands::surface_role_command::handle_scan_role(
            path.map(|p| FilePath::new(p).unwrap_or_default()),
            format,
            container.role_orchestrator.clone(),
            container.report_formatter.clone(),
        ),
        Commands::ScanExternal { path, format } => {
            cli_commands::surface_external_command::handle_scan_external(
                path.map(|p| FilePath::new(p).unwrap_or_default()),
                format,
                container.external_lint.clone(),
                container.report_formatter.clone(),
            )
        }
        Commands::Security { path } => {
            let maintenance_container =
                maintenance::root_maintenance_container::MaintenanceContainer::new();
            let orchestrator = maintenance_container.orchestrator();
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(r) => r,
                Err(_) => return ExitCode::from(2),
            };
            rt.block_on(cli_commands::surface_maintenance_command::handle_security(
                orchestrator,
                path.map(|p| FilePath::new(p).unwrap_or_default()),
            ))
        }
        Commands::Dependencies { path } => {
            let maintenance_container =
                maintenance::root_maintenance_container::MaintenanceContainer::new();
            let orchestrator = maintenance_container.orchestrator();
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(r) => r,
                Err(_) => return ExitCode::from(2),
            };
            rt.block_on(
                cli_commands::surface_maintenance_command::handle_dependencies(
                    orchestrator,
                    path.map(|p| FilePath::new(p).unwrap_or_default()),
                ),
            )
        }
        Commands::Watch { path } => {
            let fwatch_container = file_watch::FileWatchContainer::new();
            let watch_agg: std::sync::Arc<
                dyn shared::file_watch::contract_watch_aggregate::IWatchAggregate,
            > = fwatch_container.orchestrator(container.code_analysis_linter.clone());
            surface_watch_command::handle_watch(
                watch_agg,
                path.map(|p| FilePath::new(p).unwrap_or_default()),
            )
        }
        Commands::InstallHook => {
            let git_hooks_container =
                git_hooks::root_git_hooks_container::GitContainer::new_default();
            let aggregate = git_hooks_container.aggregate();
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(r) => r,
                Err(_) => return ExitCode::from(2),
            };
            let exe_path = default_file_path("lint-arwaky".to_string());
            match rt.block_on(aggregate.install_hook(&exe_path)) {
                Ok(status) if status.value => {
                    println!("Installed git pre-commit hook successfully");
                    ExitCode::SUCCESS
                }
                Ok(_) => {
                    println!("Not a git repository, hook installation skipped");
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Failed to install pre-commit hook: {:?}", e);
                    ExitCode::from(2)
                }
            }
        }
        Commands::UninstallHook => {
            let git_hooks_container =
                git_hooks::root_git_hooks_container::GitContainer::new_default();
            let aggregate = git_hooks_container.aggregate();
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(r) => r,
                Err(_) => return ExitCode::from(2),
            };
            match rt.block_on(aggregate.uninstall_hook()) {
                Ok(status) if status.value => {
                    println!("Removed git pre-commit hook successfully");
                    ExitCode::SUCCESS
                }
                Ok(_) => {
                    println!("Not a git repository, hook removal skipped");
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Failed to remove pre-commit hook: {:?}", e);
                    ExitCode::FAILURE
                }
            }
        }
        Commands::Init => {
            let setup_container =
                project_setup::root_project_setup_container::SetupContainer::new();
            cli_commands::surface_setup_command::handle_init(setup_container.aggregate())
        }
        Commands::Install { sudo } => {
            let setup_container =
                project_setup::root_project_setup_container::SetupContainer::new();
            let setup_orchestrator = setup_container.aggregate();
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(r) => r,
                Err(_) => return ExitCode::from(2),
            };
            rt.block_on(cli_commands::surface_setup_command::handle_install(
                setup_orchestrator,
                sudo,
            ))
        }
        Commands::McpConfig { client } => {
            cli_commands::surface_setup_command::handle_mcp_config(&client)
        }
        Commands::ConfigShow => {
            let config_container =
                config_system::root_config_system_container::ConfigContainer::new();
            let config_orchestrator = config_container.orchestrator();
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(r) => r,
                Err(_) => return ExitCode::from(2),
            };
            rt.block_on(cli_commands::surface_config_command::handle_config_show(
                config_orchestrator,
            ))
        }
        // P3.3: these are handled by early-exit above
        _ => ExitCode::SUCCESS,
    }
}

fn default_file_path(s: String) -> shared::common::taxonomy_path_vo::FilePath {
    shared::common::taxonomy_path_vo::FilePath::new(s).unwrap_or_default()
}

fn default_rustc_version() -> &'static str {
    option_env!("VERGEN_RUSTC_SEMVER")
        .or_else(|| option_env!("RUSTC_VERSION"))
        .unwrap_or("stable")
}
