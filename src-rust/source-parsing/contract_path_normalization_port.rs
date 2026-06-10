// PURPOSE: IPathNormalizationPort — port trait for file path normalization across platforms
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IPathNormalizationPort: Send + Sync {
    fn normalize_path(&self, path: FilePath) -> FilePath;
    fn resolve_infrastructure_path(
        &self,
        path: FilePath,
        context_path: Option<FilePath>,
    ) -> FilePath;
}
