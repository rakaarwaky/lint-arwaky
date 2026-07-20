// PURPOSE: taxonomy_language_info_vo — shared language detection value object for role auditors
use shared::common::taxonomy_language_vo::Language as DetLang;

/// Pre-computed language flags for a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageInfo {
    pub is_rs: bool,
    pub is_py: bool,
    pub is_js: bool,
    pub lang: DetLang,
}

impl LanguageInfo {
    pub fn new(is_rs: bool, is_py: bool, is_js: bool, lang: DetLang) -> Self {
        Self {
            is_rs,
            is_py,
            is_js,
            lang,
        }
    }
}
