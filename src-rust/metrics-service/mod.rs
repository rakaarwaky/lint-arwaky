// PURPOSE: Module declarations and re-exports for metrics-service (port, error)
pub mod contract_metrics_port;
pub use contract_metrics_port::IMetricsProviderPort;
pub mod taxonomy_metrics_error;
pub use taxonomy_metrics_error::MetricsError;
