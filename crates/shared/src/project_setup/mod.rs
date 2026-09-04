pub mod contract_setup_aggregate;
pub mod contract_setup_protocol;
pub mod taxonomy_setup_contract_vo;
pub mod taxonomy_skills_constant;
pub mod taxonomy_skills_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_setup_aggregate::SetupManagementAggregate;
pub use contract_setup_protocol::ISetupInstallerProtocol;
pub use contract_setup_protocol::ISetupManagementProtocol;
// ── Taxonomy types ──
pub use contract_setup_aggregate::SetupMgmtProtocol;
pub use contract_setup_protocol::InstallPackagesResult;
pub use taxonomy_setup_contract_vo::CreateConfigDirResult;
pub use taxonomy_setup_contract_vo::McpBinaryNameVO;
pub use taxonomy_setup_contract_vo::PackageManagerStatus;
pub use taxonomy_setup_contract_vo::PreFlightResult;
pub use taxonomy_setup_contract_vo::ProjectLanguageVO;
pub use taxonomy_setup_contract_vo::ProjectLanguagesVO;
pub use taxonomy_setup_contract_vo::SetupError;
pub use taxonomy_setup_contract_vo::WriteConfigResult;
pub use taxonomy_skills_constant::EMBEDDED_SKILLS;
pub use taxonomy_skills_vo::EmbeddedSkillVO;
