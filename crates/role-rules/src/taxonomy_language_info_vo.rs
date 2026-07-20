// PURPOSE: taxonomy_language_info_vo — shared language detection value object for role auditors
use shared::common::utility_language_detector::detect_language;
use shared::common::taxonomy_language_vo::Language as DetLang;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_source_vo::SourceContentVO;

/// Pre-computed language flags for a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// ─── Block 1: Struct Definition ───────────────────────────
pub struct LanguageInfo {
    pub is_rs: bool,
    pub is_py: bool,
    pub is_js: bool,
    pub lang: DetLang,
}

// ─── Block 3: Constructors & Helpers ──────────────────────
// ─── Block 2: Public Contract ─────────────────────────────
impl LanguageInfo {
    /// Detect language from a `SourceContentVO`.
    pub fn new(source: &SourceContentVO) -> Self {
        let lang = detect_language(&source.file_path);
        Self::flags_from_lang(lang)
    }

    /// Detect language from a raw `FilePath`.
    pub fn new_from_path(fp: &FilePath) -> Self {
        let lang = detect_language(fp);
        Self::flags_from_lang(lang)
    }

    fn flags_from_lang(lang: DetLang) -> Self {
        let is_rs = lang == DetLang::Rust;
        let is_py = lang == DetLang::Python;
        let is_js = lang == DetLang::JavaScript || lang == DetLang::TypeScript;
        Self {
            is_rs,
            is_py,
            is_js,
            lang,
        }
    }
}
