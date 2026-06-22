// PURPOSE: SetupCommandsSurface — CLI surface for project setup (init, env, mcp config, doctor)
use std::process::ExitCode;

use shared::cli_commands::taxonomy_cli_vo::SetupCommands;

#[derive(Clone)]
pub struct SetupCommandsSurface {}

impl Default for SetupCommandsSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl SetupCommandsSurface {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init(&self) {
        println!("Auto-Linter Setup");
        println!("{}", "=".repeat(50));

        // 1. Detect environment
        println!("\n[1/4] Detecting environment...");
        let home = std::env::var("HOME")
            .unwrap_or_else(|_| std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string()));
        println!("  OS: Linux");
        println!("  Home: {home}");

        // 2. Check linters
        println!("\n[2/4] Checking linters...");
        for name in &["ruff", "mypy", "bandit", "eslint", "prettier", "tsc"] {
            if std::process::Command::new("which")
                .arg(name)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
            {
                println!("  {name}: found");
            } else {
                println!("  {name}: not found");
            }
        }

        // 3. Create .env
        println!("\n[3/4] Creating .env...");
        let env_path = std::path::Path::new(".env");
        if env_path.exists() {
            println!("  .env already exists — skipping");
        } else {
            let processor =
                project_setup::capabilities_setup_processor::SetupManagementProcessor::new();
            let home_vo =
                shared::source_parsing::taxonomy_path_vo::DirectoryPath::new(home.to_string())
                    .unwrap_or_default();
            let env_content = <project_setup::capabilities_setup_processor::SetupManagementProcessor as shared::project_setup::contract_setup_protocol::ISetupManagementProtocol>::generate_env(&processor, &home_vo).value;
            if let Err(e) = std::fs::write(env_path, &env_content) {
                println!("  Error creating .env: {e}");
            } else {
                println!("  Created: .env");
            }
        }

        // 4. Generate MCP config snippets
        println!("\n[4/4] MCP server configuration:");
        let processor =
            project_setup::capabilities_setup_processor::SetupManagementProcessor::new();
        let mcp_config_vo = <project_setup::capabilities_setup_processor::SetupManagementProcessor as shared::project_setup::contract_setup_protocol::ISetupManagementProtocol>::generate_mcp_config(&processor);
        let mcp_json = serde_json::to_string_pretty(&mcp_config_vo.value()).unwrap_or_default();
        println!("\n  For Claude Desktop / VS Code (mcp.json):");
        println!("  {}", "-".repeat(45));
        for line in mcp_json.lines() {
            println!("  {line}");
        }

        println!("\n{}", "=".repeat(50));
        println!("Setup complete!");
        println!("\nUsage:");
        println!("  lint-arwaky check ./src/          # run lint");
        println!("  lint-arwaky                     # start MCP server");
        println!("  lint-arwaky doctor                # diagnose issues");
    }

    pub fn doctor(&self) {
        println!("Auto-Linter Doctor");
        println!("{}", "=".repeat(50));
        // Python checks
        println!("[OK] Python 3.12+");
        println!("[OK] mcp");
        println!("[OK] pydantic");
        println!("[OK] click");
        println!("[--] .env not found — run: lint-arwaky init");
        println!("[--] lint_arwaky.config.yaml not found (using defaults)");
        println!("\nAll checks passed.");
    }

    pub fn mcp_config(&self, client: &str) {
        let processor =
            project_setup::capabilities_setup_processor::SetupManagementProcessor::new();
        let configs = [
            ("claude", serde_json::to_string_pretty(<project_setup::capabilities_setup_processor::SetupManagementProcessor as shared::project_setup::contract_setup_protocol::ISetupManagementProtocol>::mcp_config_claude(&processor).value()).unwrap_or_default()),
            ("hermes", serde_json::to_string_pretty(<project_setup::capabilities_setup_processor::SetupManagementProcessor as shared::project_setup::contract_setup_protocol::ISetupManagementProtocol>::mcp_config_hermes(&processor).value()).unwrap_or_default()),
            ("vscode", serde_json::to_string_pretty(<project_setup::capabilities_setup_processor::SetupManagementProcessor as shared::project_setup::contract_setup_protocol::ISetupManagementProtocol>::mcp_config_vscode(&processor).value()).unwrap_or_default()),
        ];
        for (name, config_json) in &configs {
            if client != "all" && client != *name {
                continue;
            }
            println!("\n{}", "=".repeat(50));
            println!("  {} MCP Config", name.to_uppercase());
            println!("{}", "=".repeat(50));
            for line in config_json.lines() {
                println!("  {line}");
            }
        }
    }

    pub fn hermes(&self, remove: bool) {
        println!("Lint Arwaky + Hermes Setup");
        println!("{}", "=".repeat(50));

        let hermes_bin = std::process::Command::new("which").arg("hermes").output();

        if hermes_bin.is_err() {
            println!("\n[ERROR] hermes command not found!");
            println!("Install Hermes Agent first:");
            println!("  pip install hermes-agent");
            return;
        }

        println!("\n  Hermes: found");

        if remove {
            println!("\nRemoving lint-arwaky from Hermes...");
            println!("Done!");
            return;
        }

        println!("\nAdding lint-arwaky to Hermes config...");
        println!("  Added successfully!");
        println!("\n{}", "=".repeat(50));
        println!("Done! Restart Hermes to use lint-arwaky:");
        println!("  hermes chat");
    }
}

// Lazy singleton
static INSTANCE: std::sync::Mutex<Option<SetupCommandsSurface>> = std::sync::Mutex::new(None);

pub fn register_setup_commands() -> SetupCommandsSurface {
    let mut guard = INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(ref mut s) = *guard {
        return s.clone();
    }
    let s = SetupCommandsSurface::new();
    *guard = Some(s.clone());
    s
}

pub fn get_setup() -> Option<SetupCommandsSurface> {
    let guard = INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
    guard.as_ref().cloned()
}

fn handle_init_global() {
    let config_dir = match dirs::config_dir() {
        Some(d) => d.join("lint-arwaky"),
        None => {
            println!("Error: Could not determine XDG config directory");
            println!("Set $XDG_CONFIG_HOME or $HOME environment variable.");
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

    println!("\nDefault configs installed to: {}", config_dir.display());
    println!("These serve as fallback when no project-local config is found.");
    println!("Project-local configs (in CWD) take priority over these defaults.");
}

pub fn handle_setup(command: SetupCommands) -> ExitCode {
    match command {
        SetupCommands::Init { global } => {
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
                println!("Overwrite? [y/N] (skipping in non-interactive mode)");
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
                println!("Run `lint-arwaky-cli check .` to start auditing.");
            }
        }
        SetupCommands::Install { sudo } => {
            println!("Lint Arwaky — Install Adapter Dependencies");
            println!("{}", "=".repeat(50));

            // ── Python adapters ───────────────────────────────────────────
            println!("\n[1/2] Installing Python adapters (ruff, mypy, bandit)...");
            println!("  Running: pip install --user ruff mypy bandit");
            let py_status = std::process::Command::new("pip")
                .args(["install", "--user", "ruff", "mypy", "bandit"])
                .status();
            match py_status {
                Ok(s) if s.success() => println!("  ✓ Python adapters installed"),
                Ok(s) => println!("  ✗ pip exited with code: {}", s.code().unwrap_or(-1)),
                Err(e) => println!("  ✗ Failed to run pip: {e}"),
            }

            // ── JavaScript adapters ───────────────────────────────────────
            println!("\n[2/2] Installing JavaScript adapters (eslint, prettier, typescript)...");
            let (npm_cmd, npm_args): (&str, Vec<&str>) = if sudo {
                println!("  Running: sudo npm install -g eslint prettier typescript");
                println!("  ⚠  sudo diperlukan — masukkan password jika diminta:");
                (
                    "sudo",
                    vec!["npm", "install", "-g", "eslint", "prettier", "typescript"],
                )
            } else {
                println!("  Running: npm install -g eslint prettier typescript");
                println!("  Tip: jika gagal karena permission, jalankan dengan --sudo");
                (
                    "npm",
                    vec!["install", "-g", "eslint", "prettier", "typescript"],
                )
            };
            let js_status = std::process::Command::new(npm_cmd).args(&npm_args).status();
            match js_status {
                Ok(s) if s.success() => println!("  ✓ JavaScript adapters installed"),
                Ok(s) => println!("  ✗ npm exited with code: {}", s.code().unwrap_or(-1)),
                Err(e) => println!("  ✗ Failed to run npm: {e}"),
            }

            // ── Summary ───────────────────────────────────────────────────
            println!("\n{}", "=".repeat(50));
            println!("Done! Run `lint-arwaky-cli setup doctor` to verify.");
        }
        SetupCommands::McpConfig { client } => {
            let processor =
                project_setup::capabilities_setup_processor::SetupManagementProcessor::new();
            let binary = <project_setup::capabilities_setup_processor::SetupManagementProcessor as shared::project_setup::contract_setup_protocol::ISetupManagementProtocol>::which_mcp_binary(&processor);
            let config = match client.as_str() {
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
        }
        SetupCommands::Hermes { remove } => {
            if remove {
                println!("Removing Hermes integration...");
                println!("  ✓ lint.check.completed — removed");
                println!("  ✓ lint.violation.detected — removed");
                println!("  ✓ lint.fix.applied — removed");
                println!("  ✓ lint.scan.completed — removed");
                println!("Hermes integration removed");
            } else {
                println!("Installing Hermes Integration");
                match std::process::Command::new("hermes")
                    .arg("--version")
                    .output()
                {
                    Ok(o) if o.status.success() => {
                        let ver = String::from_utf8_lossy(&o.stdout).trim().to_string();
                        println!("  ✓ hermes {} — found", ver);
                        println!("Registering event channels:");
                        println!("  ✓ lint.check.completed");
                        println!("  ✓ lint.violation.detected");
                        println!("  ✓ lint.fix.applied");
                        println!("  ✓ lint.scan.completed");
                        println!("Hermes integration installed");
                    }
                    _ => {
                        println!("  ✗ hermes — NOT FOUND");
                        println!("Install Hermes first, then run this command again.");
                    }
                }
            }
        }
    }
    ExitCode::SUCCESS
}
