// PURPOSE: SetupCommandsSurface — CLI surface for project setup (init, install, mcp-config)
//
// Three subcommands:
//   - init:        writes lint_arwaky.config.<lang>.yaml (local), distributes docs + .agents/* from XDG config
//   - install:     pip install Python adapters (ruff, mypy, bandit) + npm install JS adapters (eslint, prettier, typescript)
//   - mcp-config:  prints MCP client config JSON for Claude/Cursor/Windsurf/Copilot
//
// Binary resolution for mcp-config: checks sibling of current exe first, falls back to PATH.
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use std::process::ExitCode;
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
    ];
    if let Some(config_dir) = dirs::config_dir() {
        let xdg_base = config_dir.join("lint-arwaky");
        for doc in &doc_files {
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

        // 3. Distribute .agents/skills/ and .agents/rules/ from XDG config to project
        let agents_src = xdg_base.join(".agents");
        if agents_src.is_dir() {
            let current_dir = match std::env::current_dir() {
                Ok(d) => d,
                Err(e) => {
                    println!("Error: could not determine current directory: {e}");
                    return ExitCode::from(1);
                }
            };

            // Copy .agents/skills/* (overwrite)
            let skills_dir = agents_src.join("skills");
            if skills_dir.is_dir() {
                if let Ok(entries) = std::fs::read_dir(&skills_dir) {
                    for entry in entries.filter_map(|e| e.ok()) {
                        let skill_name_os = entry.file_name();
                        let src = skills_dir.join(&skill_name_os);
                        let dst = current_dir
                            .join(".agents")
                            .join("skills")
                            .join(&skill_name_os);

                        if src.is_dir() {
                            match copy_dir_recursive(&src, &dst) {
                                Ok(_) => println!(
                                    "  .agents/skills/{} — distributed from XDG config",
                                    skill_name_os.to_string_lossy()
                                ),
                                Err(e) => println!(
                                    "  .agents/skills/{} — error: {e}",
                                    skill_name_os.to_string_lossy()
                                ),
                            }
                        }
                    }
                }
            }

            // Copy .agents/rules/* (overwrite)
            let rules_dir = agents_src.join("rules");
            if rules_dir.is_dir() {
                if let Ok(entries) = std::fs::read_dir(&rules_dir) {
                    for entry in entries.filter_map(|e| e.ok()) {
                        let rule_name_os = entry.file_name();
                        let src = rules_dir.join(&rule_name_os);
                        let dst = current_dir
                            .join(".agents")
                            .join("rules")
                            .join(&rule_name_os);

                        if src.is_file() {
                            match std::fs::copy(&src, &dst) {
                                Ok(_) => println!(
                                    "  .agents/rules/{} — distributed from XDG config",
                                    rule_name_os.to_string_lossy()
                                ),
                                Err(e) => println!(
                                    "  .agents/rules/{} — error: {e}",
                                    rule_name_os.to_string_lossy()
                                ),
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!("Warning: could not determine XDG config dir");
    }

    if all_ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

/// Recursively copy a directory from `src` to `dst`.
fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> Result<(), String> {
    std::fs::create_dir_all(dst).map_err(|e| e.to_string())?;
    for entry in std::fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(src_path, dst_path).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
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
