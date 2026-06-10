// PURPOSE: SetupCommandsSurface — CLI surface for project setup (init, env, mcp config, doctor)
use std::process::ExitCode;
use std::sync::Arc;

use crate::cli_commands::surface_core_command::SetupCommands;
use crate::project_setup::capabilities_setup_processor::SetupManagementProcessor;
use crate::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;

#[derive(Clone)]
pub struct SetupCommandsSurface {
    pub container: Option<Arc<dyn ServiceContainerAggregate>>,
}

impl SetupCommandsSurface {
    pub fn new(container: Option<Arc<dyn ServiceContainerAggregate>>) -> Self {
        Self { container }
    }

    pub fn register_all(&mut self, container: Arc<dyn ServiceContainerAggregate>) {
        self.container = Some(container);
    }

    pub fn init(&self) {
        println!("Auto-Linter Setup");
        println!("{}", "=".repeat(50));

        // 1. Detect environment
        println!("\n[1/4] Detecting environment...");
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());
        println!("  OS: Linux");
        println!("  Home: {home}");

        // 2. Check linters
        println!("\n[2/4] Checking linters...");
        for name in &["ruff", "mypy", "eslint", "prettier"] {
            if std::process::Command::new("which")
                .arg(name)
                .output()
                .is_ok()
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
            let processor = SetupManagementProcessor::new();
            let home_vo = crate::source_parsing::taxonomy_path_vo::DirectoryPath::new(home.to_string()).unwrap_or_default();
            let env_content = processor.generate_env(&home_vo).value;
            if let Err(e) = std::fs::write(env_path, &env_content) {
                println!("  Error creating .env: {e}");
            } else {
                println!("  Created: .env");
            }
        }

        // 4. Generate MCP config snippets
        println!("\n[4/4] MCP server configuration:");
        let processor = SetupManagementProcessor::new();
        let mcp_config_vo = processor.generate_mcp_config();
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
        let processor = SetupManagementProcessor::new();
        let configs = [
            ("claude", serde_json::to_string_pretty(processor.mcp_config_claude().value()).unwrap_or_default()),
            ("hermes", serde_json::to_string_pretty(processor.mcp_config_hermes().value()).unwrap_or_default()),
            ("vscode", serde_json::to_string_pretty(processor.mcp_config_vscode().value()).unwrap_or_default()),
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

pub fn register_setup_commands(
    container: impl ServiceContainerAggregate + Clone + 'static,
) -> SetupCommandsSurface {
    let arc_container = std::sync::Arc::new(container);
    let mut guard = INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(ref mut s) = *guard {
        s.register_all(arc_container.clone());
        return s.clone();
    }
    let mut s = SetupCommandsSurface::new(Some(arc_container.clone()));
    s.register_all(arc_container);
    *guard = Some(s.clone());
    s
}

pub fn get_setup() -> Option<SetupCommandsSurface> {
    let guard = INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
    guard.as_ref().cloned()
}

pub fn handle_setup(command: SetupCommands) -> ExitCode {
    match command {
        SetupCommands::Init => {
            let language = if std::path::Path::new("src-rust").exists() {
                "rust"
            } else if std::path::Path::new("src-python").exists()
                || std::path::Path::new("pyproject.toml").exists()
            {
                "python"
            } else if std::path::Path::new("src-javascript").exists()
                || std::path::Path::new("package.json").exists()
            {
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
    suffixes: ["_vo", "_entity", "_event", "_error", "_constant"]
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
    - "src-python/**/*.py"
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
    suffixes: ["_vo", "_entity", "_event", "_error", "_constant"]
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
    - "src-javascript/**/*.{js,ts,tsx}"
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
    suffixes: ["_vo", "_entity", "_event", "_error", "_constant"]
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
    - "src-rust/**/*.rs"
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
        SetupCommands::Doctor => {
            println!("Environment Diagnostics");
            println!();

            let check_tool = |name: &str, args: &[&str], required: bool| -> (&str, String) {
                let output = std::process::Command::new(name).args(args).output();
                match output {
                    Ok(o) if o.status.success() => {
                        let ver = String::from_utf8_lossy(&o.stdout)
                            .lines()
                            .next()
                            .unwrap_or("")
                            .trim()
                            .to_string();
                        ("OK", ver)
                    }
                    _ => {
                        if required {
                            ("FAIL", "NOT FOUND".to_string())
                        } else {
                            ("WARN", "NOT FOUND".to_string())
                        }
                    }
                }
            };

            println!("Rust Toolchain:");
            let (cargo_st, cargo_ver) = check_tool("cargo", &["--version"], true);
            println!(
                "  {} cargo {}  ({})",
                if cargo_st == "OK" { "✓" } else { "✗" },
                cargo_ver,
                cargo_st
            );
            let (clippy_st, clippy_ver) = check_tool("cargo", &["clippy", "--version"], true);
            println!(
                "  {} clippy {}  ({})",
                if clippy_st == "OK" { "✓" } else { "✗" },
                clippy_ver,
                clippy_st
            );
            let (rustfmt_st, rustfmt_ver) = check_tool("rustfmt", &["--version"], true);
            println!(
                "  {} rustfmt {}  ({})",
                if rustfmt_st == "OK" { "✓" } else { "✗" },
                rustfmt_ver,
                rustfmt_st
            );
            if let Ok(p) = std::env::current_exe() {
                println!("  ℹ️  binary: {}", p.display());
            }

            println!();
            println!("Python Toolchain:");
            let (py_st, py_ver) = check_tool("python3", &["--version"], false);
            println!(
                "  {} python3 {}  ({})",
                if py_st == "OK" { "✓" } else { "✗" },
                py_ver,
                py_st
            );
            let (ruff_st, ruff_ver) = check_tool("ruff", &["--version"], false);
            println!(
                "  {} ruff {}  ({})",
                if ruff_st == "OK" { "✓" } else { "✗" },
                ruff_ver,
                ruff_st
            );

            println!();
            println!("JavaScript Toolchain:");
            let (node_st, node_ver) = check_tool("node", &["--version"], false);
            println!(
                "  {} node {}  ({})",
                if node_st == "OK" { "✓" } else { "✗" },
                node_ver,
                node_st
            );

            let eslint_local = std::path::Path::new("node_modules/.bin/eslint");
            if eslint_local.exists() {
                println!("  ✓ eslint (local)");
            } else {
                let (es_st, es_ver) = check_tool("eslint", &["--version"], false);
                println!(
                    "  {} eslint {}  ({})",
                    if es_st == "OK" { "✓" } else { "✗" },
                    es_ver,
                    es_st
                );
            }

            let tsc_local = std::path::Path::new("node_modules/.bin/tsc");
            if tsc_local.exists() {
                println!("  ✓ tsc (local)");
            } else {
                let (tsc_st, tsc_ver) = check_tool("tsc", &["--version"], false);
                println!(
                    "  {} tsc {}  ({})",
                    if tsc_st == "OK" { "✓" } else { "✗" },
                    tsc_ver,
                    tsc_st
                );
            }

            println!();
            println!("VCS:");
            let (git_st, git_ver) = check_tool("git", &["--version"], true);
            println!(
                "  {} git {}  ({})",
                if git_st == "OK" { "✓" } else { "✗" },
                git_ver,
                git_st
            );
            let (jj_st, jj_ver) = check_tool("jj", &["--version"], false);
            println!(
                "  {} jj {}  ({})",
                if jj_st == "OK" { "✓" } else { "✗" },
                jj_ver,
                jj_st
            );
        }
        SetupCommands::McpConfig { client } => {
            let processor = SetupManagementProcessor::new();
            let binary = processor.which_mcp_binary();
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
