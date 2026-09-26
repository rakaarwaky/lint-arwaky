// PURPOSE: INamingCheckerProtocol — rich capability contract for naming check operations
use crate::common::taxonomy_definition_vo::LayerMapVO;
use crate::common::taxonomy_lint_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;

/// Capability contract for naming-rules: every operation the capabilities expose.
pub trait INamingCheckerProtocol: Send + Sync {
    /// AES101 — check each file's stem against the layer_concern_role convention.
    fn check_file_naming(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );

    /// AES102 — check each file's layer suffix and role prefix against the per-layer policy.
    fn check_domain_suffixes(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
