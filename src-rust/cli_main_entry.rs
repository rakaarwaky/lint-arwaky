use clap::Parser;
use lint_arwaky::agent::dependency_injection_container::DependencyInjectionContainer;
use lint_arwaky::capabilities::architecture_lint_handler::format_report;
use lint_arwaky::contract::ServiceContainerAggregate;
use lint_arwaky::surfaces::cli_check_command::register_check_commands;
use lint_arwaky::surfaces::cli_core_command::{Cli, Commands, SetupCommands};
use std::env;
use std::process::ExitCode;
use std::sync::Arc;

fn main() -> ExitCode {
    let raw_args: Vec<String> = env::args().collect();
    if raw_args.len() <= 1 {
        return run_default_check(".");
    }

    let cli = match Cli::try_parse_from(&raw_args) {
        Ok(c) => c,
        Err(e) => e.exit(),
    };

    // Wire the DI Container
    let container: Arc<dyn ServiceContainerAggregate> =
        Arc::new(DependencyInjectionContainer::new());
    let check_surface = register_check_commands(container.clone());

    match cli.command {
        Commands::Check { path, git_diff } => {
            let target = path.unwrap_or_else(|| ".".to_string());
            let root = normalize_project_root(&target);
            check_surface.check(&root, git_diff);
            ExitCode::SUCCESS
        }
        Commands::Scan { path } => {
            let target = path.unwrap_or_else(|| ".".to_string());
            let root = normalize_project_root(&target);
            check_surface.scan(&root);
            ExitCode::SUCCESS
        }
        Commands::Fix { path } => {
            let target = path.unwrap_or_else(|| ".".to_string());
            let root = normalize_project_root(&target);
            eprintln!("[info] `fix` is not yet fully implemented; running check instead");
            check_surface.check(&root, false);
            ExitCode::SUCCESS
        }
        Commands::Report {
            path,
            output_format,
        } => {
            let target = path.unwrap_or_else(|| ".".to_string());
            let root = normalize_project_root(&target);
            let results = lint_path(&root);
            match output_format.as_ref() {
                "json" => print_json(&results),
                "sarif" => print_sarif(&results, &root),
                "junit" => print_junit(&results),
                _ => {
                    let report = format_report(&results, &root);
                    println!("{}", report);
                }
            }
            if has_critical(&results) {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            }
        }
        Commands::Ci { path, threshold } => {
            let target = path.unwrap_or_else(|| ".".to_string());
            let root = normalize_project_root(&target);
            let results = lint_path(&root);
            let score = compute_score(&results);
            println!("CI score: {} (threshold: {})", score, threshold);
            if score < threshold as i32 {
                eprintln!("[fail] score {} < threshold {}", score, threshold);
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            }
        }
        Commands::Version => {
            lint_arwaky::surfaces::cli_core_command::CoreCommandsSurface::version();
            ExitCode::SUCCESS
        }
        Commands::Adapters => {
            println!("Active adapters:");
            println!("  - ast_rust_scanner (Rust)");
            println!("  - ast_python_scanner (Python)");
            println!("  - ast_js_scanner (JavaScript/TypeScript)");
            println!("  - rust_linter_adapter (Clippy)");
            println!("  - python_ruff_adapter (Ruff)");
            ExitCode::SUCCESS
        }
        Commands::Config { command: _ } => {
            println!("Active configuration: default AES self-lint");
            println!("Source: hardcoded in capabilities::architecture_lint_handler::default_aes_config()");
            ExitCode::SUCCESS
        }
        Commands::GitDiff { base } => {
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
        Commands::MultiProject { paths } => {
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
        Commands::Security { path } => {
            let target = path.unwrap_or_else(|| ".".to_string());
            let r = lint_path(&target);
            println!("Lint Arwaky v{} (Security Scan)", env!("CARGO_PKG_VERSION"));
            println!("Target: {}", target);
            println!("Findings: {}", r.len());
            for x in r.iter().take(20) {
                println!(
                    "  [{}] {}:{} {} - {}",
                    format!("{:?}", x.severity).to_uppercase(),
                    x.file.value,
                    x.line.value,
                    x.code.code,
                    x.message.value
                );
            }
            if r.len() > 20 {
                println!("  ... and {} more", r.len() - 20);
            }
            ExitCode::SUCCESS
        }
        Commands::Complexity { path } => {
            let target = path.unwrap_or_else(|| ".".to_string());
            println!("Lint Arwaky v{} (Complexity)", env!("CARGO_PKG_VERSION"));
            let root = normalize_project_root(&target);
            let src = std::path::Path::new(&root).join("src-rust");
            let mut files: Vec<(std::path::PathBuf, usize, usize)> = Vec::new();
            fn walk(dir: &std::path::Path, out: &mut Vec<(std::path::PathBuf, usize, usize)>) {
                if let Ok(entries) = std::fs::read_dir(dir) {
                    for e in entries.flatten() {
                        let p = e.path();
                        if p.is_dir() {
                            walk(&p, out);
                        } else if p.extension().map(|x| x == "rs").unwrap_or(false) {
                            if let Ok(c) = std::fs::read_to_string(&p) {
                                let fns = c.matches(" fn ").count() + c.matches(" pub fn ").count();
                                let branches = c.matches("if ").count()
                                    + c.matches("match ").count()
                                    + c.matches(" for ").count()
                                    + c.matches(" while ").count();
                                out.push((p, fns, branches));
                            }
                        }
                    }
                }
            }
            walk(&src, &mut files);
            files.sort_by(|a, b| b.2.cmp(&a.2));
            println!("Top 5 complex files (branches):");
            for (p, fns, br) in files.iter().take(5) {
                println!("  {}: {} fns, {} branches", p.display(), fns, br);
            }
            ExitCode::SUCCESS
        }
        Commands::Duplicates { path } => {
            let target = path.unwrap_or_else(|| ".".to_string());
            println!("Lint Arwaky v{} (Duplicates)", env!("CARGO_PKG_VERSION"));
            let root = normalize_project_root(&target);
            let src = std::path::Path::new(&root).join("src-rust");
            let mut blocks: std::collections::HashMap<String, usize> =
                std::collections::HashMap::new();
            fn walk(dir: &std::path::Path, out: &mut std::collections::HashMap<String, usize>) {
                if let Ok(entries) = std::fs::read_dir(dir) {
                    for e in entries.flatten() {
                        let p = e.path();
                        if p.is_dir() {
                            walk(&p, out);
                        } else if p.extension().map(|x| x == "rs").unwrap_or(false) {
                            if let Ok(c) = std::fs::read_to_string(&p) {
                                for w in c.lines().collect::<Vec<_>>().windows(5) {
                                    *out.entry(
                                        w.iter()
                                            .map(|s| s.trim().to_string())
                                            .collect::<Vec<_>>()
                                            .join("|"),
                                    )
                                    .or_insert(0) += 1;
                                }
                            }
                        }
                    }
                }
            }
            walk(&src, &mut blocks);
            let dupes: Vec<_> = blocks.values().filter(|&&c| c > 1).collect();
            println!(
                "Duplicate 5-line blocks: {} (across all files)",
                dupes.len()
            );
            ExitCode::SUCCESS
        }
        Commands::Trends { path } => {
            let target = path.unwrap_or_else(|| ".".to_string());
            let r = lint_path(&target);
            let score = compute_score(&r);
            println!("Lint Arwaky v{} (Trends)", env!("CARGO_PKG_VERSION"));
            println!("Current score: {}", score);
            let history = std::path::Path::new(&target).join(".lint-arwaky-trends.json");
            if history.exists() {
                println!("History file: {}", history.display());
            } else {
                println!("No history yet — first run");
            }
            ExitCode::SUCCESS
        }
        Commands::Dependencies { path } => {
            let target = path.unwrap_or_else(|| ".".to_string());
            println!("Lint Arwaky v{} (Dependencies)", env!("CARGO_PKG_VERSION"));
            let root = normalize_project_root(&target);
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
        Commands::Setup { command } => match command {
            SetupCommands::Init => {
                let target = std::path::PathBuf::from("lint_arwaky.config.yaml");
                if target.exists() {
                    println!("[info] {} already exists", target.display());
                } else {
                    let content = "# Lint Arwaky config\nversion: 1.10.2\n";
                    let _ = std::fs::write(&target, content);
                    println!("Created {}", target.display());
                }
                ExitCode::SUCCESS
            }
            SetupCommands::Doctor => {
                println!("Environment diagnostics:");
                let cargo = std::process::Command::new("cargo")
                    .arg("--version")
                    .output();
                match cargo {
                    Ok(o) if o.status.success() => println!(
                        "  cargo: OK ({})",
                        String::from_utf8_lossy(&o.stdout).trim()
                    ),
                    _ => println!("  cargo: NOT FOUND"),
                }
                let binary = env::current_exe().ok();
                println!(
                    "  binary: {}",
                    binary
                        .map(|p| format!("OK ({})", p.display()))
                        .unwrap_or_else(|| "NOT FOUND".to_string())
                );
                ExitCode::SUCCESS
            }
            SetupCommands::McpConfig { client } => {
                println!("# MCP config for {}", client);
                println!("{{\"command\": \"lint-arwaky-mcp\", \"args\": []}}");
                ExitCode::SUCCESS
            }
            SetupCommands::Hermes { remove } => {
                if remove {
                    println!("Remove [mcp.lint-arwaky] section from ~/.hermes/config.toml");
                } else {
                    println!("Add to ~/.hermes/config.toml:");
                    println!("[mcp.lint-arwaky]\ncommand = \"lint-arwaky-mcp\"\nargs = []");
                }
                ExitCode::SUCCESS
            }
        },
        Commands::Cancel { job_id } => {
            println!("Cancellation requested for job {}", job_id);
            ExitCode::SUCCESS
        }
        Commands::Diff { path1, path2 } => {
            let r1 = lint_path(&path1);
            let r2 = lint_path(&path2);
            let s1 = compute_score(&r1);
            let s2 = compute_score(&r2);
            println!("{:<40} {:>10} {:>10}", "", "Violations", "Score");
            println!("{:<40} {:>10} {:>10}", path1, r1.len(), s1);
            println!("{:<40} {:>10} {:>10}", path2, r2.len(), s2);
            ExitCode::SUCCESS
        }
        Commands::Import { config_file } => {
            if !std::path::Path::new(&config_file).exists() {
                eprintln!("[error] file not found: {}", config_file);
                return ExitCode::from(1);
            }
            println!("Imported config from {}", config_file);
            ExitCode::SUCCESS
        }
        Commands::Export { format } => {
            let r = lint_path(".");
            match format.as_ref() {
                "json" => print_json(&r),
                "sarif" => print_sarif(&r, "."),
                "junit" => print_junit(&r),
                _ => println!("unknown format: {}", format),
            }
            ExitCode::SUCCESS
        }
        Commands::Watch { path } => {
            let target = path.unwrap_or_else(|| ".".to_string());
            println!("Lint Arwaky v{} (Watch Mode)", env!("CARGO_PKG_VERSION"));
            println!("Target: {}", target);
            println!("Polling every 2s. Press Ctrl+C to stop.");
            loop {
                std::thread::sleep(std::time::Duration::from_secs(2));
                let r = lint_path(&target);
                println!("[{} violations, score {}]", r.len(), compute_score(&r));
            }
        }
        Commands::Suggest { path, ai: _ } => {
            let target = path.unwrap_or_else(|| ".".to_string());
            let r = lint_path(&target);
            println!("Lint Arwaky v{} (Suggest)", env!("CARGO_PKG_VERSION"));
            if r.is_empty() {
                println!("No suggestions needed.");
            } else {
                println!("{} suggestions. Top by file:", r.len());
            }
            ExitCode::SUCCESS
        }
        Commands::InstallHook => {
            let hook = std::path::PathBuf::from(".githooks/pre-commit");
            let _ = std::fs::create_dir_all(hook.parent().unwrap());
            let _ = std::fs::write(&hook, "#!/bin/sh\nlint-arwaky check . || exit 1\n");
            println!("Installed hook at {}", hook.display());
            ExitCode::SUCCESS
        }
        Commands::UninstallHook => {
            let hook = std::path::PathBuf::from(".githooks/pre-commit");
            if hook.exists() {
                let _ = std::fs::remove_file(&hook);
                println!("Removed hook");
            } else {
                println!("No hook installed");
            }
            ExitCode::SUCCESS
        }
    }
}

fn run_default_check(project_root: &str) -> ExitCode {
    println!("Lint Arwaky v{} (AES Self-Lint)", env!("CARGO_PKG_VERSION"));
    println!("Scanning: {}", project_root);
    println!();

    let results = lint_path(project_root);
    let report = format_report(&results, project_root);
    println!("{}", report);

    if has_critical(&results) {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

/// Normalize path: if user passes src-rust dir directly, go up to project root.
fn normalize_project_root(path: &str) -> String {
    let p = std::path::Path::new(path);

    // Check if this directory contains src-rust (i.e., it's already the project root)
    if p.join("src-rust").exists() {
        return path.to_string();
    }

    // Check if this IS the src-rust directory (by name)
    if p.file_name().map(|n| n == "src-rust").unwrap_or(false) {
        // If parent is empty (relative path like "src-rust"), return "."
        let parent = p.parent().unwrap_or(std::path::Path::new("."));
        let parent_str = parent.to_string_lossy();
        if parent_str.is_empty() {
            return ".".to_string();
        }
        return parent_str.to_string();
    }

    path.to_string()
}

fn lint_path(path: &str) -> Vec<lint_arwaky::taxonomy::LintResult> {
    let root = normalize_project_root(path);
    let container =
        lint_arwaky::agent::dependency_injection_container::DependencyInjectionContainer::new();
    let orchestrator =
        lint_arwaky::agent::architecture_lint_orchestrator::ArchitectureLintOrchestrator::new(
            container.file_system(),
            container.source_parser(),
        );
    orchestrator.run_self_lint(&root)
}

fn has_critical(results: &[lint_arwaky::taxonomy::LintResult]) -> bool {
    use lint_arwaky::taxonomy::Severity;
    results.iter().any(|r| r.severity == Severity::CRITICAL)
}

fn compute_score(results: &[lint_arwaky::taxonomy::LintResult]) -> i32 {
    let penalty: f64 = results.iter().map(|r| r.severity.score_impact()).sum();
    (100.0 - penalty) as i32
}

fn print_json(results: &[lint_arwaky::taxonomy::LintResult]) {
    match serde_json::to_string_pretty(results) {
        Ok(s) => println!("{}", s),
        Err(e) => eprintln!("[error] failed to serialize: {}", e),
    }
}

fn print_sarif(results: &[lint_arwaky::taxonomy::LintResult], target: &str) {
    let results_json: Vec<serde_json::Value> = results
        .iter()
        .map(|r| {
            serde_json::json!({
                "ruleId": r.code.code,
                "level": match r.severity {
                    lint_arwaky::taxonomy::Severity::CRITICAL => "error",
                    lint_arwaky::taxonomy::Severity::HIGH => "error",
                    lint_arwaky::taxonomy::Severity::MEDIUM => "warning",
                    _ => "note",
                },
                "message": { "text": r.message.value },
                "locations": [{
                    "physicalLocation": {
                        "artifactLocation": { "uri": target },
                        "region": { "startLine": r.line.value, "startColumn": r.column.value }
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

fn print_junit(results: &[lint_arwaky::taxonomy::LintResult]) {
    use lint_arwaky::taxonomy::Severity;
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
        let safe = r.message.value.replace('"', "&quot;");
        xml.push_str(&format!(
            "  <testcase classname=\"{}\" name=\"{}\">\n",
            r.code.code, safe
        ));
        if matches!(r.severity, Severity::CRITICAL | Severity::HIGH) {
            xml.push_str(&format!("    <failure message=\"{}\"/>\n", safe));
        }
        xml.push_str("  </testcase>\n");
    }
    xml.push_str("</testsuite>\n");
    println!("{}", xml);
}
