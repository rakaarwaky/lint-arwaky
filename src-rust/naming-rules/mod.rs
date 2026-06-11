// PURPOSE: Module declarations and re-exports for naming-rules (checkers, VOs)
pub mod taxonomy_suffix_vo;
pub use taxonomy_suffix_vo::{SuffixPolicyVO, SuffixVO};
pub mod capabilities_naming_checker;
pub use capabilities_naming_checker::ArchNamingChecker;
