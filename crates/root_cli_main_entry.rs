// PURPOSE: CLI binary entry point — wiring all dependencies + arg dispatch.
// Concrete container construction happens here; cli-commands surfaces
// (surface_*_command) handle output formatting + exit-code mapping.
use clap::{Parser, Subcommand};
use std::str::FromStr;
use std::sync::Arc;

use shared_cli_commands::Format;
use shared_common::{FilePath, GitBranchName, Threshold};

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warn,lint_arwaky::audit=info".into()),
        )
        .with_writer(std::io::stderr)
        .init();
}

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
    /// Run all 6 linters (alias of scan)
    Check {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
        #[arg(long, default_value = "text")]
        format: String,
        #[arg(long)]
        filter: Option<String>,
        #[arg(long)]
        member: Option<String>,
    },
    /// Quality rules scan (single linter)
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
    /// Taxonomy-layer scan (only `taxonomy_*` files)
    Taxonomy {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
        #[arg(long, default_value = "text")]
        format: String,
        #[arg(long)]
        filter: Option<String>,
        #[arg(long)]
        member: Option<String>,
    },
    /// Contract-layer scan (only `contract_*` files)
    Contract {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
        #[arg(long, default_value = "text")]
        format: String,
        #[arg(long)]
        filter: Option<String>,
        #[arg(long)]
        member: Option<String>,
    },
    /// Capabilities-layer scan (only `capabilities_*` files)
    Capabilities {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
        #[arg(long, default_value = "text")]
        format: String,
        #[arg(long)]
        filter: Option<String>,
        #[arg(long)]
        member: Option<String>,
    },
    /// Utility-layer scan (only `utility_*` files)
    Utility {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
        #[arg(long, default_value = "text")]
        format: String,
        #[arg(long)]
        filter: Option<String>,
        #[arg(long)]
        member: Option<String>,
    },
    /// Agent-layer scan (only `agent_*` files)
    Agents {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
        #[arg(long, default_value = "text")]
        format: String,
        #[arg(long)]
        filter: Option<String>,
        #[arg(long)]
        member: Option<String>,
    },
    /// Surface-layer scan (only `surface_*` files)
    Surface {
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
    /// Doc invariants audit (AES601–AES605) over the document chain
    Docs {
        #[arg(value_name = "PATH", default_value = ".")]
        path: String,
        #[arg(long, default_value = "text")]
        format: String,
    },
    /// Structure rules scan (AES701–AES703)
    Structure {
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
    /// Install git pre-commit hook
    InstallHook,
    /// Uninstall git pre-commit hook
    UninstallHook,
    /// Print version
    Version,
    /// Self-update: install latest release binary from GitHub
    #[command(name = "update", alias = "la")]
    Update {
        /// Check for a newer release without installing
        #[arg(long)]
        check_only: bool,
    },
    /// Read or list embedded AES skill documentation
    Skill {
        #[command(subcommand)]
        sub: SkillSubCommand,
    },
}

#[derive(Subcommand)]
enum SkillSubCommand {
    /// Print the SKILL.md (and optionally references) for the named skill
    Read {
        #[arg(value_name = "NAME")]
        name: String,
        /// Also print language-specific reference HOW-TOs
        #[arg(long)]
        with_references: bool,
    },
    /// List all embedded skills
    List,
}

/// Dispatch one of the six layer-scoped subcommands. Every layer reuses the
/// `scan` wiring and differs only in which layer's violations are reported, so
/// the six call sites stay one line each.
fn run_layer_scan(
    context: &cli_commands::surface_scan_command::LayerScanContext,
    layer: &'static str,
    path: String,
    format: String,
    filter: Option<String>,
    member: Option<String>,
) -> shared_common::ExitCode {
    cli_commands::surface_scan_command::handle_layer_scan(
        context,
        layer,
        path,
        parse_format(&format),
        filter,
        member,
    )
}

fn parse_format(s: &str) -> Format {
    Format::from_str(s).unwrap_or_else(|e| {
        tracing::error!(error = %e, "invalid format");
        std::process::exit(2);
    })
}

fn main() {
    init_tracing();
    let cli = Cli::parse();

    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem: Arc<
        dyn shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate,
    > = fs_container.orchestrator();
    let filesystem_io = fs_container.io();
    let filesystem_workspace = fs_container.workspace();
    let filesystem_tool_resolution = fs_container.tool_resolution();
    let filesystem_parser = fs_container.parser();
    let fs_seam = dispatcher::surface_check_action::FilesystemSeam {
        io: filesystem_io.clone(),
        workspace: filesystem_workspace.clone(),
        parser: filesystem_parser.clone(),
        aggregate: filesystem.clone(),
    };

    let config_container = config_system::root_config_system_container::ConfigContainer::new(
        filesystem.clone(),
        filesystem_io.clone(),
    );
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
            filesystem_io.clone(),
            filesystem_workspace.clone(),
            filesystem_parser.clone(),
        );
    let import_orchestrator = import_container.orchestrator();

    let naming_container = naming_rules::root_naming_rules_container::NamingContainer::new(
        Arc::new(
            config_orchestrator
                .execute(shared_config_system::ConfigRequest::load_sync(
                    &FilePath::new(".".to_string()).unwrap_or_default(),
                ))
                .into_sync_config(),
        ),
        Arc::new(shared_common::LayerMapVO::new(
            config_orchestrator
                .execute(shared_config_system::ConfigRequest::load_sync(
                    &FilePath::new(".".to_string()).unwrap_or_default(),
                ))
                .into_sync_config()
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
            filesystem_workspace.clone(),
        );
    let orphan_orchestrator = orphan_container.analyzer();

    let ext_container = external_lint::root_external_lint_container::ExternalLintContainer::new(
        filesystem.clone(),
        filesystem_io.clone(),
        filesystem_tool_resolution.clone(),
    );
    let external_lint = ext_container.aggregate();

    let role_container = role_rules::root_role_rules_container::RoleContainer::new_with_config(
        config_orchestrator
            .execute(shared_config_system::ConfigRequest::load_sync(
                &FilePath::new(".".to_string()).unwrap_or_default(),
            ))
            .into_sync_config(),
    );
    let role_orchestrator = role_container.orchestrator();

    let auto_fix_container =
        auto_fix::root_auto_fix_container::AutoFixContainer::new(code_analysis_linter.clone());
    // BF-1: dry_run is now per-request via execute(path, dry_run), not baked into orchestrator.
    // Factory ignores the bool parameter for backwards compatibility; callers pass dry_run to execute().
    let fix_orchestrator_factory: Arc<
        dyn Fn(bool) -> Arc<dyn shared_auto_fix::IFixAggregate> + Send + Sync,
    > = {
        let container = auto_fix_container;
        let fs_for_factory = filesystem.clone();
        let io_for_factory = filesystem_io.clone();
        Arc::new(move |_dry| {
            container.orchestrator_with_filesystem(fs_for_factory.clone(), io_for_factory.clone())
        })
    };

    let maintenance_container = maintenance::root_maintenance_container::MaintenanceContainer::new(
        filesystem.clone(),
        filesystem_io.clone(),
    );
    let maintenance_orchestrator = maintenance_container.orchestrator();

    let structure_orchestrator =
        structure_rules::root_structure_rules_container::RootStructureRulesContainer::orchestrator(
        );

    let doc_orchestrator =
        doc_rules::root_doc_rules_container::RootDocRulesContainer::orchestrator();

    let setup_container =
        project_setup::root_project_setup_container::SetupContainer::new(filesystem_io.clone());
    let setup_orchestrator = setup_container.aggregate();

    let watch_aggregate = file_watch::root_file_watch_container::FileWatchContainer::new()
        .aggregate(code_analysis_linter.clone());

    let git_container = git_hooks::root_git_hooks_container::GitContainer::new(
        FilePath::new(
            std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| ".".to_string()),
        )
        .unwrap_or_default(),
        filesystem.clone(),
        filesystem_io.clone(),
    );
    let git_orchestrator = git_container.aggregate();

    let report_formatter: Arc<dyn shared_report_formatter::IReportFormatterAggregate> = Arc::new(
        report_formatter::ReportFormatterOrchestrator::new(report_formatter::ReportFormatterDeps {
            text: Arc::new(report_formatter::TextFormatter::new()),
            json: Arc::new(report_formatter::JsonFormatter::new()),
            sarif: Arc::new(report_formatter::SarifFormatter::new()),
            junit: Arc::new(report_formatter::JunitFormatter::new()),
        }),
    );

    // Extract ignored_paths from config for all sub-commands.
    let ignored_paths: Vec<String> = config_orchestrator
        .execute(shared_config_system::ConfigRequest::ignored_paths(
            &FilePath::new(".".to_string()).unwrap_or_default(),
        ))
        .into_patterns()
        .values
        .clone();

    // W10: in-process aggregate bundle for scan/check (subprocess fallback when absent).
    let scan_aggregates = dispatcher::surface_check_action::ScanAggregates {
        quality: code_analysis_linter.clone(),
        role: role_orchestrator.clone(),
        import: import_orchestrator.clone(),
        naming: naming_orchestrator.clone(),
        external: external_lint.clone(),
        orphan: orphan_orchestrator.clone(),
        config: config_orchestrator.clone(),
        structure: structure_orchestrator.clone(),
        doc: doc_orchestrator.clone(),
        fs_seam: Arc::new(fs_seam.clone()),
    };

    // Shared wiring for the six layer-scoped subcommands: same aggregates as
    // `scan`, only the reported layer differs.
    let layer_scan_context = cli_commands::surface_scan_command::LayerScanContext {
        filesystem_seam: fs_seam.clone(),
        config_orchestrator: Some(config_orchestrator.clone()),
        scan_aggregates: Some(scan_aggregates.clone()),
    };

    let exit_code = match cli.command {
        Command::Scan {
            path,
            format,
            filter,
            member,
        } => cli_commands::surface_scan_command::handle_scan(
            cli_commands::surface_scan_command::ScanCommandParams {
                path: Some(FilePath::new(path).unwrap_or_default()),
                format: parse_format(&format),
                filesystem: filesystem.clone(),
                filesystem_seam: fs_seam.clone(),
                config_orchestrator: Some(config_orchestrator.clone()),
                filter,
                member,
                scan_aggregates: Some(scan_aggregates.clone()),
            },
        ),
        Command::Check {
            path,
            format,
            filter,
            member,
        } => cli_commands::surface_scan_command::handle_scan(
            cli_commands::surface_scan_command::ScanCommandParams {
                path: Some(FilePath::new(path).unwrap_or_default()),
                format: parse_format(&format),
                filesystem: filesystem.clone(),
                filesystem_seam: fs_seam.clone(),
                config_orchestrator: Some(config_orchestrator.clone()),
                filter,
                member,
                scan_aggregates: Some(scan_aggregates.clone()),
            },
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
            fs_seam.clone(),
            filter,
            ignored_paths.clone(),
        ),
        Command::Role {
            path,
            format,
            filter,
        } => cli_commands::surface_scan_command::handle_role(
            cli_commands::surface_scan_command::RoleCommandParams {
                path: Some(FilePath::new(path).unwrap_or_default()),
                format: parse_format(&format),
                role_orchestrator: role_orchestrator.clone(),
                report_formatter: report_formatter.clone(),
                filesystem: filesystem.clone(),
                filesystem_seam: fs_seam.clone(),
                filter,
                ignored_paths: ignored_paths.clone(),
            },
        ),
        Command::Import {
            path,
            format,
            filter,
        } => cli_commands::surface_scan_command::handle_import(
            cli_commands::surface_scan_command::ImportCommandParams {
                path: Some(FilePath::new(path).unwrap_or_default()),
                format: parse_format(&format),
                import_orchestrator: import_orchestrator.clone(),
                report_formatter: report_formatter.clone(),
                filesystem: filesystem.clone(),
                filesystem_seam: fs_seam.clone(),
                filter,
                ignored_paths: ignored_paths.clone(),
            },
        ),
        Command::Naming {
            path,
            format,
            filter,
        } => cli_commands::surface_scan_command::handle_naming(
            cli_commands::surface_scan_command::NamingCommandParams {
                path: Some(FilePath::new(path).unwrap_or_default()),
                format: parse_format(&format),
                naming_orchestrator: naming_orchestrator.clone(),
                report_formatter: report_formatter.clone(),
                filesystem: filesystem.clone(),
                filesystem_seam: fs_seam.clone(),
                filter,
                ignored_paths: ignored_paths.clone(),
            },
        ),
        Command::Orphan {
            path,
            format,
            filter,
            member,
        } => cli_commands::surface_scan_command::handle_orphan(
            cli_commands::surface_scan_command::OrphanCommandParams {
                path: Some(FilePath::new(path).unwrap_or_default()),
                member,
                format: parse_format(&format),
                orphan_orchestrator: orphan_orchestrator.clone(),
                config_orchestrator: config_orchestrator.clone(),
                report_formatter: report_formatter.clone(),
                filesystem: filesystem.clone(),
                filesystem_seam: fs_seam.clone(),
                filter,
                fs_factory: Arc::new(|| {
                    let c = filesystem::root_filesystem_container::FilesystemContainer::new();
                    dispatcher::surface_check_action::FilesystemSeam {
                        io: c.io(),
                        workspace: c.workspace(),
                        parser: c.parser(),
                        aggregate: c.orchestrator(),
                    }
                }),
                orphan_factory: Arc::new(|config, fs, ws| {
                    orphan_rules::root_orphan_detector_container::OrphanContainer::new_with_config(
                        config, fs, ws,
                    )
                    .analyzer()
                }),
            },
        ),
        Command::Taxonomy {
            path,
            format,
            filter,
            member,
        } => run_layer_scan(
            &layer_scan_context,
            shared_role_rules::LAYER_TAXONOMY,
            path,
            format,
            filter,
            member,
        ),
        Command::Contract {
            path,
            format,
            filter,
            member,
        } => run_layer_scan(
            &layer_scan_context,
            shared_role_rules::LAYER_CONTRACT,
            path,
            format,
            filter,
            member,
        ),
        Command::Capabilities {
            path,
            format,
            filter,
            member,
        } => run_layer_scan(
            &layer_scan_context,
            shared_role_rules::LAYER_CAPABILITIES,
            path,
            format,
            filter,
            member,
        ),
        Command::Utility {
            path,
            format,
            filter,
            member,
        } => run_layer_scan(
            &layer_scan_context,
            shared_role_rules::LAYER_UTILITY,
            path,
            format,
            filter,
            member,
        ),
        Command::Agents {
            path,
            format,
            filter,
            member,
        } => run_layer_scan(
            &layer_scan_context,
            shared_role_rules::LAYER_AGENT,
            path,
            format,
            filter,
            member,
        ),
        Command::Surface {
            path,
            format,
            filter,
            member,
        } => run_layer_scan(
            &layer_scan_context,
            shared_role_rules::LAYER_SURFACES,
            path,
            format,
            filter,
            member,
        ),
        Command::External {
            path,
            format,
            filter,
        } => cli_commands::surface_scan_command::handle_external(
            cli_commands::surface_scan_command::ExternalCommandParams {
                path: Some(FilePath::new(path).unwrap_or_default()),
                format: parse_format(&format),
                external_lint: external_lint.clone(),
                report_formatter: report_formatter.clone(),
                filesystem: filesystem.clone(),
                filesystem_seam: fs_seam.clone(),
                config_parser: config_container.parser(),
                filter,
                ignored_paths: ignored_paths.clone(),
            },
        ),
        Command::Docs { path, format } => cli_commands::surface_scan_command::handle_docs(
            cli_commands::surface_scan_command::DocsCommandParams {
                path: Some(FilePath::new(path).unwrap_or_default()),
                format: parse_format(&format),
                doc_orchestrator:
                    doc_rules::root_doc_rules_container::RootDocRulesContainer::orchestrator(),
            },
        ),
        Command::Structure {
            path,
            format,
            filter,
        } => cli_commands::surface_scan_command::handle_structure(
            cli_commands::surface_scan_command::StructureCommandParams {
                path: Some(FilePath::new(path).unwrap_or_default()),
                format: parse_format(&format),
                structure_orchestrator: structure_orchestrator.clone(),
                report_formatter: report_formatter.clone(),
                filesystem: filesystem.clone(),
                filesystem_seam: fs_seam.clone(),
                filter,
                ignored_paths: ignored_paths.clone(),
            },
        ),
        Command::Ci { path, threshold } => cli_commands::surface_ci_command::handle_ci(
            cli_commands::surface_ci_command::CiCommandParams {
                code_analysis_linter: code_analysis_linter.clone(),
                import_orchestrator: import_orchestrator.clone(),
                naming_orchestrator: naming_orchestrator.clone(),
                config_orchestrator: config_orchestrator.clone(),
                orphan_orchestrator: orphan_orchestrator.clone(),
                filesystem: filesystem.clone(),
                filesystem_io: filesystem_io.clone(),
                path: Some(FilePath::new(path).unwrap_or_default()),
                threshold: Threshold::new(threshold),
            },
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
        Command::Update { check_only } => {
            cli_commands::surface_maintenance_command::handle_self_update(
                maintenance_orchestrator.clone(),
                check_only,
            )
        }
        Command::Adapters => {
            cli_commands::surface_plugin_command::handle_adapters(external_lint.clone())
        }
        Command::Init => cli_commands::surface_setup_command::handle_init(
            setup_orchestrator.clone(),
            filesystem_io.clone(),
        ),
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
        Command::InstallHook => {
            tracing::info!(
                target: "lint_arwaky::audit",
                event = "git_hook_change",
                action = "install-hook",
                phase = "started",
                "CLI git-hook action started"
            );
            let exe = std::env::current_exe()
                .map(|p| FilePath::new(p.to_string_lossy().to_string()).unwrap_or_default())
                .unwrap_or_default();
            match dispatcher::surface_git_action::collect_install_hook(
                git_orchestrator.clone(),
                &exe,
            ) {
                Ok(report) => {
                    tracing::info!(
                        target: "lint_arwaky::audit",
                        event = "git_hook_change",
                        action = "install-hook",
                        success = report.success,
                        "CLI git-hook action completed"
                    );
                    println!("{}", report.message);
                    if report.success {
                        shared_common::ExitCode::OK
                    } else {
                        shared_common::ExitCode::RUNTIME_ERROR
                    }
                }
                Err(e) => {
                    tracing::info!(
                        target: "lint_arwaky::audit",
                        event = "git_hook_change",
                        action = "install-hook",
                        success = false,
                        error = %e,
                        "CLI git-hook action failed"
                    );
                    eprintln!("{e}");
                    shared_common::ExitCode::RUNTIME_ERROR
                }
            }
        }
        Command::UninstallHook => {
            tracing::info!(
                target: "lint_arwaky::audit",
                event = "git_hook_change",
                action = "uninstall-hook",
                phase = "started",
                "CLI git-hook action started"
            );
            match dispatcher::surface_git_action::collect_uninstall_hook(git_orchestrator.clone()) {
                Ok(report) => {
                    tracing::info!(
                        target: "lint_arwaky::audit",
                        event = "git_hook_change",
                        action = "uninstall-hook",
                        success = report.success,
                        "CLI git-hook action completed"
                    );
                    println!("{}", report.message);
                    if report.success {
                        shared_common::ExitCode::OK
                    } else {
                        shared_common::ExitCode::RUNTIME_ERROR
                    }
                }
                Err(e) => {
                    tracing::info!(
                        target: "lint_arwaky::audit",
                        event = "git_hook_change",
                        action = "uninstall-hook",
                        success = false,
                        error = %e,
                        "CLI git-hook action failed"
                    );
                    eprintln!("{e}");
                    shared_common::ExitCode::RUNTIME_ERROR
                }
            }
        }
        Command::Version => {
            let report = dispatcher::surface_version_action::collect_version();
            println!("lint-arwaky {}", report.version);
            shared_common::ExitCode::OK
        }
        Command::Skill { sub } => match sub {
            SkillSubCommand::Read {
                name,
                with_references,
            } => cli_commands::surface_skill_command::handle_skill_read(&name, with_references),
            SkillSubCommand::List => cli_commands::surface_skill_command::handle_skill_list(),
        },
    };

    std::process::exit(exit_code.value() as i32);
}
