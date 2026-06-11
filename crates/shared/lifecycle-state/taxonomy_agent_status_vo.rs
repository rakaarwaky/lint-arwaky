// PURPOSE: AgentStatus — value object for agent lifecycle state representation
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentStatus {
    #[serde(rename = "init")]
    INIT,
    #[serde(rename = "started")]
    STARTED,
    #[serde(rename = "stopped")]
    STOPPED,
    #[serde(rename = "degraded")]
    DEGRADED,
}

impl std::fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentStatus::INIT => write!(f, "init"),
            AgentStatus::STARTED => write!(f, "started"),
            AgentStatus::STOPPED => write!(f, "stopped"),
            AgentStatus::DEGRADED => write!(f, "degraded"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentStatusVO {
    pub(crate) value: AgentStatus,
}

impl AgentStatusVO {
    pub fn new(value: AgentStatus) -> Self {
        Self { value }
    }
    pub fn value(&self) -> &AgentStatus {
        &self.value
    }
}
