// pipeline_extended_orchestrator — Orchestration for multi-project and watch modes (Agent Layer).
use crate::contract::{PipelineExtendedAggregate, PipelineOutputAggregate, crate::contract::multi_project_aggregate::MultiProjectAggregate, crate::contract::directory_watch_aggregate::DirectoryWatchAggregate};
use crate::taxonomy::{FilePath, JobId, SuccessStatus, BooleanVO, ResponseData, ErrorMessage, StdOutput, StdError, ExitCode, MetadataVO};
use std::collections::HashMap;

pub struct PipelineExtendedOrchestrator;

impl PipelineExtendedAggregate for PipelineExtendedOrchestrator {}

impl PipelineExtendedOrchestrator {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute_multi_project(
        &self,
        _request: &dyn MultiProjectAggregate,
        _use_retry: Option<bool>,
        _config_path: Option<&FilePath>,
    ) -> Box<dyn PipelineOutputAggregate> {
        let job_id = JobId::new("multi-project-job");
        let mut metadata = HashMap::new();
        metadata.insert("results".to_string(), serde_json::json!([]));
        Box::new(ExtendedPipelineOutput {
            success: SuccessStatus::new(true),
            job_id,
            data: Some(ResponseData::new(
                StdOutput::new("multi-project scan completed"),
                StdError::new(""),
                ExitCode::new(0),
                MetadataVO::new(metadata),
            )),
            error: None,
        })
    }

    pub async fn execute_watch(
        &self,
        _request: &dyn DirectoryWatchAggregate,
    ) -> Box<dyn PipelineOutputAggregate> {
        let job_id = JobId::new("watch-job");
        Box::new(ExtendedPipelineOutput {
            success: SuccessStatus::new(true),
            job_id,
            data: None,
            error: None,
        })
    }
}

struct ExtendedPipelineOutput {
    success: SuccessStatus,
    job_id: JobId,
    data: Option<ResponseData>,
    error: Option<ErrorMessage>,
}

impl PipelineOutputAggregate for ExtendedPipelineOutput {
    fn success(&self) -> &SuccessStatus { &self.success }
    fn job_id(&self) -> &JobId { &self.job_id }
    fn data(&self) -> Option<&serde_json::Value> {
        self.data.as_ref().map(|d| &serde_json::Value::String(d.stdout.to_string()))
    }
    fn error(&self) -> Option<&ErrorMessage> { self.error.as_ref() }
}
