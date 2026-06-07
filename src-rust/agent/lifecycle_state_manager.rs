//! Agent lifecycle management.
use crate::contract::{AgentLifecycleAggregate, ServiceContainerAggregate};
use crate::taxonomy::{AgentStatusVO, BooleanVO, Duration, ResponseData};
use std::sync::Arc;

pub struct LifecycleStateManager {
    container: Option<Arc<dyn ServiceContainerAggregate>>,
}

impl LifecycleStateManager {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub fn register_container(&mut self, container: Arc<dyn ServiceContainerAggregate>) {
        self.container = Some(container);
    }
}

#[async_trait::async_trait]
impl AgentLifecycleAggregate for LifecycleStateManager {
    fn container(&self) -> Arc<dyn ServiceContainerAggregate> {
        self.container
            .clone()
            .expect("LifecycleStateManager not initialized with container")
    }

    fn status(&self) -> AgentStatusVO {
        AgentStatusVO::new(crate::taxonomy::AgentStatus::INIT)
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
