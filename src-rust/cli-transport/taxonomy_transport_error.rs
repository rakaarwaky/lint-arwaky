// PURPOSE: TransportError — structured error type wrapping protocol, message, endpoint, and underlying error
use crate::cli_transport::taxonomy_protocol_vo::TransportEndpoint;
use crate::cli_transport::taxonomy_protocol_vo::TransportProtocol;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct TransportError {
    pub protocol: TransportProtocol,
    pub message: ErrorMessage,
    pub endpoint: TransportEndpoint,
    pub underlying_error: ErrorMessage,
}

impl TransportError {
    pub fn new(protocol: TransportProtocol, message: ErrorMessage) -> Self {
        Self {
            protocol,
            message,
            endpoint: TransportEndpoint::default(),
            underlying_error: ErrorMessage::default(),
        }
    }
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ep_str = self.endpoint.to_string();
        let ep = if ep_str.is_empty() {
            String::new()
        } else {
            format!(" {}", ep_str)
        };
        write!(f, "[{}]{} {}", self.protocol, ep, self.message)
    }
}
