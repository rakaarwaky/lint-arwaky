// PURPOSE: Module declarations and re-exports for lifecycle-state (lifecycle, aggregate, VOs)
pub mod agent_status_lifecycle;
pub use agent_status_lifecycle::{get_lifecycle_state_manager, LifecycleStateManager};
pub mod contract_lifecycle_aggregate;
pub use contract_lifecycle_aggregate::AgentLifecycleAggregate;
pub mod taxonomy_agent_status_vo;
pub use taxonomy_agent_status_vo::{AgentStatus, AgentStatusVO};
pub mod root_lifecycle_container;
