// PURPOSE: LifecycleAggregate — aggregate trait for agent lifecycle management
use lifecycle_state::taxonomy_agent_status_vo::AgentStatusVO;
use pipeline_jobs::taxonomy_job_vo::ResponseData;
use shared_common::taxonomy_common_vo::BooleanVO;
use shared_common::taxonomy_duration_vo::Duration;
use async_trait::async_trait;

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
