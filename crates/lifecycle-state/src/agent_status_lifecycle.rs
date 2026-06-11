// PURPOSE: AgentStatusLifecycle — manages agent lifecycle states (starting, ready, scanning, stopping)

use crate::contract_lifecycle_aggregate::AgentLifecycleAggregate;
use crate::taxonomy_agent_status_vo::{AgentStatus, AgentStatusVO};
use shared::taxonomy_job_vo::ResponseData;
use shared::BooleanVO;
use shared::Duration;

pub struct LifecycleStateManager {}

impl Default for LifecycleStateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LifecycleStateManager {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl AgentLifecycleAggregate for LifecycleStateManager {
    fn status(&self) -> AgentStatusVO {
        AgentStatusVO::new(AgentStatus::INIT)
    }

    fn started(&self) -> BooleanVO {
        BooleanVO::new(false)
    }

    fn uptime(&self) -> Duration {
        Duration::new(0.0)
    }

    fn mark_started(&self) {
        // no-op: state tracked externally
    }

    async fn get_health(&self) -> ResponseData {
        ResponseData {
            value: None,
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata: Default::default(),
        }
    }

    fn mark_stopped(&self) {
        // no-op: state tracked externally
    }

    fn mark_degraded(&self) {
        // no-op: state tracked externally
    }
}

pub fn get_lifecycle_state_manager() -> LifecycleStateManager {
    LifecycleStateManager::new()
}
