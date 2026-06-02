use serde::{Serialize, Deserialize};
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobError {
    #[serde(default)]
    pub job_id: Option<JobId>,
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: Option<ErrorCode>,
    #[serde(default)]
    pub cause: Option<Cause>,
}

impl JobError {
    pub fn new(message: ErrorMessage) -> Self {
        Self { job_id: None, message, error_code: None, cause: None }
    }
}

impl std::fmt::Display for JobError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let target = self.job_id.as_ref().map(|j| format!(" for job {}", j)).unwrap_or_default();
        let code = self.error_code.as_ref().map(|c| format!(" [{}]", c)).unwrap_or_default();
        write!(f, "Job Error{}{}: {}", target, code, self.message)
    }
}
