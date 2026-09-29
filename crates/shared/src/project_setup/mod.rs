// project-setup — contract and taxonomy types
pub mod contract_setup_aggregate;
pub mod contract_setup_protocol;
pub mod taxonomy_project_setup_constant;
pub mod taxonomy_project_setup_request;
pub mod taxonomy_project_setup_response;
pub mod taxonomy_project_setup_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_setup_aggregate::ISetupAggregate;
pub use contract_setup_protocol::IAdapterInstallationProtocol;
pub use contract_setup_protocol::IConfigTemplateProtocol;
pub use contract_setup_protocol::IConfigWritingProtocol;
pub use contract_setup_protocol::IEnvGenerationProtocol;
pub use contract_setup_protocol::IFilePathExistenceProtocol;
pub use contract_setup_protocol::ILanguageDetectionProtocol;
pub use contract_setup_protocol::IMcpConfigGenerationProtocol;
pub use contract_setup_protocol::IPreFlightProtocol;
pub use taxonomy_project_setup_request::SetupRequest;
pub use taxonomy_project_setup_response::SetupResponse;

// ── Taxonomy types ──
pub use contract_setup_aggregate::SetupMgmtProtocol;
pub use taxonomy_project_setup_constant::EMBEDDED_SKILLS;
pub use taxonomy_project_setup_vo::CreateConfigDirResult;
pub use taxonomy_project_setup_vo::EmbeddedSkillVO;
pub use taxonomy_project_setup_vo::McpBinaryNameVO;
pub use taxonomy_project_setup_vo::PackageManagerStatus;
pub use taxonomy_project_setup_vo::PreFlightResult;
pub use taxonomy_project_setup_vo::ProjectLanguageVO;
pub use taxonomy_project_setup_vo::ProjectLanguagesVO;
pub use taxonomy_project_setup_vo::SetupError;
pub use taxonomy_project_setup_vo::WriteConfigResult;
