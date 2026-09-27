// project-setup — contract and taxonomy types
pub mod contract_setup_aggregate;
pub mod contract_setup_protocol;
pub mod taxonomy_setup_request;
pub mod taxonomy_setup_response;
pub mod taxonomy_setup_vo;
pub mod taxonomy_skills_constant;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_setup_aggregate::ISetupAggregate;
pub use contract_setup_protocol::ISetupInstallerProtocol;
pub use contract_setup_protocol::ISetupManagementProtocol;
pub use taxonomy_setup_request::SetupRequest;
pub use taxonomy_setup_response::SetupResponse;

// ── Taxonomy types ──
pub use contract_setup_aggregate::SetupMgmtProtocol;
pub use taxonomy_setup_vo::CreateConfigDirResult;
pub use taxonomy_setup_vo::EmbeddedSkillVO;
pub use taxonomy_setup_vo::McpBinaryNameVO;
pub use taxonomy_setup_vo::PackageManagerStatus;
pub use taxonomy_setup_vo::PreFlightResult;
pub use taxonomy_setup_vo::ProjectLanguageVO;
pub use taxonomy_setup_vo::ProjectLanguagesVO;
pub use taxonomy_setup_vo::SetupError;
pub use taxonomy_setup_vo::WriteConfigResult;
pub use taxonomy_skills_constant::EMBEDDED_SKILLS;
