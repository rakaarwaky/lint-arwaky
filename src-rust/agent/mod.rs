//! # Agent Layer — Composition & Orchestration
//!
//! This module is the **composition layer** of the AES architecture. It wires together
//! capabilities (business logic) and contracts (interfaces) into concrete orchestrators
//! and manages the dependency injection container. Agent orchestrators are the top-level
//! entry points that coordinate multi-step workflows across layers.
//!
//! ## Layer Rules (AES Compliance)
//! - **Allowed Imports**: `src/taxonomy/`, `src/contract/`, `src/capabilities/`,
//!   `src/infrastructure/`, and `src/agent/` only.
//! - **Allowed Suffixes**: `_orchestrator`, `_container`, `_registry`, `_manager`
//! - **Responsibility**: Orchestration only — no business logic, no I/O, no interfaces.
//!
//! ## Module Index
//!
//! | Domain / Feature | Key Types | Description |
//! |------------------|-----------|-------------|
//! | **Lint Pipeline** | `ArchitectureLintOrchestrator`, `ArchLintPipelineOrchestrator` | Lint pipeline execution orchestration |
//! | **Compliance** | `ArchComplianceCoordinator`, `ArchitectureOrchestrator` | AES compliance check orchestration |
//! | **Execution** | `PipelineExecutionOrchestrator`, `PipelineExtendedOrchestrator`, `PipelineActionOrchestrator` | Multi-step pipeline dispatch |
//! | **Commands** | `DevCommandsOrchestrator`, `GitCommandsOrchestrator`, `PluginCommandsOrchestrator`, `ReportCommandsOrchestrator` | Surface command delegation |
//! | **Analysis** | `AnalysisOrchestrator` | Analysis execution coordination |
//! | **Fix** | `LintFixOrchestrator` | Auto-fix workflow coordination |
//! | **Watch** | `WatchCommandsOrchestrator`, `WatchExecutionOrchestrator` | File system watch orchestration |
//! | **Setup** | `SetupManagementOrchestrator` | Environment & config setup |
//! | **Maintenance** | `MaintenanceCommandsOrchestrator` | System maintenance operations |
//! | **Multi-Project** | `MultiProjectOrchestrator` | Cross-project orchestration |
//! | **DI Container** | `DependencyInjectionContainer`, `Container`, `ProjectContainerRegistry` | Dependency injection & registries |
//! | **Lifecycle** | `LifecycleStateManager` | Application state management |
//! | **Job Registry** | `PipelineJobRegistry` | Background job tracking |
//! | **Output** | `OutputClientOrchestrator` | Output formatting & delivery |
//! | **Hook** | `HookManagementOrchestrator` | Git hook lifecycle management |

pub mod analysis_execution_orchestrator;
pub mod architecture_compliance_orchestrator;
pub mod architecture_lint_orchestrator;
pub mod lint_checking_coordinator;
pub mod dependency_injection_container;
pub mod dev_commands_orchestrator;
pub mod git_commands_orchestrator;
pub mod hook_management_orchestrator;
pub mod lifecycle_state_manager;
pub mod lint_fix_orchestrator;
pub mod maintenance_commands_orchestrator;
pub mod multi_project_orchestrator;
pub mod output_client_orchestrator;
pub mod pipeline_action_orchestrator;
pub mod pipeline_execution_orchestrator;
pub mod pipeline_extended_orchestrator;
pub mod pipeline_job_registry;
pub mod plugin_commands_orchestrator;
pub mod project_container_registry;
pub mod report_commands_orchestrator;
pub mod setup_management_orchestrator;

// ═══════════════════════════════════════════════════════════════════════════════
// PUBLIC RE-EXPORTS (Flat Access via Barrel)
// ═══════════════════════════════════════════════════════════════════════════════

pub use analysis_execution_orchestrator::AnalysisOrchestrator;
pub use architecture_compliance_orchestrator::{
    ArchComplianceCoordinator, ArchitectureOrchestrator, InfrastructureMixinContainer,
    OrchestratorMixinContainer, WatchCommandsOrchestrator, WatchExecutionOrchestrator,
};
pub use architecture_lint_orchestrator::{
    ArchLintPipelineOrchestrator, ArchitectureLintOrchestrator,
};
pub use dependency_injection_container::{Container, DependencyInjectionContainer};
pub use dev_commands_orchestrator::DevCommandsOrchestrator;
pub use git_commands_orchestrator::GitCommandsOrchestrator;
pub use hook_management_orchestrator::HookManagementOrchestrator;
pub use lifecycle_state_manager::{get_lifecycle_state_manager, LifecycleStateManager};
pub use lint_fix_orchestrator::LintFixOrchestrator;
pub use maintenance_commands_orchestrator::MaintenanceCommandsOrchestrator;
pub use multi_project_orchestrator::MultiProjectOrchestrator;
pub use output_client_orchestrator::OutputClientOrchestrator;
pub use pipeline_action_orchestrator::PipelineActionOrchestrator;
pub use pipeline_execution_orchestrator::PipelineExecutionOrchestrator;
pub use pipeline_extended_orchestrator::PipelineExtendedOrchestrator;
pub use pipeline_job_registry::PipelineJobRegistry;
pub use plugin_commands_orchestrator::PluginCommandsOrchestrator;
pub use project_container_registry::ProjectContainerRegistry;
pub use report_commands_orchestrator::ReportCommandsOrchestrator;
pub use setup_management_orchestrator::SetupManagementOrchestrator;
