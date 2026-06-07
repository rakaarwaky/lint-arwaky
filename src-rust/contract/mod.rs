//! # Contract Layer — The Abstraction Boundaries
//!
//! This module is the **formal promise layer** of the AES architecture. It defines
//! _what_ can be done without defining _how_. Every type in this layer is either:
//!
//! - A **Port** (`_port`): Outbound interface for technical operations (I/O, DB, Network).
//!   Implemented by **Infrastructure**.
//! - A **Protocol** (`_protocol`): Inbound interface for use cases or domain calculations.
//!   Implemented by **Capabilities**.
//! - An **Aggregate** (`_aggregate`): Composition-based facade grouping related ports/protocols.
//!   Implemented by **Agent**.
//!
//! ## Layer Rules (AES Compliance)
//! - **Allowed Imports**: `src/taxonomy/` and `src/contract/` only.
//! - **Forbidden Imports**: `src/capabilities/`, `src/infrastructure/`, `src/agent/`, `src/surfaces/`.
//!   Violations trigger **AES001**.
//! - **Allowed Suffixes**: `_port`, `_protocol`, `_aggregate` (**AES008**).
//! - **Primitive Usage**: Forbidden in trait signatures (**AES006**). Must use taxonomy VOs.
//!
//! ## Import Patterns
//!
//! Import specific interfaces directly from the barrel:
//!
//! ```rust,ignore
//! use lint_arwaky::contract::{IFileSystemPort, IArchLintProtocol, ServiceContainerAggregate};
//! ```
//!
//! ## Architectural Invariants
//!
//! - **AES026**: Aggregates may inherit from both Ports and Protocols (composition of
//!   outbound dependencies and inbound use cases). Composition via fields is also valid
//!   when inheritance is not appropriate.
//! - **AES007**: All imports of contract types must come from this barrel (`contract::*`),
//!   not from internal submodules.
//! - **AES027**: Any layer importing a contract must have at least one struct that
//!   implements it (prevents dead contract syndrome).
//!
//! ## Notes on Shims
//!
//! Some modules in this layer are intentionally retained as **backward-compat shims**
//! (e.g. `naming_variant_port`) so that historical import paths
//! continue to compile. They are not dead code: the port/protocol split between
//! `INamingVariantPort` (Infrastructure) and `INamingVariantProtocol` (Capabilities)
//! is a deliberate design choice, even though their method signatures coincide.

// ═══════════════════════════════════════════════════════════════════════════════
// MODULE DECLARATIONS
// ═══════════════════════════════════════════════════════════════════════════════

pub mod adapter_container_aggregate;
pub mod agent_lifecycle_aggregate;
pub mod analysis_orchestrator_aggregate;
pub mod architecture_compliance_port;
pub mod architecture_compliance_protocol;
pub mod architecture_coordinator_aggregate;
pub mod architecture_cycle_protocol;
pub mod architecture_import_protocol;
pub mod architecture_inheritance_protocol;
pub mod architecture_lint_protocol;
pub mod architecture_orphan_protocol;
pub mod architecture_rule_protocol;
pub mod architecture_unused_protocol;
pub mod capability_container_aggregate;
pub mod command_executor_port;
pub mod config_discovery_port;
pub mod config_orchestration_protocol;
pub mod config_parser_port;
pub mod config_reader_port;
pub mod container_registry_aggregate;
pub mod dev_commands_aggregate;
pub mod diff_result_aggregate;
pub mod directory_watch_aggregate;
pub mod dispatch_commands_aggregate;
pub mod dispatch_routing_protocol;
pub mod domain_type_protocol;
pub mod execution_orchestrator_aggregate;
pub mod file_system_port;
pub mod git_commands_aggregate;
pub mod hook_manager_port;
pub mod hook_orchestrator_aggregate;
pub mod http_provider_port;
pub mod infrastructure_container_aggregate;
pub mod javascript_flow_port;
pub mod javascript_scope_port;
pub mod job_registry_aggregate;
pub mod job_registry_port;
pub mod language_detector_port;
pub mod lint_fix_aggregate;
pub mod lint_reporting_protocol;
pub mod linter_adapter_port;
pub mod maintenance_commands_aggregate;
pub mod mcp_server_port;
pub mod metrics_provider_port;
pub mod multi_project_aggregate;
pub mod naming_provider_port;
pub mod naming_variant_port;
pub mod naming_variant_protocol;
pub mod orchestrator_container_aggregate;
pub mod output_client_aggregate;
pub mod path_normalization_port;
pub mod pipeline_dispatcher_aggregate;
pub mod pipeline_extended_aggregate;
pub mod pipeline_input_aggregate;
pub mod pipeline_output_aggregate;
pub mod plugin_commands_aggregate;
pub mod plugin_manager_port;
pub mod project_governance_protocol;
pub mod project_orchestrator_aggregate;
pub mod report_commands_aggregate;
pub mod scanner_provider_port;
pub mod semantic_flow_protocol;
pub mod semantic_tracer_port;
pub mod semantic_tracer_protocol;
pub mod service_container_aggregate;
pub mod setup_management_aggregate;
pub mod setup_management_protocol;
pub mod source_parser_port;
pub mod watch_commands_aggregate;
pub mod watch_orchestrator_aggregate;
pub mod watch_provider_port;

pub use adapter_container_aggregate::AdapterContainerAggregate;
pub use agent_lifecycle_aggregate::AgentLifecycleAggregate;
pub use analysis_orchestrator_aggregate::AnalysisOrchestratorAggregate;
pub use architecture_compliance_port::IArchCompliancePort;
pub use architecture_compliance_protocol::{IArchComplianceProtocol, IScopeBoundaryProtocol};
pub use architecture_coordinator_aggregate::ArchCoordinatorAggregate;
pub use architecture_cycle_protocol::ICycleAnalysisProtocol;
pub use architecture_import_protocol::IArchImportProtocol;
pub use architecture_inheritance_protocol::IArchInheritanceProtocol;
pub use architecture_lint_protocol::IArchLintProtocol;
pub use architecture_orphan_protocol::IArchOrphanProtocol;
pub use architecture_rule_protocol::{
    IAnalyzer, IArchRuleProtocol, IInternalCheckerProtocol, IMetricCheckerProtocol,
    INamingCheckerProtocol, INamingRuleProtocol,
};
pub use architecture_unused_protocol::IUnusedProtocol;
pub use capability_container_aggregate::CapabilityContainerAggregate;
pub use command_executor_port::ICommandExecutorPort;
pub use config_discovery_port::IConfigDiscoveryPort;
pub use config_orchestration_protocol::IConfigOrchestrationProtocol;
pub use config_parser_port::IConfigParserPort;
pub use config_reader_port::IConfigReaderPort;
pub use container_registry_aggregate::ContainerRegistryAggregate;
pub use dev_commands_aggregate::DevCommandsAggregate;
pub use diff_result_aggregate::GitDiffResultAggregate;
pub use directory_watch_aggregate::DirectoryWatchAggregate;
pub use dispatch_commands_aggregate::command_catalog;
pub use dispatch_routing_protocol::{IDispatchRoutingParserProtocol, IDispatchRoutingProtocol};
pub use domain_type_protocol::IDomainTypeProtocol;
pub use execution_orchestrator_aggregate::PipelineExecutionOrchestratorAggregate;
pub use file_system_port::IFileSystemPort;
pub use git_commands_aggregate::GitCommandsAggregate;
pub use hook_manager_port::IHookManagerPort;
pub use hook_orchestrator_aggregate::HookManagementOrchestratorAggregate;
pub use http_provider_port::IHttpProviderPort;
pub use infrastructure_container_aggregate::InfrastructureContainerAggregate;
pub use javascript_flow_port::IJavascriptFlowPort;
pub use javascript_scope_port::{IJavascriptScopePort, IJsTracerPort};
pub use job_registry_aggregate::JobRegistryAggregate;
pub use job_registry_port::IJobRegistryPort;
pub use language_detector_port::ILanguageDetectorPort;
pub use lint_fix_aggregate::LintFixOrchestratorAggregate;
pub use lint_reporting_protocol::ILintReportingProtocol;
pub use linter_adapter_port::ILinterAdapterPort;
pub use maintenance_commands_aggregate::MaintenanceCommandsAggregate;
pub use mcp_server_port::{IMcpServerPort, ToolHandler};
pub use metrics_provider_port::IMetricsProviderPort;
pub use multi_project_aggregate::MultiProjectAggregate;
pub use naming_provider_port::INamingProviderPort;
pub use naming_variant_port::INamingVariantPort;
pub use naming_variant_protocol::INamingVariantProtocol;
pub use orchestrator_container_aggregate::OrchestratorContainerAggregate;
pub use output_client_aggregate::OutputClientAggregate;
pub use path_normalization_port::IPathNormalizationPort;
pub use pipeline_dispatcher_aggregate::PipelineActionDispatcherAggregate;
pub use pipeline_extended_aggregate::PipelineExtendedOrchestratorAggregate;
pub use pipeline_input_aggregate::PipelineInputAggregate;
pub use pipeline_output_aggregate::PipelineOutputAggregate;
pub use plugin_commands_aggregate::PluginCommandsAggregate;
pub use plugin_manager_port::IPluginManagerPort;
pub use project_governance_protocol::IConfigRulesProtocol;
pub use project_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
pub use report_commands_aggregate::ReportCommandsAggregate;
pub use scanner_provider_port::IScannerProviderPort;
pub use semantic_flow_protocol::IDataFlowProtocol;
pub use semantic_tracer_port::ISemanticTracerPort;
pub use semantic_tracer_protocol::ISemanticTracerProtocol;
pub use service_container_aggregate::ServiceContainerAggregate;
pub use setup_management_aggregate::SetupManagementAggregate;
pub use setup_management_protocol::ISetupManagementProtocol;
pub use source_parser_port::ISourceParserPort;
// No unique public exports in source_system_port
pub use watch_commands_aggregate::WatchCommandsAggregate;
pub use watch_orchestrator_aggregate::WatchExecutionOrchestratorAggregate;
pub use watch_provider_port::IWatchProviderPort;

// ═══════════════════════════════════════════════════════════════════════════════
// TYPE ALIASES (Ergonomic Shortcuts for Complex Trait Object Types)
// ═══════════════════════════════════════════════════════════════════════════════

/// Arc-wrapped file system port — the standard DI container signature.
pub type FileSystemPortRef = std::sync::Arc<dyn IFileSystemPort>;

/// Arc-wrapped source parser port.
pub type SourceParserPortRef = std::sync::Arc<dyn ISourceParserPort>;

/// Arc-wrapped command executor port.
pub type CommandExecutorPortRef = std::sync::Arc<dyn ICommandExecutorPort>;

/// Arc-wrapped job registry port.
pub type JobRegistryPortRef = std::sync::Arc<dyn IJobRegistryPort>;

/// Arc-wrapped architecture linter protocol — the primary surface-facing trait.
pub type ArchLintProtocolRef = std::sync::Arc<dyn IArchLintProtocol>;

/// Arc-wrapped service container aggregate — the DI root.
pub type ServiceContainerRef = std::sync::Arc<dyn ServiceContainerAggregate>;

/// Arc-wrapped linter adapter port (generic adapter).
pub type LinterAdapterPortRef = std::sync::Arc<dyn ILinterAdapterPort>;

/// Arc-wrapped semantic tracer port.
pub type SemanticTracerPortRef = std::sync::Arc<dyn ISemanticTracerPort>;

// ═══════════════════════════════════════════════════════════════════════════════
// UTILITY FUNCTIONS (Contract-level helpers)
// ═══════════════════════════════════════════════════════════════════════════════

/// Look up a command's metadata from the contract-level command catalog.
///
/// Returns `None` if the command is not registered.
#[inline]
pub fn find_command(name: &str) -> Option<crate::taxonomy::CommandMetadataVO> {
    command_catalog()
        .get(&crate::taxonomy::ActionName::from(name))
        .cloned()
}

/// Check whether a command name is registered in the catalog.
#[inline]
pub fn is_command_registered(name: &str) -> bool {
    command_catalog().contains_key(&crate::taxonomy::ActionName::from(name))
}

/// Return the total number of registered commands.
#[inline]
pub fn command_count() -> usize {
    command_catalog().len()
}

/// List all registered command names in sorted order.
pub fn list_command_names() -> Vec<String> {
    let mut names: Vec<String> = command_catalog()
        .keys()
        .map(|k| k.value().to_string())
        .collect();
    names.sort();
    names
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::{
        command_catalog, command_count, find_command, is_command_registered, list_command_names,
    };

    #[test]
    fn test_command_catalog_not_empty() {
        assert!(!command_catalog().is_empty());
    }

    #[test]
    fn test_find_command_check() {
        let meta = find_command("check");
        assert!(meta.is_some());
        let meta = meta.unwrap();
        assert!(!meta.description.value.is_empty());
        assert!(!meta.example.value.is_empty());
    }

    #[test]
    fn test_find_command_unknown() {
        assert!(find_command("nonexistent-command-xyz").is_none());
    }

    #[test]
    fn test_is_command_registered() {
        assert!(is_command_registered("check"));
        assert!(is_command_registered("fix"));
        assert!(is_command_registered("report"));
        assert!(!is_command_registered("foobar"));
    }

    #[test]
    fn test_command_count() {
        let count = command_count();
        assert!(count > 0);
        assert!(count < 100); // sanity bound
    }

    #[test]
    fn test_list_command_names_sorted() {
        let names = list_command_names();
        assert!(!names.is_empty());
        for window in names.windows(2) {
            assert!(
                window[0] <= window[1],
                "names not sorted: {} > {}",
                window[0],
                window[1]
            );
        }
    }

    #[test]
    fn test_core_commands_present() {
        let core = ["check", "scan", "fix", "report", "ci", "version"];
        for cmd in core {
            assert!(
                is_command_registered(cmd),
                "core command '{}' missing from catalog",
                cmd
            );
        }
    }
}
