use cli_commands_lint_arwaky::infrastructure_language_detector::CliLanguageDetector;
use shared::common::contract_language_detector_port::{ILanguageDetectorPort, Language};
use shared::common::taxonomy_path_vo::FilePath;

fn make_fp(s: &str) -> FilePath {
    FilePath::new(s.to_string()).unwrap_or_default()
}

#[test]
fn language_detector_constructs() {
    let detector = CliLanguageDetector::new();
    let _ = detector;
}

#[test]
fn language_detector_detect_rust() {
    let detector = CliLanguageDetector::new();
    assert_eq!(detector.detect(&make_fp("main.rs")), Language::Rust);
}

#[test]
fn language_detector_detect_python() {
    let detector = CliLanguageDetector::new();
    assert_eq!(detector.detect(&make_fp("app.py")), Language::Python);
}

#[test]
fn language_detector_detect_typescript() {
    let detector = CliLanguageDetector::new();
    assert_eq!(detector.detect(&make_fp("component.ts")), Language::TypeScript);
}

#[test]
fn language_detector_detect_javascript() {
    let detector = CliLanguageDetector::new();
    assert_eq!(detector.detect(&make_fp("index.js")), Language::JavaScript);
}

#[test]
fn language_detector_is_lintable_rust() {
    let detector = CliLanguageDetector::new();
    assert!(detector.is_lintable(&make_fp("main.rs")));
}

#[test]
fn language_detector_is_lintable_python() {
    let detector = CliLanguageDetector::new();
    assert!(detector.is_lintable(&make_fp("app.py")));
}

#[test]
fn language_detector_is_lintable_typescript() {
    let detector = CliLanguageDetector::new();
    assert!(detector.is_lintable(&make_fp("component.ts")));
}

#[test]
fn language_detector_is_lintable_not_lintable() {
    let detector = CliLanguageDetector::new();
    assert!(!detector.is_lintable(&make_fp("image.png")));
    assert!(!detector.is_lintable(&make_fp("data.bin")));
}

#[test]
fn language_detector_default() {
    let detector = CliLanguageDetector::default();
    assert_eq!(detector.detect(&make_fp("test.py")), Language::Python);
}
