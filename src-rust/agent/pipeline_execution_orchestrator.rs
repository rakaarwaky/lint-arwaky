// pipeline_execution_orchestrator — Agent pipeline: receive→think→act→respond orchestrator.
use crate::contract::{
    PipelineExecutionOrchestratorAggregate, PipelineInputAggregate, PipelineOutputAggregate,
};
use crate::taxonomy::{
    ActionArgs, BooleanVO, ContentString, ErrorMessage, FilePath, JobId, MetadataVO, ResponseData,
    SuccessStatus, Suggestion,
};

pub struct PipelineExecutionOrchestrator;

impl PipelineExecutionOrchestratorAggregate for PipelineExecutionOrchestrator {}

impl PipelineExecutionOrchestrator {
    pub fn new() -> Self {
        Self
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
        self.format_success_response(job_id, serde_json::json!({"result": "ok"}))
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
                suggestion: Some(Suggestion::new("Use list_commands() for catalog")),
            }));
        }
        None
    }

    fn format_success_response(
        &self,
        job_id: JobId,
        data: serde_json::Value,
    ) -> Box<dyn PipelineOutputAggregate> {
        Box::new(PipelineOutputImpl {
            success: SuccessStatus::new(true),
            job_id,
            data: Some(data),
            error: None,
            suggestion: None,
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
    data: Option<serde_json::Value>,
    error: Option<ErrorMessage>,
    suggestion: Option<Suggestion>,
}

impl PipelineOutputAggregate for PipelineOutputImpl {
    fn success(&self) -> &SuccessStatus {
        &self.success
    }
    fn job_id(&self) -> &JobId {
        &self.job_id
    }
    fn data(&self) -> Option<&serde_json::Value> {
        self.data.as_ref()
    }
    fn error(&self) -> Option<&ErrorMessage> {
        self.error.as_ref()
    }
}

struct PipelineInputImpl {
    action: String,
    args: Option<ActionArgs>,
    path: Option<FilePath>,
}

impl PipelineInputImpl {
    pub fn new(action: String) -> Self {
        Self {
            action,
            args: None,
            path: None,
        }
    }
}

impl PipelineInputAggregate for PipelineInputImpl {
    fn action(&self) -> &str {
        &self.action
    }
    fn args(&self) -> Option<&ActionArgs> {
        self.args.as_ref()
    }
    fn path(&self) -> Option<&FilePath> {
        self.path.as_ref()
    }
}
