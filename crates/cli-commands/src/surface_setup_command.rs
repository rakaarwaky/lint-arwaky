// PURPOSE: SetupCommandsSurface — CLI surface for project setup (init, install, mcp-config)
//
// Three subcommands:
//   - init:        writes lint_arwaky.config.<lang>.yaml (local)
//   - install:     pip install Python adapters (ruff, mypy, bandit) + npm install JS adapters (eslint, prettier, typescript)
//   - mcp-config:  prints MCP client config JSON for Claude/Cursor/Windsurf/Copilot
//
// Binary resolution for mcp-config: checks sibling of current exe first, fails closed (no PATH fallback).

use shared::common::ExitCode;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::project_setup::SetupManagementAggregate;
use std::sync::Arc;

pub fn handle_init(setup_orchestrator: Arc<dyn SetupManagementAggregate>) -> ExitCode {
    // 1. Write language config files (always overwrite)
    let mut all_ok = true;
    let languages = setup_orchestrator.detect_languages();
    for lang in languages.iter() {
        let lang_str = lang.value();
        let target = format!("lint_arwaky.config.{}.yaml", lang_str);
        let content = setup_orchestrator.get_config_template(lang_str);
        match setup_orchestrator.write_config_file(&target, content) {
            Ok(desc) => {
                println!(
                    "Config written/overwritten: {} (language: {})",
                    target, lang_str
                );
                println!("  {}", desc.value);
            }
            Err(e) => {
                println!("Error creating config for {}: {e}", lang_str);
                all_ok = false;
            }
        }
    }

    // 2. Distribute docs from XDG config to project (always overwrite)
    let doc_files = [
        "ARCHITECTURE.md",
        "MIGRATION_RUST.md",
        "MIGRATION_PYTHON.md",
        "MIGRATION_TYPESCRIPT.md",
        "RULES_AES.md",
    ];
    if let Some(config_dir) = dirs::config_dir() {
        let xdg_base = config_dir.join("lint-arwaky");
        for doc in &doc_files {
            let xdg_src = xdg_base.join(doc);
            if !xdg_src.exists() {
                println!("  {doc} — not in XDG config, skipping");
                continue;
            }
            match filesystem::FilesystemOrchestrator::new().read_to_string(&xdg_src) {
                Ok(content) => {
                    if let Some(parent) = std::path::Path::new(doc).parent() {
                        let _ = filesystem::FilesystemOrchestrator::new().create_dir_all(parent);
                    }
                    match setup_orchestrator.write_config_file(doc, &content) {
                        Ok(_) => println!("  {doc} — copied/overwritten from XDG config"),
                        Err(e) => println!("  {doc} — error: {e}"),
                    }
                }
                Err(e) => println!("  {doc} — read error: {e}"),
            }
        }

        // 3. Copy .agents/ (prompts, rules, skills) from XDG config to current project
        let xdg_agents = xdg_base.join(".agents");
        if xdg_agents.exists() && xdg_agents.is_dir() {
            let target_agents = std::path::Path::new(".agents");
            match copy_dir_all(&xdg_agents, target_agents) {
                Ok(count) => {
                    println!(
                        "  .agents/ (prompts, rules, skills) — copied/overwritten {count} file(s) from XDG config"
                    );
                }
                Err(e) => {
                    println!("  .agents/ — copy error: {e}");
                    all_ok = false;
                }
            }
        } else {
            println!("  .agents/ — not in XDG config, skipping");
        }
    } else {
        println!("Warning: could not determine XDG config dir");
    }

    if all_ok {
        ExitCode::OK
    } else {
        ExitCode::POLICY_FAIL
    }
}

fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<usize> {
    filesystem::FilesystemOrchestrator::new().create_dir_all(dst)?;
    let mut count = 0;
    let entries = filesystem::FilesystemOrchestrator::new().scan_directory(src);
    for src_path in entries {
        let file_name = src_path.file_name().unwrap_or_default();
        let dst_path = dst.join(file_name);
        if src_path.is_dir() {
            count += copy_dir_all(&src_path, &dst_path)?;
        } else {
            filesystem::FilesystemOrchestrator::new().copy_file(&src_path, &dst_path)?;
            count += 1;
        }
    }
    Ok(count)
}

pub async fn handle_install(
    setup_orchestrator: Arc<dyn SetupManagementAggregate>,
    sudo: bool,
) -> ExitCode {
    println!("Lint Arwaky — Install Adapter Dependencies");
    println!("{}", "=".repeat(50));

    println!("\n[1/2] Installing Python adapters (ruff, mypy, bandit)...");
    let py_status = setup_orchestrator.install_python_adapters().await;
    if py_status.value {
        println!("  Python adapters installed");
    } else {
        println!("  Failed to install Python adapters");
    }

    println!("\n[2/2] Installing JavaScript adapters (eslint, prettier, typescript)...");
    let js_status = setup_orchestrator.install_javascript_adapters(sudo).await;
    if js_status.value {
        println!("  JavaScript adapters installed");
    } else {
        println!("  Failed to install JavaScript adapters");
    }

    println!("\n{}", "=".repeat(50));
    if py_status.value && js_status.value {
        println!("Done! Run `lint-arwaky doctor` to verify.");
        ExitCode::OK
    } else {
        println!("Installation failed. Run with `--sudo` if npm globally requires permissions.");
        ExitCode::POLICY_FAIL
    }
}

pub fn handle_mcp_config(client: &str) -> ExitCode {
    let binary = which_mcp_binary();
    let config = match client {
        "claude-code" | "claude" => serde_json::json!({
            "mcpServers": {
                "lint-arwaky": {
                    "command": binary,
                    "args": [],
                    "env": {}
                }
            }
        }),
        "cursor" => serde_json::json!({
            "mcpServers": {
                "lint-arwaky": {
                    "command": binary,
                    "args": [],
                    "env": {}
                }
            }
        }),
        "windsurf" => serde_json::json!({
            "config:lint-arwaky": {
                "command": binary,
                "args": [],
                "env": {}
            }
        }),
        "copilot" => serde_json::json!({
            "inputs": [],
            "server": {
                "command": binary,
                "args": [],
                "env": {}
            }
        }),
        "hermes" => serde_json::json!({
            "mcpServers": {
                "lint-arwaky": {
                    "command": binary,
                    "args": [],
                    "env": {}
                }
            }
        }),
        "vscode" => serde_json::json!({
            "mcpServers": {
                "lint-arwaky": {
                    "command": binary,
                    "args": [],
                    "env": {}
                }
            }
        }),
        "all" => serde_json::json!({
            "mcpServers": {
                "lint-arwaky": {
                    "command": binary,
                    "args": [],
                    "env": {}
                }
            }
        }),
        _ => serde_json::json!({
            "mcpServers": {
                "lint-arwaky": {
                    "command": binary,
                    "args": [],
                    "env": {}
                }
            }
        }),
    };
    let json_str = serde_json::to_string_pretty(&config).unwrap_or_default();
    println!("MCP Client Configuration for: {}", client);
    println!("Binary: {}", binary);
    println!();
    println!("{}", json_str);
    ExitCode::OK
}

fn which_mcp_binary() -> String {
    match resolve_mcp_binary() {
        Ok(path) => path.to_string_lossy().into_owned(),
        Err(_) => {
            // Fail closed — do not fall back to bare PATH lookup.
            // Caller should handle the error gracefully; we provide a fallback hint.
            "lint-arwaky-mcp".to_string()
        }
    }
}

/// Resolve the MCP binary to an absolute canonicalized path.
///
/// Resolution order:
///   1. LINT_ARWAKY_MCP_BIN env var (explicit override)
///   2. Sibling of current executable
///   3. Fail closed — no bare PATH fallback (prevents PATH hijacking)
fn resolve_mcp_binary() -> Result<std::path::PathBuf, String> {
    // 1. Explicit override
    if let Ok(explicit) = std::env::var("LINT_ARWAKY_MCP_BIN") {
        let path = std::path::PathBuf::from(&explicit);
        if !path.is_file() {
            return Err(format!(
                "LINT_ARWAKY_MCP_BIN points to non-file: {}",
                path.display()
            ));
        }
        return path
            .canonicalize()
            .map_err(|e| format!("cannot canonicalize LINT_ARWAKY_MCP_BIN: {e}"));
    }

    // 2. Sibling of current executable
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        let sibling = dir.join("lint-arwaky-mcp");
        if sibling.is_file() {
            return sibling
                .canonicalize()
                .map_err(|e| format!("cannot canonicalize sibling: {e}"));
        }
    }

    // 3. Do NOT fall back to bare PATH — fail closed (P1.2)
    Err("lint-arwaky-mcp not found. Set LINT_ARWAKY_MCP_BIN to an absolute path.".into())
}
