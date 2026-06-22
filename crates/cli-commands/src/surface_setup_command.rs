// PURPOSE: SetupCommandsSurface — CLI surface for project setup (init, install, mcp-config)
use std::process::ExitCode;

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
            "python" => {
                r#"# Lint Arwaky Configuration — Python
project:
  language: python
  name: my-project
  version: 1.0.0
ci:
  threshold: 70
layers:
  taxonomy:
    prefixes: ["taxonomy_"]
    suffixes: ["_vo", "_entity", "_event", "_error", "_constant", "_util", "_helper"]
  contract:
    prefixes: ["contract_"]
    suffixes: ["_port", "_protocol", "_aggregate"]
  capabilities:
    prefixes: ["capabilities_"]
    suffixes: ["_checker", "_analyzer", "_processor"]
  infrastructure:
    prefixes: ["infrastructure_"]
    suffixes: ["_adapter", "_provider", "_scanner"]
  agent:
    prefixes: ["agent_"]
    suffixes: ["_container", "_orchestrator", "_lifecycle"]
  surface:
    prefixes: ["surface_"]
    suffixes: ["_command", "_handler", "_controller"]
source:
  include:
    - "packages/**/*.py"
    - "modules/**/*.py"
  exclude:
    - "**/.venv/**"
    - "**/vendor/**"
"#
            }
            "javascript" => {
                r#"# Lint Arwaky Configuration — JavaScript/TypeScript
project:
  language: javascript
  name: my-project
  version: 1.0.0
ci:
  threshold: 70
layers:
  taxonomy:
    prefixes: ["taxonomy_"]
    suffixes: ["_vo", "_entity", "_event", "_error", "_constant", "_util", "_helper"]
  contract:
    prefixes: ["contract_"]
    suffixes: ["_port", "_protocol", "_aggregate"]
  capabilities:
    prefixes: ["capabilities_"]
    suffixes: ["_checker", "_analyzer", "_processor"]
  infrastructure:
    prefixes: ["infrastructure_"]
    suffixes: ["_adapter", "_provider", "_scanner"]
  agent:
    prefixes: ["agent_"]
    suffixes: ["_container", "_orchestrator", "_lifecycle"]
  surface:
    prefixes: ["surface_"]
    suffixes: ["_command", "_handler", "_controller"]
source:
  include:
    - "packages/**/*.{js,ts,tsx}"
    - "modules/**/*.{js,ts,tsx}"
  exclude:
    - "**/node_modules/**"
    - "**/dist/**"
"#
            }
            _ => {
                r#"# Lint Arwaky Configuration — Rust
project:
  language: rust
  name: my-project
  version: 1.0.0
ci:
  threshold: 70
layers:
  taxonomy:
    prefixes: ["taxonomy_"]
    suffixes: ["_vo", "_entity", "_event", "_error", "_constant", "_util", "_helper"]
  contract:
    prefixes: ["contract_"]
    suffixes: ["_port", "_protocol", "_aggregate"]
  capabilities:
    prefixes: ["capabilities_"]
    suffixes: ["_checker", "_analyzer", "_processor"]
  infrastructure:
    prefixes: ["infrastructure_"]
    suffixes: ["_adapter", "_provider", "_scanner"]
  agent:
    prefixes: ["agent_"]
    suffixes: ["_container", "_orchestrator", "_lifecycle"]
  surface:
    prefixes: ["surface_"]
    suffixes: ["_command", "_handler", "_controller"]
source:
  include:
    - "crates/**/*.rs"
    - "modules/**/*.rs"
  exclude:
    - "**/target/**"
    - "**/vendor/**"
"#
            }
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

pub fn handle_install(sudo: bool) -> ExitCode {
    println!("Lint Arwaky — Install Adapter Dependencies");
    println!("{}", "=".repeat(50));

    println!("\n[1/2] Installing Python adapters (ruff, mypy, bandit)...");
    let py_status = std::process::Command::new("pip")
        .args(["install", "--user", "ruff", "mypy", "bandit"])
        .status();
    match py_status {
        Ok(s) if s.success() => println!("  Python adapters installed"),
        Ok(s) => println!("  pip exited with code: {}", s.code().unwrap_or(-1)),
        Err(e) => println!("  Failed to run pip: {e}"),
    }

    println!("\n[2/2] Installing JavaScript adapters (eslint, prettier, typescript)...");
    let (npm_cmd, npm_args): (&str, Vec<&str>) = if sudo {
        ("sudo", vec!["npm", "install", "-g", "eslint", "prettier", "typescript"])
    } else {
        ("npm", vec!["install", "-g", "eslint", "prettier", "typescript"])
    };
    let js_status = std::process::Command::new(npm_cmd).args(&npm_args).status();
    match js_status {
        Ok(s) if s.success() => println!("  JavaScript adapters installed"),
        Ok(s) => println!("  npm exited with code: {}", s.code().unwrap_or(-1)),
        Err(e) => println!("  Failed to run npm: {e}"),
    }

    println!("\n{}", "=".repeat(50));
    println!("Done! Run `lint-arwaky doctor` to verify.");
    ExitCode::SUCCESS
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
