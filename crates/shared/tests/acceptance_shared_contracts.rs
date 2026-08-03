// Acceptance tests — verify all shared contract traits meet business requirements.
// Each test maps 1:1 to a business requirement for the foundation layer.

fn assert_send_sync<T: Send + Sync + ?Sized>() {}

// FR-001: All contract traits must be Send + Sync (cross-thread safety for async contexts)
#[test]
fn fr_001_all_aggregates_are_send_sync() {
    assert_send_sync::<dyn shared_lint_arwaky::config_system::IConfigOrchestratorAggregate>();
    assert_send_sync::<dyn shared_lint_arwaky::filesystem::IFilesystemAggregate>();
    assert_send_sync::<dyn shared_lint_arwaky::quality_rules::ICodeAnalysisAggregate>();
    assert_send_sync::<dyn shared_lint_arwaky::import_rules::IImportRunnerAggregate>();
    assert_send_sync::<dyn shared_lint_arwaky::naming_rules::INamingRunnerAggregate>();
    assert_send_sync::<dyn shared_lint_arwaky::orphan_rules::IOrphanAggregate>();
    assert_send_sync::<dyn shared_lint_arwaky::role_rules::IRoleRunnerAggregate>();
    assert_send_sync::<dyn shared_lint_arwaky::auto_fix::LintFixOrchestratorAggregate>();
    assert_send_sync::<dyn shared_lint_arwaky::file_watch::IWatchAggregate>();
    assert_send_sync::<dyn shared_lint_arwaky::git_hooks::GitHooksAggregate>();
    assert_send_sync::<dyn shared_lint_arwaky::git_hooks::HookManagementOrchestratorAggregate>();
    assert_send_sync::<dyn shared_lint_arwaky::maintenance::MaintenanceCommandsAggregate>();
    assert_send_sync::<dyn shared_lint_arwaky::external_lint::IExternalLintAggregate>();
    assert_send_sync::<dyn shared_lint_arwaky::report_formatter::IReportFormatterAggregate>();
    assert_send_sync::<dyn shared_lint_arwaky::project_setup::SetupManagementAggregate>();
}

#[test]
fn fr_001_all_protocols_are_send_sync() {
    // Config
    assert_send_sync::<dyn shared_lint_arwaky::config_system::IConfigReaderProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::config_system::IConfigParserProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::config_system::IConfigValidatorProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::config_system::IWorkspaceDetectorProtocol>();
    // Filesystem
    assert_send_sync::<dyn shared_lint_arwaky::filesystem::IFileSystemIOProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::filesystem::IGraphProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::filesystem::IParserProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::filesystem::IToolResolutionProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::filesystem::IWorkspaceProtocol>();
    // Rules
    assert_send_sync::<dyn shared_lint_arwaky::import_rules::IImportForbiddenProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::import_rules::IImportMandatoryProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::import_rules::IUnusedImportProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::import_rules::IDummyImportCheckerProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::import_rules::ICycleImportProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::naming_rules::INamingConventionChecker>();
    assert_send_sync::<dyn shared_lint_arwaky::naming_rules::ISuffixPrefixChecker>();
    assert_send_sync::<dyn shared_lint_arwaky::quality_rules::IBypassCheckerProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::quality_rules::ILineCheckerProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::quality_rules::IMandatoryClassProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::quality_rules::ICodeMetricAnalyzerProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::quality_rules::IDeadInheritanceProtocol>();
    // Orphan
    assert_send_sync::<dyn shared_lint_arwaky::orphan_rules::ITaxonomyOrphanProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::orphan_rules::IContractOrphanProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::orphan_rules::ICapabilitiesOrphanProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::orphan_rules::IUtilityOrphanProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::orphan_rules::IAgentOrphanProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::orphan_rules::ISurfacesOrphanProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::orphan_rules::IOrphanParserProtocol>();
    // Role
    assert_send_sync::<dyn shared_lint_arwaky::role_rules::ITaxonomyRoleChecker>();
    assert_send_sync::<dyn shared_lint_arwaky::role_rules::IContractRoleChecker>();
    assert_send_sync::<dyn shared_lint_arwaky::role_rules::ICapabilitiesRoleChecker>();
    assert_send_sync::<dyn shared_lint_arwaky::role_rules::IUtilityRoleChecker>();
    assert_send_sync::<dyn shared_lint_arwaky::role_rules::IAgentRoleChecker>();
    assert_send_sync::<dyn shared_lint_arwaky::role_rules::ISurfaceRoleChecker>();
    // Infrastructure
    assert_send_sync::<dyn shared_lint_arwaky::auto_fix::IFileAdapterProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::auto_fix::IFixProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::file_watch::IChangeAnalyzerProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::file_watch::IWatchProviderProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::git_hooks::IDiffProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::git_hooks::IHookProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::git_hooks::IHookManagerProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::maintenance::IMaintenanceCheckerProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::maintenance::IToolExecutorProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::external_lint::ILinterAdapterProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::external_lint::ICommandExecutorProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::external_lint::IExternalLintExecutorProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::external_lint::IExternalLintSelectorProtocol>();
    // Surface
    assert_send_sync::<dyn shared_lint_arwaky::report_formatter::IReportFormatterProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::project_setup::ISetupManagementProtocol>();
    assert_send_sync::<dyn shared_lint_arwaky::project_setup::ISetupInstallerProtocol>();
}

// FR-002: Core VOs must be Send + Sync (cross-thread safety)
#[test]
fn fr_002_core_value_objects_are_send_sync() {
    use shared_lint_arwaky::common::{
        AdapterError, ErrorCode, FilePath, Identity, JobId, Language, LintResult, Score, Severity,
        Threshold,
    };
    use shared_lint_arwaky::config_system::{ConfigSource, ProjectConfig};
    use shared_lint_arwaky::filesystem::FileEntry;

    assert_send_sync::<FilePath>();
    assert_send_sync::<Identity>();
    assert_send_sync::<ErrorCode>();
    assert_send_sync::<JobId>();
    assert_send_sync::<Language>();
    assert_send_sync::<shared_lint_arwaky::common::taxonomy_config_language_vo::ConfigLanguage>();
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
    // Config
    assert_trait::<dyn shared_lint_arwaky::config_system::IConfigReaderProtocol>();
    assert_trait::<dyn shared_lint_arwaky::config_system::IConfigParserProtocol>();
    assert_trait::<dyn shared_lint_arwaky::config_system::IConfigValidatorProtocol>();
    assert_trait::<dyn shared_lint_arwaky::config_system::IWorkspaceDetectorProtocol>();
    assert_trait::<dyn shared_lint_arwaky::config_system::IConfigOrchestratorAggregate>();
    // Filesystem
    assert_trait::<dyn shared_lint_arwaky::filesystem::IFileSystemIOProtocol>();
    assert_trait::<dyn shared_lint_arwaky::filesystem::IGraphProtocol>();
    assert_trait::<dyn shared_lint_arwaky::filesystem::IParserProtocol>();
    assert_trait::<dyn shared_lint_arwaky::filesystem::IToolResolutionProtocol>();
    assert_trait::<dyn shared_lint_arwaky::filesystem::IWorkspaceProtocol>();
    assert_trait::<dyn shared_lint_arwaky::filesystem::IFilesystemAggregate>();
    // Rules
    assert_trait::<dyn shared_lint_arwaky::import_rules::IImportForbiddenProtocol>();
    assert_trait::<dyn shared_lint_arwaky::import_rules::IImportMandatoryProtocol>();
    assert_trait::<dyn shared_lint_arwaky::import_rules::IUnusedImportProtocol>();
    assert_trait::<dyn shared_lint_arwaky::import_rules::IDummyImportCheckerProtocol>();
    assert_trait::<dyn shared_lint_arwaky::import_rules::ICycleImportProtocol>();
    assert_trait::<dyn shared_lint_arwaky::import_rules::IImportRunnerAggregate>();
    assert_trait::<dyn shared_lint_arwaky::naming_rules::INamingConventionChecker>();
    assert_trait::<dyn shared_lint_arwaky::naming_rules::ISuffixPrefixChecker>();
    assert_trait::<dyn shared_lint_arwaky::naming_rules::INamingRunnerAggregate>();
    assert_trait::<dyn shared_lint_arwaky::quality_rules::IBypassCheckerProtocol>();
    assert_trait::<dyn shared_lint_arwaky::quality_rules::ILineCheckerProtocol>();
    assert_trait::<dyn shared_lint_arwaky::quality_rules::IMandatoryClassProtocol>();
    assert_trait::<dyn shared_lint_arwaky::quality_rules::ICodeMetricAnalyzerProtocol>();
    assert_trait::<dyn shared_lint_arwaky::quality_rules::IDeadInheritanceProtocol>();
    assert_trait::<dyn shared_lint_arwaky::quality_rules::ICodeAnalysisAggregate>();
    // Orphan
    assert_trait::<dyn shared_lint_arwaky::orphan_rules::ITaxonomyOrphanProtocol>();
    assert_trait::<dyn shared_lint_arwaky::orphan_rules::IContractOrphanProtocol>();
    assert_trait::<dyn shared_lint_arwaky::orphan_rules::ICapabilitiesOrphanProtocol>();
    assert_trait::<dyn shared_lint_arwaky::orphan_rules::IUtilityOrphanProtocol>();
    assert_trait::<dyn shared_lint_arwaky::orphan_rules::IAgentOrphanProtocol>();
    assert_trait::<dyn shared_lint_arwaky::orphan_rules::ISurfacesOrphanProtocol>();
    assert_trait::<dyn shared_lint_arwaky::orphan_rules::IOrphanParserProtocol>();
    assert_trait::<dyn shared_lint_arwaky::orphan_rules::IOrphanAggregate>();
    // Role
    assert_trait::<dyn shared_lint_arwaky::role_rules::ITaxonomyRoleChecker>();
    assert_trait::<dyn shared_lint_arwaky::role_rules::IContractRoleChecker>();
    assert_trait::<dyn shared_lint_arwaky::role_rules::ICapabilitiesRoleChecker>();
    assert_trait::<dyn shared_lint_arwaky::role_rules::IUtilityRoleChecker>();
    assert_trait::<dyn shared_lint_arwaky::role_rules::IAgentRoleChecker>();
    assert_trait::<dyn shared_lint_arwaky::role_rules::ISurfaceRoleChecker>();
    assert_trait::<dyn shared_lint_arwaky::role_rules::IRoleRunnerAggregate>();
    // Infrastructure
    assert_trait::<dyn shared_lint_arwaky::auto_fix::IFileAdapterProtocol>();
    assert_trait::<dyn shared_lint_arwaky::auto_fix::IFixProtocol>();
    assert_trait::<dyn shared_lint_arwaky::auto_fix::LintFixOrchestratorAggregate>();
    assert_trait::<dyn shared_lint_arwaky::file_watch::IChangeAnalyzerProtocol>();
    assert_trait::<dyn shared_lint_arwaky::file_watch::IWatchProviderProtocol>();
    assert_trait::<dyn shared_lint_arwaky::file_watch::IWatchAggregate>();
    assert_trait::<dyn shared_lint_arwaky::git_hooks::IDiffProtocol>();
    assert_trait::<dyn shared_lint_arwaky::git_hooks::IHookProtocol>();
    assert_trait::<dyn shared_lint_arwaky::git_hooks::IHookManagerProtocol>();
    assert_trait::<dyn shared_lint_arwaky::git_hooks::GitHooksAggregate>();
    assert_trait::<dyn shared_lint_arwaky::git_hooks::HookManagementOrchestratorAggregate>();
    assert_trait::<dyn shared_lint_arwaky::maintenance::IMaintenanceCheckerProtocol>();
    assert_trait::<dyn shared_lint_arwaky::maintenance::IToolExecutorProtocol>();
    assert_trait::<dyn shared_lint_arwaky::maintenance::MaintenanceCommandsAggregate>();
    assert_trait::<dyn shared_lint_arwaky::external_lint::ILinterAdapterProtocol>();
    assert_trait::<dyn shared_lint_arwaky::external_lint::ICommandExecutorProtocol>();
    assert_trait::<dyn shared_lint_arwaky::external_lint::IExternalLintExecutorProtocol>();
    assert_trait::<dyn shared_lint_arwaky::external_lint::IExternalLintSelectorProtocol>();
    assert_trait::<dyn shared_lint_arwaky::external_lint::IExternalLintAggregate>();
    // Surface
    assert_trait::<dyn shared_lint_arwaky::report_formatter::IReportFormatterProtocol>();
    assert_trait::<dyn shared_lint_arwaky::report_formatter::IReportFormatterAggregate>();
    assert_trait::<dyn shared_lint_arwaky::project_setup::ISetupManagementProtocol>();
    assert_trait::<dyn shared_lint_arwaky::project_setup::ISetupInstallerProtocol>();
    assert_trait::<dyn shared_lint_arwaky::project_setup::SetupManagementAggregate>();
}
