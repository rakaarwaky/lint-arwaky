// lifecycle_state_manager — Agent lifecycle: startup, shutdown, health management.
use async_trait::async_trait;
use crate::contract::AgentLifecycleAggregate;
use crate::taxonomy::{Duration, ResponseData, StdOutput, StdError, MetadataVO, ExitCode};
use std::collections::HashMap;
use std::time::Instant;

pub struct AgentState {
    start_time: Option<Instant>,
    pub started: bool,
    pub status: crate::taxonomy::AgentStatus,
}

impl AgentState {
    pub fn new() -> Self {
        Self {
            start_time: None,
            started: false,
            status: crate::taxonomy::AgentStatus::INIT,
        }
    }
}

#[async_trait]
impl AgentLifecycleAggregate for AgentState {
    fn uptime(&self) -> Duration {
        match self.start_time {
            Some(start) => Duration::new(start.elapsed().as_secs_f64()),
            None => Duration::new(0.0),
        }
    }

    fn mark_started(&mut self) {
        self.started = true;
        self.start_time = Some(Instant::now());
        self.status = "running".to_string();
    }

    async fn get_health(&self) -> ResponseData {
        let mut metadata_map = HashMap::new();
        metadata_map.insert("lifecycle".to_string(), serde_json::json!({
            "status": self.status,
            "uptime_seconds": self.uptime().value,
            "started": self.started,
        }));
        metadata_map.insert("system".to_string(), serde_json::json!({
            "os": std::env::consts::OS,
            "cwd": std::env::current_dir().map(|p| p.to_string_lossy().to_string()).unwrap_or_default(),
        }));
        ResponseData::new(
            StdOutput::new(""),
            StdError::new(""),
            ExitCode::new(0),
            MetadataVO::new(metadata_map),
        )
    }

    fn mark_stopped(&mut self) {
        self.status = "stopped".to_string();
    }

    fn mark_degraded(&mut self) {
        self.status = "degraded".to_string();
    }
}

pub fn get_state() -> AgentState {
    AgentState::new()
}
