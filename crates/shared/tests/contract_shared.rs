// Contract tests — verify every contract trait declared by the shared crate
// is usable as a bound, object-safe where declared, and Send + Sync.
// shared is the foundation crate: it declares contracts but implements none.

use shared_lint_arwaky::auto_fix::{
    IFileAdapterProtocol, IFixProtocol, LintFixOrchestratorAggregate,
};
use shared_lint_arwaky::config_system::{
    IConfigOrchestratorAggregate, IConfigParserProtocol, IConfigReaderProtocol,
    IConfigValidatorProtocol, IWorkspaceDetectorProtocol,
};
use shared_lint_arwaky::external_lint::{
    ICommandExecutorProtocol, IExternalLintAggregate, IExternalLintExecutorProtocol,
    IExternalLintSelectorProtocol, ILinterAdapterProtocol,
};
use shared_lint_arwaky::file_watch::{
    IChangeAnalyzerProtocol, IWatchAggregate, IWatchProviderProtocol,
};
use shared_lint_arwaky::filesystem::{
    IFileSystemIOProtocol, IFilesystemAggregate, IGraphProtocol, IParserProtocol,
    IToolResolutionProtocol, IWorkspaceProtocol,
};
use shared_lint_arwaky::git_hooks::{
    GitHooksAggregate, HookManagementOrchestratorAggregate, IDiffProtocol, IHookManagerProtocol,
    IHookProtocol,
};
use shared_lint_arwaky::import_rules::{
    ICycleImportProtocol, IDummyImportCheckerProtocol, IImportForbiddenProtocol,
    IImportMandatoryProtocol, IImportRunnerAggregate, IUnusedImportProtocol,
};
use shared_lint_arwaky::maintenance::{
    IMaintenanceCheckerProtocol, IToolExecutorProtocol, MaintenanceCommandsAggregate,
};
use shared_lint_arwaky::naming_rules::{
    INamingConventionChecker, INamingRunnerAggregate, ISuffixPrefixChecker,
};
use shared_lint_arwaky::orphan_rules::{
    IAgentOrphanProtocol, ICapabilitiesOrphanProtocol, IContractOrphanProtocol, IOrphanAggregate,
    IOrphanParserProtocol, ISurfacesOrphanProtocol, ITaxonomyOrphanProtocol,
    IUtilityOrphanProtocol,
};
use shared_lint_arwaky::project_setup::{
    ISetupInstallerProtocol, ISetupManagementProtocol, SetupManagementAggregate,
};
use shared_lint_arwaky::quality_rules::{
    IBypassCheckerProtocol, ICodeAnalysisAggregate, ICodeMetricAnalyzerProtocol,
    IDeadInheritanceProtocol, ILineCheckerProtocol, IMandatoryClassProtocol,
};
use shared_lint_arwaky::report_formatter::{IReportFormatterAggregate, IReportFormatterProtocol};
use shared_lint_arwaky::role_rules::{
    IAgentRoleChecker, ICapabilitiesRoleChecker, IContractRoleChecker, IRoleRunnerAggregate,
    ISurfaceRoleChecker, ITaxonomyRoleChecker, IUtilityRoleChecker,
};

fn assert_trait<T: ?Sized>() {}
fn assert_send_sync<T: Send + Sync + ?Sized>() {}

// ── Config-system contracts ─────────────────────────────────
#[test]
fn config_contracts_are_traits() {
    assert_trait::<dyn IConfigReaderProtocol>();
    assert_trait::<dyn IConfigParserProtocol>();
    assert_trait::<dyn IConfigValidatorProtocol>();
    assert_trait::<dyn IWorkspaceDetectorProtocol>();
    assert_trait::<dyn IConfigOrchestratorAggregate>();
}

#[test]
fn config_contracts_are_send_sync() {
    assert_send_sync::<dyn IConfigReaderProtocol>();
    assert_send_sync::<dyn IConfigParserProtocol>();
    assert_send_sync::<dyn IConfigValidatorProtocol>();
    assert_send_sync::<dyn IWorkspaceDetectorProtocol>();
    assert_send_sync::<dyn IConfigOrchestratorAggregate>();
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
    assert_trait::<dyn INamingConventionChecker>();
    assert_trait::<dyn ISuffixPrefixChecker>();
    assert_trait::<dyn INamingRunnerAggregate>();
}

#[test]
fn naming_rule_contracts_are_send_sync() {
    assert_send_sync::<dyn INamingConventionChecker>();
    assert_send_sync::<dyn ISuffixPrefixChecker>();
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
    assert_trait::<dyn ITaxonomyRoleChecker>();
    assert_trait::<dyn IContractRoleChecker>();
    assert_trait::<dyn ICapabilitiesRoleChecker>();
    assert_trait::<dyn IUtilityRoleChecker>();
    assert_trait::<dyn IAgentRoleChecker>();
    assert_trait::<dyn ISurfaceRoleChecker>();
    assert_trait::<dyn IRoleRunnerAggregate>();
}

#[test]
fn role_rule_contracts_are_send_sync() {
    assert_send_sync::<dyn ITaxonomyRoleChecker>();
    assert_send_sync::<dyn IContractRoleChecker>();
    assert_send_sync::<dyn ICapabilitiesRoleChecker>();
    assert_send_sync::<dyn IUtilityRoleChecker>();
    assert_send_sync::<dyn IAgentRoleChecker>();
    assert_send_sync::<dyn ISurfaceRoleChecker>();
    assert_send_sync::<dyn IRoleRunnerAggregate>();
}

// ── Infrastructure contracts ────────────────────────────────
#[test]
fn auto_fix_contracts_are_traits() {
    assert_trait::<dyn IFileAdapterProtocol>();
    assert_trait::<dyn IFixProtocol>();
    assert_trait::<dyn LintFixOrchestratorAggregate>();
}

#[test]
fn auto_fix_contracts_are_send_sync() {
    assert_send_sync::<dyn IFileAdapterProtocol>();
    assert_send_sync::<dyn IFixProtocol>();
    assert_send_sync::<dyn LintFixOrchestratorAggregate>();
}

#[test]
fn file_watch_contracts_are_traits() {
    // IChangeAnalyzerProtocol is not dyn compatible (has non-object-safe methods)
    assert_trait::<dyn IWatchProviderProtocol>();
    assert_trait::<dyn IWatchAggregate>();
}

#[test]
fn file_watch_contracts_are_send_sync() {
    // IChangeAnalyzerProtocol is not dyn compatible
    assert_send_sync::<dyn IWatchProviderProtocol>();
    assert_send_sync::<dyn IWatchAggregate>();
}

#[test]
fn git_hooks_contracts_are_traits() {
    assert_trait::<dyn IDiffProtocol>();
    assert_trait::<dyn IHookProtocol>();
    assert_trait::<dyn IHookManagerProtocol>();
    assert_trait::<dyn GitHooksAggregate>();
    assert_trait::<dyn HookManagementOrchestratorAggregate>();
}

#[test]
fn git_hooks_contracts_are_send_sync() {
    assert_send_sync::<dyn IDiffProtocol>();
    assert_send_sync::<dyn IHookProtocol>();
    assert_send_sync::<dyn IHookManagerProtocol>();
    assert_send_sync::<dyn GitHooksAggregate>();
    assert_send_sync::<dyn HookManagementOrchestratorAggregate>();
}

#[test]
fn maintenance_contracts_are_traits() {
    assert_trait::<dyn IMaintenanceCheckerProtocol>();
    assert_trait::<dyn IToolExecutorProtocol>();
    assert_trait::<dyn MaintenanceCommandsAggregate>();
}

#[test]
fn maintenance_contracts_are_send_sync() {
    assert_send_sync::<dyn IMaintenanceCheckerProtocol>();
    assert_send_sync::<dyn IToolExecutorProtocol>();
    assert_send_sync::<dyn MaintenanceCommandsAggregate>();
}

#[test]
fn external_lint_contracts_are_traits() {
    assert_trait::<dyn ILinterAdapterProtocol>();
    assert_trait::<dyn ICommandExecutorProtocol>();
    assert_trait::<dyn IExternalLintExecutorProtocol>();
    assert_trait::<dyn IExternalLintSelectorProtocol>();
    assert_trait::<dyn IExternalLintAggregate>();
}

#[test]
fn external_lint_contracts_are_send_sync() {
    assert_send_sync::<dyn ILinterAdapterProtocol>();
    assert_send_sync::<dyn ICommandExecutorProtocol>();
    assert_send_sync::<dyn IExternalLintExecutorProtocol>();
    assert_send_sync::<dyn IExternalLintSelectorProtocol>();
    assert_send_sync::<dyn IExternalLintAggregate>();
}

// ── Surface contracts ───────────────────────────────────────
#[test]
fn report_formatter_contracts_are_traits() {
    assert_trait::<dyn IReportFormatterProtocol>();
    assert_trait::<dyn IReportFormatterAggregate>();
}

#[test]
fn report_formatter_contracts_are_send_sync() {
    assert_send_sync::<dyn IReportFormatterProtocol>();
    assert_send_sync::<dyn IReportFormatterAggregate>();
}

#[test]
fn project_setup_contracts_are_traits() {
    assert_trait::<dyn ISetupManagementProtocol>();
    assert_trait::<dyn ISetupInstallerProtocol>();
    assert_trait::<dyn SetupManagementAggregate>();
}

#[test]
fn project_setup_contracts_are_send_sync() {
    assert_send_sync::<dyn ISetupManagementProtocol>();
    assert_send_sync::<dyn ISetupInstallerProtocol>();
    assert_send_sync::<dyn SetupManagementAggregate>();
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
