// PURPOSE: LanguageDetector — ILanguageDetectorPort implementation for file extension-based language detection
use shared::source_parsing::contract_language_detector_port::ILanguageDetectorPort;
use shared::source_parsing::contract_language_detector_port::Language;
use shared::source_parsing::taxonomy_path_vo::FilePath;

pub struct LanguageDetector;

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

impl ILanguageDetectorPort for LanguageDetector {
    fn detect(&self, path: &FilePath) -> Language {
        let ext = path.extension();
        match ext.as_str() {
            "py" => Language::Python,
            "js" | "jsx" | "mjs" | "cjs" => Language::JavaScript,
            "ts" | "tsx" | "mts" | "cts" => Language::TypeScript,
            "rs" => Language::Rust,
            _ => Language::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_python() {
        let detector = LanguageDetector::new();
        assert_eq!(
            detector.detect(&FilePath::new("main.py").unwrap()),
            Language::Python
        );
        assert_eq!(
            detector.detect(&FilePath::new("src/utils.py").unwrap()),
            Language::Python
        );
    }

    #[test]
    fn test_detect_javascript() {
        let detector = LanguageDetector::new();
        assert_eq!(
            detector.detect(&FilePath::new("app.js").unwrap()),
            Language::JavaScript
        );
        assert_eq!(
            detector.detect(&FilePath::new("App.jsx").unwrap()),
            Language::JavaScript
        );
    }

    #[test]
    fn test_detect_typescript() {
        let detector = LanguageDetector::new();
        assert_eq!(
            detector.detect(&FilePath::new("index.ts").unwrap()),
            Language::TypeScript
        );
        assert_eq!(
            detector.detect(&FilePath::new("App.tsx").unwrap()),
            Language::TypeScript
        );
    }

    #[test]
    fn test_detect_rust() {
        let detector = LanguageDetector::new();
        assert_eq!(
            detector.detect(&FilePath::new("lib.rs").unwrap()),
            Language::Rust
        );
        assert_eq!(
            detector.detect(&FilePath::new("main.rs").unwrap()),
            Language::Rust
        );
    }

    #[test]
    fn test_detect_unknown() {
        let detector = LanguageDetector::new();
        assert_eq!(
            detector.detect(&FilePath::new("README.md").unwrap()),
            Language::Unknown
        );
        assert_eq!(
            detector.detect(&FilePath::new("style.css").unwrap()),
            Language::Unknown
        );
    }

    #[test]
    fn test_is_lintable() {
        let detector = LanguageDetector::new();
        assert!(detector.is_lintable(&FilePath::new("main.py").unwrap()));
        assert!(detector.is_lintable(&FilePath::new("app.js").unwrap()));
        assert!(detector.is_lintable(&FilePath::new("index.ts").unwrap()));
        assert!(detector.is_lintable(&FilePath::new("lib.rs").unwrap()));
        assert!(!detector.is_lintable(&FilePath::new("README.md").unwrap()));
    }
}
