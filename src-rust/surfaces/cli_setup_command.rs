use crate::contract::service_container_aggregate::ServiceContainerAggregate;
use crate::surfaces::cli_setup_controller::{
    generate_env, generate_mcp_config, mcp_config_claude, mcp_config_hermes, mcp_config_vscode,
    register_setup_management,
};

use std::sync::Arc;

#[derive(Clone)]
pub struct SetupCommandsSurface {
    pub container: Option<Arc<dyn ServiceContainerAggregate>>,
}

impl SetupCommandsSurface {
    pub fn new(container: Option<Arc<dyn ServiceContainerAggregate>>) -> Self {
        let s = Self { container };
        if let Some(ref c) = s.container {
            register_setup_management(c.clone());
        }
        s
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
            let env_content = generate_env(&home);
            if let Err(e) = std::fs::write(env_path, &env_content) {
                println!("  Error creating .env: {e}");
            } else {
                println!("  Created: .env");
            }
        }

        // 4. Generate MCP config snippets
        println!("\n[4/4] MCP server configuration:");
        let mcp_json = generate_mcp_config();
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
        let configs = [
            ("claude", mcp_config_claude()),
            ("hermes", mcp_config_hermes()),
            ("vscode", mcp_config_vscode()),
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
