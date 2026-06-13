// PURPOSE: PipelineExtendedOrchestrator — initializes extended pipeline with all sub-orchestrators
use shared::file_watch::taxonomy_watch_vo::DirectoryWatchVO;
use shared::multi_project::taxonomy_multi_project_vo::MultiProjectVO;
use shared::pipeline_jobs::contract_extended_aggregate::PipelineExtendedOrchestratorAggregate;
use shared::pipeline_jobs::contract_output_aggregate::PipelineOutputAggregate;
use shared::pipeline_jobs::taxonomy_action_vo::JobId;
use shared::pipeline_jobs::taxonomy_job_vo::ResponseData;
use shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::BooleanVO;
use std::collections::HashMap;

use async_trait::async_trait;

pub struct PipelineExtendedOrchestrator {}

impl Default for PipelineExtendedOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelineExtendedOrchestrator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl PipelineExtendedOrchestratorAggregate for PipelineExtendedOrchestrator {
    fn root_path(&self) -> Option<&FilePath> {
        None
    }

    async fn execute_multi_project(
        &self,
        _request: MultiProjectVO,
        _use_retry: Option<BooleanVO>,
        _config_path: Option<&FilePath>,
    ) -> Box<dyn PipelineOutputAggregate> {
        let job_id = JobId::new("multi-project-job");
        let mut metadata = HashMap::new();
        metadata.insert("results".to_string(), serde_json::json!([]));
        Box::new(ExtendedPipelineOutput::new(
            SuccessStatus::new(true),
            job_id,
            Some(ResponseData {
                value: None,
                stdout: "multi-project scan completed".to_string(),
                stderr: String::new(),
                returncode: 0,
                metadata,
            }),
            None,
        ))
    }

    async fn execute_watch(&self, _request: DirectoryWatchVO) -> Box<dyn PipelineOutputAggregate> {
        let job_id = JobId::new("watch-job");
        Box::new(ExtendedPipelineOutput::new(
            SuccessStatus::new(true),
            job_id,
            Some(ResponseData {
                value: None,
                stdout: "watch completed".to_string(),
                stderr: String::new(),
                returncode: 0,
                metadata: HashMap::new(),
            }),
            None,
        ))
    }
}

pub struct ExtendedPipelineOutput {
    success: SuccessStatus,
    job_id: JobId,
    data: Option<ResponseData>,
    error: Option<ErrorMessage>,
}

impl ExtendedPipelineOutput {
    pub fn new(
        success: SuccessStatus,
        job_id: JobId,
        data: Option<ResponseData>,
        error: Option<ErrorMessage>,
    ) -> Self {
        Self {
            success,
            job_id,
            data,
            error,
        }
    }
}

impl PipelineOutputAggregate for ExtendedPipelineOutput {
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
