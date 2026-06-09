// pipeline_execution_orchestrator — Agent pipeline: receive→think→act→respond orchestrator.
use crate::pipeline_jobs::contract_input_aggregate::PipelineInputAggregate;
use crate::pipeline_jobs::contract_orchestrator_aggregate::PipelineExecutionOrchestratorAggregate;
use crate::pipeline_jobs::contract_output_aggregate::PipelineOutputAggregate;
use crate::pipeline_jobs::contract_registry_port::IJobRegistryPort;
use crate::pipeline_jobs::infrastructure_registry_adapter::MemoryJobRegistryAdapter;
use crate::pipeline_jobs::taxonomy_action_vo::JobId;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use std::sync::OnceLock;

static REGISTRY: OnceLock<MemoryJobRegistryAdapter> = OnceLock::new();

pub struct PipelineExecutionOrchestrator {}

impl PipelineExecutionOrchestratorAggregate for PipelineExecutionOrchestrator {
    fn job_registry(&self) -> &dyn IJobRegistryPort {
        REGISTRY.get_or_init(MemoryJobRegistryAdapter::new)
    }
}

impl PipelineExecutionOrchestrator {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn execute(
        &self,
        request: &dyn PipelineInputAggregate,
    ) -> Box<dyn PipelineOutputAggregate> {
        // Full pipeline execution: receive → think → act → respond
        let action = request.action().to_string();

        // 1. Receive — create job
        let job_id = JobId::new("pipeline-job");

        // 2. Think — validate and decide
        if let Some(error_response) = self.stage_validate(&action, &job_id).await {
            return error_response;
        }

        // 3. Act — execute
        // 4. Respond — format and complete
        self.format_success_response(job_id, serde_json::Value::Null)
    }

    async fn stage_validate(
        &self,
        action: &str,
        _job_id: &JobId,
    ) -> Option<Box<dyn PipelineOutputAggregate>> {
        if !self.validate_action(action) {
            return Some(Box::new(PipelineOutputImpl {
                success: SuccessStatus::new(false),
                job_id: JobId::new("error"),
                data: None,
                error: Some(ErrorMessage::new(format!("Invalid action '{}'", action))),
            }));
        }
        None
    }

    fn format_success_response(
        &self,
        job_id: JobId,
        raw: serde_json::Value,
    ) -> Box<dyn PipelineOutputAggregate> {
        let mut resp = ResponseData::new();
        resp.value = Some(raw);
        Box::new(PipelineOutputImpl {
            success: SuccessStatus::new(true),
            job_id,
            data: Some(resp),
            error: None,
        })
    }

    fn validate_action(&self, action: &str) -> bool {
        let known_actions = [
            "check",
            "scan",
            "security",
            "complexity",
            "duplicates",
            "trends",
            "fix",
            "report",
            "version",
            "adapters",
            "install-hook",
            "install_hook",
            "uninstall-hook",
            "uninstall_hook",
            "batch",
            "multi_project",
            "doctor",
            "cancel",
        ];
        known_actions.contains(&action)
    }
}

struct PipelineOutputImpl {
    success: SuccessStatus,
    job_id: JobId,
    data: Option<ResponseData>,
    error: Option<ErrorMessage>,
}

impl PipelineOutputAggregate for PipelineOutputImpl {
    fn success(&self) -> &SuccessStatus {
        &self.success
    }
    fn job_id(&self) -> &JobId {
        &self.job_id
    }
    fn data(&self) -> Option<&ResponseData> {
        self.data.as_ref()
    }
    fn error(&self) -> Option<&ErrorMessage> {
        self.error.as_ref()
    }
}
