// PURPOSE: Aggregate: Lifecycle aggregation/wiring
use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;
use crate::lifecycle_state::taxonomy_agent_status_vo::AgentStatusVO;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::shared_common::taxonomy_common_vo::BooleanVO;
use crate::shared_common::taxonomy_duration_vo::Duration;
use async_trait::async_trait;
use std::sync::Arc;

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
