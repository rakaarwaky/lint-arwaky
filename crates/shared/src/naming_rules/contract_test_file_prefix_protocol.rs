// PURPOSE: ITestFilePrefixProtocol — AES103 test/bench file-prefix enforcement seam
//
// The third naming capability. AES101 reads the stem shape and AES102 reads the
// layer suffix; AES103 reads the *test-suite* prefix of files that live in
// `tests/` and `benches/`, where the prefix is the virtual folder. Files outside
// those two directories carry no test-type prefix, so this seam is silent for them.
use shared_common::taxonomy_layer_vo::LayerMapVO;
use shared_common::taxonomy_lint_vo::LintResultList;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_path_vo::FilePathList;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;

/// FR-NamingRules-003: check every file inside `tests/` and `benches/` against
/// the flat test-type prefix vocabulary (AES103).
pub trait ITestFilePrefixProtocol: Send + Sync {
    /// Check every test/bench file name against the registered prefix set.
    fn check_test_file_prefixes(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
