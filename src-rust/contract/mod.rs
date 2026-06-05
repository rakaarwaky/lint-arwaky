// Barrel exports for the Contract layer (AES Architecture)

pub mod adapter_container_aggregate;
pub mod agent_lifecycle_aggregate;
pub mod analysis_orchestrator_aggregate;
pub mod architecture_analyzer_protocol;
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
pub mod code_transformation_protocol;
pub mod command_executor_port;
pub mod config_discovery_port;
pub mod config_parser_port;
pub mod config_provider_port;
pub mod config_validation_port;
pub mod container_registry_aggregate;
pub mod dev_commands_aggregate;
pub mod diff_result_aggregate;
pub mod directory_watch_aggregate;
pub mod dispatch_check_aggregate;
pub mod dispatch_commands_aggregate;
pub mod dispatch_fix_aggregate;
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
pub mod pipeline_orchestrator_aggregate;
pub mod plugin_commands_aggregate;
pub mod plugin_manager_port;
pub mod project_container_aggregate;
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
pub mod source_system_port;
pub mod watch_commands_aggregate;
pub mod watch_orchestrator_aggregate;
pub mod watch_provider_port;

// ---- Re-exports (canonical sources) ----
pub use adapter_container_aggregate::*;
pub use agent_lifecycle_aggregate::*;
pub use analysis_orchestrator_aggregate::*;
pub use architecture_analyzer_protocol::*;
pub use architecture_compliance_port::*;
pub use architecture_compliance_protocol::*;
pub use architecture_coordinator_aggregate::*;
pub use architecture_cycle_protocol::*;
pub use architecture_import_protocol::*;
pub use architecture_inheritance_protocol::*;
pub use architecture_lint_protocol::*;
pub use architecture_orphan_protocol::*;
pub use architecture_unused_protocol::*;
pub use capability_container_aggregate::*;
pub use code_transformation_protocol::*;
pub use command_executor_port::*;
pub use config_discovery_port::*;
pub use config_parser_port::*;
pub use config_provider_port::*;
pub use config_validation_port::*;
pub use container_registry_aggregate::*;
pub use dev_commands_aggregate::*;
pub use diff_result_aggregate::*;
pub use directory_watch_aggregate::*;
pub use dispatch_check_aggregate::*;
pub use dispatch_commands_aggregate::*;
pub use dispatch_fix_aggregate::*;
pub use dispatch_routing_protocol::*;
pub use domain_type_protocol::*;
pub use execution_orchestrator_aggregate::*;
pub use file_system_port::*;
pub use git_commands_aggregate::*;
pub use hook_manager_port::*;
pub use hook_orchestrator_aggregate::*;
pub use http_provider_port::*;
pub use infrastructure_container_aggregate::*;
pub use javascript_flow_port::*;
pub use javascript_scope_port::*;
pub use job_registry_aggregate::*;
pub use job_registry_port::*;
pub use lint_fix_aggregate::*;
pub use lint_reporting_protocol::*;
pub use linter_adapter_port::*;
pub use maintenance_commands_aggregate::*;
pub use mcp_server_port::*;
pub use metrics_provider_port::*;
pub use multi_project_aggregate::*;
pub use naming_provider_port::*;
pub use naming_variant_port::*;
pub use naming_variant_protocol::*;
pub use orchestrator_container_aggregate::*;
pub use output_client_aggregate::*;
pub use path_normalization_port::*;
pub use pipeline_dispatcher_aggregate::*;
pub use pipeline_extended_aggregate::*;
pub use pipeline_orchestrator_aggregate::*;
pub use plugin_commands_aggregate::*;
pub use plugin_manager_port::*;
pub use project_container_aggregate::*;
pub use project_governance_protocol::*;
pub use project_orchestrator_aggregate::*;
pub use report_commands_aggregate::*;
pub use scanner_provider_port::*;
pub use semantic_flow_protocol::*;
pub use semantic_tracer_port::*;
pub use semantic_tracer_protocol::*;
pub use service_container_aggregate::*;
pub use setup_management_aggregate::*;
pub use setup_management_protocol::*;
pub use source_parser_port::*;
pub use source_system_port::*;
pub use watch_commands_aggregate::*;
pub use watch_orchestrator_aggregate::*;
pub use watch_provider_port::*;

// architecture_rule_protocol: IArchImportProtocol excluded (canonical in architecture_import_protocol)
pub use architecture_rule_protocol::{
    IAnalyzer, IArchImportProcessorProtocol, IArchRuleProtocol, IArchStructureProtocol,
    ICodeQualityProtocol, IInternalCheckerProtocol, IMetricCheckerProtocol, INamingCheckerProtocol,
    INamingRuleProtocol, IRoleCheckerProtocol,
};

// ---- Type aliases for backward compatibility ----
pub type LinterError = LinterOperationError;

// IJsTracerPort: simplified tracer port
#[async_trait::async_trait]
pub trait IJsTracerPort: Send + Sync {
    async fn show_enclosing_scope(
        &self,
        file_path: &crate::taxonomy::FilePath,
        line: crate::taxonomy::LineNumber,
    ) -> Result<Option<crate::taxonomy::ScopeRef>, crate::taxonomy::SemanticError>;
}

// Note: Type aliases that wrap `dyn Trait` cannot be used in `impl` blocks.
// Downstream code should use the inner trait directly.
// These aliases exist for ergonomic usage in type positions.
pub type DiffResultAggregateAlias = GitDiffResultAggregate;

// Pipeline I/O aggregates
pub trait PipelineInputAggregate: Send + Sync {
    fn action(&self) -> &str;
    fn job_id(&self) -> &crate::taxonomy::JobId;
    fn args(&self) -> Option<&str> {
        None
    }
    fn path(&self) -> Option<&str> {
        None
    }
}

pub trait PipelineOutputAggregate: Send + Sync {
    fn is_success(&self) -> bool;
    fn message(&self) -> &str;
    fn data(&self) -> Option<&serde_json::Value>;
    fn success(&self) -> bool {
        self.is_success()
    }
    fn job_id(&self) -> &str {
        ""
    }
    fn error(&self) -> Option<&str> {
        None
    }
}

// Container type alias for DI container
pub type Container = std::sync::Arc<dyn ServiceContainerAggregate>;
