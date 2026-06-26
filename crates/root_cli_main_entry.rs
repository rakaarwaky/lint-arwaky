// PURPOSE: main entry point for lint-arwaky-cli — parses args, initializes DI, dispatches commands
//
// This is the heart of the CLI binary. It:
//   1. Creates all DI containers (wiring every feature crate together)
//   2. Parses CLI arguments via clap
//   3. Dispatches to the correct surface command handler
//
// The DI wiring is manual — no framework. Each feature crate has its own
// `root_*_container.rs` that creates concrete implementations and exposes
// them as `Arc<dyn Trait>` factory methods. This file composes them all.
use std::env;
use std::process::ExitCode;
use std::sync::Arc;

use cli_commands::surface_check_command;
use cli_commands::surface_check_action;
use cli_commands::surface_fix_command;
use cli_commands::surface_plugin_command;
use cli_commands::surface_watch_command;
use code_analysis::agent_code_analysis_orchestrator::init_global_checker;
use code_analysis::{has_critical, lint_path, CodeDuplicationAnalyzer};
use import_rules::capabilities_layer_detection_analyzer::LayerDetectionAnalyzer;
use import_rules::infrastructure_filesystem_adapter::OSFileSystemAdapter;
use import_rules::root_import_rules_container::ImportContainer;
use import_rules::root_import_rules_container::NullSourceParser;
use role_rules::root_role_rules_container::RoleContainer;
use shared::cli_commands::taxonomy_cli_vo::{Cli, Commands};
use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::common::contract_parser_port::ISourceParserPort;
use shared::common::contract_system_port::IFileSystemPort;
use shared::config_system::taxonomy_config_vo::default_aes_config;

pub struct CliMainEntry {}

fn main() -> ExitCode {
    // ── Phase 1: Initialize all DI containers ──────────────────────────
    // Each container wires its feature's dependencies (checkers, adapters,
    // orchestrators) via manual Arc<dyn Trait> injection. Order matters:
    // some containers depend on types from others (e.g., NamingContainer
    // reuses the ImportContainer's LayerDetectionAnalyzer).

    // Import rules container is created first because its analyzer is
    // needed by several other containers (checker, naming, code-analysis).
    let import_container = ImportContainer::new_default();
    let analyzer = import_container.analyzer();

    // Register the layer-detection analyzer as a global singleton so that
    // all code-analysis checks can identify which AES layer a file belongs to.
    let checker_container =
        code_analysis::root_code_analysis_container::CodeAnalysisCheckerContainer::new(
            analyzer.clone(),
        );
    init_global_checker(Arc::new(checker_container));

    // Create the main code-analysis linter — this wraps all quality checks
    // (file lines, bypass suppression, mandatory definitions, todo detection).
    let arch_linter = code_analysis::root_code_analysis_container::CodeAnalysisContainer::new()
        .code_analysis_linter();
    let import_orchestrator = import_container.orchestrator();

    // Role rules: checks that each layer (taxonomy, contract, capabilities,
    // etc.) uses the correct file suffixes and dependency directions (AES401-406).
    let role_container = RoleContainer::new();
    let role_orchestrator = role_container.orchestrator();

    // Naming rules: checks suffix/prefix conventions (AES101-102).
    // Depends on the layer-detection analyzer from import_container.
    let naming_container = naming_rules::root_naming_rules_container::NamingContainer::new(
        import_container.analyzer(),
    );
    let naming_orchestrator = naming_container.orchestrator();

    // Auto-fix: applies safe fixes (removing unused/forbidden imports).
    // Depends on the code-analysis linter to collect violations first.
    let auto_fix_container =
        auto_fix::root_auto_fix_container::AutoFixContainer::new(arch_linter.clone());

    // External linters: delegates to Clippy, Ruff, ESLint, etc.
    // These run as subprocesses and their output is parsed into LintResult.
    let external_lint_container =
        external_lint::root_external_lint_container::ExternalLintContainer::new_default();
    let external_lint_aggregate = external_lint_container.aggregate();

    // Config, orphan-detector, git-hooks: standalone containers with no
    // cross-crate dependencies (they only depend on shared types).
    let config_container = config_system::root_config_system_container::ConfigContainer::new();
    let orphan_container = orphan_detector::root_orphan_detector_container::OrphanContainer::new();
    let git_container = git_hooks::root_git_hooks_container::GitContainer::new_default();

    // ── Phase 2: Build shared infrastructure ────────────────────────────
    // These are the plumbing types that multiple feature crates depend on:
    // filesystem access, source code parsing, and architectural layer detection.
    let aes_config = default_aes_config();
    let fs: Arc<dyn IFileSystemPort> = Arc::new(OSFileSystemAdapter::new());
    let parser: Arc<dyn ISourceParserPort> = Arc::new(NullSourceParser);
    let layer_detector: Arc<dyn ILayerDetectionAggregate> =
        Arc::new(LayerDetectionAnalyzer::new(aes_config, fs, parser));
    let git_aggregate = git_container.aggregate();
    let multi_project_orchestrator = config_container.multi_project_orchestrator();

    // ── Phase 3: Create per-project factory (for `scan` command) ─────
    // The `scan` command discovers workspace members (Cargo.toml members,
    // pyproject.toml modules, package.json workspaces) and runs all linters
    // on each member independently. This factory closure creates a fresh
    // CheckContext for each member project with per-project configuration.
    let external_lint_aggregate_clone = external_lint_aggregate.clone();
    let layer_detector_clone = layer_detector.clone();
    let factory: surface_check_command::OrchestratorFactory = Arc::new(move |config| {
        let import_container =
            ImportContainer::new_with_config(config.clone(), Arc::new(NullSourceParser));
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
            .code_analysis_linter();

        let orphan_container =
            orphan_detector::root_orphan_detector_container::OrphanContainer::new();

        surface_check_command::CheckContext {
            code_analysis_linter: arch_linter,
            import_orchestrator: import_container.orchestrator(),
            naming_orchestrator: naming_container.orchestrator(),
            external_lint: external_lint_aggregate_clone.clone(),
            role_orchestrator: role_container.orchestrator(),
            scanner_provider: Arc::new(
                shared::common::infrastructure_file_collector_provider::FileCollectorProvider::new(
                ),
            ),
            orphan_orchestrator: orphan_container.analyzer(),
            layer_detector: layer_detector_clone.clone(),
            language_detector: Arc::new(
                cli_commands::infrastructure_language_detector::CliLanguageDetector::new(),
            ),
        }
    });

    // ── Phase 4: Create fix orchestrator factory ──────────────────────
    // Unlike the scan factory (per-project), this factory creates fix
    // orchestrators with a dry_run toggle (preview vs apply).
    let fix_container = auto_fix_container.clone();
    let fix_orchestrator_factory: Arc<
        dyn Fn(
                bool,
            )
                -> Arc<dyn shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate>
            + Send
            + Sync,
    > = Arc::new(move |dry_run| fix_container.orchestrator(dry_run));

    // ── Phase 5: Parse CLI arguments ──────────────────────────────────
    // If no subcommand is given, default to `check .` (self-lint).
    // This is a convenience: running `lint-arwaky-cli` without args
    // audits the current project under the AES ruleset.
    let raw_args: Vec<String> = env::args().collect();
    if raw_args.len() <= 1 {
        return run_default_check(".");
    }

    let cli = match <Cli as clap::Parser>::try_parse_from(&raw_args) {
        Ok(c) => c,
        Err(e) => e.exit(),
    };

    // ── Phase 6: Dispatch to command handlers ─────────────────────────
    // Each command builds a CheckContext (all dependencies) and passes it
    // to the surface handler. CheckContext is constructed inline to avoid
    // moving/cloning expensive resources where possible.
    let filter = cli.filter.clone();
    match cli.command {
        // CHECK: Full lint pipeline on a single project directory.
        // Supports --git-diff (only scan changed files) and --code (filter by code).
        Commands::Check { path, git_diff } => surface_check_action::handle_check(
            path,
            git_diff,
            surface_check_command::CheckContext {
                code_analysis_linter: arch_linter.clone(),
                import_orchestrator: import_orchestrator.clone(),
                naming_orchestrator: naming_orchestrator.clone(),
                external_lint: external_lint_aggregate.clone(),
                role_orchestrator: role_orchestrator.clone(),
                scanner_provider: Arc::new(
                    shared::common::infrastructure_file_collector_provider::FileCollectorProvider::new(),
                ),
                orphan_orchestrator: orphan_container.analyzer(),
                layer_detector: layer_detector.clone(),
                language_detector: Arc::new(
                    cli_commands::infrastructure_language_detector::CliLanguageDetector::new(),
                ),
            },
            filter,
            Some(git_aggregate.clone()),
            shared::config_system::taxonomy_config_vo::ArchitectureConfig::default(),
        ),
        // SCAN: Multi-project lint — discovers workspace members (Cargo.toml,
        // pyproject.toml, package.json) and runs all linters on each one.
        // Uses OrchestratorFactory to create per-project DI containers.
        Commands::Scan { path } => surface_check_action::handle_scan(
            path,
            surface_check_command::CheckContext {
                code_analysis_linter: arch_linter,
                import_orchestrator,
                naming_orchestrator: naming_orchestrator.clone(),
                external_lint: external_lint_aggregate.clone(),
                role_orchestrator: role_orchestrator.clone(),
                scanner_provider: Arc::new(
                    shared::common::infrastructure_file_collector_provider::FileCollectorProvider::new(),
                ),
                orphan_orchestrator: orphan_container.analyzer(),
                layer_detector: layer_detector.clone(),
                language_detector: Arc::new(
                    cli_commands::infrastructure_language_detector::CliLanguageDetector::new(),
                ),
            },
            Some(multi_project_orchestrator.clone()),
            factory,
            filter,
        ),
        // FIX: Applies safe automatic fixes (removing unused/forbidden imports).
        // --dry-run previews changes without modifying files.
        Commands::Fix { path, dry_run } => surface_fix_command::handle_fix(
            path,
            dry_run,
            arch_linter.clone(),
            fix_orchestrator_factory,
        ),
        // CI: Lint + score calculation + exit code. Used for quality gates.
        // Exits with 1 if score < threshold.
        Commands::Ci { path, threshold } => {
            surface_check_action::handle_ci(arch_linter.clone(), path, threshold)
        }
        // DOCTOR: Environment diagnostics — checks Rust, Python, Node.js tooling.
        Commands::Doctor => {
            let maintenance_container =
                maintenance::root_maintenance_container::MaintenanceContainer::new();
            let orchestrator = maintenance_container.orchestrator();
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(r) => r,
                Err(_) => return ExitCode::FAILURE,
            };
            rt.block_on(cli_commands::surface_maintenance_command::handle_doctor(
                orchestrator,
            ))
        }
        // VERSION: Prints version info. --verbose shows commit hash and rustc.
        Commands::Version => {
            let verbose = raw_args.iter().any(|a| a == "--verbose" || a == "-v");
            let ver = env!("CARGO_PKG_VERSION");
            if verbose {
                println!("Lint Arwaky v{}", ver);
                // Try to capture current git commit hash for verbose output.
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
                let rustc = default_rustc_version();
                println!("  Rustc:     {}", rustc);
                println!("  License:   MIT");
            } else {
                println!("Lint Arwaky v{ver} (AES Semantic Builder)");
            }
            ExitCode::SUCCESS
        }
        // ADAPTERS: Lists available external linter adapters (Clippy, Ruff, ESLint, etc.).
        Commands::Adapters => surface_plugin_command::handle_adapters(external_lint_aggregate),
        // ORPHAN: Checks if a single file is unreachable/dead code (AES501-506).
        Commands::Orphan { path } => {
            let surface = surface_check_command::CheckCommandsSurface::new(
                surface_check_command::CheckContext {
                    code_analysis_linter: arch_linter.clone(),
                    import_orchestrator: import_orchestrator.clone(),
                    naming_orchestrator: naming_orchestrator.clone(),
                    external_lint: external_lint_aggregate.clone(),
                    role_orchestrator: role_orchestrator.clone(),
                    scanner_provider: Arc::new(
                        shared::common::infrastructure_file_collector_provider::FileCollectorProvider::new(),
                    ),
                    orphan_orchestrator: orphan_container.analyzer(),
                    layer_detector: layer_detector.clone(),
                    language_detector: Arc::new(
                        cli_commands::infrastructure_language_detector::CliLanguageDetector::new(),
                    ),
                },
            );
            surface.check_orphan_single_file(&path);
            ExitCode::SUCCESS
        }
        // SECURITY: Runs Cargo Audit (dependency vulnerability scan) on the target.
        Commands::Security { path } => {
            let maintenance_container =
                maintenance::root_maintenance_container::MaintenanceContainer::new();
            let orchestrator = maintenance_container.orchestrator();
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(r) => r,
                Err(_) => return ExitCode::FAILURE,
            };
            rt.block_on(cli_commands::surface_maintenance_command::handle_security(
                orchestrator,
                path,
            ))
        }
        // DUPLICATES: Detects duplicated code blocks within the target directory.
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
        // DEPENDENCIES: Analyzes and reports on project dependencies.
        Commands::Dependencies { path } => {
            let maintenance_container =
                maintenance::root_maintenance_container::MaintenanceContainer::new();
            let orchestrator = maintenance_container.orchestrator();
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(r) => r,
                Err(_) => return ExitCode::FAILURE,
            };
            rt.block_on(
                cli_commands::surface_maintenance_command::handle_dependencies(orchestrator, path),
            )
        }
        // WATCH: Real-time file watching — lint files automatically on save.
        // Uses inotify (Linux) via the `notify` crate with 500ms debounce.
        Commands::Watch { path } => {
            let container = file_watch::FileWatchContainer::new();
            let watch_agg: Arc<dyn shared::file_watch::contract_watch_aggregate::IWatchAggregate> =
                container.orchestrator(arch_linter.clone());
            surface_watch_command::handle_watch(watch_agg, path)
        }
        // INSTALL-HOOK: Installs a Git pre-commit hook that runs `lint-arwaky-cli check`.
        Commands::InstallHook => {
            let container = git_hooks::root_git_hooks_container::GitContainer::new_default();
            let aggregate = container.aggregate();
            // Tokio runtime is needed because the Git hooks aggregate is async.
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(r) => r,
                Err(_) => return ExitCode::FAILURE,
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
                    ExitCode::FAILURE
                }
            }
        }
        // UNINSTALL-HOOK: Removes the previously installed git pre-commit hook.
        Commands::UninstallHook => {
            let container = git_hooks::root_git_hooks_container::GitContainer::new_default();
            let aggregate = container.aggregate();
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(r) => r,
                Err(_) => return ExitCode::FAILURE,
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
        // INIT: Creates config files (lint_arwaky.config.*.yaml) for a new project.
        // --global creates config in the user's home directory.
        Commands::Init { global } => {
            let setup_container =
                project_setup::root_project_setup_container::SetupContainer::new();
            let setup_orchestrator = setup_container.aggregate();
            cli_commands::surface_setup_command::handle_init(setup_orchestrator, global)
        }
        // INSTALL: Installs the lint-arwaky binaries to the system.
        // --sudo uses sudo for system-wide installation.
        Commands::Install { sudo } => {
            let setup_container =
                project_setup::root_project_setup_container::SetupContainer::new();
            let setup_orchestrator = setup_container.aggregate();
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(r) => r,
                Err(_) => return ExitCode::FAILURE,
            };
            rt.block_on(cli_commands::surface_setup_command::handle_install(
                setup_orchestrator,
                sudo,
            ))
        }
        // MCP-CONFIG: Generates MCP server configuration for Claude Desktop.
        Commands::McpConfig { client } => {
            cli_commands::surface_setup_command::handle_mcp_config(&client)
        }
        // CONFIG-SHOW: Displays the current effective configuration.
        Commands::ConfigShow => {
            let config_container =
                config_system::root_config_system_container::ConfigContainer::new();
            let config_orchestrator = config_container.orchestrator();
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(r) => r,
                Err(_) => return ExitCode::FAILURE,
            };
            rt.block_on(cli_commands::surface_config_command::handle_config_show(
                config_orchestrator,
            ))
        }
    }
}

// run_default_check: Self-lint shortcut when no subcommand is provided.
// Running `lint-arwaky-cli` without args triggers a full AES audit of the
// current directory. This is used during development to self-lint the project.
// It generates a formatted compliance report grouped by severity level.
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

    // Group results by severity level for structured output
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
    // Print severity groups (non-empty only), with file and message details
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

    // Summary: total violations broken down by AES code
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
    // PASS if zero violations, FAIL otherwise
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
    // Exit code 1 only if there are CRITICAL violations
    if has_critical(&results) {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

// Creates a FilePath from a raw string, falling back to default if invalid.
// In practice this should never fail since the input is controlled.
fn default_file_path(s: String) -> shared::common::taxonomy_path_vo::FilePath {
    if let Ok(p) = shared::common::taxonomy_path_vo::FilePath::new(s) {
        return p;
    }
    shared::common::taxonomy_path_vo::FilePath::default()
}

// Returns the Rust compiler version used to build the binary.
// Checks build-time env vars: VERGEN_RUSTC_SEMVER (vergen) or RUSTC_VERSION.
fn default_rustc_version() -> &'static str {
    if let Some(v) = option_env!("VERGEN_RUSTC_SEMVER") {
        v
    } else if let Some(v) = option_env!("RUSTC_VERSION") {
        v
    } else {
        "stable"
    }
}
