// PURPOSE: main entry point for lint-arwaky-cli — parses args, initializes DI, dispatches commands
use std::env;
use std::process::ExitCode;
use std::sync::Arc;

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
use code_analysis::agent_code_analysis_orchestrator::init_global_checker;
use code_analysis::{has_critical, lint_path, CodeDuplicationAnalyzer};
use import_rules::root_import_rules_container::ImportContainer;
use role_rules::root_role_rules_container::RoleContainer;
use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;

pub struct CliMainEntry {}

fn main() -> ExitCode {
    // Inline CLI composition — create exactly what CLI needs
    let source_parsing_container =
        source_parsing::root_source_parsing_container::SourceParsingContainer::new();
    let path_norm = source_parsing_container.path_normalization();
    let source_parser = source_parsing_container.source_parser();

    let import_container = ImportContainer::new(source_parser.clone());
    let analyzer = import_container.analyzer();
    let checker_container =
        code_analysis::root_code_analysis_container::CodeAnalysisCheckerContainer::new(analyzer);
    init_global_checker(Arc::new(checker_container));

    let arch_linter = code_analysis::root_code_analysis_container::CodeAnalysisContainer::new()
        .architecture_linter();
    let import_orchestrator = import_container.orchestrator();

    let role_container = RoleContainer::new();
    let role_orchestrator = role_container.orchestrator();

    let naming_container = naming_rules::root_naming_rules_container::NamingContainer::new();
    let naming_orchestrator = naming_container.orchestrator();

    let auto_fix_container =
        auto_fix::root_auto_fix_container::AutoFixContainer::new(arch_linter.clone());

    let external_lint_container =
        external_lint::root_external_lint_container::ExternalLintContainer::new(path_norm);
    let external_lint_aggregate = external_lint_container.aggregate();

    let external_lint_aggregate_clone = external_lint_aggregate.clone();
    let source_parser_clone = source_parser.clone();
    let factory: surface_check_command::OrchestratorFactory = Arc::new(move |config| {
        let import_container = ImportContainer::new_with_config(config.clone(), source_parser_clone.clone());
        let naming_container =
            naming_rules::root_naming_rules_container::NamingContainer::new_with_config(
                config.clone(),
            );
        let role_container =
            role_rules::root_role_rules_container::RoleContainer::new_with_config(config.clone());
        let analyzer = import_container.analyzer();
        let arch_linter =
            code_analysis::root_code_analysis_container::CodeAnalysisContainer::new_with_analyzer(
                analyzer,
            )
            .architecture_linter();

        surface_check_command::CheckContext {
            arch_linter,
            import_orchestrator: import_container.orchestrator(),
            naming_orchestrator: naming_container.orchestrator(),
            external_lint: external_lint_aggregate_clone.clone(),
            role_orchestrator: role_container.orchestrator(),
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
            },
            filter,
        ),
        Commands::Scan { path } => surface_check_command::handle_scan(
            path,
            arch_linter,
            import_orchestrator,
            naming_orchestrator.clone(),
            external_lint_aggregate.clone(),
            role_orchestrator.clone(),
            factory,
            filter,
        ),
        Commands::Fix { path, dry_run } => {
            surface_fix_command::handle_fix(path, dry_run, fix_orchestrator_factory)
        }
        Commands::Report {
            path,
            output_format,
        } => surface_report_command::handle_report(path, output_format),
        Commands::Ci { path, threshold } => {
            surface_dev_command::handle_ci(arch_linter.clone(), path, threshold)
        }
        Commands::Version => {
            let verbose = raw_args.iter().any(|a| a == "--verbose" || a == "-v");
            surface_bootstrap_command::handle_version(verbose)
        }
        Commands::Adapters => surface_plugin_command::handle_adapters(external_lint_aggregate),
        Commands::Config { command } => surface_config_command::handle_config(command),
        Commands::GitDiff { base } => {
            surface_git_command::handle_git_diff(arch_linter.clone(), base)
        }
        Commands::MultiProject { paths } => {
            surface_multi_command::handle_multi_project(arch_linter.clone(), paths)
        }
        Commands::Orphan { path } => {
            let surface = surface_check_command::CheckCommandsSurface::new(
                external_lint_aggregate.clone(),
                arch_linter.clone(),
                import_orchestrator.clone(),
                naming_orchestrator.clone(),
                role_orchestrator.clone(),
            );
            surface.check_orphan_single_file(&path);
            ExitCode::SUCCESS
        }
        Commands::Security { path } => surface_maintenance_command::handle_security(path),
        Commands::Duplicates { path } => {
            let analyzer = CodeDuplicationAnalyzer::new();
            analyzer.handle_duplicates(path)
        }
        Commands::Dependencies { path } => surface_maintenance_command::handle_dependencies(path),
        Commands::Setup { command } => surface_setup_command::handle_setup(command),
        Commands::Cancel { job_id } => surface_map_command::handle_cancel(job_id),
        Commands::Diff { path1, path2 } => {
            surface_map_command::handle_diff(arch_linter.clone(), path1, path2)
        }
        Commands::Import { config_file } => surface_map_command::handle_import(config_file),
        Commands::Export { format } => {
            surface_map_command::handle_export(arch_linter.clone(), format)
        }
        Commands::Watch { path } => surface_watch_command::handle_watch(arch_linter.clone(), path),
        Commands::Suggest { path, ai: _ } => {
            surface_map_command::handle_suggest(arch_linter.clone(), path)
        }
        Commands::InstallHook => surface_git_command::handle_install_hook(),
        Commands::UninstallHook => surface_git_command::handle_uninstall_hook(),
        Commands::VscodeGraph { path } => {
            let p = path.unwrap_or_else(|| ".".to_string());
            match vscode_extension::handle_vscode_graph(&p) {
                Ok(json) => {
                    println!("{}", json);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Error generating graph: {}", e);
                    ExitCode::from(1)
                }
            }
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
