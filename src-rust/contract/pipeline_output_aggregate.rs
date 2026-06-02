use crate::taxonomy::{FilePath, SuccessStatus, JobId, ErrorMessage, Suggestion};

#[derive(Debug, Clone, Default)]
pub struct PipelineOutputAggregate {
    pub root_path: Option<FilePath>,
    pub success: SuccessStatus,
    pub job_id: Option<JobId>,
    pub data: Option<serde_json::Value>,
    pub error: Option<ErrorMessage>,
    pub suggestion: Option<Suggestion>,
}
