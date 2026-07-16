// PURPOSE: CliLanguageDetector — ILanguageDetectorPort implementation for CLI commands
//
// Detects programming language from file extension.
// Self-contained infrastructure adapter — no shared helper needed.
use shared::common::contract_language_detector_port::ILanguageDetectorPort;
use shared::common::contract_language_detector_port::Language;
use shared::common::taxonomy_path_vo::FilePath;

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

impl ILanguageDetectorPort for CliLanguageDetector {
    fn detect(&self, path: &FilePath) -> Language {
        let ext = std::path::Path::new(path.value())
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        match ext {
            "py" => Language::Python,
            "js" | "jsx" | "mjs" | "cjs" => Language::JavaScript,
            "ts" | "tsx" | "mts" | "cts" => Language::TypeScript,
            "rs" => Language::Rust,
            _ => Language::Unknown,
        }
    }
}
