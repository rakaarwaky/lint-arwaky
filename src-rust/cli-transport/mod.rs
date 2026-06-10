// PURPOSE: Module declarations and re-exports for CLI transport layer
pub mod infrastructure_transport_client;
pub use infrastructure_transport_client::StdioClient;
pub mod taxonomy_transport_error;
pub use taxonomy_transport_error::TransportError;
pub mod taxonomy_protocol_vo;
pub use taxonomy_protocol_vo::{TransportEndpoint, TransportProtocol, TransportUrlVO};
