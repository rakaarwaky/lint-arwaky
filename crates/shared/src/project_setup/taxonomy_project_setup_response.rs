// PURPOSE: SetupResponse — response payload for the setup aggregate

use crate::taxonomy_project_setup_vo::EmbeddedSkillVO;
use crate::taxonomy_project_setup_vo::{
    CreateConfigDirResult, PreFlightResult, ProjectLanguageVO, ProjectLanguagesVO, SetupError,
    WriteConfigResult,
};
use shared_common::taxonomy_job_vo::{EnvContentVO, McpConfigVO, SuccessStatus};

pub enum SetupResponse {
    CheckHttp { status: SuccessStatus },
    Env { content: EnvContentVO },
    Mcp { config: McpConfigVO },
    Installed { status: SuccessStatus },
    Language { detected: Option<ProjectLanguageVO> },
    Languages { detected: ProjectLanguagesVO },
    Template { content: Result<String, SetupError> },
    PreFlight { result: PreFlightResult },
    Skills { skills: Vec<EmbeddedSkillVO> },
    ConfigWritten { result: WriteConfigResult },
    ConfigDir { result: CreateConfigDirResult },
    Exists { exists: bool },
}

impl SetupResponse {
    pub fn into_status(self) -> SuccessStatus {
        match self {
            Self::CheckHttp { status } | Self::Installed { status } => status,
            _ => SuccessStatus::default(),
        }
    }

    pub fn into_env(self) -> EnvContentVO {
        match self {
            Self::Env { content } => content,
            _ => EnvContentVO::default(),
        }
    }

    pub fn into_mcp_config(self) -> McpConfigVO {
        match self {
            Self::Mcp { config } => config,
            _ => McpConfigVO::default(),
        }
    }

    pub fn into_language(self) -> Option<ProjectLanguageVO> {
        match self {
            Self::Language { detected } => detected,
            _ => None,
        }
    }

    pub fn into_languages(self) -> ProjectLanguagesVO {
        match self {
            Self::Languages { detected } => detected,
            _ => ProjectLanguagesVO::default(),
        }
    }

    pub fn into_template(self) -> Result<String, SetupError> {
        match self {
            Self::Template { content } => content,
            _ => Err(SetupError::other("no template available")),
        }
    }

    pub fn into_pre_flight(self) -> PreFlightResult {
        match self {
            Self::PreFlight { result } => result,
            _ => Vec::new(),
        }
    }

    pub fn into_skills(self) -> Vec<EmbeddedSkillVO> {
        match self {
            Self::Skills { skills } => skills,
            _ => Vec::new(),
        }
    }

    pub fn into_write_result(self) -> WriteConfigResult {
        match self {
            Self::ConfigWritten { result } => result,
            _ => Err(SetupError::other("no config written")),
        }
    }

    pub fn into_dir_result(self) -> CreateConfigDirResult {
        match self {
            Self::ConfigDir { result } => result,
            _ => Err(SetupError::other("no config dir created")),
        }
    }

    pub fn into_exists(self) -> bool {
        match self {
            Self::Exists { exists } => exists,
            _ => false,
        }
    }
}
