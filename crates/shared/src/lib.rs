// PURPOSE: shared — taxonomy types, contract traits, and shared definitions
// Minimal: only what filesystem crate needs.

// ── Common modules (directly included, avoiding full common/mod.rs) ──
pub mod common {
    pub mod taxonomy_config_language_vo;
    pub mod taxonomy_message_vo;
    pub mod taxonomy_operation_error;
    pub mod taxonomy_path_vo;
    pub mod taxonomy_source_vo;
    pub mod utility_command_runner;
}

// ── Re-exports ──
pub use common::taxonomy_config_language_vo::ConfigLanguage;
pub use common::taxonomy_message_vo::ComplianceStatus;
pub use common::taxonomy_operation_error::LinterOperationError;
pub use common::taxonomy_path_vo::{DirectoryPath, FilePath};
pub use common::taxonomy_source_vo::ContentString;
pub use common::utility_command_runner;

// Feature-specific modules
pub mod filesystem;
