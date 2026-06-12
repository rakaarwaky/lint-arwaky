// PURPOSE: JobError, JobErrorKind — structured error types for job registry operations
use crate::pipeline_jobs::taxonomy_action_vo::JobId;

define_error! {
    pub struct JobError {
        pub job_id: JobId,
    }
    display("Job Error", job_id: " for job {}")
}
