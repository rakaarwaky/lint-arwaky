// lifecycle_state_manager — Agent lifecycle: startup, shutdown, health management.
use async_trait::async_trait;
use crate::contract::agent_lifecycle_aggregate::AgentLifecycleAggregate;
use crate::contract::service_container_aggregate::ServiceContainerAggregate;
use crate::taxonomy::{Duration, ResponseData, StdOutput, StdError, MetadataVO, ExitCode, AgentStatusVO, BooleanVO};
use std::collections::HashMap;
use std::time::Instant;
use std::sync::Arc;

struct AgentStateInner {
    start_time: Option<Instant>,
    started: bool,
    status: crate::taxonomy::AgentStatus,
}

pub struct AgentState {
    inner: std::sync::Mutex<AgentStateInner>,
}

struct DummyContainer;
impl ServiceContainerAggregate for DummyContainer {}

impl AgentState {
    pub fn new() -> Self {
        Self {
            inner: std::sync::Mutex::new(AgentStateInner {
                start_time: None,
                started: false,
                status: crate::taxonomy::AgentStatus::INIT,
            }),
        }
    }
}

#[async_trait]
impl AgentLifecycleAggregate for AgentState {
    fn container(&self) -> Arc<dyn ServiceContainerAggregate> {
        Arc::new(DummyContainer)
    }

    fn status(&self) -> AgentStatusVO {
        let inner = self.inner.lock().unwrap();
        AgentStatusVO::new(inner.status.clone())
    }

    fn started(&self) -> BooleanVO {
        let inner = self.inner.lock().unwrap();
        BooleanVO::new(inner.started)
    }

    fn uptime(&self) -> Duration {
        let inner = self.inner.lock().unwrap();
        match inner.start_time {
            Some(start) => Duration::new(start.elapsed().as_secs_f64()),
            None => Duration::new(0.0),
        }
    }

    fn mark_started(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.started = true;
        inner.start_time = Some(Instant::now());
        inner.status = crate::taxonomy::AgentStatus::STARTED;
    }

    async fn get_health(&self) -> ResponseData {
        let (status, uptime, started) = {
            let inner = self.inner.lock().unwrap();
            (inner.status.clone(), match inner.start_time {
                Some(start) => start.elapsed().as_secs_f64(),
                None => 0.0,
            }, inner.started)
        };
        let mut metadata_map = HashMap::new();
        metadata_map.insert("lifecycle".to_string(), serde_json::json!({
            "status": status,
            "uptime_seconds": uptime,
            "started": started,
        }));
        metadata_map.insert("system".to_string(), serde_json::json!({
            "os": std::env::consts::OS,
            "cwd": std::env::current_dir().map(|p| p.to_string_lossy().to_string()).unwrap_or_default(),
        }));
        ResponseData {
            value: None,
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata: metadata_map,
        }
    }

    fn mark_stopped(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.status = crate::taxonomy::AgentStatus::STOPPED;
    }

    fn mark_degraded(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.status = crate::taxonomy::AgentStatus::DEGRADED;
    }
}

pub fn get_state() -> AgentState {
    AgentState::new()
}
