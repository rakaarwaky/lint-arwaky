use /* UNKNOWN: ErrorMessage */ crate::shared_common::taxonomy_common_error::ErrorMessage;
use /* UNKNOWN: TransportEndpoint */ crate::cli_transport::taxonomy_protocol_vo::TransportEndpoint;
use crate::cli_transport::taxonomy_protocol_vo::TransportProtocol;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct TransportError {
    pub protocol: TransportProtocol,
    pub message: ErrorMessage,
    #[serde(default)]
    pub endpoint: Option<TransportEndpoint>,
    #[serde(default)]
    pub underlying_error: Option<ErrorMessage>,
}

impl TransportError {
    pub fn new(protocol: TransportProtocol, message: ErrorMessage) -> Self {
        Self {
            protocol,
            message,
            endpoint: None,
            underlying_error: None,
        }
    }
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ep = self
            .endpoint
            .as_ref()
            .map(|e| format!(" {}", e))
            .unwrap_or_default();
        write!(f, "[{}]{} {}", self.protocol, ep, self.message)
    }
}
