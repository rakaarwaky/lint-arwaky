use async_trait::async_trait;
use std::sync::Arc;
use crate::taxonomy::{
    common_duration_vo::Duration, lint_status_vo::ResponseData, agent_status_vo::AgentStatusVO,
    common_collection_vo::BooleanVO,
};
use crate::contract::service_container_aggregate::ServiceContainerAggregate;

#[async_trait]
pub trait AgentLifecycleAggregate: Send + Sync {
    /// The orchestration boundary.
    fn container(&self) -> Arc<dyn ServiceContainerAggregate>;
    
    /// Current agent status.
    fn status(&self) -> AgentStatusVO;
    
    /// Whether the agent has started.
    fn started(&self) -> BooleanVO;

    /// ARCHITECTURAL COMMITMENT: Uptime tracking.
    fn uptime(&self) -> Duration;

    /// State transition: started.
    fn mark_started(&self);

    /// AGGREGATOR: Gather system health data.
    async fn get_health(&self) -> ResponseData;

    /// State transition: stopped.
    fn mark_stopped(&self);

    /// State transition: degraded.
    fn mark_degraded(&self);
}
