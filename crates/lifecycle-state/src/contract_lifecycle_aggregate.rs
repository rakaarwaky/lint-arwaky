// PURPOSE: LifecycleAggregate — aggregate trait for agent lifecycle management
use crate::taxonomy_agent_status_vo::AgentStatusVO;
use async_trait::async_trait;
use shared::taxonomy_job_vo::ResponseData;
use shared::BooleanVO;
use shared::Duration;

#[async_trait]
pub trait AgentLifecycleAggregate: Send + Sync {
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
