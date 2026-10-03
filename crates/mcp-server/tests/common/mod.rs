#![allow(dead_code)]
// Test support for mcp-server assertion-rich unit/contract tests (QA #641).
//
// Builds a real `McpActionSurface` with:
//   - REAL filesystem and config-system containers (so the path-validation and
//     config code paths under test behave exactly as in production), and
//   - stub rule aggregates for every dependency the tested code paths never
//     invoke. A stub panics loudly if a test accidentally wanders into a code
//     path it did not intend to exercise — vacuous success is not possible.

use std::path::PathBuf;
use std::sync::Arc;

use mcp_server_lint_arwaky::surface_mcp_action_command::{McpActionSurface, McpServerDependencies};

use shared_auto_fix::IFixAggregate;
use shared_auto_fix::taxonomy_auto_fix_request::FixRequest;
use shared_auto_fix::taxonomy_auto_fix_response::FixResponse;
use shared_doc_rules::IDocRunnerAggregate;
use shared_doc_rules::taxonomy_doc_rules_request::DocRequest;
use shared_doc_rules::taxonomy_doc_rules_response::DocResponse;
use shared_external_lint::IExternalLintAggregate;
use shared_external_lint::taxonomy_external_lint_request::ExternalLintRequest;
use shared_external_lint::taxonomy_external_lint_response::ExternalLintResponse;
use shared_git_hooks::IGitHooksAggregate;
use shared_git_hooks::taxonomy_git_hooks_request::GitHooksRequest;
use shared_git_hooks::taxonomy_git_hooks_response::GitHooksResponse;
use shared_import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared_import_rules::taxonomy_import_rules_request::ImportRequest;
use shared_import_rules::taxonomy_import_rules_response::ImportResponse;
use shared_maintenance::IMaintenanceAggregate;
use shared_maintenance::taxonomy_maintenance_request::MaintenanceRequest;
use shared_maintenance::taxonomy_maintenance_response::MaintenanceResponse;
use shared_naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared_naming_rules::taxonomy_naming_rules_request::NamingRequest;
use shared_naming_rules::taxonomy_naming_rules_response::NamingResponse;
use shared_orphan_rules::contract_orphan_aggregate::IOrphanAggregate;
use shared_orphan_rules::taxonomy_orphan_rules_request::OrphanRequest;
use shared_orphan_rules::taxonomy_orphan_rules_response::OrphanResponse;
use shared_project_setup::ISetupAggregate;
use shared_project_setup::taxonomy_project_setup_request::SetupRequest;
use shared_project_setup::taxonomy_project_setup_response::SetupResponse;
use shared_quality_rules::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared_quality_rules::taxonomy_quality_rules_request::CodeAnalysisRequest;
use shared_quality_rules::taxonomy_quality_rules_response::CodeAnalysisResponse;
use shared_role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use shared_role_rules::taxonomy_role_rules_request::RoleRequest;
use shared_role_rules::taxonomy_role_rules_response::RoleResponse;
use shared_structure_rules::contract_structure_aggregate::IStructureAggregate;
use shared_structure_rules::taxonomy_structure_rules_request::StructureRequest;
use shared_structure_rules::taxonomy_structure_rules_response::StructureResponse;

// ─── Stub aggregates (never invoked by the tested code paths) ──────

macro_rules! stub_aggregate {
    ($name:ident, $trait:path, $req:ty, $res:ty) => {
        struct $name;
        impl $trait for $name {
            fn execute(&self, _request: $req) -> $res {
                unimplemented!("QA #641 test stub: this aggregate is not exercised by these tests")
            }
        }
    };
}

stub_aggregate!(FixStub, IFixAggregate, FixRequest, FixResponse);
stub_aggregate!(DocStub, IDocRunnerAggregate, DocRequest, DocResponse);
stub_aggregate!(
    ExternalLintStub,
    IExternalLintAggregate,
    ExternalLintRequest,
    ExternalLintResponse
);
stub_aggregate!(
    GitHooksStub,
    IGitHooksAggregate,
    GitHooksRequest,
    GitHooksResponse
);
stub_aggregate!(
    ImportStub,
    IImportRunnerAggregate,
    ImportRequest,
    ImportResponse
);
stub_aggregate!(
    MaintenanceStub,
    IMaintenanceAggregate,
    MaintenanceRequest,
    MaintenanceResponse
);
stub_aggregate!(
    NamingStub,
    INamingRunnerAggregate,
    NamingRequest,
    NamingResponse
);
stub_aggregate!(OrphanStub, IOrphanAggregate, OrphanRequest, OrphanResponse);
stub_aggregate!(SetupStub, ISetupAggregate, SetupRequest, SetupResponse);
stub_aggregate!(
    QualityStub,
    ICodeAnalysisAggregate,
    CodeAnalysisRequest,
    CodeAnalysisResponse
);
stub_aggregate!(RoleStub, IRoleRunnerAggregate, RoleRequest, RoleResponse);
stub_aggregate!(
    StructureStub,
    IStructureAggregate,
    StructureRequest,
    StructureResponse
);

fn make_fs_seam() -> shared_structure_rules::surface_check_action::FilesystemSeam {
    let c = filesystem::root_filesystem_container::FilesystemContainer::new();
    shared_structure_rules::surface_check_action::FilesystemSeam {
        io: c.io(),
        workspace: c.workspace(),
        parser: c.parser(),
        aggregate: c.orchestrator(),
    }
}

/// Build an `McpActionSurface` with real filesystem/config seams and stub rule
/// aggregates. Only deps-free surface code paths (dispatch, path validation,
/// envelopes, catalog, config reporting) must be exercised against it.
///
/// Read-only by default, matching production unless
/// `LINT_ARWAKY_MCP_ALLOW_MUTATIONS` is set.
pub fn make_action_surface() -> McpActionSurface {
    make_action_surface_with(false)
}

/// Same surface, but with mutating actions authorized. Only for tests that must
/// reach the code *after* authorization — e.g. proving that a mutating action
/// still reads `args.args["path"]` and surfaces its validation error.
pub fn make_action_surface_allowing_mutations() -> McpActionSurface {
    make_action_surface_with(true)
}

fn make_action_surface_with(allow_mutations: bool) -> McpActionSurface {
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let fs_aggregate = fs_container.orchestrator();
    let fs_io = fs_container.io();
    let fs_workspace = fs_container.workspace();
    let fs_tool_resolution = fs_container.tool_resolution();
    let fs_parser = fs_container.parser();

    let config_container = config_system::root_config_system_container::ConfigContainer::new(
        fs_aggregate.clone(),
        fs_io.clone(),
    );

    let fs_seam = Arc::new(make_fs_seam());
    McpActionSurface::new(McpServerDependencies {
        code_analysis_linter: Arc::new(QualityStub),
        fix_orchestrator_factory: Arc::new(|_strict| -> Arc<dyn IFixAggregate> {
            Arc::new(FixStub)
        }),
        orphan_orchestrator: Arc::new(OrphanStub),
        maintenance_orchestrator: Arc::new(MaintenanceStub),
        git_hooks_aggregate: Arc::new(GitHooksStub),
        setup_orchestrator: Arc::new(SetupStub),
        config_orchestrator: config_container.orchestrator(),
        config_parser: config_container.parser(),
        config_reader: config_container.reader(),
        external_lint: Arc::new(ExternalLintStub),
        import_orchestrator: Arc::new(ImportStub),
        naming_orchestrator: Arc::new(NamingStub),
        role_orchestrator: Arc::new(RoleStub),
        doc_orchestrator: Arc::new(DocStub),
        structure_orchestrator: Arc::new(StructureStub),
        filesystem: fs_aggregate,
        filesystem_io: fs_io,
        filesystem_workspace: fs_workspace,
        filesystem_tool_resolution: fs_tool_resolution,
        filesystem_parser: fs_parser,
        fs_seam,
        fs_factory: Arc::new(make_fs_seam),
        orphan_factory: Arc::new(|_config, _fs, _ws| -> Arc<dyn IOrphanAggregate> {
            unimplemented!("QA #641 test stub: orphan factory is not exercised")
        }),
        parse_config_yaml: shared_config_system::utility_config_parser::parse_config_yaml,
        parse_adapter_names:
            shared_config_system::utility_config_parser::parse_adapter_names_from_yaml,
        parse_score_threshold: shared_config_system::utility_config_parser::parse_score_threshold,
        server_version: "0.0.0-qa-test".to_string(),
        // The config tests hand this surface their own `tempfile::TempDir`
        // paths, so the confinement root has to contain them. Confinement
        // itself is proven against real roots in behavioral_mcp_server.rs.
        workspace_root: PathBuf::from("/"),
        allow_mutations,
    })
}
