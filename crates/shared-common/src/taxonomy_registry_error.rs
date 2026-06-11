// PURPOSE: JobError, JobErrorKind — structured error types for job registry operations
use pipeline_jobs::taxonomy_action_vo::JobId;
use shared_common::taxonomy_common_error::Cause;
use shared_common::taxonomy_common_error::ErrorMessage;
use shared_common::taxonomy_error_vo::ErrorCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct JobError {
    pub job_id: JobId,
    pub message: ErrorMessage,
    pub error_code: ErrorCode,
    pub cause: Cause,
}

impl JobError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            job_id: JobId::default(),
            message,
            error_code: ErrorCode::default(),
            cause: Cause::default(),
        }
    }
}

impl std::fmt::Display for JobError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let job_str = self.job_id.to_string();
        let target = if job_str.is_empty() {
            String::new()
        } else {
            format!(" for job {}", job_str)
        };
        let code_str = self.error_code.to_string();
        let code = if code_str.is_empty() {
            String::new()
        } else {
            format!(" [{}]", code_str)
        };
        write!(f, "Job Error{}{}: {}", target, code, self.message)
    }
}
