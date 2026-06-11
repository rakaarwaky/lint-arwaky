// PURPOSE: OutputAggregate — aggregate trait for pipeline output generation (report, format)
use crate::pipeline_jobs::taxonomy_action_vo::JobId;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use crate::common::taxonomy_common_error::ErrorMessage;

pub trait PipelineOutputAggregate: Send + Sync {
    fn success(&self) -> &SuccessStatus;
    fn job_id(&self) -> &JobId;
    fn data(&self) -> Option<&ResponseData>;
    fn error(&self) -> Option<&ErrorMessage>;
}
