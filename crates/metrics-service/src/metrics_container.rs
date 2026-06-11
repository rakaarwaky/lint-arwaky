// PURPOSE: MetricsContainer — wiring for metrics-service feature (root layer, wiring only)
use std::sync::Arc;
use crate::contract_metrics_port::IMetricsProviderPort;

pub struct MetricsContainer {
    provider: Arc<dyn IMetricsProviderPort>,
}

impl MetricsContainer {
    pub fn new(provider: Arc<dyn IMetricsProviderPort>) -> Self {
        Self { provider }
    }

    pub fn provider(&self) -> Arc<dyn IMetricsProviderPort> {
        self.provider.clone()
    }
}
impl Default for MetricsContainer {
    fn default() -> Self {
        panic!("MetricsContainer::default() requires a provider; use MetricsContainer::new(provider) instead")
    }
}
