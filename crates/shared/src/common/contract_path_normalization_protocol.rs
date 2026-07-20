// PURPOSE: IPathNormalizationPort — protocol trait for file path normalization across platforms
use crate::common::taxonomy_path_vo::FilePath;

pub trait IPathNormalizationProtocol: Send + Sync {
    fn normalize_path(&self, path: FilePath) -> FilePath;
    fn resolve_infrastructure_path(
        &self,
        path: FilePath,
        context_path: Option<FilePath>,
    ) -> FilePath;
}
