// PURPOSE: SetupCommandsSurface — project setup business logic, no formatting.
// handle_install delegates to SetupManagementAggregate.
// No direct std::process::Command calls.
use shared::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol;
use shared::project_setup::SetupManagementAggregate;
use std::sync::Arc;

/// One setup step outcome — message + success flag for CLI rendering.
#[derive(Debug, Clone)]
pub struct SetupInitItem {
    pub message: String,
    pub ok: bool,
}

/// Adapter installation outcome.
#[derive(Debug, Clone, Copy)]
pub struct InstallReport {
    pub py_ok: bool,
    pub js_ok: bool,
}

/// MCP client config snippet.
#[derive(Debug, Clone)]
pub struct McpConfigReport {
    pub client: String,
    pub binary: String,
    pub config_json: String,
}

pub fn collect_init(
    setup_orchestrator: Arc<dyn SetupManagementAggregate>,
    filesystem: Arc<dyn IFileSystemIOProtocol>,
) -> Vec<SetupInitItem> {
    let mut items: Vec<SetupInitItem> = Vec::new();

    let languages = setup_orchestrator.detect_languages();
    let target = "lint_arwaky.config.yaml";

    // Write unified config once — all languages share the same template
    let first_lang = languages.iter().next().map(|l| l.value().to_string());
    let lang_str = first_lang.as_deref().unwrap_or("all");
    let content = match setup_orchestrator.get_config_template(lang_str) {
        Ok(c) => c,
        Err(e) => {
            items.push(SetupInitItem {
                message: format!("No config template: {e}"),
                ok: false,
            });
            return items;
        }
    };
    match setup_orchestrator.write_config_file(target, content) {
        Ok(desc) => {
            items.push(SetupInitItem {
                message: format!(
                    "Config written/overwritten: {} (unified) — {}",
                    target, desc.value
                ),
                ok: true,
            });
        }
        Err(e) => {
            items.push(SetupInitItem {
                message: format!("Error creating config: {e}"),
                ok: false,
            });
        }
    }

    // Distribute docs from XDG config to project (always overwrite)
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
                items.push(SetupInitItem {
                    message: format!("  {doc} — not in XDG config, skipping"),
                    ok: true,
                });
                continue;
            }
            match filesystem.read_to_string(&xdg_src) {
                Ok(content) => match setup_orchestrator.write_config_file(doc, &content.value) {
                    Ok(_) => items.push(SetupInitItem {
                        message: format!("  {doc} — copied/overwritten from XDG config"),
                        ok: true,
                    }),
                    Err(e) => items.push(SetupInitItem {
                        message: format!("  {doc} — error: {e}"),
                        ok: false,
                    }),
                },
                Err(e) => items.push(SetupInitItem {
                    message: format!("  {doc} — read error: {e}"),
                    ok: false,
                }),
            }
        }

        // Copy .agents/ from XDG config to current project
        let xdg_agents = xdg_base.join(".agents");
        if xdg_agents.exists() && xdg_agents.is_dir() {
            let target_agents = std::path::Path::new(".agents");
            match copy_dir_all(&xdg_agents, target_agents, &*filesystem) {
                Ok(count) => {
                    items.push(SetupInitItem {
                        message: format!(
                            "  .agents/ — copied/overwritten {count} file(s) from XDG config"
                        ),
                        ok: true,
                    });
                }
                Err(e) => {
                    items.push(SetupInitItem {
                        message: format!("  .agents/ — copy error: {e}"),
                        ok: false,
                    });
                }
            }
        } else {
            items.push(SetupInitItem {
                message: "  .agents/ — not in XDG config, skipping".to_string(),
                ok: true,
            });
        }
    } else {
        items.push(SetupInitItem {
            message: "Warning: could not determine XDG config dir".to_string(),
            ok: false,
        });
    }

    items
}

fn copy_dir_all(
    src: &std::path::Path,
    dst: &std::path::Path,
    fs: &dyn IFileSystemIOProtocol,
) -> std::io::Result<usize> {
    fs.create_dir_all(dst)?;
    let mut count = 0;
    for entry_path in fs.read_dir_entries_as_pathbuf(src)? {
        let file_name = entry_path.file_name().unwrap_or_default();
        let dst_path = dst.join(file_name);
        if entry_path.is_dir() {
            count += copy_dir_all(&entry_path, &dst_path, fs)?;
        } else {
            fs.copy_file(&entry_path, &dst_path)?;
            count += 1;
        }
    }
    Ok(count)
}

pub fn collect_install(setup: Arc<dyn SetupManagementAggregate>, sudo: bool) -> InstallReport {
    let py_ok = setup.install_python_adapters().value;
    let js_ok = setup.install_javascript_adapters(sudo).value;
    InstallReport { py_ok, js_ok }
}

pub fn collect_mcp_config(client: &str) -> McpConfigReport {
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
        "hermes" | "vscode" | "all" => serde_json::json!({
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
    McpConfigReport {
        client: client.to_string(),
        binary,
        config_json: json_str,
    }
}

fn which_mcp_binary() -> String {
    match resolve_mcp_binary() {
        Ok(path) => path.to_string_lossy().into_owned(),
        Err(_) => "lint-arwaky-mcp".to_string(),
    }
}

/// Resolve the MCP binary to an absolute canonicalized path.
/// Resolution order:
///   1. LINT_ARWAKY_MCP_BIN env var
///   2. Sibling of current executable
///   3. Fail closed — no bare PATH fallback
fn resolve_mcp_binary() -> Result<std::path::PathBuf, String> {
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

    Err("lint-arwaky-mcp not found. Set LINT_ARWAKY_MCP_BIN to an absolute path.".into())
}
