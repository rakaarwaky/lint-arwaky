// PURPOSE: Segregated protocol traits for naming check capabilities
use crate::common::taxonomy_definition_vo::LayerMapVO;
use crate::common::taxonomy_lint_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;

pub trait INamingConventionChecker: Send + Sync {
    fn check_file_naming(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}

pub trait ISuffixPrefixChecker: Send + Sync {
    fn check_domain_suffixes(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
