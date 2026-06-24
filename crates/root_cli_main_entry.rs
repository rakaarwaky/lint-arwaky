// PURPOSE: main entry point for lint-arwaky-cli — parses args, initializes DI, dispatches commands
use std::env;
use std::process::ExitCode;
use std::sync::Arc;

use cli_commands::surface_check_command;
use cli_commands::surface_fix_command;
use cli_commands::surface_plugin_command;
use cli_commands::surface_watch_command;
use code_analysis::agent_code_analysis_orchestrator::init_global_checker;
use code_analysis::{has_critical, lint_path, CodeDuplicationAnalyzer};
use import_rules::root_import_rules_container::ImportContainer;
use role_rules::root_role_rules_container::RoleContainer;
use shared::cli_commands::taxonomy_cli_vo::{Cli, Commands};
use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;

pub struct CliMainEntry {}

fn main() -> ExitCode {
    let source_parsing_container =
        source_parsing::root_source_parsing_container::SourceParsingContainer::new();
    let source_parser = source_parsing_container.source_parser();

    let import_container = ImportContainer::new(source_parser.clone());
    let analyzer = import_container.analyzer();
    let checker_container =
        code_analysis::root_code_analysis_container::CodeAnalysisCheckerContainer::new(
            analyzer.clone(),
        );
    init_global_checker(Arc::new(checker_container));

    let arch_linter = code_analysis::root_code_analysis_container::CodeAnalysisContainer::new()
        .architecture_linter();
    let import_orchestrator = import_container.orchestrator();

    let role_container = RoleContainer::new();
    let role_orchestrator = role_container.orchestrator();

    let naming_container = naming_rules::root_naming_rules_container::NamingContainer::new(
        import_container.analyzer(),
    );
    let naming_orchestrator = naming_container.orchestrator();

    let auto_fix_container =
        auto_fix::root_auto_fix_container::AutoFixContainer::new(arch_linter.clone());

    let external_lint_container =
        external_lint::root_external_lint_container::ExternalLintContainer::new(
            source_parsing_container.path_normalization(),
        );
    let external_lint_aggregate = external_lint_container.aggregate();

    let config_container =
        config_system::root_config_system_container::ConfigContainer::new();
    let orphan_container =
        orphan_detector::root_orphan_detector_container::OrphanContainer::new();
    let git_container =
        git_hooks::root_git_hooks_container::GitContainer::new_default();

    let layer_detector = orphan_container.layer_detector();
    let git_aggregate = git_container.aggregate();
    let multi_project_orchestrator = config_container.multi_project_orchestrator();

    let external_lint_aggregate_clone = external_lint_aggregate.clone();
    let source_parser_clone = source_parser.clone();
    let layer_detector_clone = layer_detector.clone();
    let factory: surface_check_command::OrchestratorFactory = Arc::new(move |config| {
        let import_container =
            ImportContainer::new_with_config(config.clone(), source_parser_clone.clone());
        let naming_container = naming_rules::root_naming_rules_container::NamingContainer::new(
            import_container.analyzer(),
        );
        let role_container =
            role_rules::root_role_rules_container::RoleContainer::new_with_config(config.clone());
        let analyzer = import_container.analyzer();
        let arch_linter =
            code_analysis::root_code_analysis_container::CodeAnalysisContainer::new_with_analyzer(
                analyzer,
            )
            .architecture_linter();

        let source_parsing_container =
            source_parsing::root_source_parsing_container::SourceParsingContainer::new();
        let orphan_container =
            orphan_detector::root_orphan_detector_container::OrphanContainer::new();

        surface_check_command::CheckContext {
            arch_linter,
            import_orchestrator: import_container.orchestrator(),
            naming_orchestrator: naming_container.orchestrator(),
            external_lint: external_lint_aggregate_clone.clone(),
            role_orchestrator: role_container.orchestrator(),
            scanner_provider: source_parsing_container.scanner_provider(),
            orphan_orchestrator: orphan_container.analyzer(),
            layer_detector: layer_detector_clone.clone(),
            language_detector: source_parsing_container.language_detector(),
        }
    });

    let fix_container = auto_fix_container.clone();
    let fix_orchestrator_factory: Arc<
        dyn Fn(
                bool,
            )
                -> Arc<dyn shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate>
            + Send
            + Sync,
    > = Arc::new(move |dry_run| fix_container.orchestrator(dry_run));

    let raw_args: Vec<String> = env::args().collect();
    if raw_args.len() <= 1 {
        return run_default_check(".");
    }

    let cli = match <Cli as clap::Parser>::try_parse_from(&raw_args) {
        Ok(c) => c,
        Err(e) => e.exit(),
    };

    let filter = cli.filter.clone();
    match cli.command {
        Commands::Check { path, git_diff } => surface_check_command::handle_check(
            path,
            git_diff,
            surface_check_command::CheckContext {
                arch_linter: arch_linter.clone(),
                import_orchestrator: import_orchestrator.clone(),
                naming_orchestrator: naming_orchestrator.clone(),
                external_lint: external_lint_aggregate.clone(),
                role_orchestrator: role_orchestrator.clone(),
                scanner_provider: source_parsing_container.scanner_provider(),
                orphan_orchestrator: orphan_container.analyzer(),
                layer_detector: layer_detector.clone(),
                language_detector: source_parsing_container.language_detector(),
            },
            filter,
            Some(git_aggregate.clone()),
            shared::config_system::taxonomy_config_vo::ArchitectureConfig::default(),
        ),
        Commands::Scan { path } => surface_check_command::handle_scan(
            path,
            surface_check_command::CheckContext {
                arch_linter,
                import_orchestrator,
                naming_orchestrator: naming_orchestrator.clone(),
                external_lint: external_lint_aggregate.clone(),
                role_orchestrator: role_orchestrator.clone(),
                scanner_provider: source_parsing_container.scanner_provider(),
                orphan_orchestrator: orphan_container.analyzer(),
                layer_detector: layer_detector.clone(),
                language_detector: source_parsing_container.language_detector(),
            },
            Some(multi_project_orchestrator.clone()),
            factory,
            filter,
        ),
        Commands::Fix { path, dry_run } => surface_fix_command::handle_fix(
            path,
            dry_run,
            arch_linter.clone(),
            fix_orchestrator_factory,
        ),
        Commands::Ci { path, threshold } => {
            surface_check_command::handle_ci(arch_linter.clone(), path, threshold)
        }
        Commands::Doctor => {
            let maintenance_container =
                maintenance::root_maintenance_container::MaintenanceContainer::new();
            let orchestrator = maintenance_container.orchestrator();
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(cli_commands::surface_maintenance_command::handle_doctor(
                orchestrator,
            ))
        }
        Commands::Version => {
            let verbose = raw_args.iter().any(|a| a == "--verbose" || a == "-v");
            let ver = env!("CARGO_PKG_VERSION");
            if verbose {
                println!("Lint Arwaky v{}", ver);
                let commit = std::process::Command::new("git")
                    .args(["rev-parse", "HEAD"])
                    .output()
                    .ok()
                    .filter(|o| o.status.success())
                    .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                println!("  Commit:    {}", commit);
                let rustc = option_env!("VERGEN_RUSTC_SEMVER")
                    .or(option_env!("RUSTC_VERSION"))
                    .unwrap_or("stable");
                println!("  Rustc:     {}", rustc);
                println!("  License:   MIT");
            } else {
                println!("Lint Arwaky v{ver} (AES Semantic Builder)");
            }
            ExitCode::SUCCESS
        }
        Commands::Adapters => surface_plugin_command::handle_adapters(external_lint_aggregate),
        Commands::Orphan { path } => {
            let surface = surface_check_command::CheckCommandsSurface::new(
                surface_check_command::CheckContext {
                    arch_linter: arch_linter.clone(),
                    import_orchestrator: import_orchestrator.clone(),
                    naming_orchestrator: naming_orchestrator.clone(),
                    external_lint: external_lint_aggregate.clone(),
                    role_orchestrator: role_orchestrator.clone(),
                    scanner_provider: source_parsing_container.scanner_provider(),
                    orphan_orchestrator: orphan_container.analyzer(),
                    layer_detector: layer_detector.clone(),
                    language_detector: source_parsing_container.language_detector(),
                },
            );
            surface.check_orphan_single_file(&path);
            ExitCode::SUCCESS
        }
        Commands::Security { path } => {
            let maintenance_container =
                maintenance::root_maintenance_container::MaintenanceContainer::new();
            let orchestrator = maintenance_container.orchestrator();
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(cli_commands::surface_maintenance_command::handle_security(
                orchestrator,
                path,
            ))
        }
        Commands::Duplicates { path } => {
            let dup_analyzer = CodeDuplicationAnalyzer::new();
            let violations = dup_analyzer.handle_duplicates(path, analyzer.fs());
            if violations.is_empty() {
                println!("No duplicate code blocks detected.");
            } else {
                for v in &violations {
                    let s: String = v.clone().into();
                    println!("{}", s);
                }
            }
            ExitCode::SUCCESS
        }
        Commands::Dependencies { path } => {
            let maintenance_container =
                maintenance::root_maintenance_container::MaintenanceContainer::new();
            let orchestrator = maintenance_container.orchestrator();
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(
                cli_commands::surface_maintenance_command::handle_dependencies(orchestrator, path),
            )
        }
        Commands::Watch { path } => {
            let container = file_watch::FileWatchContainer::new();
            let watch_agg: Arc<dyn shared::file_watch::contract_watch_aggregate::IWatchAggregate> =
                container.orchestrator(arch_linter.clone());
            surface_watch_command::handle_watch(watch_agg, path)
        }
        Commands::InstallHook => {
            let container = git_hooks::root_git_hooks_container::GitContainer::new_default();
            let aggregate = container.aggregate();
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let exe_path =
                shared::source_parsing::taxonomy_path_vo::FilePath::new("lint-arwaky".to_string())
                    .unwrap_or_default();
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
                    ExitCode::FAILURE
                }
            }
        }
        Commands::UninstallHook => {
            let container = git_hooks::root_git_hooks_container::GitContainer::new_default();
            let aggregate = container.aggregate();
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
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
        Commands::Init { global } => {
            let setup_container =
                project_setup::root_project_setup_container::SetupContainer::new();
            let setup_orchestrator = setup_container.aggregate();
            cli_commands::surface_setup_command::handle_init(setup_orchestrator, global)
        }
        Commands::Install { sudo } => {
            let setup_container =
                project_setup::root_project_setup_container::SetupContainer::new();
            let setup_orchestrator = setup_container.aggregate();
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
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
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(cli_commands::surface_config_command::handle_config_show(
                config_orchestrator,
            ))
        }
    }
}

fn run_default_check(project_root: &str) -> ExitCode {
    use shared::cli_commands::taxonomy_severity_vo::Severity;
    use std::collections::BTreeMap;

    let results = lint_path(project_root);

    let mut lines: Vec<String> = Vec::new();
    lines.push("=".repeat(60));
    lines.push("  AES Architecture Compliance Report (Self-Lint)".to_string());
    lines.push("=".repeat(60));
    lines.push(format!("  Project: {}", project_root));
    lines.push(format!("  Files scanned: {}", results.len()));
    lines.push("=".repeat(60));
    lines.push("".to_string());

    let mut critical = Vec::new();
    let mut high = Vec::new();
    let mut medium = Vec::new();
    let mut low = Vec::new();
    for r in &results {
        match r.severity {
            Severity::CRITICAL => critical.push(r),
            Severity::HIGH => high.push(r),
            Severity::MEDIUM => medium.push(r),
            Severity::LOW => low.push(r),
            _ => medium.push(r),
        }
    }
    for (sev, items) in [
        ("CRITICAL", &critical),
        ("HIGH", &high),
        ("MEDIUM", &medium),
        ("LOW", &low),
    ] {
        if items.is_empty() {
            continue;
        }
        lines.push(format!("  [{}] {} violations", sev, items.len()));
        lines.push("-".repeat(60));
        for r in items.iter() {
            lines.push(format!("  [{}] {}", r.code, r.file.value));
            for msg_line in r.message.value.lines() {
                lines.push(format!("    {}", msg_line));
            }
        }
        lines.push("".to_string());
    }

    let total = results.len();
    let mut per_code: BTreeMap<String, usize> = BTreeMap::new();
    for r in &results {
        *per_code.entry(r.code.to_string()).or_insert(0) += 1;
    }
    lines.push("=".repeat(60));
    lines.push(format!("  Total AES Violations: {}", total));
    lines.push(format!(
        "  Total Category AES Violations: {}",
        per_code.len()
    ));
    if !per_code.is_empty() {
        lines.push("-".repeat(60));
        for (code, count) in &per_code {
            lines.push(format!("  {}: {}", code, count));
        }
    }
    lines.push("".to_string());
    if total == 0 {
        lines.push("  Status: PASS - No AES violations detected".to_string());
    } else {
        lines.push("  Status: FAIL - AES violations detected".to_string());
    }
    lines.push("=".repeat(60));
    let report = lines.join("\n");

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
