// PURPOSE: INamingCheckerProtocol — protocol trait for naming check capabilities
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use async_trait::async_trait;

#[async_trait]
pub trait INamingCheckerProtocol: Send + Sync {
    async fn check_file_naming(
        &self,
        analyzer: &dyn ILayerDetectionProtocol,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
    async fn check_domain_suffixes(
        &self,
        analyzer: &dyn ILayerDetectionProtocol,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    );
}
