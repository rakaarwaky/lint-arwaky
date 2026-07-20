// PURPOSE: CliLanguageDetector — thin wrapper over language detection utilities
// Uses utility_language_detector directly — no protocol dependency.
use shared::common::taxonomy_language_vo::Language;
use shared::common::utility_language_detector::{detect_language, is_lintable};
use shared::common::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Copy)]
pub struct CliLanguageDetector;

impl CliLanguageDetector {
    pub fn new() -> Self {
        Self
    }

    pub fn detect(path: &FilePath) -> Language {
        detect_language(path)
    }

    pub fn is_lintable(path: &FilePath) -> bool {
        is_lintable(path)
    }
}

impl Default for CliLanguageDetector {
    fn default() -> Self {
        Self::new()
    }
}
