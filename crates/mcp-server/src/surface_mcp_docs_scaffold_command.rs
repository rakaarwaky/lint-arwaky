// PURPOSE: McpDocsScaffoldSurface — `docs` and `docs scaffold` MCP command wiring.
//
// Holds the two doc-domain handlers that used to inline every audit/scaffold
// JSON response in `surface_mcp_action_command.rs`. The parent surface delegates here
// so its response-building logic stays in one place.

use crate::surface_mcp_action_command::McpActionSurface;

/// MCP surface for the `docs` and `docs_scaffold` actions.
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

    /// Run `docs scaffold` — write missing template files in place.
    /// `dry_run` previews without touching disk. Strictness only affects
    /// the audit path, not scaffolding.
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
}
