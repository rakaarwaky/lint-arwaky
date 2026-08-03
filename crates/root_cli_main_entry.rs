// PURPOSE: CLI binary entry point — wiring all dependencies + arg dispatch.
// Concrete container construction happens here; cli-commands surfaces
// (surface_*_command) handle output formatting + exit-code mapping.
use clap::{Parser, Subcommand};
use std::str::FromStr;
use std::sync::Arc;

use shared::cli_commands::Format;
use shared::common::{FilePath, GitBranchName, Threshold};

#[derive(Parser)]
#[command(
    name = "lint-arwaky",
    version,
    about = "Autonomous code quality and architecture enforcement"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run all 6 linters (quality, role, import, naming, orphan, external)
    Scan {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
        #[arg(long, default_value = "text")]
        format: String,
        #[arg(long)]
        filter: Option<String>,
        #[arg(long)]
        member: Option<String>,
    },
    /// Quality rules scan (AES quality violations)
    Check {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
        #[arg(long, default_value = "text")]
        format: String,
        #[arg(long)]
        filter: Option<String>,
    },
    /// Quality rules scan (alias of check)
    Quality {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
        #[arg(long, default_value = "text")]
        format: String,
        #[arg(long)]
        filter: Option<String>,
    },
    /// Role rules scan
    Role {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
        #[arg(long, default_value = "text")]
        format: String,
        #[arg(long)]
        filter: Option<String>,
    },
    /// Import rules scan
    Import {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
        #[arg(long, default_value = "text")]
        format: String,
        #[arg(long)]
        filter: Option<String>,
    },
    /// Naming rules scan
    Naming {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
        #[arg(long, default_value = "text")]
        format: String,
        #[arg(long)]
        filter: Option<String>,
    },
    /// Orphan detection scan
    Orphan {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
        #[arg(long, default_value = "text")]
        format: String,
        #[arg(long)]
        filter: Option<String>,
        #[arg(long)]
        member: Option<String>,
    },
    /// External lint scan (ruff, eslint, ...)
    External {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
        #[arg(long, default_value = "text")]
        format: String,
        #[arg(long)]
        filter: Option<String>,
    },
    /// CI threshold validation
    Ci {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
        #[arg(long, default_value_t = 80)]
        threshold: u32,
    },
    /// Show effective architecture config
    Config,
    /// Auto-fix AES301-305 violations
    Fix {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
        #[arg(long)]
        dry_run: bool,
    },
    /// Scan files changed since git base
    Git {
        #[arg(long, default_value = "develop")]
        base: String,
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
        #[arg(long)]
        filter: Option<String>,
    },
    /// Environment diagnostics
    Doctor,
    /// Security vulnerability scan
    Security {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
    },
    /// Dependency report
    Dependencies {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
    },
    /// List external lint adapters
    Adapters,
    /// Create config files + docs in project
    Init,
    /// Install adapter dependencies
    Install {
        #[arg(long)]
        sudo: bool,
    },
    /// Print MCP client configuration snippet
    McpConfig {
        #[arg(value_name = "CLIENT")]
        client: String,
    },
    /// Watch files and auto-lint on change
    Watch {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
    },
    /// Print version
    Version,
}

fn parse_format(s: &str) -> Format {
    Format::from_str(s).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(2);
    })
}

fn main() {
    let cli = Cli::parse();

    let filesystem: Arc<
        dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate,
    > = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();

    let config_container =
        config_system::root_config_system_container::ConfigContainer::new(filesystem.clone());
    let config_orchestrator = config_container.orchestrator();

    let code_analysis_linter =
        quality_rules::root_quality_rules_container::CodeAnalysisContainer::from_orchestrator(
            &config_orchestrator,
            ".",
        )
        .code_analysis_linter();

    let import_container =
        import_rules::root_import_rules_container::ImportContainer::from_orchestrator(
            &config_orchestrator,
            ".",
            filesystem.clone(),
        );
    let import_orchestrator = import_container.orchestrator();

    let naming_container = naming_rules::root_naming_rules_container::NamingContainer::new(
        Arc::new(
            config_orchestrator
                .load_config_sync(&FilePath::new(".".to_string()).unwrap_or_default()),
        ),
        Arc::new(shared::common::LayerMapVO::new(
            config_orchestrator
                .load_config_sync(&FilePath::new(".".to_string()).unwrap_or_default())
                .layers
                .clone(),
        )),
    );
    let naming_orchestrator = naming_container.orchestrator();

    let orphan_container =
        orphan_rules::root_orphan_detector_container::OrphanContainer::from_orchestrator(
            &config_orchestrator,
            ".",
            filesystem.clone(),
        );
    let orphan_orchestrator = orphan_container.analyzer();

    let ext_container = external_lint::root_external_lint_container::ExternalLintContainer::new(
        filesystem.clone(),
        config_container.parser(),
    );
    let external_lint = ext_container.aggregate();

    let role_container = role_rules::root_role_rules_container::RoleContainer::new_with_config(
        config_orchestrator.load_config_sync(&FilePath::new(".".to_string()).unwrap_or_default()),
    );
    let role_orchestrator = role_container.orchestrator();

    let auto_fix_container =
        auto_fix::root_auto_fix_container::AutoFixContainer::new(code_analysis_linter.clone());
    // BF-1: dry_run is now per-request via execute(path, dry_run), not baked into orchestrator.
    // Factory ignores the bool parameter for backwards compatibility; callers pass dry_run to execute().
    let fix_orchestrator_factory: Arc<
        dyn Fn(bool) -> Arc<dyn shared::auto_fix::LintFixOrchestratorAggregate> + Send + Sync,
    > = {
        let container = auto_fix_container;
        let fs_for_factory = filesystem.clone();
        Arc::new(move |_dry| container.orchestrator_with_filesystem(fs_for_factory.clone()))
    };

    let maintenance_container =
        maintenance::root_maintenance_container::MaintenanceContainer::new(filesystem.clone());
    let maintenance_orchestrator = maintenance_container.orchestrator();

    let setup_container =
        project_setup::root_project_setup_container::SetupContainer::new(filesystem.clone());
    let setup_orchestrator = setup_container.aggregate();

    let watch_aggregate = file_watch::root_file_watch_container::FileWatchContainer::new()
        .aggregate(code_analysis_linter.clone());

    let report_formatter: Arc<dyn shared::report_formatter::IReportFormatterAggregate> = Arc::new(
        report_formatter::ReportFormatterOrchestrator::new(report_formatter::ReportFormatterDeps {
            text: Arc::new(report_formatter::TextFormatter::new()),
            json: Arc::new(report_formatter::JsonFormatter::new()),
            sarif: Arc::new(report_formatter::SarifFormatter::new()),
            junit: Arc::new(report_formatter::JunitFormatter::new()),
        }),
    );

    let exit_code = match cli.command {
        Command::Scan {
            path,
            format,
            filter,
            member,
        } => cli_commands::surface_scan_command::handle_scan(
            Some(FilePath::new(path).unwrap_or_default()),
            parse_format(&format),
            filesystem.clone(),
            Some(config_orchestrator.clone()),
            filter,
            member,
        ),
        Command::Check {
            path,
            format,
            filter,
        } => cli_commands::surface_scan_command::handle_check(
            Some(FilePath::new(path).unwrap_or_default()),
            parse_format(&format),
            code_analysis_linter.clone(),
            filesystem.clone(),
            Some(config_orchestrator.clone()),
            filter,
        ),
        Command::Quality {
            path,
            format,
            filter,
        } => cli_commands::surface_scan_command::handle_quality(
            Some(FilePath::new(path).unwrap_or_default()),
            parse_format(&format),
            code_analysis_linter.clone(),
            filesystem.clone(),
            filter,
        ),
        Command::Role {
            path,
            format,
            filter,
        } => cli_commands::surface_scan_command::handle_role(
            Some(FilePath::new(path).unwrap_or_default()),
            parse_format(&format),
            role_orchestrator.clone(),
            report_formatter.clone(),
            filesystem.clone(),
            filter,
        ),
        Command::Import {
            path,
            format,
            filter,
        } => cli_commands::surface_scan_command::handle_import(
            Some(FilePath::new(path).unwrap_or_default()),
            parse_format(&format),
            import_orchestrator.clone(),
            report_formatter.clone(),
            filesystem.clone(),
            filter,
        ),
        Command::Naming {
            path,
            format,
            filter,
        } => cli_commands::surface_scan_command::handle_naming(
            Some(FilePath::new(path).unwrap_or_default()),
            parse_format(&format),
            naming_orchestrator.clone(),
            report_formatter.clone(),
            filesystem.clone(),
            filter,
        ),
        Command::Orphan {
            path,
            format,
            filter,
            member,
        } => cli_commands::surface_scan_command::handle_orphan(
            Some(FilePath::new(path).unwrap_or_default()),
            member,
            parse_format(&format),
            orphan_orchestrator.clone(),
            config_orchestrator.clone(),
            report_formatter.clone(),
            filesystem.clone(),
            filter,
        ),
        Command::External {
            path,
            format,
            filter,
        } => cli_commands::surface_scan_command::handle_external(
            Some(FilePath::new(path).unwrap_or_default()),
            parse_format(&format),
            external_lint.clone(),
            report_formatter.clone(),
            filesystem.clone(),
            filter,
        ),
        Command::Ci { path, threshold } => cli_commands::surface_ci_command::handle_ci(
            code_analysis_linter.clone(),
            import_orchestrator.clone(),
            naming_orchestrator.clone(),
            config_orchestrator.clone(),
            orphan_orchestrator.clone(),
            filesystem.clone(),
            Some(FilePath::new(path).unwrap_or_default()),
            Threshold::new(threshold),
        ),
        Command::Config => {
            cli_commands::surface_config_command::handle_config_show(config_orchestrator.clone())
        }
        Command::Fix { path, dry_run } => cli_commands::surface_fix_command::handle_fix(
            Some(FilePath::new(path).unwrap_or_default()),
            dry_run,
            code_analysis_linter.clone(),
            fix_orchestrator_factory.clone(),
        ),
        Command::Git { base, path, filter } => cli_commands::surface_git_command::handle_git_diff(
            code_analysis_linter.clone(),
            GitBranchName::new(base),
            Some(&path),
            filter.as_deref(),
        ),
        Command::Doctor => cli_commands::surface_maintenance_command::handle_doctor(
            maintenance_orchestrator.clone(),
        ),
        Command::Security { path } => cli_commands::surface_maintenance_command::handle_security(
            maintenance_orchestrator.clone(),
            Some(FilePath::new(path).unwrap_or_default()),
        ),
        Command::Dependencies { path } => {
            cli_commands::surface_maintenance_command::handle_dependencies(
                maintenance_orchestrator.clone(),
                Some(FilePath::new(path).unwrap_or_default()),
            )
        }
        Command::Adapters => {
            cli_commands::surface_plugin_command::handle_adapters(external_lint.clone())
        }
        Command::Init => {
            cli_commands::surface_setup_command::handle_init(setup_orchestrator.clone())
        }
        Command::Install { sudo } => {
            cli_commands::surface_setup_command::handle_install(setup_orchestrator.clone(), sudo)
        }
        Command::McpConfig { client } => {
            cli_commands::surface_setup_command::handle_mcp_config(&client)
        }
        Command::Watch { path } => cli_commands::surface_watch_command::handle_watch(
            watch_aggregate.clone(),
            Some(FilePath::new(path).unwrap_or_default()),
        ),
        Command::Version => {
            println!("lint-arwaky {}", env!("CARGO_PKG_VERSION"));
            shared::common::ExitCode::OK
        }
    };

    std::process::exit(exit_code.value() as i32);
}
