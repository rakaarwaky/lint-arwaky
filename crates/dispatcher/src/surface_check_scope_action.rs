// PURPOSE: Scope filter and single-file scan helpers for the check action.
//
// Converts raw linter output into scan-scoped violation items and runs the
// single-file scan path that builds a one-entry index.
use crate::surface_check_action::{
    FilesystemSeam, ScanAggregates, canonicalize_via, detect_language_from_path,
};
use crate::surface_ci_action::run_isolated;
use shared_common::ViolationItem;
use shared_doc_rules::taxonomy_doc_rules_request::DocRequest;
use shared_doc_rules::taxonomy_doc_rules_response::DocResponse;
use shared_filesystem::FilesystemRequest;
use shared_filesystem::utility_test_file_discovery;
use shared_import_rules::taxonomy_import_rules_request::ImportRequest;
use shared_naming_rules::{BENCHES_DIR, TESTS_DIR, taxonomy_naming_rules_request::NamingRequest};
use shared_orphan_rules::OrphanRequest;
use shared_quality_rules::CodeAnalysisRequest;
use shared_role_rules::taxonomy_role_rules_request::RoleRequest;
pub(crate) fn structure_violations_in_scope(
    target: &str,
    agg: &ScanAggregates,
) -> Vec<ViolationItem> {
    let target_path = std::path::Path::new(target);
    // The audit resolves the workspace root the same way; mirror that here so
    // a finding path joins onto the right base.
    let has_members = ["crates", "modules", "packages"]
        .iter()
        .any(|m| target_path.join(m).is_dir());
    let ws_root: std::path::PathBuf = if has_members {
        target_path.to_path_buf()
    } else {
        target_path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| target_path.to_path_buf())
    };

    crate::surface_structure_action::collect_structure(
        target,
        agg.structure.clone(),
        &agg.fs_seam,
        agg.config_file_reader.as_ref(),
    )
    .unwrap_or_default()
    .into_iter()
    .filter(|v| {
        // A finding names a folder (AES702) or a file (AES701/AES703); both
        // resolve under the workspace root. The audit always walks every
        // member dir under that root, so a member-dir scan target must
        // additionally keep only the findings inside itself.
        let resolved = ws_root.join(&v.file.value);
        let exists = resolved.is_dir()
            || agg
                .fs_seam
                .aggregate
                .execute(FilesystemRequest::path_exists(&resolved))
                .into_path_exists();
        exists && resolved.starts_with(target_path)
    })
    .collect::<Vec<_>>()
}

/// Run the doc-invariant audit (AES601–AES606) and convert its findings into
/// violation items. Findings carry relative doc paths and resolve against the
/// scan target; a finding names a document that must exist inside the target,
/// so out-of-scope or stale findings are dropped like structure findings.
/// All doc rules are HIGH-severity invariant failures → `Severity::HIGH`.
///
/// Findings whose rule code is suppressed by `lint_arwaky.config.yaml`
/// (`architecture.rules.<CODE>.enabled: false` or `ignored_rules: [CODE]`)
/// are dropped before conversion, so `scan .` honours the same config
/// surface as `docs .`.
///
/// Missing-doc findings (`missing_required_doc`, `missing_member_doc`) —
/// whose very premise is that the file does not yet exist on disk — bypass
/// the existence guard so they surface in `check .` as well (Decision Q2);
/// all other doc findings still require the named file to exist.
pub(crate) fn doc_violations_in_scope(target: &str, agg: &ScanAggregates) -> Vec<ViolationItem> {
    let target_path = std::path::Path::new(target);
    let target_canon = canonicalize_via(&agg.fs_seam.aggregate, target_path)
        .unwrap_or_else(|| target_path.to_path_buf());

    let suppressed = crate::surface_suppression_hook::load_suppressed_rule_codes(
        target,
        &agg.fs_seam,
        agg.config_file_reader.as_ref(),
    );
    let doc_cfg = crate::surface_docs_action::load_doc_rules_config_from_yaml(target, &agg.fs_seam)
        .unwrap_or_else(shared_doc_rules::DocRulesConfig::staged);
    let DocResponse::Findings { findings } = agg.doc.execute(DocRequest::audit_all(
        &target_canon,
        Some(doc_cfg),
        shared_doc_rules::StrictModeOverride::UseConfigFile,
    ));
    findings
        .into_iter()
        .filter(|finding| {
            !crate::surface_suppression_hook::is_rule_suppressed(&suppressed, finding.code)
        })
        .filter_map(|finding| {
            let doc_path = target_canon.join(&finding.doc);
            // Store the resolved path, not `finding.doc`. A bare relative name
            // cannot be placed in the report's folder tree, so it would fall
            // back to the scan root and appear to belong to no member.
            let file =
                shared_common::FilePath::new(doc_path.to_string_lossy().into_owned()).ok()?;
            // Missing-doc findings (missing_required_doc / missing_member_doc)
            // describe a file that does not yet exist on disk — the existence
            // guard must not drop them (Decision Q2). All other doc findings
            // still require the named file to exist.
            let is_missing_doc = finding.violation_type
                == shared_doc_rules::taxonomy_doc_rules_constant::MISSING_DOC_VIOLATION
                || finding.violation_type
                    == shared_doc_rules::taxonomy_doc_rules_constant::MEMBER_DOC_VIOLATION;
            let exists = doc_path.is_file()
                || agg
                    .fs_seam
                    .aggregate
                    .execute(FilesystemRequest::path_exists(&doc_path))
                    .into_path_exists();
            if !exists && !is_missing_doc {
                return None;
            }
            let message = shared_common::LintMessage::new(format!(
                "[{}] {}",
                finding.violation_type, finding.message
            ));
            Some(ViolationItem {
                code: shared_common::ErrorCode::raw(finding.code),
                file,
                // The finding already carries the line it anchored to;
                // hardcoding 1 sent every doc finding to the top of the file.
                line: shared_common::LineNumber::new(finding.line as i64),
                // Doc findings anchor to a line, not a column. Reporting a
                // column of 1 would claim a precision the check does not have.
                column: shared_common::ColumnNumber::new(0),
                message,
                severity: shared_common::Severity::HIGH,
                violation_name: finding.violation_type.to_string(),
                why: finding.why,
                fix: finding.fix,
            })
        })
        .collect::<Vec<_>>()
}

/// Whether an external-lint violation falls inside the scan scope: its file
/// resolves under the target, or it is an AES205 cycle resolved under the
/// parent workspace. When the target is a workspace root (`member_dirs` is
/// `Some`), a violation must additionally resolve inside one of the member
/// dirs — the AES linters only lint `crates`/`packages`/`modules`, so root
/// level files (e.g. a packaging `setup.py`) are out of scope for the external
/// adapters too.
pub(crate) fn external_violation_in_scope(
    v: &ViolationItem,
    seam: &FilesystemSeam,
    target_canon: &std::path::Path,
    parent_workspace: Option<&std::path::Path>,
    member_dirs: Option<&[&std::path::Path]>,
) -> bool {
    let resolved_canon = resolve_violation_path(v, seam, target_canon, parent_workspace);
    if let Some(members) = member_dirs {
        return members.iter().any(|m| {
            resolved_canon.starts_with(m)
                && seam
                    .aggregate
                    .execute(FilesystemRequest::path_exists(&resolved_canon))
                    .into_path_exists()
        });
    }
    resolved_canon.starts_with(target_canon)
        || (v.code.code() == "AES205"
            && parent_workspace.is_some_and(|pw| resolved_canon.starts_with(pw)))
}

/// Whether a violation stays in the scan scope: the file exists under the
/// target, or it is an AES205 cycle / AES5xx orphan under the parent workspace.
/// Non-existent files are dropped so doubled/stale paths never surface as E902.
/// When `member_dirs` is `Some` (workspace root target), the violation must
/// additionally resolve inside one of the member dirs — root-level packaging
/// files (e.g. setup.py) are out of scope for every linter.
pub(crate) fn violation_in_scan_scope(
    v: &ViolationItem,
    seam: &FilesystemSeam,
    target_canon: &std::path::Path,
    parent_workspace: Option<&std::path::Path>,
    workspace_root: Option<&std::path::Path>,
    member_dirs: Option<&[&std::path::Path]>,
) -> bool {
    let resolved_canon = resolve_violation_path(v, seam, target_canon, workspace_root);
    if !&seam
        .aggregate
        .execute(FilesystemRequest::path_exists(&resolved_canon))
        .into_path_exists()
    {
        return false; // E902 guard: non-existent file — drop, no violation.
    }
    let in_target = resolved_canon.starts_with(target_canon);
    let in_parent_ws = parent_workspace.is_some_and(|pw| resolved_canon.starts_with(pw));
    // At a workspace root, scope to member dirs only so root-level packaging
    // files are never reported — matching what the AES linters lint.
    if let Some(members) = member_dirs {
        return members.iter().any(|m| resolved_canon.starts_with(m));
    }
    in_target
        || (v.code.code() == "AES205" && in_parent_ws)
        || (v.code.code().starts_with("AES5") && in_parent_ws)
}

/// Resolve a violation's file path to a canonical absolute path under the
/// target (relative paths join the target; absolute paths are canonicalized as
/// is, falling back to the path itself when canonicalization fails).
///
/// Orphan (AES5xx) and cycle (AES205) violations carry paths relative to the
/// workspace root, which is the target's parent when a member directory is the
/// scan target. Those do not resolve under the target, so a relative path that
/// misses there is retried against the parent workspace before being kept.
pub(crate) fn resolve_violation_path(
    v: &ViolationItem,
    seam: &FilesystemSeam,
    target_canon: &std::path::Path,
    workspace_root: Option<&std::path::Path>,
) -> std::path::PathBuf {
    let file_path = std::path::Path::new(&v.file.value);
    let resolved = if file_path.is_absolute() {
        file_path.to_path_buf()
    } else {
        let under_target = target_canon.join(file_path);
        let exists_under_target = &seam
            .aggregate
            .execute(FilesystemRequest::path_exists(&under_target))
            .into_path_exists();
        match workspace_root {
            Some(pw) if !exists_under_target => pw.join(file_path),
            _ => under_target,
        }
    };
    canonicalize_via(&seam.aggregate, &resolved).unwrap_or(resolved)
}

/// Run all 6 linters on a single-file target, returning in-scope violations.
pub(crate) fn run_single_file_scan(
    seam: &FilesystemSeam,
    scan_root: &std::path::Path,
    agg: &ScanAggregates,
    root_fp: &shared_common::taxonomy_path_vo::FilePath,
    ignored: &[String],
) -> Vec<ViolationItem> {
    let content = seam
        .aggregate
        .execute(FilesystemRequest::read_lintable_file(
            &scan_root.to_string_lossy(),
        ))
        .into_content_opt()
        .unwrap_or_default();
    let extension = scan_root
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_string();
    let language = detect_language_from_path(scan_root.to_string_lossy().as_ref());
    let entries = vec![shared_filesystem::taxonomy_filesystem_vo::FileEntry {
        path: scan_root.to_path_buf(),
        extension,
        language,
        size: content.len() as u64,
        content: content.clone(),
        parse_ok: !content.is_empty(),
        parse_metadata: None,
    }];
    let parsed = seam
        .parser
        .execute(FilesystemRequest::parse_all(entries.clone()))
        .into_parsed_entries();
    let entries = parsed;
    let import_map: std::collections::HashMap<
        String,
        Vec<shared_filesystem::taxonomy_filesystem_vo::ImportEntry>,
    > = std::collections::HashMap::new();
    let mut all: Vec<ViolationItem> = Vec::new();
    // Capability panics are logged after the scope filter, so an isolated
    // failure is visible without inventing a rule code.
    let mut panics: Vec<String> = Vec::new();
    all.extend(run_isolated("quality", &mut panics, || {
        agg.quality
            .execute(CodeAnalysisRequest::RunAnalysis {
                files: entries.clone(),
            })
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result)
            .collect::<Vec<_>>()
    }));
    all.extend(run_isolated("role", &mut panics, || {
        agg.role
            .execute(RoleRequest::audit(&entries))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result)
            .collect::<Vec<_>>()
    }));
    all.extend(run_isolated("import", &mut panics, || {
        agg.import
            .execute(ImportRequest::audit_with_entries_and_imports(
                &entries,
                &import_map,
            ))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result)
            .collect::<Vec<_>>()
    }));
    // AES103 judges the test/bench files, so a one-file scan of a misnamed or
    // nested test file has to reach it. The file itself is the only evidence
    // that scan target can offer, so it is passed as the test set when it sits
    // in one of those directories.
    let test_entries: Vec<shared_filesystem::taxonomy_filesystem_vo::FileEntry> =
        if utility_test_file_discovery::is_inside_any(
            &scan_root.to_string_lossy(),
            &[TESTS_DIR, BENCHES_DIR],
        ) {
            entries.clone()
        } else {
            Vec::new()
        };
    all.extend(run_isolated("naming", &mut panics, || {
        agg.naming
            .execute(NamingRequest::audit_with_tests(&entries, &test_entries))
            .into_violations()
    }));
    all.extend(run_isolated("orphan", &mut panics, || {
        let (_graph_ctx, orphan_violations) = agg
            .orphan
            .execute(OrphanRequest::scan(
                root_fp,
                &shared_common::taxonomy_common_vo::PatternList::new(ignored.to_vec()),
            ))
            .into_scan_outcome();
        orphan_violations
            .iter()
            .map(ViolationItem::from_lint_result)
            .collect::<Vec<_>>()
    }));
    // External — the MarkdownLint adapter is the only one that lints a single
    // file meaningfully: every other adapter shells out to a project-wide tool
    // (cargo, ruff, eslint) whose findings name other files, so on a one-file
    // target they would all be dropped by the scope filter below anyway. The
    // adapter itself already early-returns empty for a non-Markdown file, so
    // this is safe to call unconditionally.
    all.extend(run_isolated("external", &mut panics, || {
        crate::surface_external_action::collect_single_file_external(
            scan_root,
            ignored,
            &agg.external,
            crate::surface_external_action::load_config_entries(
                scan_root,
                seam.aggregate.as_ref(),
                agg.config_file_reader.as_ref(),
            ),
            seam.aggregate.as_ref(),
            agg.config_file_reader.as_ref(),
        )
    }));
    all.retain(|v| {
        let p = std::path::Path::new(&v.file.value);
        p.is_absolute() && p.starts_with(scan_root)
    });
    if !panics.is_empty() {
        for line in &panics {
            eprintln!("{line}");
        }
    }
    all
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared_doc_rules::IDocRunnerAggregate;
    use shared_doc_rules::taxonomy_doc_rules_request::DocFinding;
    use shared_doc_rules::taxonomy_doc_rules_response::DocResponse;
    use std::sync::Arc;

    /// Stub doc aggregate: every `audit_all` request returns the injected
    /// findings regardless of root, so the scope filter is tested in isolation.
    struct StubDoc {
        findings: Vec<DocFinding>,
    }

    impl IDocRunnerAggregate for StubDoc {
        fn execute(&self, _request: DocRequest) -> DocResponse {
            DocResponse::Findings {
                findings: self.findings.clone(),
            }
        }
    }

    /// Clean no-op aggregates: each returns empty responses, so
    /// `doc_violations_in_scope` only exercises the injected `StubDoc` slot.
    struct NoopQuality;
    impl shared_quality_rules::ICodeAnalysisAggregate for NoopQuality {
        fn execute(
            &self,
            _: shared_quality_rules::CodeAnalysisRequest,
        ) -> shared_quality_rules::CodeAnalysisResponse {
            shared_quality_rules::CodeAnalysisResponse::Analysis {
                violations: Vec::new(),
            }
        }
    }
    struct NoopRole;
    impl shared_role_rules::IRoleRunnerAggregate for NoopRole {
        fn execute(&self, _: shared_role_rules::RoleRequest) -> shared_role_rules::RoleResponse {
            shared_role_rules::RoleResponse::Audit {
                violations: Vec::new(),
            }
        }
    }
    struct NoopImport;
    impl shared_import_rules::IImportRunnerAggregate for NoopImport {
        fn execute(
            &self,
            _: shared_import_rules::ImportRequest,
        ) -> shared_import_rules::ImportResponse {
            shared_import_rules::ImportResponse::AuditEntries {
                violations: Vec::new(),
            }
        }
    }
    struct NoopNaming;
    impl shared_naming_rules::INamingRunnerAggregate for NoopNaming {
        fn execute(
            &self,
            _: shared_naming_rules::NamingRequest,
        ) -> shared_naming_rules::NamingResponse {
            shared_naming_rules::NamingResponse::Audit {
                violations: Vec::new(),
            }
        }
    }
    struct NoopExternal;
    impl shared_external_lint::IExternalLintAggregate for NoopExternal {
        fn execute(
            &self,
            _: shared_external_lint::ExternalLintRequest,
        ) -> shared_external_lint::ExternalLintResponse {
            shared_external_lint::ExternalLintResponse::Scan {
                violations: Default::default(),
            }
        }
    }
    struct NoopOrphan;
    impl shared_orphan_rules::IOrphanAggregate for NoopOrphan {
        fn execute(
            &self,
            _: shared_orphan_rules::OrphanRequest,
        ) -> shared_orphan_rules::OrphanResponse {
            shared_orphan_rules::OrphanResponse::ScanOutcome {
                context: Default::default(),
                violations: Vec::new(),
            }
        }
    }
    struct NoopConfig;
    impl shared_config_system::IConfigOrchestratorAggregate for NoopConfig {
        fn execute(
            &self,
            _: shared_config_system::ConfigRequest,
        ) -> shared_config_system::ConfigResponse {
            shared_config_system::ConfigResponse::DiscoverWorkspaces {
                workspaces: Vec::new(),
            }
        }
    }
    struct NoopStructure;
    impl shared_structure_rules::IStructureAggregate for NoopStructure {
        fn execute(
            &self,
            _: shared_structure_rules::StructureRequest,
        ) -> shared_structure_rules::StructureResponse {
            shared_structure_rules::StructureResponse::Findings {
                findings: Vec::new(),
            }
        }
    }

    fn build_aggregates(
        fs_seam: &FilesystemSeam,
        doc_aggregate: Arc<dyn IDocRunnerAggregate>,
        config_reader: Arc<dyn shared_config_system::IConfigFileReaderAggregate>,
    ) -> ScanAggregates {
        ScanAggregates {
            quality: Arc::new(NoopQuality),
            role: Arc::new(NoopRole),
            import: Arc::new(NoopImport),
            naming: Arc::new(NoopNaming),
            external: Arc::new(NoopExternal),
            orphan: Arc::new(NoopOrphan),
            config: Arc::new(NoopConfig),
            structure: Arc::new(NoopStructure),
            doc: doc_aggregate,
            config_file_reader: config_reader,
            fs_seam: Arc::new(fs_seam.clone()),
        }
    }

    fn fs_seam() -> FilesystemSeam {
        let container = filesystem::root_filesystem_container::FilesystemContainer::new();
        let orchestrator = container.orchestrator();
        FilesystemSeam {
            workspace: orchestrator.clone(),
            parser: orchestrator.clone(),
            aggregate: orchestrator,
        }
    }

    fn doc_finding(doc: &str, violation_type: &'static str, message: &str) -> DocFinding {
        DocFinding::new_with_line(
            doc,
            0,
            shared_doc_rules::RULE_CODE_TEMPLATE_CONFORMANCE,
            violation_type,
            message,
        )
    }

    /// Missing-doc findings bypass the existence guard and surface in
    /// `check .` even when the named document does not yet exist on disk.
    #[test]
    fn missing_doc_findings_pass_through_scope_guard() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();
        // Workspace marker so `find_workspace_root` and the root gate are honest.
        std::fs::write(root.join("Cargo.toml"), "[package]\nname = \"fixture\"\n").unwrap();
        // Confirm the missing files do not exist yet.
        assert!(!root.join("PRD.md").exists());

        let seam = fs_seam();
        let config_container = config_system::root_config_system_container::ConfigContainer::new(
            seam.aggregate.clone(),
            {
                let c = filesystem::root_filesystem_container::FilesystemContainer::new();
                c.io()
            },
        );

        let missing_required = doc_finding(
            "PRD.md",
            shared_doc_rules::MISSING_DOC_VIOLATION,
            "PRD.md is missing",
        );
        let missing_member = doc_finding(
            "crates/foo/FRD.md",
            shared_doc_rules::MEMBER_DOC_VIOLATION,
            "FRD.md is missing in member foo",
        );
        let agg = build_aggregates(
            &seam,
            Arc::new(StubDoc {
                findings: vec![missing_required, missing_member],
            }),
            config_container.file_reader(),
        );

        let violations = doc_violations_in_scope(root.to_string_lossy().as_ref(), &agg);
        assert_eq!(
            violations.len(),
            2,
            "both missing-doc findings must surface: {violations:?}"
        );
        let names: Vec<&str> = violations
            .iter()
            .map(|v| v.violation_name.as_str())
            .collect();
        assert!(
            names
                .iter()
                .any(|n| **n == *shared_doc_rules::MISSING_DOC_VIOLATION),
            "missing_required_doc must surface: {names:?}"
        );
        assert!(
            names
                .iter()
                .any(|n| **n == *shared_doc_rules::MEMBER_DOC_VIOLATION),
            "missing_member_doc must surface: {names:?}"
        );
    }

    /// A non-missing-doc finding for a document that does not exist on disk is
    /// still dropped by the scope guard — stale-format findings for a deleted
    /// file must not leak into `check .` output.
    #[test]
    fn stale_format_finding_for_nonexistent_file_is_dropped() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();
        std::fs::write(root.join("Cargo.toml"), "[package]\nname = \"fixture\"\n").unwrap();

        let seam = fs_seam();
        let config_container = config_system::root_config_system_container::ConfigContainer::new(
            seam.aggregate.clone(),
            {
                let c = filesystem::root_filesystem_container::FilesystemContainer::new();
                c.io()
            },
        );

        // A `section_not_table` finding names `GONE.md`, which does not exist.
        let stale = doc_finding(
            "GONE.md",
            "section_not_table",
            "Section 'State Definitions' is not a table",
        );
        let agg = build_aggregates(
            &seam,
            Arc::new(StubDoc {
                findings: vec![stale],
            }),
            config_container.file_reader(),
        );

        let violations = doc_violations_in_scope(root.to_string_lossy().as_ref(), &agg);
        assert_eq!(
            violations.len(),
            0,
            "non-missing-doc finding for a nonexistent file must be dropped: {violations:?}"
        );
    }
}
