// PURPOSE: taxonomy_language_helper — shared language detection utility for role auditors
// Eliminates boilerplate: every role auditor was repeating the same 5-line language detection pattern.
use shared::common::utility_language_detector::detect_language as detect_lang;
use shared::common::taxonomy_language_vo::Language as DetLang;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_source_vo::SourceContentVO;

/// Pre-computed language flags for a source file.
pub struct LanguageInfo {
    pub is_rs: bool,
    pub is_py: bool,
    pub is_js: bool,
    pub lang: DetLang,
}

/// Detect language from a `SourceContentVO` using the shared utility.
pub fn detect_language(source: &SourceContentVO) -> LanguageInfo {
    let lang = detect_lang(&source.file_path);
    flags_from_lang(lang)
}

/// Detect language from a raw `FilePath`.
pub fn detect_language_from_path(fp: &FilePath) -> LanguageInfo {
    let lang = detect_lang(fp);
    flags_from_lang(lang)
}

fn flags_from_lang(lang: DetLang) -> LanguageInfo {
    let is_rs = lang == DetLang::Rust;
    let is_py = lang == DetLang::Python;
    let is_js = lang == DetLang::JavaScript || lang == DetLang::TypeScript;
    LanguageInfo {
        is_rs,
        is_py,
        is_js,
        lang,
    }
}
