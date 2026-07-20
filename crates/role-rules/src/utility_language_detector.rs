// PURPOSE: utility_language_detector — stateless language detection utility for role auditors
use crate::taxonomy_language_info_vo::LanguageInfo;
use shared::common::taxonomy_language_vo::Language as DetLang;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::utility_language_detector::detect_language as detect_lang;
use shared::taxonomy_source_vo::SourceContentVO;

pub fn detect_language(source: &SourceContentVO) -> LanguageInfo {
    let lang = detect_lang(&source.file_path);
    flags_from_lang(lang)
}

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
