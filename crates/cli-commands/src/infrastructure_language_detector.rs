use shared::common::contract_language_detector_port::ILanguageDetectorPort;
use shared::common::contract_language_detector_port::Language;
use shared::common::taxonomy_language_detector_helper::LanguageDetector;
use shared::common::taxonomy_path_vo::FilePath;

pub struct CliLanguageDetector {
    inner: LanguageDetector,
}

impl CliLanguageDetector {
    pub fn new() -> Self {
        Self {
            inner: LanguageDetector::new(),
        }
    }
}

impl Default for CliLanguageDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl ILanguageDetectorPort for CliLanguageDetector {
    fn detect(&self, path: &FilePath) -> Language {
        self.inner.detect(path)
    }

    fn is_lintable(&self, path: &FilePath) -> bool {
        self.inner.is_lintable(path)
    }
}
