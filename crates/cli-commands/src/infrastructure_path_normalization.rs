use shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use shared::source_parsing::taxonomy_path_vo::FilePath;

pub struct CliPathNormalizationProvider;

impl CliPathNormalizationProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CliPathNormalizationProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl IPathNormalizationPort for CliPathNormalizationProvider {
    fn normalize_path(&self, path: FilePath) -> FilePath {
        path
    }

    fn resolve_infrastructure_path(
        &self,
        path: FilePath,
        _context_path: Option<FilePath>,
    ) -> FilePath {
        path
    }
}
