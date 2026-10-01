// PURPOSE: GitHooksResponse — response payload for the git_hooks aggregate

use crate::taxonomy_git_hooks_error::GitHookError;
use shared_common::taxonomy_job_vo::SuccessStatus;
use shared_common::taxonomy_layer_vo::Identity;
use shared_common::taxonomy_lint_result_vo::LintResultList;
use shared_common::taxonomy_message_vo::LintMessage;

pub enum GitHooksResponse {
    RunCheck {
        results: LintResultList,
    },
    Install {
        status: Result<SuccessStatus, GitHookError>,
    },
    Uninstall {
        status: Result<SuccessStatus, GitHookError>,
    },
    GetManagerIdentity {
        identity: Identity,
    },
}

impl GitHooksResponse {
    pub fn into_results(self) -> LintResultList {
        match self {
            Self::RunCheck { results } => results,
            _ => LintResultList::default(),
        }
    }

    pub fn into_status(self) -> Result<SuccessStatus, GitHookError> {
        match self {
            Self::Install { status } | Self::Uninstall { status } => status,
            _ => Err(GitHookError::new(LintMessage::new(
                "expected install or uninstall response",
            ))),
        }
    }

    pub fn into_identity(self) -> Identity {
        match self {
            Self::GetManagerIdentity { identity } => identity,
            _ => Identity::default(),
        }
    }
}
