use crate::taxonomy::{ErrorMessage, JobId, SuccessStatus};

pub trait PipelineOutputAggregate: Send + Sync {
    fn success(&self) -> &SuccessStatus;
    fn job_id(&self) -> &JobId;
    fn data(&self) -> Option<&serde_json::Value>;
    fn error(&self) -> Option<&ErrorMessage>;
}
