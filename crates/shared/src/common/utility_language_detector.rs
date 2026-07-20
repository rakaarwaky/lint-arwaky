// PURPOSE: Language detection — pure, stateless taxonomy utility.
use crate::common::taxonomy_language_info_vo::LanguageInfo;
use crate::common::taxonomy_language_vo::Language;
use crate::common::taxonomy_path_vo::FilePath;
use crate::taxonomy_source_vo::SourceContentVO;

/// Detect the programming language of a file from its extension.
pub fn detect_language(path: &FilePath) -> Language {
    match path.extension().as_str() {
        "py" => Language::Python,
        "js" | "jsx" | "mjs" | "cjs" => Language::JavaScript,
        "ts" | "tsx" | "mts" | "cts" => Language::TypeScript,
        "rs" => Language::Rust,
        _ => Language::Unknown,
    }
}

/// Whether the file's language is one this linter can process.
pub fn is_lintable(path: &FilePath) -> bool {
    matches!(
        detect_language(path),
        Language::Python | Language::JavaScript | Language::TypeScript | Language::Rust
    )
}

/// Detect language info (pre-computed flags) from a FilePath.
pub fn detect_language_info(path: &FilePath) -> LanguageInfo {
    let lang = detect_language(path);
    flags_from_lang(lang)
}

/// Detect language info (pre-computed flags) from a SourceContentVO.
pub fn detect_language_info_from_source(source: &SourceContentVO) -> LanguageInfo {
    detect_language_info(&source.file_path)
}

fn flags_from_lang(lang: Language) -> LanguageInfo {
    let is_rs = lang == Language::Rust;
    let is_py = lang == Language::Python;
    let is_js = lang == Language::JavaScript || lang == Language::TypeScript;
    LanguageInfo {
        is_rs,
        is_py,
        is_js,
        lang,
    }
}
