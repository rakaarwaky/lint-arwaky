// PURPOSE: naming-rules-domain capability contracts (AES102 `_protocol`).
//
// One file for the naming-rules feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs.

use crate::common::taxonomy_definition_vo::LayerMapVO;
use crate::common::taxonomy_lint_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::config_system::taxonomy_config_system_vo::ArchitectureConfig;

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
