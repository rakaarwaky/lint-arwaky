// PURPOSE: Module declarations and re-exports for http-client (port, provider)
pub mod contract_http_provider_port;
pub use contract_http_provider_port::IHttpProviderPort;
pub mod infrastructure_http_client;
pub use infrastructure_http_client::SyncHttpProvider;
