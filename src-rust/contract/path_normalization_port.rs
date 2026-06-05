use crate::taxonomy::source_path_vo::FilePath;

pub trait IPathNormalizationPort: Send + Sync {
    fn normalize_path(&self, path: FilePath) -> FilePath;
    fn resolve_infrastructure_path(
        &self,
        path: FilePath,
        context_path: Option<FilePath>,
    ) -> FilePath;
}
