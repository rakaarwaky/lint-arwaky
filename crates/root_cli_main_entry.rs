// PURPOSE: main entry point for lint-arwaky-cli — parses args, initializes DI, dispatches commands
use std::env;
use std::process::ExitCode;
use std::sync::Arc;

use cli_commands::surface_check_action;
use cli_commands::surface_check_command;
use cli_commands::surface_fix_command;
use cli_commands::surface_plugin_command;
use cli_commands::surface_watch_command;
use cli_commands::CliContainer;
use code_analysis::{lint_path, CodeDuplicationAnalyzer};
use shared::cli_commands::taxonomy_cli_vo::{Cli, Commands};
use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::config_system::taxonomy_config_vo::default_aes_config;

pub struct CliMainEntry {}

struct CliLayerDetector {
    config: shared::config_system::taxonomy_config_vo::ArchitectureConfig,
    layer_map: shared::taxonomy_definition_vo::LayerMapVO,
}

impl ILayerDetectionAggregate for CliLayerDetector {
    fn detect_layer(&self, file_path: &str, _root_dir: &str) -> Option<String> {
        let filename = shared::common::utility_layer_detector::extract_filename(file_path);
        let layer = shared::common::utility_layer_detector::detect_layer_from_prefix(filename)?;
        let keys = shared::common::utility_layer_detector::collect_layer_keys(&self.layer_map);
        Some(shared::common::utility_layer_detector::resolve_specialized_layer(&layer, file_path, &keys))
    }
    fn get_layer_def(&self, layer: &str) -> Option<shared::taxonomy_definition_vo::LayerDefinition> {
        shared::common::utility_layer_detector::get_layer_def(layer, &self.config.layers).cloned()
    }
    fn get_orphan_entry_points(&self) -> Vec<String> {
        vec!["_container.rs".into(),"_container.py".into(),"_container.ts".into(),"_container.js".into(),
             "_entry.rs".into(),"_entry.py".into(),"_entry.ts".into(),"_entry.js".into(),
             "main.rs".into(),"lib.rs".into(),"main.py".into(),"main.ts".into(),"main.js".into(),
             "index.ts".into(),"index.js".into()]
    }
    fn config(&self) -> &shared::config_system::taxonomy_config_vo::ArchitectureConfig { &self.config }
}

fn make_check_context(
    container: &CliContainer,
    layer_detector: &Arc<dyn ILayerDetectionAggregate>,
) -> surface_check_command::CheckContext {
    surface_check_command::CheckContext {
        code_analysis_linter: container.code_analysis_linter.clone(),
        import_orchestrator: container.import_orchestrator.clone(),
        naming_orchestrator: container.naming_orchestrator.clone(),
        external_lint: container.external_lint.clone(),
        role_orchestrator: container.role_orchestrator.clone(),
        orphan_orchestrator: container.orphan_orchestrator.clone(),
        layer_detector: layer_detector.clone(),
        language_detector: Arc::new(cli_commands::capabilities_language_detector::CliLanguageDetector::new()),
    }
}

fn make_layer_detector() -> Arc<dyn ILayerDetectionAggregate> {
    let aes_config = default_aes_config();
    let (merged_layers, _) = shared::config_system::utility_config_merger::merge_config(&aes_config);
    let mut config = aes_config;
    config.layers = merged_layers;
    let layer_map = shared::taxonomy_definition_vo::LayerMapVO::new(config.layers.clone());
    Arc::new(CliLayerDetector { config, layer_map })
}

fn main() -> ExitCode {
    let container = CliContainer::new_default();
    let layer_detector = make_layer_detector();

    let ext_lint_clone = container.external_lint.clone();
    let layer_det_clone = layer_detector.clone();
    let factory: surface_check_command::OrchestratorFactory = Arc::new(move |config| {
        let import_container =
            import_rules::root_import_rules_container::ImportContainer::new_with_config(config.clone());
        let naming_container = naming_rules::root_naming_rules_container::NamingContainer::default();
        let role_container =
            role_rules::root_role_rules_container::RoleContainer::new_with_config(config.clone());
        let arch_linter =
            code_analysis::root_code_analysis_container::CodeAnalysisContainer::new().code_analysis_linter();
        let orphan_container =
            orphan_detector::root_orphan_detector_container::OrphanContainer::new();

        surface_check_command::CheckContext {
            code_analysis_linter: arch_linter,
            import_orchestrator: import_container.orchestrator(),
            naming_orchestrator: naming_container.orchestrator(),
            external_lint: ext_lint_clone.clone(),
            role_orchestrator: role_container.orchestrator(),
            orphan_orchestrator: orphan_container.analyzer(),
            layer_detector: layer_det_clone.clone(),
            language_detector: Arc::new(cli_commands::capabilities_language_detector::CliLanguageDetector::new()),
        }
    });

    let fix_linter = container.code_analysis_linter.clone();
    let fix_orchestrator_factory: Arc<
        dyn Fn(bool) -> Arc<dyn shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate> + Send + Sync,
    > = Arc::new(move |dry_run| {
        auto_fix::root_auto_fix_container::AutoFixContainer::new(fix_linter.clone()).orchestrator(dry_run)
    });

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
        Commands::Check { path, git_diff, format } => surface_check_action::handle_check(
            path, git_diff, make_check_context(&container, &layer_detector),
            filter, Some(container.git_aggregate.clone()),
            shared::config_system::taxonomy_config_vo::ArchitectureConfig::default(), format,
        ),
        Commands::Scan { path, member, format } => surface_check_action::handle_scan(
            path, make_check_context(&container, &layer_detector),
            Some(container.multi_project_orchestrator.clone()),
            factory, filter, member, format,
        ),
        Commands::Fix { path, dry_run } => surface_fix_command::handle_fix(
            path, dry_run, container.code_analysis_linter.clone(), fix_orchestrator_factory,
        ),
        Commands::Ci { path, threshold } =>
            surface_check_action::handle_ci(container.code_analysis_linter.clone(), path, threshold),
        Commands::Doctor => {
            let maintenance_container = maintenance::root_maintenance_container::MaintenanceContainer::new();
            let orchestrator = maintenance_container.orchestrator();
            let rt = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
                Ok(r) => r, Err(_) => return ExitCode::FAILURE,
            };
            rt.block_on(cli_commands::surface_maintenance_command::handle_doctor(orchestrator))
        }
        Commands::Version => {
            let verbose = raw_args.iter().any(|a| a == "--verbose" || a == "-v");
            let ver = env!("CARGO_PKG_VERSION");
            if verbose {
                println!("Lint Arwaky v{}", ver);
                let commit = match std::process::Command::new("git").args(["rev-parse", "HEAD"]).output()
                    .ok().filter(|o| o.status.success()).map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                { Some(c) => c, None => "unknown".to_string() };
                println!("  Commit:    {}", commit);
                println!("  Rustc:     {}", default_rustc_version());
                println!("  License:   MIT");
            } else {
                println!("Lint Arwaky v{ver} (AES Semantic Builder)");
            }
            ExitCode::SUCCESS
        }
        Commands::Adapters => surface_plugin_command::handle_adapters(container.external_lint.clone()),
        Commands::Orphan { path } => {
            let surface = surface_check_command::CheckCommandsSurface::new(make_check_context(&container, &layer_detector));
            surface.check_orphan_single_file(&path);
            ExitCode::SUCCESS
        }
        Commands::Security { path } => {
            let maintenance_container = maintenance::root_maintenance_container::MaintenanceContainer::new();
            let orchestrator = maintenance_container.orchestrator();
            let rt = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
                Ok(r) => r, Err(_) => return ExitCode::FAILURE,
            };
            rt.block_on(cli_commands::surface_maintenance_command::handle_security(orchestrator, path))
        }
        Commands::Duplicates { path } => {
            let dup_analyzer = CodeDuplicationAnalyzer::new();
            let violations = dup_analyzer.handle_duplicates(path);
            if violations.is_empty() {
                println!("No duplicate code blocks detected.");
            } else {
                for v in &violations { println!("{}", <String as From<_>>::from(v.clone())); }
            }
            ExitCode::SUCCESS
        }
        Commands::Dependencies { path } => {
            let maintenance_container = maintenance::root_maintenance_container::MaintenanceContainer::new();
            let orchestrator = maintenance_container.orchestrator();
            let rt = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
                Ok(r) => r, Err(_) => return ExitCode::FAILURE,
            };
            rt.block_on(cli_commands::surface_maintenance_command::handle_dependencies(orchestrator, path))
        }
        Commands::Watch { path } => {
            let fwatch_container = file_watch::FileWatchContainer::new();
            let watch_agg: Arc<dyn shared::file_watch::contract_watch_aggregate::IWatchAggregate> =
                fwatch_container.orchestrator(container.code_analysis_linter.clone());
            surface_watch_command::handle_watch(watch_agg, path)
        }
        Commands::InstallHook => {
            let git_hooks_container = git_hooks::root_git_hooks_container::GitContainer::new_default();
            let aggregate = git_hooks_container.aggregate();
            let rt = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
                Ok(r) => r, Err(_) => return ExitCode::FAILURE,
            };
            let exe_path = default_file_path("lint-arwaky".to_string());
            match rt.block_on(aggregate.install_hook(&exe_path)) {
                Ok(status) if status.value => { println!("Installed git pre-commit hook successfully"); ExitCode::SUCCESS }
                Ok(_) => { println!("Not a git repository, hook installation skipped"); ExitCode::SUCCESS }
                Err(e) => { eprintln!("Failed to install pre-commit hook: {:?}", e); ExitCode::FAILURE }
            }
        }
        Commands::UninstallHook => {
            let git_hooks_container = git_hooks::root_git_hooks_container::GitContainer::new_default();
            let aggregate = git_hooks_container.aggregate();
            let rt = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
                Ok(r) => r, Err(_) => return ExitCode::FAILURE,
            };
            match rt.block_on(aggregate.uninstall_hook()) {
                Ok(status) if status.value => { println!("Removed git pre-commit hook successfully"); ExitCode::SUCCESS }
                Ok(_) => { println!("Not a git repository, hook removal skipped"); ExitCode::SUCCESS }
                Err(e) => { eprintln!("Failed to remove pre-commit hook: {:?}", e); ExitCode::FAILURE }
            }
        }
        Commands::Init { global } => {
            let setup_container = project_setup::root_project_setup_container::SetupContainer::new();
            cli_commands::surface_setup_command::handle_init(setup_container.aggregate(), global)
        }
        Commands::Install { sudo } => {
            let setup_container = project_setup::root_project_setup_container::SetupContainer::new();
            let setup_orchestrator = setup_container.aggregate();
            let rt = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
                Ok(r) => r, Err(_) => return ExitCode::FAILURE,
            };
            rt.block_on(cli_commands::surface_setup_command::handle_install(setup_orchestrator, sudo))
        }
        Commands::McpConfig { client } => cli_commands::surface_setup_command::handle_mcp_config(&client),
        Commands::ConfigShow => {
            let config_container = config_system::root_config_system_container::ConfigContainer::new();
            let config_orchestrator = config_container.orchestrator();
            let rt = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
                Ok(r) => r, Err(_) => return ExitCode::FAILURE,
            };
            rt.block_on(cli_commands::surface_config_command::handle_config_show(config_orchestrator))
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
    let mut critical = Vec::new(); let mut high = Vec::new(); let mut medium = Vec::new(); let mut low = Vec::new();
    for r in &results { match r.severity { Severity::CRITICAL => critical.push(r), Severity::HIGH => high.push(r), Severity::MEDIUM => medium.push(r), Severity::LOW => low.push(r), _ => medium.push(r) } }
    for (sev, items) in [("CRITICAL", &critical), ("HIGH", &high), ("MEDIUM", &medium), ("LOW", &low)] {
        if items.is_empty() { continue; }
        lines.push(format!("  [{}] {} violations", sev, items.len()));
        lines.push("-".repeat(60));
        for r in items.iter() { lines.push(format!("  [{}] {}", r.code, r.file.value)); for msg_line in r.message.value.lines() { lines.push(format!("    {}", msg_line)); } }
        lines.push("".to_string());
    }
    let total = results.len();
    let mut per_code: BTreeMap<String, usize> = BTreeMap::new();
    for r in &results { *per_code.entry(r.code.to_string()).or_insert(0) += 1; }
    lines.push("=".repeat(60));
    lines.push(format!("  Total AES Violations: {}", total));
    lines.push(format!("  Total Category AES Violations: {}", per_code.len()));
    if !per_code.is_empty() { lines.push("-".repeat(60)); for (code, count) in &per_code { lines.push(format!("  {}: {}", code, count)); } }
    lines.push("".to_string());
    if total == 0 { lines.push("  Status: PASS - No AES violations detected".to_string()); } else { lines.push("  Status: FAIL - AES violations detected".to_string()); }
    lines.push("=".repeat(60));
    println!("Lint Arwaky v{} (AES Self-Lint)", env!("CARGO_PKG_VERSION"));
    println!("Scanning: {}", project_root);
    println!();
    println!("{}", lines.join("\n"));
    if total > 0 { ExitCode::from(1) } else { ExitCode::SUCCESS }
}

fn default_file_path(s: String) -> shared::common::taxonomy_path_vo::FilePath {
    shared::common::taxonomy_path_vo::FilePath::new(s).unwrap_or_default()
}

fn default_rustc_version() -> &'static str {
    option_env!("VERGEN_RUSTC_SEMVER").or_else(|| option_env!("RUSTC_VERSION")).unwrap_or("stable")
}
