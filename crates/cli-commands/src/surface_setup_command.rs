// PURPOSE: SetupCommandsSurface — CLI surface for project setup (init, install, mcp-config)
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use std::process::ExitCode;
use std::sync::Arc;

pub fn handle_init(global: bool) -> ExitCode {
    if global {
        handle_init_global();
        return ExitCode::SUCCESS;
    }
    let language = if std::path::Path::new("crates").exists() {
        "rust"
    } else if std::path::Path::new("packages").exists()
        || std::path::Path::new("modules").exists()
        || std::path::Path::new("pyproject.toml").exists()
    {
        "python"
    } else if std::path::Path::new("package.json").exists() {
        "javascript"
    } else {
        "rust"
    };

    let target = std::path::PathBuf::from(format!("lint_arwaky.config.{}.yaml", language));
    if target.exists() {
        println!("Config already exists: {}", target.display());
    } else {
        let content = match language {
            "rust" => include_str!("../../../lint_arwaky.config.rust.yaml"),
            "python" => include_str!("../../../lint_arwaky.config.python.yaml"),
            "javascript" => include_str!("../../../lint_arwaky.config.javascript.yaml"),
            _ => include_str!("../../../lint_arwaky.config.rust.yaml"),
        };

        let _ = std::fs::write(&target, content);
        println!(
            "Config created: {} (language: {})",
            target.display(),
            language
        );
    }
    ExitCode::SUCCESS
}

fn handle_init_global() {
    let config_dir = match dirs::config_dir() {
        Some(d) => d.join("lint-arwaky"),
        None => {
            println!("Error: Could not determine XDG config directory");
            return;
        }
    };

    println!("Installing default configs to: {}", config_dir.display());

    if let Err(e) = std::fs::create_dir_all(&config_dir) {
        println!("Error creating directory: {e}");
        return;
    }

    let configs = [
        (
            "lint_arwaky.config.rust.yaml",
            include_str!("../../../lint_arwaky.config.rust.yaml"),
        ),
        (
            "lint_arwaky.config.python.yaml",
            include_str!("../../../lint_arwaky.config.python.yaml"),
        ),
        (
            "lint_arwaky.config.javascript.yaml",
            include_str!("../../../lint_arwaky.config.javascript.yaml"),
        ),
    ];

    for (filename, content) in &configs {
        let target = config_dir.join(filename);
        if target.exists() {
            println!("  {filename} — already exists, skipping");
        } else {
            match std::fs::write(&target, content) {
                Ok(()) => println!("  {filename} — created"),
                Err(e) => println!("  {filename} — error: {e}"),
            }
        }
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
