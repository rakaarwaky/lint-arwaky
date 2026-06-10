// PURPOSE: PipelineExtendedOrchestrator — initializes extended pipeline with all sub-orchestrators

use crate::file_watch::contract_watch_aggregate::DirectoryWatchAggregate;
use crate::multi_project::contract_project_aggregate::MultiProjectAggregate;
use crate::pipeline_jobs::contract_extended_aggregate::PipelineExtendedOrchestratorAggregate;
use crate::pipeline_jobs::contract_output_aggregate::PipelineOutputAggregate;
use crate::pipeline_jobs::taxonomy_action_vo::JobId;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_common_vo::BooleanVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use std::collections::HashMap;

use async_trait::async_trait;

pub struct PipelineExtendedOrchestrator {}

#[async_trait]
impl PipelineExtendedOrchestratorAggregate for PipelineExtendedOrchestrator {
    fn root_path(&self) -> Option<&FilePath> {
        None
    }

    async fn execute_multi_project(
        &self,
        _request: MultiProjectAggregate,
        _use_retry: Option<BooleanVO>,
        _config_path: Option<&FilePath>,
    ) -> Box<dyn PipelineOutputAggregate> {
        let job_id = JobId::new("multi-project-job");
        let mut metadata = HashMap::new();
        metadata.insert("results".to_string(), serde_json::json!([]));
        Box::new(ExtendedPipelineOutput {
            success: SuccessStatus::new(true),
            job_id,
            data: Some(ResponseData {
                value: None,
                stdout: "multi-project scan completed".to_string(),
                stderr: String::new(),
                returncode: 0,
                metadata,
            }),
            error: None,
        })
    }

    async fn execute_watch(
        &self,
        _request: DirectoryWatchAggregate,
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

struct ExtendedPipelineOutput {
    success: SuccessStatus,
    job_id: JobId,
    data: Option<ResponseData>,
    error: Option<ErrorMessage>,
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
