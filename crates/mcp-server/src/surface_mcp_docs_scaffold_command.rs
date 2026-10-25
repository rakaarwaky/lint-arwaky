// PURPOSE: McpDocsScaffoldSurface — `docs` and `docs scaffold` MCP command wiring.
//
// Holds the two doc-domain handlers that used to inline every audit/scaffold
// JSON response in `surface_mcp_action_command.rs`, plus `handle_read_skill`
// and its `extract_skill_section` helper. The parent surface delegates here
// so its response-building logic stays in one place.

use shared_filesystem::FilesystemRequest;

use crate::surface_mcp_action_command::McpActionSurface;

/// MCP surface for the `docs`, `docs_scaffold`, and `read_skill` actions.
///
/// Shares `McpActionSurface` dependencies; no state of its own.
pub struct McpDocsScaffoldSurface;

impl McpDocsScaffoldSurface {
    /// Run docs audit via dispatcher.
    pub async fn execute_docs(&self, surface: &McpActionSurface, path: &str) -> serde_json::Value {
        let fp = match surface.to_fp(path) {
            Ok(fp) => fp,
            Err(error) => return error,
        };
        let doc = surface.deps.doc_orchestrator.clone();
        let fs_seam = surface.deps.fs_seam.clone();
        let config_file_reader = surface.deps.config_file_reader.clone();
        let result = match McpActionSurface::run_blocking(move || {
            dispatcher::surface_docs_action::collect_docs(
                fp.value(),
                doc,
                &fs_seam,
                config_file_reader.as_ref(),
                shared_doc_rules::StrictModeOverride::UseConfigFile,
            )
        })
        .await
        {
            Ok(result) => result,
            Err(e) => return e,
        };
        match result {
            Ok(findings) => {
                let exit_code = if findings.is_empty() { 0 } else { 1 };
                let results: Vec<String> = findings.iter().map(|f| f.summary()).collect();
                // Surface `why`/`fix` when non-empty so agent-driven audits
                // carry the same remediation hints as the CLI.
                let rich: Vec<serde_json::Value> = findings
                    .iter()
                    .filter(|f| !f.why.is_empty() || !f.fix.is_empty())
                    .map(|f| {
                        serde_json::json!({
                            "location": f.location,
                            "code": f.code,
                            "violation_type": f.violation_type,
                            "why": f.why,
                            "fix": f.fix,
                        })
                    })
                    .collect();
                serde_json::json!({
                    "status": if exit_code == 0 { "ok" } else { "warning" },
                    "result": if exit_code == 0 { "clean" } else { "violations" },
                    "action": "docs",
                    "exit_code": exit_code,
                    "finding_count": findings.len(),
                    "results": results,
                    "why_fix": rich,
                })
            }
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Run `docs scaffold` — write missing template files in place from
    /// embedded skills. `dry_run` previews without touching disk. Strictness
    /// only affects the audit path, not scaffolding.
    pub async fn execute_docs_scaffold(
        &self,
        surface: &McpActionSurface,
        path: &str,
        member: Option<&str>,
        dry_run: bool,
    ) -> serde_json::Value {
        if let Err(error) = surface.authorize_action_allow_dry_run("docs_scaffold", dry_run) {
            return error;
        }
        let fp = match surface.to_fp(path) {
            Ok(fp) => fp,
            Err(error) => return error,
        };
        let fs_seam = surface.deps.fs_seam.clone();
        let member_owned = member.map(String::from);
        let result = match McpActionSurface::run_blocking(move || {
            dispatcher::surface_docs_action::collect_scaffold(
                fp.value(),
                member_owned.as_deref(),
                dry_run,
                &fs_seam,
            )
        })
        .await
        {
            Ok(result) => result,
            Err(e) => return e,
        };
        match result {
            Ok(items) => {
                let any_error = items.iter().any(|i| !i.ok);
                let exit_code = if any_error { 2 } else { 0 };
                let results: Vec<serde_json::Value> = items
                    .iter()
                    .map(|i| {
                        serde_json::json!({
                            "path": i.path,
                            "created": i.created,
                            "skipped_existing": i.skipped_existing,
                            "ok": i.ok,
                            "message": i.message,
                        })
                    })
                    .collect();
                serde_json::json!({
                    "status": if any_error { "error" } else { "ok" },
                    "result": if dry_run { "preview" } else { "scaffold" },
                    "action": "docs_scaffold",
                    "path": path,
                    "exit_code": exit_code,
                    "item_count": items.len(),
                    "results": results,
                })
            }
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Read skill documentation by section.
    pub fn handle_read_skill(
        &self,
        surface: &McpActionSurface,
        section: Option<String>,
    ) -> serde_json::Value {
        let skills = ["lint-arwaky"];
        let mut candidates: Vec<String> = skills
            .iter()
            .flat_map(|s| vec![format!(".agents/skills/{}/SKILL.md", s)])
            .collect();
        if let Some(config_dir) = dirs::config_dir() {
            let xdg = config_dir
                .join("lint-arwaky")
                .join(".agents")
                .join("skills");
            for s in &skills {
                candidates.push(xdg.join(s).join("SKILL.md").to_string_lossy().to_string());
            }
        }
        let content = candidates
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.exists())
            .and_then(|p| {
                surface
                    .deps
                    .filesystem
                    .execute(FilesystemRequest::read_file(p))
                    .into_content_opt()
            });
        let content = match content {
            Some(c) => c,
            None => {
                return serde_json::json!({"error": "Skill documentation not found", "searched": candidates, "exit_code": 2});
            }
        };
        match section.as_deref() {
            Some(s) if !s.is_empty() => match extract_skill_section(&content, s) {
                Some(body) => serde_json::json!({"section": s, "content": body, "exit_code": 0}),
                None => {
                    serde_json::json!({"error": format!("Section '{s}' not found"), "exit_code": 2})
                }
            },
            _ => serde_json::json!({"content": content, "exit_code": 0}),
        }
    }
}

/// Extract one top-level (`## `) section from a skill document.
///
/// #927: the section heading must sit at the start of a line (or the start of
/// the document) to count; a mid-line `## {section}` mention is not a heading.
/// The section ends at the next top-level H2 heading that begins its own line.
/// `\n## ` cannot match `###`/`####` headings (their third char is `#`, not a
/// space), so deeper headings are not boundaries. Trailing newlines are
/// trimmed so the body matches the heading's exact text.
pub fn extract_skill_section(content: &str, section: &str) -> Option<String> {
    let header = format!("## {section}");
    let start = if content.starts_with(&header) {
        0usize
    } else {
        content.find(&format!("\n{header}"))? + 1
    };
    let remaining = &content[start..];
    let end = remaining.find("\n## ").unwrap_or(remaining.len());
    Some(remaining[..end].trim_end_matches('\n').to_string())
}
