pub mod contract_setup_aggregate;
pub mod contract_setup_protocol;
pub mod contract_tool_executor_protocol;
pub mod taxonomy_language_vo;
pub mod taxonomy_setup_contract_vo;
pub mod utility_filesystem_checker;
pub mod utility_setup_io;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_setup_aggregate::SetupManagementAggregate;
pub use contract_setup_protocol::ISetupManagementProtocol;
pub use contract_setup_protocol::ISetupInstallerProtocol;
pub use contract_tool_executor_protocol::IToolExecutorProtocol;

// ── Taxonomy types ──
pub use contract_tool_executor_protocol::ToolOutput;
pub use contract_setup_protocol::InstallPackagesResult;
pub use contract_setup_aggregate::SetupMgmtProtocol;
pub use taxonomy_language_vo::ProjectLanguage;
pub use taxonomy_language_vo::LanguageSource;
pub use taxonomy_setup_contract_vo::SetupError;
pub use taxonomy_setup_contract_vo::McpBinaryNameVO;
pub use taxonomy_setup_contract_vo::ProjectLanguageVO;
pub use taxonomy_setup_contract_vo::ProjectLanguagesVO;
pub use taxonomy_setup_contract_vo::WriteConfigResult;
pub use taxonomy_setup_contract_vo::CreateConfigDirResult;
