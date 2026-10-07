// PURPOSE: naming-rules-domain capability contracts (AES102 `_protocol`).
//
// One file for the naming-rules feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs.

use shared_common::taxonomy_layer_vo::LayerMapVO;
use shared_common::taxonomy_lint_vo::LintResultList;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_path_vo::FilePathList;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;

/// FR-NamingRules-001: check each file's stem against the layer_concern_role
/// convention (AES101).
pub trait INamingConventionProtocol: Send + Sync {
    /// Check every file stem against the snake_case prefix_concept_suffix pattern.
    fn check_file_naming(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

/// FR-NamingRules-002: check each file's layer suffix and role prefix against
/// the per-layer policy (AES102).
pub trait ISuffixPolicyProtocol: Send + Sync {
    /// Check every file's suffix against the per-layer allowed/forbidden policy.
    fn check_domain_suffixes(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
