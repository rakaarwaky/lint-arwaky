// PURPOSE: SetupCommandsSurface — CLI surface for project setup (init, install, mcp-config)
//
// Three subcommands:
//   - init:        writes lint_arwaky.config.<lang>.yaml (local) or global XDG configs
//   - install:     pip install Python adapters (ruff, mypy, bandit) + npm install JS adapters (eslint, prettier, typescript)
//   - mcp-config:  prints MCP client config JSON for Claude/Cursor/Windsurf/Copilot
//
// Binary resolution for mcp-config: checks sibling of current exe first, falls back to PATH.
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use std::process::ExitCode;
use std::sync::Arc;

pub fn handle_init(
    setup_orchestrator: Arc<dyn SetupManagementAggregate>,
    global: bool,
) -> ExitCode {
    if global {
        return handle_init_global(setup_orchestrator);
    }
    let language = setup_orchestrator.detect_language();
    let language_str = language.value().to_string();

    let target = format!("lint_arwaky.config.{}.yaml", language_str);
    if setup_orchestrator.file_exists(&target) {
        println!("Config already exists: {}", target);
    } else {
        let content = setup_orchestrator.get_config_template(&language_str);
        match setup_orchestrator.write_config_file(&target, content) {
            Ok(desc) => {
                println!("Config created: {} (language: {})", target, language_str);
                println!("  {}", desc.value);
            }
            Err(e) => {
                println!("Error creating config: {e}");
                return ExitCode::from(1);
            }
        }
    }
    ExitCode::SUCCESS
}

fn handle_init_global(setup_orchestrator: Arc<dyn SetupManagementAggregate>) -> ExitCode {
    let config_dir = match setup_orchestrator.create_global_config_dir() {
        Ok(d) => d,
        Err(_) => {
            println!("Error: Could not determine or create XDG config directory");
            return ExitCode::from(1);
        }
    };

    println!("Installing default configs to: {}", config_dir.display());

    let configs = [
        (
            "lint_arwaky.config.rust.yaml",
            setup_orchestrator.get_config_template("rust"),
        ),
        (
            "lint_arwaky.config.python.yaml",
            setup_orchestrator.get_config_template("python"),
        ),
        (
            "lint_arwaky.config.javascript.yaml",
            setup_orchestrator.get_config_template("javascript"),
        ),
    ];

    let mut all_ok = true;
    for (filename, content) in &configs {
        let target = config_dir.join(filename);
        let target_str = target.to_string_lossy();
        if setup_orchestrator.file_exists(&target_str) {
            println!("  {filename} — already exists, skipping");
        } else {
            match setup_orchestrator.write_config_file(&target_str, content) {
                Ok(_) => println!("  {filename} — created"),
                Err(e) => {
                    println!("  {filename} — error: {e}");
                    all_ok = false;
                }
            }
        }
    }
    if all_ok { ExitCode::SUCCESS } else { ExitCode::from(1) }
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
        ExitCode::SUCCESS
    } else {
        println!("Installation failed. Run with `--sudo` if npm globally requires permissions.");
        ExitCode::from(1)
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
    ExitCode::SUCCESS
}

fn which_mcp_binary() -> String {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let sibling = dir.join("lint-arwaky-mcp");
            if sibling.exists() {
                return sibling.to_string_lossy().to_string();
            }
        }
    }
    "lint-arwaky-mcp".to_string()
}
