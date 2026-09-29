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
    // Config — 10 protocols + 1 aggregate
    assert_trait::<dyn shared_lint_arwaky::config_system::IConfigReadProtocol>();
    assert_trait::<dyn shared_lint_arwaky::config_system::IConfigLanguageProtocol>();
    assert_trait::<dyn shared_lint_arwaky::config_system::IWorkspaceDetectProtocol>();
    assert_trait::<dyn shared_lint_arwaky::config_system::IWorkspaceMembersProtocol>();
    assert_trait::<dyn shared_lint_arwaky::config_system::IConfigTomlProtocol>();
    assert_trait::<dyn shared_lint_arwaky::config_system::IConfigParseProtocol>();
    assert_trait::<dyn shared_lint_arwaky::config_system::IConfigValidateProtocol>();
    assert_trait::<dyn shared_lint_arwaky::config_system::IConfigCacheProtocol>();
    assert_trait::<dyn shared_lint_arwaky::config_system::IConfigIgnoredPathsProtocol>();
    assert_trait::<dyn shared_lint_arwaky::config_system::IConfigListProtocol>();
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
    assert_trait::<dyn shared_lint_arwaky::naming_rules::INamingConventionProtocol>();
    assert_trait::<dyn shared_lint_arwaky::naming_rules::ISuffixPolicyProtocol>();
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
    assert_trait::<dyn shared_lint_arwaky::role_rules::IClassificationProtocol>();
    assert_trait::<dyn shared_lint_arwaky::role_rules::ISurfaceRoleProtocol>();
    // Infrastructure
    assert_trait::<dyn shared_lint_arwaky::auto_fix::IUnusedImportFixProtocol>();
    assert_trait::<dyn shared_lint_arwaky::auto_fix::IBypassFixProtocol>();
    assert_trait::<dyn shared_lint_arwaky::auto_fix::ISymbolRenameProtocol>();
    assert_trait::<dyn shared_lint_arwaky::file_watch::IWatchLifecycleProtocol>();
    assert_trait::<dyn shared_lint_arwaky::file_watch::IChangeFilterProtocol>();
    assert_trait::<dyn shared_lint_arwaky::file_watch::IChangeLintProtocol>();
    assert_trait::<dyn shared_lint_arwaky::file_watch::IChangeLintProtocol>();
    assert_trait::<dyn shared_lint_arwaky::git_hooks::IDiffDetectionProtocol>();
    assert_trait::<dyn shared_lint_arwaky::git_hooks::IHookInstallProtocol>();
    assert_trait::<dyn shared_lint_arwaky::git_hooks::IHookUninstallProtocol>();
    assert_trait::<dyn shared_lint_arwaky::git_hooks::IConfigInitProtocol>();
    assert_trait::<dyn shared_lint_arwaky::maintenance::IDoctorProtocol>();
    assert_trait::<dyn shared_lint_arwaky::maintenance::IProjectStatsProtocol>();
    assert_trait::<dyn shared_lint_arwaky::maintenance::ICacheCleanupProtocol>();
    assert_trait::<dyn shared_lint_arwaky::maintenance::IToolUpdateProtocol>();
    assert_trait::<dyn shared_lint_arwaky::maintenance::IToolchainDiagnosticProtocol>();
    assert_trait::<dyn shared_lint_arwaky::maintenance::ISecurityScanProtocol>();
    assert_trait::<dyn shared_lint_arwaky::maintenance::IDependencyReportProtocol>();
    assert_trait::<dyn shared_lint_arwaky::maintenance::IAdapterHealthProtocol>();
    assert_trait::<dyn shared_lint_arwaky::maintenance::ISelfUpdateProtocol>();
    assert_trait::<dyn shared_lint_arwaky::external_lint::ILinterAdapterProtocol>();
    assert_trait::<dyn shared_lint_arwaky::external_lint::IAdapterScanProtocol>();
    assert_trait::<dyn shared_lint_arwaky::external_lint::INormalizeProtocol>();
    // Surface
    assert_trait::<dyn shared_lint_arwaky::report_formatter::ITextFormatProtocol>();
    assert_trait::<dyn shared_lint_arwaky::report_formatter::IJsonFormatProtocol>();
    assert_trait::<dyn shared_lint_arwaky::report_formatter::ISarifFormatProtocol>();
    assert_trait::<dyn shared_lint_arwaky::report_formatter::IJUnitFormatProtocol>();
    // removed — routing in agent layer
    assert_trait::<dyn shared_lint_arwaky::project_setup::IAdapterInstallationProtocol>();
    assert_trait::<dyn shared_lint_arwaky::project_setup::IConfigTemplateProtocol>();
    assert_trait::<dyn shared_lint_arwaky::project_setup::IConfigWritingProtocol>();
    assert_trait::<dyn shared_lint_arwaky::project_setup::IEnvGenerationProtocol>();
    assert_trait::<dyn shared_lint_arwaky::project_setup::IFilePathExistenceProtocol>();
    assert_trait::<dyn shared_lint_arwaky::project_setup::ILanguageDetectionProtocol>();
    assert_trait::<dyn shared_lint_arwaky::project_setup::IMcpConfigGenerationProtocol>();
    assert_trait::<dyn shared_lint_arwaky::project_setup::IPreFlightProtocol>();
}
