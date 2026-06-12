// PURPOSE: MetricsContainer — wiring for metrics-service feature (root layer, wiring only)
use crate::RustMetricsProvider;
use shared::metrics_service::contract_metrics_port::IMetricsProviderPort;
use std::sync::Arc;

pub struct MetricsContainer {
    provider: Arc<dyn IMetricsProviderPort>,
}

impl MetricsContainer {
    pub fn new(
        path_norm: Arc<
            dyn shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort,
        >,
    ) -> Self {
        Self {
            provider: Arc::new(RustMetricsProvider::new(path_norm, ".lint_history.json")),
        }
    }

    pub fn provider(&self) -> Arc<dyn IMetricsProviderPort> {
        self.provider.clone()
    }
}
