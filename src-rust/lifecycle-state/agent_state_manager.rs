//! Agent lifecycle management.
use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;
use crate::lifecycle_state::contract_lifecycle_aggregate::AgentLifecycleAggregate;
use crate::lifecycle_state::taxonomy_status_vo::AgentStatusVO;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::shared_common::taxonomy_common_vo::BooleanVO;
use crate::shared_common::taxonomy_duration_vo::Duration;
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
        self.container.clone().unwrap_or_else(|| {
            struct StubBootstrapContainer {}
            impl ServiceContainerAggregate for StubBootstrapContainer {
                fn file_system(
                    &self,
                ) -> Arc<dyn crate::file_system::contract_system_port::IFileSystemPort>
                {
                    todo!("StubBootstrapContainer: file_system not available")
                }
            }
            Arc::new(StubBootstrapContainer {})
        })
    }

    fn status(&self) -> AgentStatusVO {
        AgentStatusVO::new(crate::lifecycle_state::taxonomy_status_vo::AgentStatus::INIT)
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
