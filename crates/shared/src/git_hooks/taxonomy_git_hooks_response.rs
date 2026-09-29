// PURPOSE: GitHooksResponse — response payload for the git_hooks aggregate

use crate::common::taxonomy_job_vo::SuccessStatus;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_lint_result_vo::LintResultList;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::git_hooks::taxonomy_git_hooks_error::GitHookError;
use crate::git_hooks::taxonomy_git_hooks_vo::GitDiffDataVO;

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
    InitializeConfig {
        description: DescriptionVO,
    },
    UpdateIgnoreRule {
        description: DescriptionVO,
    },
    DiffData {
        data: GitDiffDataVO,
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

    pub fn into_description(self) -> DescriptionVO {
        match self {
            Self::InitializeConfig { description } | Self::UpdateIgnoreRule { description } => {
                description
            }
            _ => DescriptionVO::new(String::new()),
        }
    }

    pub fn into_diff_data(self) -> GitDiffDataVO {
        match self {
            Self::DiffData { data } => data,
            _ => GitDiffDataVO::default(),
        }
    }

    pub fn into_identity(self) -> Identity {
        match self {
            Self::GetManagerIdentity { identity } => identity,
            _ => Identity::default(),
        }
    }
}
