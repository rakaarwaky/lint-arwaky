// PURPOSE: PipelineContainer — wiring for pipeline-jobs feature (root layer, wiring only)
use std::sync::Arc;
use crate::pipeline_jobs::contract_extended_aggregate::PipelineExtendedOrchestratorAggregate;
use crate::pipeline_jobs::contract_output_aggregate::PipelineOutputAggregate;
use crate::pipeline_jobs::contract_registry_port::IJobRegistryPort;

pub struct PipelineContainer {
    extended_aggregate: Arc<dyn PipelineExtendedOrchestratorAggregate>,
    output_aggregate: Arc<dyn PipelineOutputAggregate>,
}

impl PipelineContainer {
    pub fn new() -> Self {
        Self {
            extended_aggregate: Arc::new(
                crate::pipeline_jobs::agent_pipeline_extended_orchestrator::PipelineExtendedOrchestrator::new(),
            ),
            output_aggregate: Arc::new(
                crate::pipeline_jobs::agent_pipeline_extended_orchestrator::ExtendedPipelineOutput::new(
                    crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus::new(true),
                    crate::pipeline_jobs::taxonomy_action_vo::JobId::new("default"),
                    Some(crate::pipeline_jobs::taxonomy_job_vo::ResponseData {
                        value: None,
                        stdout: "default".to_string(),
                        stderr: String::new(),
                        returncode: 0,
                        metadata: std::collections::HashMap::new(),
                    }),
                    None,
                ),
            ),
        }
    }

    pub fn extended_aggregate(&self) -> Arc<dyn PipelineExtendedOrchestratorAggregate> {
        self.extended_aggregate.clone()
    }

    pub fn output_aggregate(&self) -> Arc<dyn PipelineOutputAggregate> {
        self.output_aggregate.clone()
    }

    pub fn job_registry(&self) -> Arc<dyn IJobRegistryPort> {
        use std::sync::OnceLock;
        static REGISTRY: OnceLock<Arc<dyn IJobRegistryPort>> = OnceLock::new();
        REGISTRY
            .get_or_init(|| Arc::new(crate::pipeline_jobs::infrastructure_registry_adapter::MemoryJobRegistryAdapter::new()))
            .clone()
    }
}
impl Default for PipelineContainer {
    fn default() -> Self {
        Self::new()
    }
}

