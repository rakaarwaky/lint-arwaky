// E2E tests — verify all contract traits are object-safe and usable across crate boundaries.

// Verify Arc<dyn Trait> works for all aggregates (DI wiring pattern)
#[test]
fn e2e_all_aggregates_support_arc_dynamic_dispatch() {
    fn assert_arc<T: ?Sized>() {}
    assert_arc::<dyn shared_lint_arwaky::config_system::IConfigOrchestratorAggregate>();
    assert_arc::<dyn shared_lint_arwaky::filesystem::IFilesystemAggregate>();
    assert_arc::<dyn shared_lint_arwaky::quality_rules::ICodeAnalysisAggregate>();
    assert_arc::<dyn shared_lint_arwaky::import_rules::IImportRunnerAggregate>();
    assert_arc::<dyn shared_lint_arwaky::naming_rules::INamingRunnerAggregate>();
    assert_arc::<dyn shared_lint_arwaky::orphan_rules::IOrphanAggregate>();
    assert_arc::<dyn shared_lint_arwaky::role_rules::IRoleRunnerAggregate>();
    assert_arc::<dyn shared_lint_arwaky::auto_fix::IFixAggregate>();
    assert_arc::<dyn shared_lint_arwaky::file_watch::IWatchAggregate>();
    assert_arc::<dyn shared_lint_arwaky::git_hooks::IGitHooksAggregate>();
    assert_arc::<dyn shared_lint_arwaky::maintenance::IMaintenanceAggregate>();
    assert_arc::<dyn shared_lint_arwaky::external_lint::IExternalLintAggregate>();
    assert_arc::<dyn shared_lint_arwaky::report_formatter::IReportFormatterAggregate>();
    assert_arc::<dyn shared_lint_arwaky::project_setup::ISetupAggregate>();
}

// Verify all protocols are object-safe (used as dyn in DI)
#[test]
fn e2e_all_protocols_are_object_safe() {
    fn assert_trait<T: ?Sized>() {}
    // Config
    assert_trait::<dyn shared_lint_arwaky::config_system::IConfigReaderProtocol>();
    assert_trait::<dyn shared_lint_arwaky::config_system::IConfigParserProtocol>();
    assert_trait::<dyn shared_lint_arwaky::config_system::IConfigValidatorProtocol>();
    assert_trait::<dyn shared_lint_arwaky::config_system::IWorkspaceDetectorProtocol>();
    // Filesystem
    assert_trait::<dyn shared_lint_arwaky::filesystem::IFileSystemIOProtocol>();
    assert_trait::<dyn shared_lint_arwaky::filesystem::IGraphProtocol>();
    assert_trait::<dyn shared_lint_arwaky::filesystem::IParserProtocol>();
    assert_trait::<dyn shared_lint_arwaky::filesystem::IToolResolutionProtocol>();
    assert_trait::<dyn shared_lint_arwaky::filesystem::IWorkspaceProtocol>();
    // Rules
    assert_trait::<dyn shared_lint_arwaky::import_rules::IImportForbiddenProtocol>();
    assert_trait::<dyn shared_lint_arwaky::import_rules::IImportMandatoryProtocol>();
    assert_trait::<dyn shared_lint_arwaky::import_rules::IUnusedImportProtocol>();
    assert_trait::<dyn shared_lint_arwaky::import_rules::IDummyImportCheckerProtocol>();
    assert_trait::<dyn shared_lint_arwaky::import_rules::ICycleImportProtocol>();
    assert_trait::<dyn shared_lint_arwaky::naming_rules::INamingCheckerProtocol>();
    assert_trait::<dyn shared_lint_arwaky::quality_rules::IBypassCheckerProtocol>();
    assert_trait::<dyn shared_lint_arwaky::quality_rules::ILineCheckerProtocol>();
    assert_trait::<dyn shared_lint_arwaky::quality_rules::IMandatoryClassProtocol>();
    assert_trait::<dyn shared_lint_arwaky::quality_rules::ICodeMetricAnalyzerProtocol>();
    assert_trait::<dyn shared_lint_arwaky::quality_rules::IDeadInheritanceProtocol>();
    // Orphan
    assert_trait::<dyn shared_lint_arwaky::orphan_rules::ITaxonomyOrphanProtocol>();
    assert_trait::<dyn shared_lint_arwaky::orphan_rules::IContractOrphanProtocol>();
    assert_trait::<dyn shared_lint_arwaky::orphan_rules::ICapabilitiesOrphanProtocol>();
    assert_trait::<dyn shared_lint_arwaky::orphan_rules::IUtilityOrphanProtocol>();
    assert_trait::<dyn shared_lint_arwaky::orphan_rules::IAgentOrphanProtocol>();
    assert_trait::<dyn shared_lint_arwaky::orphan_rules::ISurfacesOrphanProtocol>();
    assert_trait::<dyn shared_lint_arwaky::orphan_rules::IOrphanParserProtocol>();
    // Role
    assert_trait::<dyn shared_lint_arwaky::role_rules::ITaxonomyRoleProtocol>();
    assert_trait::<dyn shared_lint_arwaky::role_rules::IContractRoleProtocol>();
    assert_trait::<dyn shared_lint_arwaky::role_rules::ICapabilitiesRoleProtocol>();
    assert_trait::<dyn shared_lint_arwaky::role_rules::IUtilityRoleProtocol>();
    assert_trait::<dyn shared_lint_arwaky::role_rules::IAgentRoleProtocol>();
    assert_trait::<dyn shared_lint_arwaky::role_rules::ISurfaceRoleProtocol>();
    // Infrastructure
    assert_trait::<dyn shared_lint_arwaky::auto_fix::IFileAdapterProtocol>();
    assert_trait::<dyn shared_lint_arwaky::auto_fix::IFixProtocol>();
    assert_trait::<dyn shared_lint_arwaky::file_watch::IChangeAnalyzerProtocol>();
    assert_trait::<dyn shared_lint_arwaky::file_watch::IWatchProviderProtocol>();
    assert_trait::<dyn shared_lint_arwaky::git_hooks::IDiffProtocol>();
    assert_trait::<dyn shared_lint_arwaky::git_hooks::IHookProtocol>();
    assert_trait::<dyn shared_lint_arwaky::git_hooks::IHookManagerProtocol>();
    assert_trait::<dyn shared_lint_arwaky::maintenance::IMaintenanceCheckerProtocol>();
    assert_trait::<dyn shared_lint_arwaky::maintenance::IToolExecutorProtocol>();
    assert_trait::<dyn shared_lint_arwaky::external_lint::ILinterAdapterProtocol>();
    assert_trait::<dyn shared_lint_arwaky::external_lint::ICommandExecutorProtocol>();
    assert_trait::<dyn shared_lint_arwaky::external_lint::IExternalLintExecutorProtocol>();
    assert_trait::<dyn shared_lint_arwaky::external_lint::IExternalLintSelectorProtocol>();
    // Surface
    assert_trait::<dyn shared_lint_arwaky::report_formatter::IReportFormatterProtocol>();
    assert_trait::<dyn shared_lint_arwaky::project_setup::ISetupManagementProtocol>();
    assert_trait::<dyn shared_lint_arwaky::project_setup::ISetupInstallerProtocol>();
}
