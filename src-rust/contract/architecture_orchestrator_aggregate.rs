use async_trait::async_trait;
use super::*;

#[async_trait]
pub trait ArchitectureOrchestratorAggregate: Send + Sync {
    async fn resolve_effective_layer_map(&self, config: ArchitectureConfig) -> LayerMapVO;
}
