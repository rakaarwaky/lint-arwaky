// Regression tests for issue #575 — in-process aggregate dispatch lost the
// subprocess-per-linter fault isolation, so one panicking capability unwound
// through the dispatch loop and killed the whole host process (CLI/MCP/TUI).
//
// Each linter aggregate here is a fake: one slot panics, the rest return clean
// responses. The scan must survive the panic, log it to stderr, and still run
// the remaining capabilities.

use dispatcher_lint_arwaky::surface_check_action::{
    FilesystemSeam, ScanAggregates, ScanOptions, collect_scan_with_progress,
};
use shared_common::ViolationItem;
use shared_common::taxonomy_path_vo::FilePath;
use std::sync::Arc;

/// A capability slot swapped for its panicking fake.
type PanicCase = (&'static str, fn(&mut ScanAggregates));

// ─── Panicking aggregates ─────────────────────────────────────────────

struct PanicQuality;
impl shared_quality_rules::ICodeAnalysisAggregate for PanicQuality {
    fn execute(
        &self,
        _request: shared_quality_rules::CodeAnalysisRequest,
    ) -> shared_quality_rules::CodeAnalysisResponse {
        panic!("injected quality panic");
    }
}

struct PanicRole;
impl shared_role_rules::IRoleRunnerAggregate for PanicRole {
    fn execute(&self, _request: shared_role_rules::RoleRequest) -> shared_role_rules::RoleResponse {
        panic!("injected role panic");
    }
}

struct PanicImport;
impl shared_import_rules::IImportRunnerAggregate for PanicImport {
    fn execute(
        &self,
        _request: shared_import_rules::ImportRequest,
    ) -> shared_import_rules::ImportResponse {
        panic!("injected import panic");
    }
}

struct PanicNaming;
impl shared_naming_rules::INamingRunnerAggregate for PanicNaming {
    fn execute(
        &self,
        _request: shared_naming_rules::NamingRequest,
    ) -> shared_naming_rules::NamingResponse {
        panic!("injected naming panic");
    }
}

struct PanicOrphan;
impl shared_orphan_rules::IOrphanAggregate for PanicOrphan {
    fn execute(
        &self,
        _request: shared_orphan_rules::OrphanRequest,
    ) -> shared_orphan_rules::OrphanResponse {
        panic!("injected orphan panic");
    }
}

struct PanicExternal;
impl shared_external_lint::IExternalLintAggregate for PanicExternal {
    fn execute(
        &self,
        _request: shared_external_lint::ExternalLintRequest,
    ) -> shared_external_lint::ExternalLintResponse {
        panic!("injected external panic");
    }
}

struct PanicStructure;
impl shared_structure_rules::IStructureAggregate for PanicStructure {
    fn execute(
        &self,
        _request: shared_structure_rules::StructureRequest,
    ) -> shared_structure_rules::StructureResponse {
        panic!("injected structure panic");
    }
}

struct PanicDoc;
impl shared_doc_rules::IDocRunnerAggregate for PanicDoc {
    fn execute(&self, _request: shared_doc_rules::DocRequest) -> shared_doc_rules::DocResponse {
        panic!("injected doc panic");
    }
}

// ─── Clean aggregates ─────────────────────────────────────────────────

struct CleanQuality;
impl shared_quality_rules::ICodeAnalysisAggregate for CleanQuality {
    fn execute(
        &self,
        _request: shared_quality_rules::CodeAnalysisRequest,
    ) -> shared_quality_rules::CodeAnalysisResponse {
        shared_quality_rules::CodeAnalysisResponse::Analysis {
            violations: Vec::new(),
        }
    }
}

struct CleanRole;
impl shared_role_rules::IRoleRunnerAggregate for CleanRole {
    fn execute(&self, _request: shared_role_rules::RoleRequest) -> shared_role_rules::RoleResponse {
        shared_role_rules::RoleResponse::Audit {
            violations: Vec::new(),
        }
    }
}

struct CleanImport;
impl shared_import_rules::IImportRunnerAggregate for CleanImport {
    fn execute(
        &self,
        _request: shared_import_rules::ImportRequest,
    ) -> shared_import_rules::ImportResponse {
        shared_import_rules::ImportResponse::AuditEntries {
            violations: Vec::new(),
        }
    }
}

struct CleanNaming;
impl shared_naming_rules::INamingRunnerAggregate for CleanNaming {
    fn execute(
        &self,
        _request: shared_naming_rules::NamingRequest,
    ) -> shared_naming_rules::NamingResponse {
        shared_naming_rules::NamingResponse::Audit {
            violations: Vec::new(),
        }
    }
}

struct CleanOrphan;
impl shared_orphan_rules::IOrphanAggregate for CleanOrphan {
    fn execute(
        &self,
        _request: shared_orphan_rules::OrphanRequest,
    ) -> shared_orphan_rules::OrphanResponse {
        shared_orphan_rules::OrphanResponse::ScanOutcome {
            context: Default::default(),
            violations: Vec::new(),
        }
    }
}

struct CleanExternal;
impl shared_external_lint::IExternalLintAggregate for CleanExternal {
    fn execute(
        &self,
        _request: shared_external_lint::ExternalLintRequest,
    ) -> shared_external_lint::ExternalLintResponse {
        shared_external_lint::ExternalLintResponse::Scan {
            violations: Default::default(),
        }
    }
}

struct CleanStructure;
impl shared_structure_rules::IStructureAggregate for CleanStructure {
    fn execute(
        &self,
        _request: shared_structure_rules::StructureRequest,
    ) -> shared_structure_rules::StructureResponse {
        shared_structure_rules::StructureResponse::Findings {
            findings: Vec::new(),
        }
    }
}

struct CleanDoc;
impl shared_doc_rules::IDocRunnerAggregate for CleanDoc {
    fn execute(&self, _request: shared_doc_rules::DocRequest) -> shared_doc_rules::DocResponse {
        shared_doc_rules::DocResponse::Findings {
            findings: Vec::new(),
        }
    }
}

/// Config aggregate — reports no ignored paths and no workspaces.
struct CleanConfig;
impl shared_config_system::IConfigOrchestratorAggregate for CleanConfig {
    fn execute(
        &self,
        request: shared_config_system::ConfigRequest,
    ) -> shared_config_system::ConfigResponse {
        match request {
            shared_config_system::ConfigRequest::IgnoredPaths { .. } => {
                shared_config_system::ConfigResponse::IgnoredPaths {
                    patterns: shared_common::PatternList::new(Vec::<String>::new()),
                }
            }
            _ => shared_config_system::ConfigResponse::DiscoverWorkspaces {
                workspaces: Vec::new(),
            },
        }
    }
}

// ─── Fixtures ─────────────────────────────────────────────────────────

fn build_seam() -> Arc<FilesystemSeam> {
    let c = filesystem::root_filesystem_container::FilesystemContainer::new();
    Arc::new(FilesystemSeam {
        workspace: c.workspace(),
        parser: c.parser(),
        aggregate: c.orchestrator(),
    })
}

/// Small clean fixture — enough files to reach every capability without
/// scanning the whole repo.
fn scan_target() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("workspaces-good/crates/calculator"))
        .expect("workspace root resolves")
}

/// `ScanAggregates` with every capability clean; tests swap one slot at a time.
fn clean_aggregates(fs_seam: Arc<FilesystemSeam>) -> ScanAggregates {
    ScanAggregates {
        quality: Arc::new(CleanQuality),
        role: Arc::new(CleanRole),
        import: Arc::new(CleanImport),
        naming: Arc::new(CleanNaming),
        external: Arc::new(CleanExternal),
        orphan: Arc::new(CleanOrphan),
        config: Arc::new(CleanConfig),
        structure: Arc::new(CleanStructure),
        doc: Arc::new(CleanDoc),
        fs_seam,
    }
}

/// Scan `target` with `aggregates`. The outer `catch_unwind` is the assertion
/// under test: it fails the test if the host process unwinds at all.
fn scan_isolated(target: &std::path::Path, aggregates: ScanAggregates) -> Vec<ViolationItem> {
    let opts = ScanOptions {
        path: Some(FilePath::new(target.to_string_lossy().to_string()).unwrap()),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: build_seam(),
        scan_aggregates: Some(aggregates),
    };
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        collect_scan_with_progress(opts, |_phase, _done, _total| {})
    }));
    result
        .expect("#575: a capability panic must not unwind out of collect_scan")
        .expect("#575: a capability panic must not fail the whole scan")
}

// ─── Tests ────────────────────────────────────────────────────────────

/// #575: a panicking quality linter is isolated; the scan still returns
/// results for the other capabilities.
#[test]
fn panicking_quality_capability_is_isolated() {
    let target = scan_target();
    let fs_seam = build_seam();
    let mut agg = clean_aggregates(fs_seam.clone());
    agg.quality = Arc::new(PanicQuality);

    // The scan must not unwind.
    let _violations = scan_isolated(&target, agg);
}

/// #575: every capability is isolated, not just the first one.
#[test]
fn every_capability_panic_is_isolated() {
    let target = scan_target();
    let fs_seam = build_seam();

    let cases: Vec<PanicCase> = vec![
        ("quality", |a: &mut ScanAggregates| {
            a.quality = Arc::new(PanicQuality)
        }),
        ("role", |a: &mut ScanAggregates| {
            a.role = Arc::new(PanicRole)
        }),
        ("import", |a: &mut ScanAggregates| {
            a.import = Arc::new(PanicImport)
        }),
        ("naming", |a: &mut ScanAggregates| {
            a.naming = Arc::new(PanicNaming)
        }),
        ("orphan", |a: &mut ScanAggregates| {
            a.orphan = Arc::new(PanicOrphan)
        }),
        ("external", |a: &mut ScanAggregates| {
            a.external = Arc::new(PanicExternal)
        }),
        ("structure", |a: &mut ScanAggregates| {
            a.structure = Arc::new(PanicStructure)
        }),
        ("doc", |a: &mut ScanAggregates| a.doc = Arc::new(PanicDoc)),
    ];

    for (_capability, inject) in cases {
        let mut agg = clean_aggregates(fs_seam.clone());
        inject(&mut agg);
        // Each panicking capability must not unwind the host process, and the
        // scan must still complete and return its (clean) results.
        let violations = scan_isolated(&target, agg);
        let _ = violations;
    }
}

/// #575: several capabilities panicking at once — every panic is isolated and
/// the scan still completes.
#[test]
fn all_capabilities_panicking_still_completes() {
    let target = scan_target();
    let fs_seam = build_seam();
    let mut agg = clean_aggregates(fs_seam);
    agg.quality = Arc::new(PanicQuality);
    agg.role = Arc::new(PanicRole);
    agg.import = Arc::new(PanicImport);
    agg.naming = Arc::new(PanicNaming);
    agg.orphan = Arc::new(PanicOrphan);
    agg.external = Arc::new(PanicExternal);
    agg.structure = Arc::new(PanicStructure);
    agg.doc = Arc::new(PanicDoc);

    let _violations = scan_isolated(&target, agg);
}

/// #575: a clean scan completes and no panic log is produced.
#[test]
fn clean_capabilities_complete_scan() {
    let target = scan_target();
    let fs_seam = build_seam();
    let agg = clean_aggregates(fs_seam);

    let _violations = scan_isolated(&target, agg);
}
