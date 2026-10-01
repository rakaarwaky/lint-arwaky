mod common;

use shared_common::taxonomy_language_vo::Language;
use shared_filesystem::utility_language_detector::{
    detect_language, detect_language_info, is_lintable,
};

#[test]
fn detect_language_info_flags() {
    let info = detect_language_info(&common::fp("x.ts"));
    assert!(info.is_js);
    assert!(!info.is_rs);
    assert_eq!(info.lang, Language::TypeScript);
    let info = detect_language_info(&common::fp("x.rs"));
    assert!(info.is_rs);
    assert!(!info.is_py);
}
#[test]
fn is_lintable_flags() {
    assert!(is_lintable(&common::fp("x.rs")));
    assert!(is_lintable(&common::fp("x.py")));
    assert!(is_lintable(&common::fp("x.ts")));
    assert!(!is_lintable(&common::fp("x.yaml")));
}
#[test]
fn detect_language_by_extension() {
    assert_eq!(detect_language(&common::fp("x.py")), Language::Python);
    assert_eq!(detect_language(&common::fp("x.ts")), Language::TypeScript);
    assert_eq!(detect_language(&common::fp("x.mjs")), Language::JavaScript);
    assert_eq!(detect_language(&common::fp("x.rs")), Language::Rust);
    assert_eq!(detect_language(&common::fp("x.md")), Language::Unknown);
}
