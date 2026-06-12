// PURPOSE: MetricsContainer — wiring for metrics-service feature (root layer, wiring only)
use std::sync::Arc;
use crate::metrics_service::contract_metrics_port::IMetricsProviderPort;

pub struct MetricsContainer {
    provider: Arc<dyn IMetricsProviderPort>,
}

impl MetricsContainer {
    pub fn new() -> Self {
        let path_norm: Arc<dyn shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort> = Arc::new(
            crate::source_parsing::infrastructure_path_provider::PathNormalizationProvider {},
        );
        Self {
            provider: Arc::new(
                crate::language_adapters::infrastructure_py_metrics_adapter::MetricsProvider::new(
                    path_norm,
                    ".lint_history.json",
                ),
            ),
        }
    }

    pub fn provider(&self) -> Arc<dyn IMetricsProviderPort> {
        self.provider.clone()
    }
}
impl Default for MetricsContainer {
    fn default() -> Self {
        Self::new()
    }
}

