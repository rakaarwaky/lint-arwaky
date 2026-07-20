// PURPOSE: IImportMandatoryProtocol — exclusive contract for mandatory import checks (AES202)
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::taxonomy_definition_vo::LayerMapVO;
use crate::taxonomy_layer_vo::Identity;
use async_trait::async_trait;

#[async_trait]
pub trait IImportMandatoryProtocol: Send + Sync {
    fn rule_name(&self) -> Identity;
    async fn run_mandatory_imports(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
