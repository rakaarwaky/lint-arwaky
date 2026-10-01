// Acceptance tests — verify all shared contract traits meet business requirements.
// Each test maps 1:1 to a business requirement for the foundation layer.

fn assert_send_sync<T: Send + Sync + ?Sized>() {}

// FR-001: All contract traits must be Send + Sync (cross-thread safety for async contexts)
#[test]
fn fr_001_all_aggregates_are_send_sync() {
    assert_send_sync::<dyn shared_config_system::IConfigOrchestratorAggregate>();
    assert_send_sync::<dyn shared_filesystem::IFilesystemAggregate>();
    assert_send_sync::<dyn shared_quality_rules::ICodeAnalysisAggregate>();
    assert_send_sync::<dyn shared_import_rules::IImportRunnerAggregate>();
    assert_send_sync::<dyn shared_naming_rules::INamingRunnerAggregate>();
    assert_send_sync::<dyn shared_orphan_rules::IOrphanAggregate>();
    assert_send_sync::<dyn shared_role_rules::IRoleRunnerAggregate>();
    assert_send_sync::<dyn shared_auto_fix::IFixAggregate>();
    assert_send_sync::<dyn shared_file_watch::IWatchAggregate>();
    assert_send_sync::<dyn shared_git_hooks::IGitHooksAggregate>();
    assert_send_sync::<dyn shared_maintenance::IMaintenanceAggregate>();
    assert_send_sync::<dyn shared_external_lint::IExternalLintAggregate>();
    assert_send_sync::<dyn shared_report_formatter::IReportFormatterAggregate>();
    assert_send_sync::<dyn shared_project_setup::ISetupAggregate>();
}

#[test]
fn fr_001_all_protocols_are_send_sync() {
    // Config
    assert_send_sync::<dyn shared_config_system::IConfigReadProtocol>();
    assert_send_sync::<dyn shared_config_system::IWorkspaceMembersProtocol>();
    assert_send_sync::<dyn shared_config_system::IWorkspaceMembersProtocol>();
    assert_send_sync::<dyn shared_config_system::IConfigMergeProtocol>();
    assert_send_sync::<dyn shared_config_system::IConfigMergeProtocol>();
    assert_send_sync::<dyn shared_config_system::IConfigMergeProtocol>();
    assert_send_sync::<dyn shared_config_system::IConfigReadProtocol>();
    // Filesystem
    assert_send_sync::<dyn shared_filesystem::IFileSystemIOProtocol>();
    assert_send_sync::<dyn shared_filesystem::IGraphProtocol>();
    assert_send_sync::<dyn shared_filesystem::IParserProtocol>();
    assert_send_sync::<dyn shared_filesystem::IToolResolutionProtocol>();
    assert_send_sync::<dyn shared_filesystem::IWorkspaceProtocol>();
    // Rules
    assert_send_sync::<dyn shared_import_rules::IImportForbiddenProtocol>();
    assert_send_sync::<dyn shared_import_rules::IImportMandatoryProtocol>();
    assert_send_sync::<dyn shared_import_rules::IUnusedImportProtocol>();
    assert_send_sync::<dyn shared_import_rules::IDummyImportCheckerProtocol>();
    assert_send_sync::<dyn shared_import_rules::ICycleImportProtocol>();
    assert_send_sync::<dyn shared_naming_rules::INamingConventionProtocol>();
    assert_send_sync::<dyn shared_naming_rules::ISuffixPolicyProtocol>();
    assert_send_sync::<dyn shared_quality_rules::IBypassCheckerProtocol>();
    assert_send_sync::<dyn shared_quality_rules::ILineCheckerProtocol>();
    assert_send_sync::<dyn shared_quality_rules::IMandatoryClassProtocol>();
    assert_send_sync::<dyn shared_quality_rules::ICodeMetricAnalyzerProtocol>();
    assert_send_sync::<dyn shared_quality_rules::IDeadInheritanceProtocol>();
    // Orphan
    assert_send_sync::<dyn shared_orphan_rules::ITaxonomyOrphanProtocol>();
    assert_send_sync::<dyn shared_orphan_rules::IContractOrphanProtocol>();
    assert_send_sync::<dyn shared_orphan_rules::ICapabilitiesOrphanProtocol>();
    assert_send_sync::<dyn shared_orphan_rules::IUtilityOrphanProtocol>();
    assert_send_sync::<dyn shared_orphan_rules::IAgentOrphanProtocol>();
    assert_send_sync::<dyn shared_orphan_rules::ISurfacesOrphanProtocol>();
    assert_send_sync::<dyn shared_orphan_rules::IOrphanParserProtocol>();
    // Role
    assert_send_sync::<dyn shared_role_rules::ITaxonomyRoleProtocol>();
    assert_send_sync::<dyn shared_role_rules::IContractRoleProtocol>();
    assert_send_sync::<dyn shared_role_rules::ICapabilitiesRoleProtocol>();
    assert_send_sync::<dyn shared_role_rules::IUtilityRoleProtocol>();
    assert_send_sync::<dyn shared_role_rules::IAgentRoleProtocol>();
    assert_send_sync::<dyn shared_role_rules::IClassificationProtocol>();
    assert_send_sync::<dyn shared_role_rules::ISurfaceRoleProtocol>();
    // Infrastructure
    assert_send_sync::<dyn shared_auto_fix::IUnusedImportFixProtocol>();
    assert_send_sync::<dyn shared_auto_fix::IBypassFixProtocol>();
    assert_send_sync::<dyn shared_auto_fix::ISymbolRenameProtocol>();
    assert_send_sync::<dyn shared_file_watch::IWatchLifecycleProtocol>();
    assert_send_sync::<dyn shared_file_watch::IChangeFilterProtocol>();
    assert_send_sync::<dyn shared_file_watch::IChangeLintProtocol>();
    assert_send_sync::<dyn shared_file_watch::IChangeLintProtocol>();
    assert_send_sync::<dyn shared_git_hooks::IDiffDetectionProtocol>();
    assert_send_sync::<dyn shared_git_hooks::IHookInstallProtocol>();
    assert_send_sync::<dyn shared_git_hooks::IHookUninstallProtocol>();
    assert_send_sync::<dyn shared_git_hooks::IConfigInitProtocol>();
    assert_send_sync::<dyn shared_maintenance::IDoctorProtocol>();
    assert_send_sync::<dyn shared_maintenance::IProjectStatsProtocol>();
    assert_send_sync::<dyn shared_maintenance::ICacheCleanupProtocol>();
    assert_send_sync::<dyn shared_maintenance::IToolUpdateProtocol>();
    assert_send_sync::<dyn shared_maintenance::ISecurityScanProtocol>();
    assert_send_sync::<dyn shared_maintenance::IDependencyReportProtocol>();
    assert_send_sync::<dyn shared_maintenance::IAdapterHealthProtocol>();
    assert_send_sync::<dyn shared_maintenance::ISelfUpdateProtocol>();
    assert_send_sync::<dyn shared_external_lint::ILinterAdapterProtocol>();
    assert_send_sync::<dyn shared_external_lint::ICommandExecutorProtocol>();
    assert_send_sync::<dyn shared_external_lint::IExternalLintSelectorProtocol>();
    assert_send_sync::<dyn shared_external_lint::INormalizeProtocol>();
    assert_send_sync::<dyn shared_external_lint::IJsToolResolutionProtocol>();
    assert_send_sync::<dyn shared_external_lint::ICargoDirProtocol>();
    // Surface
    assert_send_sync::<dyn shared_report_formatter::ITextFormatProtocol>();
    assert_send_sync::<dyn shared_report_formatter::IJsonFormatProtocol>();
    assert_send_sync::<dyn shared_report_formatter::ISarifFormatProtocol>();
    assert_send_sync::<dyn shared_report_formatter::IJUnitFormatProtocol>();
    // removed — routing in agent layer
    assert_send_sync::<dyn shared_project_setup::IAdapterInstallationProtocol>();
    assert_send_sync::<dyn shared_project_setup::IConfigTemplateProtocol>();
    assert_send_sync::<dyn shared_project_setup::IConfigWritingProtocol>();
    assert_send_sync::<dyn shared_project_setup::IEnvGenerationProtocol>();
    assert_send_sync::<dyn shared_project_setup::IFilePathExistenceProtocol>();
    assert_send_sync::<dyn shared_project_setup::ILanguageDetectionProtocol>();
    assert_send_sync::<dyn shared_project_setup::IMcpConfigGenerationProtocol>();
    assert_send_sync::<dyn shared_project_setup::IPreFlightProtocol>();
}

// FR-002: Core VOs must be Send + Sync (cross-thread safety)
#[test]
fn fr_002_core_value_objects_are_send_sync() {
    use shared_common::{
        AdapterError, ErrorCode, FilePath, Identity, JobId, Language, LintResult, Score, Severity,
        Threshold,
    };
    use shared_config_system::{ConfigSource, ProjectConfig};
    use shared_filesystem::FileEntry;

    assert_send_sync::<FilePath>();
    assert_send_sync::<Identity>();
    assert_send_sync::<ErrorCode>();
    assert_send_sync::<JobId>();
    assert_send_sync::<Language>();
    assert_send_sync::<shared_common::taxonomy_config_language_vo::ConfigLanguage>();
    assert_send_sync::<Severity>();
    assert_send_sync::<Score>();
    assert_send_sync::<Threshold>();
    assert_send_sync::<LintResult>();
    assert_send_sync::<AdapterError>();
    assert_send_sync::<FileEntry>();
    assert_send_sync::<ProjectConfig>();
    assert_send_sync::<ConfigSource>();
}

// FR-003: All contract traits are object-safe (usable as dyn Trait in DI)
#[test]
fn fr_003_all_contract_traits_are_object_safe() {
    fn assert_trait<T: ?Sized>() {}
    // Config — 10 protocols + 1 aggregate = 11 contract seams
    assert_trait::<dyn shared_config_system::IConfigReadProtocol>();
    assert_trait::<dyn shared_config_system::IWorkspaceMembersProtocol>();
    assert_trait::<dyn shared_config_system::IWorkspaceMembersProtocol>();
    assert_trait::<dyn shared_config_system::IConfigMergeProtocol>();
    assert_trait::<dyn shared_config_system::IConfigMergeProtocol>();
    assert_trait::<dyn shared_config_system::IConfigMergeProtocol>();
    assert_trait::<dyn shared_config_system::IConfigReadProtocol>();
    assert_trait::<dyn shared_config_system::IConfigOrchestratorAggregate>();
    // Filesystem
    assert_trait::<dyn shared_filesystem::IFileSystemIOProtocol>();
    assert_trait::<dyn shared_filesystem::IGraphProtocol>();
    assert_trait::<dyn shared_filesystem::IParserProtocol>();
    assert_trait::<dyn shared_filesystem::IToolResolutionProtocol>();
    assert_trait::<dyn shared_filesystem::IWorkspaceProtocol>();
    assert_trait::<dyn shared_filesystem::IFilesystemAggregate>();
    // Rules
    assert_trait::<dyn shared_import_rules::IImportForbiddenProtocol>();
    assert_trait::<dyn shared_import_rules::IImportMandatoryProtocol>();
    assert_trait::<dyn shared_import_rules::IUnusedImportProtocol>();
    assert_trait::<dyn shared_import_rules::IDummyImportCheckerProtocol>();
    assert_trait::<dyn shared_import_rules::ICycleImportProtocol>();
    assert_trait::<dyn shared_import_rules::IImportRunnerAggregate>();
    assert_trait::<dyn shared_naming_rules::INamingConventionProtocol>();
    assert_trait::<dyn shared_naming_rules::ISuffixPolicyProtocol>();
    assert_trait::<dyn shared_naming_rules::INamingRunnerAggregate>();
    assert_trait::<dyn shared_quality_rules::IBypassCheckerProtocol>();
    assert_trait::<dyn shared_quality_rules::ILineCheckerProtocol>();
    assert_trait::<dyn shared_quality_rules::IMandatoryClassProtocol>();
    assert_trait::<dyn shared_quality_rules::ICodeMetricAnalyzerProtocol>();
    assert_trait::<dyn shared_quality_rules::IDeadInheritanceProtocol>();
    assert_trait::<dyn shared_quality_rules::ICodeAnalysisAggregate>();
    // Orphan
    assert_trait::<dyn shared_orphan_rules::ITaxonomyOrphanProtocol>();
    assert_trait::<dyn shared_orphan_rules::IContractOrphanProtocol>();
    assert_trait::<dyn shared_orphan_rules::ICapabilitiesOrphanProtocol>();
    assert_trait::<dyn shared_orphan_rules::IUtilityOrphanProtocol>();
    assert_trait::<dyn shared_orphan_rules::IAgentOrphanProtocol>();
    assert_trait::<dyn shared_orphan_rules::ISurfacesOrphanProtocol>();
    assert_trait::<dyn shared_orphan_rules::IOrphanParserProtocol>();
    assert_trait::<dyn shared_orphan_rules::IOrphanAggregate>();
    // Role
    assert_trait::<dyn shared_role_rules::ITaxonomyRoleProtocol>();
    assert_trait::<dyn shared_role_rules::IContractRoleProtocol>();
    assert_trait::<dyn shared_role_rules::ICapabilitiesRoleProtocol>();
    assert_trait::<dyn shared_role_rules::IUtilityRoleProtocol>();
    assert_trait::<dyn shared_role_rules::IAgentRoleProtocol>();
    assert_trait::<dyn shared_role_rules::IClassificationProtocol>();
    assert_trait::<dyn shared_role_rules::ISurfaceRoleProtocol>();
    assert_trait::<dyn shared_role_rules::IRoleRunnerAggregate>();
    // Infrastructure
    assert_trait::<dyn shared_auto_fix::IUnusedImportFixProtocol>();
    assert_trait::<dyn shared_auto_fix::IBypassFixProtocol>();
    assert_trait::<dyn shared_auto_fix::ISymbolRenameProtocol>();
    assert_trait::<dyn shared_auto_fix::IFixAggregate>();
    assert_trait::<dyn shared_file_watch::IWatchLifecycleProtocol>();
    assert_trait::<dyn shared_file_watch::IChangeFilterProtocol>();
    assert_trait::<dyn shared_file_watch::IChangeLintProtocol>();
    assert_trait::<dyn shared_file_watch::IWatchAggregate>();
    assert_trait::<dyn shared_git_hooks::IDiffDetectionProtocol>();
    assert_trait::<dyn shared_git_hooks::IHookInstallProtocol>();
    assert_trait::<dyn shared_git_hooks::IHookUninstallProtocol>();
    assert_trait::<dyn shared_git_hooks::IConfigInitProtocol>();
    assert_trait::<dyn shared_git_hooks::IGitHooksAggregate>();
    assert_trait::<dyn shared_maintenance::IDoctorProtocol>();
    assert_trait::<dyn shared_maintenance::IProjectStatsProtocol>();
    assert_trait::<dyn shared_maintenance::ICacheCleanupProtocol>();
    assert_trait::<dyn shared_maintenance::IToolUpdateProtocol>();
    assert_trait::<dyn shared_maintenance::ISecurityScanProtocol>();
    assert_trait::<dyn shared_maintenance::IDependencyReportProtocol>();
    assert_trait::<dyn shared_maintenance::IAdapterHealthProtocol>();
    assert_trait::<dyn shared_maintenance::ISelfUpdateProtocol>();
    assert_trait::<dyn shared_maintenance::IMaintenanceAggregate>();
    assert_trait::<dyn shared_external_lint::ILinterAdapterProtocol>();
    assert_trait::<dyn shared_external_lint::ICommandExecutorProtocol>();
    assert_trait::<dyn shared_external_lint::IExternalLintSelectorProtocol>();
    assert_trait::<dyn shared_external_lint::INormalizeProtocol>();
    assert_trait::<dyn shared_external_lint::IJsToolResolutionProtocol>();
    assert_trait::<dyn shared_external_lint::ICargoDirProtocol>();
    assert_trait::<dyn shared_external_lint::IExternalLintAggregate>();
    // Surface
    assert_trait::<dyn shared_report_formatter::ITextFormatProtocol>();
    assert_trait::<dyn shared_report_formatter::IJsonFormatProtocol>();
    assert_trait::<dyn shared_report_formatter::ISarifFormatProtocol>();
    assert_trait::<dyn shared_report_formatter::IJUnitFormatProtocol>();
    // removed — routing in agent layer
    assert_trait::<dyn shared_report_formatter::IReportFormatterAggregate>();
    assert_trait::<dyn shared_project_setup::IAdapterInstallationProtocol>();
    assert_trait::<dyn shared_project_setup::IConfigTemplateProtocol>();
    assert_trait::<dyn shared_project_setup::IConfigWritingProtocol>();
    assert_trait::<dyn shared_project_setup::IEnvGenerationProtocol>();
    assert_trait::<dyn shared_project_setup::IFilePathExistenceProtocol>();
    assert_trait::<dyn shared_project_setup::ILanguageDetectionProtocol>();
    assert_trait::<dyn shared_project_setup::IMcpConfigGenerationProtocol>();
    assert_trait::<dyn shared_project_setup::IPreFlightProtocol>();
    assert_trait::<dyn shared_project_setup::ISetupAggregate>();
}
