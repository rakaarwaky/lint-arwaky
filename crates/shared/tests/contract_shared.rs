// Contract tests — verify every contract trait declared by the shared crate
// is usable as a bound, object-safe where declared, and Send + Sync.
// shared is the foundation crate: it declares contracts but implements none.

use shared_lint_arwaky::auto_fix::{
    IBypassFixProtocol, IFileAdapterProtocol, IFixAggregate, IFixPipelineProtocol,
    IManualReportProtocol, ISymbolRenameProtocol, IUnusedImportFixProtocol,
};
use shared_lint_arwaky::config_system::{
    IConfigCacheProtocol, IConfigIgnoredPathsProtocol, IConfigLanguageProtocol,
    IConfigListProtocol, IConfigOrchestratorAggregate, IConfigParseProtocol, IConfigReadProtocol,
    IConfigTomlProtocol, IConfigValidateProtocol, IWorkspaceDetectProtocol,
    IWorkspaceMembersProtocol,
};
use shared_lint_arwaky::external_lint::{
    IAdapterScanProtocol, ICargoDirProtocol, ICommandExecutorProtocol, IExternalLintAggregate,
    IExternalLintSelectorProtocol, IJsToolResolutionProtocol, ILanguageDetectProtocol,
    ILinterAdapterProtocol, INormalizeProtocol,
};
use shared_lint_arwaky::file_watch::{
    IChangeFilterProtocol, IChangeLintProtocol, IWatchAggregate, IWatchLifecycleProtocol,
};
use shared_lint_arwaky::filesystem::{
    IFileSystemIOProtocol, IFilesystemAggregate, IGraphProtocol, IParserProtocol,
    IToolResolutionProtocol, IWorkspaceProtocol,
};
use shared_lint_arwaky::git_hooks::{
    IConfigInitProtocol, IDiffDetectionProtocol, IGitHooksAggregate, IHookInstallProtocol,
    IHookUninstallProtocol,
};
use shared_lint_arwaky::import_rules::{
    ICycleImportProtocol, IDummyImportCheckerProtocol, IImportForbiddenProtocol,
    IImportMandatoryProtocol, IImportRunnerAggregate, IUnusedImportProtocol,
};
use shared_lint_arwaky::maintenance::{
    IAdapterHealthProtocol, ICacheCleanupProtocol, IDependencyReportProtocol, IDoctorProtocol,
    IMaintenanceAggregate, IProjectStatsProtocol, ISecurityScanProtocol, ISelfUpdateProtocol,
    IToolExecutorProtocol, IToolUpdateProtocol, IToolchainDiagnosticProtocol,
};
use shared_lint_arwaky::naming_rules::{
    INamingConventionProtocol, INamingRunnerAggregate, ISuffixPolicyProtocol,
};
use shared_lint_arwaky::orphan_rules::{
    IAgentOrphanProtocol, ICapabilitiesOrphanProtocol, IContractOrphanProtocol, IOrphanAggregate,
    IOrphanParserProtocol, ISurfacesOrphanProtocol, ITaxonomyOrphanProtocol,
    IUtilityOrphanProtocol,
};
use shared_lint_arwaky::project_setup::{
    IAdapterInstallationProtocol, IConfigTemplateProtocol, IConfigWritingProtocol,
    IEnvGenerationProtocol, IFilePathExistenceProtocol, ILanguageDetectionProtocol,
    IMcpConfigGenerationProtocol, IPreFlightProtocol, ISetupAggregate,
};
use shared_lint_arwaky::quality_rules::{
    IBypassCheckerProtocol, ICodeAnalysisAggregate, ICodeMetricAnalyzerProtocol,
    IDeadInheritanceProtocol, ILineCheckerProtocol, IMandatoryClassProtocol,
};
use shared_lint_arwaky::report_formatter::{
    IJUnitFormatProtocol, IJsonFormatProtocol, IReportFormatterAggregate, ISarifFormatProtocol,
    ITextFormatProtocol,
};
use shared_lint_arwaky::role_rules::{
    IAgentRoleProtocol, ICapabilitiesRoleProtocol, IClassificationProtocol, IContractRoleProtocol,
    IRoleRunnerAggregate, ISurfaceRoleProtocol, ITaxonomyRoleProtocol, IUtilityRoleProtocol,
};

fn assert_trait<T: ?Sized>() {}
fn assert_send_sync<T: Send + Sync + ?Sized>() {}

// ── Config-system contracts ─────────────────────────────────
// One `#[test]` per trait, each with a uniquely named local helper so no
// helper name repeats inside a single function.
#[test]
fn config_read_contract_is_a_trait() {
    fn assert_read_trait<T: ?Sized>() {}
    assert_read_trait::<dyn IConfigReadProtocol>();
}

#[test]
fn config_language_contract_is_a_trait() {
    fn assert_language_trait<T: ?Sized>() {}
    assert_language_trait::<dyn IConfigLanguageProtocol>();
}

#[test]
fn workspace_detect_contract_is_a_trait() {
    fn assert_detect_trait<T: ?Sized>() {}
    assert_detect_trait::<dyn IWorkspaceDetectProtocol>();
}

#[test]
fn workspace_members_contract_is_a_trait() {
    fn assert_members_trait<T: ?Sized>() {}
    assert_members_trait::<dyn IWorkspaceMembersProtocol>();
}

#[test]
fn config_parse_contract_is_a_trait() {
    fn assert_parse_trait<T: ?Sized>() {}
    assert_parse_trait::<dyn IConfigParseProtocol>();
}

#[test]
fn config_validate_contract_is_a_trait() {
    fn assert_validate_trait<T: ?Sized>() {}
    assert_validate_trait::<dyn IConfigValidateProtocol>();
}

#[test]
fn config_cache_contract_is_a_trait() {
    fn assert_cache_trait<T: ?Sized>() {}
    assert_cache_trait::<dyn IConfigCacheProtocol>();
}

#[test]
fn config_ignored_paths_contract_is_a_trait() {
    fn assert_ignored_paths_trait<T: ?Sized>() {}
    assert_ignored_paths_trait::<dyn IConfigIgnoredPathsProtocol>();
}

#[test]
fn config_toml_contract_is_a_trait() {
    fn assert_toml_trait<T: ?Sized>() {}
    assert_toml_trait::<dyn IConfigTomlProtocol>();
}

#[test]
fn config_list_contract_is_a_trait() {
    fn assert_list_trait<T: ?Sized>() {}
    assert_list_trait::<dyn IConfigListProtocol>();
}

#[test]
fn config_orchestrator_aggregate_contract_is_a_trait() {
    fn assert_aggregate_trait<T: ?Sized>() {}
    assert_aggregate_trait::<dyn IConfigOrchestratorAggregate>();
}

#[test]
fn config_contracts_are_send_sync() {
    fn assert_read_sync<T: Send + Sync + ?Sized>() {}
    fn assert_language_sync<T: Send + Sync + ?Sized>() {}
    fn assert_detect_sync<T: Send + Sync + ?Sized>() {}
    fn assert_members_sync<T: Send + Sync + ?Sized>() {}
    fn assert_parse_sync<T: Send + Sync + ?Sized>() {}
    fn assert_validate_sync<T: Send + Sync + ?Sized>() {}
    fn assert_cache_sync<T: Send + Sync + ?Sized>() {}
    fn assert_ignored_paths_sync<T: Send + Sync + ?Sized>() {}
    fn assert_toml_sync<T: Send + Sync + ?Sized>() {}
    fn assert_list_sync<T: Send + Sync + ?Sized>() {}
    fn assert_aggregate_sync<T: Send + Sync + ?Sized>() {}
    assert_read_sync::<dyn IConfigReadProtocol>();
    assert_language_sync::<dyn IConfigLanguageProtocol>();
    assert_detect_sync::<dyn IWorkspaceDetectProtocol>();
    assert_members_sync::<dyn IWorkspaceMembersProtocol>();
    assert_parse_sync::<dyn IConfigParseProtocol>();
    assert_validate_sync::<dyn IConfigValidateProtocol>();
    assert_cache_sync::<dyn IConfigCacheProtocol>();
    assert_ignored_paths_sync::<dyn IConfigIgnoredPathsProtocol>();
    assert_toml_sync::<dyn IConfigTomlProtocol>();
    assert_list_sync::<dyn IConfigListProtocol>();
    assert_aggregate_sync::<dyn IConfigOrchestratorAggregate>();
}

// ── Filesystem contracts ────────────────────────────────────
#[test]
fn filesystem_contracts_are_traits() {
    assert_trait::<dyn IFileSystemIOProtocol>();
    assert_trait::<dyn IGraphProtocol>();
    assert_trait::<dyn IParserProtocol>();
    assert_trait::<dyn IToolResolutionProtocol>();
    assert_trait::<dyn IWorkspaceProtocol>();
    assert_trait::<dyn IFilesystemAggregate>();
}

#[test]
fn filesystem_contracts_are_send_sync() {
    assert_send_sync::<dyn IFileSystemIOProtocol>();
    assert_send_sync::<dyn IGraphProtocol>();
    assert_send_sync::<dyn IParserProtocol>();
    assert_send_sync::<dyn IToolResolutionProtocol>();
    assert_send_sync::<dyn IWorkspaceProtocol>();
    assert_send_sync::<dyn IFilesystemAggregate>();
}

// ── Lint-rule contracts ─────────────────────────────────────
#[test]
fn import_rule_contracts_are_traits() {
    assert_trait::<dyn IImportForbiddenProtocol>();
    assert_trait::<dyn IImportMandatoryProtocol>();
    assert_trait::<dyn IUnusedImportProtocol>();
    assert_trait::<dyn IDummyImportCheckerProtocol>();
    assert_trait::<dyn ICycleImportProtocol>();
    assert_trait::<dyn IImportRunnerAggregate>();
}

#[test]
fn import_rule_contracts_are_send_sync() {
    assert_send_sync::<dyn IImportForbiddenProtocol>();
    assert_send_sync::<dyn IImportMandatoryProtocol>();
    assert_send_sync::<dyn IUnusedImportProtocol>();
    assert_send_sync::<dyn IDummyImportCheckerProtocol>();
    assert_send_sync::<dyn ICycleImportProtocol>();
    assert_send_sync::<dyn IImportRunnerAggregate>();
}

#[test]
fn naming_rule_contracts_are_traits() {
    assert_trait::<dyn INamingConventionProtocol>();
    assert_trait::<dyn ISuffixPolicyProtocol>();
    assert_trait::<dyn INamingRunnerAggregate>();
}

#[test]
fn naming_rule_contracts_are_send_sync() {
    assert_send_sync::<dyn INamingConventionProtocol>();
    assert_send_sync::<dyn ISuffixPolicyProtocol>();
    assert_send_sync::<dyn INamingRunnerAggregate>();
}

#[test]
fn quality_rule_contracts_are_traits() {
    assert_trait::<dyn IBypassCheckerProtocol>();
    assert_trait::<dyn ILineCheckerProtocol>();
    assert_trait::<dyn IMandatoryClassProtocol>();
    assert_trait::<dyn ICodeMetricAnalyzerProtocol>();
    assert_trait::<dyn IDeadInheritanceProtocol>();
    assert_trait::<dyn ICodeAnalysisAggregate>();
}

#[test]
fn quality_rule_contracts_are_send_sync() {
    assert_send_sync::<dyn IBypassCheckerProtocol>();
    assert_send_sync::<dyn ILineCheckerProtocol>();
    assert_send_sync::<dyn IMandatoryClassProtocol>();
    assert_send_sync::<dyn ICodeMetricAnalyzerProtocol>();
    assert_send_sync::<dyn IDeadInheritanceProtocol>();
    assert_send_sync::<dyn ICodeAnalysisAggregate>();
}

#[test]
fn orphan_rule_contracts_are_traits() {
    assert_trait::<dyn ITaxonomyOrphanProtocol>();
    assert_trait::<dyn IContractOrphanProtocol>();
    assert_trait::<dyn ICapabilitiesOrphanProtocol>();
    assert_trait::<dyn IUtilityOrphanProtocol>();
    assert_trait::<dyn IAgentOrphanProtocol>();
    assert_trait::<dyn ISurfacesOrphanProtocol>();
    assert_trait::<dyn IOrphanParserProtocol>();
    assert_trait::<dyn IOrphanAggregate>();
}

#[test]
fn orphan_rule_contracts_are_send_sync() {
    assert_send_sync::<dyn ITaxonomyOrphanProtocol>();
    assert_send_sync::<dyn IContractOrphanProtocol>();
    assert_send_sync::<dyn ICapabilitiesOrphanProtocol>();
    assert_send_sync::<dyn IUtilityOrphanProtocol>();
    assert_send_sync::<dyn IAgentOrphanProtocol>();
    assert_send_sync::<dyn ISurfacesOrphanProtocol>();
    assert_send_sync::<dyn IOrphanParserProtocol>();
    assert_send_sync::<dyn IOrphanAggregate>();
}

#[test]
fn role_rule_contracts_are_traits() {
    assert_trait::<dyn IClassificationProtocol>();
    assert_trait::<dyn ITaxonomyRoleProtocol>();
    assert_trait::<dyn IContractRoleProtocol>();
    assert_trait::<dyn ICapabilitiesRoleProtocol>();
    assert_trait::<dyn IUtilityRoleProtocol>();
    assert_trait::<dyn IAgentRoleProtocol>();
    assert_trait::<dyn ISurfaceRoleProtocol>();
    assert_trait::<dyn IRoleRunnerAggregate>();
}

#[test]
fn role_rule_contracts_are_send_sync() {
    assert_send_sync::<dyn IClassificationProtocol>();
    assert_send_sync::<dyn ITaxonomyRoleProtocol>();
    assert_send_sync::<dyn IContractRoleProtocol>();
    assert_send_sync::<dyn ICapabilitiesRoleProtocol>();
    assert_send_sync::<dyn IUtilityRoleProtocol>();
    assert_send_sync::<dyn IAgentRoleProtocol>();
    assert_send_sync::<dyn ISurfaceRoleProtocol>();
    assert_send_sync::<dyn IRoleRunnerAggregate>();
}

// ── Infrastructure contracts ────────────────────────────────
#[test]
fn auto_fix_contracts_are_traits() {
    assert_trait::<dyn IFileAdapterProtocol>();
    assert_trait::<dyn IUnusedImportFixProtocol>();
    assert_trait::<dyn IBypassFixProtocol>();
    assert_trait::<dyn ISymbolRenameProtocol>();
    assert_trait::<dyn IFixPipelineProtocol>();
    assert_trait::<dyn IManualReportProtocol>();
    assert_trait::<dyn IFixAggregate>();
}

#[test]
fn auto_fix_contracts_are_send_sync() {
    assert_send_sync::<dyn IFileAdapterProtocol>();
    assert_send_sync::<dyn IUnusedImportFixProtocol>();
    assert_send_sync::<dyn IBypassFixProtocol>();
    assert_send_sync::<dyn ISymbolRenameProtocol>();
    assert_send_sync::<dyn IFixPipelineProtocol>();
    assert_send_sync::<dyn IManualReportProtocol>();
    assert_send_sync::<dyn IFixAggregate>();
}

#[test]
fn file_watch_contracts_are_traits() {
    assert_trait::<dyn IWatchLifecycleProtocol>();
    assert_trait::<dyn IChangeFilterProtocol>();
    assert_trait::<dyn IChangeLintProtocol>();
    assert_trait::<dyn IWatchAggregate>();
}

#[test]
fn file_watch_contracts_are_send_sync() {
    assert_send_sync::<dyn IWatchLifecycleProtocol>();
    assert_send_sync::<dyn IChangeFilterProtocol>();
    assert_send_sync::<dyn IChangeLintProtocol>();
    assert_send_sync::<dyn IWatchAggregate>();
}

#[test]
fn git_hooks_contracts_are_traits() {
    assert_trait::<dyn IDiffDetectionProtocol>();
    assert_trait::<dyn IHookInstallProtocol>();
    assert_trait::<dyn IHookUninstallProtocol>();
    assert_trait::<dyn IConfigInitProtocol>();
    assert_trait::<dyn IGitHooksAggregate>();
}

#[test]
fn git_hooks_contracts_are_send_sync() {
    assert_send_sync::<dyn IDiffDetectionProtocol>();
    assert_send_sync::<dyn IHookInstallProtocol>();
    assert_send_sync::<dyn IHookUninstallProtocol>();
    assert_send_sync::<dyn IConfigInitProtocol>();
    assert_send_sync::<dyn IGitHooksAggregate>();
}

#[test]
fn maintenance_contracts_are_traits() {
    assert_trait::<dyn IDoctorProtocol>();
    assert_trait::<dyn IProjectStatsProtocol>();
    assert_trait::<dyn ICacheCleanupProtocol>();
    assert_trait::<dyn IToolUpdateProtocol>();
    assert_trait::<dyn IToolchainDiagnosticProtocol>();
    assert_trait::<dyn ISecurityScanProtocol>();
    assert_trait::<dyn IDependencyReportProtocol>();
    assert_trait::<dyn IAdapterHealthProtocol>();
    assert_trait::<dyn ISelfUpdateProtocol>();
    assert_trait::<dyn IToolExecutorProtocol>();
    assert_trait::<dyn IMaintenanceAggregate>();
}

#[test]
fn maintenance_contracts_are_send_sync() {
    assert_send_sync::<dyn IDoctorProtocol>();
    assert_send_sync::<dyn IProjectStatsProtocol>();
    assert_send_sync::<dyn ICacheCleanupProtocol>();
    assert_send_sync::<dyn IToolUpdateProtocol>();
    assert_send_sync::<dyn IToolchainDiagnosticProtocol>();
    assert_send_sync::<dyn ISecurityScanProtocol>();
    assert_send_sync::<dyn IDependencyReportProtocol>();
    assert_send_sync::<dyn IAdapterHealthProtocol>();
    assert_send_sync::<dyn ISelfUpdateProtocol>();
    assert_send_sync::<dyn IToolExecutorProtocol>();
    assert_send_sync::<dyn IMaintenanceAggregate>();
}

// One `#[test]` per trait, each with a uniquely named local helper so no
// helper name repeats inside a single function.
#[test]
fn external_language_detect_contract_is_a_trait() {
    fn assert_language_detect_trait<T: ?Sized>() {}
    assert_language_detect_trait::<dyn ILanguageDetectProtocol>();
}

#[test]
fn external_selector_contract_is_a_trait() {
    fn assert_selector_trait<T: ?Sized>() {}
    assert_selector_trait::<dyn IExternalLintSelectorProtocol>();
}

#[test]
fn external_linter_adapter_contract_is_a_trait() {
    fn assert_adapter_trait<T: ?Sized>() {}
    assert_adapter_trait::<dyn ILinterAdapterProtocol>();
}

#[test]
fn external_normalize_contract_is_a_trait() {
    fn assert_normalize_trait<T: ?Sized>() {}
    assert_normalize_trait::<dyn INormalizeProtocol>();
}

#[test]
fn external_command_executor_contract_is_a_trait() {
    fn assert_command_executor_trait<T: ?Sized>() {}
    assert_command_executor_trait::<dyn ICommandExecutorProtocol>();
}

#[test]
fn external_js_resolution_contract_is_a_trait() {
    fn assert_js_resolution_trait<T: ?Sized>() {}
    assert_js_resolution_trait::<dyn IJsToolResolutionProtocol>();
}

#[test]
fn external_cargo_dir_contract_is_a_trait() {
    fn assert_cargo_dir_trait<T: ?Sized>() {}
    assert_cargo_dir_trait::<dyn ICargoDirProtocol>();
}

#[test]
fn external_lint_aggregate_contract_is_a_trait() {
    fn assert_aggregate_trait<T: ?Sized>() {}
    assert_aggregate_trait::<dyn IExternalLintAggregate>();
}

#[test]
fn external_adapter_scan_contract_is_a_trait() {
    fn assert_adapter_scan_trait<T: ?Sized>() {}
    assert_adapter_scan_trait::<dyn IAdapterScanProtocol>();
}

#[test]
fn external_lint_contracts_are_send_sync() {
    fn assert_detect_sync<T: Send + Sync + ?Sized>() {}
    fn assert_selector_sync<T: Send + Sync + ?Sized>() {}
    fn assert_adapter_sync<T: Send + Sync + ?Sized>() {}
    fn assert_normalize_sync<T: Send + Sync + ?Sized>() {}
    fn assert_command_sync<T: Send + Sync + ?Sized>() {}
    fn assert_js_sync<T: Send + Sync + ?Sized>() {}
    fn assert_cargo_sync<T: Send + Sync + ?Sized>() {}
    fn assert_aggregate_sync<T: Send + Sync + ?Sized>() {}
    fn assert_adapter_scan_sync<T: Send + Sync + ?Sized>() {}
    assert_detect_sync::<dyn ILanguageDetectProtocol>();
    assert_selector_sync::<dyn IExternalLintSelectorProtocol>();
    assert_adapter_sync::<dyn ILinterAdapterProtocol>();
    assert_normalize_sync::<dyn INormalizeProtocol>();
    assert_command_sync::<dyn ICommandExecutorProtocol>();
    assert_js_sync::<dyn IJsToolResolutionProtocol>();
    assert_cargo_sync::<dyn ICargoDirProtocol>();
    assert_aggregate_sync::<dyn IExternalLintAggregate>();
    assert_adapter_scan_sync::<dyn IAdapterScanProtocol>();
}

// ── Surface contracts ───────────────────────────────────────
#[test]
fn report_formatter_contracts_are_traits() {
    assert_trait::<dyn ITextFormatProtocol>();
    assert_trait::<dyn IJsonFormatProtocol>();
    assert_trait::<dyn ISarifFormatProtocol>();
    assert_trait::<dyn IJUnitFormatProtocol>();
    // IFormatDelegationProtocol removed — routing is in agent layer
    assert_trait::<dyn IReportFormatterAggregate>();
}

#[test]
fn report_formatter_contracts_are_send_sync() {
    assert_send_sync::<dyn ITextFormatProtocol>();
    assert_send_sync::<dyn IJsonFormatProtocol>();
    assert_send_sync::<dyn ISarifFormatProtocol>();
    assert_send_sync::<dyn IJUnitFormatProtocol>();
    // removed
    assert_send_sync::<dyn IReportFormatterAggregate>();
}

#[test]
fn project_setup_contracts_are_traits() {
    assert_trait::<dyn IAdapterInstallationProtocol>();
    assert_trait::<dyn IConfigTemplateProtocol>();
    assert_trait::<dyn IConfigWritingProtocol>();
    assert_trait::<dyn IEnvGenerationProtocol>();
    assert_trait::<dyn IFilePathExistenceProtocol>();
    assert_trait::<dyn ILanguageDetectionProtocol>();
    assert_trait::<dyn IMcpConfigGenerationProtocol>();
    assert_trait::<dyn IPreFlightProtocol>();
    assert_trait::<dyn ISetupAggregate>();
}

#[test]
fn project_setup_contracts_are_send_sync() {
    assert_send_sync::<dyn IAdapterInstallationProtocol>();
    assert_send_sync::<dyn IConfigTemplateProtocol>();
    assert_send_sync::<dyn IConfigWritingProtocol>();
    assert_send_sync::<dyn IEnvGenerationProtocol>();
    assert_send_sync::<dyn IFilePathExistenceProtocol>();
    assert_send_sync::<dyn ILanguageDetectionProtocol>();
    assert_send_sync::<dyn IMcpConfigGenerationProtocol>();
    assert_send_sync::<dyn IPreFlightProtocol>();
    assert_send_sync::<dyn ISetupAggregate>();
}

// ── Core VOs are Send + Sync (used across async boundaries) ─
#[test]
fn core_value_objects_are_send_sync() {
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
