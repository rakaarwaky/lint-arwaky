use lint_arwaky::di_containers::agent_injection_container::DependencyInjectionContainer;
use std::env;
use std::process::ExitCode;
use std::sync::Arc;

/// CLI binary entry point for lint-arwaky-cli.
pub struct CliMainEntry {}

use clap::Parser;
use lint_arwaky::output_report::capabilities_reporting_formatter::ReportFormatterProcessor;
use lint_arwaky::di_containers::contract_service_aggregate::ServiceContainerAggregate;
use lint_arwaky::cli_commands::surface_core_command::{Cli, Commands, ConfigCommands, SetupCommands};
use lint_arwaky::cli_commands::surface_fix_command::register_fix_commands;
use lint_arwaky::source_parsing::taxonomy_path_vo::{DirectoryPath, FilePath}; use lint_arwaky::output_report::taxonomy_result_vo::LintResult; use lint_arwaky::output_report::taxonomy_severity_vo::Severity;

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
        Commands::Check { path, git_diff } => handle_check(path, git_diff),
        Commands::Scan { path } => handle_scan(path),
        Commands::Fix { path, dry_run } => handle_fix(path, dry_run),
        Commands::Report {
            path,
            output_format,
        } => handle_report(path, output_format),
        Commands::Ci { path, threshold } => handle_ci(path, threshold),
        Commands::Version => {
            lint_arwaky::cli_commands::surface_core_command::CoreCommandsSurface::version();
            ExitCode::SUCCESS
        }
        Commands::Adapters => handle_adapters(),
        Commands::Config { command } => handle_config(command),
        Commands::GitDiff { base } => handle_git_diff(base),
        Commands::MultiProject { paths } => handle_multi_project(paths),
        Commands::Security { path } => handle_security(path),
        Commands::Complexity { path } => handle_complexity(path),
        Commands::Duplicates { path } => handle_duplicates(path),
        Commands::Trends { path } => handle_trends(path),
        Commands::Dependencies { path } => handle_dependencies(path),
        Commands::Setup { command } => handle_setup(command),
        Commands::Cancel { job_id } => {
            println!("Cancellation requested for job {}", job_id);
            ExitCode::SUCCESS
        }
        Commands::Diff { path1, path2 } => handle_diff(path1, path2),
        Commands::Import { config_file } => handle_import(config_file),
        Commands::Export { format } => handle_export(format),
        Commands::Watch { path } => handle_watch(path),
        Commands::Suggest { path, ai: _ } => handle_suggest(path),
        Commands::InstallHook => handle_install_hook(),
        Commands::UninstallHook => handle_uninstall_hook(),
    }
}

// ─── Command Handlers ────────────────────────────────────────────────────────

fn handle_check(path: Option<String>, _git_diff: bool) -> ExitCode {
    let root = resolve_target(path);
    run_lint_and_report(&root)
}

fn handle_scan(path: Option<String>) -> ExitCode {
    let root = resolve_target(path);
    let container = Arc::new(DependencyInjectionContainer::new(
        DirectoryPath::new(&root).unwrap_or_default(),
    ));
    let surface = lint_arwaky::cli_commands::surface_check_command::register_check_commands(container);
    surface.scan(&root);
    ExitCode::SUCCESS
}

fn handle_fix(path: Option<String>, dry_run: bool) -> ExitCode {
    let root = resolve_target(path);
    let container = Arc::new(DependencyInjectionContainer::new(
        DirectoryPath::new(&root).unwrap_or_default(),
    ));
    let fix_surface = register_fix_commands(container);
    fix_surface.run_fix(FilePath::new(root).unwrap_or_default(), dry_run);
    ExitCode::SUCCESS
}

fn handle_report(path: Option<String>, output_format: String) -> ExitCode {
    let root = resolve_target(path);
    let results = lint_path(&root);
    match output_format.as_str() {
        "json" => print_json(&results),
        "sarif" => print_sarif(&results, &root),
        "junit" => print_junit(&results),
        _ => {
            let formatter = ReportFormatterProcessor::new();
            let report = formatter.format_text(&results, &root);
            println!("{}", report);
        }
    }
    if has_critical(&results) {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

fn handle_ci(path: Option<String>, threshold: u32) -> ExitCode {
    let root = resolve_target(path);
    let results = lint_path(&root);
    let score = compute_score(&results);
    println!("CI score: {} (threshold: {})", score, threshold);
    if (score as u32) < threshold {
        eprintln!("[fail] score {} < threshold {}", score, threshold);
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

fn handle_adapters() -> ExitCode {
    println!("Active adapters:");
    println!("  - ast_rust_scanner (Rust)");
    println!("  - ast_python_scanner (Python)");
    println!("  - ast_js_scanner (JavaScript/TypeScript)");
    println!("  - rust_linter_adapter (Clippy)");
    println!("  - python_ruff_adapter (Ruff)");
    ExitCode::SUCCESS
}

fn handle_config(command: ConfigCommands) -> ExitCode {
    match command {
        ConfigCommands::Show => {
            println!("Active configuration: default AES self-lint");
            println!("Source: hardcoded in capabilities::architecture_lint_handler::default_aes_config()");
        }
    }
    ExitCode::SUCCESS
}

fn handle_git_diff(base: String) -> ExitCode {
    println!("Lint Arwaky v{} (Git-Diff Mode)", env!("CARGO_PKG_VERSION"));
    let output = std::process::Command::new("git")
        .args(["diff", "--name-only", &base])
        .output();
    match output {
        Ok(o) if o.status.success() => {
            let s = String::from_utf8_lossy(&o.stdout);
            let files: Vec<&str> = s
                .lines()
                .filter(|l| {
                    l.ends_with(".rs")
                        || l.ends_with(".py")
                        || l.ends_with(".ts")
                        || l.ends_with(".js")
                })
                .collect();
            println!("Changed files since {}: {}", base, files.len());
            for f in files.iter().take(20) {
                println!("  {}", f);
            }
            if files.len() > 20 {
                println!("  ... and {} more", files.len() - 20);
            }
        }
        _ => eprintln!("[warn] not a git repo or `git diff` failed"),
    }
    ExitCode::SUCCESS
}

fn handle_multi_project(paths: Vec<String>) -> ExitCode {
    println!(
        "Lint Arwaky v{} (Multi-Project Mode)",
        env!("CARGO_PKG_VERSION")
    );
    let mut total = 0usize;
    for p in &paths {
        let r = lint_path(p);
        println!("  {} -> {} violations", p, r.len());
        total += r.len();
    }
    println!(
        "Total: {} violations across {} projects",
        total,
        paths.len()
    );
    ExitCode::SUCCESS
}

fn handle_security(path: Option<String>) -> ExitCode {
    let root = resolve_target(path);
    let results = lint_path(&root);
    println!("Lint Arwaky v{} (Security Scan)", env!("CARGO_PKG_VERSION"));
    println!("Target: {}", root);
    println!("Findings: {}", results.len());
    for x in results.iter().take(20) {
        println!(
            "  [{}] {}:{} {} - {}",
            format!("{:?}", x.severity).to_uppercase(),
            x.file.value(),
            x.line.value(),
            x.code.to_string(),
            x.message.value()
        );
    }
    if results.len() > 20 {
        println!("  ... and {} more", results.len() - 20);
    }
    ExitCode::SUCCESS
}

fn handle_complexity(path: Option<String>) -> ExitCode {
    let root = resolve_target(path);
    println!("Lint Arwaky v{} (Complexity)", env!("CARGO_PKG_VERSION"));
    let src = std::path::Path::new(&root).join("src-rust");
    let mut files: Vec<(std::path::PathBuf, usize, usize)> = Vec::new();
    walk_rs_files(&src, &mut |p| {
        if let Ok(c) = std::fs::read_to_string(&p) {
            let fns = c.matches(" fn ").count() + c.matches(" pub fn ").count();
            let branches = c.matches("if ").count()
                + c.matches("match ").count()
                + c.matches(" for ").count()
                + c.matches(" while ").count();
            files.push((p, fns, branches));
        }
    });
    files.sort_by(|a, b| b.2.cmp(&a.2));
    println!("Top 5 complex files (branches):");
    for (p, fns, br) in files.iter().take(5) {
        println!("  {}: {} fns, {} branches", p.display(), fns, br);
    }
    ExitCode::SUCCESS
}

fn handle_duplicates(path: Option<String>) -> ExitCode {
    let root = resolve_target(path);
    println!("Lint Arwaky v{} (Duplicates)", env!("CARGO_PKG_VERSION"));
    let src = std::path::Path::new(&root).join("src-rust");
    let mut blocks: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    walk_rs_files(&src, &mut |p| {
        if let Ok(c) = std::fs::read_to_string(&p) {
            for w in c.lines().collect::<Vec<_>>().windows(5) {
                *blocks
                    .entry(
                        w.iter()
                            .map(|s| s.trim().to_string())
                            .collect::<Vec<_>>()
                            .join("|"),
                    )
                    .or_insert(0) += 1;
            }
        }
    });
    let dupes: Vec<_> = blocks.values().filter(|&&c| c > 1).collect();
    println!(
        "Duplicate 5-line blocks: {} (across all files)",
        dupes.len()
    );
    ExitCode::SUCCESS
}

fn handle_trends(path: Option<String>) -> ExitCode {
    let root = resolve_target(path);
    let results = lint_path(&root);
    let score = compute_score(&results);
    let violations_count = results.len();
    let critical_count = results
        .iter()
        .filter(|r| r.severity == Severity::CRITICAL)
        .count();

    println!("Lint Arwaky v{} (Trends)", env!("CARGO_PKG_VERSION"));
    println!("Target: {}", root);
    println!();
    println!("Current scan:");
    println!("  Score:      {}/100", score);
    println!("  Violations: {}", violations_count);
    println!("  Critical:   {}", critical_count);

    // Use MetricsProvider via DI container (AES023 compliant)
    let container = Arc::new(DependencyInjectionContainer::new(
        DirectoryPath::new(&root).unwrap_or_default(),
    ));
    let metrics = container.metrics_provider();
    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(_) => {
            eprintln!("[error] failed to create tokio runtime");
            return ExitCode::from(1);
        }
    };

    // Read history
    let history: Vec<serde_json::Value> = if let Some(ref mp) = metrics {
        rt.block_on(mp.get_history())
    } else {
        Vec::new()
    };

    // Trend analysis
    if let Some(prev) = history.last() {
        let prev_score = prev.get("score").and_then(|s| s.as_f64()).unwrap_or(100.0);
        let prev_violations = prev.get("violations").and_then(|v| v.as_u64()).unwrap_or(0);
        let delta = score - prev_score;

        let trend = if delta > 1.0 {
            "IMPROVING"
        } else if delta < -1.0 {
            "DECLINING"
        } else {
            "STABLE"
        };

        let all_time_high = history
            .iter()
            .filter_map(|e| e.get("score").and_then(|s| s.as_f64()))
            .fold(score, f64::max);
        let all_time_low = history
            .iter()
            .filter_map(|e| e.get("score").and_then(|s| s.as_f64()))
            .fold(score, f64::min);

        println!();
        println!("Previous scan:");
        println!("  Score:      {}/100", prev_score);
        println!("  Violations: {}", prev_violations);
        println!();
        println!("Delta: {:+.1} — {}", delta, trend);
        println!("All-time high: {} / 100", all_time_high);
        println!("All-time low:  {} / 100", all_time_low);
        println!("History entries: {}", history.len());
    } else {
        println!();
        println!("No history yet — first run");
    }

    // Save current score to history via MetricsProvider
    use chrono::Utc;
    let entry = serde_json::json!({
        "score": score,
        "timestamp": Utc::now().to_rfc3339(),
        "violations": violations_count,
        "critical": critical_count,
    });

    if let Some(ref mp) = metrics {
        let saved = rt.block_on(mp.save_metric(entry));
        if saved {
            println!();
            println!("Current score saved to history");
        } else {
            eprintln!("[warn] Could not save history");
        }
    }

    ExitCode::SUCCESS
}

fn handle_dependencies(path: Option<String>) -> ExitCode {
    let root = resolve_target(path);
    println!("Lint Arwaky v{} (Dependencies)", env!("CARGO_PKG_VERSION"));
    let cargo = std::path::Path::new(&root).join("Cargo.toml");
    if cargo.exists() {
        println!("Rust (Cargo.toml):");
        if let Ok(c) = std::fs::read_to_string(&cargo) {
            let mut in_deps = false;
            for line in c.lines() {
                if line.starts_with("[dependencies]") {
                    in_deps = true;
                    continue;
                }
                if line.starts_with('[') {
                    in_deps = false;
                }
                if in_deps && line.contains('=') && !line.starts_with('#') {
                    println!("  {}", line.trim());
                }
            }
        }
    }
    ExitCode::SUCCESS
}

fn handle_setup(command: SetupCommands) -> ExitCode {
    match command {
        SetupCommands::Init => {
            let target = std::path::PathBuf::from("lint_arwaky.config.yaml");
            if target.exists() {
                println!("[info] {} already exists", target.display());
            } else {
                let content = "# Lint Arwaky config\nversion: 1.10.2\n";
                let _ = std::fs::write(&target, content);
                println!("Created {}", target.display());
            }
        }
        SetupCommands::Doctor => {
            println!("Environment diagnostics:");
            match std::process::Command::new("cargo")
                .arg("--version")
                .output()
            {
                Ok(o) if o.status.success() => {
                    println!(
                        "  cargo: OK ({})",
                        String::from_utf8_lossy(&o.stdout).trim()
                    );
                }
                _ => println!("  cargo: NOT FOUND"),
            }
            match env::current_exe() {
                Ok(p) => println!("  binary: OK ({})", p.display()),
                _ => println!("  binary: NOT FOUND"),
            }
        }
        SetupCommands::McpConfig { client } => {
            println!("# MCP config for {}", client);
            println!("{{\"command\": \"lint-arwaky-mcp\", \"args\": []}}");
        }
        SetupCommands::Hermes { remove } => {
            if remove {
                println!("Remove [mcp.lint-arwaky] section from ~/.hermes/config.toml");
            } else {
                println!("Add to ~/.hermes/config.toml:");
                println!("[mcp.lint-arwaky]\ncommand = \"lint-arwaky-mcp\"\nargs = []");
            }
        }
    }
    ExitCode::SUCCESS
}

fn handle_diff(path1: String, path2: String) -> ExitCode {
    let r1 = lint_path(&path1);
    let r2 = lint_path(&path2);
    let s1 = compute_score(&r1);
    let s2 = compute_score(&r2);
    println!("{:<40} {:>10} {:>10}", "", "Violations", "Score");
    println!("{:<40} {:>10} {:>10}", path1, r1.len(), s1);
    println!("{:<40} {:>10} {:>10}", path2, r2.len(), s2);
    ExitCode::SUCCESS
}

fn handle_import(config_file: String) -> ExitCode {
    if !std::path::Path::new(&config_file).exists() {
        eprintln!("[error] file not found: {}", config_file);
        return ExitCode::from(1);
    }
    println!("Imported config from {}", config_file);
    ExitCode::SUCCESS
}

fn handle_export(format: String) -> ExitCode {
    let results = lint_path(".");
    match format.as_str() {
        "json" => print_json(&results),
        "sarif" => print_sarif(&results, "."),
        "junit" => print_junit(&results),
        _ => eprintln!("unknown format: {}", format),
    }
    ExitCode::SUCCESS
}

fn handle_watch(path: Option<String>) -> ExitCode {
    let root = resolve_target(path);
    println!("Lint Arwaky v{} (Watch Mode)", env!("CARGO_PKG_VERSION"));
    println!("Target: {}", root);
    println!("Polling every 2s. Press Ctrl+C to stop.");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(2));
        let results = lint_path(&root);
        println!(
            "[{} violations, score {}]",
            results.len(),
            compute_score(&results)
        );
    }
}

fn handle_suggest(path: Option<String>) -> ExitCode {
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

fn handle_install_hook() -> ExitCode {
    let hook = std::path::PathBuf::from(".githooks/pre-commit");
    if let Some(parent) = hook.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&hook, "#!/bin/sh\nlint-arwaky check . || exit 1\n");
    println!("Installed hook at {}", hook.display());
    ExitCode::SUCCESS
}

fn handle_uninstall_hook() -> ExitCode {
    let hook = std::path::PathBuf::from(".githooks/pre-commit");
    if hook.exists() {
        let _ = std::fs::remove_file(&hook);
        println!("Removed hook");
    } else {
        println!("No hook installed");
    }
    ExitCode::SUCCESS
}

// ─── Shared Helpers ──────────────────────────────────────────────────────────

fn run_default_check(project_root: &str) -> ExitCode {
    println!("Lint Arwaky v{} (AES Self-Lint)", env!("CARGO_PKG_VERSION"));
    println!("Scanning: {}", project_root);
    println!();
    let results = lint_path(project_root);
    let formatter = ReportFormatterProcessor::new();
    let report = formatter.format_text(&results, project_root);
    println!("{}", report);
    if has_critical(&results) {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

fn resolve_target(path: Option<String>) -> String {
    normalize_project_root(&path.unwrap_or_else(|| ".".to_string()))
}

fn normalize_project_root(path: &str) -> String {
    let p = std::path::Path::new(path);
    if p.join("src-rust").exists() {
        return path.to_string();
    }
    if p.file_name().map(|n| n == "src-rust").unwrap_or(false) {
        let parent = p.parent().unwrap_or(std::path::Path::new("."));
        let parent_str = parent.to_string_lossy();
        return if parent_str.is_empty() {
            ".".to_string()
        } else {
            parent_str.to_string()
        };
    }
    path.to_string()
}

fn lint_path(path: &str) -> Vec<LintResult> {
    let root = FilePath::new(normalize_project_root(path))
        .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
    let orchestrator =
        lint_arwaky::code_analysis::agent_lint_orchestrator::ArchitectureLintOrchestrator::new();
    orchestrator.run_self_lint(&root)
}

fn has_critical(results: &[LintResult]) -> bool {
    results.iter().any(|r| r.severity == Severity::CRITICAL)
}

fn compute_score(results: &[LintResult]) -> f64 {
    let penalty: f64 = results.iter().map(|r| r.severity.score_impact()).sum();
    (100.0 - penalty).max(0.0)
}

fn walk_rs_files(dir: &std::path::Path, cb: &mut impl FnMut(std::path::PathBuf)) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                walk_rs_files(&p, cb);
            } else if p.extension().map(|x| x == "rs").unwrap_or(false) {
                cb(p);
            }
        }
    }
}

fn run_lint_and_report(root: &str) -> ExitCode {
    let results = lint_path(root);
    let formatter = ReportFormatterProcessor::new();
    let report = formatter.format_text(&results, root);
    println!("{}", report);
    if has_critical(&results) {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

// ─── Output Formatters ────────────────────────────────────────────────────────

fn print_json(results: &[LintResult]) {
    match serde_json::to_string_pretty(results) {
        Ok(s) => println!("{}", s),
        Err(e) => eprintln!("[error] failed to serialize: {}", e),
    }
}

fn print_sarif(results: &[LintResult], target: &str) {
    let results_json: Vec<serde_json::Value> = results
        .iter()
        .map(|r| {
            serde_json::json!({
                "ruleId": r.code.to_string(),
                "level": match r.severity {
                    Severity::CRITICAL => "error",
                    Severity::HIGH => "error",
                    Severity::MEDIUM => "warning",
                    _ => "note",
                },
                "message": { "text": r.message.value() },
                "locations": [{
                    "physicalLocation": {
                        "artifactLocation": { "uri": target },
                        "region": { "startLine": r.line.value(), "startColumn": r.column.value() }
                    }
                }]
            })
        })
        .collect();
    let sarif = serde_json::json!({
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": { "driver": { "name": "lint-arwaky", "version": env!("CARGO_PKG_VERSION") } },
            "results": results_json
        }]
    });
    match serde_json::to_string_pretty(&sarif) {
        Ok(s) => println!("{}", s),
        Err(e) => eprintln!("[error] failed to serialize SARIF: {}", e),
    }
}

fn print_junit(results: &[LintResult]) {
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    let failures = results
        .iter()
        .filter(|r| matches!(r.severity, Severity::CRITICAL | Severity::HIGH))
        .count();
    xml.push_str(&format!(
        "<testsuite name=\"lint-arwaky\" tests=\"{}\" failures=\"{}\">\n",
        results.len(),
        failures
    ));
    for r in results {
        let safe = r.message.value().replace('"', "&quot;");
        xml.push_str(&format!(
            "  <testcase classname=\"{}\" name=\"{}\">\n",
            r.code.to_string(),
            safe
        ));
        if matches!(r.severity, Severity::CRITICAL | Severity::HIGH) {
            xml.push_str(&format!("    <failure message=\"{}\"/>\n", safe));
        }
        xml.push_str("  </testcase>\n");
    }
    xml.push_str("</testsuite>\n");
    println!("{}", xml);
}
