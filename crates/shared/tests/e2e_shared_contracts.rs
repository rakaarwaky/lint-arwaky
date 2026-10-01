// E2E tests — verify all contract traits are object-safe and usable across crate boundaries.

// Verify Arc<dyn Trait> works for all aggregates (DI wiring pattern)
#[test]
fn e2e_all_aggregates_support_arc_dynamic_dispatch() {
    fn assert_arc<T: ?Sized>() {}
    assert_arc::<dyn shared_config_system::IConfigOrchestratorAggregate>();
    assert_arc::<dyn shared_filesystem::IFilesystemAggregate>();
    assert_arc::<dyn shared_quality_rules::ICodeAnalysisAggregate>();
    assert_arc::<dyn shared_import_rules::IImportRunnerAggregate>();
    assert_arc::<dyn shared_naming_rules::INamingRunnerAggregate>();
    assert_arc::<dyn shared_orphan_rules::IOrphanAggregate>();
    assert_arc::<dyn shared_role_rules::IRoleRunnerAggregate>();
    assert_arc::<dyn shared_auto_fix::IFixAggregate>();
    assert_arc::<dyn shared_file_watch::IWatchAggregate>();
    assert_arc::<dyn shared_git_hooks::IGitHooksAggregate>();
    assert_arc::<dyn shared_maintenance::IMaintenanceAggregate>();
    assert_arc::<dyn shared_external_lint::IExternalLintAggregate>();
    assert_arc::<dyn shared_report_formatter::IReportFormatterAggregate>();
    assert_arc::<dyn shared_project_setup::ISetupAggregate>();
}

// Verify all protocols are object-safe (used as dyn in DI)
#[test]
fn e2e_all_protocols_are_object_safe() {
    fn assert_trait<T: ?Sized>() {}
    // Config — 3 protocols + 1 aggregate
    assert_trait::<dyn shared_config_system::IConfigReadProtocol>();
    assert_trait::<dyn shared_config_system::IWorkspaceMembersProtocol>();
    assert_trait::<dyn shared_config_system::IConfigMergeProtocol>();
    // Filesystem
    assert_trait::<dyn shared_filesystem::IFileSystemIOProtocol>();
    assert_trait::<dyn shared_filesystem::IGraphProtocol>();
    assert_trait::<dyn shared_filesystem::IParserProtocol>();
    assert_trait::<dyn shared_filesystem::IToolResolutionProtocol>();
    assert_trait::<dyn shared_filesystem::IWorkspaceProtocol>();
    // Rules
    assert_trait::<dyn shared_import_rules::IImportForbiddenProtocol>();
    assert_trait::<dyn shared_import_rules::IImportMandatoryProtocol>();
    assert_trait::<dyn shared_import_rules::IUnusedImportProtocol>();
    assert_trait::<dyn shared_import_rules::IDummyImportCheckerProtocol>();
    assert_trait::<dyn shared_import_rules::ICycleImportProtocol>();
    assert_trait::<dyn shared_naming_rules::INamingConventionProtocol>();
    assert_trait::<dyn shared_naming_rules::ISuffixPolicyProtocol>();
    assert_trait::<dyn shared_quality_rules::IBypassCheckerProtocol>();
    assert_trait::<dyn shared_quality_rules::ILineCheckerProtocol>();
    assert_trait::<dyn shared_quality_rules::IMandatoryClassProtocol>();
    assert_trait::<dyn shared_quality_rules::ICodeMetricAnalyzerProtocol>();
    assert_trait::<dyn shared_quality_rules::IDeadInheritanceProtocol>();
    // Orphan
    assert_trait::<dyn shared_orphan_rules::ITaxonomyOrphanProtocol>();
    assert_trait::<dyn shared_orphan_rules::IContractOrphanProtocol>();
    assert_trait::<dyn shared_orphan_rules::ICapabilitiesOrphanProtocol>();
    assert_trait::<dyn shared_orphan_rules::IUtilityOrphanProtocol>();
    assert_trait::<dyn shared_orphan_rules::IAgentOrphanProtocol>();
    assert_trait::<dyn shared_orphan_rules::ISurfacesOrphanProtocol>();
    assert_trait::<dyn shared_orphan_rules::IOrphanParserProtocol>();
    // Role
    assert_trait::<dyn shared_role_rules::ITaxonomyRoleProtocol>();
    assert_trait::<dyn shared_role_rules::IContractRoleProtocol>();
    assert_trait::<dyn shared_role_rules::ICapabilitiesRoleProtocol>();
    assert_trait::<dyn shared_role_rules::IUtilityRoleProtocol>();
    assert_trait::<dyn shared_role_rules::IAgentRoleProtocol>();
    assert_trait::<dyn shared_role_rules::IClassificationProtocol>();
    assert_trait::<dyn shared_role_rules::ISurfaceRoleProtocol>();
    // Infrastructure
    assert_trait::<dyn shared_auto_fix::IUnusedImportFixProtocol>();
    assert_trait::<dyn shared_auto_fix::IBypassFixProtocol>();
    assert_trait::<dyn shared_auto_fix::ISymbolRenameProtocol>();
    assert_trait::<dyn shared_file_watch::IWatchLifecycleProtocol>();
    assert_trait::<dyn shared_file_watch::IChangeFilterProtocol>();
    assert_trait::<dyn shared_file_watch::IChangeLintProtocol>();
    assert_trait::<dyn shared_file_watch::IChangeLintProtocol>();
    assert_trait::<dyn shared_git_hooks::IDiffDetectionProtocol>();
    assert_trait::<dyn shared_git_hooks::IHookInstallProtocol>();
    assert_trait::<dyn shared_git_hooks::IHookUninstallProtocol>();
    assert_trait::<dyn shared_git_hooks::IConfigInitProtocol>();
    assert_trait::<dyn shared_maintenance::IDoctorProtocol>();
    assert_trait::<dyn shared_maintenance::IProjectStatsProtocol>();
    assert_trait::<dyn shared_maintenance::ICacheCleanupProtocol>();
    assert_trait::<dyn shared_maintenance::IToolUpdateProtocol>();
    assert_trait::<dyn shared_maintenance::ISecurityScanProtocol>();
    assert_trait::<dyn shared_maintenance::IDependencyReportProtocol>();
    assert_trait::<dyn shared_maintenance::IAdapterHealthProtocol>();
    assert_trait::<dyn shared_maintenance::ISelfUpdateProtocol>();
    assert_trait::<dyn shared_external_lint::ILinterAdapterProtocol>();
    assert_trait::<dyn shared_external_lint::ICommandExecutorProtocol>();
    assert_trait::<dyn shared_external_lint::IExternalLintSelectorProtocol>();
    assert_trait::<dyn shared_external_lint::INormalizeProtocol>();
    assert_trait::<dyn shared_external_lint::IJsToolResolutionProtocol>();
    assert_trait::<dyn shared_external_lint::ICargoDirProtocol>();
    // Surface
    assert_trait::<dyn shared_report_formatter::ITextFormatProtocol>();
    assert_trait::<dyn shared_report_formatter::IJsonFormatProtocol>();
    assert_trait::<dyn shared_report_formatter::ISarifFormatProtocol>();
    assert_trait::<dyn shared_report_formatter::IJUnitFormatProtocol>();
    // removed — routing in agent layer
    assert_trait::<dyn shared_project_setup::IAdapterInstallationProtocol>();
    assert_trait::<dyn shared_project_setup::IConfigTemplateProtocol>();
    assert_trait::<dyn shared_project_setup::IConfigWritingProtocol>();
    assert_trait::<dyn shared_project_setup::IEnvGenerationProtocol>();
    assert_trait::<dyn shared_project_setup::IFilePathExistenceProtocol>();
    assert_trait::<dyn shared_project_setup::ILanguageDetectionProtocol>();
    assert_trait::<dyn shared_project_setup::IMcpConfigGenerationProtocol>();
    assert_trait::<dyn shared_project_setup::IPreFlightProtocol>();
}
