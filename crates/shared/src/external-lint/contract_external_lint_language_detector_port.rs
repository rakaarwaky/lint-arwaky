// PURPOSE: IExternalLintLanguageDetectorPort — port for detecting languages in a project directory
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

pub struct DetectedLanguages {
    pub has_rs: BooleanVO,
    pub has_py: BooleanVO,
    pub has_js: BooleanVO,
}

#[async_trait]
pub trait IExternalLintLanguageDetectorPort: Send + Sync {
    /// Detect which languages are present at the given path.
    async fn detect_languages(&self, path: &FilePath) -> DetectedLanguages;
}
