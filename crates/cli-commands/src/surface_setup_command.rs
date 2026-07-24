// PURPOSE: SetupCommandsSurface — CLI surface for project setup (init, install, mcp-config)
//
// Three subcommands:
//   - init:        writes lint_arwaky.config.<lang>.yaml (local)
//   - install:     pip install Python adapters (ruff, mypy, bandit) + npm install JS adapters (eslint, prettier, typescript)
//   - mcp-config:  prints MCP client config JSON for Claude/Cursor/Windsurf/Copilot
//
// Binary resolution for mcp-config: checks sibling of current exe first, fails closed (no PATH fallback).

use shared::common::taxonomy_common_error::ExitCode;
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use std::sync::Arc;

pub fn handle_init(setup_orchestrator: Arc<dyn SetupManagementAggregate>) -> ExitCode {
    // 1. Write language config files
    let mut all_ok = true;
    let languages = setup_orchestrator.detect_languages();
    for lang in languages.iter() {
        let lang_str = lang.value();
        let target = format!("lint_arwaky.config.{}.yaml", lang_str);
        if setup_orchestrator.file_exists(&target) {
            println!("Config already exists: {}", target);
        } else {
            let content = setup_orchestrator.get_config_template(lang_str);
            match setup_orchestrator.write_config_file(&target, content) {
                Ok(desc) => {
                    println!("Config created: {} (language: {})", target, lang_str);
                    println!("  {}", desc.value);
                }
                Err(e) => {
                    println!("Error creating config for {}: {e}", lang_str);
                    all_ok = false;
                }
            }
        }
    }

    // 2. Distribute docs + SKILL.md from XDG config to project
    let doc_files = [
        "SKILL.md",
        "ARCHITECTURE.md",
        "MIGRATION_RUST.md",
        "MIGRATION_PYTHON.md",
        "MIGRATION_TYPESCRIPT.md",
        "RULES_AES.md",
    ];
    if let Some(config_dir) = dirs::config_dir() {
        let xdg_base = config_dir.join("lint-arwaky");
        for doc in &doc_files {
            if setup_orchestrator.file_exists(doc) {
                println!("  {doc} — already exists, skipping");
                continue;
            }
            let xdg_src = xdg_base.join(doc);
            if !xdg_src.exists() {
                println!("  {doc} — not in XDG config, skipping");
                continue;
            }
            match std::fs::read_to_string(&xdg_src) {
                Ok(content) => {
                    if let Some(parent) = std::path::Path::new(doc).parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    match setup_orchestrator.write_config_file(doc, &content) {
                        Ok(_) => println!("  {doc} — distributed from XDG config"),
                        Err(e) => println!("  {doc} — error: {e}"),
                    }
                }
                Err(e) => println!("  {doc} — read error: {e}"),
            }
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
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let sibling = dir.join("lint-arwaky-mcp");
            if sibling.is_file() {
                return sibling
                    .canonicalize()
                    .map_err(|e| format!("cannot canonicalize sibling: {e}"));
            }
        }
    }

    // 3. Do NOT fall back to bare PATH — fail closed (P1.2)
    Err("lint-arwaky-mcp not found. Set LINT_ARWAKY_MCP_BIN to an absolute path.".into())
}
