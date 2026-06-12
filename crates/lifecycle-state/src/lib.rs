// PURPOSE: Module declarations for lifecycle-state (lifecycle, container)
pub use shared::lifecycle_state::contract_lifecycle_aggregate::AgentLifecycleAggregate;
pub use shared::lifecycle_state::taxonomy_agent_status_vo::{AgentStatus, AgentStatusVO};
pub mod agent_status_lifecycle;
pub use agent_status_lifecycle::{get_lifecycle_state_manager, LifecycleStateManager};
pub mod root_lifecycle_state_container;
