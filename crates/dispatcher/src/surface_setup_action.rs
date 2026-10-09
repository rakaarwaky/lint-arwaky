// PURPOSE: SetupCommandsSurface — project setup business logic, no formatting.
// handle_install delegates to ISetupAggregate.
// No direct std::process::Command calls.
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::taxonomy_filesystem_request::FilesystemRequest;
use shared_project_setup::SetupRequest;
use shared_project_setup::{ISetupAggregate, ProjectLanguagesVO};
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
    setup_orchestrator: Arc<dyn ISetupAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
) -> Vec<SetupInitItem> {
    let mut items: Vec<SetupInitItem> = Vec::new();

    let languages = setup_orchestrator
        .execute(SetupRequest::detect_languages())
        .into_languages();
    let target = "lint_arwaky.config.yaml";

    // Write unified config once — all languages share the same template
    let first_lang = languages.iter().next().map(|l| l.value().to_string());
    let lang_str = first_lang.as_deref().unwrap_or("all");
    let content = match setup_orchestrator
        .execute(SetupRequest::get_config_template(lang_str))
        .into_template()
    {
        Ok(c) => c,
        Err(e) => {
            items.push(SetupInitItem {
                message: format!("No config template: {e}"),
                ok: false,
            });
            return items;
        }
    };
    match setup_orchestrator
        .execute(SetupRequest::write_config_file(target, &content))
        .into_write_result()
    {
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
    let doc_files = ["ARCHITECTURE.md", "RULES_AES.md"];
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
            let xdg_content = filesystem
                .execute(FilesystemRequest::read_file_result(&xdg_src))
                .into_content()
                .value;
            if xdg_content.is_empty() {
                items.push(SetupInitItem {
                    message: format!("  {doc} — could not be read from XDG config"),
                    ok: false,
                });
                continue;
            }
            match setup_orchestrator
                .execute(SetupRequest::write_config_file(doc, &xdg_content))
                .into_write_result()
            {
                Ok(_) => items.push(SetupInitItem {
                    message: format!("  {doc} — copied/overwritten from XDG config"),
                    ok: true,
                }),
                Err(e) => items.push(SetupInitItem {
                    message: format!("  {doc} — error: {e}"),
                    ok: false,
                }),
            }
        }

        // Copy .agents/ from XDG config to current project.
        // skills are embedded binary constants (installed separately below);
        // prompts are intentionally not distributed to target projects.
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

    // Install embedded skills from binary constants (filtered by detected languages)
    let embedded_skills = setup_orchestrator
        .execute(SetupRequest::get_embedded_skills())
        .into_skills();
    let mut installed_count = 0;
    let mut install_failed = false;
    let skills_root = std::path::Path::new(".agents").join("skills");

    for skill in embedded_skills {
        if is_skill_relevant_for_languages(skill.language, &languages) {
            let target_file = skills_root.join(skill.relative_path);
            if let Some(parent) = target_file.parent() {
                let dir_ok = filesystem
                    .execute(FilesystemRequest::create_dir_all(parent))
                    .into_op_ok();
                if !dir_ok {
                    items.push(SetupInitItem {
                        message: format!("  .agents/skills/ — directory error for {}", skill.name),
                        ok: false,
                    });
                    install_failed = true;
                    continue;
                }
            }
            let write_ok = filesystem
                .execute(FilesystemRequest::write_file(&target_file, skill.content))
                .into_op_ok();
            if write_ok {
                installed_count += 1;
            } else {
                items.push(SetupInitItem {
                    message: format!("  .agents/skills/ — write error for {}", skill.name),
                    ok: false,
                });
                install_failed = true;
            }
        }
    }

    let detected_names: Vec<&str> = languages.iter().map(|l| l.value()).collect();
    let lang_summary = if detected_names.is_empty() {
        "all / default".to_string()
    } else {
        detected_names.join(", ")
    };

    if !install_failed {
        items.push(SetupInitItem {
            message: format!(
                "  .agents/skills/ — installed {installed_count} skill file(s) for detected language(s) [{lang_summary}]"
            ),
            ok: true,
        });
    }

    items
}

/// Determine whether a skill is relevant given the detected project languages.
/// If skill_language is None (language-agnostic), always returns true.
/// If no languages are detected in the project, returns true as default.
/// Otherwise, checks if the skill language matches any detected language.
pub fn is_skill_relevant_for_languages(
    skill_language: Option<&str>,
    detected_languages: &ProjectLanguagesVO,
) -> bool {
    let Some(lang) = skill_language else {
        return true;
    };

    if detected_languages.is_empty() {
        return true;
    }

    match lang {
        "python" => detected_languages.iter().any(|l| l.value() == "python"),
        "rust" => detected_languages.iter().any(|l| l.value() == "rust"),
        "typescript" | "javascript" => detected_languages
            .iter()
            .any(|l| l.value() == "javascript" || l.value() == "typescript"),
        _ => false,
    }
}

fn copy_dir_all(
    src: &std::path::Path,
    dst: &std::path::Path,
    fs: &dyn IFilesystemAggregate,
) -> Result<usize, String> {
    if !fs
        .execute(FilesystemRequest::create_dir_all(dst))
        .into_op_ok()
    {
        return Err(format!("failed to create dir: {}", dst.display()));
    }
    let entries: Vec<std::path::PathBuf> = fs
        .execute(FilesystemRequest::read_dir_entries(src))
        .into_paths()
        .into_iter()
        .map(std::path::PathBuf::from)
        .collect();
    let mut count = 0;
    for entry in entries {
        let file_name = entry.file_name().unwrap_or_default();
        if file_name == "skills" || file_name == "prompts" {
            continue;
        }
        let dst_path = dst.join(file_name);
        if entry.is_dir() {
            count += copy_dir_all(&entry, &dst_path, fs)?;
        } else {
            if !fs
                .execute(FilesystemRequest::copy_file(&entry, &dst_path))
                .into_op_ok()
            {
                return Err(format!("failed to copy: {}", entry.display()));
            }
            count += 1;
        }
    }
    Ok(count)
}

pub fn collect_install(setup: Arc<dyn ISetupAggregate>, sudo: bool) -> InstallReport {
    let py_ok = setup
        .execute(SetupRequest::install_python_adapters())
        .into_status()
        .value;
    let js_ok = setup
        .execute(SetupRequest::install_javascript_adapters(sudo))
        .into_status()
        .value;
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
///   2. CARGO_HOME/bin/lint-arwaky-mcp
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

    // current_exe() is forbidden (subprocess-adjacent architecture rule);
    // use CARGO_HOME/bin as the canonical install location instead.
    let cargo_home = std::env::var_os("CARGO_HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| dirs::home_dir().map(|h| h.join(".cargo")));
    if let Some(dir) = cargo_home {
        let candidate = dir.join("bin").join("lint-arwaky-mcp");
        if candidate.is_file() {
            return candidate
                .canonicalize()
                .map_err(|e| format!("cannot canonicalize CARGO_HOME bin: {e}"));
        }
    }

    Err("lint-arwaky-mcp not found. Set LINT_ARWAKY_MCP_BIN to an absolute path.".into())
}
