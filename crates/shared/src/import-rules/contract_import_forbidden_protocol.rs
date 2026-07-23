// PURPOSE: IImportForbiddenProtocol — exclusive contract for forbidden import checks (AES201)
// Checks that files do NOT import from layers they are forbidden to depend on.
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::taxonomy_definition_vo::LayerMapVO;
use crate::taxonomy_layer_vo::Identity;
use async_trait::async_trait;

/// For each file, verify its imports do not reach any layer listed in the `forbidden` set.
/// Used by the import orchestrator as part of the AES201 gate.
#[async_trait]
pub trait IImportForbiddenProtocol: Send + Sync {
    fn rule_name(&self) -> Identity;
    async fn check_forbidden_imports(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
