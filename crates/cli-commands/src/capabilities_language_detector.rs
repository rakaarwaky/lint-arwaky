// PURPOSE: CliLanguageDetector — ILanguageDetectorProtocol implementation for CLI commands
//
// Adapts the pure `detect_language`/`is_lintable` taxonomy utilities to the
// ILanguageDetectorProtocol contract. No inner struct — the utilities are stateless.
use shared::common::contract_language_detector_protocol::ILanguageDetectorProtocol;
use shared::common::contract_language_detector_protocol::Language;
use shared::common::taxonomy_language_detector_utility::{detect_language, is_lintable};
use shared::common::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Copy)]
pub struct CliLanguageDetector;

impl CliLanguageDetector {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CliLanguageDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl ILanguageDetectorProtocol for CliLanguageDetector {
    fn detect(&self, path: &FilePath) -> Language {
        detect_language(path)
    }

    fn is_lintable(&self, path: &FilePath) -> bool {
        is_lintable(path)
    }
}
