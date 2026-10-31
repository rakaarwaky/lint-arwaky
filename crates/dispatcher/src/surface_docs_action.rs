// PURPOSE: Doc rules audit business logic, no formatting.
//
// Data Flow:
//   CLI → collect_docs → doc_orchestrator.execute(DocRequest::AuditAll) → findings
//   CLI → collect_scaffold → embedded templates → write files in place
//
// The doc-rules crate audits Markdown documents only. Unlike the source-code
// rule crates it does not consume a file index: the document chain is a fixed
// set of well-known files, so the capability walks the workspace itself.
use std::sync::Arc;

use shared_common::ColumnNumber;
use shared_common::ErrorCode;
use shared_common::FilePath;
use shared_common::LineNumber;
use shared_common::LintMessage;
use shared_common::Severity;
use shared_common::ViolationItem;
use shared_doc_rules::DocRulesConfig;
use shared_doc_rules::IDocRunnerAggregate;
use shared_doc_rules::StrictModeOverride;
use shared_doc_rules::taxonomy_doc_rules_request::DocRequest;
use shared_doc_rules::taxonomy_doc_rules_response::DocResponse;
use shared_project_setup::EMBEDDED_SKILLS;

/// Result of one scaffold file write attempt (or preview).
/// `path` is the target location relative to the audit root; `created`
/// reports whether a new file was written (false when skipped or failed).
#[derive(Debug, Clone)]
pub struct ScaffoldWriteItem {
    pub path: String,
    pub created: bool,
    pub skipped_existing: bool,
    pub ok: bool,
    pub message: String,
}

/// A structured doc finding carrying the fields every output format needs:
/// rule id, target document, violation type, detail message, and a human-
/// readable "file:line" location string that JSON/SARIF/JUnit emitters embed.
/// `severity` is fixed to HIGH: doc invariants are architectural policy gaps.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocsFinding {
    /// The invariant code that fired, e.g. `AES601`.
    pub code: String,
    /// The document this finding belongs to (may be empty for global findings).
    pub doc: String,
    /// A machine-parseable violation subtype (e.g. `missing-frd`).
    pub violation_type: String,
    /// Human-readable detail about what drifted.
    pub message: String,
    /// "doc" or "doc:line" when the checker reports a line offset, else "doc".
    pub location: String,
    /// Severity bucket used for JSON's `"severity"` and SARIF's `"level"`.
    pub severity: Severity,
    /// Why the invariant matters, rendered on its own line in the report.
    #[serde(default)]
    pub why: String,
    /// What to do about it, rendered on its own line under `why`.
    #[serde(default)]
    pub fix: String,
}

impl DocsFinding {
    /// Human-readable "file:line [code] violation_type — message" summary.
    pub fn summary(&self) -> String {
        format!(
            "{} [{}] {} — {}",
            self.location, self.code, self.violation_type, self.message
        )
    }
}

/// Run the doc invariant audit over *root*, returning structured findings.
///
/// Findings whose rule code is suppressed by `lint_arwaky.config.yaml`
/// (`architecture.rules.<CODE>.enabled: false` or `ignored_rules: [CODE]`)
/// are dropped before the result is returned, so `docs .` honours the same
/// config surface as `scan .`.
pub fn collect_docs(
    root: &str,
    doc_orchestrator: Arc<dyn IDocRunnerAggregate>,
    fs_seam: &crate::surface_check_action::FilesystemSeam,
    config_file_reader: &dyn shared_config_system::IConfigFileReaderAggregate,
    strict_override: StrictModeOverride,
) -> Result<Vec<DocsFinding>, String> {
    if !std::path::Path::new(root).is_dir() {
        return Err(format!("Error: path '{root}' does not exist"));
    }
    let suppressed = crate::surface_suppression_hook::load_suppressed_rule_codes(
        root,
        fs_seam,
        config_file_reader,
    );
    // Parse the docs: config block from the workspace-root config file so the
    // audit honours `strict_member_docs` / `strict_placeholders` (Decision Q4).
    // The config and `--strict` override are now consumed by DocRequest and
    // reach MemberDocChecker / TemplateBodyChecker through DocAuditContext.
    let doc_cfg =
        load_doc_rules_config_from_yaml(root, fs_seam).unwrap_or_else(DocRulesConfig::staged);
    let DocResponse::Findings { findings } =
        doc_orchestrator.execute(DocRequest::audit_all(root, Some(doc_cfg), strict_override));
    Ok(findings
        .into_iter()
        .filter(|finding| {
            !crate::surface_suppression_hook::is_rule_suppressed(&suppressed, finding.code)
        })
        .map(|finding| {
            let doc_path = finding.doc.clone();
            let location = if finding.line > 0 {
                format!("{doc_path}:{}", finding.line)
            } else {
                doc_path
            };
            DocsFinding {
                code: finding.code.to_string(),
                doc: finding.doc.clone(),
                violation_type: finding.violation_type.to_string(),
                message: finding.message.clone(),
                location,
                severity: Severity::HIGH,
                why: finding.why.clone(),
                fix: finding.fix.clone(),
            }
        })
        .collect())
}

/// Convert structured doc findings into generic `ViolationItem`s so the
/// existing `utility_output_text_formatter::output_violations` emitters
/// (text/json/sarif/junit) can render them without a special path.
pub fn docs_findings_to_violations(findings: &[DocsFinding]) -> Vec<ViolationItem> {
    findings
        .iter()
        .map(|f| {
            let file = FilePath::new(f.doc.clone()).unwrap_or_default();
            let line = match f.location.rsplit_once(':') {
                Some((_, num)) => num
                    .parse::<i64>()
                    .map(LineNumber::new)
                    .unwrap_or_else(|_| LineNumber::new(0)),
                None => LineNumber::new(0),
            };
            ViolationItem {
                code: ErrorCode::raw(f.code.clone()),
                file,
                line,
                column: ColumnNumber::new(0),
                message: LintMessage::new(f.message.clone()),
                severity: f.severity.clone(),
                violation_name: f.violation_type.to_string(),
                why: f.why.clone(),
                fix: f.fix.clone(),
            }
        })
        .collect()
}

/// Parse a `docs:` block from the raw `lint_arwaky.config.yaml` body found at
/// or above *root* (workspace-root scoped). Returns `None` when the file is
/// absent or the `docs:` key is missing — callers fall back to
/// `DocRulesConfig::staged()`.
pub fn load_doc_rules_config_from_yaml(
    root: &str,
    fs_seam: &crate::surface_check_action::FilesystemSeam,
) -> Option<DocRulesConfig> {
    let root_path = std::path::Path::new(root);
    let fs_ref = fs_seam.aggregate.as_ref();
    let ws_root = fs_ref
        .execute(shared_filesystem::FilesystemRequest::find_workspace_root(
            root_path,
        ))
        .into_root()
        .unwrap_or_else(|| root_path.to_path_buf());
    let content = crate::surface_external_action::find_config_content(&ws_root, fs_ref)?;
    let value = match serde_yaml_ng::from_str::<serde_yaml_ng::Value>(&content) {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!("docs: config block malformed; falling back to staged defaults: {e}");
            return None;
        }
    };
    let docs_block = value.get("docs")?.clone();

    let mut cfg = DocRulesConfig::staged();
    if let Some(v) = docs_block
        .get("strict_member_docs")
        .and_then(|v| v.as_bool())
    {
        cfg.strict_member_docs = v;
    }
    if let Some(v) = docs_block
        .get("strict_placeholders")
        .and_then(|v| v.as_bool())
    {
        cfg.strict_placeholders = v;
    }
    if let Some(exempt) = docs_block.get("exempt_members") {
        cfg.exempt_members = exempt
            .as_sequence()
            .map(|seq| {
                seq.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
    }
    Some(cfg)
}

/// Scaffold missing template files in place (or preview with `dry_run`).
///
/// Reads the embedded reference templates (`references/<doc>/template.md`)
/// from `EMBEDDED_SKILLS` and
/// writes any missing documents — root-level master docs plus per-member
/// docs — so the tree passes FR-DOC-008/009 immediately after the call.
///
/// `dry_run` prints what would be written without touching disk. Existing
/// files are never overwritten (data-loss guard).
///
/// The `member` filter, when present, limits per-member writes to that one
/// member; root-level docs are always written when missing.
pub fn collect_scaffold(
    root: &str,
    member_filter: Option<&str>,
    dry_run: bool,
    fs_seam: &crate::surface_check_action::FilesystemSeam,
) -> Result<Vec<ScaffoldWriteItem>, String> {
    use shared_doc_rules::MemberRole;

    if !std::path::Path::new(root).is_dir() {
        return Err(format!("Error: path '{root}' does not exist"));
    }
    let fs = fs_seam.aggregate.as_ref();
    let root_path = std::path::Path::new(root);

    // Resolve the workspace root so member discovery matches what the doc
    // audit sees.
    let ws_root = fs
        .execute(shared_filesystem::FilesystemRequest::find_workspace_root(
            root_path,
        ))
        .into_root()
        .unwrap_or_else(|| root_path.to_path_buf());

    // Scaffold writes under the workspace root; an arbitrary scan target that
    // is not the root itself is not a valid scaffolding boundary.
    let base_dir = ws_root.clone();

    let mut items: Vec<ScaffoldWriteItem> = Vec::new();

    // ── Root-level master docs ──────────────────────────────────────────
    // TEMPLATE_PROFILES entries that belong at the workspace root.
    // Root docs are always considered when a member filter is absent.
    let root_docs: &[&str] = &[
        shared_doc_rules::PRD_DOC,
        shared_doc_rules::ROADMAP_DOC,
        shared_doc_rules::README_DOC,
        shared_doc_rules::AGENTS_DOC,
        shared_doc_rules::ARCHITECTURE_DOC,
        shared_doc_rules::CONTRIBUTING_DOC,
    ];
    for doc_name in root_docs {
        let target = base_dir.join(doc_name);
        let rel = doc_name.to_string();
        let exists = fs
            .execute(shared_filesystem::FilesystemRequest::path_exists(&target))
            .into_path_exists();
        if exists {
            items.push(ScaffoldWriteItem {
                path: rel,
                created: false,
                skipped_existing: true,
                ok: true,
                message: format!("[skipped: exists] {doc_name}"),
            });
            continue;
        }
        let Some(content) = embedded_template_content(doc_name) else {
            items.push(ScaffoldWriteItem {
                path: rel,
                created: false,
                skipped_existing: false,
                ok: false,
                message: format!("[error: no embedded template] {doc_name}"),
            });
            continue;
        };
        if dry_run {
            items.push(ScaffoldWriteItem {
                path: rel,
                created: true,
                skipped_existing: false,
                ok: true,
                message: format!("[dry-run] would write {doc_name}"),
            });
        } else {
            let ok = fs
                .execute(shared_filesystem::FilesystemRequest::write_file(
                    &target, &content,
                ))
                .into_op_ok();
            items.push(ScaffoldWriteItem {
                path: rel,
                created: ok,
                skipped_existing: false,
                ok,
                message: if ok {
                    format!("[written] {doc_name}")
                } else {
                    format!("[error] failed to write {doc_name}")
                },
            });
        }
    }

    // ── Per-member docs ──────────────────────────────────────────────────
    let members = shared_doc_rules::utility_doc_collector::collect_members(&base_dir);
    // strictness gates the audit, not the scaffold — scaffold always writes missing docs.

    for member in &members {
        // Skip layer members (shared/layer) unless explicitly requested.
        let member_name = member
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        if let Some(filter) = member_filter {
            if filter != member_name {
                continue;
            }
        }
        let member_dir = base_dir.join(&member.path);
        let role = &member.role;
        let required: &[&str] = match role {
            MemberRole::Feature => &[shared_doc_rules::FRD_DOC, shared_doc_rules::BACKLOG_DOC],
            MemberRole::Kernel => &[shared_doc_rules::DATA_DOC, shared_doc_rules::BACKLOG_DOC],
            MemberRole::Surface => &[shared_doc_rules::DESIGN_DOC, shared_doc_rules::BACKLOG_DOC],
            MemberRole::None => {
                continue;
            }
        };
        for doc_name in required {
            let target = member_dir.join(doc_name);
            let rel: String = member_dir
                .strip_prefix(&base_dir)
                .unwrap_or(&member.path)
                .join(doc_name)
                .to_string_lossy()
                .into_owned();
            let rel_str = rel.as_str();
            let exists = fs
                .execute(shared_filesystem::FilesystemRequest::path_exists(&target))
                .into_path_exists();
            if exists {
                items.push(ScaffoldWriteItem {
                    path: rel_str.to_string(),
                    created: false,
                    skipped_existing: true,
                    ok: true,
                    message: format!("[skipped: exists] {rel_str}"),
                });
                continue;
            }
            let Some(content) = embedded_template_content(doc_name) else {
                items.push(ScaffoldWriteItem {
                    path: rel_str.to_string(),
                    created: false,
                    skipped_existing: false,
                    ok: false,
                    message: format!("[error: no embedded template] {rel_str}"),
                });
                continue;
            };
            if dry_run {
                items.push(ScaffoldWriteItem {
                    path: rel_str.to_string(),
                    created: true,
                    skipped_existing: false,
                    ok: true,
                    message: format!("[dry-run] would write {rel_str}"),
                });
            } else {
                // Ensure parent dir exists (the member folder is already on disk,
                // so this should be a no-op, but guard against a missing dir).
                let dir_ok = fs
                    .execute(shared_filesystem::FilesystemRequest::create_dir_all(
                        &member_dir,
                    ))
                    .into_op_ok();
                if !dir_ok {
                    items.push(ScaffoldWriteItem {
                        path: rel_str.to_string(),
                        created: false,
                        skipped_existing: false,
                        ok: false,
                        message: format!("[error] failed to create directory for {rel_str}"),
                    });
                    continue;
                }
                let ok = fs
                    .execute(shared_filesystem::FilesystemRequest::write_file(
                        &target, &content,
                    ))
                    .into_op_ok();
                items.push(ScaffoldWriteItem {
                    path: rel_str.to_string(),
                    created: ok,
                    skipped_existing: false,
                    ok,
                    message: if ok {
                        format!("[written] {rel_str}")
                    } else {
                        format!("[error] failed to write {rel_str}")
                    },
                });
            }
        }
    }

    Ok(items)
}

/// Look up the embedded template body for *doc_name* from the compiled-in
/// `EMBEDDED_SKILLS` catalog. `doc_name` is the bare filename, e.g. `FRD.md`;
/// the lookup key is `frd/template.md`. Returns `None` when no matching
/// template is embedded.
fn embedded_template_content(doc_name: &str) -> Option<String> {
    // Exact match on known template filenames (not a glob ends_with) so two
    // skill files ending with the same suffix in different subdirectories
    // cannot silently collide. Compares against the last two path segments of
    // the embedded skill's relative_path.
    let key = match doc_name {
        "PRD.md" => "prd/template.md",
        "ROADMAP.md" => "roadmap/template.md",
        "README.md" => "readme/template.md",
        "AGENTS.md" => "agents/template.md",
        "ARCHITECTURE.md" => "architecture/template.md",
        "CONTRIBUTING.md" => "contributing/template.md",
        "FRD.md" => "frd/template.md",
        "BACKLOG.md" => "backlog/template.md",
        "DATA.md" => "data/template.md",
        "DESIGN.md" => "design/template.md",
        _ => return None,
    };
    EMBEDDED_SKILLS
        .iter()
        .find(|s| s.relative_path.ends_with(key))
        .map(|s| s.content.to_string())
}

/// One line of output per scaffold file, tagged with its outcome class:
/// `WRITE` for a newly created file, `SKIP` for an existing file left
/// untouched, `ERROR` for a write that failed, `OK` otherwise.
pub fn scaffold_items_to_report_lines(items: &[ScaffoldWriteItem]) -> Vec<String> {
    items
        .iter()
        .map(|i| {
            let tag = if i.skipped_existing {
                "SKIP"
            } else if !i.ok {
                "ERROR"
            } else if i.created {
                "WRITE"
            } else {
                "OK"
            };
            format!("[{tag}] {}", i.message)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared_doc_rules::taxonomy_doc_rules_request::DocFinding;

    #[test]
    fn scaffold_items_to_report_lines_tags_outcome_classes() {
        let items = vec![
            ScaffoldWriteItem {
                path: "PRD.md".into(),
                created: true,
                skipped_existing: false,
                ok: true,
                message: "[written] PRD.md".into(),
            },
            ScaffoldWriteItem {
                path: "README.md".into(),
                created: false,
                skipped_existing: true,
                ok: true,
                message: "[skipped: exists] README.md".into(),
            },
            ScaffoldWriteItem {
                path: "AGENTS.md".into(),
                created: false,
                skipped_existing: false,
                ok: false,
                message: "[error] failed to write AGENTS.md".into(),
            },
        ];
        let lines = scaffold_items_to_report_lines(&items);
        assert_eq!(
            lines,
            vec![
                "[WRITE] [written] PRD.md",
                "[SKIP] [skipped: exists] README.md",
                "[ERROR] [error] failed to write AGENTS.md",
            ]
        );
    }

    #[test]
    fn collect_scaffold_dry_run_does_not_write_files() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();
        // Workspace marker so find_workspace_root resolves.
        std::fs::write(root.join("Cargo.toml"), "[package]\nname = \"fixture\"\n").unwrap();

        let container = filesystem::root_filesystem_container::FilesystemContainer::new();
        let orchestrator = container.orchestrator();
        let fs_seam = crate::surface_check_action::FilesystemSeam {
            workspace: orchestrator.clone(),
            parser: orchestrator.clone(),
            aggregate: orchestrator,
        };
        let result = collect_scaffold(
            root.to_string_lossy().as_ref(),
            None,
            /* dry_run */ true,
            &fs_seam,
        )
        .expect("dry-run scaffold of a fixture root must succeed");

        // Dry-run items must all report `created: true` (would-be writes) but
        // no file may actually exist on disk yet.
        for item in &result {
            if !item.skipped_existing {
                assert!(
                    item.created,
                    "dry-run item must be flagged as a would-be write: {item:?}"
                );
            }
        }
        // The root-level master docs were not on disk, so none were skipped.
        assert!(
            result
                .iter()
                .any(|i| !i.skipped_existing && i.path == "PRD.md"),
            "PRD.md must be a would-be write in a fresh fixture: {result:?}"
        );
        // Confirm no file was actually created.
        assert!(!root.join("PRD.md").exists());
    }

    #[test]
    fn collect_scaffold_missing_path_returns_error() {
        let container = filesystem::root_filesystem_container::FilesystemContainer::new();
        let orchestrator = container.orchestrator();
        let fs_seam = crate::surface_check_action::FilesystemSeam {
            workspace: orchestrator.clone(),
            parser: orchestrator.clone(),
            aggregate: orchestrator,
        };
        let result = collect_scaffold("/definitely/does/not/exist", None, false, &fs_seam);
        assert!(result.is_err(), "missing path must return Err: {result:?}");
    }

    #[test]
    fn docs_finding_keeps_why_and_fix_through_violation_item() {
        let f = DocsFinding {
            code: "AES601".into(),
            doc: "README.md".into(),
            violation_type: "missing-prd".into(),
            message: "PRD.md is missing".into(),
            location: "README.md".into(),
            severity: Severity::HIGH,
            why: "the PRD is the source of truth for scope".into(),
            fix: "add a PRD.md file".into(),
        };
        let items = docs_findings_to_violations(std::slice::from_ref(&f));
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].why, "the PRD is the source of truth for scope");
        assert_eq!(items[0].fix, "add a PRD.md file");
    }

    #[test]
    fn empty_why_and_fix_stay_empty_not_padded() {
        let f = DocsFinding {
            code: "AES602".into(),
            doc: "README.md".into(),
            violation_type: "no-test-file".into(),
            message: "TEST.md is missing".into(),
            location: "README.md".into(),
            severity: Severity::HIGH,
            why: String::new(),
            fix: String::new(),
        };
        let items = docs_findings_to_violations(std::slice::from_ref(&f));
        assert!(items[0].why.is_empty());
        assert!(items[0].fix.is_empty());
    }

    #[test]
    fn doc_finding_to_violation_item_propagates_reasons() {
        let f =
            DocFinding::new_with_line("PRD.md", 0, "AES602", "no-test-file", "TEST.md is missing")
                .with_reason(
                    "TEST.md documents what a green CI proves",
                    "create TEST.md at the workspace root",
                );
        let item = f.to_violation_item(std::path::Path::new("/opt/root"));
        assert_eq!(item.why, "TEST.md documents what a green CI proves");
        assert_eq!(item.fix, "create TEST.md at the workspace root");
    }

    #[test]
    fn doc_finding_without_reason_produces_empty_why_and_fix() {
        let f =
            DocFinding::new_with_line("PRD.md", 0, "AES602", "no-test-file", "TEST.md is missing");
        let item = f.to_violation_item(std::path::Path::new("/opt/root"));
        assert!(item.why.is_empty());
        assert!(item.fix.is_empty());
    }
}
