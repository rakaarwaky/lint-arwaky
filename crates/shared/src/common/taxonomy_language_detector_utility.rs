// PURPOSE: LanguageDetector — shared ILanguageDetectorPort implementation
//
// Stateless utility: detects programming language from file extension.
// Reusable by any crate that needs extension-based language detection.
use crate::common::contract_language_detector_port::ILanguageDetectorPort;
use crate::common::contract_language_detector_port::Language;
use crate::common::taxonomy_path_vo::FilePath;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct LanguageDetector;

// ─── Block 2: Public Contract ─────────────────────────────
impl ILanguageDetectorPort for LanguageDetector {
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

// ─── Block 3: Constructors ────────────────────────────────
impl LanguageDetector {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LanguageDetector {
    fn default() -> Self {
        Self::new()
    }
}
